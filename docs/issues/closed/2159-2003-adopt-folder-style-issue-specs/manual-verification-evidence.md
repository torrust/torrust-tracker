<!-- markdownlint-disable MD003 -->
issue-spec: docs/issues/closed/2159-2003-adopt-folder-style-issue-specs/ISSUE.md
last-updated-utc: 2026-09-25
last-updated-utc: 2026-09-18 11:05
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-09-18 11:00-11:05
- Artifact under test: Folder-style durable documentation artifact record policy.
- Operating system / environment: Linux repository workspace.
- Prerequisites and setup performed: Migrated selected durable records with `git mv` in bounded
  batches, then repaired live Markdown links.

## Verification Processes

### V1 - Create Durable Record Families

- Goal: Confirm that current authoring guidance provides only folder-style primary-record paths.
- Initial state: Updated creation workflows and templates for issue/EPIC specs, refactor plans,
  and PR-review audits.
- Status: `DONE`

#### Steps Performed

1. Reviewed the updated issue, refactor-plan, and PR-review workflows and their templates.
2. Confirmed their canonical primary paths use `ISSUE.md`, `EPIC.md`, `REFACTOR-PLAN.md`, and
   `PR-REVIEW.md` within a record directory.

#### Observed Result

```text
docs/issues/open/<number>-<short-slug>/ISSUE.md or EPIC.md
docs/refactor-plans/<lifecycle>/<short-slug>/REFACTOR-PLAN.md
docs/pr-reviews/pr-<number>-<suffix>/PR-REVIEW.md
```

#### Conclusion

The updated authoring surfaces provide folder-style paths for all selected durable record
families, satisfying manual scenario M1.

### V2 - Inspect Migrated Record Archives

- Goal: Confirm that selected existing records have canonical folder-style primary files and no
  flat records remain.
- Initial state: Archive migrations for issue/EPIC specs, refactor plans, and PR-review audits
  completed in bounded batches.
- Status: `DONE`

#### Steps Performed

1. Opened representative draft, open, and closed issue/EPIC records; closed refactor plans; and
  PR-review audit records in their lifecycle directories.
2. Inspected each directory and its primary record to confirm that the primary filename and
  companion artifacts are colocated.

#### Observed Result

```text
docs/issues/drafts/refactor-semantic-link-conventions/EPIC.md
docs/issues/closed/2159-2003-adopt-folder-style-issue-specs/ISSUE.md
docs/issues/closed/1029-do-not-publish-docker-tags-with-v-prefix/ISSUE.md
docs/refactor-plans/closed/2238-refactor-native-tracker-test-fixture/REFACTOR-PLAN.md
docs/pr-reviews/pr-2252-review/PR-REVIEW.md
```

#### Conclusion

The inspected records use their canonical folder-style primary file, and their companion artifacts
are colocated with the primary record. Automated count and link validation are recorded separately
in the issue progress log and acceptance evidence. This satisfies manual scenario M2.

### V3 - Review Excluded Document Families

- Goal: Confirm that the policy does not introduce folders for document families without durable
  primary records and companion artifacts.
- Initial state: Root ADR added for this repository-wide convention.
- Status: `DONE`

#### Steps Performed

1. Reviewed `20260918093757_adopt_folder_style_documentation_artifact_records.md`.
2. Confirmed the ADR retains existing family-specific layouts for reference pages, templates,
   ADRs, analyses, guides, and media.

#### Observed Result

```text
Do not add a directory solely because a document is Markdown: stable reference pages,
templates, ADRs, one-off analyses, guides, and media retain their existing family-specific layouts.
```

#### Conclusion

The policy remains bounded to durable record families and satisfies manual scenario M3.

## Failures and Follow-up

Initial broad relative-link rewriting affected already-folder-style draft records. The migration
was corrected by limiting relative-link changes to each selected source batch and rerunning local
link validation. No remaining follow-up is required.
