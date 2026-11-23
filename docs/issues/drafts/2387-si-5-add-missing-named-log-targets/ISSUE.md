---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-5-add-missing-named-log-targets/ISSUE.md
branch: "{issue-number}-2387-si-5-add-missing-named-log-targets-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:53"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
    - packages/axum-server/src/signals.rs
    - src/bootstrap/jobs/udp_tracker.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Add named targets to lifecycle lines that have none

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-5 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D2, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-2.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

Every startup and shutdown line has a named target from the list in `docs/logging.md`. No lifecycle line shows a Rust module path as its target.

## Background

Some lifecycle lines use a named target constant and the others take the default, the module path, for example `torrust_tracker_lib::bootstrap::jobs::manager` and `torrust_tracker_axum_server::signals`. An operator who filters by component must know both vocabularies, and a refactor changes the module path. This issue only adds targets. It renames none; SI-6 does that.

## Scope

### In Scope

- A `JOB_MANAGER` target for every line of `src/bootstrap/jobs/manager.rs`.
- An `APPLICATION` target for process-level lines: logging initialized, configuration, signal handlers installed, shutting down, shutdown result, startup failure, and the lines that say a service is not configured. The existing `torrust_tracker` target constant becomes `APPLICATION` here, because it is the same set of lines.
- A target for each periodic job (torrent cleanup, peers inactivity update, UDP ban cleanup) and for each event listener start and stop line that has none. The specification pull request lists the names; a job that belongs to an existing component uses that component's target.
- The shared HTTP shutdown code in `packages/axum-server/src/signals.rs` logs under the target of the service it stops (HTTP tracker, REST API or health check API), passed in by the caller.
- Rustdoc on each new target constant: what the target is for.
- Lifecycle lines logged inside a job starter without a target (for example `Starting UDP tracker instance` in `src/bootstrap/jobs/udp_tracker.rs`).
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- Renaming existing targets (SI-6).
- Changing messages, fields or levels.
- Request-handling lines.

## Architectural Decisions

- Related decisions: D2 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
- ADRs to create: none. The decision is in the ADR of SI-2.

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
| T1 | TODO | List the lines without a named target | From the inventory of SI-2: each line and the target it gets |
| T2 | TODO | Add `JOB_MANAGER` | All job manager lines; tests that match these lines updated |
| T3 | TODO | Add `APPLICATION` | All process-level lines |
| T4 | TODO | Add targets to periodic jobs and listeners | Per component |
| T5 | TODO | Pass the service target to the shared HTTP shutdown code | The three HTTP services log their shutdown under their own target |
| T6 | TODO | Recapture | A capture at `info` and `debug` with no module path as target on a lifecycle line |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | List the lines without a named target | Commit after focused validation and review. |
| T2 | Add `JOB_MANAGER` | Commit after focused validation and review. |
| T3 | Add `APPLICATION` | Commit after focused validation and review. |
| T4 | Add targets to periodic jobs and listeners | Commit after focused validation and review. |
| T5 | Pass the service target to the shared HTTP shutdown code | Commit after focused validation and review. |
| T6 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-5-add-missing-named-log-targets/ISSUE.md`
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

- [ ] AC1: No lifecycle line in a capture at `info` or `debug` has a module path as target.
- [ ] AC2: The shutdown lines of the HTTP tracker, the REST API and the health check API carry the target of that service.
- [ ] AC3: Every new target constant has rustdoc and is listed in `docs/logging.md`.
- [ ] AC4: No message, field or level changed.
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
| M1 | Read a full run | Start the release tracker with the development configuration, stop it with `SIGTERM`, and list the distinct targets of the log | Only names from the list in `docs/logging.md` | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Filter by component | From the same log, select the lines of one HTTP tracker by target | Its start and its shutdown lines are both selected | TODO | `manual-verification-evidence.md` section V2 |

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

- Tests match a module path in log text. Mitigation: T1 searches the tests and each task updates what it breaks.
- Lines logged by a dependency keep the dependency's module path. Mitigation: they are listed in the inventory as outside the tracker's control.

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
