---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/stats/responses.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/stats/responses.rs
    - packages/axum-rest-api-server/src/v1/context/stats/handlers.rs
    - packages/axum-rest-api-server/src/test_support.rs
    - packages/rest-api-protocol/src/v1/context/stats/resources/stats.rs
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-stats-handlers-tests.md
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-responses-tests.md
---

# Stats Responses File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/stats/responses.rs`, ledger row 16 (rank 5), the fifth file of S4. It is
the largest gap of the slice: 6 / 45 lines at the S4 baseline.

## Current State

- **Unit-only coverage:** 6 / 45 lines at the S4 baseline. After row 15 the file reads 45 / 45 lines, 116 / 116 regions
  and 4 / 4 functions (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace, `--lib`), measured on this branch,
  but that is **covered, not asserted**: the Prometheus case of `get_stats_handler` runs all 32 metric lines of
  `metrics_response` and asserts only the first two (`torrents 1` and `seeders 2`).
- **Existing tests:** none in the file. The four integration tests of `tests/server/v1/contract/context/stats.rs` assert
  the JSON statistics and the labeled Prometheus metrics by name, never the legacy text of `/stats?format=prometheus`.
  Nothing asserts the exact text of `metrics_response` anywhere.
- **Module-owned decisions:** the four answers of the stats context:
  - `stats_response`: `200` with the `Stats` as JSON (the field names and their order are `rest-api-protocol`'s).
  - `labeled_stats_response`: `200` with the `LabeledStats` as JSON (`{"metrics":[...]}`).
  - `labeled_metrics_response`: `200` with the Prometheus exposition text of the metric collection, as plain text.
  - `metrics_response`: `200` with the legacy line text, one `name value` line per statistic, joined with `\n` (no
    trailing newline), in a fixed order. This text is built here by hand (32 lines), so each name, value and position
    is the package's own decision and a swapped value or a typo would go unnoticed today.
  - Owned elsewhere: the JSON shape of `Stats` and `LabeledStats` (`rest-api-protocol`) and the Prometheus text of a
    collection (`torrust-metrics`).
- **T1 hypothesis:** test candidate.

## Maintainer Decisions

On 2026-11-04 the maintainer asked for the boolean line to be investigated by a separate agent (reproduce the output,
read the history, test it against Prometheus parsers, decide whether it is a bug and open an issue either way) and told
this issue to continue as if the text were normal, with a short `TODO` comment saying the behavior is pending
re-evaluation. This plan therefore pins the current text, `true` included, and does not treat it as a finding. The
direction was given in the conversation and is taken as the approval of this plan.

## Current Tests Review

No tests in the file, so R1 is skipped.

## Coverage And Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| `metrics_response`: every line, in order, joined with `\n` | 3A unit | R2: one test with the fixed statistics `a_stats()` (all fields different) and the whole expected body as a literal of 32 lines: status `200`, `text/plain; charset=utf-8`. A swapped value, a typo in a name, a dropped line or a changed separator fails it. |
| `stats_response` | 3A unit | R2: `200`, `application/json`, the literal JSON object of `a_stats()` (32 fields). |
| `labeled_stats_response` | 3A unit | R2: `200`, `application/json`, the literal JSON of `a_labeled_stats()`. |
| `labeled_metrics_response` | 3A unit | R2: `200`, `text/plain; charset=utf-8`, the literal Prometheus text of `a_labeled_stats()`. |
| `#[allow(deprecated)]` on `metrics_response` | Not tested | It has no effect today (the function compiles without it and uses no deprecated item), and it has no reason text. It is production code outside this issue's test scope; recorded in `lessons.md` as a follow-up. |
| The JSON shape and the Prometheus format themselves | Collaborator-owned | `rest-api-protocol` and `torrust-metrics`. The literals pin one counter, so a release of `torrust-metrics` that changes the text needs this test updated on purpose. |

### Observation for the maintainer: `completed_persisted_enabled true`

`metrics_response` prints the boolean `completed_persisted_enabled` with `format!("... {}", bool)`, so the line reads
`completed_persisted_enabled true` (or `false`). A Prometheus scraper needs a number, so a scraper reading
`/stats?format=prometheus` would reject that line. The legacy text is not real Prometheus (no `# TYPE` lines, and it is a
different endpoint from the labeled `/metrics?format=prometheus`), so this may be intentional or an oversight. The plan pins the text as it is (see the maintainer decisions above); a separate agent investigates whether it is a
bug.

## Test Design

- **Fixtures:** the approved `a_stats()` and `a_labeled_stats()` of `test_support.rs` (row 15).
- **Expected values:** literals built with `concat!` so that each line of the legacy text is one visible line:
  `torrents 1`, `seeders 2`, `completed 3`, `completed_in_session 4`, `completed_persisted 5`,
  `completed_persisted_enabled true`, `leechers 7`, then the 25 protocol and UDP lines with the values 8 to 32.
- **Helpers:** the shared `observed`; the responses are built by calling the four functions directly.
- **Lint allows:** none expected.

## Property-Test Decision

Not flagged and not selected: the functions format a fixed shape, and a property over random statistics would restate
the implementation. The all-different fixture already catches any swapped field.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests in the file. | None. |
| R2 | DONE | Add the in-file `#[cfg(test)]` module with the four tests. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | DONE | Integration increment: `none`. | The integration tests keep protecting the JSON and labeled Prometheus contracts; the legacy text is pinned at unit level only. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change a test must catch; revert it exactly after the run. A mutant that leaves an import
or variable unused does not compile under `unused = "deny"`, so such a mutant is redone so that it compiles.

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| P1 | `stats_response` | Answer the legacy text instead of JSON. | the JSON test |
| P2 | `labeled_stats_response` | Answer the collection without the `metrics` wrapper (`Json(&stats.metrics)`). | the labeled JSON test |
| P3 | `labeled_metrics_response` | Answer JSON instead of the Prometheus text. | the labeled Prometheus test |
| P4 | `metrics_response` | Swap the values of `seeders` and `completed`. | the legacy text test (and the Prometheus case of the stats handler test) |
| P5 | `metrics_response` | A typo in the name `tcp4_connections_handled`. | the legacy text test |
| P6 | `metrics_response` | Drop the last line, `udp6_errors_handled`. | the legacy text test |
| P7 | `metrics_response` | Join with `\r\n`. | the legacy text test |
| P8 | `metrics_response` | Swap the values of `udp4_requests` and `udp6_requests`. | the legacy text test |
| P9 | `metrics_response` | Print the negation of `completed_persisted_enabled`. | the legacy text test |
| P10 | `metrics_response` | `udp_avg_connect_processing_time_ns` prints the announce value. | the legacy text test |

## Prose-First AAA Review

Temporary prose, written before comparing with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_answer_the_statistics_as_a_json_object` | The shared statistics fixture, in which every field is its position. | `stats_response`. | The whole response is `200`, JSON, and the literal object of 32 fields in declaration order. | Kept. |
| `it_should_answer_the_statistics_as_one_name_and_value_line_per_statistic` | The same fixture. | `metrics_response`. | The whole response is `200`, plain text, and the literal 32 lines joined with `\n`, including `completed_persisted_enabled true` and no trailing newline. | The doc comment carries the `TODO` about the boolean line and points to `lessons.md`. Kept. |
| `it_should_answer_the_labeled_statistics_as_a_json_object_with_the_metric_collection` | The shared labeled fixture (one counter, value 1, recorded at second 1). | `labeled_stats_response`. | The literal `{"metrics":[...]}` object. | Kept. |
| `it_should_answer_the_labeled_statistics_as_prometheus_text` | The same labeled fixture. | `labeled_metrics_response`. | `200`, plain text, and the literal text `\n# TYPE requests_total counter\nrequests_total 1` (it starts with a line feed). | Kept. |

Helpers: none of its own. The shared `observed` and `Observed::new`, and the fixtures `a_stats` and `a_labeled_stats`,
come from `test_support`.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): `stats/responses.rs` 6 / 45
  lines at the S4 baseline and 45 / 45 after row 15 (covered, not asserted); 155 / 155 lines (100.00%), 168 / 168
  regions and 12 / 12 functions after (the counts include the test module). Package unit-only lines after this file:
  2,067 / 2,175 (95.03%), regions 2,984 / 3,168 (94.19%), functions 333 / 361 (92.24%), 156 unit tests.
