---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2416-2410-si-22-4-stop-producers-before-listeners/ISSUE.md
last-updated-utc: "2026-10-23 16:22"
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
  - `cargo test --lib`: all root library tests pass (146 in the root crate, counted on the final tree).
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
  - `event_flows.rs`: 4 new unit tests (a fifth, a near-duplicate of the first, was dropped before the first commit and was never committed).
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
- Why `it_should_count_the_statistics_events_of_a_request_answered_just_before_stop` is not a red proof:
  it is regression coverage (a request answered before `stop()`, both statistics listeners count it) and
  it also passed under mutation U-M1 below. The order is proven for UDP by the staged-join unit test,
  which fails under U-M1. **Correction (review-finding:pr-2623-f2):** this bullet first said that a UDP
  request cannot be held in flight while the receive loop drains, so the UDP environment had no
  deterministic equivalent of the partial HTTP request. That was wrong: V7 holds a request in flight in
  the UDP environment and has a deterministic red.
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
| (negative) an abandoned `stop()` leaves no listener that was never asked to stop (the listeners are cancelled, not joined) | H-M4, U-M4: the guard is disarmed when the join starts | `it_should_cancel_the_event_listener_when_the_join_is_abandoned_while_the_server_runs`, `it_should_cancel_the_event_listeners_when_the_join_is_abandoned_while_the_receive_loop_runs` |
| Dropping the environment cancels the listeners too | H-M3, U-M3: the stored listeners guard guards another token | `it_should_stop_the_event_listener_when_dropped_without_being_stopped` and the tests that call `stop()` (which then hangs until their deadline); UDP the same with `it_should_stop_the_event_listeners_when_dropped_without_being_stopped` |

### V5 - T10: M1, M2, M3 on the real binary

- Goal: the `fix-bug` skill's like-for-like recheck of the original symptom (M2 and M3, with the T0
  script and parameters) and the stop-order logs under load (M1).
