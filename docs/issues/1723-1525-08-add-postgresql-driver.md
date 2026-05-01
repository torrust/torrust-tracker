# Subissue 1525-08: Add PostgreSQL Driver

## Goal

Add PostgreSQL as a third production SQL backend by implementing an async `sqlx`-backed
driver, wiring it into the configuration and factory, creating all four migration files
(starting from migration 1, history-aligned with SQLite and MySQL), and extending the
existing QA harnesses so PostgreSQL receives the same test coverage as the other backends.

## Why Last

PostgreSQL is the feature goal of the EPIC, but adding it first would have meant building on
an ad hoc, sync, pre-migration foundation. By the time this subissue is implemented, the
persistence layer is async (`1525-05`), schema-managed (`1525-06`), and correctly typed
(`1525-07`). PostgreSQL can now land as a first-class backend with no special-casing.

## Proposed Branch

- `1525-08-add-postgresql-driver`

## Background

### Starting point

By the time this subissue is implemented:

- **1525-04** and **1525-04b** together split the monolithic `Database` trait into four
  narrow context traits (`SchemaMigrator`, `TorrentMetricsStore`, `WhitelistStore`,
  `AuthKeyStore`) plus a blanket `Database` aggregate supertrait, and migrated all
  production consumers to narrow traits. Both existing drivers (`Sqlite`, `Mysql`) satisfy
  `Database` through the blanket impl. The factory (`initialize_database`) in
  `databases/setup.rs` constructs the concrete driver once and returns a `DatabaseStores`
  struct whose fields are `Arc<dyn XxxStore>` — production consumers never see
  `Arc<Box<dyn Database>>`. The internal driver test helpers in `databases/driver/mod.rs`
  still use `Arc<Box<dyn Database>>` as a convenience wrapper for the shared test suite.

- **1525-05** has moved SQLite and MySQL to async `sqlx` connection pools. `r2d2`, `r2d2_sqlite`,
  `rusqlite`, and the `mysql` crate are gone. The `sqlx` dependency has `sqlite` and `mysql`
  features but not yet `postgres`.

- **1525-06** has replaced the raw DDL in `create_database_tables()` with `sqlx::migrate!()`.
  Each driver has a `static MIGRATOR` pointing to its backend-specific migration directory and
  a `bootstrap_legacy_schema()` helper for upgrading pre-v4 databases. Both backends have three
  migration files.

- **1525-07** has widened MySQL download-counter columns to `BIGINT` via a fourth migration,
  added a history-aligned no-op migration for SQLite, and kept `NumberOfDownloads = u32`.
  The migration file layout at the end of `1525-07` is:

  ```text
  packages/tracker-core/migrations/
    sqlite/
      20240730183000_torrust_tracker_create_all_tables.sql
      20240730183500_torrust_tracker_keys_valid_until_nullable.sql
      20250527093000_torrust_tracker_new_torrent_aggregate_metrics_table.sql
      20260409120000_torrust_tracker_widen_download_counters.sql
    mysql/
      20240730183000_torrust_tracker_create_all_tables.sql
      20240730183500_torrust_tracker_keys_valid_until_nullable.sql
      20250527093000_torrust_tracker_new_torrent_aggregate_metrics_table.sql
      20260409120000_torrust_tracker_widen_download_counters.sql
  ```

  No `postgresql/` directory exists yet.

### Driver enum locations

Two separate `Driver` enums exist and both must be extended:

- **Configuration** — `packages/configuration/src/v2_0_0/database.rs`: user-facing config
  file value. Holds `Sqlite3`, `MySQL`. Used by the tracker to select which driver to build.
- **Databases factory** — `packages/tracker-core/src/databases/driver/mod.rs`: internal
  dispatch enum. Holds `Sqlite3`, `MySQL`. `build()` matches on this to construct the driver.
  `databases/setup.rs` converts from the configuration enum to this internal enum.

### No legacy bootstrap for PostgreSQL

The `bootstrap_legacy_schema()` helper introduced in `1525-06` exists to upgrade databases
that were managed manually before v4. PostgreSQL was never supported before this subissue, so
no pre-existing PostgreSQL tracker databases exist. The PostgreSQL `create_database_tables()`
implementation skips the legacy bootstrap and calls `MIGRATOR.run()` directly.

### Connection string format

PostgreSQL uses the same `path` field as MySQL in the configuration — a single URL string:

```toml
[core.database]
driver = "postgresql"
path = "postgresql://user:password@host:port/dbname"
```

The `mask_secrets()` function in the configuration package must be extended to parse and
redact the password from this URL, mirroring the existing MySQL URL masking logic.

### Database pre-creation requirement

Unlike SQLite (which creates its file on first connection), PostgreSQL requires the target
database to already exist before `sqlx` can connect. The `torrust_tracker` database referenced
in the connection URL must be created before the tracker starts:

```sql
CREATE DATABASE torrust_tracker;
```

**Test containers**: the `PostgresConfiguration.database` field (`torrust_tracker_test` by
default) is passed as the `POSTGRES_DB` env var to the PostgreSQL container. The official
`postgres` Docker image creates this database automatically — no manual `CREATE DATABASE`
call is needed in test code.

