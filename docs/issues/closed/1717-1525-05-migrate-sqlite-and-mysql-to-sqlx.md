---
doc-type: issue
issue-type: task
status: done
priority: p1
github-issue: 1717
spec-path: docs/issues/closed/1717-1525-05-migrate-sqlite-and-mysql-to-sqlx.md
branch: 1525-05-migrate-sqlite-and-mysql-to-sqlx
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

Subissue `1525-04` has already been merged into `develop` (it is included in this branch).
It split the monolithic `Database` trait into four narrow sync traits (`SchemaMigrator`,
`TorrentMetricsStore`, `WhitelistStore`, `AuthKeyStore`) plus a `Database` aggregate supertrait
with a blanket impl. Consumers still hold `Arc<Box<dyn Database>>`.

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

### Decision update (2026-04-29)

After implementation review, we decided to keep **eager schema initialization** in this subissue
for operational clarity and parity with the existing sync drivers:

- Do **not** use per-method lazy schema checks (`ensure_schema()`).
- Keep explicit startup initialization (`create_database_tables()`) in setup/factory wiring.
- Keep using raw `sqlx::query()` DDL in this subissue; migration tooling stays in `1525-06`.

This decision also applies to Task 4 (switch commit): keep eager initialization there as well.

### What changes in the drivers

The current drivers use blocking I/O and create the schema eagerly on construction. The new
`sqlx`-backed drivers:

- Use `SqlitePool` / `MySqlPool` with lazy `connect_lazy_with()`.
- Manage the schema with raw `sqlx::query()` DDL statements (`CREATE TABLE IF NOT EXISTS ...`),
  exactly mirroring what the current sync drivers do. `sqlx::migrate!()` and migration files are
  **not** introduced here — that is subissue `1525-06`.
- Keep schema initialization eager via setup/factory initialization (`create_database_tables()`).
- All trait methods become `async fn` (via `async_trait`).

## Tasks

### Task 1 — Add sqlx infrastructure (no behavior change, stays green)

Add the async substrate without touching the existing drivers or traits.

#### Dependencies

In `packages/tracker-core/Cargo.toml`, add:

```toml
async-trait = "*"   # latest compatible with MSRV 1.72
sqlx = { version = "*", features = ["sqlite", "mysql", "runtime-tokio-native-tls"] }   # latest compatible
tokio = { version = "*", features = ["full"] }   # latest compatible; if not already present with needed features
```

Use the latest crate versions compatible with MSRV 1.72.

Keep `r2d2`, `r2d2_sqlite`, `rusqlite`, and the `mysql` crate — they are still needed by the old
drivers until Task 4.

#### Error handling

Update `databases/error.rs` so that `sqlx::Error` can be converted into the existing `Error`
type. The variants `ConnectionError`, `InvalidQuery`, and `QueryReturnedNoRows` **already exist**
in `error.rs`; do not re-introduce them. The only required change is:

- Broaden `ConnectionError`: its `source` field currently wraps `LocatedError<'static, UrlError>`
  (MySQL-specific). Generalize it to `LocatedError<'static, dyn std::error::Error + Send + Sync>`
  so it can hold any connection-level error from sqlx as well.
- Add `From<(sqlx::Error, Driver)>` — maps `sqlx::Error` variants to `ConnectionError`,
  `QueryReturnedNoRows`, or `InvalidQuery` based on error kind (see reference `error.rs`). Do not
  add `Error::migration_error()` — that belongs to `1525-06`.

Do not change any other existing variants. The `ConnectionPool` variant (wraps `r2d2::Error`) is
removed in Task 4 together with the `r2d2` dependency.

**Outcome**: `cargo test --workspace --all-targets` still passes. No behavior change.

### Task 2 — Implement async SQLite driver (stays green)

Create a new async SQLite driver in a parallel `databases/sqlx/` submodule without touching the
existing `databases/driver/sqlite/` subdirectory.

> **Note**: post-1525-04 the sync drivers are already split into per-trait files. The actual
> existing layout is:
>
> ```text
> databases/driver/sqlite/mod.rs
> databases/driver/sqlite/schema_migrator.rs
> databases/driver/sqlite/torrent_metrics_store.rs
> databases/driver/sqlite/whitelist_store.rs
> databases/driver/sqlite/auth_key_store.rs
> ```
>
> The async parallel module must mirror this layout.

