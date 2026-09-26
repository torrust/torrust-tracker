---
schema-version: 1
doc-type: issue
issue-type: task
status: open
priority: p1
epic: 1488
github-issue: 2342
spec-path: docs/issues/open/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
branch: "2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle"
related-pr: null
last-updated-utc: "2026-09-26 14:45"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/app.rs
    - src/bootstrap/jobs/udp_tracker.rs
    - src/bootstrap/jobs/manager.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/spawner.rs
    - packages/udp-server/src/server/states.rs
    - packages/udp-server/src/server/request_buffer.rs
    - packages/udp-server/src/testing/environment.rs
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - docs/issues/closed/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md
    - docs/issues/closed/2324-1488-si-13-migrate-health-check-api-token-lifecycle/ISSUE.md
    - docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md
    - docs/issues/drafts/1488-si-17-migrate-standalone-udp-environment/ISSUE.md
    - docs/issues/drafts/simplify-udp-server-main-loop/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2342 - Migrate UDP Tracker to Token Lifecycle

Parent EPIC: #1488 - Overhaul: Tracker Shutdown

> **EPIC position**: Roadmap step 10. One independently releasable UDP
> ownership slice. Active-request policy remains unchanged and is SI-15.

## Goal

Migrate each UDP tracker instance to the supervised cancellation tree. The
bootstrap derives one child `CancellationToken` per UDP binding; the UDP
receive loop observes it, stops admitting datagrams, and returns; the component
joins that loop and reports one named `udp_instance_<index>_<address>` outcome
to `JobManager`. No UDP task may panic, detach, or hang on any stop or drop
path, including the legacy `Halted` path.

## Background

The HTTP tracker (SI-11), REST API (SI-12), and health-check API (SI-13) now use
token-aware paths. UDP is the last server component on the transitional bridge.

Current path, verified on `develop` after PR #2336:

- `src/app.rs` `start_udp_instance` passes `job_manager.new_cancellation_token()`,
  a clone of the root token, not a child.
- `src/bootstrap/jobs/udp_tracker.rs` starts `Server<Running>` and returns a
  future that builds `NestedServerTask` only when first polled. On token
  cancellation it sends private `Halted::Normal` and joins the launcher task.
- `Server::start` (`states.rs`) binds the socket, spawns the launcher task
  through `Spawner`, awaits a `Started` oneshot, registers the service, and on
  registration failure sends `Halted` and joins.
- `Launcher::run_with_graceful_shutdown` (`launcher.rs`) spawns a second task,
  the receive loop (`run_udp_server_main`), then `select!`s on that task or
  `shutdown_signal_with_message(rx_halt)`, which also subscribes to
  `SIGINT`/`SIGTERM`. On halt it aborts the receive loop and awaits it.
- `run_udp_server_main` returns `()`. `Interrupted` returns, any other receive
  error `break`s, and the component then reports `Completed`: a failure is
  indistinguishable from a normal stop.
- Each datagram spawns a request processor tracked by an `AbortHandle` in
  `ActiveRequests` (capacity 50). Dropping `ActiveRequests` aborts unfinished
  processors without joining them. SI-15 owns that policy.
- `udp_ban_cleanup` is a separate application-level legacy-registry job on the
  root token, not a per-listener child.

Ownership gaps found while mapping the code (the SI-13 lesson: move handles into
a drop-safe owner before returning the component future):

1. **Legacy drop path panics and detaches.** Dropping `Server<Running>` (for
   example, an unpolled component future dropped at the `JobManager` deadline)
   drops the halt sender. `torrust_server_lib::signals::shutdown_signal` then
   panics inside the launcher (`Failed to install stop signal`), which drops the
   receive-loop `JoinHandle`: the loop detaches and keeps the socket bound.
2. **Launcher abort detaches.** Aborting the launcher task (the
   `NestedServerTask` drop path) also drops the receive-loop `JoinHandle`, so
   the loop keeps running.
3. **Silent failure.** Receive-loop errors are reported as normal completion.

## Scope

### In Scope

- A token-aware UDP server start path in `packages/udp-server` with no
  OS-signal subscription.
- A cooperative, token-aware receive loop that returns `Result` (see D1, D3).
- A flattened token-aware path with no intermediate launcher task (D2).
- A small single-task drop-safe owner in `src/bootstrap/jobs/manager.rs` (D4).
- One child token per configured UDP binding in `src/app.rs`.
- Reimplement the legacy launcher as an adapter over the token-aware runtime so
  legacy consumers keep their API and stop behavior but can no longer panic or
  detach the receive loop (D5).
