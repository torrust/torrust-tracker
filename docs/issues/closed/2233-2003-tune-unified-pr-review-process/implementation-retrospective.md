---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/issues/closed/2233-2003-tune-unified-pr-review-process/ISSUE.md
    - docs/issues/closed/2233-2003-tune-unified-pr-review-process/code-span-path-case-analysis.md
    - docs/issues/closed/2233-2003-tune-unified-pr-review-process/code-span-path-case-inventory.tsv
    - issue #2264
---

# Implementation Retrospective - Issue #2233

## Purpose

Record evidence-based process improvements discovered while implementing issue #2233. This is a
blameless review of the implementation approach; it does not replace acceptance-criteria
verification.

## Outcome

Issue #2233 delivered the reviewer-side guidance alignment, PR-review migration safeguards, and a
design-only tiered model routing note. The originally proposed Markdown code-span path checker was
not implemented in this issue; instead, the issue records a complete case inventory and defers strict
path-reference validation to the draft semantic-link/frontmatter conventions EPIC.

## What Went Well

1. The code-span path inventory made the T2 scope decision evidence-based instead of speculative.
2. Keeping T3/T4 in the governing `process-pr-review` skill avoided duplicating review-process rules
   across competing documents.

## What Changed During Implementation

T2 changed from implementation of a checker to analysis and deferral. The inventory found many
non-resolving Markdown code spans that are historical records, placeholders, globs, examples,
directory references, or retired paths. Enforcing all of them as current paths would create a large
exception burden and would overlap awkwardly with the broader semantic-link and path-reference
convention work.

The implementation therefore created `docs/issues/drafts/refactor-semantic-link-conventions/EPIC.md`
as the future design home for frontmatter metadata, semantic links, typed path references, and
validation strategy.

## Root Cause

The initial T2 plan treated inline code-span paths as a narrow linting problem. The baseline evidence
showed that path-like spans are part of a broader documentation language problem: some are navigable
paths, some are examples, some are historical evidence, and some may need typed semantic meaning.

## Improvements for Future Work

1. Define typed frontmatter, semantic-link, and path-reference conventions before adding strict
   validators for prose references.
2. When proposing a new documentation checker, inventory current repository examples before choosing
   enforcement scope.

## Avoiding Overcorrection

Do not replace Lychee for ordinary Markdown links. Do not rewrite historical review records or closed
issue specs only to satisfy a future path-reference syntax. Do not require every prose path mention
to become a semantic link.

## Evidence

- `docs/issues/closed/2233-2003-tune-unified-pr-review-process/ISSUE.md`
- `docs/issues/closed/2233-2003-tune-unified-pr-review-process/code-span-path-case-analysis.md`
- `docs/issues/closed/2233-2003-tune-unified-pr-review-process/code-span-path-case-inventory.tsv`
- `docs/issues/drafts/refactor-semantic-link-conventions/EPIC.md`
- `docs/issues/closed/2233-2003-tune-unified-pr-review-process/tiered-model-routing-design.md`
