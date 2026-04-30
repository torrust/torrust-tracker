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

Update Task 2 (where the variant is added) and the bootstrap helper code in Task 4 to use
this shape. The acceptance criterion "`Error::migration_error()` wraps `MigrateError`"
should be reworded as "a new `Error::MigrationError` variant + `From<(MigrateError,
Driver)>` impl wraps `MigrateError`".

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
only — MySQL accepts `#` as a line comment natively, and editing the MySQL file would
break immutability for installers who already applied it manually (see Q1.5). Verify by
running the SQLite driver tests after the change.

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

## Open questions (from implementer, 2026-04-30)

The following questions should be resolved before implementation starts. Please reply
inline below each question.

### Q1 — Editing migration files vs. immutability rule

Task 1 instructs us to fix content if a discrepancy is found (F5 found one: the `#`
comment in SQLite migration 1). But Task 3 also states:

> **Migration file immutability**: once a migration file has been deployed, it must
> never be modified … editing a committed migration file causes a checksum-mismatch
> error on the next startup.

The three migration files were "deployed" historically (users were told to run them
manually), but no tracker has ever called `MIGRATOR.run()` on them, so no
`_sqlx_migrations` row exists yet and there is no checksum to mismatch. My reading is
that editing them is safe **this once**, before the migrator is wired in, and the
immutability rule applies from this subissue forward. Confirm?

**Reply:**

The migration "packages/tracker-core/migrations/sqlite/20240730183000_torrust_tracker_create_all_tables.sql" emulates the initial database setup. Then the other two migrations:

- packages/tracker-core/migrations/sqlite/20240730183500_torrust_tracker_keys_valid_until_nullable.sql
- packages/tracker-core/migrations/sqlite/20250527093000_torrust_tracker_new_torrent_aggregate_metrics_table.sql

were added when we needed to make some changes. However we notified users to run them manually because there
was not migrations at that time. At the same time the hardcoded SQL queries were changed, but that was a safe change because they were executed only if the tables did not exist. WE can assume all users will be in one of these two situations:

- A new tracker installation, empty database
- An existing tracker installation, with the three tables already created but no \_sqlx_migrations table. However we cal also assume all migrations were applied manually.

In both cases we have to keep the same migrations so all installations have the same migration history, so we need to keep those migrations files. So they are immutable. The new migrations will be also immutable. The reason is we do not know is users are installing the "develop" branch, so once we merge a new migration in the "develop" branch we cannot change it.

So in the new scenario we have to run those 2 migrations only if the DB schema is still empty (fresh DB installation). If the schema is not empty we have to mark those 3 migrations as executed.

### Q2 — F6 (`INT(10)` cleanup): do it here or defer?

I propose deferring the `INT(10)` → `INT` cleanup to subissue 1525-07
(type-alignment), keeping this PR focused on wiring migrations. Confirm defer?

**Reply:**

Yes, changes in DB and Rust types to align them must be deferred to the next subissue, because they require schema changes that must be delivered through migrations. So we need to keep the `INT(10)` in the migration files for now, and we can change it in the next subissue when we align Rust and SQL types.

### Q3 — Legacy-bootstrap test: SQLite-only or both backends?

To test `bootstrap_legacy_schema()` I need to: pre-create the four tables with raw DDL
matching the post-migration-3 state, run the bootstrap helper, then assert
`_sqlx_migrations` ends up populated with the three rows at the right checksums.

This is cheap on SQLite (in-memory). For MySQL it requires the testcontainer harness
gated behind the existing MySQL driver-test environment variable. Acceptable plan:

- Add the legacy-bootstrap test only for **SQLite** in the always-on test suite.
- Cover MySQL with the same scenario inside the gated `run_mysql_driver_tests` path.

Confirm, or do you want both backends in the always-on suite?

**Reply:**

We should do it for all databases. It's the only way to verify it works. That could be a good documentation for what we had before adding migrations.

### Q4 — Partial-migration guard test: same question as Q3

Same scope question for the partial-migration error case (some legacy tables present,
others not): SQLite-only in the always-on suite, MySQL inside the gated path?

**Reply:**

