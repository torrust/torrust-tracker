---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/statistics/event/listener.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/statistics/event/listener.rs
    - packages/tracker-core/src/statistics/event/handler.rs
    - packages/events/src/bus.rs
    - packages/events/src/drain.rs
    - src/bootstrap/jobs/tracker_core.rs
    - packages/tracker-core/tests/common/test_env.rs
---

# Statistics Event Listener File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/tracker-core/src/statistics/event/listener.rs`, ledger row 4. It is also the listener
half of the design-review checkpoint T2b: the first test that spawns a listener, handles one
event, cancels it and joins it under a deadline.

## Current State

- **Unit-only coverage:** 357 / 406 lines at the S1 baseline, 362 / 411 after rows 1 to 3
  (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, `--all-features --lib`, 2026-11-18). Uncovered:
  19-54 (the spawn wrappers `run_in_memory_event_listener` and
  `run_persistent_completed_statistics_event_listener`), 62-91 (the public `*_unspawned`
  functions), and 284-285 (the `Lagged` arm of the test receiver's `try_recv`, test code).
- **Callers:** the application runs the two `*_unspawned` functions under its own supervision
  (`src/bootstrap/jobs/tracker_core.rs`) and uses the `Completion` they return. The spawn wrappers
  are used only by the integration test environment (`tests/common/test_env.rs`), which discards
  their result.
- **Existing tests:** 11, all on the private `dispatch_*` loops, with a scripted receiver: handling
  events, lag, a closed receiver, a pending cancellation that drains queued events, and an expired
  drain timeout. They already pin every decision of the loops.
- **Module-owned decisions not yet tested:**
  - The `*_unspawned` functions pass the receiver, the token, the repositories and the drain
    timeout through and return the loop's `Completion` (`Cancelled` after a cancellation).
  - The spawn wrappers run the same listener in a task and finish when it finishes.

## Asynchronous Work and Ownership

From the spec's [ownership map](../ISSUE.md#asynchronous-work-cleanup-and-test-fixtures):

- **Channel:** a real `EventBus` of the swarm registry (`torrust-tracker-events`), created by the
  test. The receiver is subscribed before the event is sent, so the event is queued.
- **Listener:** the test owns it. It sends one event, cancels the token, then awaits the future or
  the `JoinHandle` inside `tokio::time::timeout(5 s)`, which fails with a message naming the
  operation. A failed assertion drops the per-test runtime, which drops the task.
- **Determinism:** on the single-threaded test runtime the listener first runs when the test
  awaits it, after the cancellation. The `biased` select takes the cancellation, and the drain
  handles the queued event. No fixed sleep.
- **Database:** the persistent tests use `ephemeral_configuration()`; the test keeps the returned
  value, so the `TempDir` removes the database at the end, including on a failure.

## Maintainer Decision

Plan approved on 2026-11-18, with all four tests (the spawned pair keeps the T2b shape).

## Test Design

| Test | Act | Assert |
| --- | --- | --- |
| `it_should_handle_a_queued_event_and_return_cancelled_when_the_unspawned_in_memory_listener_is_cancelled` | `run_in_memory_event_listener_unspawned`, awaited under the deadline | `Completion::Cancelled`; both in-memory totals read 1 |
| `it_should_persist_a_queued_event_and_return_cancelled_when_the_unspawned_persistent_listener_is_cancelled` | `run_persistent_completed_statistics_event_listener_unspawned` | `Completion::Cancelled`; the database and the persisted total hold 1 |
| `it_should_handle_a_queued_event_then_finish_when_the_spawned_in_memory_listener_is_cancelled` | `run_in_memory_event_listener`, `JoinHandle` awaited under the deadline | the task ends without a panic; both in-memory totals read 1 |
| `it_should_persist_a_queued_event_then_finish_when_the_spawned_persistent_listener_is_cancelled` | `run_persistent_completed_statistics_event_listener` | the task ends without a panic; the database and the persisted total hold 1 |

The tests reuse the module's existing helpers (`peer_download_completed_event`,
`database_downloads_repository`, `expect_in_memory_download_metrics_to_be`,
`expect_persisted_download_metrics_to_be`) and add one: a 5-second `join_within_deadline` helper.

## Property-Test Decision

No `quickcheck`: the functions are wiring with no input domain.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | The in-memory spawn wrapper drops its arguments instead of running the listener | the spawned in-memory test |
| M2 | The persistent spawn wrapper drops its arguments instead of running the listener | the spawned persistent test |
| M3 | `run_in_memory_event_listener_unspawned` returns `Completion::Completed` | the unspawned in-memory test |
| M4 | `run_persistent_completed_statistics_event_listener_unspawned` returns `Completion::Completed` | the unspawned persistent test |
| M5 | The in-memory unspawned function passes `Duration::ZERO` instead of the drain timeout | the unspawned in-memory test (the event stays queued) |
| M6 | The persistent unspawned function passes `Duration::ZERO` | the unspawned persistent test |
| M7 | The in-memory spawn wrapper passes `Duration::ZERO` | the spawned in-memory test |

M5 to M7 rely on the existing finding that an expired drain timeout leaves a queued event
unhandled (`it_should_leave_the_queued_event_unprocessed_and_return_cancelled_when_the_drain_timeout_has_expired`).

## Out of Scope

- Lines 284-285 are the test receiver's `Lagged` arm of `try_recv`: test code, not counted as a
  decision.
- The `Completed` path through a closed real channel: the existing loop tests pin it with the
  scripted receiver.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| L1 | TODO | Add the four tests and the deadline helper. | They pass; lines 19-91 covered. |
| L2 | TODO | Run M1-M7. | Each is killed by its expected test. |
| L3 | TODO | Stop for the T2b fixture-shape review and the completed-file review. | The maintainer's decision recorded here. |

## Results

To be filled in after L3.
