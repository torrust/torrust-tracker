---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/statistics/event/listener.rs
status: done
semantic-links:
  related-artifacts:
    - packages/http-core/src/statistics/event/listener.rs
    - packages/http-core/src/statistics/event/handler.rs
    - packages/events/src/drain.rs
    - docs/issues/closed/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Statistics Listener File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/http-core/src/statistics/event/listener.rs`. It reuses the approach of the earlier plans and of
[statistics-handler-tests.md](statistics-handler-tests.md).

## Current State

- **Unit-only coverage:** 215 / 239 lines (89.96%), 275 / 298 regions, 29 / 32 functions, measured at the S5
  baseline. The file changed after the inventory: #2414 added the cancellation drain and its tests, so the
  inventory figure (133 / 148) is stale. The totals include the test module. The uncovered lines are 23 and
  30-36 (the spawned `run_event_listener` wrapper), 43 (the log line of `run_event_listener_unspawned`), 132 (the
  `warn!` for an ignored event), 140 (the `TcpAnnounce` arm of `event_connection_id`) and 192-193 (the `Closed`
  and `Lagged` arms of the test receiver's `try_recv`).
- **Existing tests:** 6, all through `dispatch_events` with a scripted receiver: a closed receiver stops the
  loop; a lag is survived; a queued event is handled and the listener returns `Cancelled` when cancelled; an
  expired drain timeout leaves the event unhandled and still returns `Cancelled`; the metrics policy applies to a
  drained event; disabled and unknown instances are ignored before an enabled one is counted.
- **Module-owned decisions:** the listener's loop (cancellation first, then receive; stop when the receiver
  closes; survive a lag with a warning); the per-instance metrics policy, enforced here and not where events are
  produced (issue #2039): an event is counted only when its own `ConfigurationInstanceId` maps to `true`, and an
  unknown instance is ignored; applying the same policy to events drained after cancellation; the `Completion`
  returned for a closed receiver (`Completed`) and for a cancellation (`Cancelled`); the two public entry points
  (`run_event_listener`, which spawns, and `run_event_listener_unspawned`). The drain algorithm itself
  (admission timeout, `try_recv` loop, yielding) is `torrust-tracker-events`'s and tested there; counting an event
  is the handler's ([statistics-handler-tests.md](statistics-handler-tests.md)).
- **T1 hypothesis:** lifecycle (#1488), now partly stale.

## Current Tests Review

Reviewed against `write-unit-test`. Conclusion: **clean; no refactor.**

| Test | Findings |
| --- | --- |
| the six tests | Names start with `it_should_`; Arrange, Act and Assert are marked; the scripted receiver is a small in-memory double, lazy like a real receiver (the comment says why); expectations are literals (`1`, `0`, `Completion::Cancelled`) with failure messages; no sleeping, no mocking framework, no hidden shared state. The `ScriptedReceiver` is the test's one helper and is justified by naming the scenario it scripts. |
| Gaps, not smells | Every event is a scrape from an IPv4 server, so the policy lookup of an announce event (line 140) is unprotected: a listener that routes announces by a wrong instance id passes all six. Only one queued event is ever drained, so a drain that handles only the first passes. The two public entry points have no test in the package. |

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| The metrics policy applies to announce events, keyed by the announce event's own instance id | 3A unit | R2a: a table of announce and scrape events from an enabled and from a disabled instance, asserting all four public counters, with a second instance in the policy so a wrong id fails. Covers line 140. |
| Every queued event is handled after cancellation, each under the policy | 3A unit | R2b: three queued events (enabled, disabled, enabled) drained after cancellation give 2 and `Cancelled`. The drain's own semantics stay `torrust-tracker-events`'s; this proves the listener hands every event to the policy and the handler. |
| `run_event_listener_unspawned` returns the documented `Completion` | 3A unit | R2c (D1): `Completed` when the receiver closes, `Cancelled` when the token is cancelled, each after counting a queued enabled event. |
| `run_event_listener` spawns the listener and its task finishes when the receiver closes | 3A unit | R2d (D1): the returned `JoinHandle` completes and the queued enabled event is counted. Lines 23 and 30-36. |
| The `warn!` for an ignored event (line 132) and the `info!` of the unspawned entry (line 43) | not tested | Logging; the branch of line 132 is tested, only its log argument is not evaluated without a subscriber. |
| `ScriptedReceiver::try_recv` `Closed` and `Lagged` arms (lines 192-193) | not tested | Test support for a drain that meets a closed or lagging receiver; that is the drain's behavior (`torrust-tracker-events`), not the listener's. |
| Supervision of the task: aborting after the shared shutdown deadline, restarting, the token hierarchy | not tested | Lifecycle owned by #1488 (open: "Overhaul: Tracker Shutdown"). |

No property candidate: the policy is a small table and the loop is scripted.

## Test Bootstrap

No change to the existing `ScriptedReceiver` and `scrape_event`. Add an `announce_event(instance_id)` next to
`scrape_event`, built the same way with the IPv4 sample peer, and, for the table, a small helper that dispatches
one scripted receiver to a repository and returns the four counters. The entry-point tests (R2c, R2d) use the same
scripted receiver and a real `Repository`; they need no runtime beyond `#[tokio::test]`.