**Container config** (`tracker.container.postgresql.toml`): the URL points to
`postgresql://postgres:postgres@postgres:5432/torrust_tracker`. The accompanying compose file
or deployment guide must ensure the `torrust_tracker` database exists — either by setting
`POSTGRES_DB=torrust_tracker` on the PostgreSQL service, or by running a setup step before the
tracker starts. Without it, the tracker will exit on startup with a `sqlx` connection error
that does not clearly identify the missing database as the cause.

## What Changes

### Migration files

Create a `postgresql/` directory under `packages/tracker-core/migrations/` with all four
migration files. The timestamps are shared with the SQLite and MySQL backends, keeping the
`_sqlx_migrations` version history identical across all three backends. Migration 4 is **not**
a no-op for PostgreSQL — PostgreSQL's migration 1 creates the columns as `INTEGER` (matching
the other backends at their migration-1 state), and migration 4 widens them to `BIGINT` using
PostgreSQL-specific `ALTER COLUMN` syntax.

**`20240730183000_torrust_tracker_create_all_tables.sql`**:

```sql
CREATE TABLE IF NOT EXISTS whitelist (
    id SERIAL PRIMARY KEY,
    info_hash VARCHAR(40) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS torrents (
    id SERIAL PRIMARY KEY,
    info_hash VARCHAR(40) NOT NULL UNIQUE,
    completed INTEGER DEFAULT 0 NOT NULL
);

CREATE TABLE IF NOT EXISTS keys (
    id SERIAL PRIMARY KEY,
    key VARCHAR(32) NOT NULL UNIQUE,
    valid_until INTEGER NOT NULL
);
```

PostgreSQL differences from MySQL and SQLite: `SERIAL` instead of `AUTO_INCREMENT` or
`INTEGER PRIMARY KEY AUTOINCREMENT`; no backtick quoting; parameter placeholders are `$1`,
`$2`, … in DML queries (not `?`).

**`20240730183500_torrust_tracker_keys_valid_until_nullable.sql`**:

```sql
ALTER TABLE keys ALTER COLUMN valid_until DROP NOT NULL;
```

**`20250527093000_torrust_tracker_new_torrent_aggregate_metrics_table.sql`**:

```sql
CREATE TABLE IF NOT EXISTS torrent_aggregate_metrics (
    id SERIAL PRIMARY KEY,
    metric_name VARCHAR(50) NOT NULL UNIQUE,
    value INTEGER DEFAULT 0 NOT NULL
);
```

**`20260409120000_torrust_tracker_widen_download_counters.sql`**:

```sql
ALTER TABLE torrents
    ALTER COLUMN completed TYPE BIGINT,
    ALTER COLUMN completed SET DEFAULT 0,
    ALTER COLUMN completed SET NOT NULL;

ALTER TABLE torrent_aggregate_metrics
    ALTER COLUMN value TYPE BIGINT,
    ALTER COLUMN value SET DEFAULT 0,
    ALTER COLUMN value SET NOT NULL;
```

### Configuration package

In `packages/configuration/src/v2_0_0/database.rs`:

- Add `PostgreSQL` variant to the `Driver` enum:

  ```rust
  #[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone)]
  #[serde(rename_all = "lowercase")]
  pub enum Driver {
      Sqlite3,
      MySQL,
      PostgreSQL,   // new
  }
  ```

- Extend `mask_secrets()` to handle the PostgreSQL URL. MySQL and PostgreSQL both use a URL
  `path`; the masking code can share a branch:

  ```rust
  Driver::MySQL | Driver::PostgreSQL => {
      let mut url = Url::parse(&self.path)?;
      url.set_password(Some("***")).ok();
      self.path = url.to_string();
  }
  ```

- Add a test:

  ```rust
  fn it_should_allow_masking_the_postgresql_user_password()
  ```

### `tracker-core` Cargo.toml

Add `"postgres"` to the `sqlx` features list:

```toml
sqlx = { version = "...", features = [
    "sqlite", "mysql", "postgres", "macros", "runtime-tokio-native-tls"
] }
```

### PostgreSQL driver

New file: `packages/tracker-core/src/databases/driver/postgres.rs`.

**Driver struct and constructor**:

```rust
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool, Row};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;

const DRIVER: &str = "postgresql";

static MIGRATOR: Migrator = sqlx::migrate!("migrations/postgresql");

pub(crate) struct Postgres {
    pool: PgPool,
    schema_ready: AtomicBool,
    schema_lock: Mutex<()>,
}

impl Postgres {
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let options = db_path
            .parse::<PgConnectOptions>()
            .map_err(|e| Error::connection_error(DRIVER, e))?
            .disable_statement_logging();
        let pool = PgPoolOptions::new().connect_lazy_with(options);
        Ok(Self {
            pool,
            schema_ready: AtomicBool::new(false),
            schema_lock: Mutex::new(()),
        })
    }
}
```

**Lazy migration latch** (same double-checked pattern as SQLite and MySQL):

```rust
async fn ensure_schema(&self) -> Result<(), Error> {
    if self.schema_ready.load(Ordering::Acquire) {
        return Ok(());
    }
    let _guard = self.schema_lock.lock().await;
    if self.schema_ready.load(Ordering::Acquire) {
        return Ok(());
    }
    self.create_database_tables().await?;
    self.schema_ready.store(true, Ordering::Release);
    Ok(())
}
```

