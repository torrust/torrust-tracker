---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-12-configuration-keys-with-units/ISSUE.md
branch: "{issue-number}-2387-si-12-configuration-keys-with-units-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:16"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - packages/configuration/src/v3_0_0/core.rs
    - share/default/config/tracker.development.sqlite3.toml
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Add the unit to the configuration keys `max_peer_timeout` and `inactive_peer_cleanup_interval`

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-12 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D6, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: Nothing.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

Every duration key of the configuration says its unit. `max_peer_timeout` becomes `max_peer_timeout_in_secs` and `inactive_peer_cleanup_interval` becomes `inactive_peer_cleanup_interval_in_secs`.

## Background

The newer keys carry their unit (`deadline_in_secs`, `http_drain_in_secs`, `ip_bans_reset_interval_in_secs`); these two older keys do not. Rule 3 of D6 logs a configured duration under its key name, so the key names must be coherent for the log to be. This is a change of the configuration schema, not of a log line, so it has its own issue.

## Scope

### In Scope

- Rename the two keys in the current configuration schema, in the default configuration files under `share/default/config/`, in tests and in documents.
- Decide with the maintainer, in the specification pull request, how the schema version is handled: the keys change inside the configuration version that 4.0.0 ships, or the schema version is bumped. The specification records the decision and its reason.
- Decide what an old key in a configuration file does: with D12 (clean break), the tracker refuses to start and the message names the new key. Check what the configuration loader does with an unknown key today and record it.
- Search for any other duration key without a unit and report it.
- Environment-variable overrides and container documents that name the old keys.

### Out of Scope

- Durations in log lines (SI-10).
- Changing the values or their meaning.
- Older configuration schema versions kept for migration, unless the decision above says so.

## Architectural Decisions

- Related decisions: D6 of the EPIC. After SI-1 is merged, the logging ADR it creates governs this work.
- Related ADRs: the configuration ADRs in `docs/adrs/` (the specification pull request links the ones that govern schema changes). ADRs to create: none expected; one if the schema-version decision is material.

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
| T1 | TODO | Record how the schema and unknown keys behave today | Evidence in the specification |
| T2 | TODO | Maintainer decision on the schema version and on old keys | Recorded with alternatives and reason |
| T3 | TODO | Rename in the configuration package | Fields, serde names, defaults, tests |
| T4 | TODO | Rename in default files, tests and documents | No old key left |
| T5 | TODO | Old key behavior | A test for a configuration file with an old key |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Record how the schema and unknown keys behave today | Commit after focused validation and review. |
| T2 | Maintainer decision on the schema version and on old keys | Commit after focused validation and review. |
| T3 | Rename in the configuration package | Commit after focused validation and review. |
| T4 | Rename in default files, tests and documents | Commit after focused validation and review. |
| T5 | Old key behavior | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-12-configuration-keys-with-units/ISSUE.md`
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

- [ ] AC1: The two new keys are read and have the same effect as the old ones.
- [ ] AC2: A configuration file with an old key does what the recorded decision says, with a message that names the new key.
- [ ] AC3: No default configuration, test or current document uses an old key.
- [ ] AC4: The change is listed for operators in "Changes in 4.0.0" of `docs/logging.md`, or where SI-1 says configuration changes go.
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
| M1 | Start with the new keys | Start the release tracker with the development configuration | It starts; the log shows the two values under the new names once SI-10 is merged | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Start with an old key | Put `max_peer_timeout = 60` in a copy of the configuration and start | The behavior the decision records | TODO | `manual-verification-evidence.md` section V2 |

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

- Deployed configuration files break on upgrade. Mitigation: 4.0.0 is a major release; the error names the new key.
- If SI-1 has not merged when this issue runs, there is no "Changes in 4.0.0" section yet. Mitigation: the working order puts SI-1 before this issue.

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
