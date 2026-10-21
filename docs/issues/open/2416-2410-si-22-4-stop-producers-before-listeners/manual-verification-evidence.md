---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2416-2410-si-22-4-stop-producers-before-listeners/ISSUE.md
last-updated-utc: "2026-10-21 17:17"
---

# Manual Verification Evidence

## Purpose

Record the verification of #2416 (SI-22.4): the red-to-green proof that the stop order closes loss
window (b), the mutations that show the tests guard each lifecycle clause, the test-environment
checks, and the M1 to M3 rechecks.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-21 from 16:40.
- Artifact under test: branch `2416-stop-producers-before-listeners`, cut from `torrust/develop` at the
  merge `Merge torrust/torrust-tracker#2615: chore(issues): archive closed issue #2518 spec to docs/issues/closed`.
  Each process below names the working-tree state it ran on.
- Operating system / environment: Linux; default toolchain `rustc 1.99.1 (58cf8f9c3 2026-10-13)`,
  `cargo 1.99.1 (5f94df478 2026-08-27)`. `cargo test` built the `test` profile, which this workspace
  configures as optimized with debug info.
- Prerequisites and setup performed: none beyond a workspace build.

## Verification Processes

### V1 - T7: window (b) reproduced with controlled late publication (red)

- Goal: show that an event published after the listener was cancelled is lost, using the real swarm
  event bus and the real in-memory tracker-core listener, and show that a consumer is cancelled while
  producers that stop before it still run. The producers are controlled by the test, so the lost
  effect does not depend on timing.
- What this proves and what it does not: the real-binary `race` scenario of T0 is not the red proof
  for this window. The coordinator's check of 2026-10-21 found that after #2414 drained the listener
  queues the real binary lost nothing in that scenario, so it is not expected to show window (b) as
  red; M3 records this branch's own rerun of the script as a recheck. The proof here is the controlled
  test, which does not depend on timing.
- Working-tree state: `torrust/develop` at the merge named above, plus an uncommitted red stub
  (`StopBefore` and `JobManager::spawn_with`, which accept the edges and do not enforce them) and two
  uncommitted tests. A red test is not committed alone; it is committed with its fix (Commit Points).
- Commands and outcomes (both fail as expected):
  - `cargo test --lib bootstrap::jobs::manager`: 17 passed, 1 failed,
    `it_should_cancel_a_consumer_only_after_every_producer_that_stops_before_it_has_completed`:
    `the consumer must keep running while both producers run, after cancel()`. The consumer was
    cancelled by `cancel()` at the same moment as the producers.
  - `cargo test --lib app::tests::it_should_count_a_download_published_during_shutdown_by_a_producer_that_stops_before_the_listener`:
    1 failed with `left: 0, right: 2`: two completed downloads were published on the real swarm event
    bus after shutdown began, by two producers declared to stop before the listener, and the
    in-memory listener counted none of them.

### V2 - T8: the stop order (green) and the in-process application test

- Goal: show that, with the edges enforced, the two V1 tests pass, and that the application wires each
  producer to the listeners of the buses it publishes on.
- Working-tree state: the T8 change, uncommitted while these runs happened (committed with the T7 tests
  as one change, see the Commit Points of the spec): `JobManager::spawn_with` with enforcement in
  `cancel()` and `wait_for_all`, `src/bootstrap/event_flows.rs`, the wiring in `src/app.rs`, and the
  tests below.
- Commands and outcomes:
  - `cargo test --lib bootstrap::jobs::manager`: 33 passed: the 18 of V1 (the 17 from before and the V1
    red test, now green) and 15 new.
  - `cargo test --lib app::tests`: 19 passed, including the V1 real-bus test, now green (`left: 2`).
  - `cargo test --lib`: all root library tests pass (144 in the root crate).
  - `cargo test --test persistence-completed-downloads-on-shutdown`: 1 passed. It completes 20 downloads
    over HTTP, shuts the application down through the `JobManager` right after the last answer, and
    reads the `SQLite` database. It is regression coverage of the wiring, not a red proof: it was not
    red before this change either, because the listener drain of #2414 already persisted what the
    listeners had queued, and a request already answered cannot publish late.
  - `cargo test --test lifecycle-signals`: 13 passed.
  - `linter all`: exit 0.
- Tests added or changed (no test was removed or merged):
  - `manager.rs`: 16 new tests (the red V1 test among them) for the stop order, the clause tests below, the
    registration errors, and the report of what shutdown cancels and what waits.
  - `event_flows.rs`: 5 new unit tests.
  - `app.rs`: `it_should_count_a_download_published_during_shutdown_by_a_producer_that_stops_before_the_listener`
    (V1), `it_should_subscribe_every_started_swarm_listener_before_its_task_runs_or_any_producer_starts`
    (new), and the existing
    `it_should_stop_every_started_component_cooperatively_when_the_manager_is_cancelled`, extended with the
    declared stop order and the completion order, and renamed to
    `it_should_stop_every_started_component_cooperatively_and_each_producer_before_its_listeners_when_the_manager_is_cancelled`.
  - `tests/persistence/completed_downloads_on_shutdown.rs` (new binary) and
    `torrust_tracker_test_helpers::http::http_complete_download` (new helper).