**`SchemaMigrator` implementation**:

`create_database_tables()` skips the legacy bootstrap (PostgreSQL has no pre-v4 databases)
and calls `MIGRATOR.run()` directly:

```rust
async fn create_database_tables(&self) -> Result<(), Error> {
    // PostgreSQL is a new backend — no legacy databases exist without _sqlx_migrations.
    // MIGRATOR.run() always takes the fresh-database path.
    MIGRATOR
        .run(&self.pool)
        .await
        .map_err(|e| Error::migration_error(DRIVER, e))?;
    Ok(())
}
```

`drop_database_tables()` drops all five tables including `_sqlx_migrations` so the
drop/create cycle used in the test suite works correctly. Use `DROP TABLE IF EXISTS`
consistently for all drops, matching the style established in `1525-06`:

```rust
async fn drop_database_tables(&self) -> Result<(), Error> {
    sqlx::query("DROP TABLE IF EXISTS _sqlx_migrations")
        .execute(&self.pool).await?;
    sqlx::query("DROP TABLE IF EXISTS torrent_aggregate_metrics")
        .execute(&self.pool).await?;
    sqlx::query("DROP TABLE IF EXISTS whitelist")
        .execute(&self.pool).await?;
    sqlx::query("DROP TABLE IF EXISTS torrents")
        .execute(&self.pool).await?;
    sqlx::query("DROP TABLE IF EXISTS keys")
        .execute(&self.pool).await?;
    Ok(())
}
```

**SQL syntax differences from SQLite and MySQL**:

| Aspect                | SQLite / MySQL                                                    | PostgreSQL                                           |
| --------------------- | ----------------------------------------------------------------- | ---------------------------------------------------- |
| Parameter placeholder | `?`                                                               | `$1`, `$2`, …                                        |
| Upsert                | `ON DUPLICATE KEY UPDATE` (MySQL) or `INSERT OR REPLACE` (SQLite) | `ON CONFLICT (col) DO UPDATE SET col = EXCLUDED.col` |
| Auto-increment (DDL)  | `AUTO_INCREMENT` / `AUTOINCREMENT`                                | `SERIAL` (in migration files only)                   |

**Counter encode/decode helpers** (identical contract to SQLite and MySQL):

```rust
fn decode_counter(value: i64) -> Result<NumberOfDownloads, Error> {
  u32::try_from(value).map_err(|err| Error::invalid_query(DRIVER, err))
}

fn encode_counter(value: NumberOfDownloads) -> Result<i64, Error> {
    i64::try_from(value).map_err(|err| Error::invalid_query(DRIVER, err))
}
```

Use these helpers in every place a counter column is read from or written to the database.
Do not use bare `as i64` casts or `as u32` casts.

**`TorrentMetricsStore`, `WhitelistStore`, `AuthKeyStore` implementations**: Follow the same
structure as the SQLite and MySQL drivers, substituting `$1`/`$2` placeholders and the
PostgreSQL upsert syntax. There are no behavior differences relative to the other backends.

### Driver factory

In `packages/tracker-core/src/databases/driver/mod.rs`:

- Add `PostgreSQL` variant to the `Driver` enum (and extend `as_str()` and `FromStr` to
  recognize `"postgresql"`).
- Add a `pub mod postgres;` declaration.

There is no `build()` helper in this module. The concrete driver is constructed
directly in `setup.rs`.

### Database setup

In `packages/tracker-core/src/databases/setup.rs`:

- Extend the first `match` (config driver → internal `Driver` enum):

  ```rust
  torrust_tracker_configuration::Driver::PostgreSQL => Driver::PostgreSQL,
  ```

- Add a `Driver::PostgreSQL` arm to the second `match` (internal `Driver` → concrete
  construction), mirroring the `Sqlite3` and `MySQL` arms:

  ```rust
  Driver::PostgreSQL => {
      use super::driver::postgres::Postgres;
      let db = Arc::new(Postgres::new(&config.database.path).expect("Database driver build failed."));
      db.create_database_tables().await.expect("Could not create database tables.");
      build_database_stores(db)
  }
  ```

### Default configuration file

Add `share/default/config/tracker.container.postgresql.toml` modelled on the existing MySQL
container config. The PostgreSQL connection string points to a service named `postgres`:

```toml
[core.database]
driver = "postgresql"
path = "postgresql://postgres:postgres@postgres:5432/torrust_tracker"
```

All other sections remain the same as the existing container configs.

### Driver tests

Add an inline `#[cfg(test)]` module in `postgres.rs`. The test is guarded by an environment
variable to avoid requiring a PostgreSQL container in every `cargo test` run.

**Environment variables** (matching the MySQL driver pattern — testcontainers only):

| Variable                                         | Purpose                                    | Default                 |
| ------------------------------------------------ | ------------------------------------------ | ----------------------- |
| `TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST`  | Enable the test (must be set to any value) | unset → test is skipped |
| `TORRUST_TRACKER_CORE_POSTGRES_DRIVER_IMAGE_TAG` | PostgreSQL Docker image tag                | `16`                    |

No external-URL option. The test always starts a container, matching the MySQL driver
pattern.

**Test container defaults**:

```text
internal port:  5432
database:       torrust_tracker_test
user:           postgres
password:       test
```

