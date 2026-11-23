---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-21-metrics-registration-summary-line/ISSUE.md
branch: "{issue-number}-2387-si-21-metrics-registration-summary-line-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:53"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - Cargo.toml
    - .claude/skills/update-dependencies/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Summarize metrics registration in one line

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-21 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D7a, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: A release of `torrust-metrics` with the fix.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

Starting the tracker logs one `INFO` line for metrics registration, with the number of counters and gauges, instead of one line for each metric.

## Background

The crate `torrust-metrics` 0.2.0 logs one `INFO` line under the target `METRICS` in `describe_counter` and `describe_gauge`, with `unit` and `description` as `Debug` of an `Option`. The baseline has 28 such lines, more than a tenth of a start. The cause is in that crate, so the fix is there: `DEBUG` level, `Display` values, no `Option` wrapper. This issue is blocked until that crate releases the fix.

## Scope

### In Scope

- Bump `torrust-metrics` to the release with the fix (see the `update-dependencies` skill).
- Log `Registered metrics counters=<n> gauges=<n>` once, after registration, with the real counts. The specification pull request decides the target and whether it is one line for the process or one for each registry.
- Check that the per-metric lines appear at `debug` and not at `info`.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- Changing `torrust-metrics` itself (another repository).
- Filtering the `METRICS` target in the tracker's logging setup: decision D7a rejected it.

## Architectural Decisions

- Related decisions: D7a of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
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
| T1 | TODO | Confirm the upstream release | Version and changelog entry linked |
| T2 | TODO | Bump the dependency | Workspace builds; `cargo machete` clean |
| T3 | TODO | Add the summary line | With a test of the counts |
| T4 | TODO | Recapture | One line at `info`; per-metric lines at `debug` |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Confirm the upstream release | Commit after focused validation and review. |
| T2 | Bump the dependency | Commit after focused validation and review. |
| T3 | Add the summary line | Commit after focused validation and review. |
| T4 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-21-metrics-registration-summary-line/ISSUE.md`
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

- [ ] AC1: A capture at `info` has no per-metric line and one `Registered metrics` line for each place decided in the specification.
- [ ] AC2: The counts equal the number of metrics the REST API's metrics endpoint exposes.
- [ ] AC3: "Changes in 4.0.0" lists the change.
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
| M1 | Read a full start at `info` and at `debug` | Start the release tracker at each level | `info`: the summary only; `debug`: the summary and the per-metric lines without `Some(` | TODO | `manual-verification-evidence.md` section V1 |

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

- The crate is not released before the EPIC's other sub-issues finish. Mitigation: the EPIC's close-out rule lets the maintainer detach this issue.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D7a)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
