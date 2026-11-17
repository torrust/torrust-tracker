---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/statistics/repository.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/statistics/repository.rs
    - packages/tracker-core/src/statistics/metrics.rs
    - packages/tracker-core/src/statistics/mod.rs
    - packages/tracker-core/src/statistics/persisted/mod.rs
    - packages/tracker-core/src/statistics/event/handler.rs
    - docs/issues/open/2902-1347-add-tests-tracker-core/coverage-evidence.md
---

# Statistics Repository File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/tracker-core/src/statistics/repository.rs`, ledger row 1, the first file of S1.

## Current State

- **Unit-only coverage:** 86 / 140 lines (61.43%), 117 / 172 regions, 22 / 29 functions, at the S1
  baseline (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace,
  `--all-features --lib`, 2026-11-17). It is the T1 figure: no file under `packages/` changed
  between the measured commit and the S1 branch point.
- **Uncovered lines** (as `cargo-llvm-cov` reports them): 104 (the error arm of
  `increment_counter`), 114-120 (the entry of `set_counter`), 139-158 (`set_gauge`), 164-182
  (`increment_gauge`) and 188-206 (`decrement_gauge`). The async bodies make the per-line split
  of `set_counter` unreliable, so the decisions below are per behavior, not per line.
- **Existing tests:** 2 in the file: the persisted counter is omitted when persisted statistics are
  disabled, and it is exported at zero when they are enabled. The handler tests
  (`event/handler.rs`) reach `increment_counter` and the three `get_torrents_downloads_*` readers.
- **Module-owned decisions:**
  - Each of the five write operations takes the write lock, delegates to the metric collection,
    releases the lock, logs an error when the collection refuses the write, and returns the
    collection's result unchanged.
  - The three readers answer the value of one unlabeled counter, and `0` when that counter is not
    in the collection (`map_or(0, ...)`), for example the persisted counter when persisted
    statistics are disabled.
  - Owned elsewhere: which metrics exist at start-up (`describe_metrics` in `statistics/mod.rs`,
    row 7), the collection rules, including when a write is refused (`torrust-metrics`: a name
    already used by a metric of the other type gives `Error::MetricNameCollisionAdding`), and the
    thin `Metrics` wrapper (row 2).
- **Callers:** in this package, `increment_counter` (event handler) and `set_counter` (startup
  load, `persisted/mod.rs`). `set_gauge`, `increment_gauge` and `decrement_gauge` have no caller in
  the workspace, and the package describes no gauge (see the finding below).
- **T1 hypothesis:** test candidate (four counter and gauge methods and their error arms).

## Finding F6

`Repository::set_gauge`, `increment_gauge` and `decrement_gauge` are public but unused: no gauge is
described by `describe_metrics` and no code in the workspace calls them (`grep -rn` over
`packages/` and `src/`, 2026-11-17). Removing them is a public-API change of a published crate,
so it is not done here. Recorded as F6 in the register.

## Maintainer Decisions

The maintainer approved the plan on 2026-11-17, with decision D1: test the three unused gauge
methods as public API while they exist (F6 records them as unused; removal would be a separate
issue).

## Current Tests Review

Both existing tests are **clean**: `it_should_*` names, visible AAA, literal expectations, one
reason to fail each. One smell in the first test: it asserts three booleans with three `assert!`
calls, so a failure names only the line, not the metric. R1 replaces them with one `assert_eq!`
on the tuple against `(true, true, false)`, which prints all three values on failure.

## Coverage And Decision Analysis

| Behavior group | Lines | Classification | Decision |
| --- | --- | --- | --- |
| `increment_counter` refused by the collection | 104 | unit | R2: increment a name that is already a gauge; the result is `Err(MetricNameCollisionAdding)` with that name, and the gauge is unchanged. |
| `set_counter` success | 114-120 | unit | R2: set the persisted counter to `7` in a repository with persisted statistics enabled; `get_torrents_downloads_persisted_total` reads `7`. |
| `set_counter` refused | 129 | unit | R2: set a name that is already a gauge; `Err(MetricNameCollisionAdding)`. |
| `set_gauge` success and refusal | 139-158 | unit (D1) | R2: set a new gauge to `2.5` and read it back from the collection; set the deprecated persistent counter's name as a gauge and get `Err(MetricNameCollisionAdding)`, with the counter still `0`. |
| `increment_gauge` success and refusal | 164-182 | unit (D1) | R2: from `2.5` to `3.5`; the counter-name case is refused as above. |
| `decrement_gauge` success and refusal | 188-206 | unit (D1) | R2: from `2.5` to `1.5`; the counter-name case is refused as above. |
| Reader of an absent counter | covered | unit | R2: with persisted statistics disabled, `get_torrents_downloads_persisted_total` answers `0`. The line is reached today but its result is not asserted; this pins the `map_or(0, ...)` decision. |
| The `tracing::error!` messages | the five error arms | not asserted | The log text is diagnostic, not a contract; the error value returned is asserted instead. |
| Lock release before logging (`drop(stats_lock)`) | covered | not tested | Not observable from a single-task test: the guard is dropped at the end of the function anyway. |
| When the collection refuses a write, and the values it holds | - | collaborator-owned | `torrust-metrics` has its own tests for each collision; these tests only prove that the repository returns the refusal unchanged and keeps the earlier value. |

