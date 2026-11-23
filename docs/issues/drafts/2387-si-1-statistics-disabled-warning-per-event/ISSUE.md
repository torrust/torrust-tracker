---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p1
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-1-statistics-disabled-warning-per-event/ISSUE.md
branch: "{issue-number}-2387-si-1-statistics-disabled-warning-per-event-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:53"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - packages/udp-server/src/statistics/event/listener.rs
    - packages/udp-core/src/statistics/event/listener.rs
    - packages/http-core/src/statistics/event/listener.rs
    - .claude/skills/fix-bug/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Bug: statistics listeners log a `WARN` for every event when usage statistics are disabled

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-1 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D8, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: Nothing.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

A tracker with `tracker_usage_statistics = false` logs nothing for each event it does not count. An event from a listener the statistics policy does not know is an `ERROR`. The operator sees once, at startup, for which services statistics are disabled.

## Background

Three statistics event listeners share one branch for two cases: the service has statistics disabled (a supported configuration) and the service is unknown (a bug). Both log `WARN … Ignoring … event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { … }`. A UDP request publishes one to four events and an HTTP announce or scrape publishes one, so a tracker that serves traffic with statistics disabled writes a `WARN` for every request. The baseline capture with statistics disabled shows eight of them for one health check request. The sites are `packages/udp-server/src/statistics/event/listener.rs`, `packages/udp-core/src/statistics/event/listener.rs` and `packages/http-core/src/statistics/event/listener.rs`.

## Scope

### In Scope

- In the three listeners, split the policy lookup into its three results: enabled (count the event), disabled (no log; `TRACE` at most), unknown (`ERROR` with `service_role` and `instance_index` as fields, not a `Debug` struct).
- Report the state once at startup: the statistics listener's start line says how many service instances have statistics enabled and disabled (for example `enabled_instances=0 disabled_instances=2`). The `Starting` line of each UDP and HTTP tracker already has, or gets, `tracker_usage_statistics`.
- A regression test: a tracker with statistics disabled handles a UDP request and an HTTP request and logs no `WARN` and no `ERROR`.
- A test for the unknown-listener case at the unit boundary of one listener.
- This issue runs before SI-2, which creates `docs/logging.md`. It lists its operator-visible changes in its own specification, and SI-2 copies them into "Changes in 4.0.0". If SI-2 is already merged when this issue runs, update `docs/logging.md` directly.

### Out of Scope

- Renaming the targets of these lines (SI-6).
- Stopping the publication of events when no listener counts them. The draft `docs/issues/drafts/optimize-event-publication-without-consumers` covers that.
- Any other `WARN` line.

## Architectural Decisions

- Related decisions: D8 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
- ADRs to create: none known. The level rule itself is recorded by SI-2.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. If a decision of the EPIC does not hold in the code, stop, record the finding and ask the maintainer.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Use `.claude/skills/fix-bug/SKILL.md`. Sequence: analysis (done in the EPIC: one branch for two cases); reproduction with the real binary (T1); regression-test boundary selection; red test (T2); production fix (T3, T4); green test and the like-for-like recheck (T6).

## Regression Test Strategy

Two boundaries. A unit test in each listener for the three results of the policy lookup, because the defect is in that branch. One test at the tracker level that starts a statistics-disabled tracker, sends a request and asserts on the captured log, because the defect an operator sees is the log line; the specification pull request names the existing harness it uses (the native tracker fixture in `tests/common/native_tracker/`).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce with the real binary | Capture of a statistics-disabled tracker that serves one UDP announce and one HTTP announce, showing the `WARN` lines, in `manual-verification-evidence.md` |
| T2 | TODO | Write the failing regression test | Red test output recorded |
| T3 | TODO | Fix the three listeners | Three results, each handled in its own branch |
| T4 | TODO | Report the state at startup | Start line of each statistics listener carries the counts |
| T5 | TODO | Test the unknown-listener case | A unit test asserts the `ERROR` and its fields |
| T6 | TODO | Recheck with the real binary | Same scenario as T1: no `WARN`, no `ERROR` |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Reproduce with the real binary | Commit after focused validation and review. |
| T2 | Write the failing regression test | Commit after focused validation and review. |
| T3 | Fix the three listeners | Commit after focused validation and review. |
| T4 | Report the state at startup | Commit after focused validation and review. |
| T5 | Test the unknown-listener case | Commit after focused validation and review. |
| T6 | Recheck with the real binary | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-1-statistics-disabled-warning-per-event/ISSUE.md`
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

- [ ] AC1: With statistics disabled, handling UDP and HTTP requests logs no `WARN` and no `ERROR`.
- [ ] AC2: An event from an unknown listener logs one `ERROR` with `service_role` and `instance_index` fields.
- [ ] AC3: The startup log says for how many instances statistics are enabled and disabled.
- [ ] AC4: With statistics enabled, the counted metrics are the same as before the change.
- [ ] AC5: The regression test fails without the fix and passes with it.
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
| M1 | Reproduce the defect | Start the release tracker with `tracker_usage_statistics = false` in every tracker; send one UDP announce and one HTTP announce with the tracker client; stop it with `SIGTERM`; count the `WARN` lines | Before the fix: one or more `WARN` for each request | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Recheck after the fix | Same steps on the fixed binary | No `WARN`, no `ERROR`; the start lines show the disabled state | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Statistics enabled | Same steps with the unchanged development configuration; read the metrics from the REST API | Counters increase as before the change | TODO | `manual-verification-evidence.md` section V3 |

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

- The unknown-listener `ERROR` fires in a case that is normal and was hidden by the shared branch. Mitigation: the manual scenarios run every service; any `ERROR` found is reported before merging.
- A test that asserts the absence of a log line passes for the wrong reason. Mitigation: the red run of T2 proves the test can fail.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D8)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