Start the container using `testcontainers::GenericImage` (already a dev-dependency from
MySQL tests). Set container env vars `POSTGRES_PASSWORD`, `POSTGRES_USER`, `POSTGRES_DB`.

**Test function skeleton** (following the MySQL driver pattern):

```rust
#[tokio::test]
async fn run_postgres_driver_tests() -> Result<(), Box<dyn std::error::Error + 'static>> {
    if std::env::var("TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST").is_err() {
        println!("Skipping the PostgreSQL driver tests.");
        return Ok(());
    }

    let postgres_configuration = PostgresConfiguration::default();
    let stopped_container = StoppedPostgresContainer::default();
    let container = stopped_container.run(&postgres_configuration).await.unwrap();

    let host = container.get_host().await;
    let port = container.get_host_port_ipv4().await;
    let config = core_configuration(&host, port, &postgres_configuration);

    let driver = Arc::new(Box::new(Postgres::new(&config.database.path).unwrap()) as Box<dyn Database>);
    run_tests(&driver).await;
    Ok(())
}
```

**Shared test suite**: reuse the `tests::run_tests()` function already used by the SQLite and
MySQL test modules. All three backends must pass the same set of behavioral scenarios (torrent
CRUD, whitelist CRUD, auth key CRUD, schema drop/create cycle).

## Tasks

### Task 1 — Add `Driver::PostgreSQL` to the configuration package

Steps:

- Add `PostgreSQL` variant to the `Driver` enum in
  `packages/configuration/src/v2_0_0/database.rs`.
- Extend `mask_secrets()` to handle the PostgreSQL URL (share a branch with the MySQL case).
- Add test `it_should_allow_masking_the_postgresql_user_password`.

Acceptance criteria:

- [ ] `Driver::PostgreSQL` serializes as `"postgresql"` in TOML.
- [ ] `mask_secrets()` correctly redacts the password in a PostgreSQL URL.
- [ ] The new test passes.

### Task 2 — Add sqlx `postgres` feature and create PostgreSQL migration files

Steps:

- Add `"postgres"` to the `sqlx` features in `packages/tracker-core/Cargo.toml`.
- Create `packages/tracker-core/migrations/postgresql/` with the four migration files listed
  in the "What Changes" section above.
- Verify the SQL content is correct by running each migration in sequence against a temporary
  PostgreSQL database and confirming the expected schema is produced.

Acceptance criteria:

- [ ] `packages/tracker-core/migrations/postgresql/` contains exactly four files with the
      same timestamps as the SQLite and MySQL directories.
- [ ] Migration 1 creates `whitelist`, `torrents`, and `keys` with PostgreSQL DDL (`SERIAL`,
      no backtick quoting, `$1`/`$2` placeholders in DML).
- [ ] Migration 2 makes `keys.valid_until` nullable.
- [ ] Migration 3 creates `torrent_aggregate_metrics`.
- [ ] Migration 4 widens `torrents.completed` and `torrent_aggregate_metrics.value` to
      `BIGINT` using `ALTER COLUMN ... TYPE BIGINT` syntax.
- [ ] Running all four migrations in sequence produces a schema consistent with the SQLite
      and MySQL schemas after their four migrations.

### Task 3 — Implement the PostgreSQL driver

Create `packages/tracker-core/src/databases/driver/postgres.rs` with:

- `Postgres` struct (pool, `schema_ready` latch, `schema_lock` mutex).
- `Postgres::new(db_path: &str) -> Result<Self, Error>` using `PgConnectOptions` and
  `PgPoolOptions::connect_lazy_with()`.
- `static MIGRATOR: Migrator = sqlx::migrate!("migrations/postgresql");`
- `ensure_schema()` latch — same double-checked pattern as SQLite and MySQL.
- `SchemaMigrator` impl: `create_database_tables()` (MIGRATOR.run() only, no legacy
  bootstrap) and `drop_database_tables()` (all five tables with `DROP TABLE IF EXISTS`).
- `TorrentMetricsStore`, `WhitelistStore`, `AuthKeyStore` impls — same semantics as the
  other backends, using `$1`/`$2` placeholders and PostgreSQL upsert syntax.
- `decode_counter`/`encode_counter` helpers.

Acceptance criteria:

- [ ] `Postgres` satisfies the `Database` aggregate supertrait through the blanket impl
      (no manual `impl Database for Postgres {}` block).
- [ ] `create_database_tables()` calls `MIGRATOR.run()` with no legacy bootstrap.
- [ ] `drop_database_tables()` drops all five tables including `_sqlx_migrations`.
- [ ] All counter reads use `decode_counter`; all counter writes use `encode_counter`.
- [ ] No bare `as i64` or `as u32` casts in the driver.

### Task 4 — Wire the PostgreSQL driver into the factory and setup

Steps:

- In `packages/tracker-core/src/databases/driver/mod.rs`:
  - Add `PostgreSQL` to the `Driver` enum.
  - Extend `as_str()` to return `"postgresql"` for `PostgreSQL`.
  - Extend `FromStr` to accept `"postgresql"` and update the error message to include it.
  - Add `pub mod postgres;`.