#### New files

```text
packages/tracker-core/src/databases/sqlx/mod.rs              ← async trait definitions + AsyncDatabase aggregate
packages/tracker-core/src/databases/sqlx/sqlite/mod.rs       ← SqliteSqlx struct + pool/latch
packages/tracker-core/src/databases/sqlx/sqlite/schema_migrator.rs
packages/tracker-core/src/databases/sqlx/sqlite/torrent_metrics_store.rs
packages/tracker-core/src/databases/sqlx/sqlite/whitelist_store.rs
packages/tracker-core/src/databases/sqlx/sqlite/auth_key_store.rs
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

Create a `packages/tracker-core/src/databases/sqlx/mysql/` subdirectory mirroring the same
per-trait file layout as `databases/sqlx/sqlite/` (i.e. `mod.rs`, `schema_migrator.rs`,
`torrent_metrics_store.rs`, `whitelist_store.rs`, `auth_key_store.rs`) but using `MySqlPool`. Schema initialization uses raw
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
   `databases/traits/` (replacing the sync trait definitions in
   `databases/traits/schema.rs`, `databases/traits/torrent_metrics.rs`,
   `databases/traits/whitelist.rs`, `databases/traits/auth_keys.rs`).
   Move the driver subdirectories, overwriting the old sync drivers:
   `databases/sqlx/sqlite/` → `databases/driver/sqlite/` and
   `databases/sqlx/mysql/` → `databases/driver/mysql/`.
   Remove the now-empty `databases/sqlx/` submodule.

2. **Rename driver structs**: rename `SqliteSqlx` → `Sqlite`, `MysqlSqlx` → `Mysql`.

3. **Clean up `databases/driver/mod.rs`**: remove the sync test helpers that call trait methods
   without `.await`; replace with async equivalents.

4. **Update `databases/setup.rs` — `initialize_database()`**: this function already returns
   `DatabaseStores` (a struct of four `Arc<dyn XxxStore>` fields, one per narrow trait — not
   `Arc<Box<dyn Database>>`). Keep eager `create_database_tables()` during initialization.
   No return-type change is needed.

5. **Add `.await` at all consumer call sites**: every location that called a narrow-trait method
   synchronously now needs `.await`. The affected files are:
   - `statistics/persisted/downloads.rs` (`DatabaseDownloadsMetricRepository`)
   - `whitelist/repository/persisted.rs` (`DatabaseWhitelist`)
   - `whitelist/setup.rs`
   - `authentication/key/repository/persisted.rs` (`DatabaseKeyRepository`)
   - `authentication/handler.rs` (test helpers)
   - `src/bin/persistence_benchmark/driver_bench/` and
     `src/bin/persistence_benchmark/driver_bench/operations/` (benchmark binary)
   - Any integration tests in `tests/`

6. **Remove unused dependencies**: remove `r2d2`, `r2d2_sqlite`, `rusqlite`, and `r2d2_mysql`
   from `tracker-core/Cargo.toml`. Also remove the `ConnectionPool` error variant and its
   `From<(r2d2::Error, Driver)>` impl from `databases/error.rs`. Run `cargo machete` to verify.

7. **Update mock usage**: `#[automock]` on the narrow traits generates async mocks via `mockall`.
   Note that `MockDatabase` was already removed in `1525-04` (the aggregate supertrait has no
   methods). The actual breakage surface in this switch commit is the four narrow-trait mocks:
   `MockSchemaMigrator`, `MockTorrentMetricsStore`, `MockWhitelistStore`, and `MockAuthKeyStore`.
   Any tests written against the **sync** versions of these mocks (from `1525-04`) will fail to
   compile after the switch because async `mockall` mocks use
   `.returning(|| Box::pin(async { Ok(()) }))` rather than `.returning(|| Ok(()))`. Find and
   update all such tests before declaring this task complete.

**Outcome**: `cargo test --workspace --all-targets` passes. `linter all` exits `0`. Sync drivers
and all `r2d2`/`rusqlite`/`mysql` dependencies are gone.

### Task 5 — Remove sync-to-async runtime bridges (cleanup follow-up)

During Task 4, some sync wrappers were introduced to keep existing sync consumers working
while trait methods became async (helpers named `block_on_current_or_new_runtime`).
These wrappers are a transitional compatibility mechanism and should be removed.