- Deterministic tests and manual SIGTERM/rebind evidence.

### Out of Scope

- `ActiveRequests` capacity, eviction, or request-processor abort semantics;
  request drain deadlines, outcomes, or metrics (SI-15).
- Migrating the standalone UDP environment and example to the token-aware path
  (SI-17); they keep using the legacy API, which D5 makes safe.
- Deprecating or removing the legacy `Halted` API or `global_shutdown_signal()`
  (SI-18, SI-19).
- `udp_ban_cleanup` migration out of the legacy registry.
- The request-pipeline refactor in the `simplify-udp-server-main-loop` draft;
  touch `run_udp_server_main` only for the stop condition and return type.
- Shutdown deadline configuration and exit codes (SI-20).

## Design Decisions

Decisions agreed with the maintainer before drafting (2026-09-25). D1 and D5
were agent recommendations requested by the maintainer; the maintainer approved
them with the draft on 2026-09-26.

### D1 - Cooperative stop in the receive loop (recommended)

The token-aware receive loop `select!`s `cancellation_token.cancelled()`
(biased first) against `receiver.next()`. On cancellation it stops admitting
datagrams, drops `ActiveRequests` exactly as today, and returns `Ok(())`.

| Option | Pros | Cons |
| ------ | ---- | ---- |
| **Cooperative (chosen)** | Admission stops at a defined point between datagrams, never mid-way through per-datagram work (event publish, ban check, processor spawn), so metrics stay consistent. The join yields a real result instead of `JoinError::Cancelled`, which cannot be told apart from an external abort. Tests assert a returned value. It is the seam SI-15 needs: after the loop exits, a drain policy can replace the drop without restructuring. Matches the ADR: cooperative cancellation top-down, abort only as escalation. | Touches the hot loop (one extra `select!` branch per receive, negligible). Cancellation is observed only between datagrams, so a slow await inside the loop body delays the stop; the `JobManager` deadline plus owner-drop abort remains the bound. |
| Abort-and-join | Mirrors legacy code; no change to the loop body. | Abort can cut the loop at any `.await`, for example after publishing `UdpRequestReceived` but before spawning its processor. The outcome is always a cancelled `JoinError`. Leaves no seam for SI-15 without redoing this work. |

### D2 - Flatten the token-aware path

`Server::start_with_cancellation` binds the socket, logs the existing startup
lines, spawns only the receive loop, registers the service, and returns the
loop handle. On registration failure it cancels the token, joins the loop, and
releases the socket before returning. The component wraps the handle in its
owner before returning its future. There is no launcher task and no `Started`
oneshot on this path.

| Option | Pros | Cons |
| ------ | ---- | ---- |
| **Flatten (chosen)** | One task to own, so no nested handle can detach when an intermediate task is aborted or panics (gaps 1 and 2 cannot occur). Startup errors are returned directly instead of through a oneshot. Same shape as the health-check migration, easing SI-18/SI-19 consolidation. Fewer moving parts to test. | Diverges from the legacy structure, so the token-aware and legacy paths look different until D5 makes the legacy launcher an adapter. Startup logging and registration move into the new function and must keep identical messages. |
| Keep launcher task, swap `rx_halt` for the token | Smallest diff to `launcher.rs`; keeps `Spawner`/`LaunchRequest` in use. | Keeps two nested tasks; aborting the launcher still detaches the loop unless another owner is added inside it. The `Started` oneshot remains a failure mode. |

### D3 - Receive-loop failures are explicit

On the token-aware path, cancellation returns `Ok(())` and the component
reports `Cancelled`. A receive error (including `Interrupted`) or stream end
without cancellation returns an error and the component reports a failed
`udp_instance_*` outcome. Error logs keep their current messages.

### D4 - Single-task drop-safe owner

Add a small owner type next to `TokenAwareServerTask` in `manager.rs` that
holds one `JoinHandle`, aborts it on drop, and exposes `join`. The UDP
component creates it before returning its future.

| Option | Pros | Cons |
| ------ | ---- | ---- |
| **Owner in manager.rs (chosen)** | Consistent with `NestedServerTask` and `TokenAwareServerTask`; no new crate feature; the owner vocabulary stays in one module for SI-18/SI-19 consolidation. | One more small type. |
| `tokio_util::task::AbortOnDropHandle` | Existing library type. | Requires enabling tokio-util's `rt` feature on the root crate; mixes two owner vocabularies. |

