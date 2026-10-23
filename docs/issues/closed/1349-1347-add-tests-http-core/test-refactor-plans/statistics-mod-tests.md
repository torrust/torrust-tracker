---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/statistics/mod.rs
status: done
semantic-links:
  related-artifacts:
    - packages/http-core/src/statistics/mod.rs
    - packages/http-core/src/statistics/repository.rs
    - docs/issues/closed/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Statistics Module File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/http-core/src/statistics/mod.rs`.

## Current State

- **Unit-only coverage:** 8 / 8 lines (100.00%), 10 / 10 regions, 1 / 1 functions. `describe_metrics` is called by
  every repository test, but nothing asserts what it registers.
- **Existing tests:** none in the file.
- **Module-owned decisions:** the public name of the request counter, `http_tracker_core_requests_received_total`
  (the Prometheus series name that dashboards and alerts read; it appears in the dual-stack experiment output under
  `contrib/dev-tools/experiments/`), and the description
  and unit that `describe_metrics` registers for it. The counter's labels are `event.rs`'s.
- **T1 hypothesis:** probably a small change: pin the metric-name contract.

## Current Tests Review

Nothing to review. The constant and the description are public contract with no assertion, so a rename or a
dropped description would not fail any test.

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| The counter's public name | 3A unit | R2a: the constant equals the literal `http_tracker_core_requests_received_total`. |
| `describe_metrics` registers the counter with its description and unit | 3A unit | R2b: after one sample is added, the Prometheus exposition text of the collection contains the `# HELP` line with the description and the `# TYPE ... counter` line, under the literal name. The exposition format is how the description and the unit are observable. |
| A fresh `describe_metrics()` has no sample | 3A unit | R2c: the exposition text is empty (the collection prints only metrics that have samples) and `contains_counter` is true. |
| `Metrics` default and serialization | not tested | `Metrics` derives `Default` and `Serialize`; the derives are the compiler's. |

No property candidate.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | N/A | No existing test. | Recorded in the plan. |
| R2 | DONE | Add R2a-R2c in a test module of `mod.rs`. | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review; the named mutations D1-D3 each make a test fail. |
| R3 | DONE | Select an integration increment, or `none`. | Expected: `none`. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results and `coverage-evidence.md`. |

## Named Mutations

| ID | Test | Site | Temporary change |
| --- | --- | --- | --- |
| D1 | R2a | `HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL` | Change the value (for example `http_tracker_core_requests_total`). |
| D2 | R2b | `describe_metrics` | Change the description text. |
| D3 | R2b, R2c | `describe_metrics` | Do not describe the counter. |

Gates: this plan needs approval first; the completed file needs the maintainer's completed-file review before its
one signed commit.

## Prose-First AAA Review

Three tests in a new test module of `mod.rs`.

| Test | Behavior it states | Cases | Expected value is |
| --- | --- | --- | --- |
| `it_should_name_the_request_counter_http_tracker_core_requests_received_total` | The public series name. | 1 | The literal `http_tracker_core_requests_received_total`. |
| `it_should_describe_the_request_counter_without_any_sample` | `describe_metrics` registers the counter, and a collection with no sample prints nothing. | 1 | `contains_counter` is true and the exposition text is empty. |
| `it_should_describe_the_request_counter_with_its_description_and_type` | The description and type a scraper reads. | 1 | The exact three-line exposition text (`# HELP`, `# TYPE ... counter`, the sample), written as a literal. |

The literal text was first read from the collection's output and then checked against the registered description
and the metric type by hand; it is the Prometheus contract the plan wanted pinned. The unit (`Unit::Count`) is not
part of this output. This paragraph first said that no test could see it; that was wrong, and the review round
added a whole-value comparison that does (see "Independent Review Round").

## Results

Unit-only coverage, measured on 2026-10-21 at 10:09 UTC.

| Scope | Lines before | Lines after | Regions before | Regions after | Functions before | Functions after |
| --- | --- | --- | --- | --- | --- | --- |
| `statistics/mod.rs` | 8 / 8 (100.00%) | 32 / 32 (100.00%) | 10 / 10 | 43 / 43 | 1 / 1 | 4 / 4 |

The file was already fully covered, by tests that never asserted what it registers; the counted lines now include
the new test module. Production code is byte-identical to `HEAD`.

Named mutations (each reverted; the production diff was checked empty afterwards):

| ID | Result | Tests that failed |
| --- | --- | --- |
| D1 | killed | 2, both new: no other test of the package failed, because every other test reads the name through the constant, so the value was unpinned before |
| D2 | killed | the description test |
| D3 | killed | 4: the two new describe tests and the two start tests of the repository |

During D2 one edit removed lines of the file by mistake (the old text of the edit was longer than its
replacement); the removed lines were restored from the text just printed and the production code was checked
byte-identical to `HEAD` before the mutation was redone with a one-line edit.

### Independent Review Round

The independent Task Reviewer showed that `Some(Unit::Seconds)` in place of `Some(Unit::Count)` survived all tests,
although the plan's coverage table pointed at the exposition text for "the description and the unit": that text
omits the unit. A new test compares `describe_metrics()` with a `Metrics` built independently (the same name,
`Unit::Count` and the description as literals), which also pins the unit; the mutant is killed by it. The file has
4 tests, 41 / 41 lines, 55 / 55 regions and 5 / 5 functions.

## Approval

Approved by Jose Celano on 2026-10-21 (10:03 UTC): R2a-R2c, R3 `none` and mutations D1-D3.

## Progress Log

- 2026-10-21 10:02 UTC - GitHub Copilot - Created the plan from `mod.rs` and the S5 baseline. No Rust test or
  production code changed. The plan awaits maintainer approval.
- 2026-10-21 10:03 UTC - Jose Celano - Approved the plan.
- 2026-10-21 10:09 UTC - GitHub Copilot - R2-R4 done: 3 tests, mutations D1-D3 killed, the file at 32 / 32
  lines. The AAA review and the results are recorded above.
- 2026-10-21 10:23 UTC - Jose Celano - Approved the completed file. The plan is `done`.