If there is at least one legacy legacy table, but others are missing we assume a corrupted DB and stop executions with an error concrete informative error message. We do not need to check that the tables have the correct definition, the application will fail later running newer migrations or running some queries.

### Q5 — Where does the v4 changelog / upgrade-guide entry go?

Acceptance criterion: _"The v4 changelog or upgrade guide documents the pre-upgrade
requirement"_. There is no `CHANGELOG.md` or upgrade guide in the repo today. Pick one:

- (a) Create a new `docs/upgrade-to-v4.md` and add the entry there.
- (b) Document the pre-upgrade requirement only in
  `packages/tracker-core/migrations/README.md` and mark the changelog item as out of
  scope (tracked separately in a follow-up issue).
- (c) Create a stub changelog/upgrade-guide file for someone else to expand later.

**Reply:**

This is not a breaking change, we have to document it inside the package. Since migrations are
going to be executed automatically and it's compatible with any well-formed database, we can just document it in the `packages/tracker-core/migrations/README.md` file. We can add a section "Upgrade from older versions" and explain the requirement there.

### Q6 — `MigrateError::Source` vs. a new `Error` variant for precondition failures

In F3 / Task 3 the precondition guard returns an error if legacy tables don't match the
post-migration-3 state. I planned to wrap a human message in
`sqlx::migrate::MigrateError::Source(... .into())` so it flows through
`From<(MigrateError, Driver)>`. If sqlx 0.8's `MigrateError::Source` doesn't accept a
`Box<dyn Error + Send + Sync>` cleanly, the fallback is to add a dedicated
`Error::LegacyDatabaseNotMigrated { driver, reason }` variant directly. OK to decide
during implementation, or do you want a specific choice now?

**Reply:**

We can decide during implementation, I don't have a string preference for now.

### Q7 — Commit granularity (single PR, multiple commits)

Plan: one PR (this branch), four commits — one per task:

1. Task 1 — fix `#` → `--` comments in SQLite migration 1 only (do not edit MySQL migration 1).
2. Task 2 — add sqlx `macros` feature, `MIGRATOR` statics, `Error::MigrationError`
   variant + `From` impl. (Compiles; nothing called yet.)
3. Task 3 — wire `bootstrap_legacy_schema()` + `MIGRATOR.run()` into
   `create_database_tables()`, update `drop_database_tables()` (`IF EXISTS` everywhere
   plus `_sqlx_migrations`), update `migrations/README.md`.
4. Task 4 — add tests (fresh DB, idempotency, legacy bootstrap, partial-migration
   guard).

Acceptable, or do you prefer different granularity (one task per PR, or fewer/larger
commits)?

**Reply:**

One PR is fine. I guess the way I would split it would be something like:

1. Add the scaffolding to run migrations without running them yet.
2. Make the change in both drivers assuming fresh empty databases (including tests)
3. Implement the patch for backward compatibility (including tests)

### Q1.5 — Follow-up: residual conflict between Q1 immutability rule and the `#` comment in SQLite migration 1

Your Q1 reply states that the three existing migration files are immutable. But finding F5
documents that `packages/tracker-core/migrations/sqlite/20240730183000_torrust_tracker_create_all_tables.sql`
line 7 contains:

```sql
# todo: rename to `torrent_metrics`
```

SQLite does not accept `#` line comments. As soon as we wire `MIGRATOR.run()` in for a
fresh install (one of the two scenarios you listed), `sqlx` will execute this file and
SQLite will return a syntax error. This means the file as currently committed cannot be
shipped as-is once the migrator is enabled.

The pragmatic resolution: this PR ships the migrator. Before this PR, no installation has
ever had a `_sqlx_migrations` row referencing this file (the migrator has never been
wired in), so fixing the `#` → `--` in this PR causes zero checksum-mismatch errors in
the field. The immutability rule then kicks in from the moment this PR merges.

Three options:

- (a) Fix `#` → `--` in this PR as part of "Step 2 — Fresh-install path". Document it as a
  one-time pre-shipment correction in the commit message and in `migrations/README.md`.
- (b) Add a NEW migration on top (e.g. `20260501000000_fix_create_all_tables_comment.sql`)
  that drops and recreates the table — strictly correct under immutability but heavyweight
  for a comment fix and risks production data loss if anyone runs it in error.