### D5 - Controlled stop on the legacy path too (recommended)

Principle from the maintainer: every start and stop is controlled; no panics,
no child task in an unknown state, and nothing hangs. Reimplement
`Launcher::run_with_graceful_shutdown` as a thin adapter over the token-aware
receive loop: it creates its own `CancellationToken`, owns the receive loop with
the D4 owner semantics, and cancels the token when the halt message arrives,
the halt sender is dropped, or `global_shutdown_signal()` fires (preserving the
legacy OS-signal behavior until SI-19). It then joins the loop and returns.
`Server::start`, `Server::stop`, `Running`, `Spawner`, and `LaunchRequest` keep
their public signatures.

| Option | Pros | Cons |
| ------ | ---- | ---- |
| **Adapter over the token-aware loop (chosen)** | One receive-loop implementation, not two; legacy consumers (standalone environment, example, contract tests) get the no-panic and no-detach guarantees now; SI-17 becomes a call-site migration and SI-19 deletes a thin adapter. Satisfies the maintainer principle everywhere. | Larger SI-14. Legacy failure modes change: a dropped halt sender stops cleanly instead of panicking, and a receive error surfaces as `Err` from the task instead of `Ok`. Both are fixes, but they are behavior changes for legacy consumers and need tests. |
| Fix only the legacy panic and detach in place | Smaller than the adapter. | Keeps two receive-loop code paths with separate stop logic until SI-19. |
| Record only; defer to SI-17/SI-19 | Smallest SI-14. | Leaves a known panic and detached socket in supported consumers, contrary to the maintainer principle. |

Observed legacy behavior changes after implementation (all are failure or
stop-timing modes; normal start, serve, and stop are unchanged):

- A dropped halt sender stops the launcher cleanly instead of panicking.
- Aborting the launcher aborts the receive loop instead of detaching it.
- A receive-loop error or panic surfaces as `Err` from the launcher task, so
  `Server::stop` returns `UdpError::Launcher` instead of succeeding. This path
  has no dedicated test: a real UDP receive error cannot be provoked
  deterministically, the error mapping is unit-tested in `admit_received`, and
  the launcher returns the loop result directly.
- A halt now stops the loop at its next check between datagrams instead of
  aborting it at any await point.
- The "Halting UDP Service Bound to Socket" info line is now logged from the
  UDP server launcher module instead of `torrust_server_lib::signals`, so its
  tracing target changed; the message text is unchanged.

### D6 - Measure UDP throughput before and after

D1 is the only change on the per-datagram hot path. Per wake-up the extra
`biased` branch costs one mutex lock with no contention in
`CancellationToken::is_cancelled` plus one poll of an already-registered
`Notified` (tokio-util 0.7.19): tens of nanoseconds, no allocation, no syscall.
That is small next to the existing per-datagram work (the `recvfrom` syscall,
event publishing, ban checks, the processor `tokio::spawn`, and
`ActiveRequests::force_push`), so no measurable regression is expected. D2-D5
do not touch the hot path.

The maintainer asked to verify this rather than rely on reasoning: record a
baseline on `develop` before implementation starts (T1) and measure again after
implementation (T7), on the same machine with the same build profile, tracker
configuration, and load-test settings. Both measurements live in issue-local
`performance-evidence.md`.

## Implementation Constraints

1. Legacy `Server::start` / `Server::stop` consumers remain source-compatible;
   their normal stop behavior is unchanged. Only the D5 failure-mode fixes
   change observable legacy behavior.
2. The token-aware path has no `SIGINT` or `SIGTERM` subscription.
3. Cancellation stops datagram admission before the receive loop returns;
   in-flight processors keep today's drop-abort behavior.
4. The component owns and joins its single child before returning its outcome;
   `JobManager` never receives nested handles. The owner exists before the
   component future is returned.
5. Every stop, failure, and drop path is explicit: no panic, no detached task,
   no unbounded wait beyond the `JobManager` deadline.
6. Startup log target and messages are unchanged (the E2E log parser matches
   `Started on: udp://`).
7. The receive loop creates its `cancelled()` future once, pinned outside the
   loop, and reuses it in every `select!`; creating it per iteration would
   register and remove a notifier waiter for each datagram.