This task migrates remaining sync call paths to native async end-to-end:

1. Make repository/service methods async where they call async persistence traits.
2. Propagate `.await` through callers instead of blocking at lower layers.
3. Remove all `block_on_current_or_new_runtime` helpers from tracker-core modules.
4. Keep runtime ownership at application boundaries only (no nested runtime creation).
5. Preserve eager schema initialization behavior while using async initialization paths.

**Outcome**: no `block_on_current_or_new_runtime` helper remains; persistence interactions
are fully async from call sites to drivers; tests, linters, and benchmarks still pass.

### Task 6 — Remove legacy persistence surface and temporary sqlx staging tree

The branch still contains a mixed layout:

- canonical runtime code under `packages/tracker-core/src/databases/driver/` and
  `packages/tracker-core/src/databases/traits/`
- temporary migration staging code under `packages/tracker-core/src/databases/sqlx/`
- legacy compatibility dependencies and error conversions that were expected to disappear in the
  switch commit

This task finishes the structural cleanup so the repository reflects a single persistence model.

1. Remove the temporary staging subtree under `packages/tracker-core/src/databases/sqlx/`,
   including its nested `driver/` and `traits/` directories.
2. Ensure `packages/tracker-core/src/databases/driver/` contains only the canonical sqlx-backed
   implementations that remain in use.
3. Ensure `packages/tracker-core/src/databases/traits/` contains only the canonical async trait
   definitions that remain in use.
4. Remove leftover legacy compatibility code tied to the pre-sqlx drivers, including obsolete
   error conversions and type references.
5. Remove obsolete dependencies from `packages/tracker-core/Cargo.toml`: `r2d2`, `r2d2_sqlite`,
   `rusqlite`, and `r2d2_mysql`.
6. Regenerate lockfile state as needed and confirm `cargo machete` still passes.

**Outcome**: there is one canonical async persistence surface only; the temporary `databases/sqlx/`
tree is gone; legacy sync-driver compatibility code and dependencies are gone.

### Task 7 — Record final validation and benchmark status

Once the structural cleanup is complete, record the remaining evidence needed to close the
subissue cleanly.

Benchmark entrypoints and docs for the implementer:

- Binary entrypoint: `packages/tracker-core/src/bin/persistence_benchmark_runner.rs`
- Binary-private implementation modules: `packages/tracker-core/src/bin/persistence_benchmark/`
- Benchmark artifact index and workflow notes: `packages/tracker-core/docs/benchmarking/README.md`
- Baseline benchmark spec and command examples: `docs/issues/1710-1525-03-persistence-benchmarking.md`
- Current committed baseline artifacts: `packages/tracker-core/docs/benchmarking/runs/2026-04-28/`

Typical commands:

```text
cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- \
  --driver sqlite3

cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- \
  --driver mysql \
  --db-version 8.4
```

1. Run and record focused validation for the final cleanup work.
2. Run `cargo test --workspace --all-targets` and `linter all` on the final state.
3. Run the persistence benchmark comparison against the committed baseline from subissue `1525-03`,
   or explicitly document why that comparison is still deferred.
4. Update the acceptance criteria in this spec to match the final verified state.

**Outcome**: the spec contains closure-quality evidence for remaining acceptance criteria instead
of inferred status.

## Constraints

- Do not add PostgreSQL in this step.
- Do not introduce `sqlx::migrate!()`, migration files, or the `sqlx` `macros` feature — those
  are introduced in subissue `1525-06`.
- Do not change the SQL schema in this step (schema evolution is `1525-06`).
- `DatabaseStores` (four `Arc<dyn XxxStore>` fields, one per narrow trait) is already the
  consumer-facing type returned by `initialize_database()`; do not change this. Do not introduce
  `Arc<Box<dyn Database>>` or the `Persistence` struct from the reference implementation.
- Keep startup schema initialization eager in this subissue and in Task 4.

## Acceptance Criteria

### Progress Review (2026-04-30)

Status: structural cleanup and benchmark validation complete.

What is done:

