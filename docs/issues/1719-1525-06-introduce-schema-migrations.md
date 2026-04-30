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
and MySQL drivers backed by `sqlx`. `SchemaMigrator::create_database_tables()` is invoked
once from `databases::setup::initialize_database()` after the driver is built; subissue
`1525-05` explicitly chose **not** to use a per-method lazy `ensure_schema()` latch. The
current `create_database_tables()` issues raw `sqlx::query()` DDL. This subissue replaces
that raw DDL path with `sqlx::migrate!()`.

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
current `drop_database_tables()` already drops all four tables (`whitelist`, `torrents`,
`keys`, **and** `torrent_aggregate_metrics`) — there is no pre-existing omission. What is
missing is `_sqlx_migrations`, which does not exist today and will be introduced by this
subissue. All current drops use bare `DROP TABLE` (no `IF EXISTS`).

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
(used in tests and in the `axum-rest-tracker-api-server` `force_database_error` helper) must
also drop `_sqlx_migrations` (newly introduced by this subissue) and switch every drop to
`DROP TABLE IF EXISTS` so the drop/create cycle used by `databases::driver::tests::run_tests`
(create → drop → create) leaves a clean slate that `MIGRATOR.run()` can re-bootstrap as a
fresh database.

## Findings from current-code analysis (2026-04-30)

Review of `develop` (post-`1525-05`) before starting implementation. These items refine or
correct statements elsewhere in this spec; tasks below should be read with these in mind.

### F1. No `ensure_schema()` latch exists — and none is planned

