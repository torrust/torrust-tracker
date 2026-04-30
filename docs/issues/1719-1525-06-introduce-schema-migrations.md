# Subissue Draft for #1525-06: Introduce Schema Migrations

## Goal

Replace the raw DDL calls in the async drivers with `sqlx`'s versioned migration framework,
making schema evolution explicit, reproducible, and aligned across all SQL backends.

## Why

After subissue `1525-05` the drivers still manage their schema through hand-written
`CREATE TABLE IF NOT EXISTS ...` statements executed by `create_database_tables()`. That approach
has no history, no ordering guarantees, and no way to apply incremental schema changes safely to
an existing database. `sqlx::migrate!()` gives us versioned SQL files, automatic up-migration on
startup, and a `_sqlx_migrations` tracking table — a foundation required before PostgreSQL can
be added (subissue `1525-08`).

## Proposed Branch

- `1525-06-introduce-schema-migrations`

## Background

### Starting point

By the time this subissue is implemented, subissue `1525-05` will have delivered async SQLite
and MySQL drivers backed by `sqlx`. Each driver has an `ensure_schema()` latch that calls
`create_database_tables()` lazily. That method currently issues raw `sqlx::query()` DDL. This
subissue replaces that raw DDL path with `sqlx::migrate!()`.

There are already 3 migration files under `packages/tracker-core/migrations/` (both `sqlite/`
and `mysql/` subdirectories) that capture the schema history:

```text
20240730183000_torrust_tracker_create_all_tables.sql
20240730183500_torrust_tracker_keys_valid_until_nullable.sql
20250527093000_torrust_tracker_new_torrent_aggregate_metrics_table.sql
```

These files were written for users to run manually. The tracker has never executed them
automatically. This subissue is the first time they are wired into the application startup path.

### Current code behavior

The current `create_database_tables()` method issues `CREATE TABLE IF NOT EXISTS` for all four
tables (`whitelist`, `torrents`, `torrent_aggregate_metrics`, `keys`) using hardcoded DDL that
already reflects the final schema state (nullable `valid_until`, all four tables present). The
current `drop_database_tables()` drops `whitelist`, `torrents`, and `keys` but **not**
`torrent_aggregate_metrics`, which leaks across test drop/create cycles.

This gives two distinct behaviors today:

- **New (empty) database**: all four tables are created in the final schema state — equivalent
  to having run all three migrations in sequence. The database is immediately usable.
- **Existing database (no `_sqlx_migrations` table)**: `IF NOT EXISTS` silently skips tables
  that already exist. Migration 2's `ALTER TABLE` (making `valid_until` nullable) never runs,
  so an old `keys` table with `valid_until NOT NULL` stays broken. Migration 3's
  `torrent_aggregate_metrics` table is created if absent (it did not exist before migration 3).
  The user is expected to run the missing migrations manually, as documented in
  `packages/tracker-core/migrations/README.md`.

### How sqlx migrations work

`sqlx::migrate!("path/to/migrations")` is a compile-time macro that embeds all `.sql` files
found under the given directory into the binary. At runtime, calling `MIGRATOR.run(&pool)`
applies any unapplied migrations in timestamp order and records them in the `_sqlx_migrations`
tracking table. Each migration is applied exactly once; on subsequent runs its checksum is
verified but it is not re-applied. Migrations are irreversible by default (no down migrations).

The `macros` feature of `sqlx` is required for the `sqlx::migrate!()` macro.

Because the migration files are embedded at compile time, the running binary carries all
migrations and does not need the `.sql` files on disk at runtime. No special deployment
packaging is required beyond distributing the binary.

### Migration file layout

```text
packages/tracker-core/migrations/
  sqlite/
    20240730183000_torrust_tracker_create_all_tables.sql
    20240730183500_torrust_tracker_keys_valid_until_nullable.sql
    20250527093000_torrust_tracker_new_torrent_aggregate_metrics_table.sql
  mysql/
    20240730183000_torrust_tracker_create_all_tables.sql
    20240730183500_torrust_tracker_keys_valid_until_nullable.sql
    20250527093000_torrust_tracker_new_torrent_aggregate_metrics_table.sql
  postgresql/  ← added in subissue 1525-08; see "PostgreSQL migration alignment" below
    ...
```

