---
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2003
github-issue: 2158
spec-path: docs/issues/open/2158-2003-inventory-existing-clippy-allows/ISSUE.md
branch: "2158-2003-inventory-existing-clippy-allows"
related-pr: null
last-updated-utc: 2026-09-07 11:20
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - AGENTS.md
    - Cargo.toml
    - .github/agents/clippy-fixer.agent.md
    - .github/agents/committer.agent.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - src/
    - packages/
    - console/
    - tests/
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

# Issue #2158 - Inventory Existing Clippy Allows

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Inventory and explain every existing Clippy `allow` attribute in this specification's
`clippy-allow-inventory.md` artifact, then remove unjustified suppressions or create stable
follow-up work for justified temporary ones.

## Background

The workspace currently has approximately 215 `allow(clippy::...)` attributes, including 25
crate-level attributes. Prospective enforcement alone cannot explain this historical baseline.
Each existing suppression needs an explicit decision: retain with rationale, remove after a fix, or
retain temporarily with a tracked follow-up.

## Scope

### In Scope

- Inventory every item- and crate-level Clippy allow in maintained Rust source.
- Store the human-readable inventory in this issue's folder as
  `clippy-allow-inventory.md`. It is the audit record and moves with the issue specification; it
  is not the stable machine-readable baseline for prospective enforcement.
- Record lint name, source location, scope, rationale category, evidence, owner, and disposition.
- Add nearby rationale comments for retained allows.
- Remove allows that are no longer necessary and fix resulting diagnostics.
- Update `ClippyFixer` to declare the `edit` tool so it can apply focused remediation and nearby
  rationale comments; continue to delegate commits to `Committer`.
- For each temporary suppression, create an issue-spec draft in `docs/issues/drafts/` that states
  the removal condition and links the inventory entry. Request user approval before creating the
  follow-up GitHub issue, then replace the draft reference with the assigned issue number.

### Out of Scope

- Introducing the prospective enforcement mechanism for new allows; that belongs to the sibling policy issue.
- Using the issue-local Markdown inventory as the machine-readable prospective-enforcement
  baseline; the sibling policy issue owns that stable validation input.
- Broad refactoring unrelated to eliminating an unjustified suppression.
- Blanket suppression of Clippy warnings to finish the inventory.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: None. Create an ADR only if the inventory reveals a new repository-wide lint policy decision.

## Implementation Plan

| ID  | Status | Task                              | Notes / Expected Output                                                                                                    |
| --- | ------ | --------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Create the issue-local inventory  | Add `clippy-allow-inventory.md` with source roots, generation method, and a row for every item- and crate-level attribute. |
| T2  | TODO   | Enable ClippyFixer remediation    | Grant its profile the `edit` tool while retaining Committer ownership of signed commits.                                   |
| T3  | TODO   | Classify each allow               | Retain, remove, or temporary follow-up with evidence.                                                                      |
| T4  | TODO   | Draft and create follow-up issues | For each temporary suppression, follow the repository spec-first workflow before creating the linked GitHub issue.         |
| T5  | TODO   | Remediate in reviewable batches   | Use ClippyFixer for focused fixes; group commits by package or rationale category.                                         |
| T6  | TODO   | Validate final inventory          | Ensure no allow lacks a recorded disposition and relevant checks pass.                                                     |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-inventory-existing-clippy-allows/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2158 created and issue number added to this spec
- [ ] Issue-local `clippy-allow-inventory.md` created and completeness checked
- [ ] ClippyFixer has the `edit` tool and its documented commit delegation remains intact
- [ ] Draft specifications created and approved before every temporary-allow follow-up GitHub issue
- [ ] Implementation completed and verified
- [ ] Acceptance criteria reviewed after implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Created as separate remediation work for the existing Clippy-allow baseline - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2158, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2158

## Acceptance Criteria

- [ ] A durable inventory accounts for every Clippy allow in the agreed source roots.
- [ ] The inventory is stored as `clippy-allow-inventory.md` in this specification folder and
      identifies its source roots and generation/completeness method.
- [ ] ClippyFixer declares the `edit` tool and can apply the focused remediation required by an
      inventory disposition while Committer retains commit responsibility.
- [ ] Every retained allow has a nearby specific rationale and a matching inventory decision.
- [ ] Every temporary allow has a stable removal condition and a linked GitHub follow-up issue
      created only after its folder-style draft specification was approved.
- [ ] Every removed allow is accompanied by a focused fix and validation.
- [ ] The inventory contains no unclassified entries and no blanket exceptions.
- [ ] `linter all` exits with code `0` and relevant package tests pass.

## Verification Plan

### Automatic Checks

- `linter all`
- Inventory completeness check
- Relevant package tests for remediation batches
- Focused profile check for the ClippyFixer `edit` tool and Committer delegation

### Manual Verification Scenarios

| ID  | Scenario                           | Command/Steps                                                                                                           | Expected Result                                                                                            | Status | Evidence               |
| --- | ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ------ | ---------------------- |
| M1  | Compare source and inventory       | Regenerate the source list and compare it with `clippy-allow-inventory.md`.                                             | Every allow has one recorded decision.                                                                     | TODO   | Pending implementation |
| M2  | Sample retained and removed cases  | Review representative cases from each disposition category.                                                             | Rationale, source, and validation evidence agree.                                                          | TODO   | Pending implementation |
| M3  | Create a temporary-allow follow-up | Select a temporary entry, draft its folder-style follow-up specification, obtain approval, and create its GitHub issue. | The inventory records the resulting issue number; no GitHub issue is created before its draft is approved. | TODO   | Pending implementation |
| M4  | Use ClippyFixer for remediation    | Provide a removable or retainable fixture finding to ClippyFixer.                                                       | The agent can edit the focused files, documents the decision, and delegates the commit to Committer.       | TODO   | Pending implementation |

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
| AC8   | TODO                   | Pending implementation |

## Risks and Trade-offs

- The inventory is substantial. Batch by package and preserve one complete machine-checkable inventory.
- Some suppressions may be macro-generated or technically required. Retain them only with concrete evidence.
- The issue-local inventory is intentionally a human audit record. Do not make the later
  prospective validator depend on a path that moves when this issue closes.
- Temporary follow-ups increase issue count, but they prevent suppressions from becoming permanent
  undocumented debt. Group only genuinely inseparable fixes in one follow-up specification.

## Implementation Completion Review

After implementation, record material inventory, remediation, or follow-up workflow lessons in an
issue-local `implementation-retrospective.md`. If none occurred, add a concise progress-log entry
explaining why no retrospective is needed.

## References

- Parent EPIC: #2003
- GitHub issue: #2158
- Prospective policy: #2157 - Require Documented Clippy Allows
  (`docs/issues/open/2157-2003-require-documented-clippy-allows/ISSUE.md`)
- Inventory artifact: `clippy-allow-inventory.md` in this issue specification folder
- Remediation agent: `.github/agents/clippy-fixer.agent.md`
