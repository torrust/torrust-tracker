# Subissue 1525-07: Align Rust and Database Types

## Goal

Widen the download-counter type in Rust from `u32` to `u64` and widen the corresponding
database columns from `INTEGER` (32-bit, MySQL) to `BIGINT` (64-bit), delivered as a versioned
`sqlx` migration so the change is explicit, testable, and tracked as a forward schema change.

## Background

### Current state

By the time this subissue is implemented, subissue `1525-06` will have wired `sqlx::migrate!()`
into both drivers. The schema at that point contains:

- `torrents.completed` — `INTEGER` in MySQL (32-bit signed, max ≈ 2.1 billion), `INTEGER` in
  SQLite (storage is already 64-bit for any integer value).
- `torrent_aggregate_metrics.value` — same types as above.

The Rust type alias is `NumberOfDownloads = u32` in
`packages/primitives/src/lib.rs`. The `SwarmMetadata.downloaded` field also uses this type.
The drivers read the column as `i64` (sqlx always returns integer columns as `i64`) and
immediately narrow-cast to `u32`.

### Why this is a problem

The MySQL `INT` column type is **signed 32-bit** (max 2,147,483,647). Writing a `u32` value
above that limit silently overflows or errors. Practically, the counter saturates at the same
point as the UDP scrape wire format (`completed` is `i32` in BEP 15), but the correct fix is
to widen the storage type rather than rely on implicit saturation in the driver.

`u32::MAX` (4,294,967,295) is already higher than the `i32::MAX` wire limit, so protocol
saturation happens before storage overflow today. However, aligning storage to `BIGINT` and the
Rust type to `u64` makes the storage contract explicit and decoupled from any particular
protocol encoding. Future protocol changes or a direct-database query tool cannot accidentally
exceed a silently-constrained column.

**Protocol encoding** (read-only, no changes needed in this subissue):

- UDP scrape response (`i32` wire field): the existing conversion from `NumberOfDownloads` to
  `i32` already saturates at `i32::MAX`. This remains unchanged.
- HTTP scrape response (bencoded `i64`): `bencode_download_count()` saturates at `i64::MAX`.
  This remains unchanged.

### Why migrations first (1525-06 before 1525-07)

The column-widening change must be delivered as a versioned migration rather than an ad hoc DDL
update. Having the migration framework from `1525-06` in place ensures the change is tracked in
`_sqlx_migrations`, tested like any other migration, and can be reasoned about in production
upgrade scenarios.

## Proposed Branch

- `1525-07-align-rust-and-db-types`

## What Changes

### Migration files

Add the fourth migration to both existing backends:

```text
packages/tracker-core/migrations/sqlite/20260409120000_torrust_tracker_widen_download_counters.sql
packages/tracker-core/migrations/mysql/20260409120000_torrust_tracker_widen_download_counters.sql
```

**SQLite** — no-op (SQLite already stores any `INTEGER` value as a 64-bit signed integer):

```sql
-- SQLite stores INTEGER values as signed 64-bit integers already.
-- This migration is intentionally a no-op so the migration history stays
-- aligned with the MySQL backend.
```

**MySQL** — widen both download-counter columns:

```sql
ALTER TABLE torrents
    MODIFY completed BIGINT NOT NULL DEFAULT 0;

ALTER TABLE torrent_aggregate_metrics
    MODIFY value BIGINT NOT NULL DEFAULT 0;
```

