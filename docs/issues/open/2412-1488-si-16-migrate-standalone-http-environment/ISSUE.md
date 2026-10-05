---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 1488
github-issue: 2412
spec-path: docs/issues/open/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md
branch: "2412-migrate-standalone-http-environment"
related-pr: null
last-updated-utc: "2026-10-05 15:53"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - packages/axum-http-server/src/testing/environment.rs
    - packages/axum-http-server/examples/http_only_public_tracker.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-health-check-api-server/tests/server/contract.rs
    - packages/axum-server/src/signals.rs
    - packages/http-core/src/statistics/event/listener.rs
    - src/main.rs
    - src/bootstrap/jobs/http_tracker.rs
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/drafts/1488-si-3-fix-environment-stop/ISSUE.md
    - docs/issues/drafts/1488-si-17-migrate-standalone-udp-environment/ISSUE.md
    - docs/issues/drafts/1488-si-20-configure-shutdown-policy/ISSUE.md
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - docs/issues/closed/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md
    - docs/issues/closed/2289-1488-si-11-migrate-http-tracker-token-lifecycle/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2412 - Migrate Standalone HTTP Environment and Example to the Token Lifecycle

Parent: [EPIC #1488 - Overhaul: Tracker Shutdown](../../open/1488-overhaul-tracker-shutdown/ISSUE.md)

> **EPIC position**: Roadmap sequence 12 (SI-16). It runs before SI-20 on
> purpose: it moves the HTTP package's integration tests onto the token-aware
> shutdown path that production uses, so later budget and readiness changes
> (SI-20, SI-21) are covered by those tests.

## Goal

Make the HTTP test `Environment` and the `http_only_public_tracker` example use
the token-aware HTTP server lifecycle. `Environment::stop()` cancels and joins
every task it owns (server, drain controller, statistics event listener) instead
of aborting any of them. The example maps SIGINT and Unix SIGTERM to
`Environment::stop()`; the HTTP library stays free of OS-signal subscriptions.

## Background

SI-11 (#2289) migrated the tracker application's HTTP component to
`HttpServer::start_with_cancellation`, which returns a `CancellationRunning`
with a joinable server task and a joinable drain controller. The package's own
test environment was not migrated, so today:

- `Environment::start_with_health_check` starts the server through the legacy
  `Halted` path (`HttpServer::start_with_health_check`).
- `Environment::stop()` aborts the statistics event listener (with a `todo`
  comment) and stops the server through legacy `HttpServer::stop()`.
- The `http_only_public_tracker` example waits only for Ctrl-C.

As a result, the package's integration tests (about 30 `env.stop().await`
calls under `packages/axum-http-server/tests/`) and the health-check API
contract tests never exercise the shutdown path the tracker runs in production.
An aborted listener also leaves no proof of orderly completion, and copied
example code ignores the standard Unix termination signal.

Facts found while refreshing this draft (2026-10-01) that shape the design:

1. **The health-check seam is private on the token-aware path.**
   `start_with_cancellation_and_health_check` is private. The HTTPS
   health-check contract test needs `Environment::start_with_health_check` to
   inject a client that trusts its test certificate, so the token-aware
   variant must become public (additive).
2. **A consumer touches the environment's server field directly.**
   `packages/axum-health-check-api-server/tests/server/contract.rs` calls
   `service.server.stop()` on `Environment<Running>`. Replacing the running
   server state changes that field's type, so this test must migrate too.
3. **A cancelled token cannot be reset.** `Environment<Stopped>` currently
   keeps the token it was created with. After a stop, that token is cancelled,
   so a restart would immediately stop the new listener and server.
4. **Event listeners drop queued events on cancellation.** Every event
   listener stops as soon as its token is cancelled, even with events still
   queued, and requests completing during the server drain emit events after
   the listener has stopped. This affects the tracker application too and was
   already true before the cancellation-token refactor. SI-22 owns the fix; this
   issue keeps the application's behavior (one token, cancelled together).

## Scope

### In Scope

- Make the token-aware start with an injected health-check callback public on
  `HttpServer<Stopped>` (additive; legacy methods unchanged).
- Migrate `Environment` start and stop to the token-aware path, with a fresh
  token per start (D2, D3).
- Remove `event_listener_job.abort()` and its `todo` comment.
- Migrate the health-check API contract test that calls `service.server.stop()`.
- Make `http_only_public_tracker` stop on SIGINT or Unix SIGTERM.
- Deterministic tests for the new `stop()` guarantees, without OS signals.
- Update the shutdown task inventory and feature documentation.

### Out of Scope

- The standalone UDP environment and example (SI-17).
- Tracker `main()`, `JobManager`, and application bootstrap.
- Deprecating or removing the legacy `Halted` lifecycle (SI-18, SI-19).
- Drain budgets, configuration, and exit-code policy (SI-20). The environment
  keeps the current `HTTP_GRACEFUL_DRAIN_TIMEOUT`.
- Processing queued events before listeners stop, and stopping event
  producers before consumers (SI-22).
- Readiness changes (SI-21).

## Design Decisions

- **D1 - Keep the `stop()` signature.** `Environment<Running>::stop(self) ->
  Environment<Stopped>` stays as is, so the ~30 existing callers do not change.
  Like today, it panics with a descriptive message when it cannot stop
  cleanly: server task join error, drain-controller join error, drain
  `TimedOut`, or listener join error. Collect all join results before panicking,
  so an early error does not drop the remaining handles and detach their tasks.
  A panic is the test environment's defined failure result.
- **D2 - One fresh token per start.** `start` creates the tokens for that run;
  a stopped environment never reuses a cancelled token, so stop-then-start
  works.
- **D3 - One token, same as the application.** `stop()` cancels the
  environment token once, then joins the server task, the drain controller,
  and the listener. Matching production is the point of this issue: package
  tests exercise the shutdown path the tracker runs. Events still queued at
  stop are discarded, as today. SI-22 changes the application and the test
  environments together. (An ordered stop was considered and rejected: the
  listener drops queued events on cancellation, so ordering alone does not
  prevent loss, and it would make the test environment diverge from
  production.)
- **D4 - Public token-aware health-check start.** Expose the existing private
  `start_with_cancellation_and_health_check` (final name decided during
  implementation). This is additive and mirrors legacy
  `start_with_health_check`.
- **D5 - Running state holds the token-aware handles.** `Environment<Running>`
  stores the token-aware running state instead of `HttpServer<Running>`.
  Callers that stopped the server through the field use `Environment::stop()`.
  The legacy `HttpServer::start`/`stop` API and its own unit test stay.
- **D6 - The example is the OS-signal boundary.** It waits for SIGINT or Unix
  SIGTERM (Ctrl-C only on non-Unix) in its own `main`, following the pattern in
  `src/main.rs`, then calls `Environment::stop()`. No library module subscribes
  to OS signals.

## Architectural Decisions

- Related ADRs:
  [Adopt a supervised cancellation tree for shutdown](../../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md).
- ADRs to create: none known. Shutdown ordering between event producers and
  consumers is SI-22's decision.

## Design and Ownership Review

- **Owner**: `Environment<Running>` owns the server task, the drain controller,
  the statistics event listener, and the tokens that stop them. Nothing outside
  it may abort them.
- **Normal path**: `stop()` cancels the environment token, joins the server,
  the drain controller, and the listener, then returns `Environment<Stopped>`.
  The HTTP binding is released when `stop()` returns.
- **Failure path**: collect outcomes and join every owned task before reporting
  any join error or drain timeout as a panic naming the failing task (D1).
- **Drop path**: dropping a running environment without `stop()` (for example
  after a test panic) keeps today's behavior. No new detached task is added.
- **Deadlines**: the drain is bounded by `HTTP_GRACEFUL_DRAIN_TIMEOUT`.
  Deterministic tests bound every await with an explicit test timeout and do
  not depend on the 90-second value.
- **Checkpoint**: after the first passing vertical slice (T2), stop for a
  design review before migrating consumers and the example.

## Bug-Fix Process

Not applicable. This is a planned migration. The lost-event bug in Background
fact 4 is tracked and fixed by SI-22.

## Regression Test Strategy

Not applicable (not a bug). See the Implementation Plan for the tests that
guard the new `stop()` guarantees.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                       | Notes / Expected Output                                                                                                                                    |
| --- | ------ | ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T0  | DONE   | Record baseline                            | Both packages pass (36 + 61; 3 + 8). Example SIGTERM: the library catches it, stops only the server, and the process stays up; a later SIGINT makes `stop()` panic (exit 101). See evidence V1. |
| T1  | DONE   | Public token-aware health-check start (D4) | `start_with_cancellation_and_health_check` made public (name kept: mirrors legacy `start_with_health_check`). Unit test proves the registry runs the injected callback; mutation-proven. Maintainer approved 2026-10-05. |
| T2  | DONE   | Migrate `Environment` start/stop (D1-D3)   | Implemented with the four planned tests, each mutation-proven. Design approved 2026-10-05. |
| T3  | DONE   | Migrate direct field consumers (D5)        | Health-check API contract tests use `Unstarted` and `Environment::stop()`; the 100 ms port-release sleep is removed (T2's binding test proves `stop()` releases it). 3/3 runs pass. |
| T4  | DONE   | Example signal boundary (D6)               | `main` installs SIGINT/SIGTERM handlers (Ctrl-C on non-Unix) before printing readiness, then calls `Environment::stop()`; module docs updated. M2 and M3 exit 0. |
| T5  | DONE   | Documentation                              | Task inventory findings 4 and 8 record the migrated HTTP consumer; the feature README's stale "implementation has not started" status now points to EPIC #1488. |
| T6  | DONE   | Verification and completion review         | Automatic checks, manual scenarios, AC review, pre-push checks (pass, 58 s), and two Task Reviewer passes (second: REVIEW PASSED). A drain-timeout test was added for AC3. |

T2 tests (use the `write-unit-test` skill, no OS signals):

- `stop()` lets the listener finish through cancellation: its join result is
  `Ok`, not a cancelled `JoinError`.
- `stop()` releases the HTTP binding: the same address can be bound right
  after `stop()` returns.
- Stop-then-start runs a working server on the restarted environment (guards
  D2).
- An early server/drain failure still joins the listener before `stop()` panics;
  test waits are bounded. Drop without `stop()` remains explicitly unchanged,
  not a new claim that panic cleanup is graceful.

## Commit Points

| Task | Coherent change set                                         | Commit policy                                        |
| ---- | ----------------------------------------------------------- | ---------------------------------------------------- |
| T0   | Baseline evidence in `manual-verification-evidence.md`      | Commit with T1 or separately.                        |
| T1   | Public token-aware health-check start and its unit test     | Commit after focused validation and required review. |
| T2-T3 | Environment, tests, and direct field consumer migration    | Commit together after design/maintainer review and both package checks, since the field type changes. |
| T4   | Example signal boundary                                     | Commit after manual SIGTERM/SIGINT check.            |
| T5   | Shutdown documentation updates                              | Commit after `linter all`.                           |
| T6   | Verification evidence, AC review, and completion review     | Commit after all checks pass.                        |

For test-producing work, use the `write-unit-test` skill and complete the
prose-first Arrange-Act-Assert design review after each passing test increment,
before maintainer review and commit. Sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted (moved to `docs/issues/open/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md`)
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2412)
- [x] Spec-only PR merged into `develop` before implementation (#2421)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-25 - Planner - Initial SI-16 draft written before SI-11 to SI-15 merged.
- 2026-10-01 14:35 UTC - GitHub Copilot - Refreshed against current code: added the public health-check seam (D4), the direct `server` field consumer (D5), fresh tokens per start (D2), and ordered stop (D3); converted to the current issue template. Sequenced before SI-20 so package tests cover the production shutdown path.
- 2026-10-01 15:13 UTC - GitHub Copilot - Maintainer review: replaced the ordered stop with one token, same as the application (D3), because listeners drop queued events on cancellation. That pre-existing bug moves to SI-22. Renamed `verification.md` to `manual-verification-evidence.md`.
- 2026-10-02 06:39 UTC - GitHub Copilot - Maintainer approved this spec (2026-10-01). Next: GitHub issue creation and the shared #1488 spec-only pull request.
- 2026-10-04 10:34 UTC - GitHub Copilot - T0 done. The SIGTERM baseline differs from the expected one: the legacy `Halted` path subscribes to SIGTERM inside the library (`torrust-server-lib` `global_shutdown_signal`), so the server stops while the example keeps running, and the later Ctrl-C makes `Environment::stop()` panic. T2 and T4 remove both causes.
- 2026-10-04 16:53 UTC - GitHub Copilot - T1 implemented: the existing private method is public with the same name, since it is the token-aware counterpart of `start_with_health_check`. Test `it_should_register_the_injected_health_check_when_starting_the_token_aware_http_server` was proven by registering `check_fn` instead of the injected callback (failed with `left: Ok("200 OK")`), then reverted by hand. Prose-first AAA review: the code states the causal input (the injected callback in the Act) and the independent expected marker; no extra prose kept.
- 2026-10-05 11:44 UTC - GitHub Copilot - Maintainer approved T1; committed. Next: T2.
- 2026-10-05 12:06 UTC - GitHub Copilot - T2 first passing slice. Environment states are private `Stopped`/`Running` types with `Unstarted`/`Started` aliases (mirroring the UDP environment); each start creates a fresh token; `stop()` cancels once and joins server, drain controller, and listener through `join_owned_tasks`, which awaits all three before reporting every failure. Tests and mutation proofs: listener aborted (3 stop-path tests fail, naming the listener); early return on server failure (the listener-join test fails); start with a pre-cancelled token (the restart test fails with connection reset). Finding: a stop with an idle keep-alive connection takes about 1 s, because `graceful_shutdown_on_cancellation` re-checks the connection count every second; the integration suite went from 0.28 s to 2.24 s. That is production drain behavior, previously hidden because the legacy path did not join its drain task. Tokio's `time` feature added as a dev-dependency for bounded test waits.
- 2026-10-05 12:19 UTC - GitHub Copilot - Maintainer approved the T2 design and accepted the ~1 s drain cost (2.24 s suite is fine). No follow-up issue: a short comment in `graceful_shutdown_on_cancellation` records the future options (wake on connection-count changes, or a cancellation kind that skips the drain), which are outside the shutdown refactor. T3 done; workspace `cargo check --all-targets` and `linter all` pass.
- 2026-10-05 12:36 UTC - GitHub Copilot - T4 done. Handlers are installed before readiness is printed, so a signal sent after `Listening on` cannot hit the default action; on Unix this uses signal streams, which register immediately (`ctrl_c()` registers on first poll). Non-Unix uses `ctrl_c()`; only the Linux target is installed locally, so CI's Windows build checks that branch (corrected 2026-10-05 15:53 UTC: this entry first said "Windows/macOS", but macOS is `cfg(unix)` and compiles the signal-stream branch). M2 (SIGTERM) and M3 (SIGINT) stop in order and exit 0. M4: no direct signal use in the HTTP library; the legacy path still subscribes indirectly until SI-19, so AC7 is read as direct subscriptions only.
- 2026-10-05 12:48 UTC - GitHub Copilot - AC review against the current tree. AC3 had no test for the drain-timeout case; added `it_should_report_a_drain_timeout_even_when_every_task_joins`, proven by ignoring `TimedOut`. Completion review: no `implementation-retrospective.md` needed; the two material discoveries (the SIGTERM baseline caught inside the library, and the 1 s drain polling) are recorded in this log, the evidence, and a code comment. Pending: pre-push checks and the independent Task Reviewer report.
- 2026-10-05 13:18 UTC - GitHub Copilot - Task Reviewer report (REVIEW FAILED: two Major documentation gaps, no code defects) recorded in `agent-review-reports.md`. Fixes: (1) prose-first AAA comparison for the five environment tests, below; (2) the evidence file's template placeholders replaced by a table of the automated tests; (3) AC7 text scoped to direct subscriptions; (4) the first environment test renamed to what it asserts, `it_should_stop_without_a_join_failure_when_every_owned_task_finishes_through_cancellation`; (5) the example's readiness comment qualified as Unix-only, since non-Unix `ctrl_c()` registers on first poll; (6) the Unix signal import moved to the file top under `cfg(unix)`; (7) the `Future` import dropped (edition 2024 prelude); (8) spec status set to `in-progress`. The drain-cost suggestion needs no action.
- 2026-10-05 13:18 UTC - GitHub Copilot - Prose-first AAA comparison for the environment tests. (a) Stop without a join failure: Arrange a started environment; Act stop it within the deadline; Assert the stop task finished without a panic. The code says exactly this; the old name claimed more, so it was renamed. (b) Release binding: Arrange a started environment and its actual binding; Act stop; Assert the same address binds immediately. Matches. (c) Restart: Arrange an environment stopped once; Act start it again; Assert its health check answers 200. Matches. (d) Server failure: Arrange a panicking server task, a drained controller, and a listener that finishes only after cancellation and one more scheduling turn; Act join them; Assert the server failure is reported and the listener finished. The extra scheduling turn was causal but implicit, so a one-line comment now says why. (e) Drain timeout: Arrange a finished server, a timed-out controller, and a finished listener; Act join them; Assert exactly the drain-timeout failure. Matches.
- 2026-10-05 14:55 UTC - GitHub Copilot - Second Task Reviewer pass: REVIEW PASSED; all first-pass findings resolved; one optional Nit (non-Unix `ctrl_c()` called by full path) left as is, since a single-use `cfg(not(unix))` import adds nothing. Pre-push checks pass in 58 s (nightly `rustc 1.101.0-nightly (282215592 2026-10-04)` for fmt/check/doc, stable `rustc 1.99.0` for the full test suite). T6 done; next: the implementation PR.
- 2026-10-05 15:53 UTC - GitHub Copilot - PR #2439 Copilot review (three findings, all valid). F1 (Major): `graceful_shutdown_on_cancellation` started an unbounded graceful phase (`graceful_shutdown(None)`), so after `TimedOut` a held connection kept the server task alive and `stop()` could wait forever; it now calls `handle.shutdown()` at the deadline, as the legacy path's bounded grace did. This changes the shared helper used by the HTTP, REST API, and health-check servers; their tests pass. New test `it_should_force_close_a_connection_still_open_when_the_drain_times_out` holds a real connection; removing the force-close makes it fail. F2 (Major): the environment spawned the listener before the fallible server start, so a failed start detached it while its bus stayed open; it now subscribes first and spawns after a successful start. New test `it_should_not_leave_the_statistics_listener_running_when_the_http_server_fails_to_start` keeps the container (and so the bus) alive and checks no extra holder of the statistics repository remains; the pre-fix order makes it fail with 2 holders. A first version that dropped the container passed even before the fix, because closing the bus ends the leaked listener; the test now states that condition. F3 (Minor): the T4 entry's macOS claim is corrected above.

## Acceptance Criteria

- [x] AC1: `Environment` starts the HTTP server through the token-aware path,
      including when a custom health-check callback is injected.
- [x] AC2: `Environment::stop()` cancels the environment token and joins the
      server task, the drain controller, and the event listener; it never
      calls `abort()`.
- [x] AC3: `stop()` panics with a message naming the failing task on any join
      error or drain timeout, and keeps its current signature.
- [x] AC4: A stopped environment can be started again and serves requests.
- [x] AC5: The health-check API contract tests pass without using the
      environment's `server` field to stop the HTTP tracker.
- [x] AC6: `http_only_public_tracker` stops gracefully on SIGINT and on Unix
      SIGTERM and exits with code 0.
- [x] AC7: No HTTP library module subscribes to OS signals directly; the
      legacy `HttpServer::start`/`stop` API still compiles and its tests pass,
      and keeps its indirect subscription (via `axum-server` and
      `torrust-server-lib`) until SI-19.
- [x] AC8: The shutdown task inventory and feature documentation reflect the
      migration.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-axum-http-server`
- `cargo test -p torrust-tracker-axum-health-check-api-server`
- `cargo build -p torrust-tracker-axum-http-server --example http_only_public_tracker`
- Pre-push checks before opening the implementation PR

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

Run the built example binary directly
(`target/debug/examples/http_only_public_tracker`), not through `cargo run`,
so the signal reaches the example's own PID.

| ID  | Scenario                       | Human-oriented command/steps                                                                                                 | Expected Result                                                                    | Status | Evidence                                     |
| --- | ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Baseline SIGTERM (before T4)   | Start the example, wait for `Listening on`, `kill -TERM <pid>`, record output and `$?`.                                      | Recorded: SIGTERM caught by the library, process stays up; SIGINT then panics `stop()` (exit 101). | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Announce, then SIGTERM         | Start the example, announce with `tracker_client http announce` to its address, `kill -TERM <pid>`, record output and `$?`. | Announce succeeds; output shows shutdown and `Stopped.`; exit code 0.             | DONE   | `manual-verification-evidence.md` section V2 |
| M3  | SIGINT                         | Repeat M2 with `kill -INT <pid>`.                                                                                            | Same orderly stop as M2; exit code 0.                                              | DONE   | `manual-verification-evidence.md` section V3 |
| M4  | No library signal subscription | `rg -n 'tokio::signal' packages/axum-http-server/src`                                                                        | No matches. The legacy path still subscribes indirectly via `axum-server`/`torrust-server-lib` until SI-19. | DONE   | `manual-verification-evidence.md` section V4 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record the toolchain for every command result that it can affect.
- Create `manual-verification-evidence.md` from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing the scenarios.
- Record a failed scenario and its diagnosis in the progress log before
  continuing.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `Environment::start_with_health_check` calls `start_with_cancellation_and_health_check`; HTTPS health-check contract test passes with the injected trusted client. |
| AC2   | DONE                   | `stop()` cancels once and calls `join_owned_tasks`; no `abort()` in `environment.rs`. Tests: listener finishes through cancellation (mutation: abort fails 3 tests). |
| AC3   | DONE                   | Signature unchanged; `join_owned_tasks` names each failing task. Tests: server failure reported after the listener joins; drain timeout reported (both mutation-proven). |
| AC4   | DONE                   | `it_should_serve_requests_after_being_stopped_and_started_again` (mutation: pre-cancelled token fails it). |
| AC5   | DONE                   | Contract tests use `Unstarted` and `Environment::stop()`; 3/3 runs pass without the port-release sleep. |
| AC6   | DONE                   | Evidence V2 (SIGTERM) and V3 (SIGINT): orderly stop, exit 0. |
| AC7   | DONE                   | Evidence V4: no direct `tokio::signal` in `packages/axum-http-server/src`; legacy path keeps its indirect subscription until SI-19; legacy unit test passes. |
| AC8   | DONE                   | Task inventory findings 4 and 8; feature README status. |

## Dependencies

- Done: SI-2 (#2234) token-aware server lifecycle API, SI-10 (#2274)
  token-aware Axum drain helper, SI-11 (#2289) HTTP tracker token lifecycle.
- Not required: SI-20 exit-code mapping. The example exits 0 on graceful stop
  under the current policy.

## Risks and Trade-offs

- **Slow drains block tests.** A test that leaves a connection open makes
  `stop()` wait for the 90-second drain, then panic (D1). This is the same
  bound as the legacy path; SI-20 makes the budget configurable.
- **Events queued at stop are still dropped.** Unchanged from today and from
  production; no current test reads statistics after `stop()`. SI-22 fixes it.
- **Public API growth.** D4 adds one public method that SI-19 does not remove,
  because it belongs to the token-aware path.

## Rollback

Revert the environment, consumer, and example commits; the legacy `Halted`
path is still available, so the package tests return to their previous
behavior. The public method from T1 can stay. Revert it only after all callers
added by this migration have been reverted or migrated away from it.

## Implementation Completion Review

- Retrospective: `Not needed` (see the 2026-10-05 12:48 UTC progress-log entry)
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory;
  otherwise add a progress-log entry explaining why it was not needed.
- Independent reviewers record results in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1488
- Related issues: #2234, #2274, #2289 (SI-2, SI-10, SI-11)
- Follow-up items: SI-17 (UDP environment), SI-22 (listener event loss),
  SI-18, SI-19, SI-20, SI-21
- Related ADR: `docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`
