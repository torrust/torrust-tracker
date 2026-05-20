---
doc-type: issue
issue-type: task
status: done
priority: p1
github-issue: 1721
spec-path: docs/issues/closed/1721-1525-07-align-rust-and-db-types.md
branch: 1721-1525-07-align-rust-and-db-types
related-pr: null
last-updated-utc: null
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/README.md
    - docs/issues/closed/1525-overhaul-persistence.md
    - packages/tracker-core/
---

# Subissue 1525-07: Align Rust and Database Types

## Goal

Widen the MySQL download-counter columns from `INTEGER` (32-bit signed) to `BIGINT` (64-bit),
delivered as a versioned `sqlx` migration. The Rust type `NumberOfDownloads` stays `u32` —
the database column is intentionally wider than the Rust type, and that is the correct design
(see [Design Decision](#design-decision-widen-db-only-keep-rust-type) below).

## Type-Mapping Diagram

### Current state (before this subissue)

```text
DB column (MySQL)       sqlx read    Driver cast    Rust domain     Wire (write)
────────────────────    ──────────   ────────────   ─────────────   ──────────────────────
torrents.completed
  INT (signed 32-bit)   → i64      → u32::try_from  NumberOfDownloads  UDP: i32::try_from (saturate)
  max 2,147,483,647                  (may error!)    = u32              HTTP: i64::from(u32) (infallible)

torrent_aggregate_metrics.value
  INT (signed 32-bit)   → i64      → u32::try_from  (same alias)
  max 2,147,483,647                  (may error!)
```

**Problem**: `u32::MAX` (4,294,967,295) > `i32::MAX` (2,147,483,647). Once the counter exceeds
`i32::MAX`, the MySQL write fails or overflows silently.

### Final state (after this subissue)

```text
DB column (MySQL)       sqlx read    Driver cast    Rust domain     Wire (write)
────────────────────    ──────────   ────────────   ─────────────   ──────────────────────
torrents.completed
  BIGINT (signed 64)    → i64      → u32::try_from  NumberOfDownloads  UDP: i32::try_from (saturate)
  max 9,223,372,036,…               (infallible      = u32              HTTP: i64::from(u32) (infallible)
                                     for u32 range)

torrent_aggregate_metrics.value
  BIGINT (signed 64)    → i64      → u32::try_from  (same alias)
  max 9,223,372,036,…               (infallible
                                     for u32 range)
```

**SQLite**: no column change needed — SQLite `INTEGER` already stores any value as signed
64-bit. A no-op migration is added solely to keep the migration history aligned with MySQL.

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
narrow-cast to `u32`.

### Why this is a problem

The MySQL `INT` column type is **signed 32-bit** (max 2,147,483,647). `u32::MAX` is
4,294,967,295 — roughly double that limit. Once the download counter exceeds `i32::MAX` the
MySQL write fails or silently overflows. Widening the column to `BIGINT` removes this ceiling
while keeping the Rust type and all existing wire-encoding logic unchanged.

**Protocol encoding** (no changes in this subissue):

- UDP scrape (`i32` wire field): `i32::try_from(u32)` already saturates at `i32::MAX`.
- HTTP scrape (bencoded `i64`): `i64::from(u32)` is infallible; no change needed.

### Why migrations first (1525-06 before 1525-07)

The column-widening change must be a versioned migration, not ad hoc DDL. The migration
framework from `1525-06` ensures the change is recorded in `_sqlx_migrations`, testable, and
safe in production upgrade scenarios.

## Design Decision: Widen DB Only, Keep Rust Type

The initial proposal for this subissue suggested widening `NumberOfDownloads` from `u32` to
`u64` alongside the database column. After analysis, **only the DB column is widened**. The
Rust type stays `u32`. Here is the reasoning:

### Why NOT widen the Rust type

The database in this tracker is an internal persistence store, not a shared external system.
No other service writes to it directly. Writing a value above `u32::MAX` into this database
would mean the application logic itself had produced that value — which is impossible while
`NumberOfDownloads = u32`. The write path is therefore fully bounded by the Rust type at
compile time.

This is the same reasoning as storing an enum variant as a string in the database: the string
column could hold arbitrary text, but the application only ever writes valid variant names. The
wider storage type is intentional; it does not indicate that the application type should match it.

### The read path is safe too

If someone bypassed the application and wrote a value above `u32::MAX` directly into the
database, the driver would return a `MalformedDatabaseRecord` error at read time — which is the
correct behaviour. The application should not silently accept data that violates its own
invariants. We already have similar guarded conversions elsewhere in the drivers.

### Why the original proposal suggested `u64`

The original motivation was defensive: aligning the Rust type to the full BIGINT range would
make the read path infallible and future-proof against protocol changes. That reasoning is
valid, but it comes at the cost of a large cascade change (scrape encoders, swarm metadata,
benchmark helpers, UDP handler) for a scenario — direct external writes — that is out of scope
and would break other invariants anyway. The simpler approach (widen DB only) fixes the actual
bug with minimal churn.

### `SwarmMetadata` field types

`complete` and `incomplete` in `SwarmMetadata` are point-in-time counts of currently connected
seeders and leechers. They are in-memory only and never persisted. Widening them would add
scope without fixing any real problem; they remain `u32`.

`downloaded` is the persisted accumulator. It stays `u32` in Rust but the field should use the
`NumberOfDownloads` type alias (not the bare `u32`) to make the intent explicit. This is a
cosmetic fix included in Task 2.

## Proposed Branch

- `1721-1525-07-align-rust-and-db-types`

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
PostgreSQL starting from migration 1. PostgreSQL's migration 4 widens the columns using
PostgreSQL-specific `ALTER COLUMN ... TYPE BIGINT` syntax; it is not a no-op for PostgreSQL.

### Rust changes (cosmetic only)

**`packages/primitives/src/swarm_metadata.rs`** — use the `NumberOfDownloads` alias instead
of the bare `u32` for the `downloaded` field and the `downloads()` return type:

```rust
// Before
pub downloaded: u32,
pub fn downloads(&self) -> u32 { ... }

// After
pub downloaded: NumberOfDownloads,
pub fn downloads(&self) -> NumberOfDownloads { ... }
```

`NumberOfDownloads` remains `u32` in `packages/primitives/src/lib.rs`. No other Rust types
change. No cascade compilation fixes are required.

## Tasks

### Task 1 — Add migration files

Create the two new migration files listed above. Do not modify any existing migration file.

**Outcome**: `packages/tracker-core/migrations/` has four files in each of `sqlite/` and
`mysql/`. The fourth file is verified by running the migration against a fresh test database
of each type.

### Task 2 — Use `NumberOfDownloads` alias in `SwarmMetadata`

Update `SwarmMetadata.downloaded` and `downloads()` to use the `NumberOfDownloads` alias
instead of the bare `u32`. This is a cosmetic change; no logic changes.

**Outcome**: `cargo build --workspace` succeeds with no warnings or errors.

### Task 3 — Validate the migration

Add or extend tests that verify:

- **MySQL migration**: running the migration on a database with the pre-migration `INT` column
  produces a `BIGINT` column, and writing and reading a value in the range `(i32::MAX, u32::MAX]`
  round-trips correctly (this range was previously unsafe with `INT`).
- **SQLite no-op**: the migration applies cleanly (recorded in `_sqlx_migrations`) and the
  column continues to accept all values in the `u32` range.

These tests extend the existing driver `#[cfg(test)]` modules.

**Outcome**: `cargo test --workspace --all-targets` passes.

## Out of Scope

- Widening `NumberOfDownloads` to `u64` — explicitly out of scope (see Design Decision above).
- PostgreSQL migration files — added in subissue `1525-08`.
- Down migrations (rollback) — not needed at this stage.
- Trait splitting or other structural refactoring.
- Changes to `complete` / `incomplete` fields in `SwarmMetadata`.

## Acceptance Criteria

- [ ] `packages/tracker-core/migrations/sqlite/20260409120000_torrust_tracker_widen_download_counters.sql`
      exists and is a comment-only no-op.
- [ ] `packages/tracker-core/migrations/mysql/20260409120000_torrust_tracker_widen_download_counters.sql`
      exists and widens `torrents.completed` and `torrent_aggregate_metrics.value` to `BIGINT`.
- [ ] `NumberOfDownloads` remains `u32` in `packages/primitives/src/lib.rs`.
- [ ] `SwarmMetadata.downloaded` and `downloads()` use the `NumberOfDownloads` alias; bare
      `u32` is replaced with the alias in that struct.
- [ ] A test verifies that writing and reading a value in `(i32::MAX, u32::MAX]` round-trips
      correctly on MySQL after the migration.
- [ ] A test verifies the SQLite no-op migration applies cleanly.
- [ ] No new `as u32` casts or compiler-suppression attributes introduced by this subissue.
- [ ] Persistence benchmarking (see subissue `1525-03`) shows no regression against the
      committed baseline.
- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `linter all` exits with code `0`.

## References

- EPIC: `#1525`
- Subissue `1525-06`: `docs/issues/1719-1525-06-introduce-schema-migrations.md` — must be completed
  first (provides the migration framework)
- Subissue `1525-08`: `docs/issues/1723-1525-08-add-postgresql-driver.md` — adds PostgreSQL
  migration files including the history-aligned no-op for this migration
- Subissue `1525-03`: `docs/issues/1525-03-persistence-benchmarking.md` — benchmark baseline
- Reference implementation branch: `josecelano:pr-1684-review` — see EPIC for checkout
  instructions (`docs/issues/1525-overhaul-persistence.md`)
- Reference files:
  - `packages/tracker-core/migrations/sqlite/20260409120000_torrust_tracker_widen_download_counters.sql`
  - `packages/tracker-core/migrations/mysql/20260409120000_torrust_tracker_widen_download_counters.sql`
  - `packages/primitives/src/swarm_metadata.rs` (alias cosmetic fix)
