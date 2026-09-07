---
doc-type: issue
issue-type: enhancement
status: in-review
priority: p2
epic: 2003
github-issue: 2155
spec-path: docs/issues/open/2155-2003-document-ai-agent-orchestration/ISSUE.md
branch: "2155-2003-document-ai-agent-orchestration"
related-pr: 2163
last-updated-utc: 2026-09-07 15:35
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - .github/agents/
    - .github/agents/README.md
    - docs/AGENTS.md
    - docs/index.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

# Issue #2155 - Document AI Agent Orchestration

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Create the durable, repository-owned model of the workflow between custom AI agents, including
their prerequisites, handoffs, feedback loops, artifacts, and terminal states.

## Background

Custom agent profiles define their individual responsibilities but do not currently present one
complete workflow. This makes intended dependencies implicit and prevents later orchestration
enforcement from being based on reviewed, observed process documentation.

## Scope

### In Scope

- Create `docs/agents/README.md` and `docs/agents/orchestration.md` as the canonical location for
  AI-agent process documentation, separate from runtime architecture and profile definitions.
- Add a Mermaid flowchart covering every custom agent: Planner, Implementer, Complexity Auditor,
  Task Reviewer, Committer, PR Reviewer, Copilot Suggestions Handler, Researcher, GitHub Operator,
  and ClippyFixer.
- Document required versus optional/event-driven handoffs, review and remediation loops, artifacts,
  and terminal states.
- Link the documentation from `docs/index.md` and `.github/agents/README.md`.
- State that this is a documented process model, not technical transition enforcement.

### Out of Scope

- Implementing a state machine, graph database, external workflow tool, or transition gate.
- Selecting an enforcement architecture before workflow use produces evidence.
- Changing individual agent responsibilities except to correct an evidenced mismatch with the
  documented current workflow.

## Architectural Decisions

- Related ADR: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: None. A future enforcement mechanism requires its own evidence-backed ADR.

## Implementation Plan

| ID  | Status | Task                             | Notes / Expected Output                                                                                                 |
| --- | ------ | -------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Inventory profile workflows      | Traced every `.github/agents/*.agent.md` profile and relevant referenced skills.                                        |
| T2  | DONE   | Write orchestration guide        | Added `docs/agents/orchestration.md` with authority boundaries and artifact ownership.                                  |
| T3  | DONE   | Add and verify Mermaid flowchart | Added an overview plus focused implementation and pull-request workflow diagrams with profile-grounded edge semantics.  |
| T4  | DONE   | Index the guide                  | Added `docs/agents/README.md` and updated documentation and agent-profile indexes.                                      |
| T5  | DONE   | Validate documentation           | All three Mermaid diagrams passed syntax validation and preview; `linter all` and workspace documentation tests passed. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-document-ai-agent-orchestration/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2155 created and issue number added to this spec
- [x] Implementation completed and verified
- [x] Acceptance criteria reviewed after implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Split from the combined AI-agent process draft into an independently implementable EPIC #2003 child specification - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2155, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2155
- 2026-09-07 12:40 UTC - GitHub Copilot - Inventoried all custom agent profiles and added the repository-owned orchestration documentation, index, and Mermaid flowchart - Pending validation
- 2026-09-07 12:45 UTC - GitHub Copilot - Validated Mermaid syntax and preview; `linter all` and `cargo test --doc --workspace` passed; awaiting independent task review - Local validation
- 2026-09-07 13:00 UTC - Task Reviewer - Independently verified AC1, AC4, and AC5. AC2 and AC3 remain unverified because the diagram labels user-to-Planner and commit-to-pull-request transitions mandatory without profile support. AC6 remains unverified because `linter all` fails CSpell; `cargo test --doc --workspace` passes. No implementation-completion assessment explains why a retrospective was not needed - Independent task review
- 2026-09-07 13:10 UTC - GitHub Copilot - Corrected unsupported mandatory transitions, added the `docs/agents/` directory-map entry, and created a retrospective for the review finding - Pending revalidation
- 2026-09-07 13:20 UTC - GitHub Copilot - Reclassified the remaining unsupported approval and PR-stage transitions as conditional after the independent re-review - Pending final revalidation
- 2026-09-07 13:30 UTC - GitHub Copilot - Expanded the guide with focused implementation and pull-request workflow diagrams while retaining the global overview and authoritative handoff tables - Pending final revalidation
- 2026-09-07 15:00 UTC - GitHub Copilot - Validated all three Mermaid diagrams with syntax checks and previews; `linter all` and `cargo test --doc --workspace` passed - Awaiting final independent task review
- 2026-09-07 15:30 UTC - GitHub Copilot - Processing a valid Copilot suggestion on PR #2163: replace move-prone issue-spec semantic-link paths with stable issue references - `docs/copilot-pr-reviews/pr-2163-copilot-suggestions.md`
- 2026-09-07 15:35 UTC - GitHub Copilot - Applied and committed the semantic-link fix, then replied to and resolved the PR #2163 Copilot suggestion; tracker completion record pending its own commit - https://github.com/torrust/torrust-tracker/pull/2163#discussion_r3951125417
- 2026-09-07 15:00 UTC - Task Reviewer - Independently verified all acceptance criteria against the working-tree documentation diff, all ten custom agent profiles, and rendered Mermaid diagrams - REVIEW PASSED