- Limits of the application-level evidence: the completion-order assertion in the extended app test is
  real but timing-dependent (an idle HTTP server can complete before another producer), so it would not
  catch a dropped bus declaration by itself; the declared-order assertion (`waiting_components()`) does,
  deterministically (mutations W1 and W3 below).

### V3 - T8: mutations, one per lifecycle clause

Each mutation was applied to the working tree alone, the named test command was run, and the file was
restored from a copy. A mutation that did not compile is not counted (F4, W6 and W7 were first written
so that an unused variable failed under `-D warnings`, and rewritten). `manager` is
`cargo test --lib bootstrap::jobs::manager`; `app` is `cargo test --lib app::tests`.

| Clause (negative ones marked) | Mutation | Failing tests |
| --- | --- | --- |
| C1 a consumer is not cancelled while a predecessor is unfinished | M1: `cancel()` cancels every component | manager: 7 (`it_should_cancel_a_consumer_only_after_every_producer_that_stops_before_it_has_completed` and six others); app: the V1 real-bus test |
| C2 (negative) the ordered shutdown does not delay a component that waits for nobody | M2: `cancel()` cancels nobody while any edge exists | manager: `it_should_cancel_a_component_that_waits_for_nobody_at_once_while_another_waits_for_its_producer`, `it_should_report_which_components_the_shutdown_cancels_and_which_wait` |
| C3 a consumer is cancelled when its LAST predecessor completes | M3a: released when any predecessor completes | manager: `it_should_cancel_a_consumer_only_after_every_producer_that_stops_before_it_has_completed`, `it_should_cancel_each_consumer_when_the_producers_that_stop_before_it_have_completed`; app: the V1 real-bus test |
| C3 (and clause C5, below) | M3b: completion never releases a consumer | manager: 5 tests, among them `it_should_cancel_a_consumer_whose_producer_completed_before_shutdown_was_requested`; app: the V1 real-bus test |
| C4 (negative) completion alone before `cancel()` does not cancel a consumer | M4: release without checking that shutdown was requested | manager: `it_should_not_cancel_a_consumer_when_its_producers_complete_before_shutdown_is_requested` |
| C5 a predecessor that finished before `cancel()` does not hold its consumer | covered by M3b only | The clause holds by construction (the join that observes the completion happens after `cancel()`), so no separate natural mutation exists; the test `it_should_cancel_a_consumer_whose_producer_completed_before_shutdown_was_requested` documents it and is killed by M3b |
| C6 a failed or panicked predecessor releases its consumer and stays visible | M6: only `Completed` and `Cancelled` release | manager: `it_should_cancel_a_consumer_when_its_producer_fails_and_keep_the_failure_visible`, `it_should_cancel_a_consumer_when_its_producer_panics_and_keep_the_panic_visible` |
| C7 a slow producer does not delay unrelated consumers | M7: consumers released only when no component waits any more | manager: `it_should_cancel_each_consumer_when_the_producers_that_stop_before_it_have_completed` |
| C8 at the deadline, consumers never asked to stop are aborted and named | M8: every token is cancelled before the abort | manager: `it_should_abort_and_name_a_consumer_that_was_never_asked_to_stop_when_the_deadline_expires` |
| C9 (negative) a registration with a foreign id is rejected | M9a: a foreign id is read as a local index | manager: `it_should_reject_a_stop_before_target_issued_by_another_manager`, `it_should_leave_no_edge_and_no_running_component_when_a_spawn_with_edges_is_rejected` |
| C9 a released id is rejected | M9b: released ids accepted | manager: `it_should_reject_a_stop_before_target_whose_reservation_was_dropped` |
| C9 a target that is the component itself or later is rejected | M9c | manager: `it_should_reject_a_component_that_stops_before_itself`, `it_should_reject_a_stop_before_target_reserved_after_the_component` |
| C9 edges after `cancel()` are rejected | M9d | manager: `it_should_reject_stop_before_edges_after_shutdown_was_requested` |
| C9 (negative) a rejected registration leaves no edge | M9e: edges stored before validation finishes | manager: `it_should_leave_no_edge_and_no_running_component_when_a_spawn_with_edges_is_rejected` |
| C10 a second shutdown request changes nothing | M10: no idempotence guard | manager: `it_should_report_which_components_the_shutdown_cancels_and_which_wait` |
| F1 `EventFlows` rejects a consumer after a producer of its bus | consumer accepted after a producer | event_flows: `it_should_reject_a_consumer_registered_after_a_producer_of_its_bus` |
| F1 | a producer does not mark its buses | event_flows: the same test |
| F2 `consumers_of` unions the buses | only the first bus | event_flows: `it_should_return_the_consumers_of_every_bus_a_producer_publishes_on_without_duplicates` |
| F2 (no duplicates) | duplicates returned | event_flows: the same test |
| F3 (negative) a producer of one bus does not block consumers of another | any producer blocks every consumer | event_flows: `it_should_accept_a_consumer_of_a_bus_that_has_no_producer_yet` |
| W the wiring declares every bus of every producer | W1 HTTP without swarm; W2 HTTP without http-core; W3 UDP without swarm; W4 UDP without udp-server; W5 UDP without udp-core; W6 torrent cleanup without swarm; W7 the in-memory listener registered on the wrong bus | app: `it_should_stop_every_started_component_cooperatively_and_each_producer_before_its_listeners_when_the_manager_is_cancelled` for all seven (before the declared-order assertion existed, W1 and W3 survived: the HTTP and UDP instances happened to complete first) |
| Subscription before publication | the in-memory listener subscribes inside its task | app: `it_should_subscribe_every_started_swarm_listener_before_its_task_runs_or_any_producer_starts` (`got Some(Ok(2))`) |

