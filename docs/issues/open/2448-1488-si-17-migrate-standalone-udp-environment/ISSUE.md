---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 1488
github-issue: 2448
spec-path: docs/issues/open/2448-1488-si-17-migrate-standalone-udp-environment/ISSUE.md
branch: "2448-migrate-standalone-udp-environment"
related-pr: null
last-updated-utc: "2026-10-06 16:02"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - packages/udp-server/src/testing/environment.rs
    - packages/udp-server/examples/udp_only_public_tracker.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/states.rs
    - packages/udp-server/tests/server/contract.rs
    - packages/axum-health-check-api-server/tests/server/contract.rs
    - packages/axum-http-server/src/testing/environment.rs
    - src/app.rs
    - docs/issues/closed/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/drafts/1488-si-3-fix-environment-stop/ISSUE.md
    - docs/issues/closed/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
    - docs/issues/closed/2370-1488-si-15-define-udp-active-request-policy/ISSUE.md
    - docs/issues/open/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md
    - docs/issues/open/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2448 - Migrate Standalone UDP Environment and Example to the Token Lifecycle

Parent: [EPIC #1488 - Overhaul: Tracker Shutdown](../../open/1488-overhaul-tracker-shutdown/ISSUE.md)

> **EPIC position**: Roadmap sequence 13 (SI-17). The UDP counterpart of SI-16
> (#2412): it moves the UDP package's integration tests onto the token-aware
> shutdown path that production uses.

## Goal

Make the UDP test `Environment` and the `udp_only_public_tracker` example use
the token-aware UDP server lifecycle. `Environment::stop()` cancels and joins
every task it owns (the receive loop and three event listeners) instead of
aborting the listeners. The example maps SIGINT and Unix SIGTERM to
`Environment::stop()`; the UDP library stays free of direct OS-signal
subscriptions.

## Background

SI-14 (#2342) added `Server::start_with_cancellation`, which returns a
`CancellationRunning` holding the receive-loop task; the tracker application
uses it. SI-15 (#2370) made that loop drain its request processors for up to
`REQUEST_DRAIN_DEADLINE` (5 s), then abort the rest, before returning. The
package's own test environment was not migrated, so today:

- `Environment::start` starts the server through the legacy `Halted` path
  (`Server::start`).
- `Environment::stop()` aborts the UDP core, server statistics, and server
  banning event listeners (three `todo` comments), then calls legacy
  `Server::stop()` with a 5 s timeout.
- The `udp_only_public_tracker` example waits only for Ctrl-C.

The package's integration tests (11 `stop()` calls under
`packages/udp-server/tests/`) and the health-check API contract tests therefore
never exercise the shutdown path the tracker runs in production.

Facts found while refreshing this draft (2026-10-06), including lessons from
SI-16 and its PR review (#2439):

1. **The library catches both signals today.** The legacy launcher selects on
   `torrust-server-lib`'s `global_shutdown_signal` (Ctrl-C and Unix SIGTERM).
   SI-14 manual verification found that Ctrl-C makes the example panic with
   `FailedToStartOrStopServer("Normal")`: the launcher stops first, so the
   environment's later `Server::stop()` cannot send its halt message. SIGTERM
   is expected to stop only the server and leave the process running, as the
   SI-16 baseline showed for HTTP; T0 records it.
2. **A failed start leaks the listeners.** `start` spawns the three listeners
   before the fallible server start, then panics on failure. The listeners keep
   the only token clones, so nothing cancels them (the same defect as
   `review-finding:pr-2439-f2`).
3. **The token lives in the stopped environment.** `new()` creates the token
   and `start` reuses it. Today `stop()` never cancels it; once it does, a
   restart would hand a cancelled token to the new listeners and server.
4. **The receive-loop drain is already bounded.** Unlike the HTTP helper before
   SI-16 D7, the loop aborts processors still running at the deadline, so its
   task always finishes. It returns `Ok(())` after cancellation even when it
   aborted processors; it logs the outcome but does not report a timeout.
5. **The stop timeout equals the drain deadline.** `DEFAULT_SERVER_LIFECYCLE_TIMEOUT`
   and `REQUEST_DRAIN_DEADLINE` are both 5 s, so a drain that reaches its
   deadline can trip the environment's stop panic first.
6. **A consumer stops the server through the field.**
   `packages/axum-health-check-api-server/tests/server/contract.rs` calls
   `service.server.stop()` on the UDP `Started` environment. All environment
   fields are public; no other consumer uses `server`, the listener handles,
   or `cancellation_token`.
7. **One token in production.** The application gives the three listeners and
   the server tokens that the job manager cancels together. As in SI-16, the
   listeners drop queued events on cancellation; SI-22 owns that fix.

## Scope

### In Scope

- Migrate `Environment` start and stop to `Server::start_with_cancellation`,
  with a fresh token per start (D2, D3).
- Spawn the listeners only after the server starts (D4).
- Remove the three `abort()` calls and their `todo` comments.
- Replace the stop timeout with joins bounded by the owned tasks (D5).
- Migrate the health-check API contract test that calls `service.server.stop()`.
- Make `udp_only_public_tracker` stop on SIGINT or Unix SIGTERM.
- Deterministic tests for the new `stop()` guarantees, without OS signals,
  replacing the sleep-based `it_should_make_and_stop_udp_server`.
- Update the shutdown task inventory.

### Out of Scope

- The standalone HTTP environment (done in SI-16).
- The REST API and health-check API test environments, which also still stop
  through the legacy path (SI-23, SI-24).
- Tracker `main()`, `JobManager`, and application bootstrap.
- The receive loop and the active-request policy (SI-14, SI-15).
- Deprecating or removing the legacy lifecycle (SI-18, SI-19).
- Drain budgets, configuration, and exit-code policy (SI-20).
- Processing queued events before listeners stop (SI-22).

## Design Decisions

- **D1 - Keep the `stop()` signature.** `Environment<Running>::stop(self) ->
  Environment<Stopped>` stays, so the existing callers do not change. It
  panics, naming the failing task, when the receive loop returns an error or
  any join fails. It joins every owned task before panicking, so an early
  failure does not detach the others (as SI-16's `join_owned_tasks`).
- **D2 - One fresh token per start.** `start` creates the token for that run;
  a stopped environment never reuses a cancelled token (fact 3).
- **D3 - One token, same as the application.** `stop()` cancels the token
  once, then joins the receive loop and the three listeners. Queued events are
  still discarded, as in production, until SI-22.
- **D4 - Spawn listeners after a successful start.** Take the three event
  receivers before the server starts, so no event is missed, and spawn the
  listeners only after `start_with_cancellation` succeeds (fact 2).
- **D5 - No outer stop timeout.** Every owned task is bounded: the receive loop
  by its drain deadline (fact 4) and the listeners by cancellation. `stop()`
  joins without its own timeout, like the HTTP environment, which removes the
  5 s race with the drain deadline (fact 5). Tests bound their own waits. The
  start timeout in `Environment<Running>::new` stays. Evidence: after
  cancellation, the receive loop's `biased` select takes the cancelled branch,
  drains with the deadline, and returns; each listener selects on
  `cancelled()`. Maintainer rule (2026-10-06): remove the timeout if it is not
  needed, otherwise raise it above the drain deadline; if implementation finds
  an unbounded await on the stop path, use a timeout above
  `REQUEST_DRAIN_DEADLINE` instead.
- **D6 - Running state holds the token-aware handles.** `Environment<Running>`
  stores the token-aware running state instead of `Server<Running>`, with
  private state types behind the existing `Started`/`Unstarted` aliases, as in
  SI-16. `bind_address()`, `registar`, and `container` stay available.
  `Environment::stop()` is the only stop path.
- **D7 - The example is the OS-signal boundary.** It installs SIGINT and Unix
  SIGTERM handlers (Ctrl-C only on non-Unix) before printing readiness, then
  calls `Environment::stop()`, as SI-16's HTTP example does.

## Architectural Decisions

- Related ADRs:
  [Adopt a supervised cancellation tree for shutdown](../../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md).
- ADRs to create: none known.

## Design and Ownership Review

- **Owner**: `Environment<Running>` owns the receive-loop task, the three
  listeners, and the token that stops them. Nothing outside it aborts them.
- **Normal path**: `stop()` cancels the token, joins all four tasks, and
  returns `Environment<Stopped>`. The UDP socket is released when `stop()`
  returns.
- **Failure path**: join every owned task, then panic naming each failing task
  (D1). A failed start leaves no task running (D4).
- **Drop path**: dropping a running environment without `stop()` keeps today's
  behavior. No new detached task is added.
- **Deadlines**: the drain is bounded by `REQUEST_DRAIN_DEADLINE`; tests bound
  every start and stop with an explicit test deadline.
- **Checkpoint**: after the first passing vertical slice (T1), stop for a
  design review before migrating the consumer and the example.

## Bug-Fix Process

Not applicable. This is a planned migration. The example's Ctrl-C panic
(fact 1) and the listener leak (fact 2) are fixed by the migration itself; T0
records the first as baseline and T1 guards both with tests.

## Regression Test Strategy

Not applicable (not a bug). See the Implementation Plan for the tests that
guard the new `stop()` guarantees.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                  | Notes / Expected Output                                                                                     |
| --- | ------ | ------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| T0  | DONE   | Record baseline                       | Both packages pass (212 + 12 + 1; 3 + 8). SIGINT panics (exit 101); SIGTERM releases the socket but the process stays up, and a later SIGINT panics. Evidence V1. |
| T1  | DONE   | Migrate `Environment` start/stop      | D1-D6 with the five T1 tests, each mutation-proven (see the 2026-10-06 15:31 UTC log entry). Design approved 2026-10-06. |
| T2  | DONE   | Migrate the direct field consumer     | Health-check API contract test uses `Environment::stop()`; 3 + 8 pass.                                      |
| T3  | DONE   | Example signal boundary (D7)          | `main` installs SIGINT/SIGTERM handlers (Ctrl-C on non-Unix) before printing readiness, then calls `Environment::stop()`; module docs updated. M2 and M3 exit 0. |
| T4  | DONE   | Documentation                         | Task inventory findings 4 and 8 record the migrated UDP consumer and point the remaining legacy users to SI-23 and SI-24. |
| T5  | TODO   | Verification and completion review    | Automatic checks, manual scenarios, AC review, pre-push checks, independent Task Reviewer report.           |

T1 tests (use the `write-unit-test` skill, no OS signals, every wait bounded):

- `stop()` lets the three listeners finish through cancellation: no join
  failure.
- `stop()` releases the UDP socket: the same address can be bound right after
  `stop()` returns.
- Stop-then-start serves requests on the restarted environment (guards D2).
- A failed server start leaves no listener running (guards D4).
- A receive-loop failure is reported after every listener joins (guards D1).

## Commit Points

| Task  | Coherent change set                                         | Commit policy                                                       |
| ----- | ----------------------------------------------------------- | ------------------------------------------------------------------- |
| T0    | Baseline evidence in `manual-verification-evidence.md`      | Commit with T1 or separately.                                       |
| T1-T2 | Environment, tests, and direct field consumer migration     | Commit together after design review, since the field type changes.  |
| T3    | Example signal boundary                                     | Commit after the manual SIGTERM/SIGINT check.                       |
| T4    | Shutdown documentation updates                              | Commit after `linter all`.                                          |
| T5    | Verification evidence, AC review, and completion review     | Commit after all checks pass.                                       |

For test-producing work, use the `write-unit-test` skill and complete the
prose-first Arrange-Act-Assert design review after each passing test increment,
before maintainer review and commit. Sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted (moved to `docs/issues/open/2448-1488-si-17-migrate-standalone-udp-environment/ISSUE.md`)
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2448)
- [x] Spec-only PR merged into `develop` before implementation (#2451)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-30 12:24 UTC - Planner - Draft written after SI-14 and SI-15.
- 2026-10-06 11:18 UTC - GitHub Copilot - Refreshed against current code and SI-16's lessons: converted to the issue template; added facts 1-7 (library-level SIGTERM, listener leak on failed start, token reuse, bounded drain, stop-timeout race, direct `server` field consumer, one production token) and decisions D1-D7. Removed the draft `verification.md`; `manual-verification-evidence.md` is created from the template at T0, as in SI-16; the SI-3 draft's link now points to this spec.
- 2026-10-06 11:34 UTC - GitHub Copilot - Maintainer review: D5 approved on the condition that the stop timeout is not needed, otherwise raised above the drain deadline; D5 now records the evidence (the receive loop and the listeners both end on cancellation). The REST API and health-check API environment gap became SI-23 and SI-24, one item each.
- 2026-10-06 11:41 UTC - GitHub Copilot - Maintainer approved this spec with SI-23 and SI-24. Created #2448 (sub-issue of #1488) and moved the spec to `docs/issues/open/`. Next: the shared spec-only pull request.
- 2026-10-06 15:31 UTC - GitHub Copilot - T0 done (evidence V1): the baseline matches fact 1. T1 and T2 done as the first vertical slice. Environment states are private `Stopped`/`Running` types behind the existing aliases; each start creates a fresh token, takes the three event receivers, starts through `start_with_cancellation`, and spawns the listeners only after success; `stop()` cancels once and joins the receive loop and the three listeners through `join_owned_tasks`, with no outer timeout (D5); the start timeout stays, renamed `DEFAULT_SERVER_START_TIMEOUT`. The sleep-based `it_should_make_and_stop_udp_server` is replaced by five tests. Mutation proofs, each reverted by hand: (A) aborting one listener fails the three stop-path tests, naming the listener; (B) spawning a listener before the start fails the failed-start test with holders `[1, 2, 1]` against `[1, 1, 1]`; (C) a pre-cancelled token fails only the restart test (no answer within the 10 s deadline); (D) returning on a receive-loop failure before joining fails the join-order test (0 of 3 listeners finished); (E) a `stop()` that never stops the server fails the socket test with `AddrInUse`, and so does the narrower one that cancels but does not await the receive loop (3/3 runs), unlike HTTP in SI-16: the UDP socket is owned by the receive-loop task and freed only when it ends. Suite timing: unit 2.17 s (was 2.17 s), integration 5.04 s (was 5.05 s), so no drain slowdown, unlike HTTP. Prose-first AAA review of the five tests: (a) stop: Arrange a started environment, Act stop within the deadline, Assert no join failure; matches. (b) failed start: Arrange an occupied port and the holder counts of the listeners' shared state, Act start, Assert it panicked and the counts are unchanged; the kept container is causal and has a one-line comment. (c) socket: Arrange a started environment and its binding, Act stop, Assert the address binds; matches. (d) restart: Arrange an environment stopped once, Act start it again, Assert a connect request gets a connect response; matches. (e) join order: Arrange a failed receive loop and three listeners that finish only after cancellation and one more scheduling turn, Act join, Assert the failure is reported and all three finished; the extra turn is causal and commented. Clippy (pedantic) required renaming the listener fields to drop the shared `udp_` prefix. Checkpoint: design review before T3.
- 2026-10-06 15:43 UTC - GitHub Copilot - Maintainer approved the T1 design (including the renamed listener fields and the still-public `connection_id_validation`); T0-T2 committed. Next: T3.
- 2026-10-06 15:51 UTC - GitHub Copilot - T3 done, mirroring SI-16's HTTP example: `main` installs the Unix signal streams before printing readiness (non-Unix uses `ctrl_c()`), prints which signal it received, and calls `Environment::stop()`. M2 (SIGTERM) and M3 (SIGINT): announce succeeds, orderly stop, exit 0, socket released (evidence V2, V3). M4: no direct `tokio::signal` in `packages/udp-server/src`; the legacy launcher keeps its indirect subscription until SI-19 (V4).
- 2026-10-06 15:55 UTC - GitHub Copilot - T4 done: task inventory findings 4 and 8 now record the migrated UDP environment and example, and name SI-23 and SI-24 as the remaining legacy test-environment users. `questions.md` is the decision record and stays unchanged, as in SI-16. The EPIC's findings 2 and 8 still say "until SI-17"; they are updated at archive time, as SI-16's were.
- 2026-10-06 16:02 UTC - GitHub Copilot - AC review against the current tree: AC1-AC9 hold, each with its evidence in Acceptance Verification. Pre-push checks pass in 1 m 22 s (nightly `rustc 1.101.0-nightly (ea137335b 2026-10-05)` for fmt/check/doc, stable `rustc 1.99.0` for the full test suite). Completion review: no `implementation-retrospective.md` needed. The design held as specified (D1-D7, no D5 fallback needed). The two material discoveries are recorded in the 15:31 entry: the UDP socket is freed only when the receive-loop task ends, so the socket test also guards the await (unlike HTTP); and the UDP stop adds no measurable suite time. Next: the independent Task Reviewer report, then the implementation PR.

## Acceptance Criteria

- [x] AC1: `Environment` starts the UDP server through `start_with_cancellation`.
- [x] AC2: `Environment::stop()` cancels the environment token and joins the
      receive loop and the three event listeners; it never calls `abort()`.
- [x] AC3: `stop()` panics with a message naming each failing task, after
      joining every owned task, and keeps its current signature.
- [x] AC4: A stopped environment can be started again and serves requests.
- [x] AC5: A failed start leaves no event listener running.
- [x] AC6: The health-check API contract tests pass without using the
      environment's `server` field to stop the UDP tracker.
- [x] AC7: `udp_only_public_tracker` stops gracefully on SIGINT and on Unix
      SIGTERM and exits with code 0.
- [x] AC8: No UDP library module subscribes to OS signals directly; the legacy
      `Server::start`/`stop` API still compiles, its tests pass, and it keeps
      its indirect subscription until SI-19.
- [x] AC9: The shutdown task inventory reflects the migration.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-udp-server`
- `cargo test -p torrust-tracker-axum-health-check-api-server`
- `cargo build -p torrust-tracker-udp-server --example udp_only_public_tracker`
- Pre-push checks before opening the implementation PR

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

Run the built example binary directly
(`target/debug/examples/udp_only_public_tracker`), not through `cargo run`,
so the signal reaches the example's own PID.

| ID  | Scenario                       | Human-oriented command/steps                                                                                               | Expected Result                                                                              | Status | Evidence                                     |
| --- | ------------------------------ | -------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Baseline SIGINT and SIGTERM    | Before T3: start the example, wait for `Listening on`, send `kill -INT <pid>`; repeat with `kill -TERM <pid>`; record output and `$?`. | Recorded as found (expected: SIGINT panics; SIGTERM stops only the server).                  | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Announce, then SIGTERM         | Start the example, announce with `tracker_client udp announce` to its address, `kill -TERM <pid>`, record output and `$?`. | Announce succeeds; output shows shutdown and `Stopped.`; exit code 0.                       | DONE   | `manual-verification-evidence.md` section V2 |
| M3  | SIGINT                         | Repeat M2 with `kill -INT <pid>`.                                                                                          | Same orderly stop as M2; exit code 0.                                                        | DONE   | `manual-verification-evidence.md` section V3 |
| M4  | No library signal subscription | `rg -n 'tokio::signal' packages/udp-server/src`                                                                            | No matches. The legacy path still subscribes indirectly via `torrust-server-lib` until SI-19. | DONE   | `manual-verification-evidence.md` section V4 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record the toolchain for every command result that it can affect.
- Create `manual-verification-evidence.md` from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` at T0.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `Environment::start` calls `Server::start_with_cancellation`; the contract and environment tests pass. |
| AC2   | DONE                   | `stop()` cancels once and calls `join_owned_tasks`; no `abort()` in `environment.rs`. Mutation A (abort a listener) fails 3 tests. |
| AC3   | DONE                   | Signature unchanged; `join_owned_tasks` names each failing task after joining all four. Mutation D fails the join-order test. |
| AC4   | DONE                   | `it_should_serve_requests_after_being_stopped_and_started_again` (mutation C, a pre-cancelled token, fails it). |
| AC5   | DONE                   | `it_should_not_leave_event_listeners_running_when_the_udp_server_fails_to_start` (mutation B fails it with `[1, 2, 1]`). |
| AC6   | DONE                   | The UDP contract test uses `let _stopped = service.stop().await;`; 3 + 8 pass. The remaining `service.server.stop()` is the REST API one (SI-23). |
| AC7   | DONE                   | Evidence V2 (SIGTERM) and V3 (SIGINT): orderly stop, exit 0, socket released. |
| AC8   | DONE                   | Evidence V4: no direct `tokio::signal` in `packages/udp-server/src`; the legacy launcher keeps `global_shutdown_signal` until SI-19; its tests pass in the 216 unit tests. |
| AC9   | DONE                   | Task inventory findings 4 and 8. |

## Dependencies

- Done: SI-14 (#2342) token-aware UDP lifecycle, SI-15 (#2370) active-request
  policy, SI-16 (#2412) as the reference migration.
- Not required: SI-20 exit-code mapping. The example exits 0 on graceful stop
  under the current policy.

## Risks and Trade-offs

- **Slow drains slow tests.** A test that leaves a request processor running
  makes `stop()` wait up to the 5 s drain deadline; it no longer panics at a
  5 s stop timeout (D5).
- **Events queued at stop are still dropped.** Unchanged from today and from
  production; SI-22 fixes it.
- **Two more test environments still use the legacy stop.** The REST API and
  health-check API environments stop through the legacy path. Found while
  refreshing this spec; the maintainer chose one item each:
  [SI-23](../2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md) (#2449) and
  [SI-24](../2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md) (#2450).

## Rollback

Revert the environment, consumer, and example commits; the legacy `Halted`
path is still available, so the package tests return to their previous
behavior.

## Implementation Completion Review

- Retrospective: `Not needed` (see the 2026-10-06 16:02 UTC progress-log entry)
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory;
  otherwise add a progress-log entry explaining why it was not needed.
- Independent reviewers record results in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1488
- Reference migration: #2412 (SI-16), PR #2439
- Related issues: #2342, #2370 (SI-14, SI-15)
- Follow-up items: SI-22 (listener event loss), SI-18, SI-19, SI-20
- Related ADR: `docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`