- In `packages/tracker-core/src/databases/setup.rs`:
  - Add `torrust_tracker_configuration::Driver::PostgreSQL => Driver::PostgreSQL` to the
    first `match` (config → internal enum).
  - Add the `Driver::PostgreSQL` arm to the second `match` (internal enum → concrete
    construction), constructing `Arc::new(Postgres::new(...))` and calling
    `create_database_tables()` then `build_database_stores(db)` — matching the existing
    `Sqlite3` and `MySQL` arms exactly.

Acceptance criteria:

- [ ] `cargo build --workspace` succeeds with `driver = "postgresql"` in a config file.
- [ ] `databases/setup.rs` correctly dispatches to the PostgreSQL driver when the
      configuration specifies `driver = "postgresql"`.

### Task 5 — Add the PostgreSQL driver tests

Add an inline `#[cfg(test)]` module to `postgres.rs` as described in the "Driver tests"
section above.

Steps:

- Implement `run_postgres_driver_tests` guarded by
  `TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST`, matching the MySQL driver test
  structure exactly.
- Always start a `testcontainers::GenericImage` container (no external-URL fallback).
- Default container tag: `16`. Tag is overridable via
  `TORRUST_TRACKER_CORE_POSTGRES_DRIVER_IMAGE_TAG` (enables the compatibility matrix loop
  in Task 6).
- Call `tests::run_tests(&driver).await` — the shared test suite used by all backends.

Acceptance criteria:

- [ ] `TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST` is unset → test prints skip message
      and returns immediately without error.
- [ ] When the env var is set, the test starts a PostgreSQL container via testcontainers,
      runs the shared test suite, and passes.
- [ ] The container started by the test is removed unconditionally on completion or failure.

### Task 6 — Extend the compatibility matrix (completing subissue 1525-01)

Steps:

- In `contrib/dev-tools/qa/run-db-compatibility-matrix.sh`, add:
  - A test for the PostgreSQL configuration URL masking (after the existing protocol tests):

    ```bash
    cargo test -p torrust-tracker-configuration postgresql_user_password -- --nocapture
    ```

  - A PostgreSQL versions loop after the MySQL loop:

    ```bash
    POSTGRES_VERSIONS_STRING="${POSTGRES_VERSIONS:-14 15 16 17}"
    read -r -a POSTGRES_VERSIONS <<< "$POSTGRES_VERSIONS_STRING"

    for version in "${POSTGRES_VERSIONS[@]}"; do
        print_heading "PostgreSQL ${version}"
        docker pull "postgres:${version}"
        TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST=1 \
        TORRUST_TRACKER_CORE_POSTGRES_DRIVER_IMAGE_TAG="${version}" \
            cargo test -p bittorrent-tracker-core run_postgres_driver_tests -- --nocapture
    done
    ```

  - `POSTGRES_VERSIONS` defaults to `14 15 16 17`; override via env var.

- The script already has `set -euo pipefail`; failures in the PostgreSQL loop will abort
  the script with the failing version visible in the output.

Acceptance criteria:

- [ ] The script runs the PostgreSQL driver test for each version in `POSTGRES_VERSIONS`.
- [ ] The `POSTGRES_VERSIONS` set is overridable via env var.
- [ ] The script fails fast on the first failing backend/version combination.
- [ ] The script runs successfully end-to-end in a clean environment; a passing run log is
      included in the PR description.
- [ ] The compatibility matrix exercises PostgreSQL 14, 15, 16, and 17 by default.

### Task 7 — Extend the qBittorrent E2E runner with MySQL and PostgreSQL (completing subissue 1525-02)

The qBittorrent E2E runner introduced in subissue `1525-02` uses SQLite only. The `Args`
struct in `src/console/ci/qbittorrent_e2e/runner.rs` has no `--db-driver` flag;
`config_builder.rs` defaults to an SQLite path for all runs. MySQL E2E support was
explicitly deferred in `1525-02` and has NOT been added since. This task adds
`--db-driver` support for all three backends: `sqlite3` (existing default, preserved),
`mysql` (new), and `postgresql` (new).

Steps:

- Add a `--db-driver` CLI argument to the E2E runner binary. Accept `sqlite3`, `mysql`, and
  `postgresql`. Default: `sqlite3` (preserving existing behavior).
- When `--db-driver postgresql` is specified:
  - Start a PostgreSQL container via `testcontainers::GenericImage` (or a `DockerCompose`
    stack if a compose file is preferred). Wait for the container to be ready before starting
    the tracker. Readiness can be checked by attempting a database connection or by running
    `pg_isready` inside the container via `docker exec`.
  - Generate a tracker config with `driver = "postgresql"` and the appropriate connection URL.
  - Run the rest of the E2E scenario unchanged (seeder → tracker → leecher flow is
    database-agnostic).
- Reuse the `Drop` guard pattern from the existing runner for unconditional PostgreSQL
  container cleanup.
- Add a CI step (or extend the existing E2E step) that exercises `--db-driver postgresql`.
- Document the `--db-driver` argument in the binary's module doc comment.

Acceptance criteria:

- [ ] The E2E runner completes a full seeder → leecher download with PostgreSQL as the
      backend.
- [ ] No orphaned containers remain on success or failure.
- [ ] The `--db-driver` argument is documented in the binary's module doc comment.

### Task 8 — Extend the benchmark runner with PostgreSQL (completing subissue 1525-03)

