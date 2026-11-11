---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: 1488
github-issue: null
spec-path: docs/issues/drafts/2801-follow-up-remove-unused-error-variants/ISSUE.md
branch: "2801-follow-up-remove-unused-error-variants"
related-pr: null
last-updated-utc: "2026-11-11 09:45"
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
