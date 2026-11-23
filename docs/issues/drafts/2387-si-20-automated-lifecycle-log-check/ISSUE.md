---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-20-automated-lifecycle-log-check/ISSUE.md
branch: "{issue-number}-2387-si-20-automated-lifecycle-log-check-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:53"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - tests/lifecycle/signals.rs
    - tests/common/native_tracker/mod.rs
    - .claude/skills/structured-runtime-logging/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Check the lifecycle log convention automatically

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-20 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D11, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-1 to SI-19.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

A test that runs in CI starts the tracker, stops it, and fails when a lifecycle line breaks the convention.

## Background

Without a check, the next change adds a `local_addr` again. The convention has rules a program can test on the log of a real run.

## Scope

### In Scope

- A maintained Rust test (not a shell script) that runs the tracker binary with every service enabled, waits for `Tracker started`, sends `SIGTERM`, and reads the whole log.
- It asserts: no `WARN` and no `ERROR` on a clean run, except the listed deliberate-configuration warnings; no `Debug` pattern in a lifecycle line (`Some(`, `None`, a struct dump `Name { … }`); every lifecycle line has a target from the list in `docs/logging.md`, with no module path; no removed field name (`local_addr`, `address`, `bind_addr`, `timeout_seconds`, `latency_ms`); no `service_binding` value with a trailing slash; no duration value with a unit suffix.
- The same run with usage statistics disabled.
- The list of targets and of removed names is in one place that the test and `docs/logging.md` share, or the test checks that they agree.
- A message that tells the contributor which rule failed and where the rule is written.
- Decide in the specification pull request where the test lives (the root `tests/lifecycle/` with the native tracker fixture is the candidate) and what it costs in CI time.

### Out of Scope

- Checking request-handling lines beyond the removed field names.
- A lint over source code; this check reads real output.
- Lines logged by dependencies, which the test lists as exceptions with a reason.

## Architectural Decisions

- Related decisions: D11 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
- ADRs to create: none; D11 is in the ADR of SI-2.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. If a decision of the EPIC does not hold in the code, stop, record the finding and ask the maintainer.

## Design and Ownership Review

The test owns a child process. The specification states: the fixture that owns the child and kills it on drop; the absolute deadline for the start and for the stop; and that a failed assertion still stops the tracker.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Choose the boundary and the harness | Recorded with the CI cost |
| T2 | TODO | Clean-run assertions | Test with statistics enabled |
| T3 | TODO | Statistics-disabled run | Second case |
| T4 | TODO | Shared lists | Targets and removed names in one place |
| T5 | TODO | Prove it fails | For each rule, a temporary violation that makes the test fail, recorded as evidence and reverted |
| T6 | TODO | Document | `docs/logging.md` and the skill say the check exists and how to run it |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Choose the boundary and the harness | Commit after focused validation and review. |
| T2 | Clean-run assertions | Commit after focused validation and review. |
| T3 | Statistics-disabled run | Commit after focused validation and review. |
| T4 | Shared lists | Commit after focused validation and review. |
| T5 | Prove it fails | Commit after focused validation and review. |
| T6 | Document | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-20-automated-lifecycle-log-check/ISSUE.md`
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

- [ ] AC1: The test passes on `develop` and runs in CI.
- [ ] AC2: For each rule there is recorded evidence that a violation fails the test.
- [ ] AC3: The exception list is empty or each entry has a reason.
- [ ] AC4: The failure message names the rule.
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
| M1 | Break a rule by hand | Add a lifecycle `tracing::info!` with a `local_addr` field and no target, and run the test | It fails and names both rules | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Run it alone | Run only this test with the command the documents give | It passes in the time the specification records | TODO | `manual-verification-evidence.md` section V2 |

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

- A test that parses text logs breaks when #2766 changes the format. Mitigation: the test reads fields through one small parser that #2766 replaces.
- False failures from timing. Mitigation: it waits for a line, with a deadline, not for a fixed time.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D11)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
