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
last-updated-utc: 2026-09-11
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/write-markdown-docs/SKILL.md
    - docs/AGENTS.md
    - docs/issues/README.md
    - docs/issues/drafts/README.md
    - docs/issues/open/README.md
    - docs/templates/ISSUE.md
    - docs/templates/EPIC.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

# Issue #2159 - Adopt Folder-Style Issue Specifications

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Make folder-style specifications the only layout for new issues and EPICs, while retaining existing
single-file specifications as historical artifacts except for the explicitly approved, bounded
migration of three inactive open issue specifications.

## Background

New issue specs need room for verification evidence, reports, plans, and retrospectives. The
repository's templates and most current guidance already use folders, but some long-lived guidance
still presents single files as a current alternative or requires incidental migration when a legacy
spec is touched. This creates avoidable churn and ambiguous expectations.

## Scope

### In Scope

- Create a root ADR recording the prospective folder-only policy and no-bulk-migration decision.
- Update issue creation, Markdown, and issue-lifecycle guidance to require a folder for every new
  issue or EPIC spec.
- Remove single-file paths from current creation guidance while preserving them as legacy examples
  only where historical accuracy requires it.
- Update templates and indexes so issue-local artifacts are expected to live beside `ISSUE.md` or
  `EPIC.md`.
- Migrate these inactive open single-file issue specifications to folder-style layout:
  - `docs/issues/open/1768-refactor-update-dependencies-skill-automation/ISSUE.md`
  - `docs/issues/open/1774-automate-cleanup-completed-issues-skill-script/ISSUE.md`
  - `docs/issues/open/1843-migrate-git-hooks-scripts-from-bash-to-rust/ISSUE.md`

### Out of Scope

- Bulk or incidental migration of existing single-file issue specs.
- Changing the separate refactor-plan lifecycle unless it is explicitly included after review.
- Building automated migration tooling.

### Approved Bounded Migration Exception

The three named open specifications are an explicit exception to the no-bulk-migration policy. The
maintainer confirmed that all three remain open and that no contributor is currently working on
them. Migrate only their primary-spec layout from a single Markdown file to a same-named directory
containing `ISSUE.md`; preserve each specification's content, GitHub issue identity, lifecycle
state, and references, then repair live links to the new path.

This exception does not authorize migration of any other existing single-file specification. Closed
single-file specifications remain historical artifacts, and any additional open-spec migration
requires a separately recorded reason and maintainer approval.

## Architectural Decisions

- Related ADR: `docs/adrs/20260830124000_place_adrs_by_decision_scope.md`
- ADRs to create: Adopt prospective folder-style issue and EPIC specifications.

## Implementation Plan

| ID  | Status | Task                              | Notes / Expected Output                                                                                       |
| --- | ------ | --------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Identify conflicting guidance     | Inventory active paths, the approved exception, and migration requirements.                                   |
| T2  | TODO   | Record the ADR                    | Define scope, legacy treatment, and consequences.                                                             |
| T3  | TODO   | Align templates and workflows     | Update current issue-spec documentation and indexes.                                                          |
| T4  | TODO   | Validate new and legacy scenarios | Prove new specs are folders, the three excepted open specs migrated, and other legacy specs remain untouched. |

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

## Acceptance Criteria

- [ ] A root ADR requires folder-style layout for new issue and EPIC specs and rejects bulk migration.
- [ ] Active issue creation guidance and templates no longer present single-file specs as a new-spec option.
- [ ] Existing single-file specs remain valid historical records and are not migrated by this change, except for the three explicitly named inactive open specifications.
- [ ] New issue-local artifacts have a clear home beside the folder's primary spec file.
- [ ] `linter all` exits with code `0` and relevant documentation checks pass.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario                              | Command/Steps                                                                           | Expected Result                                                                                              | Status | Evidence               |
| --- | ------------------------------------- | --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ | ------ | ---------------------- |
| M1  | Draft a new spec                      | Follow the updated creation workflow.                                                   | The only offered primary-spec path is a folder with `ISSUE.md` or `EPIC.md`.                                 | TODO   | Pending implementation |
| M2  | Review legacy and excepted open specs | Inspect a retained legacy single-file spec and the three approved open-spec migrations. | The retained legacy spec remains unchanged; each named inactive open spec is a folder containing `ISSUE.md`. | TODO   | Pending implementation |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence               |
| ----- | ---------------------- | ---------------------- |
| AC1   | TODO                   | Pending implementation |
| AC2   | TODO                   | Pending implementation |
| AC3   | TODO                   | Pending implementation |
| AC4   | TODO                   | Pending implementation |
| AC5   | TODO                   | Pending implementation |

## Risks and Trade-offs

- Retaining legacy layouts preserves historical inconsistency but avoids documentation-only churn.
- The policy must not silently alter refactor-plan conventions without an explicit scope decision.

## Implementation Completion Review

After implementation, record material legacy-layout or migration-policy discoveries in an
issue-local `implementation-retrospective.md`. If none occurred, add a concise progress-log entry
explaining why no retrospective is needed.

## References

- Parent EPIC: #2003
- GitHub issue: #2159
