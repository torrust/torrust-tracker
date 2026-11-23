---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-6-rename-log-targets-to-underscore-style/ISSUE.md
branch: "{issue-number}-2387-si-6-rename-log-targets-to-underscore-style-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:53"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - src/console/ci/e2e/logs_parser.rs
    - tests/common/native_tracker/health.rs
    - packages/udp-core/src/lib.rs
    - packages/axum-rest-api-server/src/lib.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Rename log targets to underscore style and `API` to `REST_API`

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-6 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D2, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-5.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

All targets use one style, uppercase with underscores, and the REST API's target says which API it is.

## Background

Targets with a space (`UDP TRACKER`, `HTTP TRACKER`, `HEALTH CHECK API`, `UDP CLIENT`) sit beside targets with an underscore (`TRACKER_CORE`). A name with a space needs quotes in a filter. `API` does not say which of the two APIs it is. The E2E runner and the native tracker fixture select lines by these names.

## Scope

### In Scope

- Rename the values of the target constants: `UDP TRACKER` to `UDP_TRACKER`, `HTTP TRACKER` to `HTTP_TRACKER`, `HEALTH CHECK API` to `HEALTH_CHECK_API`, `UDP CLIENT` to `UDP_CLIENT`, `API` to `REST_API`.
- Rename the constant `API_LOG_TARGET` to match its new value, and any other constant whose name no longer matches.
- Update the readers that match the old names: `src/console/ci/e2e/logs_parser.rs`, `tests/common/native_tracker/health.rs`, and any test or document found by a search.
- Update documents and examples that show the old names (`docs/`, package READMEs, `docs/containers.md`).
- Check `EVENTS`, the swarm coordination registry target and any other existing target against the style and rename the ones that differ.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- `METRICS`, which belongs to the crate `torrust-metrics` and already follows the style.
- Adding targets (SI-5).
- Messages, fields and levels.

## Architectural Decisions

- Related decisions: D2 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
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
| T1 | TODO | Search every use of the old names | List of code, tests and documents, in the task evidence |
| T2 | TODO | Rename the tracker service targets and their readers | UDP, HTTP and health check targets, with the E2E parser and the fixture, in one commit so CI stays green |
| T3 | TODO | Rename `API` to `REST_API` | Constant, value and readers |
| T4 | TODO | Rename `UDP CLIENT` and any other target that differs | Constant and value |
| T5 | TODO | Update documents | No old name left outside closed specifications and review records |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Search every use of the old names | Commit after focused validation and review. |
| T2 | Rename the tracker service targets and their readers | Commit after focused validation and review. |
| T3 | Rename `API` to `REST_API` | Commit after focused validation and review. |
| T4 | Rename `UDP CLIENT` and any other target that differs | Commit after focused validation and review. |
| T5 | Update documents | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-6-rename-log-targets-to-underscore-style/ISSUE.md`
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

- [ ] AC1: No target in a capture contains a space.
- [ ] AC2: The REST API logs under `REST_API`.
- [ ] AC3: The E2E runner and the native tracker tests pass.
- [ ] AC4: A search for the old names finds them only in closed specifications, review records and the list of changes.
- [ ] AC5: "Changes in 4.0.0" lists each rename.
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
| M1 | Read a full run | Start and stop the release tracker; list the distinct targets | Every target is uppercase with underscores | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Run the E2E runner | Run the E2E tests runner as documented | It finds the service URLs and passes | TODO | `manual-verification-evidence.md` section V2 |

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

- Operators filter on the old names. Mitigation: listed in "Changes in 4.0.0".
- `UDP CLIENT` is used by console tools outside the tracker process. Mitigation: T1 finds every user before the rename.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D2)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
