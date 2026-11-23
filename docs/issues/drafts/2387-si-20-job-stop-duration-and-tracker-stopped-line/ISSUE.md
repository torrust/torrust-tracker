---
schema-version: 1
doc-type: issue
issue-type: feature
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-20-job-stop-duration-and-tracker-stopped-line/ISSUE.md
branch: "{issue-number}-2387-si-20-job-stop-duration-and-tracker-stopped-line-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:16"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - src/app.rs
    - src/bootstrap/jobs/manager.rs
    - tests/lifecycle/signals.rs
    - tests/lifecycle/shutdown_policy.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Log how long each job took to stop and one final `Tracker stopped` line

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-20 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D10, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-10, SI-14.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

Each job outcome line says how long the job took to stop, and the last line of a run says how the run ended: `APPLICATION: Tracker stopped outcome=graceful signal=SIGTERM jobs_completed=<n> jobs_failed=<n> jobs_aborted=<n> elapsed_in_ms=<n> exit_code=0`.

## Background

The job manager logs an outcome for each job, without a duration, so an operator who tunes the shutdown deadline cannot see which job is slow. The last line is `Torrust tracker successfully shutdown.` on success; on failure the facts are spread over several `WARN` lines. Nothing states the exit code.

## Scope

### In Scope

- `elapsed_in_ms` on every job outcome line (completed, cancelled, failed, panicked, aborted), measured from the moment that job was asked to stop. A job that ended before it was asked to stop logs what the specification pull request decides (for example the time since the shutdown began).
- The final line. On a clean stop it is `INFO` with `outcome=graceful`. Otherwise it is `ERROR` with `outcome=incomplete`, the counts, and the names of the jobs that failed or were aborted.
- `signal` is the signal that started the stop. A stop for another reason (a failed start that joins its jobs) uses a value the specification defines.
- `exit_code` is the code the process returns. The line is logged where that code is known.
- Replace `Torrust tracker successfully shutdown.` and `report_shutdown`'s messages with the final line; update `tests/lifecycle/signals.rs` and `tests/lifecycle/shutdown_policy.rs`.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- Changing shutdown behavior, ordering or deadlines.
- `run_id` on the final line unless SI-18 is merged, in which case it is added.

## Architectural Decisions

- Related decisions: D10 of the EPIC. After SI-1 is merged, the logging ADR it creates governs this work.
- ADRs to create: none.

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
| T1 | TODO | Record when each job is asked to stop | The job manager keeps the instant; unit tests |
| T2 | TODO | `elapsed_in_ms` on outcome lines | With tests |
| T3 | TODO | Final line on a clean stop | With tests; old success message removed |
| T4 | TODO | Final line on an incomplete stop | With tests for failed and aborted jobs |
| T5 | TODO | Recapture | A clean stop and an incomplete stop |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Record when each job is asked to stop | Commit after focused validation and review. |
| T2 | `elapsed_in_ms` on outcome lines | Commit after focused validation and review. |
| T3 | Final line on a clean stop | Commit after focused validation and review. |
| T4 | Final line on an incomplete stop | Commit after focused validation and review. |
| T5 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-20-job-stop-duration-and-tracker-stopped-line/ISSUE.md`
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

## Acceptance Criteria

- [ ] AC1: Every job outcome line has `elapsed_in_ms`.
- [ ] AC2: The last line of every run that reached the job manager is `Tracker stopped` with the fields above.
- [ ] AC3: `exit_code` equals the process exit code.
- [ ] AC4: On an incomplete stop the line is `ERROR` and names the jobs.
- [ ] AC5: "Changes in 4.0.0" lists the removed success message and the new line.
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
| M1 | Clean stop | Start the release tracker and send `SIGTERM`; compare `exit_code` with the shell's `$?` | `outcome=graceful exit_code=0`, and `$?` is `0` | TODO | `manual-verification-evidence.md` section V1 |
| M2 | `Ctrl-C` | Stop it with `Ctrl-C` | `signal` names the interrupt signal | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Incomplete stop | Use the lifecycle test scenario with a job that outlives the deadline | `ERROR … outcome=incomplete … exit_code=1` with the job's name | TODO | `manual-verification-evidence.md` section V3 |

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

- Tests and scripts wait for the old success message. Mitigation: updated here and listed in "Changes in 4.0.0".
- The final line is not logged when the process is killed. Mitigation: documented; that is what its absence means.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D10)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
