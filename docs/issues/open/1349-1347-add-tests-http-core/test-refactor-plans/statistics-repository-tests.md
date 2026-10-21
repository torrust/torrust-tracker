---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/statistics/repository.rs
status: done
semantic-links:
  related-artifacts:
    - packages/http-core/src/statistics/repository.rs
    - packages/http-core/src/statistics/mod.rs
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Statistics Repository File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/http-core/src/statistics/repository.rs`.

## Current State

- **Unit-only coverage:** 21 / 24 lines (87.50%), 26 / 29 regions, 5 / 6 functions, measured at the S5 baseline.
  The uncovered lines are 58-60: `Default::default`. Three doc tests (two `compile_fail` for the private lock and
  collection field, one read example) are not counted by the unit-only measure.
- **Existing tests:** no unit test; the repository is exercised through the handler and listener tests.
- **Module-owned decisions:** `Repository` is the shared, cloneable store of the HTTP statistics. It starts with
  the request counter described and at zero (`new` and `Default`); a clone shares the same store; `read_metrics`
  gives a consistent view of the current state; `increase_counter` adds one to the sample with the given labels and
  returns the collection's error. The lock is private (pinned by the doc tests). Counter arithmetic and its errors
  are `torrust-metrics`'s.
- **T1 hypothesis:** test candidate.

## Current Tests Review

Nothing to review: no unit test exists. The doc tests are the contract of the private fields and stay as they are.

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| A new and a default repository start with the request counter described and at zero | 3A unit | R2a: a table over `Repository::new()` and `Repository::default()`; the view has the described counter (`contains_counter`) and the four totals are `(0, 0, 0, 0)`. Covers `Default`, which `Metrics::default()` would break. |
| `increase_counter` adds one to the sample of its labels, and only to it | 3A unit | R2b: two increases of one label set and one of another give exact samples of 2 and 1 (`get_counter_value` by exact labels). |
| A clone shares the store | 3A unit | R2c: an increase through a clone is visible through the original and the other way round. |
| Concurrent increases are not lost | 3A unit | R2d: many increases joined concurrently give the exact total. It pins the lock, which a lost update would break. |
| The error of `increase_counter` | not tested | The collection fails only when a metric of another type has the counter's name; the repository always describes a counter and offers no way to add another metric, so the case is unreachable through the repository. Owner: `torrust-metrics`. |
| `read_metrics` holding the read lock while the closure runs | not tested | A lock-contention timing test needs sleeps or a deadlock detector; the doc comment states the contract. |

No property candidate: four small operations.

## Test Bootstrap

A fixed `DurationSinceUnixEpoch` and two literal label sets. Each test builds its own `Repository`; no shared
state, no clock, no I/O.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | N/A | No existing unit test. | Recorded in the plan. |
| R2 | DONE | Add R2a-R2d. | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review; the named mutations B1-B4 each make a test fail. |
| R3 | DONE | Select an integration increment, or `none`. | Expected: `none`. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results and `coverage-evidence.md`. |

## Named Mutations

| ID | Test | Site | Temporary change |
| --- | --- | --- | --- |
| B1 | R2a | `Default::default` | Build an empty `Metrics::default()` repository instead of `Self::new()`. |
| B2 | R2b | `increase_counter` | Ignore `labels` (use an empty set). |
| B3 | R2c | `Clone` | Replace the derive by a manual `Clone` that returns `Self::new()`. |
| B4 | R2b, R2d | `increase_counter` | Increase the counter twice per call. |

R2d (concurrent increases) has no named mutation: the repository's one lock cannot be removed by hand without
changing the type, so the test is recorded as a design check that a lost update would fail, not as a mutation
proof.

Gates: this plan needs approval first; the completed file needs the maintainer's completed-file review before its
one signed commit.

## Prose-First AAA Review

Five cases in four test functions, in a new test module. Each builds its own `Repository` and uses two helpers:
`increase_requests` (increase the request counter for given labels) and `requests_with_exactly` (the value of the
sample with exactly those labels, or `None`).

| Test | Behavior it states | Cases | Expected value is |
| --- | --- | --- | --- |
| `it_should_start_with_the_request_counter_described_and_at_zero` | `new` and `Default` both register the counter and start at zero. | 2 | The counter is described; the four totals are `(0, 0, 0, 0)`. |
| `it_should_add_one_to_the_sample_of_the_given_labels_and_to_no_other` | `increase_counter` adds one per call to its own sample. | 1 (announce twice, scrape once) | `Some(2)` and `Some(1)`. |
| `it_should_share_its_store_with_its_clones` | A clone sees the original's increases and the other way round. | 1 | `Some(2)` through both handles. |
| `it_should_not_lose_increases_made_concurrently` | Increases from eight tasks of twenty-five add up. | 1 | `Some(200)`. |

The concurrency test is a design check, not a mutation proof (see the plan): it would fail a lost update, but no
hand-made mutation of the one lock exists. It uses a multi-thread runtime of four workers and no sleep.

## Results

Unit-only coverage, measured on 2026-10-21 at 10:09 UTC.

| Scope | Lines before | Lines after | Regions before | Regions after | Functions before | Functions after |
| --- | --- | --- | --- | --- | --- | --- |
| `statistics/repository.rs` | 21 / 24 (87.50%) | 108 / 108 (100.00%) | 26 / 29 | 161 / 161 | 5 / 6 | 21 / 21 |

The counted lines include the new test module; every production line is covered, including `Default::default`,
which was the uncovered code. The three doc tests are unchanged. Production code is byte-identical to `HEAD`.

Named mutations (each reverted; the production diff was checked empty afterwards):

| ID | Result | Tests that failed |
| --- | --- | --- |
| B1 | killed | the `Default` case of the start test |
| B2 | killed | 3 (the add, clone and concurrency tests) |
| B3 | killed | 2 (the clone and concurrency tests) |
| B4 | killed | 3 (the add, clone and concurrency tests) |

B2 and B3 needed `std::hint::black_box` to keep the unused value compiling under the workspace lints. The
failed-increase `Err` arm of `increase_counter` stays untested, as decided: the repository offers no way to put a
metric of another type under the counter's name.

## Approval

Approved by Jose Celano on 2026-10-21 (10:03 UTC): R2a-R2d, R3 `none` and mutations B1-B4.

## Progress Log

- 2026-10-21 10:02 UTC - GitHub Copilot - Created the plan from `repository.rs` and the S5 baseline. No Rust test
  or production code changed. The plan awaits maintainer approval.
- 2026-10-21 10:03 UTC - Jose Celano - Approved the plan.
- 2026-10-21 10:09 UTC - GitHub Copilot - R2-R4 done: 5 cases in 4 new test functions, mutations B1-B4 killed,
  the file at 108 / 108 lines. The AAA review and the results are recorded above.
- 2026-10-21 10:23 UTC - Jose Celano - Approved the completed file. The plan is `done`.