Each backend has its own directory because SQL dialects differ.

### History-alignment pattern

All backends must have the **same set of migration filenames** with the same timestamps. When a
schema change is not needed for a specific backend (e.g., a column-type widening that the
backend's native type system already handles), the migration file still exists for that backend
but contains only a comment:

```sql
-- This migration is intentionally a no-op for this backend.
-- The migration file exists to keep the version history aligned
-- with the other backends.
```

This keeps the `_sqlx_migrations` version history identical across backends, which simplifies
reasoning about compatibility and avoids gaps in the timestamp sequence.

### PostgreSQL migration alignment

When subissue `1525-08` adds the PostgreSQL driver, its migration directory must contain the
**same set of migration filenames** as SQLite and MySQL, starting from migration 1 — treating
PostgreSQL as if it existed in the project from the beginning. This keeps the
`_sqlx_migrations` version history identical across all three backends.

Concretely, PostgreSQL's migration 1 creates the original schema (same initial table definitions
as SQLite and MySQL migration 1), and the subsequent migrations apply the same schema changes in
order. Any migration that is a no-op for PostgreSQL follows the history-alignment pattern
(comment-only file) rather than being omitted.

This means no additional "catch-up" migration is needed when PostgreSQL is added: the full
history starts from migration 1, identical to the other backends.

### Legacy upgrade path

When a v4 tracker starts against a database that was managed by an older tracker version, the
`_sqlx_migrations` table will not yet exist. Calling `MIGRATOR.run(&pool)` blindly on such a
database would try to re-apply migration 1 (`CREATE TABLE IF NOT EXISTS ...`) which is harmless
for `whitelist` and `torrents`, but migration 2's `ALTER TABLE` would fail because the
columns it targets are already in their expected state (on a fully-updated old schema) or in an
inconsistent state (on a partially-updated one).

**Decision: legacy bootstrap with a v4 upgrade pre-condition.**

The v4 changelog requires that users running an older tracker must apply all three existing
manual migrations before upgrading to v4. Once that pre-condition is met, the driver can
safely detect the legacy state and bootstrap the tracking table automatically:

1. If `_sqlx_migrations` does **not** exist and the schema tables (`whitelist`, `torrents`,
   `keys`, `torrent_aggregate_metrics`) do exist → **legacy bootstrap path**:
   - Create the `_sqlx_migrations` table (via `MIGRATOR.ensure_migrations_table(&pool)`).
   - Insert fake-applied rows for the three pre-existing migrations (correct versions and
     checksums from the embedded `MIGRATOR`), marking them as already executed.
   - Call `MIGRATOR.run(&pool)` to apply any migrations added after those three.
2. If `_sqlx_migrations` exists → **normal path**: call `MIGRATOR.run(&pool)` directly; sqlx
   skips already-applied migrations.
3. If no tables exist at all → **fresh database path**: `MIGRATOR.run(&pool)` creates
   `_sqlx_migrations` and applies all migrations from scratch.

This logic lives in a helper function called before `MIGRATOR.run(&pool)` inside
`create_database_tables()`.

### Effect on `ensure_schema()` / `create_database_tables()`

After this subissue, `SchemaMigrator::create_database_tables()` calls the legacy-bootstrap
helper and then `MIGRATOR.run(&pool)` instead of issuing raw DDL. `drop_database_tables()`
(used only in tests) must also drop the `_sqlx_migrations` and `torrent_aggregate_metrics`
tables (fixing the pre-existing omission) so that the drop/create cycle used in the test suite
works correctly.

## Tasks

### Task 1 — Verify existing migration files

The three migration files already exist under `packages/tracker-core/migrations/`. Verify that
their SQL content is correct and consistent with the current schema produced by the hardcoded
DDL in `1525-05`. Do not change existing file timestamps or names. Fix content only if a
discrepancy is found.

**Outcome**: all three migration files are verified correct; nothing else changes yet.

### Task 2 — Enable `sqlx` `macros` feature and add `MIGRATOR` statics

