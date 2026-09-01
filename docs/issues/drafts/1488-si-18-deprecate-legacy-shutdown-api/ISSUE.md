---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-server/src/signals.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/states.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/drafts/1488-si-2-remove-global-shutdown-signal/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-18 — Deprecate Legacy Shutdown API

> **EPIC position**: Roadmap step 14. Compatibility-preserving deprecation after
> every supported in-workspace and standalone consumer uses token lifecycle APIs.

## Goal

Deprecate the legacy shutdown API without changing runtime behavior or removing
symbols. Mark `Halted`-based server start/stop entry points and library-level
OS-signal shutdown helpers as deprecated, point consumers to the token-aware
lifecycle API, and publish migration guidance with a removal release target.

This task must not remove `Halted`, `shutdown_signal`,
`shutdown_signal_with_message`, `global_shutdown_signal`, or legacy server
start/stop methods. Existing consumers must compile and retain their present
behavior after deprecation warnings are addressed or explicitly allowed.

## Eligibility Gate

Do not start this work until the following evidence is recorded:

- [ ] HTTP tracker, REST API, health-check API, and UDP tracker application
      components use token-aware lifecycle paths.
- [ ] Standalone HTTP and UDP environments/examples use token-aware lifecycle
      paths.
- [ ] The task inventory confirms no supported in-workspace consumer depends on
      the legacy shutdown path.
- [ ] The external `torrust-server-lib` release notes identify external consumer
      migration guidance and a compatible deprecation version.
- [ ] Process-wrapper documentation follows Q5: no same-process Tokio task is
      treated as surviving SIGKILL, and verification targets the actual tracker
      process or a deliberately selected process group.

## Scope

### In scope

- Add Rust `#[deprecated]` attributes and documentation to legacy public APIs.
- Document the token-aware replacement API and required migration steps.
- Update in-workspace call sites that remain only in tests, examples, or
  compatibility checks to suppress/allow the warning locally with a rationale.
- Add compiler/test coverage proving legacy API consumers still compile.
- Publish release notes describing deprecation, compatibility duration, and the
  planned breaking removal release.

### Out of scope

- Removing legacy shutdown APIs or library-level OS-signal subscriptions.
- Changing any server shutdown behavior, signal routing, deadline, or exit-code
  policy.
- Migrating a component or standalone consumer not already migrated before this
  eligibility gate.
- Requiring unknown external consumers to upgrade immediately.

## Deprecation Requirements

1. A deprecation message must name the token-aware replacement and explain that
   executable entry points, not libraries, own OS-signal subscriptions.
2. The message must identify the planned breaking removal release according to
   the package versioning policy.
3. Legacy API behavior stays unchanged. Deprecation is source guidance, not a
   behavioral migration.
4. The release notes must state the support window and the evidence required
   before removal.
5. The final removal task must not proceed merely because in-workspace code has
   migrated; it also requires the declared external compatibility period to end.

## Acceptance Criteria

- [ ] All eligibility-gate evidence is present and linked from this issue's
      verification record.
- [ ] Each deprecated legacy public API identifies its token-aware replacement
      and removal release target.
- [ ] Deprecated APIs remain source-compatible and preserve runtime behavior.
- [ ] In-workspace compatibility coverage compiles representative legacy
      server-library consumers.
- [ ] No new production code introduces a legacy shutdown API dependency.
- [ ] Release notes document migration, support window, and final removal
      prerequisites.
- [ ] `linter all` passes.

## Dependencies

- SI-11 through SI-17 are complete for the HTTP, REST, health-check, UDP, and
  standalone migrations.
- SI-2's additive server lifecycle API is released and documented.
- #1588 revalidates the final supported-consumer inventory.
- Q5's process-wrapper verification rule is reflected in release notes and
  removal planning.

## Rollback

Remove only the deprecation attributes, migration text, and release-note entry.
Because this task does not remove or alter legacy behavior, reverting it is
source-compatible and has no runtime impact.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Link the completed migration verification records and final task inventory.
2. Compile representative code using each deprecated API; record the expected
   warnings and confirm unchanged runtime behavior.
3. Compile migrated paths with warnings treated as errors to prove they no
   longer depend on deprecated APIs.
4. Review generated Rust documentation and release notes for accurate
   replacement and support-window guidance.
