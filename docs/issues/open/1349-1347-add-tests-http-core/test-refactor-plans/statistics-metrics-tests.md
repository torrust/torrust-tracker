---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/statistics/metrics.rs
status: done
semantic-links:
  related-artifacts:
    - packages/http-core/src/statistics/metrics.rs
    - packages/http-core/src/statistics/event/handler.rs
    - docs/issues/open/1349-1347-add-tests-http-core/test-refactor-plans/statistics-handler-tests.md
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Statistics Metrics File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/http-core/src/statistics/metrics.rs`.

## Current State

- **Unit-only coverage:** 48 / 57 lines (84.21%), 66 / 74 regions, 8 / 9 functions, measured at the S5 baseline.
  The totals include the test module. The uncovered lines are 40, 47 and 48: the `set_gauge` wrapper.
- **Existing tests:** 1, a regression test for #2560: a counter total of 2^53 + 1 is projected exactly, not through
  an `f64`.
- **Module-owned decisions:** `Metrics` is the read model of the HTTP statistics. It owns the projection of the
  generic metric collection to four public totals (`tcp4_announces_handled`, `tcp4_scrapes_handled`,
  `tcp6_announces_handled`, `tcp6_scrapes_handled`): each is the sum of every sample of
  `http_tracker_core_requests_received_total` whose `server_binding_address_ip_family` and `request_kind` labels
  match, whatever its other labels; a missing metric is `0`; a gauge of that name is `0` (the documented
  behavior of `counter_total`). It also forwards `increase_counter` and `set_gauge` to the collection. Summing,
  storing and the error cases of the collection are `torrust-metrics`'s.
- **T1 hypothesis:** test candidate.

## Current Tests Review

Reviewed against `write-unit-test`. Conclusion: **keep; add.**

| Test | Findings |
| --- | --- |
| `it_should_project_exactly_a_total_that_an_f64_cannot_represent` | A good regression test: `it_should_*` name, marked Arrange, Act and Assert, a named constant for the value, an explanatory comment. It reaches inside through `metric_collection.set_counter`, which is acceptable in-crate Arrange for a value that cannot be produced by incrementing. It covers one of the four getters. |
| Gaps | The other three getters are untested (a getter that swaps its family or kind passes); summing over samples that differ in other labels is untested; the zero cases (no samples, a gauge of that name) are untested; the two forwarding methods have no test. |

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Each getter selects its own family and request kind | 3A unit | R2a: set four samples of distinct values (one per family and kind), read the four getters as a tuple. A getter that swaps family or kind returns another value. |
| A total sums every sample with the matching family and kind, whatever its other labels | 3A unit | R2b: two samples of the same family and kind that differ in `server_binding_port`, plus one of another kind; the getter returns the sum of the two. |
| A missing metric and a gauge of the same name are `0` | 3A unit | R2c: an empty `Metrics::default()` gives `(0, 0, 0, 0)`; a gauge set under the counter's name gives `0` (the documented behavior); a freshly described `describe_metrics()` gives `(0, 0, 0, 0)`. |
| `increase_counter` and `set_gauge` forward to the collection | 3A unit | R2d: increasing a counter twice gives a total of 2, increasing a counter whose name is a gauge returns the collection's error, and a gauge set through `set_gauge` is readable with its value. |
| The `server_binding_address_ip_family` label is the family of the server's bind address, not of the client | 3A unit, decision | See D1. |
| `metric_collection()` accessor | covered | Used by the handler tests and the repository doc tests. |

No property candidate: four getters over a small label grid.

## Decisions needed

- **D1 (family semantics):** the doc comments say "requests from IPv4 peers" and "from IPv6 peers", but the
  getters select on the family of the server's bind address: an IPv6 client of an IPv4-bound tracker counts as
  `tcp4`. This looks like the intended per-listener meaning with a misleading comment. Options:
  (a) pin the behavior in a test named for the server binding and record the comment mismatch in the plan
  without a hand-off (recommended: it changes no behavior and the metric label name says "server binding"), or
  (b) hand the comment off through the findings procedure (draft spec and hand-off document under `.tmp`), or
  (c) leave the cross-family case untested.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | N/A | The one existing test is kept as it is. | Recorded in the plan. |
| R2 | DONE | Add R2a-R2d and the D1 case as decided. | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review; the named mutations G1-G5 each make a test fail. |
| R3 | DONE | Select an integration increment, or `none`. | Expected: `none`. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change that its test must catch; it is reverted after the run. Use a form that
compiles under the workspace lints.

| ID | Test | Site | Temporary change |
| --- | --- | --- | --- |
| G1 | R2a | `tcp4_scrapes_handled` | Select `inet6` instead of `inet`. |
| G2 | R2a | `tcp6_announces_handled` | Select `scrape` instead of `announce`. |
| G3 | R2c | `counter_total` | `unwrap_or(1)` instead of `unwrap_or_default()`. |
| G4 | R2d | `increase_counter` | Pass an empty `LabelSet` instead of `labels`. |
| G5 | R2d | `set_gauge` | Pass `0.0` instead of `value`. |

Gates: this plan needs approval first (including D1); the completed file needs the maintainer's completed-file
review before its one signed commit.

## Prose-First AAA Review

The existing regression test is unchanged. Nine cases in seven new test functions were added (10 cases in 8
functions in the file). A small set of helpers keeps each test to one arrangement: `request_labels(family, kind, port)`
builds a sample's labels, `set_requests` sets a sample to a value, `totals` returns the four public getters as a
tuple, and `metrics_with_a_gauge_named_like_the_request_counter` builds the gauge case.

| Test | Behavior it states | Cases | Expected value is |
| --- | --- | --- | --- |
| `it_should_project_each_getter_from_its_own_family_and_request_kind` | Each getter reads its own sample. | 1 (four samples valued 1, 2, 3, 4) | The tuple `(1, 2, 3, 4)`. |
| `it_should_sum_the_samples_of_a_family_and_request_kind_whatever_their_other_labels` | A total adds every matching sample, whatever its port, and no other kind. | 1 | `(7, 5, 0, 0)`. |
| `it_should_count_by_the_family_of_the_server_binding_and_not_of_the_client` | An IPv6 client of an IPv4-bound tracker counts as IPv4 (decision D1, option a). | 1 | `(1, 0, 0, 0)`. |
| `it_should_total_zero_when_there_is_no_request_counter_sample` | No metric, a described counter without samples, and a gauge under the counter's name all total zero. | 3 | `(0, 0, 0, 0)`. |
| `it_should_increase_a_counter_by_one_for_each_call` | `increase_counter` adds one to the sample of its labels. | 1 | `(2, 0, 0, 0)` after two calls. |
| `it_should_return_the_error_of_the_collection_when_it_increases_a_gauge_as_a_counter` | The forwarding returns the collection's error. | 1 | `is_err()` (the error value is `torrust-metrics`'s). |
| `it_should_store_the_value_of_a_gauge_set_through_the_metrics` | `set_gauge` stores the value at the labels. | 1 | `Some(Gauge::new(5.0))`. |

No test sleeps, mocks or compares floats except the one gauge value, which is exact by construction (`5.0`).
D1 is pinned as option (a): the comment mismatch ("peers" against the server binding's family) is recorded here and
in the handler plan, with no hand-off.

## Results

Unit-only coverage, measured on 2026-10-21 at 10:09 UTC.

| Scope | Lines before | Lines after | Regions before | Regions after | Functions before | Functions after |
| --- | --- | --- | --- | --- | --- | --- |
| `statistics/metrics.rs` | 48 / 57 (84.21%) | 141 / 141 (100.00%) | 66 / 74 | 238 / 238 | 8 / 9 | 19 / 19 |
| Package `http-core` (after all three last S5 files) | 2,614 / 2,648 (98.72%) | 2,818 / 2,840 (99.23%) | 3,339 / 3,380 (98.79%) | 3,679 / 3,709 (99.19%) | 329 / 332 (99.10%) | 359 / 360 (99.72%) |

The counted lines include the test module, which is why the file's counts rise by far more than its production
lines; every production line and function is covered (the uncovered `set_gauge` wrapper is now tested). The package
has 174 unit tests after the three last S5 files (157 before them). Production code is byte-identical to `HEAD`.

Named mutations (each reverted; the production diff was checked empty afterwards):

| ID | Result | Tests that failed |
| --- | --- | --- |
| G1 | killed | 2 (the getter table and the sum test) |
| G2 | killed | 1 (the getter table) |
| G3 | killed | 2 (the no-metric and gauge cases of the zero table; the described case passes, as expected) |
| G4 | killed | 1 (the increase test) |
| G5 | killed | 1 (the gauge test) |

G4 and G5 needed `std::hint::black_box` to keep the unused argument compiling under the workspace lints.

### Independent Review Round

The independent Task Reviewer showed that `increase_counter` and `set_gauge` could pass the wrong time to the
collection (`now + 7 s`) and all tests passed. Two tests now compare the whole `Metrics` value, after one increase
and after one `set_gauge` at 7 seconds, with a value built independently through the collection; each mutant is
killed by its own test. The file has 12 cases in 10 test functions, 167 / 167 lines, 295 / 295 regions and
21 / 21 functions.

## Approval

Approved by Jose Celano on 2026-10-21 (10:03 UTC): R2a-R2d, the D1 cross-family case pinned as option (a) (a test named for the server binding; the comment mismatch is recorded here and not handed off), R3 `none` and mutations G1-G5.

## Progress Log

- 2026-10-21 10:02 UTC - GitHub Copilot - Created the plan from `metrics.rs`, the S5 baseline and the handler plan.
  No Rust test or production code changed. The plan awaits maintainer approval, with one decision: D1.
- 2026-10-21 10:03 UTC - Jose Celano - Approved the plan with D1 (a): pin the server-binding family.
- 2026-10-21 10:09 UTC - GitHub Copilot - R2-R4 done: 9 cases in 7 new test functions, mutations G1-G5 killed,
  the file at 141 / 141 lines. The AAA review and the results are recorded above.
- 2026-10-21 10:23 UTC - Jose Celano - Approved the completed file. The plan is `done`.
