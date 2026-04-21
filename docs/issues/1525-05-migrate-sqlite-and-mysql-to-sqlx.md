# Subissue Draft for #1525-05: Migrate SQLite and MySQL Drivers to sqlx

## Goal

Move the existing SQL backends to a shared async `sqlx` substrate before adding PostgreSQL.

## Why

PostgreSQL should not be added as a special case. The existing SQL backends need to follow the same
async persistence model first so PostgreSQL can land on a common foundation.

## Proposed Branch

- `1525-05-migrate-sqlite-and-mysql-to-sqlx`

## Background

### Starting point

By the time this subissue is implemented, subissue `1525-04` will have split the monolithic
`Database` trait into four narrow sync traits (`SchemaMigrator`, `TorrentMetricsStore`,
`WhitelistStore`, `AuthKeyStore`) plus a `Database` aggregate supertrait with a blanket impl.
Consumers still hold `Arc<Box<dyn Database>>`.

The existing drivers (`Sqlite` in `driver/sqlite.rs`, `Mysql` in `driver/mysql.rs`) use
synchronous connection pools (`r2d2_sqlite`/`r2d2` for SQLite, the `mysql` crate for MySQL).
`build()` in `driver/mod.rs` calls `create_database_tables()` eagerly on startup.

### Migration strategy: green parallel → single switch commit

Rewriting both drivers at once while simultaneously making all four traits async would keep the
branch in a broken ("red") state for an extended period. Instead, this subissue uses a
**green parallel approach**:

1. Build the async infrastructure and new driver implementations alongside the existing sync code
   (Tasks 1–3). The branch compiles and all tests pass throughout these tasks.
2. Wire everything up and remove the old code in a single focused switch commit (Task 4). The
   branch is briefly in a red state only during this commit.

The technique is to put the async traits and new drivers in a temporary `databases/sqlx/`
submodule during Tasks 1–3. Task 4 moves them into place, updates consumers, and removes the sync
code.

### What changes in the drivers

The current drivers use blocking I/O and create the schema eagerly on construction. The new
`sqlx`-backed drivers:

- Use `SqlitePool` / `MySqlPool` with lazy `connect_lazy_with()`.
- Manage the schema with raw `sqlx::query()` DDL statements (`CREATE TABLE IF NOT EXISTS ...`),
  exactly mirroring what the current sync drivers do. `sqlx::migrate!()` and migration files are
  **not** introduced here — that is subissue `1525-06`.
- Run `create_database_tables()` lazily the first time any operation is called, protected by an
  `AtomicBool` + `Mutex` double-checked latch (`ensure_schema()`).
- All trait methods become `async fn` (via `async_trait`).

## Tasks

### Task 1 — Add sqlx infrastructure (no behavior change, stays green)

Add the async substrate without touching the existing drivers or traits.

#### Dependencies

In `packages/tracker-core/Cargo.toml`, add:

```toml
async-trait = "..."
sqlx = { version = "...", features = ["sqlite", "mysql", "runtime-tokio-native-tls"] }
tokio = { version = "...", features = ["full"] }   # if not already present with needed features
```

Keep `r2d2`, `r2d2_sqlite`, `rusqlite`, and the `mysql` crate — they are still needed by the old
drivers until Task 4.

#### Error handling

Update `databases/error.rs` so that `sqlx::Error` can be converted into the existing `Error` type.
Add the following constructor methods and their corresponding enum variants. Do not add
`Error::migration_error()` — that belongs to `1525-06`:

- `Error::connection_error()` — wraps connection failures (`sqlx::Error::Io`, pool errors,
  etc.). Introduce the `ConnectionError` variant.
- `Error::invalid_query()` — wraps type-decoding and encoding failures. Used by
  `decode_info_hash`, `decode_key`, `decode_valid_until`, and counter conversion helpers in
  the async drivers. Also used by the `decode_counter`/`encode_counter` helpers introduced in
  `1525-07` — introduce the variant here so `1525-07` requires no additional `error.rs`
  changes. Introduce the `InvalidQuery` variant.
- `Error::query_returned_no_rows()` — for `sqlx::Error::RowNotFound`. Introduce the
  `QueryReturnedNoRows` variant.
- `From<(sqlx::Error, Driver)>` — maps `sqlx::Error` variants to `ConnectionError`,
  `QueryReturnedNoRows`, or `InvalidQuery` based on error kind (see reference `error.rs`).

Do not change existing variants.

**Outcome**: `cargo test --workspace --all-targets` still passes. No behavior change.

### Task 2 — Implement async SQLite driver (stays green)