## Architectural Decisions

- Related ADRs:
  [`20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`](../../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md)
- ADRs to create: none planned. D1-D5 apply the existing ADR. Create one if
  implementation changes component ownership, deadline, or compatibility policy.

## Design and Ownership Review

- **Interfaces**: `Server::start_with_cancellation` (package) returns the bound
  address and the receive-loop handle; `udp_tracker::start_job` (bootstrap)
  returns the component future; `JobManager` sees only that future.
- **Normal path**: root token -> component child token -> receive loop exits
  -> component joins it -> `Cancelled`.
- **Failure paths**: receive error or stream end -> loop returns error ->
  component reports failure. Registration failure -> cancel, join, release
  socket, return error.
- **Drop paths**: component future dropped before or after first poll -> owner
  aborts the loop -> socket released. Legacy `Server<Running>` dropped -> halt
  sender dropped -> adapter cancels and joins -> socket released, no panic.
- **Deadlines**: no new awaited readiness step on the token-aware path; stop is
  bounded by the `JobManager` shared deadline with owner-drop escalation.
- **Checkpoint**: design review after the first passing token-aware vertical
  slice (T4), before the legacy adapter (T5).

## Bug-Fix Process

Not applicable as a standalone bug: this is a lifecycle migration. The legacy
drop-path panic and detach (gaps 1 and 2) are fixed as part of D5; T5 proves
each with a regression test that fails against the current legacy launcher,
following the write-unit-test "Prove a Regression Test Guards the Bug" rule.

## Regression Test Strategy

- Gap 1: drop a legacy `Server<Running>` and assert the socket becomes bindable
  within a bound, with no launcher panic.
- Gap 2 and the token-aware drop path: drop the unpolled component future and
  assert the socket becomes bindable within a bound.
