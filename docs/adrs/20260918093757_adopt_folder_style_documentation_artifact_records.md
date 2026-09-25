---
semantic-links:
  skill-links:
    - create-issue
    - create-refactor-plan
    - process-pr-review
  related-artifacts:
    - docs/AGENTS.md
    - docs/issues/
    - docs/refactor-plans/
    - docs/pr-reviews/
    - docs/templates/ISSUE.md
    - docs/templates/EPIC.md
    - docs/templates/REFACTOR-PLAN.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/create-refactor-plan/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# Adopt Folder-Style Documentation Artifact Records

## Scope

Root ADR. This repository-wide documentation convention governs the layout of durable record
families across `docs/`; no extractable package owns it.

## Description

Issues, EPICs, refactor plans, and pull-request review audits accumulate companion artifacts such
as evidence, analyses, reports, and retrospectives during their lifecycle. A flat primary Markdown
file provides no stable local home for those artifacts and makes future extension inconsistent.

The repository already uses folder-style issue and EPIC records in many places, but legacy issue
and EPIC records, every refactor plan, and every PR-review audit remain flat. A prospective-only
rule would leave these selected durable record families permanently inconsistent.

## Agreement

Use folder-style layout for every issue spec, EPIC spec, refactor plan, and PR-review audit.

Each record directory is named after its existing record stem. Its primary document uses the
family's canonical filename:

| Record family | Primary filename | Example |
| ------------- | ---------------- | ------- |
| Issue spec | `ISSUE.md` | `docs/issues/closed/2159-2003-adopt-folder-style-issue-specs/ISSUE.md` |
| EPIC spec | `EPIC.md` | `docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` |
| Refactor plan | `REFACTOR-PLAN.md` | `docs/refactor-plans/closed/2238-refactor-native-tracker-test-fixture/REFACTOR-PLAN.md` |
| PR-review audit | `PR-REVIEW.md` | `docs/pr-reviews/pr-2252-review/PR-REVIEW.md` |

Store companion artifacts in the same record directory. Do not add a directory solely because a
document is Markdown: stable reference pages, templates, ADRs, one-off analyses, guides, and media
retain their existing family-specific layouts unless a later ADR classifies them as durable records
with lifecycle-specific companion artifacts.

Migrate all current flat records in the selected families. Use `git mv` so Git history remains
traceable. Preserve body content, identity, and lifecycle state. Change only frontmatter paths,
directory-local relative links, and live repository references that would otherwise become stale.
Do not move a record that another contributor is actively modifying until the move is coordinated.

## Alternatives Considered

**Use folders only for new records.** Rejected because it permanently preserves inconsistent
layouts across record families and leaves historical records without a stable home for future
evidence.

**Put every documentation artifact in a directory.** Rejected because nesting has a maintenance
cost and no benefit for documents that have no primary-record lifecycle or companion artifacts.

**Keep primary files flat and create a sibling assets directory.** Rejected because a record's
identity and supporting artifacts should move together across lifecycle locations and be visible as
one unit.

## Consequences

New and existing durable record families have one extensible layout and predictable primary
filenames. Archive migration produces a large path-only change, so it must be staged by record
family and lifecycle, with link validation after each batch. Creation guidance and templates must
be changed before archive migration to prevent new flat records.

## Date

2026-09-18

## References

- Issue: #2159
- [Place ADRs by Decision Scope](20260830124000_place_adrs_by_decision_scope.md)
- [Migration inventory](../issues/closed/2159-2003-adopt-folder-style-issue-specs/migration-inventory.md)
