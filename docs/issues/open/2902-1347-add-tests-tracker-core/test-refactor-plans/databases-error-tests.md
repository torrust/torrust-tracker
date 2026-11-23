---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/error.rs
status: done
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
| E1 | DONE | The tests. | They pass. |
| E2 | DONE | Run M1-M8. | Each is killed by its expected case. |
| E3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage
  target directory, 2026-11-23): the file 35 / 35 lines (reached by rows 8-12) to 38 / 40 with the
  test module's helpers, regions 23 / 23 to 28 / 30, functions 4 / 4 to 5 / 5. The two lines left
  are the fallback arms of the test helpers (lines 147 and 158 at the S2 head, after the rebase on
  `develop` `f73a359f8`): test code. The package's unit-only
  lines 4,948 / 5,719 (86.52%) to 4,951 / 5,724 (86.50%; the new test helpers add measured lines),
  regions 5,440 / 6,366 (85.45%), functions 860 / 1,060 (81.13%). Line coverage did not show what
  these tests protect: run against the tests that existed before them (checked on the committed
  file), M2, M3, M5, M6 and M8 survive every test of the package; M4 was already killed by the
  closed-pool tests of row 12.
- **Tests:** 15 cases added (10 sqlx errors, one migration error, four drivers); the two existing
  tests are replaced by their cases in the first table: 250 to 263 unit tests in the package.
- **Mutations** (each applied alone and reverted, the file compared with its copy afterwards;
  M6 was first written wrongly, did not compile and was rewritten):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the `RowNotFound` case |
  | M2 | Killed by the `Tls` case |
  | M3 | Killed by the `PoolTimedOut` case |
  | M4 | Killed by the `PoolClosed` case and by the two closed-pool tests of row 12 |
  | M5 | Killed by the `WorkerCrashed` case |
  | M6 | Killed by the `Configuration` case |
  | M7 | Killed by the three `InvalidQuery` cases and by the `InvalidQuery` tests of rows 8-11 |
  | M8 | Killed by the migration failure case of the driver table |

- **Review outcome:** the maintainer approved the completed file on 2026-11-23.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the
  existing `axum::serve` warning), `cargo +nightly fmt --all -- --check` and
  `cargo machete --with-metadata` pass.