- (c) Delete the `#` comment line entirely (still a content edit, same caveat as option a).

I recommend (a). Confirm the choice (or pick another).

**Reply:**

That is not an easy change because we have to update the code. We can simply document it as a refactoring proposal to be implemented in the future. We can include that proposal in the packages/tracker-core/docs folder in a new markdown file.

## Tasks

Implementation is split into **three phases** (one commit per phase, in the same PR; see Q7):

1. **Scaffolding** — add the `sqlx` `macros` feature, the `MIGRATOR` statics, the new
   `Error::MigrationError` variant + `From` impl, and fix the SQLite-only `#`-comment in
   migration 1. No call to `MIGRATOR.run()` yet, so no behaviour change.
2. **Fresh-install path** — wire `MIGRATOR.run()` into `create_database_tables()` and
   convert all `drop_database_tables()` statements to `DROP TABLE IF EXISTS`, plus add
   `_sqlx_migrations`. Add tests for fresh DB, idempotency, drop/create cycle.
3. **Legacy bootstrap path** — add `bootstrap_legacy_schema()` to handle pre-v4
   installations that already have the four legacy tables but no `_sqlx_migrations`. Add
   tests for legacy bootstrap and the partial-migration guard.

### Task 1 — Fix the SQLite-only `#` comment in migration 1

The three existing migration files are **immutable from now on** (Q1): once this PR ships
the migrator, editing any of them would cause checksum-mismatch errors in the field. This
is our **one and only** chance to correct content before the migrator is wired in.

The only correction needed (finding F5):
`packages/tracker-core/migrations/sqlite/20240730183000_torrust_tracker_create_all_tables.sql`
contains a `#`-prefixed TODO line. SQLite does not accept `#` as a line-comment marker, so
`sqlx::migrate!()` would fail to parse the file on every fresh install. Fix is a single
character swap (Q1.5):

```diff
-# todo: rename to `torrent_metrics`
+-- todo: rename to `torrent_metrics`
```

The MySQL counterpart is **not** edited — MySQL accepts `#` as a line comment natively, and
editing it would also break immutability for any installer who already manually applied it.

The table-rename TODO (`metrics` → `torrent_metrics`) is intentionally left as a comment
for a future change — the table currently holds only metrics but may grow other fields, so
the rename is deferred until a real driver requires it.

**Outcome**: `sqlx::migrate!("migrations/sqlite")` parses all three files cleanly.

### Task 2 — Scaffolding: enable `sqlx` `macros` feature and add `MIGRATOR` statics

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

For the partial-migration error case (Q4), the implementer may either reuse `MigrateError`
(e.g. `MigrateError::Source(...)`) or add a dedicated `Error::LegacyDatabaseNotMigrated
{ driver, reason }` variant — Q6 leaves this to implementation taste.

**Outcome**: project compiles with migration statics defined but not yet called. No
behaviour change.

### Task 3 — Fresh-install path: wire `MIGRATOR.run()` and update `drop_database_tables()`

#### Updated `create_database_tables()` (fresh-install only — legacy bootstrap added in Task 4)

```rust
async fn create_database_tables(&self) -> Result<(), Error> {
    MIGRATOR.run(&self.pool).await.map_err(|e| (e, DRIVER))?;
    Ok(())
}
```

#### Updated `drop_database_tables()`

Add a drop for `_sqlx_migrations` (the only newly required drop — `torrent_aggregate_metrics`
is already dropped today; see finding F2). Convert every drop to `DROP TABLE IF EXISTS` for
safer test teardown.

```rust
sqlx::query("DROP TABLE IF EXISTS _sqlx_migrations").execute(&self.pool).await...?;
sqlx::query("DROP TABLE IF EXISTS torrent_aggregate_metrics").execute(&self.pool).await...?;
sqlx::query("DROP TABLE IF EXISTS whitelist").execute(&self.pool).await...?;
sqlx::query("DROP TABLE IF EXISTS torrents").execute(&self.pool).await...?;
sqlx::query("DROP TABLE IF EXISTS keys").execute(&self.pool).await...?;
```

#### Update `migrations/README.md`

Replace the stale "We don't support automatic migrations yet" content with documentation
covering (Q5):

