---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/statistics/event/handler.rs
status: done
semantic-links:
  related-artifacts:
    - packages/http-core/src/statistics/event/handler.rs
    - packages/http-core/src/event.rs
    - packages/http-core/src/statistics/metrics.rs
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Statistics Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/http-core/src/statistics/event/handler.rs`. It reuses the approach of the
[announce](services-announce-tests.md), [scrape](services-scrape-tests.md), [event](event-tests.md) and
[container](container-tests.md) plans.

## Current State

- **Unit-only coverage:** 158 / 161 lines (98.14%), 215 / 221 regions, 16 / 17 functions, measured at the S5
  baseline (`develop` at the merge of #2593; the file has not changed since S1). The totals include the test
  module. The uncovered lines are 20 (the `Err` arm that logs a failed counter increase) and 25 (the closure
  that formats the debug line).
- **Existing tests:** 5. Four tests, one per request kind and IP family, each build one event inline (about
  15 lines) and assert one counter. A fifth test checks the label set of one scrape event.
- **Module-owned decisions:** how a published HTTP event becomes a counter increase: the event kind chooses the
  `request_kind` label (`announce` or `scrape`); the connection context supplies the other labels (the
  contract pinned in [event-tests.md](event-tests.md)); exactly one sample of
  `http_tracker_core_requests_received_total` is increased per event, and repeated events with the same labels
  accumulate. The counter, its storage and the label types are `torrust-metrics`'s and the repository's; the
  per-listener metrics policy is the listener's.
- **T1 hypothesis:** probably no change; to be confirmed against the review below.

## Current Tests Review

Reviewed against `write-unit-test`. Conclusion: **refactor**.

| Test | Smells found |
| --- | --- |
| the four `should_increase_the_tcp*_counter_*` tests | Names do not start with `it_should_`; no `// Arrange`, `// Act`, `// Assert`; about 15 lines of event construction repeated four times (two copies hold the 8-group IPv6 literal); each asserts one of the four counters, so an increment that also touches the wrong request kind or family passes; the `peer` is built only to be passed in. |
| `it_should_propagate_all_connection_labels_and_request_kind` | The best test of the file (literal expected labels), but: it covers a scrape event only, so the `announce` value of `request_kind` is never asserted and an announce event with the wrong kind passes the four counter tests only by accident; it compares with `sum`, which matches a label subset, so an extra label on the sample would still pass; the assertion is a floating-point comparison `(value - 1.0).abs() < f64::EPSILON` of a counter that is an integer; the expected set is built from the production `label_name!`. |
| No test | Accumulation (two events add up), separate series (two label sets stay separate), and an announce event's full label set. |

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| One event increases exactly the counter of its request kind and server family, and no other | 3A unit | R1: one table of the four events. Each case asserts all four public counters (`tcp4_announces_handled`, `tcp4_scrapes_handled`, `tcp6_announces_handled`, `tcp6_scrapes_handled`) as a tuple, so a cross-counting increment fails. Client and server families match in every case: the family counters follow the server binding, and a client of the other family is left to the `metrics.rs` plan (see "Decisions needed", D1). |
| The sample has exactly the connection labels plus `request_kind` (announce and scrape, IPv4 and IPv6, public URL present and absent) | 3A unit | R2a: a table of whole expected label sets as string literals, read with `MetricCollection::get_counter_value`, which looks the sample up by its exact label set. It replaces the float `sum` comparison and covers the `announce` value. |
| Repeated events accumulate; events with different labels stay separate | 3A unit | R2b: two identical events give 2 on one series; two events that differ in one label (the public URL) give two series of 1 and a family counter of 2. |
| `now` stored with the sample | not tested | The recorded timestamp is not readable through the public API of the package (`Metrics` exposes counter totals and the collection getter, which returns the counter value only), so a test cannot see it. It is `torrust-metrics`'s behavior. |
| The failed-increase `Err` arm (line 20) | not tested | `increase_counter` fails only if a metric of another type has the counter's name. The repository always describes it as a counter and exposes no way to add another metric, so the arm cannot be reached through the package. Owner: `torrust-metrics`. |
| The debug log closure (line 25) | not tested | Logging, evaluated only when a `debug` subscriber is installed; asserting log output needs a subscriber crate this package does not use. |

No property candidate: the handler has two event kinds and no numeric input space.

## Test Bootstrap

Two in-file helpers replace the repeated construction: `connection(client_ip, bind_ip)`, which builds a
`ConnectionContext` with a fixed instance id, client port and HTTP binding, and `announce_event` / `scrape_event`
built on it. `Repository::new()` is the real in-memory repository (no I/O). The clock value is a fixed
`DurationSinceUnixEpoch`, so no stopped clock is needed. The existing `crate::tests` samples
(`sample_info_hash`, `sample_peer_using_ipv4`, `sample_peer_using_ipv6`) supply the announce payload.

## Decisions needed

- **D1 (family semantics, for the `metrics.rs` plan):** the doc comments of `tcp4_announces_handled` and its
  three siblings say "requests from IPv4 peers", but the code selects on `server_binding_address_ip_family`, the
  family of the server's bind address. An IPv6 client of an IPv4-bound tracker is therefore counted as
  `tcp4`. That is probably the intended meaning (the counter is per listener family) and only the comment is
  wrong, but it is a decision. This plan does not pin the cross-family case; the `metrics.rs` plan will ask.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Refactor the 5 tests: in-file builders, AAA markers, `it_should_*` names, the four counter tests as one table asserting all four counters, and the label test replaced by the R2a table. | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review. Maintainer review of the result. Done together with R2 (disclosed); pending the maintainer's review. |
