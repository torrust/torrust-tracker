---
doc-type: issue
issue-type: enhancement
status: planned
priority: p2
epic: 2003
github-issue: 2155
spec-path: docs/issues/open/2155-2003-document-ai-agent-orchestration/ISSUE.md
branch: "2155-2003-document-ai-agent-orchestration"
related-pr: null
last-updated-utc: 2026-09-07 11:20
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

| ID  | Status | Task                             | Notes / Expected Output                                                                   |
| --- | ------ | -------------------------------- | ----------------------------------------------------------------------------------------- |
| T1  | TODO   | Inventory profile workflows      | Trace every `.github/agents/*.agent.md` profile and its referenced skills.                |
| T2  | TODO   | Write orchestration guide        | Document phases, prerequisites, artifacts, exceptions, and authority boundaries.          |
| T3  | TODO   | Add and verify Mermaid flowchart | Include all profiles, feedback loops, required/optional distinction, and terminal states. |
| T4  | TODO   | Index the guide                  | Update the documentation and agent-profile indexes.                                       |
| T5  | TODO   | Validate documentation           | Run required checks and manually compare the guide to all profile definitions.            |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-document-ai-agent-orchestration/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2155 created and issue number added to this spec
- [ ] Implementation completed and verified
- [ ] Acceptance criteria reviewed after implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Split from the combined AI-agent process draft into an independently implementable EPIC #2003 child specification - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2155, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2155

## Acceptance Criteria

- [ ] `docs/agents/` is the discoverable canonical documentation collection for AI-agent workflows.
- [ ] The Mermaid flowchart includes every custom profile and its documented handoffs.
- [ ] The guide distinguishes required handoffs from optional/event-driven paths and identifies
      issue specifications, retrospectives, review reports, commits, and Copilot-thread trackers.
- [ ] Feedback loops and terminal states are explicit.
- [ ] The guide declares that it documents current expectations only and reserves enforcement-tool
      selection for a separately evidenced ADR.
- [ ] `linter all` exits with code `0` and relevant documentation checks pass.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario             | Command/Steps                                                             | Expected Result                                      | Status | Evidence               |
| --- | -------------------- | ------------------------------------------------------------------------- | ---------------------------------------------------- | ------ | ---------------------- |
| M1  | Trace every profile  | Compare the guide and flowchart to each `.github/agents/*.agent.md` file. | Every profile and handoff is represented accurately. | TODO   | Pending implementation |
| M2  | Render the flowchart | Open the documentation in a Mermaid-capable Markdown renderer.            | The flowchart renders and labels remain readable.    | TODO   | Pending implementation |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence               |
| ----- | ---------------------- | ---------------------- |
| AC1   | TODO                   | Pending implementation |
| AC2   | TODO                   | Pending implementation |
| AC3   | TODO                   | Pending implementation |
| AC4   | TODO                   | Pending implementation |
| AC5   | TODO                   | Pending implementation |
| AC6   | TODO                   | Pending implementation |

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