- Migrations are now applied automatically on startup via `sqlx::migrate!()`.
- The `_sqlx_migrations` table tracks which migrations have run.
- To add a new migration: create a `.sql` file with the next timestamp in all applicable
  backend directories, following the history-alignment pattern.
- **Upgrade from older versions** (formerly "v4 upgrade requirement"): users on a pre-v4
  tracker must have applied all three manual migrations before upgrading. The automatic
  bootstrap (Task 4) handles the `_sqlx_migrations` row insertion. This goes only in this
  README — there is no separate `CHANGELOG.md` or upgrade guide for v4.
- **Migration file immutability**: once a migration file has been deployed, it must never
  be modified. `sqlx` records each migration's checksum in `_sqlx_migrations`; editing a
  committed migration file causes a checksum-mismatch error on the next startup for any
  database that has already applied that migration.

#### Tests added in this phase

- **Fresh database**: a single `create_database_tables()` call runs all migrations and
  leaves the database in the correct final schema state. Both backends.
- **Idempotency**: a second `create_database_tables()` call is a no-op. Both backends.
- **Drop/create cycle**: covered by the existing `databases::driver::tests::database_setup`
  harness (see F11) — verify it still passes.

**Outcome**: fresh installs work end-to-end via `MIGRATOR.run()`. Pre-v4 installs would still
fail at this point — that is fixed in Task 4.

### Task 4 — Legacy bootstrap path

Add a private async helper function `bootstrap_legacy_schema` to each driver. This function
detects whether the database is in the legacy state (user-managed schema, no
`_sqlx_migrations` table) and, if so, fake-applies the three pre-existing migrations so that
`MIGRATOR.run()` can continue with only the new ones (Q3, Q4):

```rust
const LEGACY_TABLES: &[&str] = &[
    "whitelist",
    "torrents",
    "keys",
    "torrent_aggregate_metrics",
];

async fn bootstrap_legacy_schema(pool: &Pool) -> Result<(), Error> {
    // Check whether _sqlx_migrations already exists.
    let migrations_table_exists: bool = /* backend-appropriate query */;
    if migrations_table_exists {
        return Ok(());  // normal path — nothing to do here
    }

    // Count which of the four expected legacy tables are present.
    // SQLite: query sqlite_master.
    // MySQL: query information_schema.tables filtered by DATABASE().
    let present_legacy_tables: usize = /* backend-appropriate query */;

    if present_legacy_tables == 0 {
        return Ok(());  // fresh database — MIGRATOR.run() will handle it
    }

    if present_legacy_tables < LEGACY_TABLES.len() {
        // PRECONDITION GUARD (Q4): some legacy tables exist but not all four.
        // We treat this as a corrupted/partially-migrated database and stop with a
        // descriptive error. We do NOT verify column-level structure — if the user
        // has all four tables we trust the upgrade-guide precondition; subsequent
        // queries will surface any structural problem.
        return Err(/* MigrateError::Source(...) or Error::LegacyDatabaseNotMigrated — see Q6 */);
    }

    // PRECONDITION: all four legacy tables exist. Per the upgrade guide in
    // packages/tracker-core/migrations/README.md the user has applied all three
    // manual migrations before upgrading to v4.
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

#### Updated `create_database_tables()` (full version)

```rust
async fn create_database_tables(&self) -> Result<(), Error> {
    bootstrap_legacy_schema(&self.pool).await?;
    MIGRATOR.run(&self.pool).await.map_err(|e| (e, DRIVER))?;
    Ok(())
}
```

`create_database_tables()` continues to be invoked once from
`databases::setup::initialize_database()` (no `ensure_schema()` latch — see finding F1).

#### Tests added in this phase (Q3, Q4 — both backends)

- **Legacy bootstrap (SQLite + MySQL)**: pre-create the four tables with raw DDL matching
  the post-migration-3 state, run `bootstrap_legacy_schema()` followed by `MIGRATOR.run()`,
  then assert `_sqlx_migrations` is populated with the three rows at the embedded
  checksums and that a follow-up `MIGRATOR.run()` is a no-op.
- **Partial-migration guard (SQLite + MySQL)**: pre-create only some of the four legacy
  tables (e.g. `whitelist` and `torrents` but not `keys` or `torrent_aggregate_metrics`)
  and assert `bootstrap_legacy_schema()` returns the descriptive error rather than
  silently fake-applying. We do **not** assert column-level details.

MySQL coverage uses the same gated path as the existing driver tests (the env-var-gated
`run_mysql_driver_tests` setup); SQLite coverage runs in the always-on suite.

These tests live alongside the existing behavioral tests in the driver `#[cfg(test)]`
modules.

