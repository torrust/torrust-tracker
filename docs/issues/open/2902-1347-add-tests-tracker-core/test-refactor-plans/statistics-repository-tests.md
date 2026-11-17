---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/statistics/repository.rs
status: proposed
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

## Finding Proposed: F6

`Repository::set_gauge`, `increment_gauge` and `decrement_gauge` are public but unused: no gauge is
described by `describe_metrics` and no code in the workspace calls them (`grep -rn` over
`packages/` and `src/`, 2026-11-17). Removing them is a public-API change of a published crate,
so it is not done here. Proposed register entry: F6, handed to a separate issue if the maintainer
wants them removed.

## Maintainer Decisions

None yet. The plan asks for one decision (D1):

- **D1:** test the three unused gauge methods as public API (recommended: they are part of the
  published crate API and the tests are three small cases; a removal issue would delete them with
  the methods), or leave them uncovered with F6 as their owner.

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
| R1 | TODO | Replace the three `assert!` calls of the first existing test with one tuple `assert_eq!`. | Focused tests pass. |
| R2 | TODO | Add the tests of the analysis table (the gauge ones only if D1 says so). | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, the prose-first AAA review, and the named mutations. |
| R3 | TODO | Integration increment: none expected; the integration binary reaches this file through the event handler. | Recorded here. |
| R4 | TODO | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Prose-First AAA Review

Filled in R2, before the completed-file review.

## Results

Filled in R4.