Create a new async SQLite driver in a parallel `databases/sqlx/` submodule without touching the
existing `databases/driver/sqlite.rs`.

#### New files

```text
packages/tracker-core/src/databases/sqlx/mod.rs   ← async trait definitions + AsyncDatabase aggregate
packages/tracker-core/src/databases/sqlx/sqlite.rs ← SqliteSqlx struct
```

#### Async trait definitions (`databases/sqlx/mod.rs`)

Define async versions of the four narrow traits. Use `async_trait` for object safety:

```rust
use async_trait::async_trait;

#[async_trait]
pub trait AsyncSchemaMigrator: Send + Sync {
    async fn create_database_tables(&self) -> Result<(), Error>;
    async fn drop_database_tables(&self) -> Result<(), Error>;
}

// ... AsyncTorrentMetricsStore, AsyncWhitelistStore, AsyncAuthKeyStore (same method
// signatures as their sync counterparts but with async fn)

pub trait AsyncDatabase:
    AsyncSchemaMigrator + AsyncTorrentMetricsStore + AsyncWhitelistStore + AsyncAuthKeyStore
{
}

impl<T> AsyncDatabase for T where
    T: AsyncSchemaMigrator + AsyncTorrentMetricsStore + AsyncWhitelistStore + AsyncAuthKeyStore
{
}
```

#### `SqliteSqlx` struct (`databases/sqlx/sqlite.rs`)

Mirrors the reference `Sqlite` in `driver/sqlite.rs` (PR branch):

```rust
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;

pub(crate) struct SqliteSqlx {
    pool: SqlitePool,
    schema_ready: AtomicBool,
    schema_lock: Mutex<()>,
}
```

Implement `AsyncSchemaMigrator`, `AsyncTorrentMetricsStore`, `AsyncWhitelistStore`, and
`AsyncAuthKeyStore` for `SqliteSqlx`. All SQL queries use `sqlx::query(...)`. Schema
initialization in `create_database_tables()` executes raw `CREATE TABLE IF NOT EXISTS ...`
statements via `sqlx::query()` — no `sqlx::migrate!()` in this step.

#### Tests

Add an inline `#[cfg(test)]` module in `databases/sqlx/sqlite.rs`. Use the shared
`databases/driver/tests::run_tests()` helper (or a new async equivalent) to run all behavioral
tests against `SqliteSqlx`. Use `torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database`
for the in-memory/temp-file path.

**Outcome**: `cargo test --workspace --all-targets` still passes. Old sync `Sqlite` driver
untouched.

### Task 3 — Implement async MySQL driver (stays green)

Create `packages/tracker-core/src/databases/sqlx/mysql.rs` with a `MysqlSqlx` struct mirroring
the same structure as `SqliteSqlx` but using `MySqlPool`. Schema initialization uses raw
`sqlx::query()` DDL — no `sqlx::migrate!()` in this step.

Implement the same four async traits. Add an inline `#[cfg(test)]` module that runs the shared
behavioral test suite against a real MySQL instance (via environment variable guard
`TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true`, consistent with existing MySQL test gating).

**Outcome**: `cargo test --workspace --all-targets` still passes. Old sync `Mysql` driver
untouched.

### Task 4 — Switch: replace sync traits with async, update consumers (brief red)

This task is a single focused commit. Steps within the commit:

1. **Rename async traits to canonical names**: rename `AsyncSchemaMigrator` → `SchemaMigrator`,
   `AsyncTorrentMetricsStore` → `TorrentMetricsStore`, etc. in `databases/sqlx/mod.rs`. Rename
   `AsyncDatabase` → `Database`. Move the trait definitions from `databases/sqlx/mod.rs` into
   `databases/mod.rs` (replacing the sync trait definitions). Move the driver files into the
   existing driver directory, overwriting the old sync drivers:
   `databases/sqlx/sqlite.rs` → `databases/driver/sqlite.rs` and
   `databases/sqlx/mysql.rs` → `databases/driver/mysql.rs`. Remove the now-empty
   `databases/sqlx/` submodule.

2. **Rename driver structs**: rename `SqliteSqlx` → `Sqlite`, `MysqlSqlx` → `Mysql`.

3. **Clean up old driver module helpers**: remove the sync test helpers from
   `databases/driver/mod.rs` that reference `Arc<Box<dyn Database>>` with sync methods; replace
   with async equivalents. (The old sync driver files at `databases/driver/sqlite.rs` and
   `databases/driver/mysql.rs` were already overwritten by the async drivers in step 1.)

4. **Update `databases/driver/mod.rs` `build()`**: the function no longer calls
   `create_database_tables()` eagerly (schema is now lazy). Update the return type if needed.

