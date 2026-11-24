---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/driver/sqlite/schema_migrator.rs
status: done
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
| G1 | DONE | The three tests. | They pass; lines 38, 56 and 90 run. |
| G2 | DONE | Run M1-M3. | Each is killed by its expected test. |
| G3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Prose-First AAA Review

Temporary prose, written from this plan's Test Design when the review of #3009 found this section
missing (it belongs before the completed-file review), then compared with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_refuse_a_migration_history_that_does_not_match_the_embedded_migrations` | A database migrated once, then its recorded checksums overwritten. | Create the tables again. | A migration error, and the tables of the first run are still there. | Matches the code. |
| `it_should_report_a_closed_pool_as_a_connection_error_when_creating_the_tables` | A driver whose pool is closed. | Create the tables. | A connection error. | Matches the code. |
| `it_should_report_a_closed_pool_as_a_connection_error_when_dropping_the_tables` | A driver whose pool is closed. | Drop the tables. | A connection error. | Matches the code. |

Smells found: none. All three carry the AAA comments. The legacy-bootstrap tests of the file were not
changed.

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage
  target directory, 2026-11-23): the file 137 / 150 lines to 175 / 184, regions 129 / 153 to
  185 / 202, functions 15 / 23 to 24 / 29 (the counts include the test module). Lines 38, 56 and 90
  now run. The package's unit-only lines 4,903 / 5,685 (86.24%) to 4,948 / 5,719 (86.52%), regions
  5,435 / 6,359 (85.47%), functions 859 / 1,059 (81.11%). Left uncovered, as planned: 103, 122-123,
  136, 139 and 152 (no decision, reasons above) and 279 (test code). `databases/error.rs` is now
  35 / 35 lines: the tampered-history test reaches `From<(MigrateError, Driver)>`, which row 13 had
  planned to test.
- **Tests:** 3 added, none changed: 247 to 250 unit tests in the package.
- **Mutations** (each applied alone and reverted, the file compared with its copy afterwards):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the tampered-history test |
  | M2 | **Survived**, and is equivalent for this fault: on a closed pool the next query (line 103) fails with the same `ConnectionError`, so ignoring the first failure cannot be seen. The `?` of line 90 is covered but redundant under a closed pool; no other fault fails the first query and not the later ones |
  | M3 | Killed by the closed-pool dropping test |

- **Review outcome:** the maintainer approved the completed file on 2026-11-23, with M2 recorded as equivalent.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the
  existing `axum::serve` warning), `cargo +nightly fmt --all -- --check` and
  `cargo machete --with-metadata` pass.
