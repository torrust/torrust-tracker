---
doc-type: issue
issue-type: enhancement
status: planned
priority: p2
epic: 2003
github-issue: 2159
spec-path: docs/issues/open/2159-2003-adopt-folder-style-issue-specs/ISSUE.md
branch: "2159-2003-adopt-folder-style-issue-specs"
related-pr: null
last-updated-utc: 2026-09-18
semantic-links:
  skill-links:
    - create-issue
    - create-refactor-plan
    - process-pr-review
    - write-markdown-docs
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/create-refactor-plan/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - .github/skills/dev/planning/write-markdown-docs/SKILL.md
    - docs/AGENTS.md
    - docs/issues/README.md
    - docs/issues/drafts/README.md
    - docs/issues/open/README.md
    - docs/refactor-plans/
    - docs/pr-reviews/README.md
    - docs/templates/ISSUE.md
    - docs/templates/EPIC.md
    - docs/templates/REFACTOR-PLAN.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

# Issue #2159 - Adopt Folder-Style Documentation Artifact Records

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Make folder-style layout the only format for new durable documentation artifact records that can
accumulate related evidence, plans, reports, or retrospectives. Initially, this includes issues,
EPICs, refactor plans, and pull-request review audits. Retain existing single-file records as
historical artifacts only when migration would not be safe or useful; migrate the current records
in these families to establish one consistent, extensible layout.

## Background

Durable records need room for verification evidence, reports, plans, and retrospectives. Issue
specs, refactor plans, and pull-request review audits all have this property: each has one primary
record plus artifacts that document the record's lifecycle. The repository's templates and most
current issue guidance already use folders, but some long-lived guidance still presents single
files as a current alternative or requires incidental migration when a legacy record is touched.
The current refactor-plan and PR-review archives are also flat, despite both being durable record
families. This creates avoidable churn and ambiguous expectations.

Folder style is not appropriate for every document. Stable reference pages, one-off analyses,
ADRs, templates, and media retain their existing family-specific layouts unless a later decision
identifies a durable primary record and a concrete need for colocated artifacts.

## Scope

### In Scope

- Create a root ADR recording the folder-style policy for durable documentation records and its
  deliberate migration of existing records in the selected families.
- Define the classification rule: a document family uses folder style when a primary record owns
  lifecycle-specific companion artifacts; standalone references, templates, and media do not.
- Update issue creation, refactor-plan creation, pull-request review, Markdown, and lifecycle
  guidance to require folders for all new issue specs, EPICs, refactor plans, and review audits.
- Remove single-file paths from current creation guidance while preserving them as legacy examples
  only where historical accuracy requires it.
- Update templates and indexes so companion artifacts live beside the family's primary file:
  `ISSUE.md`, `EPIC.md`, `REFACTOR-PLAN.md`, or `PR-REVIEW.md`.
- Migrate all current single-file issue and EPIC specs, refactor plans, and PR-review audits to
  folder-style layout using Git renames. Preserve each record's contents, identity, lifecycle
  state, and links; repair every live reference to its new primary-file path.
- The already completed migration includes these inactive open issue specifications:
  - `docs/issues/open/1768-refactor-update-dependencies-skill-automation/ISSUE.md`
  - `docs/issues/open/1774-automate-cleanup-completed-issues-skill-script/ISSUE.md`
  - `docs/issues/open/1843-migrate-git-hooks-scripts-from-bash-to-rust/ISSUE.md`

### Out of Scope

- Changing any artifact family's lifecycle states, retention policy, or substantive content.
- Converting every document, template, ADR, analysis, guide, or media item to a folder.
- Building automated migration tooling.

### Migration Contract

The current legacy records in the selected families are deliberately in scope for migration. Move
each record into a directory named after its existing stem and rename the primary document to the
family's primary filename: `ISSUE.md`, `EPIC.md`, `REFACTOR-PLAN.md`, or `PR-REVIEW.md`. For
historical PR-review duplicates, retain their legacy suffix in the directory name and use
`PR-REVIEW.md` as the primary file.

Use `git mv` so history remains traceable. Do not alter record content except for required
frontmatter path updates, directory-local relative links, and repository references that would
otherwise become stale. Do not migrate a record with concurrent contributor changes without first
coordinating the move.

## Architectural Decisions

