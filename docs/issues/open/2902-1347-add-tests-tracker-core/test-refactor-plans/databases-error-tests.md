---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/error.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/databases/error.rs
    - packages/tracker-core/src/databases/driver/sqlite/schema_migrator.rs
---

# Database Error File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 13,
`packages/tracker-core/src/databases/error.rs`: the conversions from `sqlx` and migration errors
into the driver-neutral `Error`.

## Current State

- **Unit-only coverage:** 28 / 35 lines at the S2 baseline; 35 / 35 now (stable Rust 1.100.0,
  `cargo-llvm-cov 0.9.1`, `--all-features --lib`, 2026-11-23): the test of row 12 that refuses a
  tampered migration history reached `From<(MigrateError, Driver)>`, and the SQLite store tests of
  rows 8 to 11 reached the `InvalidQuery` arm.
- **Existing tests:** 2 in the file: `RowNotFound` gives `QueryReturnedNoRows` and an I/O error gives
  `ConnectionError`. The row 12 test asserts only that the migration conversion gives
  `MigrationError`; none asserts the driver that the error carries.
- **Module-owned decisions:**
  - `RowNotFound` is `QueryReturnedNoRows`.
  - `Io`, `Tls`, `PoolTimedOut`, `PoolClosed`, `WorkerCrashed` and `Configuration` are
    `ConnectionError`: the database cannot be reached.
  - Every other `sqlx` error is `InvalidQuery`.
  - A migration error is `MigrationError`.
  - Every conversion keeps the driver it was given.

Line coverage hides these decisions: an or-pattern shows as covered when one alternative runs, and
no test reads the driver back.

## Maintainer Decision

Plan approved on 2026-11-23. The two existing tests are subsumed by the first table (their `RowNotFound` and `Io` cases) and replaced by it.

## Test Design

Two table tests in the file's `tests` module, with two small helpers that name an error's variant and
read its driver.

| Test | Cases | Assert |
| --- | --- | --- |
| each `sqlx` error is classified (`rstest`) | `RowNotFound`; `Io`, `Tls`, `PoolTimedOut`, `PoolClosed`, `WorkerCrashed`, `Configuration`; `ColumnNotFound`, `Decode`, `Protocol` | `QueryReturnedNoRows`; `ConnectionError` for the six; `InvalidQuery` for the three |
| a migration error is classified | `MigrateError::VersionMissing(1)` | `MigrationError` |
| the driver is kept (`rstest`) | one case of each conversion above, converted with `Driver::PostgreSQL` | the error carries `PostgreSQL` |

## Property-Test Decision

No `quickcheck`: the inputs are the variants of a closed enum, listed in the table.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | `RowNotFound` is `InvalidQuery` | the `RowNotFound` case |
| M2 | `Tls` falls to `InvalidQuery` (removed from the `ConnectionError` pattern) | the `Tls` case |
| M3 | `PoolTimedOut` falls to `InvalidQuery` | the `PoolTimedOut` case |
| M4 | `PoolClosed` falls to `InvalidQuery` | the `PoolClosed` case |
| M5 | `WorkerCrashed` falls to `InvalidQuery` | the `WorkerCrashed` case |
| M6 | `Configuration` falls to `InvalidQuery` | the `Configuration` case |
| M7 | the fallback arm is `ConnectionError` | the three `InvalidQuery` cases |
| M8 | the migration conversion reports `Driver::Sqlite3` whatever it is given | the driver case of the migration error |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| E1 | TODO | The tests. | They pass. |
| E2 | TODO | Run M1-M8. | Each is killed by its expected case. |
| E3 | TODO | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

To be filled in after E3.