- Initial state: the working tree was clean at the commit
  `docs(architecture): [#2416] document the stop order, move the diagrams, and add the swarm bus`; the
  code under test is that of
  `fix(axum-http-server,udp-server): [#2416] stop the server before the listeners in the test environments`
  (the commits after it change documentation and module-doc comments only). Built with
  `cargo build --bin torrust-tracker --bin tracker_client` (the `dev` profile, `opt-level = 1`, as the
  T0 evidence), `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Linux 7.0.0-38-generic, 32 CPUs, a shared machine.
  Times are UTC on 2026-10-21.
- Status: `DONE`.

#### M2 and M3: the T0 parameters, the sub-EPIC's script

`reproduce-lost-completions.sh` with `PEERS=500` for the control runs and `PEERS=2000 SIGTERM_DELAY=0.5`
for the race runs. The run logs are retained in the [`evidence/`](evidence/) folder, each run as
`<run>.tracker.log` and `<run>.summary.txt` (the script's own summary, with its `work_dir=` line removed because
it was a local absolute path with a random suffix): `m2-control-wait1`, `m2-control-wait10`, `m3-race-1`,
`m3-race-2`, and the failed start `m3-race-failed-start` below (the run directories were under `.tmp/si22-t0/`,
which is git-ignored, so the repository copy is the evidence). A retained log is the tracker's stdout and
stderr with the colour codes removed and **only** the lines that contain `request{` removed, which are the
thousands of per-request `INFO` lines of the HTTP tracker; every other line is kept (116 to 182 lines each).
The command was `sed 's/\x1b\[[0-9;]*m//g' tracker.log | grep -v ' request{' > <run>.tracker.log`.

| Run (UTC start) | Mode | Answered `completed` (refused) | Persisted (torrent / global) | Answered minus persisted | Exit status | Persistent listener drain | T0 on `develop` (answered, persisted, lost) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 19:00:13 | control, `CONTROL_WAIT=1` | 500 (0) | 500 / 500 | 0 | 0 | `processed=196 lost_to_lag=0 outcome="emptied"` | 500, 467, 33 |
| 19:00:55 | control, `CONTROL_WAIT=10` | 500 (0) | 500 / 500 | 0 | 0 | `processed=0` (the queue was already empty) | 500, 500, 0 |
| 19:01:46 | race | 813 (1,187) | 813 / 813 | 0 | 0 | `processed=1496 lost_to_lag=0 outcome="emptied"` | 856, 66, 790 |
| 19:04:46 | race | 889 (1,111) | 889 / 889 | 0 | 0 | `processed=1144 lost_to_lag=0 outcome="emptied"` | 845, 70, 775 |

- Every control completion was persisted within the budget (sub-EPIC AC7). The race runs show no deficit
  between answered completions and persisted rows.
- What the race rows do not show: after #2414 the real binary lost nothing in this scenario (the
  coordinator's check of 2026-10-21 and these runs agree), so they cannot show window (b) as red and do
  not prove the stop order. The proof is V1. The `drain` counts are events of every type on the bus
  (the persistent listener also receives the `started` announces' events), so `processed` is not a
  count of completions, and the table claims no per-event attribution.
- Observed lag, aborts, and failures: the four logs contain no `lost_to_lag` above 0, no `Aborted`,
  `Failed`, or `Panicked` job outcome, no `WARN`, and no `ERROR` line. These counts were rechecked on
  2026-10-23 on the **full** logs, with no line excluded (an earlier filter of this section also dropped
  the `request{` lines, which could in principle have hidden a warning logged inside a request span), with
  `grep -cE '^[0-9T:.-]+Z +(WARN|ERROR) '`, `grep -cE 'lost_to_lag=[1-9]'`, `grep -cE 'Job was aborted|Job reported a failure|Job panicked'`,
  `grep -ci lagged`, `grep -c 'Failed to'` and `grep -ci 'database error'`, each on the colour-stripped log:
  every count was 0 in the four runs and in the M1 burst run (2,147 lines in each control log, 5,773 and 5,926
  in the race logs, 6,368 in the burst log). The retained logs hold the same lines minus the `request{` ones, so
  a reviewer can repeat the counts there and know that only per-request `INFO` lines are missing. The race runs let the persistent listener drain for about 2.4 and 1.6 seconds, within
  the 5 second admission budget, and the tracker exited 0.
- The filter used for the log claims (run per log): `sed 's/\x1b\[[0-9;]*m//g' <log> | grep -E "Event listener drained its queue|Job was aborted|Job reported a failure|Job panicked|^[0-9T:.-]+Z +(WARN|ERROR) |Remaining alive|Requested the job to stop: every|shutting down \(SIGTERM\)|successfully shutdown" | grep -v "request{"`.
- One extra run, 19:02:31 (race; log `evidence/m3-race-failed-start.tracker.log`), did not start the tracker: "Tracker startup failed ... Could not start
  the HTTP tracker listener ... bind address is available". The previous race run had ended 40 seconds
  earlier on the same port, and `ss` showed no `:47070` socket 90 seconds later. #2414's V10 saw the same
  after a run and I did not investigate it either (out of scope). Its log shows the startup-failure path of
  `start_jobs` with the new manager: every listener (none had a producer yet) was asked to stop at once and
  completed cooperatively, and no component waited for a producer that never started. The run is not
  counted in the table; the second race run was repeated 90 seconds later.

#### M1: the stop order in the logs, with SIGTERM during a burst

A disposable harness, kept here as evidence like #2414's:
[`evidence/m1-tracker.toml`](evidence/m1-tracker.toml) (an HTTP tracker, a UDP tracker, the health check,
persistent completed statistics, SQLite) and [`evidence/run_m1.py`](evidence/run_m1.py). During the run
the configuration was at `.tmp/m1/tracker.toml`. It starts the tracker, announces `started` for 300 UDP
and 3,000 HTTP peers, then sends the `completed` announces from 48 worker threads and SIGTERM 0.15 seconds
into the burst: `UDP_PEERS=300 HTTP_PEERS=3000 SIGTERM_DELAY=0.15 python3 docs/issues/open/2416-2410-si-22-4-stop-producers-before-listeners/evidence/run_m1.py`.
A first run with the defaults (150 UDP and 500 HTTP peers, SIGTERM at 0.3 s) finished the whole burst
before SIGTERM, so it showed nothing under load and is not used. A second run failed to start for the port
reason above and is not used either.

Run at 18:59:37 to 18:59:39 (SIGTERM at 18:59:38.012): tracker exit status 0; completed announces
answered, UDP 300 of 300 and HTTP 91 of 3,000 (the others were refused after the server stopped
accepting); persisted rows `(1, '3b24...', 300)`, `(2, '5349...', 91)` and
`('torrents_downloads_total', 391)`: every answered completion was persisted. No lagged receiver and no
aborted component.

The shutdown log, reduced to the stop-order lines (colour codes and span prefixes removed, listener
lines abbreviated; the full order is the order of the lines):

```text
18:59:38.012234 shutting down (SIGTERM)
18:59:38.012268 Requested the job to stop: it waits for no other job  job=udp_instance_0_127.0.0.1:47271
18:59:38.012273 Requested the job to stop: it waits for no other job  job=http_instance_0_127.0.0.1:47270
18:59:38.012278 Requested the job to stop: it waits for no other job  job=torrent_cleanup
18:59:38.012281 Requested the job to stop: it waits for no other job  job=peers_inactivity_update
18:59:38.012300 Requested the job to stop: it waits for no other job  job=health_check_api
18:59:38.012304 Job will be asked to stop after the jobs it waits for have completed  job=swarm_coordination_registry_event_listener waiting_for=udp_instance_0_127.0.0.1:47271, http_instance_0_127.0.0.1:47270, torrent_cleanup
         (the same for tracker_core_in_memory_event_listener and tracker_core_persistent_completed_statistics_event_listener)
18:59:38.012365 Job will be asked to stop after ...  job=http_core_event_listener waiting_for=http_instance_0_127.0.0.1:47270
18:59:38.012370 Job will be asked to stop after ...  job=udp_core_event_listener waiting_for=udp_instance_0_127.0.0.1:47271
         (the same for udp_server_stats_event_listener and udp_server_banning_event_listener)
18:59:38.012325 Remaining alive connections: 18 (89.999999358s remaining)       <- HTTP drain
18:59:38.012419 Job completed after cooperative cancellation  job=udp_instance_0_127.0.0.1:47271
18:59:38.012425 Requested the job to stop: every job it waited for has completed  job=udp_core_event_listener last_completed_job=udp_instance_0_127.0.0.1:47271
         (the same for the two udp-server listeners; each then logs "drained its queue ... processed=0 lost_to_lag=0")
18:59:39.013437 All connections closed, shutting down server in address 127.0.0.1:47270
18:59:39.013497 Job completed after cooperative cancellation  job=http_instance_0_127.0.0.1:47270
18:59:39.013522 Requested the job to stop: every job it waited for has completed  job=swarm_coordination_registry_event_listener last_completed_job=http_instance_0_127.0.0.1:47270
         (the same for the in-memory listener, the persistent listener, and http_core_event_listener, each then draining with processed=0 lost_to_lag=0)
18:59:39.013635 Torrust tracker successfully shutdown.
```

Reading it: every listener that has a producer logged `waiting_for` the producers of its buses at
SIGTERM, and each "every job it waited for has completed" line follows the `Job completed` line of the
last producer it waited for. The UDP listeners were released as soon as the UDP instance completed
(18:59:38.0124), while the HTTP instance was still draining its 18 live connections; the swarm and
http-core listeners were released a second later (18:59:39.0135), when the HTTP instance completed.
That second is the window in which HTTP requests could still publish events after SIGTERM; with every
component cancelled at once, the listeners would have been asked to stop at 18:59:38.012. The log
proves the order; it does not count how many events the second carried, and the drain lines show
`processed=0` because the persistent listener kept up while requests were still being answered.

### V6 - Automatic checks before the pull request

- Working tree: clean at `docs(architecture): [#2416] document the stop order, move the diagrams, and add the swarm bus`
  (the evidence and spec commit that follows changes only documents).
- `./contrib/dev-tools/git/hooks/pre-push.sh` (with `TORRUST_GIT_HOOKS_LOG_DIR=.tmp`), exit 0, six steps
  passed: nightly format, nightly workspace check, nightly documentation (binaries and libraries), the
  workspace packaging check, and the full stable test suite (about one minute of tests).
- `cargo test --test lifecycle-signals`: 13 passed.
- `linter all`: exit 0 on every commit (the pre-commit hook ran it).

### V7 - Review round 1: a deterministic UDP late-event test (review-finding:pr-2623-f2)

- Goal: answer the review finding that a UDP request can be held in flight, as the crate's own
  `HoldAcceptedRequestsSender` (`packages/udp-server/src/server/launcher.rs`, in the receive-loop shutdown
  tests) does, so the UDP environment can have a deterministic red like the HTTP one.
- Working-tree state: the tree at the commit
  `test(udp-server): [#2416] assert the pre-drop listener holders in the UDP drop test` plus the uncommitted test
  below, which was committed as `test(udp-server): [#2416] hold a UDP request in flight to test the late event when stopping`
  after these runs. The mutations were applied to `packages/udp-server/src/testing/environment.rs` alone
  and the file was restored after each. `rustc 1.99.1`, the `test` profile, 2026-10-22.
- Test: `it_should_count_the_statistics_event_of_a_request_that_is_published_while_stop_drains_the_receive_loop`
  in `packages/udp-server/src/testing/environment.rs`. A wrapping sender (`HoldAcceptedRequestsBeforePublishing`)
  replaces the container's UDP-server event sender, holds each `UdpRequestAccepted` publication, which a request
  processor makes, until released, and forwards it to the real bus afterwards. The test sends a connect request,
  waits until its processor holds the publication, starts `stop()`, lets the other tasks run (so `stop()` has
  cancelled the receive loop and waits for the processor), releases the publication, and checks that
  `udp4_connect_requests_accepted_total` is 1 after `stop()` returns.
- Red: with mutation U-M1 (the listeners are cancelled together with the receive loop, as the environment did
  with one token) `cargo test -p torrust-tracker-udp-server --lib testing::environment` fails
  `it_should_count_the_statistics_event_of_a_request_that_is_published_while_stop_drains_the_receive_loop` and
  `it_should_cancel_the_event_listeners_only_after_the_receive_loop_has_finished`. Under mutation U-M3 (the
  listeners guard guards another token) it fails too, with the tests that call `stop()`. The red run used
  U-M1 and was not run on the tree before the T9 change: U-M1 cancels the listeners together with the
  receive loop, which reproduces the one-token order of that tree exactly. **Correction
  (review-finding:pr-2623-f9):** this bullet first said the tree before the T9 change "does not have the
  wrapper hooks the test needs". That was wrong: the wrapper and its helper need only the container fields
  and `stop()`, which that tree also has; the real reason is that U-M1 was the cheaper way to reproduce the
  one-token order, and the test has not been run on that tree.
- Green: on the tree without a mutation the module passes (13 tests), and 15 repeated runs of the test passed.
- The answered-request test stays as regression coverage; no test was removed or merged.

### V8 - Review round 4: the retained recheck logs (Copilot review thread on V5)

- Finding: the V5 claims about lag, warnings, failures and aborts depended on logs that existed only under the
  git-ignored `.tmp/`, against the "Preserving Verification Artifacts" rule of the evidence template.
- Action: the logs of the four M2 and M3 runs, of the failed start, and of the M1 burst run are retained in
  `evidence/` (the M1 one as `m1-burst.tracker.log`), with the per-run summaries, in the form and with the
  filter described at the top of V5. I did not rerun the scenarios: the original logs were available.
- Retained files and the first 16 hex digits of their SHA-256, so that a later edit is visible:
  `m1-burst.tracker.log` `0deef13a104c1ad2`, `m2-control-wait1.tracker.log` `5384a3ecadac08af`,
  `m2-control-wait10.tracker.log` `fad0a44e4d6eca1e`, `m3-race-1.tracker.log` `6e799e0124a941dc`,
  `m3-race-2.tracker.log` `f45db69d776a32cb`, `m3-race-failed-start.tracker.log` `dffd0638c0a0d56c`.
- The check of the same claims on the full logs, without the `request{` filter, is recorded in V5.
