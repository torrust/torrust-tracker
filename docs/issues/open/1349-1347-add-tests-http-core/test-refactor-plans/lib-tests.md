---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/lib.rs
status: done
semantic-links:
  related-artifacts:
    - packages/http-core/src/lib.rs
    - packages/http-core/src/statistics/event/listener.rs
    - packages/axum-http-server/src/lib.rs
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Crate Root File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/http-core/src/lib.rs`.

## Current State

- **Unit-only coverage:** 41 / 41 lines (100.00%), counted in the shared `crate::tests` helper module, which every
  service, event and statistics test calls. The production part (the module declarations, the `CurrentClock` alias
  and `HTTP_TRACKER_LOG_TARGET`) has no executable line.
- **Existing tests:** none in the file. The helpers are used by more than 90 call sites in other files.
- **Module-owned decisions:** the value of the public constant `HTTP_TRACKER_LOG_TARGET` (`"HTTP TRACKER"`), which
  the listener logs under and which `src/console/ci/e2e/logs_parser.rs` matches through the identical constant of
  `axum-http-server`; and the behavior the shared `RecordingSender` documents: it records an event only when the
  future returned by `send` is polled.
- **T1 hypothesis:** test support only.

## Current Tests Review

Nothing to review in the file. The service tests of S2-S4 rely on two `RecordingSender` properties that have no test
of their own: a dropped `send` future records nothing, and the recorded events keep their order. A change to the
helper would silently weaken every "publishes nothing" assertion.

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| `HTTP_TRACKER_LOG_TARGET` value | 3A unit | R2a: equals the literal `HTTP TRACKER`. The same value is declared again in `axum-http-server` (which cannot be imported here without breaking the layering); the test pins this copy and the duplication goes to `lessons.md`. |
| `RecordingSender` records on poll | 3A unit | R2b: a new sender has no events; awaiting `send` records the event and returns `Some(Ok(1))`; events keep their order. |
| `RecordingSender` ignores a dropped future | 3A unit | R2c: creating the `send` future and dropping it records nothing. |
| `sample_*` fixtures | 3A unit | R2d: `sample_peer_using_ipv4` is the sample peer with an IPv4 address, `sample_peer_using_ipv6` differs from it only by an IPv6 address, and `sample_info_hash` is the fixed hash. The service tests assume these families. |
| `CurrentClock` alias | not tested | The alias selects `Stopped` in tests and `Working` otherwise; `cfg(test)` makes the production variant unreachable from a unit test. The listener and handler tests already depend on the stopped clock. |
| Module declarations | not tested | The compiler checks them. |

No property candidate.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | N/A | No existing test. | Recorded in the plan. |
| R2 | DONE | Add R2a-R2d in a sibling test module `root_tests` (a module nested in `crate::tests` would have repeated the name). | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review; the named mutations D1-D4 each make a test fail. |
| R3 | DONE | Select an integration increment, or `none`. | `none`. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results and `coverage-evidence.md`. |

## Named Mutations

| ID | Test | Site | Temporary change |
| --- | --- | --- | --- |
| D1 | R2a | `HTTP_TRACKER_LOG_TARGET` | Change the value (for example `"HTTP_TRACKER"`). |
| D2 | R2c | `RecordingSender::send` | Record the event before returning the future. |
| D3 | R2b | `RecordingSender::send` | Return `Some(Ok(0))`; separately, insert each event at the front. |
| D4 | R2d | `sample_peer_using_ipv6`, `sample_info_hash` | Change the last group of the IPv6 address; change the last digit of the hash. |

Gates: this plan needs approval first; the completed file needs the maintainer's completed-file review before its
one signed commit.

## Prose-First AAA Review

Eight tests in a new `root_tests` module of `lib.rs`; the helpers are unchanged.

| Test | Behavior it states | Cases | Expected value is |
| --- | --- | --- | --- |
| `it_should_log_http_tracker_events_under_the_http_tracker_target` | The public log target value. | 1 | The literal `HTTP TRACKER`. |
| `it_should_record_no_event_before_any_event_is_sent` | A new sender is empty. | 1 | An empty list. |
| `it_should_record_an_event_when_the_send_future_is_polled` | Awaiting `send` records the event and reports one receiver. | 1 | `Some(1)` and the one event. |
| `it_should_record_events_in_the_order_they_are_sent` | Recorded events keep their send order. | 1 | The two events, oldest first (told apart by public URL). |
| `it_should_record_nothing_when_the_send_future_is_dropped_without_being_polled` | A dropped future publishes nothing, and a later polled one is still recorded. | 1 | Only the polled event. |
| `it_should_sample_a_fixed_info_hash` | The fixture hash. | 1 | The literal hash. |
| `it_should_sample_an_ipv4_peer_that_is_the_default_sample_peer` | The IPv4 fixture is the default sample at `126.0.0.1:8080`. | 1 | The default sample and the literal address. |
| `it_should_sample_an_ipv6_peer_that_differs_from_the_ipv4_sample_only_by_its_address` | The IPv6 fixture differs only by its address. | 1 | The IPv4 sample with the literal IPv6 address. |

No test repeats the helper's own logic: events, addresses and the hash are literals or independently built values.

## Results

Unit-only coverage, measured on 2026-10-21 at 13:57 UTC.

| Scope | Lines before | Lines after | Regions before | Regions after | Functions before | Functions after |
| --- | --- | --- | --- | --- | --- | --- |
| `lib.rs` | 41 / 41 (100.00%) | 97 / 97 (100.00%) | 42 / 42 | 150 / 150 | 8 / 8 | 20 / 20 |

The file was already fully covered, by tests that never asserted the helpers' own contract; the counted lines now
include the new test module. Production code is byte-identical to `HEAD` (the diff of `lib.rs` has no removed line).
The package has 191 unit tests.

Named mutations (each reverted; the diff was checked to contain no removed line afterwards):

| ID | Result | Tests that failed |
| --- | --- | --- |
| D1 | killed | 1, new: no other test reads the value |
| D2 | killed | 1, new (`..._dropped_without_being_polled`), for a sender that records at call time. No service test noticed it: they never drop a `send` future. A first attempt that both recorded eagerly and again on poll failed 27 tests, because the doubled events broke their assertions; it proved nothing about the dropped-future contract, so the mutant was redone to record only eagerly. |
| D3 | killed | 1 each, new: the receiver count `Some(Ok(0))`, and the reversed order |
| D4 | killed | 1 each, new: the IPv6 address and the hash |

Two slips are recorded: a one-line edit that included the next line's text in its old text deleted that line and
broke the helper (repaired from the text just shown; the diff then showed no removed line), and the first version
of the poll test used `matches!`, whose failure arm counted as one uncovered region of the test module; it now uses
a whole-value comparison and the file is 150 / 150 regions.

## Approval

Approved by Jose Celano on 2026-10-21 (between 13:23 and 13:50 UTC; the exact time was not noted): R2a-R2d, R3 `none` and mutations D1-D3 (D4 was added during the work).

## Progress Log

- 2026-10-21 13:23 UTC - GitHub Copilot - Created the plan from `lib.rs` and the S6 baseline. No Rust test or
  production code changed. The plan awaits maintainer approval.
- 2026-10-21 between 13:23 and 13:50 UTC - Jose Celano - Approved the three S6 plans, including the documentation correction D1 of
  `services/mod.rs` (the exact time was not noted).
- 2026-10-21 13:59 UTC - GitHub Copilot - R2-R4 done: 8 tests, mutations D1-D4 killed, the file at 97 / 97 lines.
  The AAA review and the results are recorded above.
- 2026-10-21 14:27 UTC - Jose Celano - Approved the completed file. The plan is `done`.