### V4 - T9: the HTTP and UDP test environments stop the server first

- Goal: show that `stop()` of each environment cancels and joins the server, then the listeners, keeps
  the join-all failure reporting and both drop-path guarantees, and counts the event of a request that
  completes while the server drains.
- Working-tree state: the T9 change in `packages/axum-http-server/src/testing/environment.rs` and
  `packages/udp-server/src/testing/environment.rs`, uncommitted while these runs happened.
- Red (HTTP), run on the tree before the T9 change (the T7-T8 commit, environments untouched):
  `cargo test -p torrust-tracker-axum-http-server --lib it_should_count_the_statistics_event_of_a_request_that_completes_while_stop_drains_the_server`
  failed with `left: 0, right: 1`: "the announce completed while stop() was draining the server, so its
  event must be counted before stop() returns". The test sends every header of a `completed` announce
  but the final blank line, calls `stop()`, waits until the server refuses new connections (the
  observable sign that it began to shut down), finishes the request, and checks the answer
  (`HTTP/1.1 200`) before checking the metric, so the failure is the lost event and not an unanswered
  request. A first probe of this idea printed `stats: 0` on the same tree before the test existed.
- Green: after the change the same test passes (`tcp4_announces_handled` is 1).
- Why the UDP late-event test is not a red proof: a UDP request cannot be held in flight while the
  receive loop drains, so the UDP environment has no deterministic equivalent of the partial HTTP
  request. `it_should_count_the_statistics_events_of_a_request_answered_just_before_stop` is
  regression coverage (a request answered before `stop()`, both statistics listeners count it); it also
  passed under mutation U-M1 below. The order is proven for UDP by the staged-join unit test, which
  fails under U-M1.
- Commands and outcomes: `cargo test -p torrust-tracker-axum-http-server -p torrust-tracker-udp-server`:
  HTTP library 48 passed (11 in the environment module), UDP library 232 passed (12 in the environment
  module), and the integration and doc tests of both packages pass.
- Tests added (no test removed or merged): HTTP, the in-flight request test, the staged-order unit test,
  the abandoned-join unit test, and the drop-stops-the-listener test; UDP, the same four with the
  answered-request test in place of the in-flight one. The two existing failure-reporting tests keep
  their purpose and only pass the listener guard to `join_owned_tasks`; they no longer cancel the token
  before the call, because the function now cancels the listeners itself after the server stage.
- Design detail applied beyond D20's wording (maintainer approval requested, see record entry 32):
  `stop()` disarms the server guard, but the listeners guard moves into `join_owned_tasks` and is
  disarmed after the server stage, so a `stop()` future that is dropped early cancels the listeners.

| Clause | Mutation | Failing tests |
| --- | --- | --- |
| The listeners are cancelled only after the server (and its drain controller) finished | H-M1: HTTP, listener cancelled before the server stage | `it_should_cancel_the_event_listener_only_after_the_server_and_its_drain_controller_have_finished`, `it_should_count_the_statistics_event_of_a_request_that_completes_while_stop_drains_the_server` |
| The same, for the drain controller | H-M5: HTTP, the drain controller is not awaited before the listener is cancelled | `it_should_cancel_the_event_listener_only_after_the_server_and_its_drain_controller_have_finished` |
| The same, for UDP | U-M1: UDP, listeners cancelled with the receive loop | `it_should_cancel_the_event_listeners_only_after_the_receive_loop_has_finished` |
| (negative) a server failure does not skip the listener stage | H-M2, U-M2: the listeners are not joined when the first stage failed | `it_should_join_the_event_listener_before_reporting_an_http_server_task_failure`, `it_should_join_every_event_listener_before_reporting_a_receive_loop_failure` |
| (negative) an abandoned `stop()` leaves no detached listener | H-M4, U-M4: the guard is disarmed when the join starts | `it_should_cancel_the_event_listener_when_the_join_is_abandoned_while_the_server_runs`, `it_should_cancel_the_event_listeners_when_the_join_is_abandoned_while_the_receive_loop_runs` |
| Dropping the environment cancels the listeners too | H-M3, U-M3: the stored listeners guard guards another token | `it_should_stop_the_event_listener_when_dropped_without_being_stopped` and the tests that call `stop()` (which then hangs until their deadline); UDP the same with `it_should_stop_the_event_listeners_when_dropped_without_being_stopped` |
