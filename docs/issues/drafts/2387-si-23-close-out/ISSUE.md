---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-23-close-out/ISSUE.md
branch: "{issue-number}-2387-si-23-close-out-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:53"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Close out: recapture, compare, and review the acceptance criteria

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-23 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: None (it verifies all), in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: All other sub-issues, or the maintainer's decision to detach a blocked one.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

The EPIC is shown to be complete: a new capture is compared with the baseline line by line, every difference is in the list of changes, and the EPIC's acceptance criteria are checked with evidence.

## Background

Each sub-issue verifies its own lines. Nobody has read the whole log of the final tracker as an operator would, and nobody has checked that the list of changes is complete.

## Scope

### In Scope

- Recapture the three baseline runs (`info`, `debug`, statistics disabled) with the same steps, and store them beside the baseline in the EPIC folder.
- A comparison document: for each baseline line, the line it became or the reason it is gone; for each new line, the sub-issue that added it.
- Check every difference against "Changes in 4.0.0" in `docs/logging.md` and add what is missing.
- Read the final log as an operator: is anything still incoherent or missing? Report findings to the maintainer as candidate issues; do not fix them here.
- Remove the "not yet implemented" marks from the catalogue in `docs/logging.md`.
- Search the repository for programs that read a fact from log message text (AC15).
- Fill the EPIC's acceptance verification table, record the completion review, and do the EPIC close-out as the repository's skills describe.
- Check the status of the two external issues and of SI-21 and SI-22, and record them.

### Out of Scope

- Fixing findings. Each becomes an issue.
- The project-wide `CHANGELOG`.

## Architectural Decisions

- Related decisions: None (it verifies all) of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
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
| T1 | TODO | Recapture | Three capture documents |
| T2 | TODO | Compare | `before-after-comparison.md` in the EPIC folder |
| T3 | TODO | Complete the list of changes | Every difference is listed |
| T4 | TODO | Operator read-through | Findings reported, or a note that there is none |
| T5 | TODO | Acceptance review | The EPIC's table filled with evidence |
| T6 | TODO | Close-out | EPIC closed and its folder moved, when the maintainer agrees |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Recapture | Commit after focused validation and review. |
| T2 | Compare | Commit after focused validation and review. |
| T3 | Complete the list of changes | Commit after focused validation and review. |
| T4 | Operator read-through | Commit after focused validation and review. |
| T5 | Acceptance review | Commit after focused validation and review. |
| T6 | Close-out | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-23-close-out/ISSUE.md`
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

## Acceptance Criteria

- [ ] AC1: The three recaptures are stored and the comparison covers every baseline line.
- [ ] AC2: "Changes in 4.0.0" has every operator-visible difference.
- [ ] AC3: Each EPIC acceptance criterion is `DONE` with evidence, or its gap is reported to the maintainer.
- [ ] AC4: Findings of the read-through are filed as issues or recorded as not accepted.
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
| M1 | Operator read-through | Start the final release tracker with every service, send one request to each service, stop it, and read the log from the first line to the last | A coherent story with no line that needs the source code to be understood; findings recorded | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Upgrade as an operator | Take a filter written for the baseline log (for example the E2E parser's old expression) and convert it using only "Changes in 4.0.0" | The document is enough to convert it | TODO | `manual-verification-evidence.md` section V2 |

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

## Risks and Trade-offs

- The comparison is large (about 250 lines for each capture). Mitigation: it is grouped by line category, as the baseline's parts are.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (None (it verifies all))
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
