---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-14-lifecycle-log-levels/ISSUE.md
branch: "{issue-number}-2387-si-14-lifecycle-log-levels-spec"
related-pr: null
last-updated-utc: "2026-11-23 17:29"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - src/bootstrap/jobs/manager.rs
    - packages/axum-server/src/signals.rs
    - tests/lifecycle/signals.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Apply the level rule to job manager and shutdown lines

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-14 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries) and stops for the maintainer to review and approve it. Only after that approval it creates the GitHub issue and opens the specification pull request, before any code.

- Implements: D9, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-5.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

Lifecycle lines have the level the rule gives them: a failed job is an `ERROR`, the shutdown plan is `DEBUG`, and shutdown messages are plain sentences.

## Background

`Job panicked`, `Job reported a failure` and `Job was aborted after the shutdown deadline` are `WARN`, although the process exits `1` in these cases. The shutdown plan is logged as one `INFO` line for each dependency edge (`Job will be asked to stop after …`), which is how the stop is ordered and not a state change. The shared HTTP shutdown code logs `!! Shutting down … in 15s !!`.

## Scope

### In Scope

- `log_panicked`, `log_failed` and `log_aborted` in `src/bootstrap/jobs/manager.rs` log at `ERROR`. So does the line for the expired shutdown deadline.
- The shutdown plan lines log at `DEBUG`.
- `!! Shutting down … !!` becomes a plain sentence with the service identity as fields.
- Review every other lifecycle `WARN` and `INFO` line of the inventory against the level table and change the ones that differ; list each change in the specification pull request before changing it.
- Check that a line repeated for each job at `INFO` is a state change of that job and not a step.
- Update the tests that assert these lines (`tests/lifecycle/signals.rs`).
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- The private-mode `WARN` (SI-3).
- The statistics `WARN` (SI-1).
- Levels of request-handling lines (#1166).
- Durations in these lines (SI-9) and new fields (SI-18).

## Architectural Decisions

- Related decisions: D9 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
- ADRs to create: none; the rule is in the ADR of SI-2.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. If a decision of the EPIC does not hold in the code, stop, record the finding and ask the maintainer.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | List the lines whose level changes | From the inventory, for maintainer review in the specification pull request |
| T2 | TODO | Job outcome lines | Three lines to `ERROR`, with unit tests of the job manager |
| T3 | TODO | Shutdown plan lines | To `DEBUG` |
| T4 | TODO | Shutdown sentences | Plain sentences with fields |
| T5 | TODO | Other lines found in T1 | One commit for each component |
| T6 | TODO | Recapture | A clean run at `info` and a run with a failing job |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | List the lines whose level changes | Commit after focused validation and review. |
| T2 | Job outcome lines | Commit after focused validation and review. |
| T3 | Shutdown plan lines | Commit after focused validation and review. |
| T4 | Shutdown sentences | Commit after focused validation and review. |
| T5 | Other lines found in T1 | Commit after focused validation and review. |
| T6 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-14-lifecycle-log-levels/ISSUE.md`
- [ ] Draft completed by the sub-issue session and approved by the maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update. When an AI agent did the work, name its model after the agent.

- 2026-11-23 16:16 UTC - Claude Code (Fable 5.2) - Drafted with EPIC #2387 from the maintainer's decisions of 2026-11-21 and 2026-11-23. Not approved as a final specification; no GitHub issue created.
- 2026-11-23 16:53 UTC - Claude Code (Fable 5.2) - Renumbered the sub-issues by working order after review round 1 of PR 3010 (finding F4); this draft's number and its references to other sub-issues follow the new numbers.
- 2026-11-23 17:29 UTC - Claude Code (Fable 5.2) - Corrected after the CodeRabbit review of PR 3010 (audit findings F15 to F22): this draft now names the maintainer's approval before the GitHub issue is created; the drafts of SI-4, SI-8, SI-9, SI-13 and SI-23 have one more correction each.

## Acceptance Criteria

- [ ] AC1: A job that panics, fails or is aborted logs an `ERROR`.
- [ ] AC2: A clean run at `info` has no shutdown plan lines; at `debug` it has them.
- [ ] AC3: No lifecycle message contains `!!`.
- [ ] AC4: A clean run has no `WARN` and no `ERROR` apart from the ones the convention lists.
- [ ] AC5: "Changes in 4.0.0" lists each level change.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- Relevant tests for changed components, including tests that assert log output
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Clean run | Start and stop the release tracker at `info` | No `WARN`, no `ERROR`, no plan lines | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Failing stop | Run the existing lifecycle test scenario in which a job does not stop before the deadline, or reproduce it by hand with a short deadline | An `ERROR` for the aborted job; exit code `1` | TODO | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated tests pass. Record the actual commands, output and relevant tracker logs in `manual-verification-evidence.md`, created from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- Every recorded command result names the toolchain or runtime that produced it when one can affect behavior.
- The capture steps are the ones of the baseline: start the release binary, poll the health check, wait one second, send `SIGTERM`, strip ANSI codes.

### Disposable Verification Scripts

None planned. If one is needed, record its path, purpose and removal owner here before creating it.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |
| AC4 | TODO | {test/log/PR link} |
| AC5 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- Alerts on `ERROR` fire where `WARN` was logged. Mitigation: that is the intent; listed in "Changes in 4.0.0".

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D9)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
