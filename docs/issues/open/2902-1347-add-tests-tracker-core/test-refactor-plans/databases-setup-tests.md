---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/setup.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/databases/setup.rs
    - packages/tracker-core/src/databases/driver/sqlite/mod.rs
    - docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md
---

# Database Setup File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 14,
`packages/tracker-core/src/databases/setup.rs`: the factory that builds the four narrow stores from
a database configuration.

## Current State

- **Unit-only coverage:** 33 / 41 lines (80.49%), 28 / 46 regions, 7 / 7 functions, at the S2
  baseline (`develop` at `83f3ecbbc`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`,
  `--all-features --lib`, 2026-11-22). Uncovered lines: 114-115 (the `?` of the SQLite arm: the
  driver constructor and the table creation), 119-122 (the MySQL arm) and 125-128 (the PostgreSQL
  arm).
- **Existing tests:** one, `it_should_initialize_the_sqlite_database`, which calls
  `initialize_database` on a temporary SQLite database and asserts nothing. Other modules' tests
  build their stores through it.
- **Module-owned decisions:**
  - The factory builds **one** driver, creates its tables and hands the same driver, as four narrow
    trait objects, to the four stores (ADR `keep_database_as_aggregate_supertrait`). Dropping the
    tables through the schema migrator must therefore break the other three stores too.
  - `initialize_database_from_configuration` returns the driver or migration error;
    `initialize_database`, its legacy convenience form, panics with a documented message when the
    configuration has no database or the initialization fails.

## Maintainer Decision

Plan approved on 2026-11-23; the MySQL and PostgreSQL arms stay with S3.

## Test Design

Tests join the file's `tests` module. The SQLite ones use a temporary database kept alive by the
test (`ephemeral_configuration()`).

| Test | Act | Assert |
| --- | --- | --- |
| the stores share one migrated database | initialize SQLite; write through the whitelist, key and metrics stores; read back through each; then `drop_database_tables` through the schema migrator | each write reads back (the tables exist); after the drop, `load_whitelist` through the whitelist store fails (they share the driver) |
| a database that cannot be opened | `initialize_database_from_configuration` with a SQLite path in a directory that does not exist | an `Err`, not a panic |
| the legacy factory without a database | `initialize_database` on a `Core` whose `database` is `None` | panics with "database configuration is required" (`should_panic`) |
| the legacy factory on a failing database | `initialize_database` with the same unreachable SQLite path | panics with "database initialization requested" (`should_panic`) |

**Not in this plan:** lines 119-122 and 125-128, the MySQL and PostgreSQL arms. They are the
server-driver arms the ledger assigns to S3 (Q3): a Docker-free test of them needs an unreachable
endpoint, whose connect timeout the pool decides, so S3 measures it before choosing; the Docker
suite builds them through the same function.

## Property-Test Decision

No `quickcheck`: the input is a configuration enum and one path.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | the SQLite arm does not create the tables (the call removed) | the shared-database test (the first write fails) |
| M2 | the SQLite arm ignores a failed table creation (`.ok()`) | the test of a database that cannot be opened |
| M3 | the schema migrator is a second driver, on another file | the shared-database test (the drop does not break the whitelist store) |
| M4 | `initialize_database` panics with another message when the initialization fails | the failing legacy factory test |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| S1 | TODO | The four tests. | They pass; lines 114-115 run. |
| S2 | TODO | Run M1-M4. | Each is killed by its expected test. |
| S3 | TODO | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

To be filled in after S3.
