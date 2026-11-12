---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 1488
github-issue: 2847
spec-path: docs/issues/open/2847-1488-si-28-verify-hung-component-abort/ISSUE.md
branch: "2847-verify-hung-component-abort"
related-pr: null
last-updated-utc: "2026-11-12 14:30"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
    - src/main.rs
    - docs/issues/closed/2697-1488-si-21-mark-health-unhealthy-during-shutdown/manual-verification-evidence.md
    - docs/issues/closed/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2847 - Verify the Hung-Component Abort at the Deadline on the Real Binary (SI-28)

> **EPIC position**: Roadmap row 25, sub-issue SI-28 of [EPIC #1488](../1488-overhaul-tracker-shutdown/ISSUE.md).
> A follow-up of [SI-21](../../closed/2697-1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md)
> (and SI-20). Implementation order 4 of 4, after SI-26. Priority p3. Evidence only: no production
> change is planned.

## Goal

Record executable-boundary evidence that a component which never finishes is aborted at the process
deadline `P`, that the tracker exits with code 1 within `P`, and that the log names the aborted
component. Today only unit tests cover it.

## Background

Checked on 2026-11-12 against `torrust/develop` at `388faf59a`.

- SI-20 gave the process one deadline `P` (`[application.shutdown].deadline_in_secs`, default 25 s);
  `JobManager::wait_for_all` aborts every component still running at `P` and `report_shutdown` maps
  that to exit code 1.
- The SI-21 evidence (section V6) records a failed component (a REST drain that timed out) and says
  the case "a hung component that never completes would be aborted at P together with the health
  server (unit test above); the validated budgets make that hard to provoke on the binary, and it was
  not run."
- The EPIC protocol requires issue-local executable-boundary verification for runtime-affecting
  shutdown work. This path (`Aborted` outcome, exit code 1, no delay past `P`) has none.

**Why it is hard.** Every component drains within a validated budget below `P` and the Axum drain
force-closes at its deadline, so an ordinary overrun ends as a *failed* component, not a hung one. A
hung component needs a task that ignores cancellation, which no production component does.

## Scope

### In Scope

- T1: a feasibility note choosing the least invasive way to run a never-finishing component under the
  real shutdown code. Candidates, in the order to try:
  1. an executable harness (an example or a dev-only test binary) that builds the real `JobManager`,
     `begin_shutdown` path and the reusable exit-code mapping (`shutdown_exit_code` and
     `incomplete_components` in `bootstrap::jobs::manager`; `report_shutdown` is private to the binary) with one component that
     never completes, then runs it under the same signalling protocol as the tracker;
  2. an external stall of a real component (for example holding a lock or suspending a dependency the
     component awaits) if one exists without changing production code;
  3. a test-only fault-injection point behind a Cargo feature that is off in every release profile.
- T2: run the chosen technique with `SIGTERM` and `SIGINT`, capture bounded exit status, the log lines
  (`aborted`, the component name, what it waited for) and timing against `P`, and record it in
  `manual-verification-evidence.md`.
- If no technique is acceptable, record a reasoned decision that the unit tests plus the harness are the
  strongest feasible evidence, and close the issue with that decision.

### Out of Scope

- Any change to the production shutdown code or defaults. A fault-injection feature (candidate 3) needs
  the maintainer's explicit approval and is the last resort.
- Changing the budget validation so that a hung component becomes easy to provoke.
- Re-verifying the failed-component path (already done in SI-21 V6).

## Dependencies

- SI-20 (#2668) and SI-21 (#2697) are done and merged.
- After SI-26, so the evidence covers the final supervisor and the migrated `udp_ban_cleanup`.
- Not required before the first publish (#2583) by itself, but #2583 is blocked on EPIC #1488 as a
  whole, so the EPIC close-out needs this issue done or closed with the T1 decision.

## Architectural Decisions

- Related ADRs: `docs/adrs/20261027145446_budget_shutdown_in_two_chained_stages_inside_one_process_deadline.md`
  (the deadline `P`), `docs/adrs/20261104120915_stop_the_health_check_api_last_to_report_not_ready_during_shutdown.md`.
- ADRs to create: none.

## Design and Ownership Review

Not applicable to production code (none changes). The harness owns its own process and must satisfy the
repository rules for child processes: an absolute deadline on every wait, cleanup of the child on every
path, and no port or file left behind.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Feasibility note: choose the technique, and the maintainer approves it if it adds a feature or a harness to the tree. | Decision in the spec. |
| T2 | TODO | Build the harness (if chosen). | Issue-local path, removal or retention owner stated. |
| T3 | TODO | Run with `SIGTERM` and `SIGINT`; record the evidence. | `manual-verification-evidence.md`. |
| T4 | TODO | Update the shutdown docs if they say the path is unverified. | Doc edit or none. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | None (decision in the spec) | Commit the spec edit after the decision. |
| T2 | Harness, if any | Commit after focused validation. |
| T3 | Evidence | Commit after the run. |
| T4 | Documentation | Commit last. |

Sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/`, moved to `docs/issues/open/`
- [x] Spec reviewed and approved by the maintainer (decisions of 2026-11-12, in chat)
- [x] GitHub issue created and issue number added to this spec (#2847)
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Technique chosen and approved (T1)
- [ ] Manual verification executed and recorded in `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-12 14:30 UTC - AI agent (Claude Sonnet 5.5) - Spec created from the SI-21 evidence gap on `torrust/develop` at `388faf59a`; recorded the maintainer's decision of 2026-11-12 to open it as SI-28, after SI-26.

## Acceptance Criteria

- [ ] The technique and its approval are recorded (T1), or the reasoned no-go decision is.
- [ ] Evidence shows a never-finishing component reported as aborted at `P`, exit code 1 within `P` plus
      a small margin, and the log naming the component, for `SIGTERM` and `SIGINT`.
- [ ] No production behavior or default changed, and any harness or feature is off in release builds.
- [ ] `linter all` passes.

## Verification Plan

### Automatic Checks

- `linter all`
- If a harness is added: `cargo check --workspace --all-targets` warning-free and its own tests

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Hung component, `SIGTERM` | Start the harness with `deadline_in_secs` small (for example 4), signal the actual PID. | Exit code 1 within `P` plus a small margin; the aborted component is named in the log. | TODO | `manual-verification-evidence.md` |
| M2 | Hung component, `SIGINT` | Same with `SIGINT`. | Same result. | TODO | `manual-verification-evidence.md` |
| M3 | Health server does not delay the exit | Include the stop-last health server in the harness. | The health server is released and the process still exits within `P`. | TODO | `manual-verification-evidence.md` |

### Disposable Verification Scripts

If T2 adds a script, the spec names its issue-local path, why an automatic test is not enough, and who
removes or keeps it (Rust unless a reason is given).

### Acceptance Verification

| AC | Status | Evidence |
| -- | ------ | -------- |
| Technique recorded or no-go decision | TODO | T1 |
| Abort evidence for both signals | TODO | M1, M2, M3 |
| No production change | TODO | Diff review |
| `linter all` passes | TODO | Automatic checks |

## Risks and Trade-offs

- The harness runs the real supervisor but not the real components, so it proves the deadline path, not
  that production components never hang. That is the claim to make, and the evidence says so.
- A fault-injection feature is the strongest evidence and the largest risk; it needs explicit approval.

## Implementation Completion Review

Before closing, review what the implementation discovered. If it found anything material, create
`implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this
directory; otherwise add a progress-log entry saying why none was needed.

## References

- Parent EPIC: #1488 (roadmap row 25)
- Related: #2697 (SI-21, evidence V6), #2668 (SI-20, the deadline), #2583 (first publish)

## Rollback

Evidence-only: remove the harness and the evidence file. Nothing in production changes.
