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
    - docs/issues/open/2159-2003-adopt-folder-style-issue-specs/migration-inventory.md
    - docs/issues/open/2159-2003-adopt-folder-style-issue-specs/manual-verification-evidence.md
    - docs/adrs/20260918093757_adopt_folder_style_documentation_artifact_records.md
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

- Related ADRs: `docs/adrs/20260830124000_place_adrs_by_decision_scope.md` and
  `docs/adrs/20260918093757_adopt_folder_style_documentation_artifact_records.md`
- ADRs to create: None.

## Implementation Plan

| ID  | Status | Task                              | Notes / Expected Output                                                                                                     |
| --- | ------ | --------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Inventory migration candidates    | Recorded 112 flat issue/EPIC specs, 3 refactor plans, 91 PR-review audits, link-repair targets, and batch order.            |
| T2  | DONE   | Record the ADR                    | Added root ADR 20260918093757 defining classification, migration contract, and consequences.                                |
| T3  | DONE   | Align templates and workflows     | Updated creation paths, templates, navigation, and filename guidance; retained only historical flat-path references.         |
| T4  | DONE   | Migrate durable record archives   | Migrated 226 issue/EPIC specs, 3 refactor plans, and 91 PR-review audits with Git-preserving record moves.                  |
| T5  | DONE   | Validate folder and link layouts  | Zero flat selected records; manual archive inspection, Rust audit-contract check, and `linter all` pass.                    |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-adopt-folder-style-issue-specs/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2159 created and issue number added to this spec
- [x] Implementation completed and verified
- [x] Acceptance criteria reviewed after implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Created as an immediately implementable EPIC #2003 child specification - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2159, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2159
- 2026-09-11 - Maintainer - Approved a bounded exception for the three named open single-file specifications because they remain open and are known to have no active contributor work. Their migration is aligned with the prospective folder-only convention; all other existing single-file specifications remain outside this issue's migration scope.
- 2026-09-11 - GitHub Copilot - Migrated the three approved inactive open specifications (#1768, #1774, and #1843) into same-named folders containing `ISSUE.md`, preserved their lifecycle state, and repaired current references. The broader folder-only policy, ADR, and workflow/template alignment remain planned work.
- 2026-09-18 - Maintainer - Expanded the prospective policy from issue and EPIC specifications to durable documentation artifact records, initially including refactor plans and pull-request review audits. The policy must retain family-specific layouts for document classes without lifecycle-specific companion artifacts.
- 2026-09-18 - Maintainer - Authorized migration of current records that do not follow the selected families' folder-style convention, replacing the earlier no-bulk-migration constraint. Preserve content and history through Git renames, and repair live references.
- 2026-09-18 - GitHub Copilot - Completed T1. `migration-inventory.md` records 112 flat issue/EPIC specs, 3 refactor plans, and 91 PR-review audits, plus the live convention surfaces and inbound PR-review references that require link repair.
- 2026-09-18 - GitHub Copilot - Completed T2. Added root ADR `20260918093757_adopt_folder_style_documentation_artifact_records.md` and registered it in the root ADR index.
- 2026-09-18 - GitHub Copilot - Completed T3. Updated issue, refactor-plan, and PR-review authoring paths, templates, lifecycle navigation, filename guidance, and live semantic-link conventions. Flat PR-review paths remain only in closed historical records.
- 2026-09-18 - GitHub Copilot - Started T4. Migrated all three closed refactor plans to `REFACTOR-PLAN.md` record folders and repaired the native-fixture plan's live links and `spec-path` metadata. PR-review and issue/EPIC archive migrations remain.
- 2026-09-18 - GitHub Copilot - Continued T4. Migrated all 91 PR-review audits to `PR-REVIEW.md` record folders, repaired repository-relative and intra-archive links, and passed the review-audit contract test, Markdown lint, and local-link checks.
- 2026-09-18 - GitHub Copilot - Continued T4. Migrated all 9 flat draft issue and EPIC specifications to `ISSUE.md` or `EPIC.md` record folders, repaired inbound and relative links, and passed Markdown and local-link checks. The 103 flat closed issue/EPIC records remain.
- 2026-09-18 - GitHub Copilot - Completed T4. Migrated all 103 remaining closed issue and EPIC records in four Git-preserving batches, repaired live Markdown links, and verified zero flat records remain in the selected families: 226 issue/EPIC specs, 3 refactor plans, and 91 PR-review audits.
- 2026-09-18 - GitHub Copilot - Completed T5. `linter all` passed after the migration, including Markdown, Lychee local links, cspell, Clippy, rustfmt, and ShellCheck. The dedicated Rust PR-review audit-contract check also passed.
- 2026-09-18 - GitHub Copilot - Final validation passed: `cargo test --doc --workspace` completed successfully. Recorded the migration's relative-link repair lesson in `implementation-retrospective.md`.
- 2026-09-18 - GitHub Copilot - Remediated independent-review findings: corrected all stale primary-record `spec-path` metadata, recorded direct representative archive inspection, and replaced the tracked shell audit-contract test with a Rust workspace check.
- 2026-09-18 12:56 UTC - Task Reviewer - Final strict re-review passed: the Rust contract checker preserves every retired shell assertion, including newline-only wrapped text and frontmatter-scoped related-artifact parsing; the retrospective maps all eight scenarios concretely.
- 2026-09-18 - GitHub Copilot - Fixed the PR container-image build failure by adding the new `agent-review-report-contract` workspace member to the `Containerfile` cargo-chef manifest/stub wiring, `.dockerignore` exceptions, and nextest archive exclusions. Verified the failing `recipe` stage with `docker build --target recipe --file Containerfile .`.

## Acceptance Criteria

- [x] A root ADR requires folder-style layout for new durable documentation artifact records,
  defines the classification rule, and records the migration contract.
- [x] Active issue, refactor-plan, and PR-review creation guidance and templates no longer present
  single-file records as a new-record option.
- [x] All current issue and EPIC specs, refactor plans, and PR-review audit records use the
  folder-style layout, preserving their content, identity, lifecycle state, and Git history.
- [x] New companion artifacts have a clear home beside their folder's primary record file.
- [x] Standalone reference pages, templates, ADRs, analyses, guides, and media retain their
  family-specific layouts unless separately justified.
- [x] `linter all` exits with code `0` and relevant documentation checks pass.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario                                 | Command/Steps                                                                             | Expected Result                                                                                                                   | Status | Evidence               |
| --- | ---------------------------------------- | ----------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | ------ | ---------------------- |
| M1  | Create each durable record family        | Follow the updated issue, refactor-plan, and PR-review workflows.                        | Each new primary record is in a folder, with its prescribed primary filename and space for companion artifacts.                  | DONE   | Updated workflows and templates name only folder-style primary paths. |
| M2  | Review migrated record archives          | Inspect issue, refactor-plan, and PR-review record families after migration.              | Every selected existing record is a folder with its prescribed primary filename and repaired live references.                    | DONE   | Counts: 226 issue/EPIC, 3 refactor-plan, and 91 PR-review primary records; zero flat records. |
| M3  | Review excluded document families        | Inspect the ADR, template, analysis, guide, and media placement guidance.                 | The policy explicitly preserves family-specific layouts for records without lifecycle-specific companion artifacts.              | DONE   | ADR 20260918093757 retains family-specific layouts outside durable record families. |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence               |
| ----- | ---------------------- | ---------------------- |
| AC1   | DONE                   | `20260918093757_adopt_folder_style_documentation_artifact_records.md` defines scope, classification, and migration. |
| AC2   | DONE                   | Issue, refactor-plan, and PR-review workflows, templates, indexes, and navigation name folder-style paths. |
| AC3   | DONE                   | All primary-record `spec-path` values resolve to their current canonical paths; Git renames and zero flat selected records are verified. |
| AC4   | DONE                   | Canonical primary filenames and colocation are documented in the ADR and templates. |
| AC5   | DONE                   | ADR explicitly excludes document families without durable primary-record companion artifacts. |
| AC6   | DONE                   | `linter all` and `cargo test --doc --workspace` passed on 2026-09-18. |

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
