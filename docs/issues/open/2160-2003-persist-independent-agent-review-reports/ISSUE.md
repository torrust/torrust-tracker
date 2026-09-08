---
doc-type: issue
issue-type: enhancement
status: in-progress
priority: p2
epic: 2003
github-issue: 2160
spec-path: docs/issues/open/2160-2003-persist-independent-agent-review-reports/ISSUE.md
branch: "2160-2003-persist-independent-agent-review-reports"
related-pr: 2166
last-updated-utc: 2026-09-07 17:20
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/pr-reviewer.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/committer.agent.md
    - docs/templates/ISSUE.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

# Issue #2160 - Persist Independent Agent Review Reports

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Retain explicit, chronological, evidence-based reports from independent review agents in the
folder-style issue specification they review.

## Background

Complexity, task-completion, and pull-request reviewers currently return their results only to the
caller. Their evidence and verdicts should be durable issue-local records that a maintainer can
inspect later. The current Complexity Auditor lacks permission to edit files, so reviewer profiles
must be updated before reviewers can own these reports.

## Scope

### In Scope

- Create a reusable `agent-review-reports.md` template with YAML frontmatter.
- Define append-only report entries with UTC timestamp, reviewer, invocation scope, inputs and
  evidence, findings, verdict, and follow-up actions.
- Add `edit` to the declared tools of `Complexity Auditor`, `Task Reviewer`, and `PR Reviewer`.
- Require those reviewers to create or append the report before returning their caller-facing verdict
  when a folder-style issue specification is supplied.
- Require report-only changes made on a branch or PR to be committed through `Committer`.
- Keep Copilot Suggestions Handler's PR-thread tracker in `docs/copilot-pr-reviews/`; it is not an
  independent reviewer under this policy.

### Out of Scope

- Requiring reports from planning, implementation, research, commit, or GitHub-operation agents.
- Moving existing Copilot suggestion trackers.
- Storing raw logs, secrets, tokens, or external data that has not been removed in reports.
- Adding runtime enforcement that prevents a reviewer from returning before persistence succeeds.

## Architectural Decisions

- Related ADR: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: None. Create an ADR only if evidence shows repository-wide orchestration architecture must change.

## Implementation Plan

| ID  | Status | Task                                | Notes / Expected Output                                                                                       |
| --- | ------ | ----------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Define report template              | Added `docs/templates/AGENT-REVIEW-REPORTS.md` with frontmatter and append-only entries.                      |
| T2  | DONE   | Update reviewer tools and workflows | Added shared persistence guidance; Complexity Auditor now declares `edit`.                                    |
| T3  | DONE   | Update issue-spec template          | Added review-report checkpoint and artifact guidance.                                                         |
| T4  | DONE   | Test sequential reporting           | All three reviewers appended chronological entries; the commit-ownership correction was tested and committed. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-persist-independent-agent-review-reports/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2160 created and issue number added to this spec
- [x] Implementation completed and verified
- [x] Acceptance criteria reviewed after implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Created as an immediately implementable EPIC #2003 child; reviewer edit permission selected by maintainer - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2160, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2160
- 2026-09-07 16:20 UTC - GitHub Copilot - Added the reusable template, reviewer persistence contract, caller/Committer handoff, indexes, and structural contract test - Pending validation
- 2026-09-07 16:22 UTC - Complexity Auditor and Task Reviewer - Appended sequential independent-review entries. Task Reviewer found the expected remaining evidence steps: a Committer-handled report update, PR Reviewer invocation, and completion assessment - `agent-review-reports.md`
- 2026-09-07 16:30 UTC - GitHub Copilot - Recorded prior PR #2163 Copilot tracker evidence for the separate-tracker boundary; preparing the reviewed change set for the Committer, after which a PR Reviewer can append the third entry - Pending commit
- 2026-09-07 16:40 UTC - PR Reviewer - Appended the third sequential report entry and requested explicit caller-to-Committer ownership constraints for Task Reviewer and PR Reviewer, with matching contract-test coverage - PR #2166
- 2026-09-07 16:45 UTC - GitHub Copilot - Added the required retrospective for the material commit-authority correction; preparing the correction and appended review records for a Committer-handled evidence commit - Pending commit
- 2026-09-07 16:49 UTC - Task Reviewer - Verified the signed Committer-handled evidence commit `6b39d10bf332b07be107a55afe95f2d64e63215b` is pushed to PR #2166; AC5 and M1/M2/M3 pass, and the required retrospective records the material correction - `agent-review-reports.md`
- 2026-09-07 17:00 UTC - GitHub Copilot - Processing four valid Copilot suggestions on PR #2166. The first clarifies that failed-review evidence may use a caller-requested documentation-only Committer handoff without authorizing implementation commits - `docs/copilot-pr-reviews/pr-2166-copilot-suggestions.md`
- 2026-09-07 17:05 UTC - GitHub Copilot - Processing the duplicate PR #2166 contract-test suggestions with one hardened frontmatter and `edit`-tool assertion change; validation passed - `docs/copilot-pr-reviews/pr-2166-copilot-suggestions.md`
- 2026-09-07 17:15 UTC - GitHub Copilot - Processing the final PR #2166 suggestion by separating the passed-review implementation-commit and failed-review evidence-commit paths in the orchestration tables - `docs/copilot-pr-reviews/pr-2166-copilot-suggestions.md`
- 2026-09-07 17:20 UTC - GitHub Copilot - Applied, validated, and committed the action fixes, then replied to and resolved all four PR #2166 Copilot suggestions; tracker completion record pending its own commit - `docs/copilot-pr-reviews/pr-2166-copilot-suggestions.md`

