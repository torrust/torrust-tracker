---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/request_buffer.rs
status: completed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/request_buffer.rs
    - packages/udp-server/src/server/launcher.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/performance-evidence.md
    - packages/udp-server/docs/adrs/20260907152707_keep_oldest_first_udp_request_eviction.md
    - docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md
---

# UDP Request Buffer Test Refactor Plan

Follow the shared [purpose, quality goals, and plan structure](README.md). This plan applies only
to `packages/udp-server/src/server/request_buffer.rs`.

## Phase 1 — Identify Problems

### Strengths to preserve

1. `ActiveRequests::force_push` has a documented normal-operation overload policy: retain up to
  50 processor-task abort handles, reclaim completed handles encountered before the first
  still-active task, otherwise abort that oldest active task to make space.
2. `Drop` explicitly aborts remaining unfinished processor tasks, avoiding detached work when the
   normal-operation buffer is released.
3. The implementation retains single-owner buffer invariants and does not use shared mutable
   state.

### Problems and opportunities

#### P1 — Normal capacity behavior has no direct contract

**Problem.** No test proves that inserting a pending task while capacity is available retains the
task and returns `false`.

**Why it matters.** `Launcher::run_udp_server_main` uses the return value to decide whether to
publish `UdpRequestAborted`. A regression could emit an abort fact without an eviction.

**Opportunity.** Create a pending task with a deterministic synchronization channel, insert its
abort handle, and assert no eviction occurred while preserving the production buffer behavior.

#### P2 — Oldest-first bounded eviction is unprotected

**Problem.** The full-buffer path has no test for its intentional oldest-first decision: it does
not scan newer completed handles before evicting the first oldest task that remains active after a
scheduler yield.

**Why it matters.** A future refactor could mistake this intentional performance trade-off for a
bug, introduce a slower full-buffer scan, or change the eviction/event result without review.

**Opportunity.** Fill the buffer with one oldest pending task followed by completed handles. Insert
a new pending task and assert that the oldest task is evicted and `force_push` reports the eviction.

#### P3 — Active-task eviction is unprotected

**Problem.** When all tracked handles remain active, `force_push` yields once and aborts the oldest
observed unfinished task. No test proves the eviction or its `true` result.

**Why it matters.** This is the buffer's material overload behavior and the only condition that
causes the launcher to publish an aborted-request fact.

**Opportunity.** Fill the buffer with pending tasks controlled by deterministic cancellation
observers, push one additional pending task, then verify the selected oldest handle was aborted
and the other tracked work remains active.

#### P4 — Drop cleanup is unprotected

**Problem.** `Drop::drop` aborts unfinished handles and skips finished handles, but no regression
test protects that distinction.

**Why it matters.** Leaving pending processor tasks alive after the normal-operation buffer drops
would leak work; aborting an already finished task is unnecessary but harmless.

**Opportunity.** Drop a buffer containing one confirmed completed task and one pending task, then
assert the pending task observes cancellation without relying on elapsed time.

#### P5 — Scheduler coupling must remain constrained

**Problem.** The implementation calls `tokio::task::yield_now()` before deciding an old task is
still unfinished.

**Why it matters.** Tests based on sleeps, polling, or task scheduling order would be flaky and
would make an implementation detail look like a shutdown contract.

**Opportunity.** Use channels and bounded awaits solely to establish task completion or abort
observation. Do not specify drain, deadline, join, or shutdown behavior.

## Phase 2 — Proposed Refactorings

Apply items in order. Complete one approved increment—including review, focused validation, and the
mapped commit point—before beginning the next item.

### R1 — Cover insertion while capacity is available

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Add one deterministic unit test that inserts a pending task handle into an empty
  `ActiveRequests` buffer and asserts `force_push` returns `false`.