## Decisions needed

- **D1 (public entry points):** test `run_event_listener_unspawned` and `run_event_listener` for their return
  values and effect (R2c, R2d; recommended: the application, the Axum server tests and the benchmark all call them,
  and the tests assert only the documented `Completion` and the count, not the shutdown policy that #1488 will
  change), or leave both to #1488 and record the uncovered lines (23, 30-36, 43) as lifecycle-owned.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | N/A | The six existing tests are clean; nothing to refactor. | Recorded in the plan. |
| R2 | DONE | Add R2a-R2d (R2c and R2d only if D1 is approved). | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review; the named mutations K1-K5 each make a test fail. All five did, plus one probe. Pending the maintainer's completed-file review. |
| R3 | DONE | Select an integration increment, or `none`. | The entry-point tests (R2c, R2d) are the increment; they drive the public functions with a scripted receiver and a real repository. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change that its test must catch; it is reverted after the run and recorded in
Results. The workspace denies `unused` and `warnings`: use a compiling form, and count a mutation only when its
test runs and fails.

| ID | Test | Site | Temporary change |
| --- | --- | --- | --- |
| K1 | R2a | `event_connection_id`, the `TcpAnnounce` arm | Return a fixed instance id instead of the event's. |
| K2 | R2b | `drain_events` | Hand only the first queued event to the handler (stop the drain after one). |
| K3 | R2c | `run_event_listener_unspawned` | Return `Completion::Completed` whatever the inner result. |
| K4 | R2d | `run_event_listener` | Spawn a task that does not run the listener. |
| K5 | R2a | `handle_received_event` | Treat an instance missing from the policy as enabled (`unwrap_or(true)`); a survivor check, since the existing policy test should already kill it. |

Gates: this plan needs approval first (including D1); the completed file needs the maintainer's
completed-file review before its one signed commit.

## Prose-First AAA Review

The six existing tests are unchanged. Eight cases in four new test functions were added; they use the same
scripted receiver. Two small helpers were added next to `scrape_event`: `connection` (extracted from it, so
`scrape_event` and the new `announce_event` share one construction), and `counters`, which returns the four
public counters as a tuple. `enabled_id` and `disabled_id` name the two instances of the policy.

| Test | Behavior it states | Cases | Expected value is |
| --- | --- | --- | --- |
| `it_should_apply_the_metrics_policy_of_the_instance_of_each_event_kind` | The policy lookup uses the instance of the event itself, for announce and scrape events. | announce and scrape, each from an enabled and from a disabled instance | The tuple of the four counters as literals: counted once, or not at all. |
| `it_should_handle_every_queued_event_after_cancellation_under_the_metrics_policy` | Cancellation does not drop queued events, and each is under the policy. | 1 (enabled, disabled, enabled) | Counters `(0, 2, 0, 0)` and `Completion::Cancelled`. |
| `it_should_return_the_completion_that_says_why_the_unspawned_listener_stopped` | `run_event_listener_unspawned` returns `Completed` when the receiver closes and `Cancelled` when the token is cancelled, and counts the queued event either way. | 2 | A literal `Completion` and the counters `(0, 1, 0, 0)`. |
| `it_should_run_the_spawned_listener_until_its_receiver_closes` | `run_event_listener` spawns a task that runs the listener and finishes when the receiver closes. | 1 | The task joins without a panic; counters `(0, 1, 0, 0)`. |