## Acceptance Criteria

- [x] A reusable review-report template provides frontmatter and every required entry field.
- [x] Complexity Auditor, Task Reviewer, and PR Reviewer each declare the `edit` tool.
- [x] Each reviewer creates or appends an explicit report before returning its verdict when given a folder-style spec.
- [x] Sequential reports preserve earlier entries and make reviewer, evidence, findings, verdict, and follow-up visible.
- [x] Branch or PR report updates are committed through Committer.
- [x] Copilot Suggestions Handler keeps its dedicated PR-thread tracker location and workflow.
- [x] `linter all` exits with code `0` and relevant documentation checks pass.

## Verification Plan

### Automatic Checks

- `linter all`
- Structural checks for template and profile requirements
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario                 | Command/Steps                                                | Expected Result                                                                      | Status | Evidence                                                                                                                                     |
| --- | ------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------------------------------ | ------ | -------------------------------------------------------------------------------------------------------------------------------------------- |
| M1  | Sequential reviews       | Invoke all three reviewers against a folder-style test spec. | One report file contains complete chronological entries from each reviewer.          | DONE   | `agent-review-reports.md` preserves complete chronological Complexity Auditor, Task Reviewer, and PR Reviewer entries.                       |
| M2  | PR report commit         | Perform a PR-review report update on a disposable branch.    | The report update is committed through Committer before final completion.            | DONE   | Signed Committer-handled commit `6b39d10bf332b07be107a55afe95f2d64e63215b` includes the PR Reviewer report and is pushed to PR #2166.        |
| M3  | Copilot tracker boundary | Inspect a completed representative Copilot review tracker.   | Its tracker remains in `docs/copilot-pr-reviews/` rather than the issue spec folder. | DONE   | `docs/copilot-pr-reviews/pr-2163-copilot-suggestions.md` records an addressed, replied-to, resolved PR thread outside the issue spec folder. |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                              |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | `docs/templates/AGENT-REVIEW-REPORTS.md`; structural contract test passed                                                             |
| AC2   | DONE                   | Reviewer profile frontmatter; structural contract test passed                                                                         |
| AC3   | DONE                   | Shared reviewer persistence policy; structural contract test passed                                                                   |
| AC4   | DONE                   | Complexity Auditor entry and Task Reviewer entry in `agent-review-reports.md`                                                         |
| AC5   | DONE                   | Signed Committer-handled commit `6b39d10bf332b07be107a55afe95f2d64e63215b` includes the PR Reviewer report and is pushed to PR #2166. |
| AC6   | DONE                   | Copilot Suggestions Handler profile and structural contract test passed                                                               |
| AC7   | DONE                   | `linter all`, structural contract test, and `cargo test --doc --workspace` passed                                                     |

## Risks and Trade-offs

- Reports add small process overhead. Keep entries concise and only require them when a folder-style spec is supplied.
- Reviewer writes can create uncommitted documentation changes. The workflow must make Committer ownership explicit.

## Implementation Completion Review

After implementation, record material report-persistence or reviewer-workflow discoveries in an
issue-local `implementation-retrospective.md`. If none occurred, add a concise progress-log entry
explaining why no retrospective is needed.

- Retrospective: `implementation-retrospective.md`

## References

- Parent EPIC: #2003
- GitHub issue: #2160
- Reviewer profiles: `.github/agents/complexity-auditor.agent.md`, `.github/agents/task-reviewer.agent.md`, `.github/agents/pr-reviewer.agent.md`
