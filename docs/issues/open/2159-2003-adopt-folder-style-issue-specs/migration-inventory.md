---
semantic-links:
  skill-links:
    - create-issue
    - create-refactor-plan
    - process-pr-review
  related-artifacts:
    - docs/issues/
    - docs/refactor-plans/
    - docs/pr-reviews/
    - docs/AGENTS.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/create-refactor-plan/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# Folder-Style Migration Inventory

## Snapshot

Inventory captured on 2026-09-18 from the branch
`2159-2003-adopt-folder-style-issue-specs`.

| Record family | Flat primary records | Existing folder-style records | Migration batches |
| ------------- | -------------------- | ----------------------------- | ----------------- |
| Issue and EPIC specs | 112 | 38 | 9 drafts; 103 closed; no open records |
| Refactor plans | 3 | 0 | 3 closed records |
| PR-review audits | 91 | 0 | 91 archive records |

## Canonical Layouts

| Record family | Current flat form | Target folder-style form |
| ------------- | ----------------- | ------------------------ |
| Issue spec | `docs/issues/<lifecycle>/<slug>.md` | `docs/issues/<lifecycle>/<slug>/ISSUE.md` |
| EPIC spec | `docs/issues/<lifecycle>/<slug>.md` | `docs/issues/<lifecycle>/<slug>/EPIC.md` |
| Refactor plan | `docs/refactor-plans/<lifecycle>/<slug>.md` | `docs/refactor-plans/<lifecycle>/<slug>/REFACTOR-PLAN.md` |
| PR-review audit | `docs/pr-reviews/pr-<number>-<suffix>.md` | `docs/pr-reviews/pr-<number>-<suffix>/PR-REVIEW.md` |

The 112 issue and EPIC records must be classified by their existing `doc-type` frontmatter before
each rename so the target is `ISSUE.md` or `EPIC.md`. Legacy records without usable frontmatter
require a body-heading review before moving.

## Link-Repair Inventory

The following live convention surfaces encode the current flat layouts and must be updated before
or with the corresponding archive migrations:

- `docs/AGENTS.md`: new-record placement table.
- `.github/skills/dev/planning/create-issue/SKILL.md`: issue and EPIC creation and promotion
  paths.
- `.github/skills/dev/planning/create-refactor-plan/SKILL.md`: refactor-plan creation and
  lifecycle paths.
- `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`: PR-review audit path.
- `docs/templates/REFACTOR-PLAN.md` and `docs/templates/PR-REVIEW-TEMPLATE.md`: primary-record
  `spec-path` and supporting-artifact conventions.
- `docs/index.md`, `docs/issues/README.md`, lifecycle README files, and
  `docs/pr-reviews/README.md`: navigation and current-layout descriptions.
- `docs/skills/semantic-skill-link-convention.md`: stable references to migrated PR-review
  audits.

Known inbound PR-review audit references outside the archive are in:

- `contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh`
- `docs/agents/orchestration.md`
- issue-local artifacts under `docs/issues/closed/2219-2003-unify-pr-review-processing/`
- issue-local artifacts under `docs/issues/closed/2233-2003-tune-unified-pr-review-process/`
- `docs/skills/semantic-skill-link-convention.md`
- `docs/templates/README.md`

The migration must also search the full tracked tree for every old primary-record path after each
batch. A reference inside a migrated record should use a path that remains valid after both the
folder move and any later issue lifecycle move.

## Concurrent-Work Risk

The inventory branch was clean when captured. Open issue and EPIC records already use folders, so
their paths do not need migration. Before moving draft or closed records, confirm no concurrent
branch changes touch the proposed source paths. Do not move a record concurrently edited by another
contributor without coordinating the rename.

## Recommended Order

1. Record the policy ADR and update creation guidance and templates.
2. Migrate the three closed refactor plans as a small, independently reviewable proof.
3. Migrate PR-review audits in a dedicated path-only batch and repair their inbound references.
4. Migrate issue and EPIC records by lifecycle: drafts, then closed records in bounded batches.
