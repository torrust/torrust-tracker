---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/driver/sqlite/mod.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/databases/driver/sqlite/mod.rs
    - packages/tracker-core/src/databases/driver/sqlite/torrent_metrics_store.rs
    - packages/tracker-core/migrations/sqlite
---

# SQLite Driver Module File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 11,
`packages/tracker-core/src/databases/driver/sqlite/mod.rs`: the driver's constructor and the two
aggregate-metric helpers `load_torrent_aggregate_metric` and `save_torrent_aggregate_metric`, which
the global-downloads methods of the torrent metrics store call.

## Current State

- **Unit-only coverage:** 75 / 84 lines at the S2 baseline, 87 / 97 after rows 8 to 10 (the
  fixture and its use in the tests added lines) (`develop` at `83f3ecbbc`, stable Rust 1.100.0,
  `cargo-llvm-cov 0.9.1`, `--all-features --lib`). Uncovered production lines:
  - 54: the query failure of `load_torrent_aggregate_metric`;
  - 58: the decode of its `value` column;
  - 60-62: a stored `value` outside `u32`;
  - 79: the execute failure of `save_torrent_aggregate_metric`.

  Lines 99 and 136 are the `unreachable!` arms of test helpers: test code.
- **Existing tests:** the shared driver suite (through `run_sqlite_driver_tests`) saves, loads and
  increases the global total, and `create_database_tables_should_be_idempotent_on_a_fresh_database`
  checks the four migrations are recorded once.
- **Module-owned decisions:**
  - The helpers read and write one row of `torrent_aggregate_metrics` per metric name, never another
    metric's row.
  - Saving a name again overwrites its value; a stored value outside `u32` is
    `MalformedDatabaseRecord`; a column of the wrong type or a failed query is `InvalidQuery`.
  - The value is stored as a signed 64-bit integer: the largest `u32` must round-trip.

## Maintainer Decision

Plan approved on 2026-11-23.

## Test Design

Tests join the existing `tests` module of the file, on the shared `migrated_store()` and `execute()`.
The helpers are private to the module, so the tests call them directly with metric names of their own,
which also shows the metric name is honoured.

| Test | Cases | Assert |
| --- | --- | --- |
| loading a bad aggregate row (`rstest`) | `a_value_below_zero` (`-1`), `a_value_above_u32` (`4294967296`) | `MalformedDatabaseRecord` |
| loading an unreadable aggregate row or failing (`rstest`) | `a_value_that_is_not_an_integer` (`'many'`), `a_missing_aggregate_metrics_table` | `InvalidQuery` |
| saving on a missing table | `DROP TABLE torrent_aggregate_metrics` | `InvalidQuery` |
| two metrics side by side | save `a` as 1 and `b` as 2 | `a` loads 1, `b` loads 2, `c` loads `None` |
| saving a name again | `1` then `7` | the stored value is 7 |
| the largest value | `u32::MAX` | it loads back as `u32::MAX` |

## Property-Test Decision

No `quickcheck`: the inputs are SQL rows and named boundaries.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | the load saturates a value outside `u32` (`unwrap_or(u32::MAX)`) | the two malformed cases |
| M2 | the load ignores a failed query (`unwrap_or(None)`) | the missing-table load case |
| M3 | the load ignores a value it cannot decode (`unwrap_or(0)`) | the not-an-integer case |
| M4 | the save ignores a failed statement | the missing-table save test |
| M5 | the save does nothing when the name exists (`DO NOTHING`) | the save-again test |
| M6 | the load ignores the metric name (`WHERE metric_name = ?1` becomes `WHERE ?1 IS NOT NULL`) | the two-metrics test |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| D1 | TODO | The tests. | They pass; lines 54-62 and 79 run. |
| D2 | TODO | Run M1-M6. | Each is killed by its expected test. |
| D3 | TODO | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

To be filled in after D3.
