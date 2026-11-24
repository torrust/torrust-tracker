---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/driver/sqlite/mod.rs
status: done
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
| D1 | DONE | The tests. | They pass; lines 54-62 and 79 run. |
| D2 | DONE | Run M1-M6. | Each is killed by its expected test. |
| D3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Prose-First AAA Review

Temporary prose, written from this plan's Test Design when the review of #3009 found this section
missing (it belongs before the completed-file review), then compared with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_report_a_corrupted_aggregate_value_as_a_malformed_record` (cases) | A metric row whose value is below zero or above `u32`. | Load that metric. | A malformed record. | Matches the code. |
| `it_should_report_an_unreadable_aggregate_value_or_a_failed_query_as_an_invalid_query` (cases) | A metric row whose value is text, or no aggregate table. | Load that metric. | An invalid query. | Matches the code. |
| `it_should_report_a_failed_save_of_an_aggregate_metric_as_an_invalid_query` | A store without the aggregate table. | Save a metric. | An invalid query. | Matches the code. |
| `it_should_keep_each_aggregate_metric_separate_when_several_are_stored` | Metrics `a` as `1` and `b` as `2` saved. | Load `a`, `b` and the never saved `c`. | `(Some(1), Some(2), None)`. | Matches the code. |
| `it_should_overwrite_an_aggregate_metric_that_is_saved_again` | A metric saved with `1`. | Save it again with `7`. | It loads `7`. | Matches the code. |
| `it_should_store_and_load_back_the_largest_aggregate_value` | An empty store. | Save `u32::MAX`. | It loads `u32::MAX`. | Matches the code. |

Smells found: none. The helper `store_with_aggregate_metric` builds the Arrange from one argument,
and the case line states the stored value. The four multi-statement tests carry the AAA comments.

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage
  target directory, 2026-11-23): the file 87 / 97 lines (after the shared fixture of row 9) to
  137 / 139, regions 81 / 95 to 170 / 172, functions 16 / 20 to 32 / 32 (the counts include the
  fixture and the test module). The package's unit-only lines 4,853 / 5,643 (86.00%) to
  4,903 / 5,685 (86.24%), regions 5,375 / 6,310 (85.18%), functions 849 / 1,053 (80.63%). Left
  uncovered: the `unreachable!` arms of the two test helpers (lines 99 and 141): test code.
- **Tests:** 8 added (cases included), none changed: 239 to 247 unit tests in the package.
- **Mutations** (each applied alone and reverted, the file compared with its copy afterwards):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the two malformed-value cases |
  | M2 | Killed by the missing-table load case |
  | M3 | Killed by the not-an-integer case |
  | M4 | Killed by the failed-save test |
  | M5 | Killed by the save-again test |
  | M6 | Killed by the two-metrics test |

- **Review outcome:** the maintainer approved the completed file on 2026-11-23.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the
  existing `axum::serve` warning), `cargo +nightly fmt --all -- --check` and
  `cargo machete --with-metadata` pass.