- Gap 3: a receive-loop error yields a failed component outcome.
- Prove each by mutation in the working tree (never staged), then restore.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Confirm UDP ownership map and record the performance baseline | No change: no commit touched the UDP server package, `udp_tracker.rs`, `manager.rs`, or `src/app.rs` between the mapping (after PR #2336) and the branch base. Baseline of five runs recorded in `performance-evidence.md` (mean 148406.26 responses/s). |
| T2 | DONE | Token-aware receive loop and start path | `run_udp_server_main` observes a pinned `cancelled()` future (biased) and returns `Result`; the pure `admit_received` decision maps receive errors and stream end to errors. `Server::start_with_cancellation` binds, logs, spawns only the loop, registers, and rolls back. Tests: token stop with socket release, registration with a working health check, registration rollback, and four admission decisions. |
| T3 | DONE | Single-task owner and UDP component migration | `OwnedTask` in `manager.rs`; `udp_tracker::start_job` uses the token-aware path and builds the owner before returning; child token in `start_udp_instance`. Tests: Cancelled, error, and panic outcomes; drop-before-run socket release (mutation-proven); bootstrap cancellation through `JobManager`. |
| T4 | DONE | Review first passing vertical slice | See the 2026-09-26 15:30 progress-log entry. No material correction needed. |
| T5 | DONE | Legacy launcher adapter | `run_with_graceful_shutdown` owns the loop through an abort-on-drop guard and cancels it on halt, dropped halt sender, or global OS signal, then joins it. Both regression tests were written first and failed against the old launcher (panic `Failed to install stop signal`; socket still bound after launcher abort). Legacy contract and environment tests pass unchanged. |
| T6 | DONE | Update shutdown documentation | UDP rows of `task-inventory.md`, plus the HTTP/REST rows left stale after SI-11/SI-12; notes in the SI-15, SI-17, and SI-19 drafts. |
| T7 | DONE | Executable-boundary and performance verification | M1-M3 in `manual-verification-evidence.md`; M4 in `performance-evidence.md`. |
| T8 | DONE | Acceptance and completion review | Independent Task Reviewer report in `agent-review-reports.md`; findings fixed; `implementation-retrospective.md` created. AC12 awaits maintainer confirmation. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Baseline `performance-evidence.md`, plus an ownership-map correction only if material | Commit the baseline before any code change; otherwise record a no-change decision in the progress log. |
| T2 | Token-aware loop and start path with package tests | Commit after focused validation and test-design review. |
| T3 | Owner type, component migration, child token, and their tests | Commit after focused validation and test-design review. |
| T4 | Material ownership correction only | Commit substantive corrections separately. |
| T5 | Legacy adapter with regression tests | Separate commit, so it can be reverted independently of T2-T3. |
| T6 | Documentation updates | One `docs(...)` commit. |
| T7-T8 | Evidence and completion review | Commit with final evidence. |

For each test-producing increment, use the `write-unit-test` skill: write
temporary Arrange/Act/Assert prose, refactor until the test body expresses it,
remove redundant prose, and record the review in the progress log. Each test
exposes its one causal initial-state difference, keeps the production Act
visible, and states its expected result independently. Stop for maintainer
review after the final test increment before final verification and the PR.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2342 created and issue number added to this spec
- [x] Spec-only PR #2343 merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-21 UTC - GitHub Copilot - Initial SI-14 draft.
- 2026-09-25 08:30 UTC - GitHub Copilot - Rewrote the draft against `develop`
  after SI-13 (PR #2336) merged. Mapped the current UDP path, found three
  ownership gaps (legacy drop-path panic, launcher-abort detach, silent receive
  failure), and recorded decisions D1-D5 with pros and cons. D2-D4 were chosen
  by the maintainer; D1 and D5 are recommendations awaiting draft approval.
- 2026-09-26 UTC - GitHub Copilot - Added D6 after the maintainer asked about
  UDP performance: record a throughput baseline on `develop` at the start of
  implementation (T1) and measure again after it (T7) on the same machine, both
  in issue-local `performance-evidence.md` (M4, AC12). Added constraint 7 (one
  pinned `cancelled()` future).
- 2026-09-26 08:30 UTC - GitHub Copilot - Maintainer approved the draft,
  including recommendations D1 and D5. Created GitHub issue #2342, linked it as
  a sub-issue of EPIC #1488, and promoted this spec to its numbered open-issue
  folder. Next step: spec-only PR before implementation.
- 2026-09-26 12:24 UTC - GitHub Copilot - Spec-only PR #2343 merged (Copilot
  review: no findings). Created the implementation branch from the merge
  result. T1: the ownership map needs no change. Recorded the UDP throughput
  baseline before any code change: five 30-second `aquatic_udp_load_test` runs
  against a release build of `0f1dcd28`, mean 148406.26 responses/s, spread
  142150.63-155327.06. See `performance-evidence.md`.
- 2026-09-26 12:57 UTC - GitHub Copilot - T2 and T3 committed; T4 design
  review of the first passing token-aware slice:
  - Ownership: `JobManager` owns the named component; the component owns the
    receive loop through `OwnedTask`, built before the future is returned; the
    loop owns the receiver, socket `Arc`, and `ActiveRequests`. Processors still
    hold socket `Arc` clones and are aborted, not joined, when the loop drops
    `ActiveRequests`, so the socket closes once the runtime drops those aborted
    tasks. That is prompt but not awaited; joining processors is SI-15.
  - Drop paths: component drop aborts the loop (mutation-proven test);
    registration failure cancels, aborts, and joins before returning.
  - Outcomes: the loop returns `Ok(())` only after cancellation, so the
    component derives `Cancelled` from the loop result, not from the token;
    errors and panics fail the component. UDP has no `Completed` outcome.
  - Deadlines: no new awaited step; stop is bounded by the `JobManager`
    deadline plus owner-drop abort.
  - Startup logs: the same messages now come from one `log_listener_startup`
    helper shared by both paths (split out to satisfy the cognitive-complexity
    lint); confirm in M1.
  - `NestedServerTask` and `HaltSignal` now have no production consumer; only
    their own tests use them. Note for SI-19.
  - Prose-first test review: package tests own token stop, registration, and
    rollback; the pure admission tests own the D3 error mapping; component tests
    own outcome mapping and drop safety; the application test owns `JobManager`
    token propagation. Each test states its causal state, keeps the production
    Act visible, and asserts an independent expected result.
- 2026-09-26 13:26 UTC - GitHub Copilot - Maintainer approved the tested
  slice. T5 legacy adapter and T6 documentation committed. T7:
  - M1/M2: two direct tracker-binary runs each served a real UDP announce,
    received `SIGTERM`, logged cooperative cancellation for
    `udp_instance_0_127.0.0.1:16969`, exited `0`, and the second run rebound the
    socket immediately.
  - M3: legacy tests pass; the standalone example served an announce and
    released its port but panicked on Ctrl-C with
    `FailedToStartOrStopServer("Normal")`. The identical panic occurs on the
    baseline `develop` build (double OS-signal listener), so it is pre-existing
    and recorded in the SI-17 draft.
  - M4: five after-implementation runs averaged 157254.83 responses/s against
    the 148406.26 baseline; every after run is above every baseline run. No
    regression; the literal "within the spread" wording is flagged for the
    maintainer because the result is higher, not lower.
  - Timeline: M1-M3 ran 13:11-13:14 UTC and the after-implementation load
    test 13:15-13:20 UTC, so they did not overlap.
- 2026-09-26 14:45 UTC - GitHub Copilot - T8: the independent Task Reviewer
  (`agent-review-reports.md`) found no code blockers; AC1-AC11 pass and AC12
  needs maintainer confirmation. Findings addressed:
  - The token-aware start path now keeps the receive loop in its drop-safe
    guard across the registration await; token-aware traces, the `states.rs`
    test-ownership doc, unbounded test awaits, and a test name were fixed
    ("fix(udp-server): [#2342] address SI-14 completion-review findings").
  - D5 now lists every observed legacy behavior change.
  - Progress-log times above were corrected from commit and log-file
    timestamps; some had been written in local time (UTC+1).
  - Prose-first review of the T5 legacy launcher tests: the
    `RunningLegacyLauncher` fixture owns only incidental mechanics (bind,
    spawn, wait for the startup notification); each test states its one
    causal difference in the Act (drop the halt sender, or abort the launcher
    while keeping the halt sender alive), and asserts independently specified
    results (clean `Ok(())` exit, socket bindable within a bounded wait). Both
    were red before the adapter.
  - Created `implementation-retrospective.md`.

## Acceptance Criteria

- [x] AC1: Each configured UDP instance receives a child `CancellationToken`
      derived from the `JobManager` root token.
- [x] AC2: Token cancellation stops the receive loop cooperatively, with no
      OS-signal subscription and no `Halted` channel on the token-aware path.
- [x] AC3: The component joins its receive loop before reporting its named
      `udp_instance_<index>_<address>` outcome; the owner exists before the
      component future is returned.
- [x] AC4: A receive-loop error or unexpected stream end yields a failed
      component outcome, not `Completed`.
- [x] AC5: Registration failure and dropping the component (polled or not)
      release the UDP socket without detached tasks.
- [x] AC6: Legacy `Server::start` / `Server::stop` consumers compile and keep
      their normal stop behavior; dropping a legacy `Server<Running>` no longer
      panics or detaches the receive loop.
- [x] AC7: `ActiveRequests` capacity and request-processor abort behavior are
      unchanged; existing request-buffer tests pass without modification.
- [x] AC8: `udp_ban_cleanup` remains a separate manager-owned, token-cancellable
      job.
- [x] AC9: Startup log target and messages are unchanged.
- [x] AC10: Manual SIGTERM verification records the `main()` signal event, the
      UDP component's cooperative stop, a clean exit, and immediate UDP rebind.
- [x] AC11: `linter all` exits with code `0` and relevant tests pass.
- [ ] AC12: UDP throughput measured after implementation is within the
      baseline's run-to-run spread on the same machine and settings; any larger
      drop is investigated and explained before closing. (No regression; the
      result is above the spread. Awaiting maintainer confirmation.)

## Verification Plan

### Automatic Checks

- Focused `torrust-tracker-udp-server` unit and contract tests, including
  `request_buffer` tests and the legacy environment start/stop tests.
- Focused `torrust-tracker` `udp_tracker` component and `app` bootstrap tests.
- `linter all`.
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`.
- Pre-push checks.

### Manual Verification Scenarios

Follow the EPIC executable-boundary protocol: signal the direct tracker-binary
PID, bound the wait, capture the exit status and logs, and prove the binding is
released. Record everything in issue-local `manual-verification-evidence.md`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Token-driven UDP shutdown | Start `target/debug/torrust-tracker` with one UDP binding, confirm readiness with a `tracker_client udp announce`, send `SIGTERM` to the binary PID, and capture bounded exit and logs. | `main()` cancels the root token; the UDP component stops cooperatively and reports `Cancelled`; exit `0`. | DONE | `manual-verification-evidence.md` M1 |
| M2 | UDP listener release | Restart the same configuration immediately after M1 and announce again. | The UDP socket rebinds immediately and serves the announce. | DONE | `manual-verification-evidence.md` M2 |
| M3 | Legacy UDP lifecycle | Run the standalone UDP example or environment start/stop path. | It starts, serves, and stops as before. | DONE | `manual-verification-evidence.md` M3: behaves as before, including a pre-existing Ctrl-C panic now tracked in SI-17. |
| M4 | UDP throughput before and after | Follow the E2E UDP load test in `docs/benchmarking.md`: release build, `share/default/config/tracker.udp.benchmarking.toml`, `aquatic_udp_load_test` with one saved config. Run it at least three times on the baseline `develop` commit (T1) and three times on the implementation branch (T7) on the same machine. Record the machine, commits, toolchain, load-test config, and each run's responses per second. | The implementation's mean is within the baseline's min-max spread. | DONE | `performance-evidence.md`: mean 157254.83 vs baseline 148406.26; above the spread, no regression. |

### Disposable Verification Scripts

None planned. If a repeatable direct-PID harness becomes necessary, place it in
this issue directory and record why a maintained Rust test cannot cover it.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | `app::tests::it_should_cancel_the_udp_tracker_component_through_the_job_manager`; `start_udp_instance` passes `new_cancellation_token().child_token()`. |
| AC2 | DONE | `server::tests::token_aware_start::it_should_stop_the_receive_loop_and_release_the_socket_when_its_cancellation_token_is_cancelled`; only the legacy adapter references `global_shutdown_signal` in `packages/udp-server/src`. |
| AC3 | DONE | `udp_tracker` outcome tests and `it_should_release_the_socket_when_the_component_is_dropped_before_it_runs` (fails when the owner is built inside the future). |
| AC4 | DONE | `receive_loop_admission` tests (error, `Interrupted`, stream end) and `it_should_fail_when_the_receive_loop_stops_with_an_error`. |
| AC5 | DONE | `token_aware_start::it_should_release_the_socket_when_registration_fails` and the drop-before-run test. |
| AC6 | DONE | Unchanged contract and environment tests; `it_should_stop_without_panicking_and_release_the_socket_when_the_legacy_halt_sender_is_dropped` and `it_should_release_the_socket_when_the_legacy_launcher_task_is_aborted` (both red before T5). |
| AC7 | DONE | No diff to `request_buffer.rs` or `processor.rs` against `develop`; their tests pass unmodified. |
| AC8 | DONE | `start_udp_ban_cleanup_job` unchanged in `src/app.rs`. |
| AC9 | DONE | M1 logs show the same `Starting on`, `Started on: udp://`, and `Started UDP tracker` lines. |
| AC10 | DONE | `manual-verification-evidence.md` M1-M2. |
| AC11 | DONE | `linter all` in every pre-commit run and pre-push checks on nightly Rust `1.100.0-nightly`. |
| AC12 | TODO | `performance-evidence.md`: no regression (after mean 157254.83 vs baseline 148406.26); result above the spread, awaiting maintainer confirmation. |

## Dependencies

- SI-2 (#2234) and the SI-13 (#2324) owner pattern are merged.
- SI-1 is required for manual SIGTERM verification.
- SI-15 builds on the D1 cooperative exit point.

## Rollback

Revert the T3 commit to put the bootstrap back on the legacy path; the T2
token-aware API remains unused. Revert the T5 commit independently to restore
the original legacy launcher. HTTP, REST, and health-check components are
unaffected.

## Risks and Trade-offs

- **Hot-loop change (D1)**: an extra `select!` branch per receive. Mitigation:
  `biased;` token branch, one pinned `cancelled()` future (constraint 7), a
  per-instance child token, and the D6 before/after load test (AC12).
- **Legacy behavior change (D5)**: failure and stop-timing modes changed as
  listed under D5. Mitigation: regression tests where the path is
  deterministic, and the changes are recorded for SI-17/SI-19.
- **Overlap with `simplify-udp-server-main-loop`**: both touch
  `run_udp_server_main`. Mitigation: keep SI-14 changes to the stop condition
  and return type; note the dependency in that draft.

## Implementation Completion Review

Before closing, the independent Task Reviewer verifies the acceptance criteria,
child-task ownership on every drop path, registration rollback, legacy adapter
parity, test design, manual evidence, and validation, recording its report in
`agent-review-reports.md`. Create `implementation-retrospective.md` for
reusable lessons or material deviations; otherwise record why none is needed in
the progress log.
