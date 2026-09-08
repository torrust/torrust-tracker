---
doc-type: issue
issue-type: enhancement
status: in-review
priority: p2
epic: 2003
github-issue: 2156
spec-path: docs/issues/open/2156-2003-create-markdown-template-skill/ISSUE.md
branch: "2156-2003-create-markdown-template-skill"
related-pr: 2170
last-updated-utc: 2026-09-08 12:25
semantic-links:
  skill-links:
    - create-issue
    - add-new-skill
    - write-markdown-docs
  related-artifacts:
    - .github/skills/add-new-skill/SKILL.md
    - .github/skills/dev/planning/write-markdown-docs/SKILL.md
    - docs/templates/
    - docs/index.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

# Issue #2156 - Create Markdown Template Skill

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Provide repository-owned guidance that places reusable Markdown templates in `docs/templates/` and
makes concrete documents link to those templates instead of duplicating their full reusable body.

## Background

A reusable template embedded inside concrete documentation is difficult to find, maintain, and
link. `docs/templates/` already contains canonical templates, but no skill governs template
creation and the boundary between a reusable template and a useful illustrative excerpt.

## Scope

### In Scope

- Create a `create-markdown-template` skill under `.github/skills/`.
- Define `docs/templates/` as the canonical location for reusable repository Markdown templates.
- Define naming, frontmatter, semantic-link, indexing, and validation requirements.
- Require concrete documents to link to canonical templates instead of copying full reusable
  template bodies.
- Permit short illustrative excerpts and immutable GitHub/external-surface examples where a
  repository link cannot serve the reader.
- Inventory template-like content in agent skills and documentation, moving only clear reusable
  templates in scope; record deliberately retained examples with rationale.

### Out of Scope

- Rewriting all existing documentation that happens to include Markdown snippets.
- Creating templates for one-off issue evidence or immutable historical records.
- Building a generic document-generation tool.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: None. This is a repository documentation convention.

## Implementation Plan

| ID  | Status | Task                                   | Notes / Expected Output                                                                    |
| --- | ------ | -------------------------------------- | ------------------------------------------------------------------------------------------ |
| T1  | DONE   | Inventory reusable template candidates | Recorded extraction/retention decisions in `template-candidate-inventory.md`.              |
| T2  | DONE   | Create the skill                       | Added `.github/skills/dev/planning/create-markdown-template/SKILL.md`.                     |
| T3  | DONE   | Move or link in-scope templates        | Added `SECURITY-ANALYSIS.md` and linked live security authoring guidance.                  |
| T4  | DONE   | Index and validate                     | `linter all`, workspace documentation tests, and manual template/exceptions review passed. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-create-markdown-template-skill/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2156 created and issue number added to this spec
- [x] Implementation completed and verified
- [x] Acceptance criteria reviewed after implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Split from the combined AI-agent process draft into an independently implementable EPIC #2003 child specification - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2156, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2156
- 2026-09-08 10:00 UTC - GitHub Copilot - Inventoried template-like content, added the template-creation skill and canonical security-analysis template, and linked live security workflows without altering retained historical examples - Pending validation
- 2026-09-08 10:15 UTC - GitHub Copilot - Verified the canonical template extraction and retained-example boundaries; `linter all` and `cargo test --doc --workspace` passed - Awaiting independent task review
- 2026-09-08 10:46 UTC - GitHub Copilot (Task Reviewer) - Independently reviewed the implementation; verified all stated acceptance criteria but found an invalid semantic-link list entry in `docs/security/analysis/README.md` - Review failed pending indentation correction
- 2026-09-08 10:50 UTC - GitHub Copilot - Corrected both malformed semantic-link list entries in the security-analysis README; revalidation and focused re-review pending
- 2026-09-08 11:00 UTC - GitHub Copilot - Confirmed the prior whitespace change had not persisted, then corrected both semantic-link list entries in the current file; revalidation and focused re-review pending
- 2026-09-08 12:00 UTC - GitHub Copilot - Processing two valid Copilot suggestions on PR #2170. The first corrects the `add-new-skill` relative link in the new Markdown template skill - `docs/copilot-pr-reviews/pr-2170-copilot-suggestions.md`
- 2026-09-08 12:10 UTC - GitHub Copilot - Processing the second valid Copilot suggestion on PR #2170: align the security-analysis template identifier with the existing `cve-id` catalog convention - `docs/copilot-pr-reviews/pr-2170-copilot-suggestions.md`
- 2026-09-08 12:25 UTC - GitHub Copilot - Applied, validated, and committed the action fixes, then replied to and resolved both PR #2170 Copilot suggestions; tracker completion record pending its own commit - `docs/copilot-pr-reviews/pr-2170-copilot-suggestions.md`
- 2026-09-08 11:15 UTC - GitHub Copilot (Task Reviewer) - Verified the corrected YAML flow lists, full issue scope, and documented validation checks; all acceptance criteria pass and implementation completion is verified

## Acceptance Criteria

- [x] The repository has a documented `create-markdown-template` skill with the correct triggers.
- [x] Reusable Markdown templates are stored and indexed in `docs/templates/`.
- [x] The skill defines when documents must link rather than duplicate a template, and documented exceptions.
- [x] In-scope template-like content is classified, migrated, or retained with a recorded rationale.
- [x] `linter all` exits with code `0` and relevant documentation checks pass.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario           | Command/Steps                                                                                                      | Expected Result                                                                         | Status | Evidence                                                                                           |
| --- | ------------------ | ------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------- | ------ | -------------------------------------------------------------------------------------------------- |
| M1  | Create a template  | Followed the skill to extract the reusable security-analysis authoring structure.                                  | `docs/templates/SECURITY-ANALYSIS.md` is indexed and linked by live security workflows. | DONE   | `docs/templates/SECURITY-ANALYSIS.md`, `docs/index.md`, and security workflow links.               |
| M2  | Check an exception | Reviewed retained historical and illustrative template-like content recorded in `template-candidate-inventory.md`. | Retained records remain readable and were not altered as duplicate canonical templates. | DONE   | `template-candidate-inventory.md` decisions for closed issue evidence and Copilot review examples. |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                            |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | `.github/skills/dev/planning/create-markdown-template/SKILL.md` has the required name and explicit trigger phrases. |
| AC2   | DONE                   | `docs/templates/SECURITY-ANALYSIS.md` is indexed in `docs/index.md`.                                                |
| AC3   | DONE                   | The skill defines link-versus-duplicate rules and exceptions; live security workflows link to the template.         |
| AC4   | DONE                   | `template-candidate-inventory.md` records extraction/retention decisions; retained records are unchanged.           |
| AC5   | DONE                   | Re-review reran `linter all`, `cargo test --doc --workspace`, and `git diff --check` successfully on 2026-09-08.    |

## Risks and Trade-offs

- Excessive indirection can harm readability. Keep short examples when they teach a local point.
- Broad migration would create noisy documentation changes. Restrict changes to clear canonical templates.

## Implementation Completion Review

After implementation, record material template-classification discoveries or reusable workflow
lessons in an issue-local `implementation-retrospective.md`. If none occurred, add a concise
progress-log entry explaining why no retrospective is needed.

- Assessment: The inventory found one live authoring skeleton that needed extraction and several
  deliberately retained historical/examples. This was expected scope, with no material design
  change beyond the issue's planned template classification; no retrospective is needed.

## References

- Parent EPIC: #2003
- GitHub issue: #2156
- Template directory: `docs/templates/`
