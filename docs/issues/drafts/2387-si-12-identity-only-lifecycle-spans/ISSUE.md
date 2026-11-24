---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-12-identity-only-lifecycle-spans/ISSUE.md
branch: "{issue-number}-2387-si-12-identity-only-lifecycle-spans-spec"
related-pr: null
last-updated-utc: "2026-11-23 17:29"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - src/bootstrap/jobs/udp_tracker.rs
    - src/app.rs
    - packages/udp-server/src/server/states.rs
    - packages/axum-server/src/signals.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Keep only the service identity in lifecycle spans

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-12 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries) and stops for the maintainer to review and approve it. Only after that approval it creates the GitHub issue and opens the specification pull request, before any code.

- Implements: D5, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-10.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

A lifecycle span carries `service_role` and `instance_index` and nothing else. No line shows the `Debug` output of an internal type, and a setting is logged once, on the `Starting` line.

## Background

`#[instrument]` records every argument it is not told to skip, with `Debug`. The baseline shows `start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=0}` in front of each line of a job, and nested spans from `complete_startup`, `start_with_cancellation` and `start_v1`.

## Scope

### In Scope

- On each service job's `start_job`: skip all arguments and keep the two identity fields.
- Remove `#[instrument]` from `complete_startup` and from the inner start functions (`start_with_cancellation`, `start_v1`, and the equivalents of the REST API and the health check API).
- Move the settings that were span fields to fields of the `Starting` line: `connection_id_validation`, the cookie lifetime, the drain durations, the HTTP `version`. Duration names follow SI-9 if it is merged; otherwise they keep today's names and SI-9 renames them.
- Spans of code without a service identity (`graceful_shutdown_on_cancellation`, job manager spans) carry no fields, or are removed when the lines inside already have the identity as event fields.
- Check that spawned tasks that log lifecycle lines still have the identity, as span or event fields.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- Request-handling spans (`request_id` and the like).
- Messages and levels.

## Architectural Decisions

- Related decisions: D5 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
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
| T1 | TODO | List the lifecycle spans | Each `#[instrument]` and manual span on the start and stop paths, with its fields today |
| T2 | TODO | UDP tracker job | Span reduced; settings on `Starting` |
| T3 | TODO | HTTP tracker job | Same |
| T4 | TODO | REST API and health check API jobs | Same |
| T5 | TODO | Application and shared code | `complete_startup`, `graceful_shutdown_on_cancellation` |
| T6 | TODO | Recapture | No `Debug` struct dump and no `Some(` in a span |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | List the lifecycle spans | Commit after focused validation and review. |
| T2 | UDP tracker job | Commit after focused validation and review. |
| T3 | HTTP tracker job | Commit after focused validation and review. |
| T4 | REST API and health check API jobs | Commit after focused validation and review. |
| T5 | Application and shared code | Commit after focused validation and review. |
| T6 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-12-identity-only-lifecycle-spans/ISSUE.md`
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

- [ ] AC1: Every lifecycle span in a capture at `debug` has only `service_role` and `instance_index`, or no fields.
- [ ] AC2: No line contains `CancellationToken`.
- [ ] AC3: Each setting that was a span field is a field of exactly one `Starting` line.
- [ ] AC4: Every lifecycle line of a service can still be attributed to it.
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
| M1 | Read a full run at `debug` | Start and stop the release tracker at `debug` | Short span prefixes with identity only | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Find a setting | Search the log for `connection_id_validation` | Found once for each UDP tracker, on its `Starting` line | TODO | `manual-verification-evidence.md` section V2 |

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

- A line logged in a spawned task loses its identity when the outer span is removed. Mitigation: T1 marks spawned tasks and T6 checks each line.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D5)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
