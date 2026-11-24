---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-13-job-names-from-role-and-index/ISSUE.md
branch: "{issue-number}-2387-si-13-job-names-from-role-and-index-spec"
related-pr: null
last-updated-utc: "2026-11-23 17:30"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
    - tests/lifecycle/signals.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Name service jobs by role and index and add identity fields to job manager lines

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-13 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries) and stops for the maintainer to review and approve it. Only after that approval it creates the GitHub issue and opens the specification pull request, before any code.

- Implements: D3d, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-5, SI-10.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

A service job is named by its role and index (`udp_tracker_0`, `http_tracker_1`), and the job manager's lines about a service job carry `service_role` and `instance_index`, so they can be joined with the service's own lines.

## Background

Service jobs are named `udp_instance_0_0.0.0.0:6868` and `http_instance_1_0.0.0.0:7171`: the word `instance`, the index and the configured bind address. The address may be port `0`, so it can differ from the real listener, and the name says `instance` where the fields say `udp_tracker`. The job manager's lines have only `job=<name>`.

## Scope

### In Scope

- Name UDP and HTTP tracker jobs `<service_role>_<instance_index>`.
- Check the other job names (`http_api`, `health_check_api`, listeners, periodic jobs) for coherence with the service roles and propose renames in the specification pull request (for example `http_api` to `rest_api`).
- Let the job manager know the identity of a service job, and add `service_role` and `instance_index` to its lines about that job (reserved, started, asked to stop, outcome).
- Update the tests that match job names (`tests/lifecycle/signals.rs` and others).
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- Levels of job manager lines (SI-14).
- Durations (SI-9, SI-18).

## Architectural Decisions

- Related decisions: D3d of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
- ADRs to create: none. The specification states how the job manager receives the identity (for example optional metadata on `reserve`) without the manager depending on service packages.

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
| T1 | TODO | List the job names and their users | Names, tests and documents |
| T2 | TODO | Rename the service jobs | With tests |
| T3 | TODO | Give the job manager the identity | API change and its unit tests |
| T4 | TODO | Add the fields to the lines | With tests |
| T5 | TODO | Recapture | Job manager lines of service jobs show the two fields |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | List the job names and their users | Commit after focused validation and review. |
| T2 | Rename the service jobs | Commit after focused validation and review. |
| T3 | Give the job manager the identity | Commit after focused validation and review. |
| T4 | Add the fields to the lines | Commit after focused validation and review. |
| T5 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-13-job-names-from-role-and-index/ISSUE.md`
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

- [ ] AC1: No job name contains an address.
- [ ] AC2: Every job manager line about a service job has `service_role` and `instance_index`.
- [ ] AC3: Selecting by `service_role` and `instance_index` returns a service's own lines and the job manager's lines about it.
- [ ] AC4: "Changes in 4.0.0" lists the renamed jobs.
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
| M1 | Follow one service | Start and stop the release tracker; select the lines with `service_role=udp_tracker` and `instance_index=1` | Its start lines, its shutdown lines and the job manager's lines about it | TODO | `manual-verification-evidence.md` section V1 |

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

- Job names appear in the shutdown plan and in error messages. Mitigation: T1 lists every user.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D3d)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
