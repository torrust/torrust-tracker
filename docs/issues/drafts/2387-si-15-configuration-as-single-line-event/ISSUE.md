---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-15-configuration-as-single-line-event/ISSUE.md
branch: "{issue-number}-2387-si-15-configuration-as-single-line-event-spec"
related-pr: null
last-updated-utc: "2026-11-24 12:46"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - src/bootstrap/app.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Log the configuration as one single-line event and add an `Enabled services` line

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-15 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries) and stops for the maintainer to review and approve it. Only after that approval it creates the GitHub issue and opens the specification pull request, before any code.

- Implements: D7b, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-5.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

The effective configuration is logged as one line, and a second line says in a few fields which services are enabled.

## Background

`src/bootstrap/app.rs` logs `Configuration:` followed by the configuration as indented JSON, about 110 lines in one event. Line-based tools see 110 lines of which one has a timestamp, and in JSON output it becomes one string with escaped newlines. An operator who wants to know what is running has to read all of it.

## Scope

### In Scope

- `APPLICATION: Loaded configuration configuration=<compact JSON>`: the same content as today, on one line, with secrets masked as today.
- `APPLICATION: Enabled services udp_trackers=<n> http_trackers=<n> rest_api=<bool> health_check_api=<bool> database_driver=<name> private=<bool> listed=<bool>`. The specification pull request fixes the list of fields.
- A test that the masked values stay masked in the one-line form.
- Check whether the configuration line belongs at `INFO` or `DEBUG` under the level rule and record the answer.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- Changing what the configuration contains or how secrets are masked.
- The JSON output format of #2766.

## Findings of the coordinator's check (2026-11-24)

The EPIC coordinator compared this draft with the code at `develop` `6a058dd4d` on 2026-11-24. Nothing changed since the draft was written; these are cases the draft did not name. The session that completes the draft checks each one and decides how its specification uses it.

- Under order B′ (D1) the output is JSON when this issue runs. The specification decides whether `configuration` is a JSON string or a nested object in the record, and records it in `docs/logging.md`.
- `to_redacted_json` uses `to_string_pretty` (`packages/configuration/src/v3_0_0/mod.rs:521`), so a compact form needs a change in `packages/configuration`.
- `Configuration:` is logged at `src/bootstrap/app.rs:66`, after the logging setup. Masking: API tokens by `mask_secrets` (`packages/configuration/src/v3_0_0/mod.rs:526`), the database password by its `Serialize` (`v3_0_0/database.rs:149,158`).
- For `Enabled services`: `core.database` is an `Option` (`packages/configuration/src/v3_0_0/core.rs:28`), so `database_driver` can be absent; `health_check_api` is always present (`mod.rs:343`).
- In the `pretty` style one event spans several lines whatever this issue does.

## Architectural Decisions

- Related decisions: D7b of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
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
| T1 | TODO | Record what is masked today | Fields and the masking code, as evidence |
| T2 | TODO | One-line configuration event | With the masking test |
| T3 | TODO | `Enabled services` line | With a test of the counts |
| T4 | TODO | Recapture | Two lines replace about 110 |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Record what is masked today | Commit after focused validation and review. |
| T2 | One-line configuration event | Commit after focused validation and review. |
| T3 | `Enabled services` line | Commit after focused validation and review. |
| T4 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-15-configuration-as-single-line-event/ISSUE.md`
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

- [ ] AC1: The configuration is logged as exactly one line.
- [ ] AC2: No secret that is masked today appears in it.
- [ ] AC3: The `Enabled services` line matches the services that start.
- [ ] AC4: "Changes in 4.0.0" lists the change.
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
| M1 | Read a full start | Start the release tracker with the development configuration | One `Loaded configuration` line and one `Enabled services` line with `udp_trackers=2 http_trackers=2` | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Secrets | Search the log for the API access token of the development configuration | Not found | TODO | `manual-verification-evidence.md` section V2 |

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

- A long line is hard to read in a terminal. Mitigation: the summary line answers the common question; the full line is for tools.
- A new secret is added to the configuration later and is not masked. Mitigation: out of scope here; issue #2993 covers secrets in logs.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D7b)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
