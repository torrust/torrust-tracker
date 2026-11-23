---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p1
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-2-logging-convention-adr-and-docs/ISSUE.md
branch: "{issue-number}-2387-si-2-logging-convention-adr-and-docs-spec"
related-pr: null
last-updated-utc: "2026-11-23 17:29"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/structured-runtime-logging/SKILL.md
    - docs/index.md
    - docs/adrs/index.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Record the logging convention: ADR, `docs/logging.md`, inventory and skill

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-2 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries) and stops for the maintainer to review and approve it. Only after that approval it creates the GitHub issue and opens the specification pull request, before any code.

- Implements: D2, D3, D4, D5, D6, D9, D11, D12, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: Nothing.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

The lifecycle logging convention the maintainer decided is written where contributors and operators find it: an ADR for the decisions, `docs/logging.md` as the reference, and the `structured-runtime-logging` skill for the workflow. An inventory maps every lifecycle line of today to the line it becomes.

## Background

The decisions of the EPIC are in its `decisions.md`, which moves to `docs/issues/closed/` when the EPIC closes. Rules that bind future code need a lasting place. Every later sub-issue adds its lines and its operator-visible changes to `docs/logging.md`, so that document must exist first. This issue changes no code.

## Scope

### In Scope

- A root ADR in `docs/adrs/` with the decisions D2 to D6, D9, D11 and D12 and their reasons, linked from the ADR index.
- `docs/logging.md`, linked from `docs/index.md`, with: the four-address glossary; the list of targets and what each is for; the canonical fields (`service_role`, `instance_index`, `service_binding`, `bind_address`, `public_url`, duration fields, `elapsed_in_ms`, `job`); the value format rules; the span rule; the level table; the list of deliberate-configuration warnings that a clean run may log; a catalogue of lifecycle lines (target, level, message, fields, when it is logged); and the section "Changes in 4.0.0" for operators.
- An issue-local inventory (`lifecycle-log-inventory.md`) of every startup and shutdown line in the baseline: source location, level, target, message, fields, and the sub-issue that changes it. It covers the application layer, the job manager, the UDP and HTTP trackers, the REST API, the health check API, the event listeners and the periodic jobs.
- In the inventory, the start sequence of the REST API and of the health check API, traced in the code line by line: where the socket is bound, where each line is logged and where the service is registered. Decision D4 was traced only for the UDP and HTTP trackers.
- The `structured-runtime-logging` skill links to the ADR and to `docs/logging.md` instead of repeating them, and says that a pull request that changes a lifecycle line updates the catalogue and the list of changes.
- The catalogue describes the target state. Lines not yet implemented are marked with the sub-issue that implements them.

### Out of Scope

- Any change to Rust code, including rustdoc on target constants (each code sub-issue adds the rustdoc for what it touches).
- The level policy for request-handling logs (#1166).
- The private-mode ADR (SI-3).

## Architectural Decisions

- Related decisions: D2, D3, D4, D5, D6, D9, D11, D12 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
- ADRs to create: one root ADR, "Define the lifecycle logging convention" (final title in the specification pull request). It is a root ADR because the convention spans the application and several packages.

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
| T1 | TODO | Write the inventory | `lifecycle-log-inventory.md` lists every lifecycle line of the three baseline captures with its source location and the sub-issue that changes it; the REST API and health check API start sequences are traced |
| T2 | TODO | Report inventory findings that contradict a decision | A list for the maintainer, or a note that there is none. Work stops on a contradiction until he answers |
| T3 | TODO | Write the ADR | ADR in `docs/adrs/`, indexed |
| T4 | TODO | Write `docs/logging.md` | Reference document linked from `docs/index.md`, with an empty "Changes in 4.0.0" section ready to fill |
| T5 | TODO | Update the skill | `structured-runtime-logging` links to both documents; its validation script, if it has one, passes |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Write the inventory | Commit after focused validation and review. |
| T2 | Report inventory findings that contradict a decision | Commit after focused validation and review. |
| T3 | Write the ADR | Commit after focused validation and review. |
| T4 | Write `docs/logging.md` | Commit after focused validation and review. |
| T5 | Update the skill | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-2-logging-convention-adr-and-docs/ISSUE.md`
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

- [ ] AC1: The ADR is accepted and states each decision with its reason.
- [ ] AC2: `docs/logging.md` holds the glossary, targets, fields, formats, spans, levels, catalogue and list of changes, and `docs/index.md` links it.
- [ ] AC3: The inventory covers every lifecycle line of the three baseline captures, and each line has a target state or a recorded reason to stay as it is.
- [ ] AC4: The REST API and health check API start sequences are traced, and D4 is confirmed for them or the difference is reported to the maintainer.
- [ ] AC5: The skill links to the documents and duplicates none of their content.
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
| M1 | Use the reference as an operator | Take ten lines at random from the baseline capture and find for each, using only `docs/logging.md`, what it will look like | Each line is found in the catalogue or in the inventory | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Use the skill as a contributor | Follow the skill to decide target, fields and level for a made-up new lifecycle line | The skill and the documents give one answer with no contradiction | TODO | `manual-verification-evidence.md` section V2 |

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

- The documents describe lines that do not exist yet. Mitigation: each catalogue entry names the sub-issue that implements it, and SI-23 removes the marks.
- The inventory finds a case the decisions did not foresee. Mitigation: task T2 stops and asks.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D2, D3, D4, D5, D6, D9, D11, D12)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
