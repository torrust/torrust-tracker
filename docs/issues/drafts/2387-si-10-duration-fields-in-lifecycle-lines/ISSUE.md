---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-10-duration-fields-in-lifecycle-lines/ISSUE.md
branch: "{issue-number}-2387-si-10-duration-fields-in-lifecycle-lines-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:16"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - src/bootstrap/jobs/manager.rs
    - tests/lifecycle/shutdown_policy.rs
    - tests/common/native_tracker/mod.rs
    - packages/configuration/src/v3_0_0/shutdown.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Write durations in lifecycle lines as numbers with the unit in the name

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-10 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D6, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-1.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

Every duration in a lifecycle line is a number in a field whose name ends with its unit, and a configured duration has the name of its configuration key.

## Background

The same kind of value appears as `timeout_seconds=25`, `drain_timeout=15s`, `request_drain_deadline=5s` and as `in 15s` inside a message. `Debug` of `Duration` changes its unit with the value, so a tool cannot read it as a number.

## Scope

### In Scope

- Rule 1: a number in a field; never `Debug` of `Duration`; never only in the message.
- Rule 2: the suffix `_in_secs` or `_in_ms`.
- Rule 3: a configured duration uses its configuration key, prefixed with its section when ambiguous: `shutdown_deadline_in_secs`, `http_drain_in_secs`, `udp_request_drain_in_secs`, `event_listener_drain_in_secs`, `cookie_lifetime_in_secs`, `ip_bans_reset_interval_in_secs`. For the two keys that have no unit yet, the log uses the name with the unit (`max_peer_timeout_in_secs`, `inactive_peer_cleanup_interval_in_secs`).
- Rule 4: a measured duration is `elapsed_in_ms`, an integer. This issue renames existing measured durations in lifecycle lines; new ones come with SI-19 and SI-20.
- Update the tests that read `timeout_seconds=` (`tests/lifecycle/shutdown_policy.rs`, `tests/common/native_tracker/mod.rs`).
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- `latency_ms` in request logs (SI-11).
- Renaming the configuration keys (SI-12).
- Adding durations that are not logged today (SI-19, SI-20).

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
| T1 | TODO | List the duration values | From the inventory: each duration in a lifecycle line, its form today and its new name |
| T2 | TODO | Job manager and application lines | `timeout_seconds` to `shutdown_deadline_in_secs`; tests updated |
| T3 | TODO | UDP tracker lines | Drain and cookie durations |
| T4 | TODO | HTTP services and shared shutdown code | Drain durations; `in 15s` leaves the message |
| T5 | TODO | Periodic jobs | Interval fields |
| T6 | TODO | Recapture | No duration with a unit letter in its value |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | List the duration values | Commit after focused validation and review. |
| T2 | Job manager and application lines | Commit after focused validation and review. |
| T3 | UDP tracker lines | Commit after focused validation and review. |
| T4 | HTTP services and shared shutdown code | Commit after focused validation and review. |
| T5 | Periodic jobs | Commit after focused validation and review. |
| T6 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-10-duration-fields-in-lifecycle-lines/ISSUE.md`
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

- [ ] AC1: No lifecycle field value in a capture at `debug` is a duration with a unit suffix such as `15s` or `250ms`.
- [ ] AC2: Each configured duration in the log has the name of its configuration key.
- [ ] AC3: No lifecycle message contains a duration.
- [ ] AC4: "Changes in 4.0.0" lists the renamed fields.
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
| M1 | Compare with the configuration | Start the release tracker; for each duration key of the configuration file, search the log for the key name | Each is found with the configured value | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Shutdown deadline | Stop the tracker with `SIGTERM` | The shutdown lines show `shutdown_deadline_in_secs=25` | TODO | `manual-verification-evidence.md` section V2 |

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

- A configured value is fractional or not in seconds. Mitigation: T1 records the type of each; the unit in the name is the unit of the logged number.

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
