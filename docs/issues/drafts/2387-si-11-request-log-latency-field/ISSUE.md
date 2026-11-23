---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-11-request-log-latency-field/ISSUE.md
branch: "{issue-number}-2387-si-11-request-log-latency-field-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:16"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - packages/axum-http-server/src/v1/routes.rs
    - packages/axum-rest-api-server/src/routes.rs
    - packages/axum-health-check-api-server/src/server.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Rename `latency_ms` to `latency_in_ms` in request logs

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-11 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D6, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-10.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

The latency field of request logs follows the duration rule: `latency_in_ms`.

## Background

The HTTP tracker, the REST API and the health check API log `latency_ms` on their response lines (`packages/axum-http-server/src/v1/routes.rs`, `packages/axum-rest-api-server/src/routes.rs`, `packages/axum-health-check-api-server/src/server.rs`). It is the only duration field outside lifecycle lines that breaks rule 2. It is a request log, so it is separate from the lifecycle work and can be reviewed alone.

## Scope

### In Scope

- Rename the field at the nine sites in the three packages.
- Check that the value is a number in milliseconds and not `Debug` of `Duration`; fix it if not.
- Update tests and documents that show the field.
- Check the UDP tracker's request lines for a latency or duration field and apply the same rule.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- Levels and messages of request logs (#1166).
- Any other request log field.

## Architectural Decisions

- Related decisions: D6 of the EPIC. After SI-1 is merged, the logging ADR it creates governs this work.
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
| T1 | TODO | Search every use | Code, tests, documents |
| T2 | TODO | Rename in the three packages | One commit for each package |
| T3 | TODO | Recapture a request | The response line shows `latency_in_ms=<number>` |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Search every use | Commit after focused validation and review. |
| T2 | Rename in the three packages | Commit after focused validation and review. |
| T3 | Recapture a request | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-11-request-log-latency-field/ISSUE.md`
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

- [ ] AC1: No `latency_ms` in a capture or in the code.
- [ ] AC2: `latency_in_ms` is a number.
- [ ] AC3: "Changes in 4.0.0" lists the rename.
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
| M1 | Read request lines | Start the release tracker; send one HTTP announce, one REST API request and one health check request | Each response line has `latency_in_ms=<number>` | TODO | `manual-verification-evidence.md` section V1 |

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

- Dashboards of operators read `latency_ms`. Mitigation: listed in "Changes in 4.0.0".

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D6)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