In `packages/tracker-core/Cargo.toml`, add the `macros` feature to the existing `sqlx`
dependency:

```toml
sqlx = { version = "...", features = ["sqlite", "mysql", "macros", "runtime-tokio-native-tls"] }
```

In each driver file add a static migrator:

```rust
use sqlx::migrate::Migrator;

// SQLite driver
static MIGRATOR: Migrator = sqlx::migrate!("migrations/sqlite");

// MySQL driver
static MIGRATOR: Migrator = sqlx::migrate!("migrations/mysql");
```

Add `Error::migration_error()` to `databases/error.rs` to wrap `sqlx::migrate::MigrateError`.

**Outcome**: project compiles with migration statics defined but not yet called.

### Task 3 — Wire migrations into `create_database_tables()` and `drop_database_tables()`

#### Legacy bootstrap helper

Add a private async helper function `bootstrap_legacy_schema` to each driver. This function
detects whether the database is in the legacy state (user-managed schema, no
`_sqlx_migrations` table) and, if so, fake-applies the three pre-existing migrations so that
`MIGRATOR.run()` can continue with only the new ones:

```rust
async fn bootstrap_legacy_schema(pool: &Pool) -> Result<(), Error> {
    // Check whether _sqlx_migrations already exists.
    let migrations_table_exists: bool = /* backend-appropriate query */;
    if migrations_table_exists {
        return Ok(());  // normal path — nothing to do here
    }

    // Check whether the legacy tables exist (whitelist is a reliable sentinel).
    let legacy_tables_exist: bool = /* backend-appropriate query */;
    if !legacy_tables_exist {
        return Ok(());  // fresh database — MIGRATOR.run() will handle it
    }

    // PRECONDITION GUARD: before fake-applying, verify that migration 2 (nullable
    // valid_until) and migration 3 (torrent_aggregate_metrics table) were applied.
    // If not, return a descriptive error rather than silently bootstrapping a broken schema.
    // SQLite: use `PRAGMA table_info(keys)` and `sqlite_master`.
    // MySQL: use `information_schema.columns` and `information_schema.tables`.
    let migration_2_applied: bool = /* check keys.valid_until is nullable */;
    let migration_3_applied: bool = /* check torrent_aggregate_metrics table exists */;
    if !migration_2_applied || !migration_3_applied {
        return Err(Error::migration_error(
            DRIVER,
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Legacy database is not fully migrated. Apply all three manual migrations \
                 listed in packages/tracker-core/migrations/README.md before upgrading to v4.",
            ),
        ));
    }

    // PRECONDITION: all three manual migrations have been verified as applied:
    //   (1) whitelist/torrents/keys tables exist (whitelist sentinel check above)
    //   (2) keys.valid_until is nullable (verified above)
    //   (3) torrent_aggregate_metrics table exists (verified above)
    // The v4 upgrade guide requires the user to have applied all three manual migrations
    // before upgrading to v4.
    MIGRATOR
        .ensure_migrations_table(pool)
        .await
        .map_err(|e| Error::migration_error(DRIVER, e))?;
    for migration in MIGRATOR.iter() {
        if migration.version <= 20_250_527_093_000 {
            // sqlx 0.8 does not expose a public `apply_fake()` API on `Migrator`.
            // Fake-apply by inserting directly into `_sqlx_migrations`. The `checksum`
            // field MUST equal the value embedded in the compiled binary (from
            // `migration.checksum`) so that subsequent `MIGRATOR.run()` calls pass the
            // checksum-verification step and do not raise a mismatch error.
            //
            // The INSERT uses `?` placeholders, valid for both SQLite and MySQL (this
            // function lives in the driver-specific file, not in shared code).
            sqlx::query(
                "INSERT INTO _sqlx_migrations \
                 (version, description, installed_on, success, checksum, execution_time) \
                 VALUES (?, ?, CURRENT_TIMESTAMP, TRUE, ?, 0)",
            )
            .bind(migration.version)
            .bind(migration.description.as_ref())
            .bind(migration.checksum.as_ref())
            .execute(pool)
            .await
            .map_err(|e| Error::migration_error(DRIVER, e))?;
        }
    }
    Ok(())
}
```