No test sleeps, mocks or reads the clock. The tests of the entry points assert only the documented `Completion`
and the count, not the shutdown policy that #1488 will change. The cancelled case of the unspawned entry point
also drives the drain to meet the receiver's `Closed` result, which covers that arm of the test receiver.

## Results

Unit-only coverage (`cargo llvm-cov -p torrust-tracker-http-core --all-features --lib`), measured on 2026-10-21 at
09:49 UTC.

| Scope | Lines before | Lines after | Regions before | Regions after | Functions before | Functions after |
| --- | --- | --- | --- | --- | --- | --- |
| `statistics/event/listener.rs` | 215 / 239 (89.96%) | 308 / 310 (99.35%) | 275 / 298 | 390 / 394 | 29 / 32 | 44 / 44 |
| Package `http-core` | 2,521 / 2,577 (97.83%) | 2,614 / 2,648 (98.72%) | 3,224 / 3,284 (98.17%) | 3,339 / 3,380 (98.79%) | 314 / 320 (98.12%) | 329 / 332 (99.10%) |

The package "before" column is the state after `handler.rs`; the S5 baseline of the file is in "Current State".
Every production line is covered except line 132, the log argument of the ignored-event `warn!` (its branch is
tested), and the `Lagged` arm of the test receiver's `try_recv` (line 195), which is the drain's behavior. The
package has 157 unit tests (149 after `handler.rs`, 144 at the S5 baseline), and the suite passes three more runs
and a run with `--test-threads=1`. Production code before the first `#[cfg(test)]` is byte-identical to `HEAD`.

Named mutations (each reverted; the production diff was checked empty afterwards):

| ID | Result | Tests that failed |
| --- | --- | --- |
| K1 | killed | the disabled-announce case |
| K2 | killed | the drain-every-event test only (the six existing single-event drain tests pass this mutant) |
| K3 | killed | the cancelled case of the unspawned test |
| K4 | killed | the spawned-listener test |
| K5 | killed | the existing ignore-disabled-and-unknown test, as the plan predicted |

Survivor probe beyond the plan: a lag that stops the listener was killed by the existing lag test. All five named
mutations compiled in their first form.

### Independent Review Round

The independent Task Reviewer found three points in this file:

- **F2 (wiring):** `run_event_listener` could receive a fresh token or a zero timeout in place of its arguments
  and all tests passed. A table with two cases now gives the spawned listener one queued event and a token that
  is already cancelled: with the default timeout the drain counts the event; with `Duration::ZERO` it counts
  nothing. A token replaced by a live one fails the second case; a timeout replaced by zero fails the first.
- **Time (the shared F1):** the handler's `now` is read from the clock by `handle_received_event`; a mutant that
  adds 7 seconds survived. A new test sets the stopped clock to 7 seconds, runs one scrape and compares the
  repository's whole `Metrics` with one built from literals.
- **INFO, logic in a test:** the `if cancelled { token.cancel() }` of the unspawned test became two small helpers
  (`live_token`, `cancelled_token`) passed as case parameters.

A known limit, recorded and not fixed: a mutant that makes the `Closed` arm return `false` makes the loop spin
forever, because the scripted receiver returns `Closed` indefinitely and never yields, so no timeout can fire.
The existing closed-receiver test has the same property. It only matters when a mutation breaks the loop.

After the round the file has 17 tests, 354 / 356 lines, 440 / 444 regions and 48 / 48 functions, with the same two
uncovered lines.

## Approval

Approved by Jose Celano on 2026-10-21 (09:46 UTC): R1 N/A, R2a-R2d, R3 and mutations K1-K5, with D1 (test the two public entry points).

## Progress Log

- 2026-10-21 09:45 UTC - GitHub Copilot - Created the plan from `listener.rs`, the S5 baseline and the S2-S5
  plans. No Rust test or production code changed. The plan awaits maintainer approval, with one decision: D1.
- 2026-10-21 09:46 UTC - Jose Celano - Approved the plan with D1: test the two public entry points.
- 2026-10-21 09:49 UTC - GitHub Copilot - R2-R4 done: 8 cases in 4 new test functions, mutations K1-K5 and one
  probe killed, the file at 308 / 310 lines. The AAA review and the results are recorded above.
- 2026-10-21 09:59 UTC - Jose Celano - Approved the completed file. The plan is `done`.
