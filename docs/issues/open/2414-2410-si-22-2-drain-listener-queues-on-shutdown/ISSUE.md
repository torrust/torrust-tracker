---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p1
epic: 2410
github-issue: 2414
spec-path: docs/issues/open/2414-2410-si-22-2-drain-listener-queues-on-shutdown/ISSUE.md
branch: "2414-drain-listener-queues-on-shutdown"
related-pr: null
last-updated-utc: "2026-10-20 15:47"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - packages/events/src/receiver.rs
    - packages/events/src/broadcaster.rs
    - packages/tracker-core/src/statistics/event/listener.rs
    - packages/tracker-core/tests/common/test_env.rs
    - packages/swarm-coordination-registry/src/statistics/event/listener.rs
    - packages/http-core/src/statistics/event/listener.rs
    - packages/udp-core/src/statistics/event/listener.rs
    - packages/udp-server/src/statistics/event/listener.rs
    - packages/udp-server/src/banning/event/listener.rs
---

<!-- skill-link: create-issue -->

# Issue #2414 - Drain Listener Queues on Shutdown

Parent: [EPIC #2410 - Process queued events before event listeners stop](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md),
under EPIC #1488.

> Sub-issue 2 of 4 (SI-22 tasks T3 to T5). Closes loss window (a), where the
> control reproduction isolated backlog loss; the race did not isolate each window.

## Goal

When an event listener is cancelled, it processes the events already in its
queue until the queue is empty or its drain timeout expires, logs how many it
processed, and warns with the number left when the timeout cuts the drain
short (D1).

## Bug in Plain Terms

Each listener checks for "stop" before checking for new events. When the
tracker shuts down, a listener with a queue of events stops at once and the
queue is thrown away. The reproduction lost about 92% of answered
`completed` announces, consistent with this backlog loss but without exact
per-window attribution (sub-EPIC
[manual-verification-evidence.md](../2410-1488-si-22-process-queued-events-before-listeners-stop/manual-verification-evidence.md), V1).

## Scope

### In Scope

- `try_recv`, `len`, and `TryRecvError` on the `Receiver` trait, and a shared
  drain helper in `packages/events` (D13).
- The drain in all seven listener components, with the timeout injected
  through each listener's constructor and a default (D9).
- Choosing the default from a `release`-build measurement of the persistent
  listener (D18, option c).
- Replacing the seven `it_should_prioritize_*` characterization tests.
- The package-level integration test in `packages/tracker-core/tests/`
  (AC11), including a `TestEnv` that owns and stops its listeners.

### Out of Scope

- The stop order between producers and listeners (window b, sub-issue 4).
- Configuration of the drain timeouts (SI-20, D9).
- Faster persistence (batching draft, D18).

## Design and Ownership Review

See the sub-EPIC's
[Design and Ownership Review](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#design-and-ownership-review).
In this issue: each listener owns its drain. Its timeout stops admission of
another event, not an in-progress handler. The shared shutdown deadline can
abort that handler; its outcome may then be unknown. A design-review
checkpoint follows T4.

## Bug-Fix Process

Follows `.github/skills/dev/debugging/fix-bug/SKILL.md`. Analysis and
real-artifact reproduction are done (sub-EPIC T0, V1). This issue does the
red tests (T3), the fix (T4, T5), and the recheck of the `control` scenario
(M2), which isolates window (a).

## Regression Test Strategy

Sub-EPIC
[Regression Test Strategy](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#regression-test-strategy):
unit tests at each dispatch function (pre-cancelled token plus ready events),
a paused-time timeout test, and option 1, the package-level integration test
with a deterministic red.

## Implementation Plan

Detailed steps: sub-EPIC
[Implementation Steps](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#implementation-steps),
items 4 (T3), 5 (T4), and 6 (T5).

**Revalidate before starting.** This plan was written on 2026-10-02. Two pull
requests merged into `develop` on 2026-10-16 changed the code it touches
(checked with `git diff --stat` against each merge's first parent):

- PR #2545 (#2536, hide the statistics repositories' read lock) changed the
  listeners in `packages/{http-core,swarm-coordination-registry,udp-core,udp-server}/src/statistics/event/listener.rs`,
  the event handlers in the same `statistics/event/` folders, the banning
  handler `packages/udp-server/src/banning/event/handler.rs` and
  `packages/udp-server/src/banning/mod.rs`, and
  `packages/tracker-core/tests/common/test_env.rs` (T3b). It also changed the
  statistics repositories and metrics of those packages, including
  `packages/tracker-core/src/statistics/repository.rs`, which the persistent
  listener's handler (T3a, T4c) also updates (read from source, not run).
- PR #2552 (#2244, UDP metric aggregates) changed the banning handler again,
  plus other statistics files outside `statistics/event/` (metrics,
  repositories, and the activity metrics updater).

Recheck the listener loops, handler signatures, the `it_should_prioritize_*`
tests, and `TestEnv` against the current code before T3.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T3a | DONE | Red unit test, persistent listener | Pre-cancelled token and a ready event: red output recorded (V3). The test is committed with the T4c fix, never failing. |
| T3b | DONE | `TestEnv` owns its listeners | Token and handles kept; `stop()` cancels and joins; remove only waits that shutdown makes redundant, preserving tests of live persistence. Waits are kept for now: `stop()` discards the queue until T4c. Evidence: V1 in `manual-verification-evidence.md`. |
| T3c | DONE | Red package test (AC11) | N completions, already-cancelled listener: red output recorded (V4), with a control that passes without cancellation. The test is committed with the T4c fix, never failing. |
| T4a | DONE | Receiver trait and drain helper | `try_recv`, `len`, `TryRecvError`; all implementations, mocks, and scripted receivers compile together; helper unit tests (empty, closed with buffered events, timeout with leftovers, lag). |
| T4b | DONE | Measure drain rate (`release`) | Time to drain the persistent listener's queue; default timeout chosen and recorded. |
| T4c | DONE | Persistent listener drain | T3a and T3c green; paused-time timeout test. Design-review checkpoint. |
| T5 | DONE | Remaining six listener components | Same helper; characterization tests replaced. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T3b | `TestEnv` listener ownership | Commit after the package tests pass. |
| T3a, T3c, T4a-T4c | Red tests, helper, first listener | Commit after the design review and maintainer review; the red tests must not be committed failing. |
| T5 | One listener package per commit | Commit after focused validation. |

Use the `write-unit-test` skill and the prose-first Arrange-Act-Assert design
review for every test increment. Sign every commit with GPG.

T3b defines normal stop, startup-failure, and drop cleanup for every owned
listener handle; it must not detach tasks on test panic. All readiness and
join waits use an absolute test deadline, not retries that reset the budget.
Review this ownership slice when it first passes, as well as after T4.
The slow-handler test must drive time explicitly; an always-ready receiver
alone will not advance a paused clock. Test cooperative scheduling as well.
Test timeout before observing lag: `len()` includes overwritten unread events
until the receiver advances. Logs must not label that value as recoverable
queue size or double-count it with already-observed lag (D13).

## Acceptance Criteria

- [ ] AC1 (sub-EPIC AC1): a listener cancelled with events already queued
  processes all of them when the finite backlog fits its budgets and no
  lag or handler failure occurs, then returns `Cancelled` (all seven components).
- [ ] AC2 (sub-EPIC AC3): each listener logs the number of events processed
      after cancellation, and warns with the listener name and leftover count
      when its drain timeout expires.
- [ ] AC3 (sub-EPIC AC4): the drain timeout default is chosen from a
  recorded release measurement. It bounds admission between events, not
  handler completion; shared-deadline escalation and producer-budget
  starvation remain explicit exceptions, covered in sub-issue 4.
- [ ] AC4 (sub-EPIC AC6): the seven `it_should_prioritize_*` tests are
      replaced by tests of the new policy.
- [ ] AC5 (sub-EPIC AC11): the package-level integration test is proven red
      against the cancel-first loop and green after the fix.
- [ ] AC6: the `control` scenario of `reproduce-lost-completions.sh`, rerun
      with the T0 parameters, loses no completion.
- [ ] AC7 (sub-EPIC AC12): the drain timeout is checked only between events.
  A test shows a slow handler crossing this timeout still completes before
  the drain stops. Shared-deadline abort remains an explicit exception,
  verified with supervisor coverage in sub-issue 4; a received event is
  not included in the receiver's leftover count.
- [ ] `linter all` exits with code `0`; relevant tests pass.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`
- Pre-push checks before opening the PR

### Manual Verification Scenarios

| ID | Scenario | Steps | Expected Result | Status |
| --- | --- | --- | --- | --- |
| M1 | Drain logs | Run the tracker, send `completed` announces, SIGTERM | Each listener logs its drain count; a cut-short drain logs a warning | DONE (V10) |
| M2 | Recheck window (a) | `reproduce-lost-completions.sh control` with the T0 parameters | No loss | DONE (V10) |
| M3 | Race (informative) | `reproduce-lost-completions.sh race` with the T0 parameters | Record counts and lag/failure/abort logs; scheduling can change the deficit, and this run does not isolate window (b) | DONE (V10) |

Record evidence in an issue-local `manual-verification-evidence.md`.

## Risks and Trade-offs

- Shutdown takes longer. Admission stops at the drain timeout, but the current
  handler may continue until it completes or the supervisor aborts it.
- A large persistent backlog can still be cut short (D18). Queue leftovers can
  be counted; interrupted database outcomes may be unknown.

## Architectural Decisions

Apply the sub-EPIC's draft ADR and D1, D9, D13, D18. Do not promote the draft
to a root ADR here; sub-issue 4 owns that step.

### Acceptance Verification

| AC ID | Status | Evidence |
| --- | --- | --- |
| AC1 | DONE | All seven listeners: a queued event is handled and the listener returns `Cancelled` (V7, V9; one test per listener, mutation-checked). Real binary: V10. The finite-backlog qualification stands: a backlog larger than the drain clears is cut short (V6, V10). |
| AC2 | DONE | The `log_drain_report!` fields on the real binary for all seven listeners, and the warning with the listener name and the counts when the drain timed out (V8, V10). Counts are events, not completions (record entry 28). Verified on the real binary; no unit test captures the log lines (read from source, not run: the macro). |
| AC3 | DONE, with a qualification | The `release` measurement is recorded (V6). It showed that a full buffer cannot drain within the 10 s deadline, so the default (5 s, `DEFAULT_DRAIN_TIMEOUT`) is a policy choice bounded by the measurement, not derived from it; the maintainer agreed (record entry 28, 2026-10-20). The timeout bounds admission, not handler completion. |
| AC4 | DONE | The seven `it_should_prioritize_*` tests are replaced: the persistent one in V3 and V7, the other six in V9. |
| AC5 | DONE | `it_should_persist_every_completed_download_queued_before_the_persistent_listener_is_cancelled`: red on the cancel-first loop (V4), green after the fix (V7), with a passing uncancelled control (V4). |
| AC6 | DONE | V10: the control run with the T0 parameters persists 500 of 500 (T0: 467). |
| AC7 | DONE, helper level | `it_should_finish_a_slow_handler_that_crosses_the_timeout_before_stopping` and its mutation (V5). The helper owns this rule for every listener; no per-listener slow-handler test exists. The shared-deadline abort is sub-issue 4's. |

## Implementation Completion Review

Follow the sub-EPIC's [Shared Delivery Gates](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#shared-delivery-gates),
including local manual evidence, acceptance re-review, retrospective assessment,
and independent Task Reviewer report.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted
- [x] Spec reviewed and approved by the maintainer
- [x] GitHub issue created and linked to the sub-EPIC
- [x] Design-review checkpoint after T4 recorded
- [ ] Implementation completed and acceptance criteria reviewed

### Progress Log

- 2026-10-02 11:03 UTC - GitHub Copilot - Drafted from SI-22 T3-T5 (D17).
- 2026-10-02 13:30 UTC - GitHub Copilot - Added AC7 (sub-EPIC AC12): the drain timeout does not interrupt a handler; shared-deadline abort remains an exception.
- 2026-10-19 17:08 UTC - GitHub Copilot (Claude Opus 5.5) - Refreshed before implementation: `branch:` names the implementation branch `2414-drain-listener-queues-on-shutdown`; removed the obsolete note about setting `epic`; the Implementation Plan now asks to revalidate the code that PRs #2545 and #2552 changed after this spec was written (file lists from `git diff --stat` of each merge).
- 2026-10-19 19:09 UTC - GitHub Copilot (Claude Opus 5.5) - The revalidation note now also names the statistics repositories and metrics that PR #2545 changed, including tracker-core's repository, which the persistent listener's handler updates (nit N1 of the #2413 Task Reviewer re-review).
- 2026-10-20 12:02 UTC - GitHub Copilot (Claude Sonnet 5.5) - Started after PR #2569 merged (branch `2414-drain-listener-queues-on-shutdown` cut from `develop`). Revalidated against the code (read from source, not run): `Receiver` still has only `recv` and a `mockall` `automock`; tokio is 1.53.3; the seven `it_should_prioritize_*` tests are where the sub-EPIC lists them; `TestEnv` created a local token, dropped the three listener handles, and slept 100 ms. T3b done on the maintainer-approved design: `TestEnv` owns a token with a `DropGuard` and the named handles, and `stop()` cancels and joins them under one absolute deadline. New test `it_should_release_every_listener_receiver_when_the_environment_is_stopped`, shown to fail when `stop()` does not cancel and when it does not join (V1). The 100 ms sleep and the persistence polling stay until `stop()` can drain (T4c). At the T3b review the maintainer asked for a drop-path test: `it_should_cancel_every_listener_when_the_environment_is_dropped_without_being_stopped`, shown to fail when the drop guard is on a different token (V2). A listener that panics is not simulated (read from source, not run).
- 2026-10-20 12:08 UTC - GitHub Copilot (Claude Sonnet 5.5) - T3a and T3c red evidence recorded in `manual-verification-evidence.md` (V3, V4), with both tests embedded verbatim because they are committed only with the T4c fix. T3a replaces `it_should_prioritize_a_pre_cancelled_token_over_a_ready_persistent_event` and fails with `left: None, right: Some(1)`; T3c (N = 3, the maintainer left the value to the proposed default) fails with `left: None, right: Some(3)`, and a temporary uncancelled control persists all three, so the red is caused by cancellation. A package run on the working tree failed exactly those two tests. The shared helper `expect_persisted_download_metrics_to_be` gained assertion messages. Both tests wait in the working tree for T4a to T4c.
- 2026-10-20 12:45 UTC - GitHub Copilot (Claude Sonnet 5.5) - T4a to T4c implemented in the working tree; the T4 design-review checkpoint is pending, so T4c stays `IN_PROGRESS` and nothing of T3a, T3c, or T4 is committed (Commit Points). T4a: `Receiver::try_recv`, `len`, a default `is_empty`, `TryRecvError`, and `packages/events/src/drain.rs` with its tests, all mutation-checked (V5). T4b: the release measurement (V6). T4c: the persistent listener drains on cancellation with an injected timeout and logs through `log_drain_report!`; T3a and T3c are green; an interim real-binary check (V8): the T0 control run now persists 500 of 500 (T0: 467), the drain logged `outcome="emptied" processed=72`, and a larger race run logged the timeout warning after 5 s. Proposed design changes are in the sub-EPIC's Design Discussion Record, entry 28.
- 2026-10-20 12:51 UTC - GitHub Copilot (Claude Sonnet 5.5) - T4 design review held: the maintainer agreed to a 5 s default drain timeout, the `log_drain_report!` macro, the "events" wording, `Receiver::is_empty`, keeping the package `TestEnv` polling, and three commits (T4a; the T4c fix with T3a and T3c; the docs). `adr-draft.md`, D9, D13, and the Chosen Design are updated; the sub-EPIC's record entry 28 is marked agreed. T4c is `DONE`; the checkpoint is ticked. T5 is next.
- 2026-10-20 12:57 UTC - GitHub Copilot (Claude Sonnet 5.5) - T3a, T3c, and T4 committed as two code commits, `feat(events): [#2414] add try_recv, len, TryRecvError, and the shared listener drain helper` and `fix(tracker-core): [#2414] drain the persistent listener's queue when it is cancelled`, each tested alone before its commit, plus this documentation commit. The unpushed first commit was amended once to correct its test count (seven drain tests, not eight). T5 (the other six listeners, one package per commit) is next.
- 2026-10-20 15:14 UTC - GitHub Copilot (Claude Sonnet 5.5) - T5 started after the maintainer approved its plan and test design: one commit per package, each with two tests replacing the `it_should_prioritize_*` test. First done: the `tracker-core` in-memory listener (V9), mutation-checked, workspace 3,156 passed, `linter all` 0. Stashes are shared by every worktree of the repository, so none is used from here on.
- 2026-10-20 15:19 UTC - GitHub Copilot (Claude Sonnet 5.5) - T5: the `swarm-coordination-registry` listener drains on cancellation (V9), mutation-checked; workspace 3,157 passed, `linter all` 0.
- 2026-10-20 15:26 UTC - GitHub Copilot (Claude Sonnet 5.5) - T5: the `http-core` listener drains on cancellation (V9), mutation-checked, with a third test that a drained event is subject to the metrics policy. `axum-http-server` gained a direct dependency on `torrust-tracker-events` for the default timeout (the listener's callers there pass it); workspace 3,159 passed, `linter all` 0.
- 2026-10-20 15:32 UTC - GitHub Copilot (Claude Sonnet 5.5) - T5: the `udp-core` listener drains on cancellation (V9), mutation-checked, with the metrics-policy test; workspace 3,161 passed, `linter all` 0. Only the `udp-server` listeners remain.
- 2026-10-20 15:39 UTC - GitHub Copilot (Claude Sonnet 5.5) - T5 done: the `udp-server` statistics and banning listeners drain on cancellation (V9), mutation-checked; workspace 3,164 passed, `linter all` 0. All seven listeners drain through the shared helper, one package per commit. Remaining: the final real-binary recheck (M1 to M3) after this last production change, the acceptance-criteria evidence, the Task Reviewer report, the retrospective, and the pre-push checks.
- 2026-10-20 15:47 UTC - GitHub Copilot (Claude Sonnet 5.5) - M1 to M3 rechecked on the real binary after the last production change (V10): the control run with the T0 parameters persists 500 of 500 (T0: 467), both race runs persist everything they answered (T0: 790 and 775 lost), all seven listeners log their drain with the same fields, and a cut-short drain logs the warning after 5 s. The Verification Plan rows are `DONE` and the Acceptance Verification table has its evidence. The acceptance criteria are not ticked: the reviewer does that. Two slips in the recheck (a UDP client argument and ports) are recorded in V10. Next: the Task Reviewer report, the retrospective, and the pre-push checks.