#### Updated `create_database_tables()`

```rust
async fn create_database_tables(&self) -> Result<(), Error> {
    bootstrap_legacy_schema(&self.pool).await?;
    MIGRATOR.run(&self.pool).await.map_err(|e| Error::migration_error(DRIVER, e))?;
    Ok(())
}
```

#### Updated `drop_database_tables()`

Fix the pre-existing omission: drop `torrent_aggregate_metrics` and `_sqlx_migrations` in
addition to the existing drops so that the test setup cycle (drop → create) works correctly.

Use `DROP TABLE IF EXISTS` for all five drops. This matches the reference implementation and
is the safer choice for test teardown (avoids errors on a partially torn-down database).

```rust
// Example using DROP TABLE IF EXISTS for all five drops:
sqlx::query("DROP TABLE IF EXISTS _sqlx_migrations").execute(&self.pool).await...?;
sqlx::query("DROP TABLE IF EXISTS torrent_aggregate_metrics").execute(&self.pool).await...?;
sqlx::query("DROP TABLE IF EXISTS whitelist").execute(&self.pool).await...?;
sqlx::query("DROP TABLE IF EXISTS torrents").execute(&self.pool).await...?;
sqlx::query("DROP TABLE IF EXISTS keys").execute(&self.pool).await...?;
```

#### Legacy bootstrap precondition guard

The `bootstrap_legacy_schema()` helper must verify the critical schema elements before
fake-applying migrations. If any element is absent, it must return an error rather than
silently bootstrapping a broken schema. Add the precondition checks described in the code
block above (migration 2 nullable check and migration 3 table existence check) and document
the verified state with a comment:

```rust
// PRECONDITION: all three manual migrations have been verified as applied:
//   (1) whitelist/torrents/keys tables exist (whitelist sentinel check above)
//   (2) keys.valid_until is nullable (verified above)
//   (3) torrent_aggregate_metrics table exists (verified above)
// The v4 upgrade guide requires the user to have applied all three manual migrations
// before upgrading to v4.
```

#### Update `migrations/README.md`

Update `packages/tracker-core/migrations/README.md` to replace the stale content (currently:
"We don't support automatic migrations yet") with accurate documentation covering:

- Migrations are now applied automatically on startup via `sqlx::migrate!()`.
- The `_sqlx_migrations` table tracks which migrations have run.
- To add a new migration: create a `.sql` file with the next timestamp in all applicable backend
  directories, following the history-alignment pattern.
- v4 upgrade requirement: users on a pre-v4 tracker must apply all three manual migrations before
  upgrading to v4. The automatic bootstrap handles the rest.
- **Migration file immutability**: once a migration file has been deployed, it must never be
  modified. `sqlx` records each migration's checksum in `_sqlx_migrations`; editing a committed
  migration file causes a checksum-mismatch error on the next startup for any database that has
  already applied that migration.

The `ensure_schema()` latch remains in place — it now guards the
`bootstrap_legacy_schema()` + `MIGRATOR.run()` sequence.

**Outcome**: `cargo test --workspace --all-targets` passes. Schema is owned by migration files.
The README accurately reflects the new automatic migration behavior.

### Task 4 — Validate migration behavior

Add or extend tests that verify:

- **Fresh database**: a single `create_database_tables()` call runs all migrations and
  leaves the database in the correct final schema state.
- **Idempotency**: calling `create_database_tables()` a second time on an already-migrated
  database is a no-op (all migrations already recorded in `_sqlx_migrations`).
- **Drop/create cycle**: `drop_database_tables()` followed by `create_database_tables()`
  produces a clean schema (all tables including `_sqlx_migrations` and
  `torrent_aggregate_metrics` are dropped and recreated).
- **Legacy bootstrap**: a database that has the pre-existing three tables (created without
  `_sqlx_migrations`) is correctly bootstrapped — `_sqlx_migrations` is created, the three
  migrations are marked fake-applied, and any new migrations are applied.
- **Partial-migration guard**: a database that has the schema tables but is missing
  `torrent_aggregate_metrics` (migration 3 not applied) must cause `bootstrap_legacy_schema()`
  to return an error, not silently proceed.