**Outcome**: `cargo test --workspace --all-targets` passes for SQLite, and the gated MySQL
suite passes when MySQL is available. Schema is fully owned by migration files.

## Out of Scope

- PostgreSQL migration files — those are added in subissue `1525-08`. The
  [PostgreSQL migration alignment](#postgresql-migration-alignment) section above specifies
  the history-alignment requirement: PostgreSQL must start from migration 1 (not a catch-up
  migration) to keep version history identical across all backends.
- Down migrations (rollback) — not needed at this stage.
- Handling legacy databases where not all three manual migrations were applied — the
  upgrade-from-older-versions section in `packages/tracker-core/migrations/README.md`
  states that all three migrations must be applied before upgrading. The partial-migration
  guard returns an error if the precondition is not met (see Task 4).
- `INT(10)` → `INT` cleanup in the MySQL migration file (finding F6) — deferred to subissue
  `1525-07` together with the rest of the Rust↔SQL type alignment work (Q2).
- Renaming `metrics` → `torrent_metrics` (the TODO comment kept in migration 1) — deferred
  until a real driver requires the rename and the table's purpose is settled (Q1.5).
- **Migration file integrity check in CI** — `sqlx migrate check` (or an equivalent
  step that connects to a fresh database and verifies checksums) can detect if a deployed
  migration file has been edited after deployment. This requires a live database in CI and
  is a follow-up improvement. It is out of scope here but worth adding once a database
  service is reliably available in the CI pipeline (e.g., after subissue `1525-08` wires in
  the PostgreSQL service).

## Acceptance Criteria

- [ ] The SQLite migration 1 (`#` → `--`) is the only existing-file edit; MySQL migration 1
      and the other four files are byte-for-byte unchanged (Q1, Q1.5).
- [ ] `sqlx::migrate!()` (`macros` feature) is used in both drivers; no raw DDL remains in
      `create_database_tables()`.
- [ ] `drop_database_tables()` adds a drop for `_sqlx_migrations` (the only newly required
      drop — `torrent_aggregate_metrics` is already dropped today; see finding F2) and every
      drop is converted to `DROP TABLE IF EXISTS`.
- [ ] `bootstrap_legacy_schema()` accepts "all four legacy tables present" as the only
      success precondition; if 1–3 of them exist it returns a descriptive error (Q4).
- [ ] A new `Error::MigrationError` variant plus `impl From<(sqlx::migrate::MigrateError,
Driver)> for Error` wrap `MigrateError`, matching the existing tuple-`From` pattern
      used by every other `sqlx` error site (see finding F3).
- [ ] `packages/tracker-core/migrations/README.md` is updated to document automatic migration
      behaviour, migration-file immutability, and the upgrade-from-older-versions requirement
      (apply all three manual migrations first). No separate `CHANGELOG.md` or upgrade guide
      is created (Q5).
- [ ] Guidance for `1525-08`: PostgreSQL migration files start from migration 1 following the
      history-alignment pattern, with the same filenames/timestamps as SQLite and MySQL.
- [ ] Fresh database: `create_database_tables()` runs all migrations from scratch via
      `MIGRATOR.run()` (verified by test on both backends).
- [ ] Migration idempotency is verified by tests (second call is a no-op) on both backends.
- [ ] Drop/create cycle continues to pass via the existing
      `databases::driver::tests::database_setup` harness (see F11).
- [ ] Legacy bootstrap scenario is verified by tests on both backends — SQLite in the
      always-on suite, MySQL in the gated `run_mysql_driver_tests` path (Q3).
- [ ] Partial-migration guard is verified by tests on both backends, same gating as above
      (Q4).
- [ ] Existing behavioral tests continue to pass.
- [ ] Persistence benchmarking (see subissue `1525-03`) shows no regression against the
      committed baseline.
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
