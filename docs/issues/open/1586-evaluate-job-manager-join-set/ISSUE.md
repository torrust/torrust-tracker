---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1586
spec-path: docs/issues/open/1586-evaluate-job-manager-join-set/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/drafts/1488-si-6-align-grace-periods/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #1586 — Evaluate `JoinSet` for `JobManager`

> **EPIC position**: Roadmap sequence 2. This existing GitHub issue replaces
> the current SI-6 implementation direction; do not implement SI-6 separately.

## Goal

Evaluate and, if appropriate, replace `JobManager`'s manual `Vec<Job>` of
already-spawned `JoinHandle<()>` values with direct task ownership through
`tokio::task::JoinSet`. The result must support concurrent completion
observation, explicit cancellation/escalation policy, and named supervisor
outcomes without spawning wrapper tasks solely to await existing handles.

## GitHub Issue Scope

Issue #1586 requires this decision to be made after shutdown architecture is
settled. Its stated constraints are preserved here:

- tracked futures should be spawned directly into `JoinSet` or an explicitly
  justified alternative;
- task names remain available in completion, panic, timeout, and cancellation
  logs;
- tasks left after cooperative shutdown are not silently detached; and
- focused tests cover completion order, panic reporting, deadline expiry, and
  cancellation.

## Relationship to the Selected Architecture

The supervised cancellation tree is now selected. `JobManager` owns direct,
named top-level component tasks and their root `CancellationToken`; components
own their nested tasks. `JoinSet` is therefore a candidate implementation for
only the supervisor's direct task set. It must not flatten component-owned
children into `JobManager` or undermine component lifecycle boundaries.

The existing SI-6 draft proposed concurrent outcomes while preserving a
`Vec<Job>` of already-spawned handles. That misses #1586's central design
constraint and is superseded by this issue.

## Acceptance Criteria

- [ ] Re-evaluate `JoinSet` against the selected cancellation-tree architecture
      and record whether it is adopted or rejected with rationale.
- [ ] If adopted, direct top-level component futures are registered without
      spawning an additional wrapper solely to await an existing handle.
- [ ] Job/component names remain available for completed, failed, panicked,
      timed-out, cancelled, and deliberately aborted outcomes.
- [ ] Supervisor waiting observes components concurrently under the configured
      process-wide deadline; it is not a sequential per-job timeout loop.
- [ ] Components still own and join or deliberately abort their nested tasks;
      `JobManager` does not collect those child handles.
- [ ] Tasks remaining after cooperative shutdown follow an explicit escalation
      policy and are not silently detached.
- [ ] Focused deterministic tests cover completion order, panic/failure,
      deadline expiry, cancellation, and escalation behavior.
- [ ] `linter all` passes.

## Dependencies

- Q2 selected supervisor ownership and cancellation-tree boundaries.
- Q3/Q4 selected outcome and deadline policy.
- #1588's inventory is supporting evidence and must be revalidated before this
  issue closes, but it does not block this initial design evaluation.

## Rollback

If `JoinSet` is adopted, restore the prior `Vec<Job>` supervisor implementation
as one coherent revert. Do not retain partial wrapper-task adapters merely to
preserve an intermediate design. If the evaluation rejects `JoinSet`, close the
issue with its documented rationale and retain the explicit alternative.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Record the decision matrix or rationale for adopting/rejecting `JoinSet`.
2. Run focused deterministic supervisor tests for all required outcome paths.
3. Review the task registration path to confirm no task is spawned solely to
   await an already-spawned handle for supervisor registration.
4. Confirm component child handles are not added to the supervisor.