- **Guardrails:** The task must remain pending until the test cleans it up. Do not inspect private
  ring-buffer internals or add production observation APIs. If a production change becomes
  necessary, stop this increment and follow the baseline policy in
  [performance-evidence.md](../performance-evidence.md) before changing the hot path.
- **Done when:** the test names the capacity-available causal state and proves no task was evicted.

### R2 — Assess and document oldest-first bounded eviction

- **Status:** DONE
- **Priority:** High impact / medium effort
- **Addresses:** P2, P5
- **Change:** Use the deterministic oldest-pending/later-completed scenario to assess the current
  behavior. Document its historical performance rationale in a package-local ADR and clarify the
  production comments. Defer a behavior test until the ADR and source wording receive maintainer
  review.
- **Guardrails:** Do not reinterpret the historic comment as a full-buffer reclamation guarantee.
  Do not change hot-path production behavior or add a benchmark for documentation-only work.
- **Decision:** The current oldest-first behavior is intentional. PR #921 documents the starvation
  concern and one-yield opportunity; PR #922 records that a refactor separating removal from
  cleaning all completed tasks regressed performance. The initial failing R2 test asserted the
  rejected full-scan alternative, not a production defect.
- **Done when:** the ADR and production comment clarify the policy, and the unsupported bug handoff
  and failing test evidence are removed.

### R3 — Cover active-task eviction at capacity

- **Status:** DONE
- **Priority:** High impact / medium effort
- **Addresses:** P3, P5
- **Change:** Add a deterministic full-buffer test in which every tracked task remains pending;
  insert one more pending task, assert `force_push` returns `true`, and observe cancellation of the
  oldest selected task. Use a file-local `FullBufferWithPendingTasks` scenario fixture so the
  Arrange section names the causal full-buffer state while the test retains the visible `force_push`
  Act and eviction assertion.
- **Guardrails:** Assert only the normal-operation eviction contract. Do not establish a task
  drain, deadline, join, shutdown metric, or graceful-shutdown policy. The fixture may create and
  clean up tasks, but it must not call `force_push`, decide the expected result, or hide the
  eviction assertion. Keep it specialized to this full-pending-buffer scenario; do not generalize
  it into a builder or shared test factory.
- **Done when:** the test demonstrates exactly one required capacity eviction, names the full
  pending-buffer state in Arrange, and keeps the Act and eviction assertion visible.

### R3a — Clarify full-buffer scenario construction

- **Status:** DONE
- **Priority:** Medium impact / low effort
- **Addresses:** R3 Arrange readability
- **Change:** Add a file-local `PendingTask::insert_into` helper that creates a pending task,
  inserts its abort handle into the scenario buffer, and returns the task for deterministic
  cleanup. Rename `new_task` to `incoming_task` because it represents the request arriving after
  capacity is exhausted.
- **Guardrails:** The helper owns only Arrange mechanics and must not invoke `force_push`, decide
  an expected result, or assert eviction behavior. Keep it private to this module; do not create a
  general builder or shared test factory.
- **Done when:** `FullBufferWithPendingTasks::new` visibly constructs the oldest task, the
  remaining 49 tasks, and the incoming task without duplicating buffer-insertion mechanics.

### R4 — Cover drop cleanup for active work

- **Status:** DONE
- **Priority:** Medium impact / medium effort
- **Addresses:** P4, P5
- **Change:** Add a deterministic test that drops a buffer containing a completed and a pending
  task handle, then observes pending-task cancellation. Keep the completed task inline because its
  only causal role is to establish mixed buffer state; use the local `PendingTask` helper for the
  pending task's controlled lifetime and cancellation assertion.
- **Guardrails:** Do not use this test to define server shutdown behavior. `ActiveRequests` is a
  normal-operation capacity buffer; shutdown task policy belongs to SI-15.
- **Done when:** the test proves unfinished retained work is aborted by buffer drop without timing
  dependence.

### R5 — Assess finished incoming-task behavior