- Related ADR: `docs/adrs/20260830124000_place_adrs_by_decision_scope.md`
- ADRs to create: Adopt folder-style durable documentation artifact records and migrate existing
  records in selected families.

## Implementation Plan

| ID  | Status | Task                              | Notes / Expected Output                                                                                                     |
| --- | ------ | --------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Inventory migration candidates    | Identify all legacy records in selected families, their current paths, inbound links, and concurrent work risks.            |
| T2  | TODO   | Record the ADR                    | Define classification, migration contract, and consequences.                                                                  |
| T3  | TODO   | Align templates and workflows     | Update issue, refactor-plan, and PR-review documentation, templates, indexes, and creation paths.                           |
| T4  | TODO   | Migrate durable record archives   | Move all selected legacy records with Git renames and repair live references.                                                 |
| T5  | TODO   | Validate folder and link layouts  | Prove new and migrated records use folders, links resolve, and excluded families remain unchanged.                           |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-adopt-folder-style-issue-specs/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2159 created and issue number added to this spec
- [ ] Implementation completed and verified
- [ ] Acceptance criteria reviewed after implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Created as an immediately implementable EPIC #2003 child specification - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2159, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2159
- 2026-09-11 - Maintainer - Approved a bounded exception for the three named open single-file specifications because they remain open and are known to have no active contributor work. Their migration is aligned with the prospective folder-only convention; all other existing single-file specifications remain outside this issue's migration scope.
- 2026-09-11 - GitHub Copilot - Migrated the three approved inactive open specifications (#1768, #1774, and #1843) into same-named folders containing `ISSUE.md`, preserved their lifecycle state, and repaired current references. The broader folder-only policy, ADR, and workflow/template alignment remain planned work.
- 2026-09-18 - Maintainer - Expanded the prospective policy from issue and EPIC specifications to durable documentation artifact records, initially including refactor plans and pull-request review audits. The policy must retain family-specific layouts for document classes without lifecycle-specific companion artifacts.
- 2026-09-18 - Maintainer - Authorized migration of current records that do not follow the selected families' folder-style convention, replacing the earlier no-bulk-migration constraint. Preserve content and history through Git renames, and repair live references.

## Acceptance Criteria

- [ ] A root ADR requires folder-style layout for new durable documentation artifact records,
  defines the classification rule, and records the migration contract.
- [ ] Active issue, refactor-plan, and PR-review creation guidance and templates no longer present
  single-file records as a new-record option.
- [ ] All current issue and EPIC specs, refactor plans, and PR-review audit records use the
  folder-style layout, preserving their content, identity, lifecycle state, and Git history.
- [ ] New companion artifacts have a clear home beside their folder's primary record file.
- [ ] Standalone reference pages, templates, ADRs, analyses, guides, and media retain their
  family-specific layouts unless separately justified.
- [ ] `linter all` exits with code `0` and relevant documentation checks pass.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario                                 | Command/Steps                                                                             | Expected Result                                                                                                                   | Status | Evidence               |
| --- | ---------------------------------------- | ----------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | ------ | ---------------------- |
| M1  | Create each durable record family        | Follow the updated issue, refactor-plan, and PR-review workflows.                        | Each new primary record is in a folder, with its prescribed primary filename and space for companion artifacts.                  | TODO   | Pending implementation |
| M2  | Review migrated record archives          | Inspect issue, refactor-plan, and PR-review record families after migration.              | Every selected existing record is a folder with its prescribed primary filename and repaired live references.                    | TODO   | Pending implementation |
| M3  | Review excluded document families        | Inspect the ADR, template, analysis, guide, and media placement guidance.                 | The policy explicitly preserves family-specific layouts for records without lifecycle-specific companion artifacts.              | TODO   | Pending implementation |

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

- Archive migration can create a large path-only diff and must be isolated from content changes so
  reviews can distinguish relocations from substantive edits.
- Applying folder style indiscriminately would add nesting without a lifecycle or companion-artifact
  benefit; the ADR's classification rule must keep the policy bounded.

## Implementation Completion Review

After implementation, record material legacy-layout or migration-policy discoveries in an
issue-local `implementation-retrospective.md`. If none occurred, add a concise progress-log entry
explaining why no retrospective is needed.

## References

- Parent EPIC: #2003
- GitHub issue: #2159