5. **Update `databases/setup.rs`**: `initialize_database()` constructs the new async `Sqlite` or
   `Mysql` and wraps in `Arc<Box<dyn Database>>` (type stays the same, traits are now async).

6. **Add `.await` at all consumer call sites**: every location that called a `Database` method
   synchronously now needs `.await`. The affected files are:
   - `statistics/persisted/downloads.rs` (`DatabaseDownloadsMetricRepository`)
   - `whitelist/repository/persisted.rs` (`DatabaseWhitelist`)
   - `whitelist/setup.rs`
   - `authentication/key/repository/persisted.rs` (`DatabaseKeyRepository`)
   - `authentication/handler.rs` (test helpers)
   - Any integration tests in `tests/`

7. **Remove unused dependencies**: remove `r2d2`, `r2d2_sqlite`, `rusqlite`, and the `mysql` crate
   from `tracker-core/Cargo.toml`. Run `cargo machete` to verify.

8. **Update mock usage**: `#[automock]` on the narrow traits generates async mocks via `mockall`.
   Note that `MockDatabase` was already removed in `1525-04` (the aggregate supertrait has no
   methods). The actual breakage surface in this switch commit is the four narrow-trait mocks:
   `MockSchemaMigrator`, `MockTorrentMetricsStore`, `MockWhitelistStore`, and `MockAuthKeyStore`.
   Any tests written against the **sync** versions of these mocks (from `1525-04`) will fail to
   compile after the switch because async `mockall` mocks use
   `.returning(|| Box::pin(async { Ok(()) }))` rather than `.returning(|| Ok(()))`. Find and
   update all such tests before declaring this task complete.

**Outcome**: `cargo test --workspace --all-targets` passes. `linter all` exits `0`. Sync drivers
and all `r2d2`/`rusqlite`/`mysql` dependencies are gone.

## Constraints

- Do not add PostgreSQL in this step.
- Do not introduce `sqlx::migrate!()`, migration files, or the `sqlx` `macros` feature — those
  are introduced in subissue `1525-06`.
- Do not change the SQL schema in this step (schema evolution is `1525-06`).
- Keep `Arc<Box<dyn Database>>` as the consumer-facing type; do not introduce the `Persistence`
  struct from the reference implementation (that is a separate concern).
- The lazy `ensure_schema()` latch must be correct under concurrent async access: use
  `AtomicBool` (Acquire/Release) + `Mutex` double-checked pattern as in the reference.

## Acceptance Criteria

- [ ] SQLite and MySQL drivers use `sqlx` with async trait methods.
- [ ] Schema initialization is lazy (`ensure_schema()` pattern) — no eager call in `build()`.
- [ ] Schema management uses raw `sqlx::query()` DDL; `sqlx::migrate!()` is not used.
- [ ] `r2d2`, `r2d2_sqlite`, `rusqlite`, and the `mysql` crate are removed from
      `tracker-core/Cargo.toml`.
- [ ] Existing behavior is preserved end-to-end.
- [ ] The branch compiles and all tests pass after each of Tasks 1–3 individually (verified by CI
      or manual `cargo test` run after each task).
- [ ] Persistence benchmarking (see subissue `1525-03`) shows no regression against the committed
      baseline.
- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `linter all` exits with code `0`.
- [ ] `cargo machete` reports no unused dependencies.

## Out of Scope

- PostgreSQL driver — that is subissue `1525-08`.
- `sqlx::migrate!()` and migration files — that is subissue `1525-06`.
- `async_trait` removal — the `async_trait` crate is required at MSRV 1.72 because
  async-fn-in-traits was stabilized in Rust 1.75. When the MSRV is raised to 1.75+, remove
  `async_trait` and replace `#[async_trait]` attribute usage with native async trait syntax.
  Track this as a follow-up when the MSRV is next bumped.

## References

- EPIC: `#1525`
- Subissue `1525-04`: `docs/issues/1525-04-split-persistence-traits.md` — must be completed first
- Subissue `1525-03`: `docs/issues/1525-03-persistence-benchmarking.md` — benchmark baseline
- Reference PR: `#1695`
- Reference implementation branch: `josecelano:pr-1684-review` — see EPIC for checkout
  instructions (`docs/issues/1525-overhaul-persistence.md`)
- Reference files (async driver implementations — note: the reference uses `sqlx::migrate!()`
  which is not adopted in this step; use raw DDL instead):
  - `packages/tracker-core/src/databases/driver/sqlite.rs`
  - `packages/tracker-core/src/databases/driver/mysql.rs`
  - `packages/tracker-core/src/databases/driver/mod.rs`
