---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/driver/sqlite/schema_migrator.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/databases/driver/sqlite/schema_migrator.rs
    - packages/tracker-core/src/databases/driver/sqlite/mod.rs
    - packages/tracker-core/src/databases/error.rs
    - packages/tracker-core/migrations/sqlite
---

# SQLite Schema Migrator File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 12,
`packages/tracker-core/src/databases/driver/sqlite/schema_migrator.rs`, whose hypothesis was "no
change likely; challenge each `?`". Each uncovered line is challenged below.

## Current State

- **Unit-only coverage:** 137 / 150 lines (91.33%), 129 / 153 regions, 15 / 23 functions, at the S2
  baseline (`develop` at `83f3ecbbc`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`,
  `--all-features --lib`, 2026-11-22). Uncovered lines: 38, 56, 90, 103, 122-123, 136, 139, 152 and
  279.
- **Existing tests:** 3 in the file, on temporary databases: the legacy bootstrap is a no-op on an
  empty database, seeds the history when the four legacy tables exist (and the migrator then runs as
  a no-op), and rejects a partial legacy state. The shared driver suite and
  `create_database_tables_should_be_idempotent_on_a_fresh_database` cover creating and dropping the
  tables.
- **Module-owned decisions:**
  - `create_database_tables` first bootstraps a legacy database, then runs the embedded migrator; an
    error of either is returned (a migrator failure as `MigrationError`).
  - A migration history that does not match the embedded migrations is refused, not repaired.
  - `drop_database_tables` drops the five tables and returns the first failure.

## Challenge of Each Uncovered Line

| Lines | What they are | Decision |
| --- | --- | --- |
| 38 | `MIGRATOR.run` fails | **Test.** Reachable: tamper a checksum in `_sqlx_migrations`, then run again. It is also the only way to reach `From<(MigrateError, Driver)>` in `databases/error.rs` (row 13), so that test asserts `MigrationError` |
| 90 | the first query of the bootstrap fails | **Test.** Reachable with a closed pool: `create_database_tables` returns `ConnectionError` |
| 56 | a `DROP TABLE` fails | **Test.** Reachable the same way: `drop_database_tables` on a closed pool returns `ConnectionError` |
| 103, 122-123, 136, 152 | a later query, the connection acquire, `ensure_migrations_table` or the history insert fails | No decision: the first query of the function succeeds, so a failure here needs a connection that breaks mid-function; a closed pool fails at line 90 first. A fault-injecting pool would test `sqlx`, not this module |
| 139 | `continue` for a version already recorded | No decision: the bootstrap only reaches this loop right after `ensure_migrations_table` created an empty table, and the embedded migrations have distinct versions, so no version is ever recorded yet; defensive code |
| 279 | the `panic!` arm of an existing test | test code |

## Maintainer Decision

Plan approved on 2026-11-23.

## Test Design

The three tests join the file's `tests` module, on its `new_pool()` and `driver()` helpers (a driver
over the temporary database).

| Test | Act | Assert |
| --- | --- | --- |
| a tampered migration history | `create_database_tables`, then `UPDATE _sqlx_migrations SET checksum = X'00'`, then `create_database_tables` again | the second call is `MigrationError`; the tables created by the first call are still there |
| a closed pool, creating | close the pool, `create_database_tables` | `ConnectionError` |
| a closed pool, dropping | close the pool, `drop_database_tables` | `ConnectionError` |

## Property-Test Decision

No `quickcheck`: no input domain.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | the migrator failure is ignored (`MIGRATOR.run(..).await.ok()`) | the tampered-history test |
| M2 | the first bootstrap query failure is ignored (`unwrap_or(0)`) | the closed-pool creating test (the error becomes `MigrationError`) |
| M3 | a failed `DROP TABLE` is ignored | the closed-pool dropping test |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| G1 | TODO | The three tests. | They pass; lines 38, 56 and 90 run. |
| G2 | TODO | Run M1-M3. | Each is killed by its expected test. |
| G3 | TODO | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

To be filled in after G3.
