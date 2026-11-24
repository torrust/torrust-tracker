---
schema-version: 1
doc-type: issue
issue-type: enhancement
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-3-reject-private-mode-with-udp-trackers/ISSUE.md
branch: "{issue-number}-2387-si-3-reject-private-mode-with-udp-trackers-spec"
related-pr: null
last-updated-utc: "2026-11-24 12:46"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - src/app.rs
    - packages/configuration/src/v3_0_0/mod.rs
    - packages/configuration/src/validator.rs
    - packages/test-helpers/src/configuration.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Reject private mode with UDP trackers configured at startup

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-3 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries) and stops for the maintainer to review and approve it. Only after that approval it creates the GitHub issue and opens the specification pull request, before any code.

- Implements: D9, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: Nothing.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

A configuration with `private = true` and one or more UDP trackers does not start. The tracker exits with a message that says why and what to change.

## Background

Today the tracker logs `WARN Could not start UDP trackers while in private mode. UDP is not safe for private trackers!` and runs without the configured UDP trackers (`log_udp_tracker_services_not_started` in `src/app.rs`). A configured service silently does not run. By the level rule that is an `ERROR`, and an `ERROR` on every start of a tracker that otherwise works is not acceptable either. Rejecting the configuration removes the question: the configuration does what it says or the tracker does not start.

## Scope

### In Scope

- Validate the combination where the configuration is validated (semantic validation, before any service starts) and return a typed startup error.
- Remove the `WARN` and the branch that skips the UDP trackers.
- An ADR for the decision: alternatives (keep the warning, reject, start UDP anyway), the reason UDP is not supported in private mode, and the consequence for existing configurations.
- Check the default and example configuration files, the container documents and the tests for this combination and fix them.
- The message a user sees, proposed in the specification pull request.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- Supporting private mode over UDP.
- Other configuration validations.
- The `No UDP trackers configured` line, which stays as an `INFO` (its target comes from SI-5).

## Findings of the coordinator's check (2026-11-24)

The EPIC coordinator compared this draft with the code at `develop` `6a058dd4d` on 2026-11-24. Nothing changed since the draft was written; these are cases the draft did not name. The session that completes the draft checks each one and decides how its specification uses it.

- The check belongs where sections can be compared: `impl Validator for Configuration` (`packages/configuration/src/v3_0_0/mod.rs:535-540`), with its error in `packages/configuration/src/validator.rs:13`. `Core::validate` (`core.rs:109`) cannot see `udp_trackers`. The error surfaces as `Error::SemanticValidation` (`src/bootstrap/app.rs:59-61`).
- Today's skip is `should_start_udp_tracker_services` (`src/app.rs:484-490`); the `WARN` is `log_udp_tracker_services_not_started` (`src/app.rs:492-503`). The unit test `it_should_not_start_udp_tracker_services_for_a_private_tracker` (`src/app.rs:860-871`) pins the skip.
- `ephemeral_private()` and `ephemeral_private_and_listed()` (`packages/test-helpers/src/configuration.rs:139,159`) build on `ephemeral()`, which has a UDP tracker (`:64`). They become invalid configurations. They are used in about 40 places (REST API, HTTP core and HTTP server tests, `src/app.rs:1177`).
- No shipped configuration under `share/default/config/` has `private = true`.
- EPIC #2993's specification mentions the skip (`docs/issues/open/2993-secrets-and-client-error-text/EPIC.md:87`). Its coordinator was told on 2026-11-24 and rewrote that line in PR 3025 to name this issue's startup failure; the check in its `evidence/reproduce.sh` never ran (its private configuration has no UDP tracker) and PR 3025 removes it. Nothing in #2993 depends on the skip.
- A startup failure exits `1` today (`src/main.rs:34`); `2` is for usage errors. The UDP packages have no private-mode check at all, which the ADR can state.

## Architectural Decisions

- Related decisions: D9 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
- ADRs to create: one root ADR, "Reject private mode with UDP trackers configured" (final title in the specification pull request). Related: the configuration validation ADRs, linked in the specification pull request.

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
| T1 | TODO | Record today's behavior | Capture of a private tracker with a UDP tracker configured: the `WARN`, and that UDP does not listen |
| T2 | TODO | Write the ADR | Accepted before code |
| T3 | TODO | Add the validation and its error | Unit tests of the validation |
| T4 | TODO | Remove the skip and the `WARN` | Application code and tests |
| T5 | TODO | Fix configurations and documents | No shipped configuration is rejected |
| T6 | TODO | Recheck | The same configuration as T1 does not start |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Record today's behavior | Commit after focused validation and review. |
| T2 | Write the ADR | Commit after focused validation and review. |
| T3 | Add the validation and its error | Commit after focused validation and review. |
| T4 | Remove the skip and the `WARN` | Commit after focused validation and review. |
| T5 | Fix configurations and documents | Commit after focused validation and review. |
| T6 | Recheck | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-3-reject-private-mode-with-udp-trackers/ISSUE.md`
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
- 2026-11-24 12:46 UTC - Claude Code (Opus 5.5), EPIC coordinator - Compared with the code at `develop` `6a058dd4d`: corrected the claims the code contradicts, recorded the maintainer's answers of 2026-11-24 that concern this draft, and added "Findings of the coordinator's check" (order B′ of D1 places #2766 after SI-1 and before SI-2).

## Acceptance Criteria

- [ ] AC1: A private configuration with a UDP tracker exits non-zero before any service starts, with a message that names both settings.
- [ ] AC2: A private configuration without UDP trackers and a public configuration with UDP trackers start as before.
- [ ] AC3: Every configuration file shipped in the repository starts.
- [ ] AC4: The ADR is accepted.
- [ ] AC5: The change is listed in "Changes in 4.0.0".
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
| M1 | Rejected configuration | Set `private = true` in a copy of the development configuration, which has two UDP trackers, and start | No service starts; the message says what to change; non-zero exit code | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Private without UDP | Remove the UDP trackers from that copy and start | It starts with no `WARN` | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Public with UDP | Start with the unchanged development configuration | It starts as before | TODO | `manual-verification-evidence.md` section V3 |

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

- Deployments that rely on the silent skip stop starting after the upgrade. Mitigation: a major release, a clear message, and the entry in "Changes in 4.0.0".
- The exit code: #2766 plans exit `2` for configuration errors. Mitigation: use the code other configuration errors use today; #2766 changes them together.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D9)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