## Test Design

- **Location:** the existing `#[cfg(test)] mod tests` of `repository.rs`.
- **Arrange:** `Repository::new(true, true)` (or `new(true, false)` for the absent-counter case).
  The refusal of a counter write needs a gauge in the collection; the test creates one with
  `set_gauge` in its Arrange and asserts that call succeeded, so a broken `set_gauge` fails in the
  Arrange with its own message instead of pretending to be a counter failure.
- **Fixed time:** `DurationSinceUnixEpoch::from_secs(0)`, a literal; no clock.
- **Reading back:** the counter readers for counters; `read_metrics` with
  `metric_collection().get_gauge_value(...)` for gauges, compared as `Option<f64>`.
- **Error assertion:** `matches!(result, Err(Error::MetricNameCollisionAdding { metric_name }) if
  metric_name == metric_name!("..."))`, the pattern of the `torrust-metrics` tests.
- **Shape:** one test per operation and outcome, about ten small tests, no loops and no table
  helper. A table over the five operations would need a closure per operation, which hides the Act.
- **Asynchronous work:** none spawned; `#[tokio::test]` only because the methods are `async`.
- **Dependencies:** none new.

## Property-Test Decision

Not selected. The candidate property (incrementing `n` times reads `n`) belongs to the collection
(`torrust-metrics`); the repository only delegates, so examples prove the delegation.

## Named Mutations

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| M1 | `increment_counter` | Return `Ok(())` instead of `result`. | the counter-on-gauge refusal test |
| M2 | `set_counter` | Call `stats_lock.increment_counter(...)` instead of `set_counter` (value ignored). | the `set_counter` success test (reads `1`, not `7`) |
| M3 | `set_gauge` | Return `Ok(())` instead of `result`. | the `set_gauge` refusal test |
| M4 | `increment_gauge` | Call `decrement_gauge` instead. | the `increment_gauge` success test |
| M5 | `decrement_gauge` | Call `increment_gauge` instead. | the `decrement_gauge` success test |
| M6 | `get_counter_value` | `map_or(1, ...)` instead of `map_or(0, ...)`. | the absent-counter test |
| M7 | `get_torrents_downloads_persisted_total` | Read the in-session counter instead. | the `set_counter` success test |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Replace the three `assert!` calls of the first existing test with one tuple `assert_eq!`. | Focused tests pass. |
| R2 | DONE | Add the tests of the analysis table (the gauge ones only if D1 says so). | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, the prose-first AAA review, and the named mutations. |
| R3 | DONE | Integration increment: none expected; the integration binary reaches this file through the event handler. | Recorded here. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Prose-First AAA Review