- **Status:** DONE
- **Priority:** Low impact / medium effort
- **Addresses:** P5
- **Change:** After R1–R4, decide whether a task that completes before reinsertion has a stable,
  independently stated behavior worth asserting.
- **Guardrails:** Record a no-change decision if the behavior is scheduler-dependent or has no
  observable package contract. Do not create a test merely to cover the `new_task.is_finished()`
  branch.
- **Decision:** No test added. `Launcher::run_udp_server_main` checks `abort_handle.is_finished()`
  immediately after spawning a processor and does not call `force_push` for an already completed
  task. The `new_task.is_finished()` check is therefore a defensive race guard only for completion
  between that caller check and buffer admission. A direct test would need to control scheduler
  timing rather than prove an observable UDP-server contract.
- **Done when:** the plan records a justified no-change decision.

### R6 — Address Copilot review feedback on test mechanics

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** bounded cleanup waits, public-behavior setup, and capacity-independent full-buffer
  setup.
- **Change:** Assess the three Copilot review suggestions from draft PR #2174 before changing the
  completed test suite: (1) bound task joins so a cleanup regression fails rather than hangs;
  (2) use `force_push` where it can establish setup without obscuring the intended full-buffer
  state; and (3) derive the full-buffer fill count from the buffer capacity rather than hard-coding
  49 retained tasks. Apply the mandatory prose-first Arrange-Act-Assert comparison before any
  approved refactor.
- **Guardrails:** Preserve deterministic synchronization and the oldest-first eviction contract.
  Do not use polling or arbitrary sleeps as a timeout substitute. Keep the `force_push` Act and
  eviction assertion visible in the behavior test. Do not change hot-path production code, capacity,
  eviction policy, or shutdown semantics.
- **Decision:** Accept bounded task joins and capacity-derived setup. A timeout is an absolute
  failure bound for an awaited cleanup outcome, not a delay or polling mechanism. Derive the count
  of retained pending tasks from the buffer's actual capacity so the full-buffer scenario remains
  correct if that policy changes. Decline public-API-only setup: `force_push` is the behavior under
  test, so repeatedly calling it during Arrange would make the initial full-buffer state depend on
  the Act and obscure which task is oldest. Keep the private `rb.try_push` operation only inside the
  narrowly named setup helper, with a comment recording this reason.
- **Prose-first review:** The temporary Arrange prose was “a request buffer is full of controlled
  pending tasks, with a separately retained oldest task”; the final
  `FullBufferWithPendingTasks` constructor expresses this with an oldest task, a count derived from
  `rb.capacity()`, and retained pending tasks. The Act remains one visible `force_push` call. The
  temporary Assert prose was “the oldest task is aborted and all retained work is cleaned up”; the
  named assertions express it, while `TASK_COMPLETION_TIMEOUT` documents the irreducible
  test-process failure bound. The direct-insertion comment remains because it records why a
  superficially attractive public-API setup would incorrectly execute the Act during Arrange.
- **Done when:** each suggestion has either a reviewed test-only change or a documented no-action
  rationale, the affected tests remain readable/deterministic, and the PR threads have replies
  before resolution.

## Progress Tracking

### Plan Checklist

- [x] Phase 1 findings reviewed against current code and issue coverage evidence.
- [x] Phase 2 refactorings ordered by impact and effort.
- [x] Maintainer approved implementation of R1.
- [x] R1 implemented, reviewed, validated, and committed.
- [x] Maintainer approved implementation of R2.
- [x] R2 assessment, ADR, and source-comment clarification committed independently.
- [x] Maintainer approved implementation of R3.
- [x] R3 implemented, reviewed, validated, and committed.
- [x] Maintainer approved implementation of R3a.
- [x] R3a implemented, reviewed, validated, and committed.
- [x] Maintainer approved implementation of R4.
- [x] R4 implemented, reviewed, validated, and committed.
- [x] R5 assessment completed and decision recorded.
- [x] Maintainer approved R6.
- [x] R6 assessment and approved test refactor completed.
- [x] Maintainer reviewed all approved changes.
- [x] Plan completed and ready for final verification.

