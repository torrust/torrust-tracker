---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-11-read-service-urls-from-structured-fields/ISSUE.md
branch: "{issue-number}-2387-si-11-read-service-urls-from-structured-fields-spec"
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
    - tests/lifecycle/shutdown_policy.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Read service URLs from structured fields and remove `Starting on:` and `Started on:`

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-11 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D4, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-10.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

No program in the repository reads a service URL from message text. The E2E runner and the native tracker fixture read `service_binding` from the `Started` line of the wanted service, and the lines `Starting on:` and `Started on:` are gone.

## Background

`src/console/ci/e2e/logs_parser.rs` extracts URLs with a regular expression on `Started on: udp://…` and `Started on: http://…`. `tests/common/native_tracker/health.rs` takes what follows `Started on: http://` on a `HEALTH CHECK API` line. `tests/lifecycle/shutdown_policy.rs` asserts that `Started on:` is absent in one case. After SI-10 the same facts are fields of the `Started` line, selected by `service_role`.

## Scope

### In Scope

- The E2E parser reads `service_binding=` from lines whose message is `Started <service>` and whose `service_role=` is the wanted one. Its unit tests use lines copied from a real capture.
- The native tracker fixture does the same for the health check API.
- Tests that assert the presence or absence of `Started on:` assert the new line instead.
- Delete `Starting on:` and `Started on:` from the four services, and the constant `STARTED_ON`.
- Update documents that show these lines.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- Parsing JSON. #2766 changes the format later; this issue reads fields from the text format.
- Other log readers in the lifecycle tests (SI-9, SI-14 and SI-18 update the lines they read).

## Architectural Decisions

- Related decisions: D4 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
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
| T1 | TODO | Move the E2E parser | Parser and unit tests; the E2E runner passes |
| T2 | TODO | Move the native tracker fixture | Fixture; the lifecycle tests pass |
| T3 | TODO | Update the absence assertions | `shutdown_policy.rs` |
| T4 | TODO | Delete the old lines and the constant | Four services |
| T5 | TODO | Search for remaining readers | A search for `Started on` finds only closed specifications, review records and the list of changes |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Move the E2E parser | Commit after focused validation and review. |
| T2 | Move the native tracker fixture | Commit after focused validation and review. |
| T3 | Update the absence assertions | Commit after focused validation and review. |
| T4 | Delete the old lines and the constant | Commit after focused validation and review. |
| T5 | Search for remaining readers | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-11-read-service-urls-from-structured-fields/ISSUE.md`
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

- [ ] AC1: The E2E runner finds every service URL and passes.
- [ ] AC2: The native tracker tests pass, including with port `0`.
- [ ] AC3: No capture contains `Starting on:` or `Started on:`.
- [ ] AC4: No code in the repository matches a service URL in message text.
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
| M1 | Run the E2E runner | Run the E2E tests runner as documented | It passes | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Read a full start | Start the release tracker | No `Starting on:` and no `Started on:` line | TODO | `manual-verification-evidence.md` section V2 |

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

- A reader outside the repository parses `Started on:` (deployment scripts, the demo). Mitigation: listed in "Changes in 4.0.0" with the replacement.
- The parser depends on the order of fields in the text format. Mitigation: it matches each field by name, not by position.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D4)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