Temporary prose, written before comparing with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_omit_persisted_metric_when_persisted_completed_statistics_are_disabled` (R1) | A repository with persisted statistics disabled. | Ask the collection whether each of the three download counters exists. | The three answers, as one tuple, are present, present, absent. | One `assert_eq!` on the tuple replaces three `assert!` calls, so a failure prints all three values. Kept. |
| `it_should_export_persisted_metric_with_zero_value_when_persisted_completed_statistics_are_enabled` | A repository with persisted statistics enabled. | Ask whether the persisted counter exists. | It exists and reads `0`. | Unchanged (clean). |
| `it_should_read_a_counter_that_is_not_in_the_collection_as_zero` | A repository with persisted statistics disabled, so the persisted counter is not described. | Read the persisted total. | It is `0`. | Kept; pins the `map_or(0, ...)` decision. |
| `it_should_set_a_counter_to_the_given_value` | A repository with persisted statistics enabled. | Set the persisted counter to `7`. | The call succeeds and the persisted total reads `7`. | Kept. |
| `it_should_refuse_to_set_a_counter_that_has_the_name_of_a_gauge` | A repository with a gauge `test_gauge` set to `2.5`. | Set a counter with the gauge's name. | A name-collision error on that name, and the gauge still reads `2.5`. | Kept. The gauge is created by `set_gauge` in the Arrange helper, which fails with its own message if that call breaks. |
| `it_should_refuse_to_increment_a_counter_that_has_the_name_of_a_gauge` | The same gauge. | Increment a counter with the gauge's name. | The same error, and the gauge unchanged. | Kept. |
| `it_should_set_a_gauge_to_the_given_value` | A repository with no gauge. | Set `test_gauge` to `2.5`. | The call succeeds and the gauge reads `2.5`. | Kept. |
| `it_should_refuse_to_set_a_gauge_that_has_the_name_of_a_counter` | A repository; the deprecated persistent counter exists at `0`. | Set a gauge with that counter's name. | A name-collision error on that name, and the counter still `0`. | Kept. |
| `it_should_increment_a_gauge_by_one` | A gauge at `2.5`. | Increment it. | The call succeeds and the gauge reads `3.5`. | Kept. |
| `it_should_refuse_to_increment_a_gauge_that_has_the_name_of_a_counter` | As the set-gauge refusal. | Increment a gauge with the counter's name. | The same error, and the counter still `0`. | Kept. |
| `it_should_decrement_a_gauge_by_one` | A gauge at `2.5`. | Decrement it. | The call succeeds and the gauge reads `1.5`. | Kept. |
| `it_should_refuse_to_decrement_a_gauge_that_has_the_name_of_a_counter` | As the set-gauge refusal. | Decrement a gauge with the counter's name. | The same error, and the counter still `0`. | Kept. |

Helpers: `repository_with_a_gauge_set_to` (Arrange), `gauge` (reads one unlabeled gauge through
`read_metrics`), `is_a_name_collision_on` (the error pattern), `no_labels` and the literal `NOW`.
Gauges are compared as `Option<Gauge>` (`Gauge` derives `PartialEq`), which keeps the exact values
`2.5`, `3.5` and `1.5` without a float comparison that `clippy::float_cmp` would reject.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace,
  `--all-features --lib`): `statistics/repository.rs` 86 / 140 lines at the S1 baseline; 277 / 277
  lines (100.00%), 394 / 394 regions and 57 / 57 functions after (the counts include the test
  module). Package unit-only lines after this file: 4,324 / 5,241 (82.50%), regions 6,023 / 7,349
  (81.96%), functions 726 / 982 (73.93%), 167 unit tests (157 before).
- **Covered but not asserted:** `statistics/metrics.rs` (row 2) is now 45 / 45 lines, because every
  repository write calls its `Metrics` wrapper. Row 2 decides whether that needs its own tests.
- **Tests:** 10 added, 1 changed (R1), 1 unchanged. No integration test was added or changed.
- **Mutations** (each applied alone on the production part of the file, stable toolchain, then the
  file restored from a copy and compared with `cmp`): M1 killed by
  `it_should_refuse_to_increment_a_counter_that_has_the_name_of_a_gauge`; M2 by
  `it_should_set_a_counter_to_the_given_value`; M3 by
  `it_should_refuse_to_set_a_gauge_that_has_the_name_of_a_counter`; M4 by
  `it_should_increment_a_gauge_by_one`; M5 by `it_should_decrement_a_gauge_by_one`; M6 by
  `it_should_read_a_counter_that_is_not_in_the_collection_as_zero`; M7 by
  `it_should_set_a_counter_to_the_given_value` and six handler and listener tests. M7 first did not
  compile (it left the persisted-counter import unused under `unused = "deny"`) and was redone with
  `let _ = TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL;` so that it compiles.
- **Dependencies:** none new. **Clippy allows:** none.
- **Validation:** `cargo test -p torrust-tracker-core --all-features --lib` (167 passed),
  `cargo clippy -p torrust-tracker-core --all-targets --all-features` (stable; its only warning is
  the existing `axum::serve` entry of `clippy.toml`), `cargo +nightly fmt --all -- --check`
  (nightly) and `git diff --check` pass.
- **Review outcome:** the maintainer approved the plan (with D1) and the completed file on 2026-11-17.
