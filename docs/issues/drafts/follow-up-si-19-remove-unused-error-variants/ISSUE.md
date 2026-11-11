---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: null
github-issue: null
spec-path: docs/issues/drafts/follow-up-si-19-remove-unused-error-variants/ISSUE.md
branch: "follow-up-si-19-remove-unused-error-variants"
related-pr: null
last-updated-utc: "2026-11-11 10:59"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft: Remove the unused public error variants

> **Status**: Draft follow-up of [SI-19](../../closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md). Not yet a GitHub issue.

## Goal

SI-19 removed the legacy shutdown API and left public error variants that nothing constructs. Remove them in the next breaking release.

## Scope

`Error::Stop`, `Error::StartupNotification` and `Error::StartupNotificationDropped` of the HTTP and REST servers, and the `UdpError` variants `StartupNotification`, `Launcher` and `FailedToStartOrStopServer`. This is a breaking change to public enums, so it needs its own release decision.

## Acceptance Criteria

- [ ] The change is made and `linter all` passes.
- [ ] A search for each variant finds no use in the tree.

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