- SQLite and MySQL driver implementations use `sqlx` pools and async trait methods.
- Schema initialization is still eager in `initialize_database()`.
- Schema creation still uses raw `sqlx::query()` DDL, and `sqlx::migrate!()` is not used.
- Sync-to-async bridge helpers introduced during the migration have been removed, and async initialization has been propagated through current call paths.
- The temporary staging subtree under `packages/tracker-core/src/databases/sqlx/` has been removed; the canonical `databases/driver/` and `databases/traits/` directories are the single persistence surface.
- Legacy `r2d2`, `r2d2_sqlite`, and `r2d2_mysql` dependencies have been removed from `packages/tracker-core/Cargo.toml` (the `rusqlite` symbol was only re-exported through `r2d2_sqlite`; no separate direct dep existed).
- Legacy compatibility/error plumbing has been removed from `packages/tracker-core/src/databases/error.rs` (no more `ConnectionPool` variant or `r2d2`/`rusqlite`/`mysql` `From` impls) and from `packages/tracker-core/src/authentication/key/mod.rs` (the `From<rusqlite::Error>` impl is now `From<sqlx::Error>`).
- Stale `r2d2_*` references in driver doc comments have been replaced with accurate `sqlx`-based wording.
- Current validation passed: `cargo machete`, `linter all`, doc tests, and full workspace tests on the cleaned-up state.
- Persistence benchmark comparison against the `2026-04-28` baseline recorded under `packages/tracker-core/docs/benchmarking/runs/2026-04-30/`. No regression: MySQL totals are 13–16% faster and SQLite per-operation medians stay within run-to-run variance. The bench harness was updated to wait for the MySQL container's TCP listener (sqlx no longer hides this race the way r2d2 did); production code paths are unchanged.

What is still not done:

- There is no recorded evidence in this branch that Tasks 1 to 3 were each validated independently at the time they were completed.

- [x] SQLite and MySQL drivers use `sqlx` with async trait methods.
- [x] Schema initialization remains eager via setup/factory initialization.
- [x] Schema management uses raw `sqlx::query()` DDL; `sqlx::migrate!()` is not used.
- [x] `r2d2`, `r2d2_sqlite`, `rusqlite`, and the `mysql` crate are removed from
      `tracker-core/Cargo.toml`.
- [x] Existing behavior is preserved end-to-end.
- [x] All temporary sync-to-async runtime bridge helpers (e.g. `block_on_current_or_new_runtime`) are removed and replaced with native async call paths.
- [ ] The branch compiles and all tests pass after each of Tasks 1–3 individually (verified by CI
      or manual `cargo test` run after each task).
- [x] Persistence benchmarking (see subissue `1525-03`) shows no regression against the committed
      baseline. — See `packages/tracker-core/docs/benchmarking/runs/2026-04-30/REPORT.md` for the
      full comparison; MySQL totals improved by 13–16% and SQLite per-op medians remained within
      run-to-run variance.
- [x] `cargo test --workspace --all-targets` passes.
- [x] `linter all` exits with code `0`.
- [x] `cargo machete` reports no unused dependencies.

## Out of Scope

- PostgreSQL driver — that is subissue `1525-08`.
- `sqlx::migrate!()` and migration files — that is subissue `1525-06`.
- `async_trait` removal — the `async_trait` crate is required at MSRV 1.72 because
  async-fn-in-traits was stabilized in Rust 1.75. When the MSRV is raised to 1.75+, remove
  `async_trait` and replace `#[async_trait]` attribute usage with native async trait syntax.
  Track this as a follow-up when the MSRV is next bumped.

## References

- EPIC: `#1525`
- Subissue `1525-04`: `docs/issues/1713-1525-04-split-persistence-traits.md` — **already merged
  into `develop`**
- Subissue `1525-03`: `docs/issues/1525-03-persistence-benchmarking.md` — benchmark baseline
- Reference PR: `#1695`
- Reference implementation branch: `josecelano:pr-1684-review` — local checkout at
  `/home/josecelano/Documents/git/committer/me/github/torrust/torrust-tracker-pr-1700`;
  consult only if blocked during implementation
- Reference files (async driver implementations — note: the reference uses `sqlx::migrate!()`
  which is not adopted in this step; use raw DDL instead):
  - `packages/tracker-core/src/databases/driver/sqlite.rs`
  - `packages/tracker-core/src/databases/driver/mysql.rs`
  - `packages/tracker-core/src/databases/driver/mod.rs`