| R2 | DONE | Add R2a (exact label sets) and R2b (accumulation and separate series). | Same validation; the named mutations H1-H4 each make a test fail. All four did, plus one probe. |
| R3 | DONE | Select an integration increment, or `none`. | `none`: the listener tests already drive the handler end to end. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change that its test must catch; it is reverted after the run and recorded in
Results. The workspace denies `unused` and `warnings`: use a compiling form, and count a mutation only when its
test runs and fails.

| ID | Test | Site | Temporary change |
| --- | --- | --- | --- |
| H1 | R1, R2a | `labels_and_request_kind`, the `TcpAnnounce` arm | Return `RequestKind::Scrape`. |
| H2 | R2a | `handle_event` | Do not upsert the `request_kind` label. |
| H3 | R2a | `labels_and_request_kind`, the `TcpScrape` arm | Use an empty `LabelSet` instead of the connection's labels. |
| H4 | R2b | `handle_event` | Increase the counter twice per event. |

Gates: this plan needs approval first; R1 needs the maintainer's review of its result (or the disclosed combined
review accepted in S2-S4); the completed file needs the maintainer's completed-file review before its one signed
commit.

## Prose-First AAA Review

The 5 old tests became 10 cases in 4 test functions (two tables of 4 cases and two single tests). Each reads as: an in-file builder for the connection (fixed instance id and
client port), one event, one call to `handle_event` with a fixed time, one whole-value assertion. The realistic
announce payloads use the existing IPv4 and IPv6 sample peers, so those `crate::tests` helpers have a user again.

| Test | Behavior it states | Cases | Expected value is |
| --- | --- | --- | --- |
| `it_should_increase_only_the_counter_of_the_request_kind_and_family_of_the_event` | An event increases the counter of its request kind and server family and no other. | IPv4 announce, IPv4 scrape, IPv6 announce, IPv6 scrape | The tuple of the four public counters as literals, so cross-counting fails. |
| `it_should_count_the_event_in_the_sample_with_exactly_the_connection_labels_and_the_request_kind` | The sample has exactly the connection labels plus `request_kind`: announce and scrape, both families, and an IPv4-mapped client of an IPv6 server on another port with a public URL. | 4 | A whole literal label set looked up by exact match, `Some(1)`. |
| `it_should_accumulate_events_with_the_same_labels_in_one_sample` | Two events with the same labels (two clients of one server) share one sample. | 1 | `Some(2)` and the counters `(2, 0, 0, 0)`. |
| `it_should_keep_events_with_different_labels_in_separate_samples` | A public URL makes a separate sample. | 1 | `Some(1)` for each sample and the counters `(0, 2, 0, 0)`. |

No test uses the production label names: every expected label set is written as strings. None mocks, sleeps or
compares floats. The announce case that pins the label set also shows the info hash and the peer are not
labels (a cardinality decision).

## Results

Unit-only coverage (`cargo llvm-cov -p torrust-tracker-http-core --all-features --lib`), measured on 2026-10-21 at
09:40 UTC.

| Scope | Lines before | Lines after | Regions before | Regions after | Functions before | Functions after |
| --- | --- | --- | --- | --- | --- | --- |
| `statistics/event/handler.rs` | 158 / 161 (98.14%) | 143 / 146 (97.95%) | 215 / 221 | 175 / 181 | 16 / 17 | 21 / 22 |
| Package `http-core` | 2,536 / 2,592 (97.84%) | 2,521 / 2,577 (97.83%) | 3,264 / 3,324 (98.19%) | 3,224 / 3,284 (98.17%) | 309 / 315 (98.10%) | 314 / 320 (98.12%) |

The percentages are flat because the file's production lines were already covered and the new test module is
smaller than the old one: the counts fall by 15 lines, all test-module lines (the repeated event construction).
The uncovered lines are the two decided in the plan: the failed-increase `Err` arm (line 20) and the debug closure
(line 25), whose reasons are in "Coverage and Decision Analysis". The package has 149 unit tests (144 before).
Production code before the first `#[cfg(test)]` is byte-identical to `HEAD`.

Named mutations (each reverted; the production diff was checked empty afterwards):

| ID | Result | Tests that failed |
| --- | --- | --- |
| H1 | killed | 5 (the announce cases of the counter and label tables and the accumulation test) |
| H2 | killed | all 10 (the counters and the samples both select on `request_kind`) |
| H3 | killed | 5 (the scrape cases of both tables and the separation test) |
| H4 | killed | all 10 |

Survivor probe beyond the plan: `RequestKind::Announce.as_str()` returning `scrape` was killed by 5 tests. Two
mutation forms did not compile under the workspace lints (a dead variant in H1, a non-binding `let` in H4) and
were redone in a compiling form before they counted.

## Approval

Approved by Jose Celano on 2026-10-21 (09:34 UTC): R1, R2a, R2b, R3 `none` and mutations H1-H4.

## Progress Log

- 2026-10-21 09:31 UTC - GitHub Copilot - Created the plan from `handler.rs`, the S5 baseline and the S2-S4
  plans. No Rust test or production code changed. The plan awaits maintainer approval; D1 is a note for the
  `metrics.rs` plan and needs no answer now.
- 2026-10-21 09:34 UTC - Jose Celano - Approved the plan. R1 starts.
- 2026-10-21 09:40 UTC - GitHub Copilot - R1-R4 done together (disclosed): 10 cases in 4 test functions,
  mutations H1-H4 and one extra probe killed, the file at 143 / 146 lines with only the two decided lines
  uncovered. The AAA review and the results are recorded above.
- 2026-10-21 09:42 UTC - Jose Celano - Approved R1 and the completed file. The plan is `done`.
