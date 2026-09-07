---
doc-type: issue
issue-type: enhancement
status: planned
priority: p2
epic: 2003
github-issue: 2160
spec-path: docs/issues/open/2160-2003-persist-independent-agent-review-reports/ISSUE.md
branch: "2160-2003-persist-independent-agent-review-reports"
related-pr: null
last-updated-utc: 2026-09-07 11:20
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

| ID  | Status | Task                                | Notes / Expected Output                                                        |
| --- | ------ | ----------------------------------- | ------------------------------------------------------------------------------ |
| T1  | TODO   | Define report template              | Document frontmatter and append-only entry shape.                              |
| T2  | TODO   | Update reviewer tools and workflows | Give all three reviewers edit permission and persistence instructions.         |
| T3  | TODO   | Update issue-spec template          | Make issue-local review-record expectations discoverable.                      |
| T4  | TODO   | Test sequential reporting           | Verify multiple reviewers append complete entries without overwriting history. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-persist-independent-agent-review-reports/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2160 created and issue number added to this spec
- [ ] Implementation completed and verified
- [ ] Acceptance criteria reviewed after implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Created as an immediately implementable EPIC #2003 child; reviewer edit permission selected by maintainer - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2160, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2160

## Acceptance Criteria

- [ ] A reusable review-report template provides frontmatter and every required entry field.
- [ ] Complexity Auditor, Task Reviewer, and PR Reviewer each declare the `edit` tool.
- [ ] Each reviewer creates or appends an explicit report before returning its verdict when given a folder-style spec.
- [ ] Sequential reports preserve earlier entries and make reviewer, evidence, findings, verdict, and follow-up visible.
- [ ] Branch or PR report updates are committed through Committer.
- [ ] Copilot Suggestions Handler keeps its dedicated PR-thread tracker location and workflow.
- [ ] `linter all` exits with code `0` and relevant documentation checks pass.

## Verification Plan

### Automatic Checks

- `linter all`
- Structural checks for template and profile requirements
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario                 | Command/Steps                                                | Expected Result                                                                      | Status | Evidence               |
| --- | ------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------------------------------ | ------ | ---------------------- |
| M1  | Sequential reviews       | Invoke all three reviewers against a folder-style test spec. | One report file contains complete chronological entries from each reviewer.          | TODO   | Pending implementation |
| M2  | PR report commit         | Perform a PR-review report update on a disposable branch.    | The report update is committed through Committer before final completion.            | TODO   | Pending implementation |
| M3  | Copilot tracker boundary | Process a representative Copilot thread.                     | Its tracker remains in `docs/copilot-pr-reviews/` rather than the issue spec folder. | TODO   | Pending implementation |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence               |
| ----- | ---------------------- | ---------------------- |
| AC1   | TODO                   | Pending implementation |
| AC2   | TODO                   | Pending implementation |
| AC3   | TODO                   | Pending implementation |
| AC4   | TODO                   | Pending implementation |
| AC5   | TODO                   | Pending implementation |
| AC6   | TODO                   | Pending implementation |
| AC7   | TODO                   | Pending implementation |

## Risks and Trade-offs

- Reports add small process overhead. Keep entries concise and only require them when a folder-style spec is supplied.
- Reviewer writes can create uncommitted documentation changes. The workflow must make Committer ownership explicit.

## Implementation Completion Review

After implementation, record material report-persistence or reviewer-workflow discoveries in an
issue-local `implementation-retrospective.md`. If none occurred, add a concise progress-log entry
explaining why no retrospective is needed.

## References

- Parent EPIC: #2003
- GitHub issue: #2160
- Reviewer profiles: `.github/agents/complexity-auditor.agent.md`, `.github/agents/task-reviewer.agent.md`, `.github/agents/pr-reviewer.agent.md`
