---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/statistics/persisted/mod.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/statistics/persisted/mod.rs
    - packages/tracker-core/src/statistics/persisted/downloads.rs
    - packages/tracker-core/src/statistics/repository.rs
    - packages/tracker-core/src/databases/traits/torrent_metrics.rs
    - packages/tracker-core/tests/common/test_env.rs
---

# Statistics Persisted Metrics Loader File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/tracker-core/src/statistics/persisted/mod.rs`, ledger row 3.

## Current State

- **Unit-only coverage:** 0 / 11 lines, at the S1 baseline and after rows 1 and 2 (stable Rust
  1.99.1, `cargo-llvm-cov 0.6.16`, `--all-features --lib`). Uncovered: 21-46
  (`load_persisted_metrics`) and 58-68 (the two `From` conversions into `Error`).
- **Callers:** the tracker start-up (`src/app.rs:385`) and the integration test environment
  (`tests/common/test_env.rs:138`), which unwraps the result on a real SQLite database with no
  stored total. No unit test calls it.
- **Module-owned decisions:**
  - With a stored global total `N`, both the persistent total (deprecated name) and the persisted
    total are set to `N`; the in-session total is not touched.
  - With no stored total, nothing is written and the result is `Ok`.
  - A database failure is returned as `Error::DatabaseError`, and nothing is written.
  - A refused metric write is returned as `Error::MetricsError`; the first refusal stops the load.
    The load is not atomic: when the second write is refused, the first stays. Only the second can
    be refused in practice, because `describe_metrics` always registers the persistent total as a
    counter, while the persisted total exists only when persisted statistics are enabled.

## Maintainer Decision

Plan approved on 2026-11-18, including the test that pins the non-atomic load.

## Test Design

Unit tests in a new `tests` module of the file. The database is `MockTorrentMetricsStore`
(`automock` under the `testing` feature, as `scrape_handler.rs` uses it), wrapped in
`DatabaseDownloadsMetricRepository`; the statistics repository is a real
`Repository::new(true, true)`. Values are read back with the repository's three readers.

| Test | Arrange | Assert |
| --- | --- | --- |
| `it_should_set_the_persistent_and_the_persisted_totals_to_the_stored_global_downloads` | the store returns `Some(7)` | `Ok`; both totals read 7; the in-session total reads 0 |
| `it_should_set_both_totals_to_the_largest_stored_value` | the store returns `Some(u32::MAX)` | both totals read `u64::from(u32::MAX)` (the widening is lossless) |
| `it_should_leave_the_totals_at_zero_when_no_global_downloads_are_stored` | the store returns `None` | `Ok`; both totals read 0 |
| `it_should_return_a_database_error_and_load_nothing_when_the_store_fails` | the store returns `MalformedDatabaseRecord` | `Err(Error::DatabaseError { .. })`; both totals read 0 |
| `it_should_return_a_metrics_error_and_keep_the_first_total_when_the_repository_refuses_the_second` | the store returns `Some(7)`; `Repository::new(true, false)` and the persisted total's name already holds a gauge (`set_gauge` first) | `Err(Error::MetricsError { .. })`; the persistent total reads 7 (the load is not atomic) |

Each mock expectation uses `.times(1)`, so a second database read also fails the test.

## Property-Test Decision

No `quickcheck`: the function is async and has one numeric input, whose only rule is the lossless
`u32` to `u64` widening; the `Some(7)` and `Some(u32::MAX)` examples pin it. A property would need
a runtime per case for no extra mutant.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | Remove the persistent-total write | the `Some(7)` test |
| M2 | Remove the persisted-total write | the `Some(7)` test |
| M3 | Write the in-session total instead of the persisted total | the `Some(7)` test (in-session reads 7) |
| M4 | `if let Some` branch skipped (always `Ok(())`) | the `Some(7)` test |
| M5 | Database error swallowed (`.unwrap_or(None)`) | the database-error test |
| M6 | Metrics error swallowed (`let _ =` on the second write) | the metrics-error test |
| M6b | The two writes swapped (persisted first) | the metrics-error test (the persistent total reads 0) |
| M7 | `u64::from(downloads) + 1` | the `Some(7)` test |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| P1 | TODO | Add the five tests. | They pass; the file is 11 / 11. |
| P2 | TODO | Run M1-M7 and M6b. | Each is killed by its expected test. |
| P3 | TODO | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

To be filled in after P3.
