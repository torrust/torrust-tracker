---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/setup.rs
status: done
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
| S1 | DONE | The four tests. | They pass; lines 114-115 run. |
| S2 | DONE | Run M1-M4. | Each is killed by its expected test. |
| S3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage
  target directory, 2026-11-23): the file 33 / 41 lines to 96 / 104, regions 28 / 46 to 91 / 108,
  functions 7 / 7 to 16 / 16 (the counts include the test module). Line 115, the table creation of
  the SQLite arm, now runs. Left uncovered: 114 (`Sqlite::new(path)?`: the constructor opens a lazy
  pool and cannot fail, so its error arm has no decision) and 119-128 (the MySQL and PostgreSQL
  arms, assigned to S3). The package's unit-only lines 4,951 / 5,724 (86.50%) to 5,014 / 5,787
  (86.64%), regions 5,503 / 6,428 (85.61%), functions 869 / 1,069 (81.29%).
- **Tests:** 4 added, 1 unchanged: 263 to 267 unit tests in the package.
- **Mutations** (each applied alone and reverted, the file compared with its copy afterwards):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the shared-database test and by every test of the package that builds its stores through the factory |
  | M2 | Killed by the two tests of a database that cannot be opened |
  | M3 | Killed by the shared-database test |
  | M4 | Killed by the legacy factory test of a database that cannot be opened |

- **Review outcome:** the maintainer approved the completed file on 2026-11-23.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the
  existing `axum::serve` warning), `cargo +nightly fmt --all -- --check` and
  `cargo machete --with-metadata` pass.
