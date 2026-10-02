---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p1
epic: 2410
github-issue: 2414
spec-path: docs/issues/open/2414-2410-si-22-2-drain-listener-queues-on-shutdown/ISSUE.md
branch: "2410-process-queued-events-before-listeners-stop-spec"
related-pr: null
last-updated-utc: "2026-10-02 17:28"
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

Parent: [EPIC #2410 - Process queued events before event listeners stop](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md)
(`epic` is set to the sub-EPIC's issue number once it exists),
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

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T3a | TODO | Red unit test, persistent listener | Pre-cancelled token and a ready event: red output recorded. |
| T3b | TODO | `TestEnv` owns its listeners | Token and handles kept; `stop()` cancels and joins; remove only waits that shutdown makes redundant, preserving tests of live persistence. |
| T3c | TODO | Red package test (AC11) | N completions, already-cancelled listener: red output recorded. |
| T4a | TODO | Receiver trait and drain helper | `try_recv`, `len`, `TryRecvError`; all implementations, mocks, and scripted receivers compile together; helper unit tests (empty, closed with buffered events, timeout with leftovers, lag). |
| T4b | TODO | Measure drain rate (`release`) | Time to drain the persistent listener's queue; default timeout chosen and recorded. |
| T4c | TODO | Persistent listener drain | T3a and T3c green; paused-time timeout test. Design-review checkpoint. |
| T5 | TODO | Remaining six listener components | Same helper; characterization tests replaced. |

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
| M1 | Drain logs | Run the tracker, send `completed` announces, SIGTERM | Each listener logs its drain count; a cut-short drain logs a warning | TODO |
| M2 | Recheck window (a) | `reproduce-lost-completions.sh control` with the T0 parameters | No loss | TODO |
| M3 | Race (informative) | `reproduce-lost-completions.sh race` with the T0 parameters | Record counts and lag/failure/abort logs; scheduling can change the deficit, and this run does not isolate window (b) | TODO |

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
| AC1-AC5 | TODO | Focused tests, red/green evidence, and release measurement |
| AC6 | TODO | Like-for-like control recheck |
| AC7 | TODO | Slow-handler test; shared-deadline proof linked from sub-issue 4 |

## Implementation Completion Review

Follow the sub-EPIC's [Shared Delivery Gates](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#shared-delivery-gates),
including local manual evidence, acceptance re-review, retrospective assessment,
and independent Task Reviewer report.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted
- [x] Spec reviewed and approved by the maintainer
- [x] GitHub issue created and linked to the sub-EPIC
- [ ] Design-review checkpoint after T4 recorded
- [ ] Implementation completed and acceptance criteria reviewed

### Progress Log

- 2026-10-02 11:03 UTC - GitHub Copilot - Drafted from SI-22 T3-T5 (D17).
- 2026-10-02 13:30 UTC - GitHub Copilot - Added AC7 (sub-EPIC AC12): the drain timeout does not interrupt a handler; shared-deadline abort remains an exception.