- **What changed besides the figure:** the 45 production lines were run but not asserted after row 15 (only two of the 32
  metric lines were checked). Now every line, name, value, position and the separator of the legacy text is asserted
  with one literal, so each of the mutations below fails.
- **Tests:** 4 test functions, 4 test cases; the package went from 152 to 156 unit tests. No integration test was added or
  changed (the integration tests still do not assert the legacy text).
- **Mutations** (each applied alone, stable toolchain, then reverted; the file was identical to the tested one
  afterwards): P1 killed by the JSON test, P2 by the labeled JSON test, P3 by the labeled Prometheus test, P4 to P10 by the
  legacy text test (swapped values, a typo in a name, a dropped last line, the `\r\n` separator, swapped UDP values, the
  negated boolean, the wrong average). A first P2 attempt was a no-op comment and its first P3 did not compile (an unused
  import under `unused = "deny"`); both were redone as compiling mutants, and P7 was re-applied after the first anchor
  broke on the escaped newline.
- **Dependency:** none new. **Clippy allows:** none.
- **Validation:** `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `cargo clippy -p
  torrust-tracker-axum-rest-api-server --all-targets` (stable), `cargo machete --with-metadata`, `git diff --check` and
  `cargo test -p torrust-tracker-axum-rest-api-server` (156 unit and 58 integration tests) pass.
- **Review outcome:** the maintainer directed this plan on 2026-11-04 (pin the text, investigate the boolean line apart) and approved the completed file on 2026-11-04.
