---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1586
spec-path: docs/issues/open/1586-evaluate-job-manager-join-set/ISSUE.md
branch: 1586-evaluate-job-manager-join-set
related-pr: null
last-updated-utc: 2026-09-07
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

## Historical Context

The original #1586 proposal was intentionally deferred while EPIC #1488 and
its shutdown architecture were reviewed in [draft PR #1993](https://github.com/torrust/torrust-tracker/pull/1993).
The EPIC subsequently adopted the supervised cancellation-tree architecture
and added #1586 at roadmap sequence 2. The historic flat specification was
superseded by this folder-style specification after this re-scope.

## Relationship to the Selected Architecture

The supervised cancellation tree is now selected. `JobManager` owns direct,
named top-level component tasks and their root `CancellationToken`; components
own their nested tasks. `JoinSet` is therefore a candidate implementation for
only the supervisor's direct task set. It must not flatten component-owned
children into `JobManager` or undermine component lifecycle boundaries.

The existing SI-6 draft proposed concurrent outcomes while preserving a
`Vec<Job>` of already-spawned handles. That misses #1586's central design
constraint and is superseded by this issue.

## Implementation Decisions

### Direct supervisor registration API

Adopt `JoinSet` for `JobManager`'s direct top-level component ownership. Add a
`spawn(name, future)` registration API and migrate all current `push` and
`push_opt` callers. The manager must spawn the supplied component future
directly into its `JoinSet`; it must not accept an already-spawned
`JoinHandle`.

This deliberately removes the current two-step pattern:

```rust
let handle = tokio::spawn(component());
job_manager.push("component", handle);
```

It is replaced by direct registration:

```rust
job_manager.spawn("component", component());
```

Retaining `push(name, JoinHandle)` would require a second wrapper task solely
to await the already-spawned task before it could enter `JoinSet`. Aborting
that wrapper could detach the real component, violating the supervisor's
ownership and escalation requirements.

### Compatibility boundary: pre-spawned periodic jobs

The periodic-job token migrations proposed by SI-4/SI-5 are explicitly out of
scope for this issue. Their existing starter and Ctrl-C behavior remains
unchanged. Their already-spawned handles remain registered, joined, and
escalated by `JobManager` through a narrow compatibility registry, rather than
through `JoinSet`. `JoinSet` cannot adopt a pre-spawned `JoinHandle` without
the forbidden wrapper task. This exception is limited to torrent cleanup,
activity metrics, and UDP ban cleanup; direct `JoinSet` ownership applies to
component runners that can return the explicit result contract below. These
transitional legacy jobs share the same process-wide deadline as direct
components and are expected to leave the registry when SI-4/SI-5 migrate their
periodic-job cancellation and startup APIs.

### Explicit component result contract

Every direct component runner registered by `src/app.rs` returns
`ComponentResult`: `Completed`, `Cancelled`, or a `ComponentError` with failure
context. `JobManager` maps those component-reported outcomes directly, while
task panics and deadline-triggered aborts remain supervisor-owned outcomes.
In particular, it must not derive `Cancelled` merely because the shared
`CancellationToken` was cancelled.

### Deadline escalation outcome

When the single process-wide deadline expires, `JobManager` must deliberately
abort every remaining direct top-level component, join them, and record each
as the named `Aborted` outcome. `Aborted` is abnormal shutdown and must remain
distinct from a component that completed cooperatively or independently
panicked. It supplies the structured evidence required for the non-zero
process result defined by Q3 and implemented later by SI-20.

This policy applies only to direct component tasks owned by `JobManager`.
Each component remains responsible for joining or deliberately aborting its
own nested children before it completes.

## Acceptance Criteria

- [x] Re-evaluate `JoinSet` against the selected cancellation-tree architecture
      and record whether it is adopted or rejected with rationale.
- [x] Adopt a `spawn(name, future)` registration API for direct component
      runners so they enter `JoinSet` without wrapper tasks. Existing periodic
      job starters remain outside this issue's scope.
- [x] Job/component names remain available for completed, failed, panicked,
      timed-out, cancelled, and deliberately aborted outcomes.
- [x] Supervisor waiting observes components concurrently under the configured
      process-wide deadline; it is not a sequential per-job timeout loop.
- [x] Direct components still own and join or deliberately abort their nested
      tasks; `JobManager` does not collect those child handles.
- [x] Tasks remaining after cooperative shutdown are deliberately aborted,
      joined, and reported as named `Aborted` outcomes; none are silently
      detached.
- [x] Focused deterministic tests cover completion order, panic/failure,
      deadline expiry, cancellation, and escalation behavior.
- [x] Server component runners use drop-safe cleanup so deadline-aborting an
      outer component cannot detach its nested server task or running future.
- [x] Affected launcher and registration APIs preserve existing asynchronous
      startup readiness guarantees and startup-failure behavior.
- [x] `linter all` passes.

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
5. Verify affected server launchers preserve their externally visible startup
   readiness and startup-failure behavior after ownership changes.
6. Exercise named graceful-completion, deadline-escalation, and panic-isolation
   outcomes through deterministic tests; operational OS-signal verification is
   covered by the lifecycle signal suite.
