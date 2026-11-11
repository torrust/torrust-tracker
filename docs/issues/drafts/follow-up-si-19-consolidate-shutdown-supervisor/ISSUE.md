---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: null
github-issue: null
spec-path: docs/issues/drafts/follow-up-si-19-consolidate-shutdown-supervisor/ISSUE.md
branch: "follow-up-si-19-consolidate-shutdown-supervisor"
related-pr: null
last-updated-utc: "2026-11-11 10:59"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft: Consolidate the shutdown supervisors and review the owner types

> **Status**: Draft follow-up of [SI-19](../../closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md). Not yet a GitHub issue.

## Goal

Split out of SI-19 scope (Removal Constraint 1): consolidate the token-aware component supervisors, review the owner types (`OwnedTask`, `TokenAwareServerTask`, `OwnedReceiveLoop`) and adopt the health-check join order, a refactor that would widen the breaking surface.

## Scope

The three parts (the supervisors, the owner types and the health-check join order) described in the Out of scope section of the SI-19 spec.

## Acceptance Criteria

- [ ] The change is made and `linter all` passes.
- [ ] The maintainer's decision on the design is recorded before implementation.

## Architectural Decisions

To be written when the draft is refined into an issue.

## Verification Plan

To be written when the draft is refined into an issue.

## Implementation Plan

To be written when the draft is refined into an issue.

## Commit Points

To be written when the draft is refined into an issue.

## Progress Tracking

### Workflow Checkpoints

- [ ] Draft refined and the GitHub issue created

### Progress Log

- 2026-11-11 09:40 UTC - AI agent (Claude Sonnet 5.5) - Draft created as a follow-up of SI-19; not yet a GitHub issue.
- 2026-11-11 10:59 UTC - AI agent (Claude Sonnet 5.5) - Renamed to an unnumbered folder with `epic: null` and added the Architectural Decisions, Verification Plan and the other initialization sections as placeholders.

## Acceptance Verification

To be written when the draft is refined into an issue.

## Implementation Completion Review

To be completed when the work is done.