Subissue `1525-05` explicitly decided not to introduce a per-method lazy schema latch (see
`docs/issues/1717-1525-05-migrate-sqlite-and-mysql-to-sqlx.md`: _"Do **not** use per-method
lazy schema checks (`ensure_schema()`)"_). `create_database_tables()` is called exactly once
from `databases::setup::initialize_database()`. Any references to an `ensure_schema()` latch
in earlier drafts of this spec are obsolete. Replace mentions of "the `ensure_schema()` latch
remains in place" with "`create_database_tables()` continues to be invoked once from
`initialize_database()`".

### F2. `drop_database_tables()` already drops `torrent_aggregate_metrics`

Both the SQLite and MySQL drivers in current code already drop all four tables. The spec's
claim that this is a "pre-existing omission" is incorrect. The only **new** drop required by
this subissue is `_sqlx_migrations`. Acceptance criteria below are reworded accordingly. The
`DROP TABLE IF EXISTS` switch (covering all five drops) remains a real change — current code
uses bare `DROP TABLE`.

### F3. Error construction follows a tuple-`From` pattern, not a constructor

All existing `sqlx`-error sites use `.map_err(|e| (e, DRIVER))?` and rely on
`impl From<(SqlxError, Driver)> for Error`. The proposed `Error::migration_error(driver,
source)` constructor breaks that convention. Preferred shape:

- Add a new `Error::MigrationError { source, driver }` variant.
- Add `impl From<(sqlx::migrate::MigrateError, Driver)> for Error`.
- Call sites then write `.map_err(|e| (e, DRIVER))?`, identical to every other driver call.

Update Task 2 and the bootstrap helper code in Task 3 to use this shape. The acceptance
criterion "`Error::migration_error()` wraps `MigrateError`" should be reworded as "a new
`Error::MigrationError` variant + `From<(MigrateError, Driver)>` impl wraps `MigrateError`".

### F4. `sqlx`'s `migrate` feature is already enabled transitively; only `macros` is missing

`cargo tree` confirms `sqlx-core` is built with the `migrate` feature already (so the
`sqlx::migrate::Migrator` and `MigrateError` types are reachable today). The required
addition in `packages/tracker-core/Cargo.toml` is the **`macros`** feature on `sqlx`, which
gates the compile-time `sqlx::migrate!()` macro. No other feature additions are needed.

### F5. SQLite migration 1 contains an invalid `#` comment

`packages/tracker-core/migrations/sqlite/20240730183000_torrust_tracker_create_all_tables.sql`
contains a Bash-style comment line (`# todo: rename to torrent_metrics`). SQLite's lexer does not
accept `#` as a comment introducer (only `--` and `/* … */`); only MySQL does. When
`MIGRATOR.run()` executes this file against SQLite, the statement parser is expected to
fail with a syntax error. **Action in Task 1**: replace `#` with `--` in the SQLite file
(and in the MySQL file as well, for consistency, since `--` is portable). Verify by running
the SQLite driver tests after the change.

### F6. MySQL migration 1 still uses `INT(10)` display-width syntax

MySQL 8.0 deprecated integer display-width attributes. `INT(10)` still parses but emits a
warning and is dropped from `SHOW CREATE TABLE` output, which can cause schema-comparison
noise. Not blocking for this subissue; flag as an optional cleanup or defer to subissue
`1525-07` (Rust ↔ SQL type alignment) where integer widths are revisited.

### F7. `keys.key` width is `VARCHAR(32)`, matches `AUTH_KEY_LENGTH`

Verified: `AUTH_KEY_LENGTH = 32` in `packages/tracker-core/src/authentication/key/mod.rs`.
MySQL migration 1 uses `VARCHAR(32)`, so the migration file matches the `format!`-built DDL
in the current driver. No discrepancy. Once migrations own the schema, the `format!` /
`AUTH_KEY_LENGTH` coupling in `mysql/schema_migrator.rs` disappears (the column width is
frozen in the migration file).

### F8. Other consumers of `drop_database_tables()` outside the test harness

`packages/axum-rest-tracker-api-server/tests/server/mod.rs::force_database_error` calls
`drop_database_tables()` to provoke query failures. After this subissue it will additionally
drop `_sqlx_migrations`. Behaviour is unchanged for the test (subsequent queries still
fail), but worth a sentence in the PR description.

### F9. `bootstrap_legacy_schema()` precondition queries — concrete forms

The spec describes the checks abstractly. Concrete queries to use:

- **`_sqlx_migrations` exists**
  - SQLite: `SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = '_sqlx_migrations'`
  - MySQL: `SELECT 1 FROM information_schema.tables WHERE table_schema = DATABASE() AND
table_name = '_sqlx_migrations'`
- **Legacy sentinel (`whitelist` exists)** — same shape as above with `name='whitelist'`.
- **Migration 2 applied (`keys.valid_until` is nullable)**
  - SQLite: `PRAGMA table_info(keys)` → row where `name='valid_until'` has `notnull = 0`.
  - MySQL: `SELECT is_nullable FROM information_schema.columns WHERE table_schema =
DATABASE() AND table_name = 'keys' AND column_name = 'valid_until'` → `'YES'`.
- **Migration 3 applied (`torrent_aggregate_metrics` exists)** — sentinel-table check, same
  shape as the first two.

Important ordering: check `_sqlx_migrations` existence with a raw query **before** calling
`MIGRATOR.ensure_migrations_table(pool)`, because the latter creates the table if absent and
would defeat the detection.

### F10. `apply_fake` SQL — confirm column types and key types in sqlx 0.8

`Migration::version` is `i64`, `Migration::description` is `Cow<'static, str>`, and
`Migration::checksum` is `Cow<'static, [u8]>`. Binding `&[u8]` for the checksum column works
in both backends. The `_sqlx_migrations` schema has columns
`(version BIGINT PK, description TEXT, installed_on TIMESTAMP, success BOOL, checksum BLOB,
execution_time BIGINT)` — verify this once during implementation by inspecting the table sqlx
creates against a fresh DB; if column types differ across backends, adjust the INSERT bind
types accordingly.

### F11. `database_setup` test cycle is the natural drop/create test

`packages/tracker-core/src/databases/driver/mod.rs::database_setup` already does
`create → drop → create`. After this subissue, the second `create` runs `MIGRATOR.run()` on
a database where everything (including `_sqlx_migrations`) was just dropped. No additional
test is needed for the drop/create cycle scenario beyond verifying that this existing test
still passes.

## Tasks

### Task 1 — Verify existing migration files

The three migration files already exist under `packages/tracker-core/migrations/`. Verify that
their SQL content is correct and consistent with the current schema produced by the hardcoded
DDL in `1525-05`. Do not change existing file timestamps or names. Fix content only if a
discrepancy is found.

Known issue to fix as part of this task (see finding F5): the SQLite (and MySQL) migration
`20240730183000_torrust_tracker_create_all_tables.sql` contains a Bash-style line
(`# todo: rename to ...torrent_metrics`). SQLite does not accept `#` line comments — replace `#` with `--` in
both backend files. This is the only content change expected; verify by running
`cargo test -p bittorrent-tracker-core run_sqlite_driver_tests` after Task 3 wires the
migrator in.

**Outcome**: all three migration files compile under `sqlx::migrate!()` for both backends;
the `#`-comment incompatibility is fixed.

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

Add a new `Error::MigrationError { source, driver }` variant to `databases/error.rs` and an
`impl From<(sqlx::migrate::MigrateError, Driver)> for Error` so the new code can keep the
established `.map_err(|e| (e, DRIVER))?` call pattern (see finding F3).

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
        // Build a `MigrateError` directly so the conversion goes through the
        // standard `From<(MigrateError, Driver)> for Error` impl introduced in Task 2.
        return Err((
            sqlx::migrate::MigrateError::Source(
                "Legacy database is not fully migrated. Apply all three manual migrations \
                 listed in packages/tracker-core/migrations/README.md before upgrading to v4."
                    .into(),
            ),
            DRIVER,
        )
            .into());
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
        .map_err(|e| (e, DRIVER))?;
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
            .map_err(|e| (e, DRIVER))?;
        }
    }
    Ok(())
}
```

#### Updated `create_database_tables()`

```rust
async fn create_database_tables(&self) -> Result<(), Error> {
    bootstrap_legacy_schema(&self.pool).await?;
    MIGRATOR.run(&self.pool).await.map_err(|e| (e, DRIVER))?;
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

`create_database_tables()` continues to be invoked once from
`databases::setup::initialize_database()` (no `ensure_schema()` latch — see finding F1).

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
- [ ] `drop_database_tables()` adds a drop for `_sqlx_migrations` (the only newly required
      drop — `torrent_aggregate_metrics` is already dropped today; see finding F2) and every
      drop is converted to `DROP TABLE IF EXISTS`.
- [ ] `bootstrap_legacy_schema()` verifies that migrations 2 and 3 were applied before
      fake-applying, and returns a descriptive error if the precondition is not met.
- [ ] A new `Error::MigrationError` variant plus `impl From<(sqlx::migrate::MigrateError,
    Driver)> for Error` wrap `MigrateError`, matching the existing tuple-`From` pattern
      used by every other `sqlx` error site (see finding F3).
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