## Acceptance Criteria

- [x] `docs/agents/` is the discoverable canonical documentation collection for AI-agent workflows.
- [x] The Mermaid flowchart includes every custom profile and its documented handoffs.
- [x] The guide distinguishes required handoffs from optional/event-driven paths and identifies
      issue specifications, retrospectives, review reports, commits, and Copilot-thread trackers.
- [x] Feedback loops and terminal states are explicit.
- [x] The guide declares that it documents current expectations only and reserves enforcement-tool
      selection for a separately evidenced ADR.
- [x] `linter all` exits with code `0` and relevant documentation checks pass.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario             | Command/Steps                                                                 | Expected Result                                      | Status | Evidence                                                                                        |
| --- | -------------------- | ----------------------------------------------------------------------------- | ---------------------------------------------------- | ------ | ----------------------------------------------------------------------------------------------- |
| M1  | Trace every profile  | Compared the guide and all diagrams to each `.github/agents/*.agent.md` file. | Every profile and handoff is represented accurately. | DONE   | Profile inventory, edge reclassification, and final independent review requested on 2026-09-07. |
| M2  | Render the flowchart | Opened `docs/agents/orchestration.md` in a Mermaid preview.                   | All diagrams render and labels remain readable.      | DONE   | Overview and both focused diagrams passed syntax validation and preview on 2026-09-07.          |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                                                                                                                                                                    |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | `docs/agents/README.md`, `docs/index.md`, and `.github/agents/README.md` provide discoverable canonical entry points; semantic-link targets exist.                                                                                                                                          |
| AC2   | DONE                   | Final independent review matched all ten profiles to the overview diagram and handoff tables. Solid delivery edges trace to declared Planner, Implementer, Complexity Auditor, Task Reviewer, Committer, and Copilot Suggestions Handler workflows; unsupported paths are dashed.           |
| AC3   | DONE                   | Final independent review verified the legend, required and event-driven tables, and artifact-ownership table distinguish declared mandatory handoffs from conditional paths and identify issue specifications, retrospectives, review reports, signed commits, and Copilot-thread trackers. |
| AC4   | DONE                   | The flowchart explicitly shows revision, audit, task-review, and review-feedback loops, plus the `Merged` terminal state.                                                                                                                                                                   |
| AC5   | DONE                   | The guide expressly describes a non-enforcing model and requires a separately evidenced ADR before enforcement-tool selection.                                                                                                                                                              |
| AC6   | DONE                   | `linter all` and `cargo test --doc --workspace` passed on 2026-09-07; the independent re-run of `linter all` included successful CSpell and Markdown checks.                                                                                                                                |

## Risks and Trade-offs

- The guide can drift from individual profiles. Link the source profiles and review the guide when
  a profile's workflow changes.
- A diagram can be mistaken for a technical gate. State its documentation-only status explicitly.

## Implementation Completion Review

After implementation, compare the delivered guide with this specification and record material
workflow discoveries, profile mismatches, and reusable lessons. Create an issue-local
`implementation-retrospective.md` when those findings are material; otherwise add a concise
progress-log entry explaining why no retrospective is needed.

## References

- Parent EPIC: #2003
- GitHub issue: #2155
- Agent profiles: `.github/agents/`