The benchmark runner introduced in subissue `1525-03` supports SQLite and MySQL. Extend it to
also benchmark PostgreSQL.

Steps:

- Add `postgresql` as an accepted value for `--dbs` in the benchmark runner CLI.
- Add `contrib/dev-tools/bench/compose.bench-postgresql.yaml` following the same structure as
  the MySQL compose file: tracker service + PostgreSQL service, parameterized tracker image tag
  via env var, no fixed host ports, `healthcheck` defined for each service.
- Wire the PostgreSQL compose file into the runner's per-suite lifecycle (same as MySQL/SQLite:
  `DockerCompose::up()`, port discovery, workloads, `DockerCompose::down()` via `Drop` guard).
- Re-run the benchmark with both SQLite, MySQL, and PostgreSQL and update
  `docs/benchmarks/baseline.md` and `docs/benchmarks/baseline.json` with the new results.

Acceptance criteria:

- [ ] `--dbs postgresql` produces benchmark results.
- [ ] `compose.bench-postgresql.yaml` starts and stops cleanly with no orphaned resources.
- [ ] `docs/benchmarks/baseline.md` is updated and includes PostgreSQL results.

### Task 9 — Add the default PostgreSQL container config, update docs, and fix spell-check

Steps:

- Add `share/default/config/tracker.container.postgresql.toml` as described in the
  "What Changes" section.

- Update `share/container/entry_script_sh` to handle `postgresql` alongside the existing
  `sqlite3` and `mysql` branches. Add an `elif` branch immediately after the `mysql` branch:

  ```sh
  elif cmp_lc "$TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER" "postgresql"; then

      # (no database file needed for PostgreSQL)

      # Select default PostgreSQL configuration
      default_config="/usr/share/torrust/default/config/tracker.container.postgresql.toml"
  ```

  Also update the error message in the `else` branch to list all three supported backends:

  ```sh
  echo "Please Note: Supported Database Types: \"sqlite3\", \"mysql\", \"postgresql\"."
  ```

  The `Containerfile` already copies this file via
  `COPY --chmod=0555 ./share/container/entry_script_sh /usr/local/bin/entry.sh`; no
  `Containerfile` changes are needed.

- Rename `compose.yaml` to `compose.mysql.yaml`. This file is used by
  `.github/workflows/container.yaml` in the `docker compose build` step. Update the
  workflow to pass `-f compose.mysql.yaml` so the rename is transparent to CI.
  Update any documentation that references `compose.yaml` for the MySQL demo.

- Add a new `compose.postgresql.yaml` for the PostgreSQL backend. Model it after the
  renamed `compose.mysql.yaml` but replace the `mysql` service with a `postgres` service:

  ```yaml
  postgres:
    image: postgres:16
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 3s
      retries: 5
      start_period: 30s
    environment:
      - POSTGRES_PASSWORD=postgres
      - POSTGRES_USER=postgres
      - POSTGRES_DB=torrust_tracker
    networks:
      - server_side
    volumes:
      - postgres_data:/var/lib/postgresql/data
  ```

  The tracker service in `compose.postgresql.yaml` should default to
  `TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER=postgresql` and depend on
  `postgres` only (not `mysql`).

- Add a second `docker compose -f compose.postgresql.yaml build` step to the
  `container.yaml` workflow so both compose files are validated in CI.

- Update user-facing documentation to document PostgreSQL as a supported backend:
  - `README.md` — add `postgresql` to the list of supported database backends.
  - `docs/containers.md` — add a section (or extend the existing database section) describing
    how to run the tracker with PostgreSQL, including the `POSTGRES_DB` pre-creation
    requirement and a reference to the new container config file.

- Run `linter cspell` and add any new technical terms to `project-words.txt` in alphabetical
  order. Terms likely to be flagged: `postgresql` (lowercase), `isready`, and any other
  identifiers used in scripts or code comments.

Acceptance criteria:

- [ ] `share/default/config/tracker.container.postgresql.toml` exists and is valid TOML.
- [ ] `share/container/entry_script_sh` has a `postgresql` branch that selects
      `tracker.container.postgresql.toml`; the `else` error message lists all three supported
      backends.
- [ ] `compose.yaml` is renamed to `compose.mysql.yaml`; `.github/workflows/container.yaml`
      uses `-f compose.mysql.yaml`.
- [ ] `compose.postgresql.yaml` exists with a `postgres` service and a tracker service
      that defaults to the PostgreSQL driver.
- [ ] `docker compose -f compose.postgresql.yaml up` starts the tracker successfully
      against the PostgreSQL container.
- [ ] The container configuration or its companion documentation (compose file or README)
      creates the `torrust_tracker` database (via `POSTGRES_DB` env var or equivalent) before
      the tracker is started.
- [ ] The tracker starts successfully when pointed at this config with a running PostgreSQL
      container named `postgres`.
- [ ] `README.md` lists PostgreSQL as a supported database backend.
- [ ] `docs/containers.md` documents how to run the tracker with PostgreSQL and states the
      database pre-creation requirement.
- [ ] `linter cspell` reports no new failures.

## Out of Scope

- Changing the internal driver test helpers (`databases/driver/mod.rs`) from
  `Arc<Box<dyn Database>>` to narrow trait objects. Production consumers already use
  narrow traits (`Arc<dyn XxxStore>`) via `DatabaseStores`; the test-helper wiring is
  an internal concern and can be migrated separately.