### Progress Log

- 2026-09-07 11:10 UTC - GitHub Copilot - Created this proposed plan from the
  `request_buffer.rs` implementation, the issue baseline evidence, and the SI-15 shutdown-policy
  boundary. No test or production change has been made.
- 2026-09-07 11:10 UTC - User/maintainer - Required performance protection for this hot-path
  component. Test-only changes retain focused validation; any approved production change must first
  establish the issue-local release-performance baseline and later record an equivalent after
  measurement.
- 2026-09-07 11:27 UTC - User/maintainer - Approved R1 as a test-only increment and required the
  accumulated issue planning, test-plan, and performance-evidence changes to be committed before
  test implementation begins.
- 2026-09-07 11:43 UTC - User/maintainer - Reviewed and approved R1. The increment adds one
  deterministic capacity-available contract without production changes, sleeps, polling, or a
  performance-baseline requirement.
- 2026-09-07 13:08 UTC - User/maintainer - Approved R2. The test must establish an oldest pending
  handle followed by completed handles, then prove whether finished work is reclaimed before active
  work is aborted. If the expected contract fails, stop before changing the hot-path implementation
  and follow the issue performance-baseline policy.
- 2026-09-07 15:12 UTC - GitHub Copilot - The initial R2 test assumed that all later completed
  handles must be reclaimed before an oldest pending task can be evicted. The test failed as
  expected against the implementation.
- 2026-09-07 15:27 UTC - GitHub Copilot - History review found this is intentional, not a defect:
  PR #921 documents the starvation/fairness rationale, and PR #922 records a rejected
  finished-handle-cleanup refactor due to a performance regression. A package-local ADR and source
  comment clarification record this decision. The unsupported bug handoff and failing test snapshot
  were removed.
- 2026-09-07 15:32 UTC - User/maintainer - Approved a deterministic R3 test for the documented
  oldest-first eviction policy. The test must fill the buffer with pending tasks, prove that one
  oldest task is aborted to admit the new task, and clean up every retained task explicitly.
- 2026-09-07 15:35 UTC - User/maintainer - Approved refactoring R3's complex Arrange section into
  a file-local `FullBufferWithPendingTasks` scenario fixture before completing the test increment.
  The fixture may own setup and cleanup mechanics only; the test retains the `force_push` Act and
  observable eviction assertion.
- 2026-09-07 16:05 UTC - User/maintainer - Requested a further simplification of
  `FullBufferWithPendingTasks::new`. Replaced duplicated channel/task mechanics with the file-local
  `PendingTask` helper; the scenario constructor now directly states construction of the oldest task,
  the remaining 49 pending tasks, and the incoming task.
- 2026-09-07 16:25 UTC - User/maintainer - Approved an R3a readability refinement: factor only
  repeated pending-task insertion into `PendingTask::insert_into` and rename the incoming request
  task. Commit the plan update before changing test code.
- 2026-09-07 16:30 UTC - User/maintainer - Reviewed and approved R3a. The local helper now owns
  only pending-task construction/insertion mechanics, while the scenario constructor and test Act
  remain readable and behavior-specific.
- 2026-09-07 16:37 UTC - User/maintainer - Approved R4. Use an inline completed task and the
  focused `PendingTask` helper to make the mixed completion state visible without adding another
  scenario fixture.
- 2026-09-07 16:42 UTC - User/maintainer - Reviewed and approved R4. The inline Arrange retains
  the causal mixed completion state without a premature builder or scenario fixture; the test-only
  increment leaves production behavior and the hot path unchanged.
- 2026-09-07 16:55 UTC - GitHub Copilot - Completed R5 assessment. No test is added: the launcher
  filters already finished processor handles before buffer admission, and the remaining
  `new_task.is_finished()` branch is a scheduler-dependent defensive race guard rather than a
  stable observable contract.
