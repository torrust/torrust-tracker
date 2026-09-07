---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/request_buffer.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/request_buffer.rs
    - packages/udp-server/src/server/launcher.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/performance-evidence.md
    - docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md
---

# UDP Request Buffer Test Refactor Plan

Follow the shared [purpose, quality goals, and plan structure](README.md). This plan applies only
to `packages/udp-server/src/server/request_buffer.rs`.

## Phase 1 — Identify Problems

### Strengths to preserve

1. `ActiveRequests::force_push` documents the current normal-operation overload policy: retain up
   to 50 processor-task abort handles, remove finished work first, otherwise abort the oldest
   observed unfinished task to make space.
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

#### P2 — Finished-handle reclamation is unprotected

**Problem.** The full-buffer path must discard finished handles before aborting active work, but no
test distinguishes this priority.

**Why it matters.** Aborting live requests when completed handles already make space would violate
the documented normal-operation overload policy.

**Opportunity.** Fill the buffer with completed task handles plus one pending task. Insert another
pending handle and assert the existing pending task was not aborted while completed handles were
removed.

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

### R2 — Cover finished-handle reclamation before eviction

- **Status:** TODO
- **Priority:** High impact / medium effort
- **Addresses:** P2, P5
- **Change:** Add a deterministic full-buffer test with completed handles and one pending handle;
  insert a new pending handle and assert the existing pending task remains active.
- **Guardrails:** Synchronize completed and pending task state explicitly. Do not use sleeps,
  retries, or assumptions about a task's scheduling order beyond the test's own synchronization.
- **Done when:** the test proves finished work is reclaimed before a live task is aborted.

### R3 — Cover active-task eviction at capacity

- **Status:** TODO
- **Priority:** High impact / medium effort
- **Addresses:** P3, P5
- **Change:** Add a deterministic full-buffer test in which every tracked task remains pending;
  insert one more pending task, assert `force_push` returns `true`, and observe cancellation of the
  oldest selected task.
- **Guardrails:** Assert only the normal-operation eviction contract. Do not establish a task
  drain, deadline, join, shutdown metric, or graceful-shutdown policy.
- **Done when:** the test demonstrates exactly one required capacity eviction and keeps the new
  task's lifecycle cleanup explicit.

### R4 — Cover drop cleanup for active work

- **Status:** TODO
- **Priority:** Medium impact / medium effort
- **Addresses:** P4, P5
- **Change:** Add a deterministic test that drops a buffer containing a completed and a pending
  task handle, then observes pending-task cancellation.
- **Guardrails:** Do not use this test to define server shutdown behavior. `ActiveRequests` is a
  normal-operation capacity buffer; shutdown task policy belongs to SI-15.
- **Done when:** the test proves unfinished retained work is aborted by buffer drop without timing
  dependence.

### R5 — Assess finished incoming-task behavior

- **Status:** TODO
- **Priority:** Low impact / medium effort
- **Addresses:** P5
- **Change:** After R1–R4, decide whether a task that completes before reinsertion has a stable,
  independently stated behavior worth asserting.
- **Guardrails:** Record a no-change decision if the behavior is scheduler-dependent or has no
  observable package contract. Do not create a test merely to cover the `new_task.is_finished()`
  branch.
- **Done when:** the plan records either one behavior-focused test or a justified deferral.

## Progress Tracking

### Plan Checklist

- [x] Phase 1 findings reviewed against current code and issue coverage evidence.
- [x] Phase 2 refactorings ordered by impact and effort.
- [x] Maintainer approved implementation of R1.
- [x] R1 implemented, reviewed, validated, and committed.
- [ ] Maintainer approved implementation of R2.
- [ ] R2 implemented, reviewed, validated, and committed.
- [ ] Maintainer approved implementation of R3.
- [ ] R3 implemented, reviewed, validated, and committed.
- [ ] Maintainer approved implementation of R4.
- [ ] R4 implemented, reviewed, validated, and committed.
- [ ] R5 assessment completed and decision recorded.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

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

### Validation Evidence

| Increment          | Status | Evidence                                                    |
| ------------------ | ------ | ----------------------------------------------------------- |
| Plan documentation | TODO   | Run Markdown and spelling checks after plan review changes. |
| R1                 | DONE   | `cargo fmt --all -- --check`, focused request-buffer test, and `git diff --check` passed. |
| R2                 | TODO   | Focused request-buffer test, formatting, and diff checks.   |
| R3                 | TODO   | Focused request-buffer test, formatting, and diff checks.   |
| R4                 | TODO   | Focused request-buffer test, formatting, and diff checks.   |
| R5                 | TODO   | Test or documented no-change decision.                      |

## Non-Goals

- Do not change the fixed capacity, ring-buffer implementation, or production control flow merely
  to expose test internals.
- Do not make a production hot-path change without first recording the required baseline in
  [performance-evidence.md](../performance-evidence.md).
- Do not define active-request draining, deadlines, joins, outcomes, or shutdown metrics; SI-15
  owns that policy.
- Do not test `Launcher` event publication here; this plan protects only the buffer's own contract.
- Do not add sleeps, polling loops, unbounded awaits, or log assertions.

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
