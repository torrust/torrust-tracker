---
schema-version: 1
doc-type: issue
issue-type: feature
status: draft
priority: p3
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-22-run-id-on-every-record/ISSUE.md
branch: "{issue-number}-2387-si-22-run-id-on-every-record-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:16"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md
    - packages/configuration/src/v3_0_0/logging.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Add `run_id` to every log record

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-22 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D10, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: #2766 (daemon JSON logs) and SI-18.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

Every log record carries the `run_id` of its run, so any single line can be attributed to a run and one filter returns a whole run.

## Background

SI-18 puts `run_id` on the start line. To find the lines of a run an operator still needs the time range after that line. Decision D10 chose to add the field to every record, in the formatter and not as a root span: a root span would print in every span list and is lost in tasks that do not inherit it. Issue #2766 replaces the formatter with JSON output, so this issue comes after it to avoid writing the change twice.

## Scope

### In Scope

- Add `run_id` to each record in the logging setup of `packages/configuration` as #2766 leaves it.
- Cover records from every task and thread, including ones outside any span.
- Decide in the specification pull request whether the text styles that remain after #2766, if any, also carry it.
- Tests of the field's presence in records from the main task and from a spawned task.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- Generating the `run_id` (SI-18).
- Propagating the `run_id` to clients or to other processes.

## Architectural Decisions

- Related decisions: D10 of the EPIC. After SI-1 is merged, the logging ADR it creates governs this work.
- ADRs to create: none expected. Related: the ADR that #2766 creates for the log format.

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
| T1 | TODO | Read the logging setup after #2766 | Where a per-record field is added |
| T2 | TODO | Add the field | With tests |
| T3 | TODO | Recapture | Every record of a run has the same `run_id` |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Read the logging setup after #2766 | Commit after focused validation and review. |
| T2 | Add the field | Commit after focused validation and review. |
| T3 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-22-run-id-on-every-record/ISSUE.md`
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

- [ ] AC1: Every record of a run has `run_id`, with the value of the start line.
- [ ] AC2: Records from spawned tasks have it.
- [ ] AC3: It is documented in `docs/logging.md`.
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
| M1 | One run | Start and stop the tracker; count the records without `run_id` | Zero | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Two runs in one file | Append the logs of two runs to one file and filter by one `run_id` | Exactly the records of that run | TODO | `manual-verification-evidence.md` section V2 |

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

## Risks and Trade-offs

- #2766 is delayed. Mitigation: the EPIC's close-out rule lets the maintainer detach this issue.
- The design of #2766 makes a per-record field hard. Mitigation: this need is written into the specification of #2766.

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