- 2026-09-07 17:03 UTC - User/maintainer - Reviewed and approved the completed request-buffer
  plan. R1–R4 add deterministic coverage, R2 documents the intentional performance trade-off, and
  R5 is a justified no-change decision. The next test plan may begin.
- 2026-09-09 - GitHub Copilot - Draft PR #2174 received three Copilot suggestions on the completed
  request-buffer tests: bound task-completion waits, avoid direct ring-buffer insertion where
  public behavior can express setup, and avoid hard-coded capacity assumptions. R6 records them as
  a new approval-gated test-maintainability assessment; no test change has been made.
- 2026-09-09 - User/maintainer - Approved R6 decisions: use bounded task joins and derive
  full-buffer setup from capacity; retain private direct insertion only as controlled Arrange
  mechanics because calling `force_push` during Arrange would test the Act before the test Act.
  Make this rationale discoverable in the test code for future reviewers.
- 2026-09-09 - GitHub Copilot - Completed R6. All task joins now have the one-second absolute
  cleanup bound; full-buffer construction derives retained-task count from `rb.capacity()`; and
  comments document why direct insertion is controlled Arrange mechanics rather than a call to the
  `force_push` Act. Focused request-buffer tests pass.

### Validation Evidence

| Increment          | Status | Evidence                                                    |
| ------------------ | ------ | ----------------------------------------------------------- |
| Plan documentation | TODO   | Run Markdown and spelling checks after plan review changes. |
| R1                 | DONE   | `cargo fmt --all -- --check`, focused request-buffer test, and `git diff --check` passed. |
| R2                 | DONE   | History review, package ADR, and production comments record the intentional oldest-first bounded policy; committed in `208f1d70`. |
| R3                 | DONE   | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server server::request_buffer::tests`, and `git diff --check` passed. The reviewed `FullBufferWithPendingTasks` scenario uses a local `PendingTask` helper for setup/cleanup mechanics. |
| R3a                | DONE   | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server server::request_buffer::tests`, and `git diff --check` passed. |
| R4                 | DONE   | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server server::request_buffer::tests`, and `git diff --check` passed. |
| R5                 | DONE   | No change: the launcher filters already finished handles; the remaining defensive race guard has no stable observable contract. |
| R6                 | DONE   | Bounded cleanup waits and capacity-derived setup implemented. Public-API-only setup declined because `force_push` is the visible Act; the controlled direct-insertion rationale is documented in code. |
| Plan completion    | DONE   | Maintainer reviewed all approved increments and decisions before the next file plan begins. |

## Non-Goals

- Do not change the fixed capacity, ring-buffer implementation, or production control flow merely
  to expose test internals.
- Do not make a production hot-path change without first recording the required baseline in
  [performance-evidence.md](../performance-evidence.md).
- Do not define active-request draining, deadlines, joins, outcomes, or shutdown metrics; SI-15
  owns that policy.
- Do not test `Launcher` event publication here; this plan protects only the buffer's own contract.
- Do not add sleeps, polling loops, unbounded awaits, or log assertions.
- Do not replace the oldest-first policy with a full-buffer scan without a separately approved
  production change, direct benchmark evidence, and ADR review.

## Validation Per Approved Increment

- Run the focused `ActiveRequests` unit tests.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- For an approved production change, complete the applicable release throughput and, when needed,
  focused microbenchmark evidence before committing the production increment.
- Review the changed test's causal state, visible Act, independently specified expected outcome,
  and explicit task cleanup before the next increment.

## Completion Criteria

- Each approved test is deterministic, behavior-focused, and limited to current normal-operation
  buffer semantics.
- Task completion and abort observation use explicit bounded synchronization rather than elapsed
  time.
- No test changes the shutdown boundary owned by SI-15.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