- PostgreSQL-specific performance tuning or connection pool size configuration beyond the
  default `PgPoolOptions` settings.
- Down migrations (rollback support).
- TLS configuration for the PostgreSQL connection (can be expressed in the URL without code
  changes).
- Any persistence redesign not required for the driver to work.
- UDP E2E testing against PostgreSQL (can be added later without redesigning the E2E setup).

## Acceptance Criteria

- [ ] `Driver::PostgreSQL` serializes as `"postgresql"` in TOML; the configuration package
      compiles cleanly.
- [ ] `mask_secrets()` redacts the password from a PostgreSQL URL.
- [ ] `packages/tracker-core/migrations/postgresql/` contains four migration files with the
      same timestamps as SQLite and MySQL.
- [ ] Migration 1 creates the tables with PostgreSQL DDL (`SERIAL`, no backtick quoting).
- [ ] Migration 4 widens `torrents.completed` and `torrent_aggregate_metrics.value` to
      `BIGINT` using `ALTER COLUMN ... TYPE BIGINT` syntax.
- [ ] `packages/tracker-core/src/databases/driver/postgres.rs` exists and satisfies
      `Database` through the blanket impl (no manual `impl Database for Postgres {}`).
- [ ] `create_database_tables()` calls `MIGRATOR.run()` with no legacy bootstrap.
- [ ] `drop_database_tables()` drops all five tables including `_sqlx_migrations`.
- [ ] All counter reads/writes use `decode_counter`/`encode_counter`; no bare truncating
      casts.
- [ ] The shared driver test suite passes against PostgreSQL when
      `TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST` is set.
- [ ] `TORRUST_TRACKER_CORE_POSTGRES_DRIVER_IMAGE_TAG` controls the PostgreSQL version used
      in tests, enabling the compatibility matrix loop.
- [ ] `run-db-compatibility-matrix.sh` loops over `POSTGRES_VERSIONS` (default:
      `14 15 16 17`).
- [ ] The qBittorrent E2E runner completes a full download cycle with both MySQL and
      PostgreSQL (the `--db-driver` flag is added for all three backends).
- [ ] The benchmark runner produces results for PostgreSQL; `docs/benchmarks/baseline.md`
      is updated.
- [ ] `share/default/config/tracker.container.postgresql.toml` exists and is valid TOML.
- [ ] `share/container/entry_script_sh` has a `postgresql` branch; the `else` error message
      lists all three supported backends.
- [ ] `compose.yaml` is renamed to `compose.mysql.yaml`; `compose.postgresql.yaml` exists;
      both are validated by `.github/workflows/container.yaml`; `docker compose -f
  compose.postgresql.yaml up` starts the tracker successfully against PostgreSQL.
- [ ] `project-words.txt` is up to date; `linter cspell` reports no failures.
- [ ] `README.md` lists PostgreSQL as a supported database backend.
- [ ] `docs/containers.md` documents how to run the tracker with PostgreSQL and states the
      database pre-creation requirement.
- [ ] Persistence benchmarking shows no regression for SQLite or MySQL against the committed
      baseline.
- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `cargo machete` reports no unused dependencies.
- [ ] `linter all` exits with code `0`.

## Implementation Questions

The following questions must be answered before starting implementation.

### Q1 — PR scope: single PR or phased?

Do you want everything in this spec implemented in one PR, or split into phases
(e.g. core driver + migrations first, then QA/E2E/benchmark extensions)?

**Answer**:

I want one PR, but commits must be incremental and logically organized to allow for review in phases.
Each commit your be deployable (pass the pre-commit checks) and testable independently.

### Q2 — CI scope for this subissue

Should the PostgreSQL compatibility matrix be wired into
`.github/workflows/testing.yaml` now, or keep CI changes minimal and run
PostgreSQL checks manually for the first iteration?

**Answer**:

Yes, but that can be one of the independent tasks.

### Q3 — MySQL support in the qBittorrent E2E runner

The spec includes adding `--db-driver mysql` support to the qBittorrent E2E
runner as part of this subissue (Task 7). Should that stay coupled here, or
should this subissue deliver PostgreSQL-only E2E and defer MySQL E2E to a
follow-up?

**Answer**:

MySQL E2E was already added (confirmed). We have to add PostgreSQL to the E2E runner.
This can be an independent commit. Task 7 will add both `--db-driver` support and the
PostgreSQL E2E integration.

### Q4 — Benchmark artifacts in this branch

Should fresh benchmark results for PostgreSQL be generated and committed in
this same branch, or deferred until the driver is stable and a follow-up run
is done?

**Answer**:

Yes, after finishing the implementation and verifying the driver works, we can run benchmarks and update the baseline in the same branch. Again this can be another independent commit.

### Q5 — `compose.yaml` database service strategy

The spec says the tracker `depends_on` both `mysql` and `postgres` so both DB
services start regardless of which driver is selected. Alternatively, services
could be profile-based so only the selected backend starts. Which do you
prefer?

**Answer**:

Confirmed: the spec is correct. Rename `compose.yaml` → `compose.mysql.yaml`, add
`compose.postgresql.yaml` with the PostgreSQL service (tracker depends on `postgres` only),
and update `.github/workflows/container.yaml` to validate both files. This can be implemented
as part of Task 9 (containers and documentation updates).

### Q6 — PostgreSQL driver test: testcontainers vs external URL

The spec supports both a pre-existing PostgreSQL instance (via
`TORRUST_TRACKER_CORE_POSTGRES_DRIVER_URL`) and a testcontainers container.
Is this two-mode approach correct, or should the test always start a container
(matching the MySQL driver test pattern)?

**Answer**:

Match the MySQL driver test pattern: testcontainers only, no external-URL fallback.
This ensures consistent, isolated test environments across all three backends.

### Q7 — Reference implementation alignment

Should implementation prioritize parity with the reference branch
(`josecelano:pr-1684-review`) or prioritize the smallest clean diff against
the current refactored codebase, even where that diverges from the reference?

**Answer**:

Not at all. The reference implementation is a guide, not a spec. The implementation should prioritize the cleanest solution, even if that means diverging from the reference in some places. The reference may contain code that is no longer relevant or optimal in the context of the refactored codebase, and blindly following it could lead to unnecessary complexity or technical debt. By clean solutions, I mean solutions that are well-structured, maintainable, testable,and fit well with the existing codebase, even if they differ from the reference implementation.

### Q8 — Implementation pace in this session

After all answers are provided, should implementation proceed immediately and
run through lint/tests in the same session without pausing for interim review?

**Answer**:

No. Read replies, update spec, analyze code readiness, then begin implementation.
All commits must be incremental, deployable, and logically organized.

---

## Implementation Summary

Based on the answers above, the work will be delivered as **one PR with independent,
incremental commits** organized in the following phases:

### Phase 1: Core driver (Tasks 1–6)

These tasks establish the PostgreSQL driver fundamentals and must be completed first.
Each can be committed independently once it passes `linter all` and `cargo test`.

- **Task 1**: Add `Driver::PostgreSQL` to configuration package
- **Task 2**: Add `Driver::PostgreSQL` variant to internal driver enum and `build()` factory
- **Task 3**: Implement `packages/tracker-core/src/databases/driver/postgres/mod.rs` (schema,
  pools, traits)
- **Task 4**: Add migration files for PostgreSQL
- **Task 5**: Extend `packages/tracker-core/Cargo.toml` with `postgres` feature and
  implement the driver tests
- **Task 6**: Extend the persistence benchmark runner (`BenchmarkResource::Postgres`)

### Phase 2: Extended integration (Tasks 7–9)

These tasks integrate PostgreSQL across the E2E harness, containers, and documentation.
Each can be a separate commit once Phase 1 is complete.

- **Task 7**: Add `--db-driver` flag and PostgreSQL support to the qBittorrent E2E runner
- **Task 8**: Extend `.github/workflows/testing.yaml` with PostgreSQL compatibility matrix
- **Task 9**: Add container configs, update `entry_script_sh`, rename/add compose files,
  update workflows and documentation

### Phase 3: Verification (Task 10 — implicit)

After all commits, run benchmarks and update baseline artifacts in a final commit.

### Task dependencies

**No hard blockers between phases.** Phase 1 tasks can run in parallel for code review
(all changes are scoped). Phase 2 tasks depend only on Phase 1 being complete. Benchmarks
(Phase 3) run last for data freshness.

## References

- EPIC: `#1525` — `docs/issues/1525-overhaul-persistence.md`
- Subissue `1525-01`: `docs/issues/1525-01-persistence-test-coverage.md` — compatibility
  matrix structure (PostgreSQL loop deferred here)
- Subissue `1525-02`: `docs/issues/1706-1525-02-qbittorrent-e2e.md` — E2E runner (PostgreSQL
  deferred here)
- Subissue `1525-03`: `docs/issues/1525-03-persistence-benchmarking.md` — benchmark runner
  (PostgreSQL deferred here)
- Subissue `1525-06`: `docs/issues/1719-1525-06-introduce-schema-migrations.md` — migration
  framework and history-alignment pattern
- Subissue `1525-07`: `docs/issues/1721-1525-07-align-rust-and-db-types.md` — fourth migration
  and DB-only widening (`NumberOfDownloads = u32`)
- Reference PR: `#1695`
- Reference implementation branch: `josecelano:pr-1684-review` — see EPIC for checkout
  instructions
- Reference files:
  - `packages/configuration/src/v2_0_0/database.rs` (`Driver::PostgreSQL`, URL masking)
  - `packages/tracker-core/src/databases/driver/postgres.rs` (full driver)
  - `packages/tracker-core/src/databases/driver/mod.rs` (`Driver::PostgreSQL` in `build()`)
  - `packages/tracker-core/src/databases/setup.rs` (PostgreSQL dispatch)
  - `packages/tracker-core/migrations/postgresql/` (all four migration files)
  - `share/default/config/tracker.container.postgresql.toml`
  - `contrib/dev-tools/qa/run-db-compatibility-matrix.sh` (PostgreSQL versions loop)
  - `contrib/dev-tools/qa/run-qbittorrent-e2e.py` (E2E reference with PostgreSQL)
  - `contrib/dev-tools/qa/run-before-after-db-benchmark.py` (benchmark with PostgreSQL)
