---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: 1488
github-issue: null
spec-path: docs/issues/drafts/2801-follow-up-consolidate-shutdown-supervisor/ISSUE.md
branch: "2801-follow-up-consolidate-shutdown-supervisor"
related-pr: null
last-updated-utc: "2026-11-11 09:45"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft: Consolidate the shutdown supervisor

> **Status**: Draft follow-up of [SI-19](../../closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md). Not yet a GitHub issue.

## Goal

Split out of SI-19 scope (Removal Constraint 1): consolidate the supervisor and the health-check join order, a refactor that would widen the breaking surface.

## Scope

The supervisor and health-check join order described in the Out of scope section of the SI-19 spec.

## Acceptance Criteria

- [ ] The change is made and `linter all` passes.
- [ ] The maintainer's decision on the design is recorded before implementation.
