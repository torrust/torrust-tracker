---
doc-type: issue
issue-type: enhancement
status: planned
priority: p2
epic: 2003
github-issue: 2156
spec-path: docs/issues/open/2156-2003-create-markdown-template-skill/ISSUE.md
branch: "2156-2003-create-markdown-template-skill"
related-pr: null
last-updated-utc: 2026-09-07 11:20
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

| ID  | Status | Task                                   | Notes / Expected Output                                                 |
| --- | ------ | -------------------------------------- | ----------------------------------------------------------------------- |
| T1  | TODO   | Inventory reusable template candidates | Classify template, example, and immutable record content.               |
| T2  | TODO   | Create the skill                       | Include triggers, workflow, exceptions, semantic links, and validation. |
| T3  | TODO   | Move or link in-scope templates        | Avoid unrelated documentation churn.                                    |
| T4  | TODO   | Index and validate                     | Update template discovery and run focused checks.                       |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-create-markdown-template-skill/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2156 created and issue number added to this spec
- [ ] Implementation completed and verified
- [ ] Acceptance criteria reviewed after implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Split from the combined AI-agent process draft into an independently implementable EPIC #2003 child specification - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2156, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2156

## Acceptance Criteria

- [ ] The repository has a documented `create-markdown-template` skill with the correct triggers.
- [ ] Reusable Markdown templates are stored and indexed in `docs/templates/`.
- [ ] The skill defines when documents must link rather than duplicate a template, and documented exceptions.
- [ ] In-scope template-like content is classified, migrated, or retained with a recorded rationale.
- [ ] `linter all` exits with code `0` and relevant documentation checks pass.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario           | Command/Steps                                                        | Expected Result                                                      | Status | Evidence               |
| --- | ------------------ | -------------------------------------------------------------------- | -------------------------------------------------------------------- | ------ | ---------------------- |
| M1  | Create a template  | Follow the new skill for a disposable template example.              | It is created in `docs/templates/`, indexed, and linked correctly.   | TODO   | Pending implementation |
| M2  | Check an exception | Review a short illustrative snippet and an external-surface example. | Both remain readable without becoming duplicate canonical templates. | TODO   | Pending implementation |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence               |
| ----- | ---------------------- | ---------------------- |
| AC1   | TODO                   | Pending implementation |
| AC2   | TODO                   | Pending implementation |
| AC3   | TODO                   | Pending implementation |
| AC4   | TODO                   | Pending implementation |
| AC5   | TODO                   | Pending implementation |

## Risks and Trade-offs

- Excessive indirection can harm readability. Keep short examples when they teach a local point.
- Broad migration would create noisy documentation changes. Restrict changes to clear canonical templates.

## Implementation Completion Review

After implementation, record material template-classification discoveries or reusable workflow
lessons in an issue-local `implementation-retrospective.md`. If none occurred, add a concise
progress-log entry explaining why no retrospective is needed.

## References

- Parent EPIC: #2003
- GitHub issue: #2156
- Template directory: `docs/templates/`