These tests can live alongside the existing behavioral tests in the driver `#[cfg(test)]`
modules.

## Out of Scope

- PostgreSQL migration files — those are added in subissue `1525-08`. The
  [PostgreSQL migration alignment](#postgresql-migration-alignment) section above specifies
  the history-alignment requirement: PostgreSQL must start from migration 1 (not a catch-up
  migration) to keep version history identical across all backends.
- Down migrations (rollback) — not needed at this stage.
- Handling legacy databases where not all three manual migrations were applied — the v4
  changelog must state that all three migrations must be applied before upgrading to v4.
  The legacy bootstrap path verifies this precondition and returns an error if it is not met
  (see the precondition guard above).
- **Migration file integrity check in CI** — `sqlx migrate check` (or an equivalent
  step that connects to a fresh database and verifies checksums) can detect if a deployed
  migration file has been edited after deployment. This requires a live database in CI and
  is a follow-up improvement. It is out of scope here but worth adding once a database
  service is reliably available in the CI pipeline (e.g., after subissue `1525-08` wires in
  the PostgreSQL service).

## Acceptance Criteria

- [ ] The three existing migration files under `migrations/sqlite/` and `migrations/mysql/` are
      confirmed correct and match the final schema produced by the hardcoded DDL in `1525-05`.
- [ ] `sqlx::migrate!()` (`macros` feature) is used in both drivers; no raw DDL remains in
      `create_database_tables()`.
- [ ] `drop_database_tables()` drops `_sqlx_migrations` **and** `torrent_aggregate_metrics`
      (fixing the pre-existing omission) so the test cycle works. All five drops use
      `DROP TABLE IF EXISTS`.
- [ ] `bootstrap_legacy_schema()` verifies that migrations 2 and 3 were applied before
      fake-applying, and returns a descriptive error if the precondition is not met.
- [ ] `Error::migration_error()` wraps `sqlx::migrate::MigrateError`.
- [ ] `packages/tracker-core/migrations/README.md` is updated to document automatic migration
      behavior and the v4 upgrade requirement.
- [ ] Guidance for `1525-08`: PostgreSQL migration files start from migration 1 following the
      history-alignment pattern, with the same filenames/timestamps as SQLite and MySQL.
- [ ] Legacy bootstrap: a database with the pre-existing tables but no `_sqlx_migrations` is
      correctly detected; the three pre-existing migrations are fake-applied; new migrations
      run normally.
- [ ] Fresh database: `create_database_tables()` runs all migrations from scratch via
      `MIGRATOR.run()`.
- [ ] Migration idempotency is verified by tests (second call is a no-op).
- [ ] Drop/create cycle is verified by tests (all tables cleaned up and recreated).
- [ ] Legacy bootstrap scenario is verified by a test (fully-migrated legacy database is
      bootstrapped correctly).
- [ ] Partial-migration guard is verified by a test (database missing `torrent_aggregate_metrics`
      causes an error rather than silent bootstrap).
- [ ] Existing behavioral tests continue to pass.
- [ ] The v4 changelog or upgrade guide documents the pre-upgrade requirement: apply all three
      manual migrations before upgrading to v4.
- [ ] Persistence benchmarking (see subissue `1525-03`) shows no regression against the committed
      baseline.
- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `linter all` exits with code `0`.

## References

- EPIC: `#1525`
- Subissue `1525-05`: `docs/issues/1525-05-migrate-sqlite-and-mysql-to-sqlx.md` — must be
  completed first
- Subissue `1525-03`: `docs/issues/1525-03-persistence-benchmarking.md` — benchmark baseline
- Reference PR: `#1695`
- Reference implementation branch: `josecelano:pr-1684-review` — see EPIC for checkout
  instructions (`docs/issues/1525-overhaul-persistence.md`)
- Reference files (migration files and driver wiring):
  - `packages/tracker-core/migrations/sqlite/`
  - `packages/tracker-core/migrations/mysql/`
  - `packages/tracker-core/src/databases/driver/sqlite.rs`
  - `packages/tracker-core/src/databases/driver/mysql.rs`
- Existing migration README: `packages/tracker-core/migrations/README.md`