PostgreSQL migration files are not created here. They will be added in subissue `1525-08` when
the PostgreSQL driver is introduced. Following the
[history-alignment pattern](1719-1525-06-introduce-schema-migrations.md#history-alignment-pattern)
established in `1525-06`, subissue `1525-08` creates **all four** migration files for
PostgreSQL starting from migration 1. PostgreSQL's migration 1 creates the columns as
`INTEGER` (matching the original schema from the other backends), and migration 4 widens them
to `BIGINT` using PostgreSQL-specific `ALTER COLUMN ... TYPE BIGINT` syntax. Migration 4 is
not a no-op for PostgreSQL.

### Rust type changes

**`packages/primitives/src/lib.rs`** — widen the type alias:

```rust
// Before
pub type NumberOfDownloads = u32;

// After
pub type NumberOfDownloads = u64;
```

**`packages/primitives/src/swarm_metadata.rs`** — `downloaded` field currently uses the bare
`u32`. Update it to use `NumberOfDownloads` explicitly:

```rust
// Before
pub downloaded: u32,

// After
pub downloaded: NumberOfDownloads,
```

Also update the `downloads()` method return type to `NumberOfDownloads`.

### Driver conversion changes

After `1525-05`, the sqlx drivers read counter columns as `i64`. With `NumberOfDownloads = u32`
the read path does `u32::try_from(i64_value)`. After this subissue it becomes
`u64::try_from(i64_value)`.

Because the database column type is `BIGINT` (signed), the **write path** must also encode
`u64 → i64`. Values above `i64::MAX` (≈ 9.2 × 10¹⁸) cannot be stored and must return an
error rather than silently truncate. Add named helper methods to each driver to make the
conversion explicit and consistent:

```rust
fn decode_counter(value: i64) -> Result<NumberOfDownloads, Error> {
    u64::try_from(value).map_err(|err| Error::invalid_query(DRIVER, err))
}

fn encode_counter(value: NumberOfDownloads) -> Result<i64, Error> {
    i64::try_from(value).map_err(|err| Error::invalid_query(DRIVER, err))
}
```

Use these helpers in every place a counter column is read from or written to the database.

### Cascade compilation fixes

Widening `NumberOfDownloads` from `u32` to `u64` will produce compilation errors wherever the
old `u32` range was assumed. Fix all errors; do not add `as u32` casts or `allow` attributes
to suppress them.

## Tasks

### Task 1 — Add migration files

Create the two new migration files listed above. Do not modify any existing migration file.

**Outcome**: `packages/tracker-core/migrations/` has four files in each of `sqlite/` and
`mysql/`. The fourth file is verified by running the migration against a fresh test database
of each type.

### Task 2 — Widen `NumberOfDownloads` and fix cascade

Change `NumberOfDownloads = u32 → u64` in `packages/primitives/src/lib.rs` and update
`SwarmMetadata.downloaded` to use the alias. Fix all resulting compilation errors across the
workspace (driver conversion logic, scrape response encoding, announce handler arithmetic,
etc.).

Add `decode_counter` / `encode_counter` helpers to both driver files as described above.

**Outcome**: `cargo build --workspace` succeeds with no warnings or errors.

### Task 3 — Validate migration and type alignment

Add or extend tests that verify:

- **MySQL migration**: running the migration on a database with the pre-migration `INT` column
  produces a `BIGINT` column, and writing and reading a value larger than `2^31 − 1` round-trips
  correctly.
- **SQLite no-op**: the migration applies cleanly (recorded in `_sqlx_migrations`) and the
  column already accepts large values.
- **Boundary encode**: writing a `u64` counter value of exactly `i64::MAX` succeeds; writing
  `i64::MAX + 1` returns an appropriate error rather than panicking or wrapping.

These tests extend the existing driver `#[cfg(test)]` modules.

**Outcome**: `cargo test --workspace --all-targets` passes.

## Out of Scope

- PostgreSQL migration files — added in subissue `1525-08`.
- Down migrations (rollback) — not needed at this stage.
- Trait splitting or other structural refactoring.
- Other numeric types beyond `NumberOfDownloads` / download counters.

## Acceptance Criteria

- [ ] `packages/tracker-core/migrations/sqlite/20260409120000_torrust_tracker_widen_download_counters.sql`
      exists and is a comment-only no-op.
- [ ] `packages/tracker-core/migrations/mysql/20260409120000_torrust_tracker_widen_download_counters.sql`
      exists and widens `torrents.completed` and `torrent_aggregate_metrics.value` to `BIGINT`.
- [ ] `NumberOfDownloads = u64` in `packages/primitives/src/lib.rs`.
- [ ] `SwarmMetadata.downloaded` uses `NumberOfDownloads`; bare `u32` is removed from that field.
- [ ] Both driver files use explicit `decode_counter` / `encode_counter` helpers for all
      counter-column reads and writes.
- [ ] `encode_counter` returns an error (not a panic, not silent truncation) for values
      above `i64::MAX`.
- [ ] A test verifies round-trip of a value larger than `u32::MAX` for each backend.
- [ ] A test verifies the encode error path for values above `i64::MAX`.
- [ ] No `as u32` casts or compiler-suppression attributes introduced by this subissue.
- [ ] Persistence benchmarking (see subissue `1525-03`) shows no regression against the
      committed baseline.
- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `linter all` exits with code `0`.

## References

- EPIC: `#1525`
- Subissue `1525-06`: `docs/issues/1719-1525-06-introduce-schema-migrations.md` — must be completed
  first (provides the migration framework)
- Subissue `1525-08`: `docs/issues/1525-08-add-postgresql-driver.md` — adds PostgreSQL
  migration files including the history-aligned no-op for this migration
- Subissue `1525-03`: `docs/issues/1525-03-persistence-benchmarking.md` — benchmark baseline
- Reference implementation branch: `josecelano:pr-1684-review` — see EPIC for checkout
  instructions (`docs/issues/1525-overhaul-persistence.md`)
- Reference files:
  - `packages/tracker-core/migrations/sqlite/20260409120000_torrust_tracker_widen_download_counters.sql`
  - `packages/tracker-core/migrations/mysql/20260409120000_torrust_tracker_widen_download_counters.sql`
  - `packages/primitives/src/lib.rs` (type alias change)
  - `packages/primitives/src/swarm_metadata.rs` (field type change)
  - `packages/tracker-core/src/databases/driver/sqlite.rs` (decode/encode helpers)
  - `packages/tracker-core/src/databases/driver/mysql.rs` (decode/encode helpers)
