# M3 Configured-Driver Lifecycle Verification

**Date:** 2026-08-29

## Scope

This verification exercised the existing tracker-core configured-driver and
schema-migration suites for the three supported v3 persistence backends. The
tests construct the selected backend, run its embedded migrations, and exercise
the shared database-driver contract.

## SQLite

```text
cargo test -p torrust-tracker-core databases::setup::tests::it_should_initialize_the_sqlite_database
cargo test -p torrust-tracker-core run_sqlite_driver_tests
```

Both commands passed. The first test exercises
`initialize_database` with an ephemeral configured SQLite path. The second
executes the SQLite database-driver contract on an ephemeral SQLite database.

## PostgreSQL

Docker Engine `28.3.3` was available. The repository's opt-in testcontainers
test passed:

```text
TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST=true cargo test -p torrust-tracker-core --features db-compatibility-tests run_postgres_driver_tests -- --nocapture
```

The test completed successfully in 19.38 seconds. It starts a disposable
PostgreSQL 16 container, runs the shared driver contract, verifies a second
migration run is a no-op, creates a fresh schema, and asserts that all four
embedded migrations are recorded in `_sqlx_migrations`.

## MySQL

Docker Hub access recovered without an explicit login, and the required image
was pulled successfully:

```text
docker pull mysql:8.0
Status: Downloaded newer image for mysql:8.0
```

The repository's canonical compatibility command then passed:

```text
TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG=8.0 cargo test -p torrust-tracker-core --features db-compatibility-tests run_mysql_driver_tests -- --nocapture

test databases::driver::mysql::tests::run_mysql_driver_tests ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 132 filtered out; finished in 9.24s
```

The suite starts a disposable MySQL 8.0 container and exercises the shared
database-driver contract, including complete schema migration and idempotent
second migration behavior.

## Result

SQLite, PostgreSQL, and MySQL configured-driver lifecycle checks passed. M3,
T4, and AC7 are complete. The earlier Docker Hub HTTP 401 was transient and
did not indicate a persistent local authentication requirement.
