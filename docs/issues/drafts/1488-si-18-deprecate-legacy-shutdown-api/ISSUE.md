---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 1488
github-issue: null
spec-path: docs/issues/drafts/1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
branch: "{issue-number}-deprecate-legacy-shutdown-api"
related-pr: null
last-updated-utc: "2026-11-06 12:00"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-server/src/signals.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/udp-server/src/server/spawner.rs
    - src/bootstrap/jobs/manager.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/states.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/closed/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-18 — Deprecate Legacy Shutdown API

> **EPIC position**: Roadmap step 18. Compatibility-preserving deprecation after
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

Evidence checked on 2026-11-06 against `torrust/develop` at `da8bd15d1`:

- [x] HTTP tracker, REST API, health-check API, and UDP tracker application
      components use token-aware lifecycle paths (`src/bootstrap/jobs/*` call
      `start_with_cancellation`).
- [x] Standalone HTTP and UDP environments/examples use token-aware lifecycle
      paths (SI-16 #2412, SI-17 #2448).
- [x] REST API and health-check API test environments use token-aware
      lifecycle paths (SI-23 #2449, SI-24 #2450).
- [x] The task inventory confirms no supported in-workspace consumer depends on
      the legacy shutdown path. The remaining callers are unit tests that
      exercise the legacy path on purpose.
- [ ] The external `torrust-server-lib` release notes identify external consumer
      migration guidance and a compatible deprecation version. **Blocked**:
      0.4.0 (2026-10-26) is the latest release, has no `#[deprecated]`
      attributes on the legacy symbols, and publishes no release notes. See
      [Split with the external crate](#split-with-the-external-crate).
- [x] Process-wrapper documentation follows Q5 (`questions.md` Q5 is resolved;
      the deployment contract is in `docs/containers.md#stopping-the-tracker`).

## Split with the External Crate

The maintainer decided on 2026-11-06 to deprecate in the library first. The
symbols below are defined in the external `torrust-server-lib` crate, so this
repository cannot deprecate them.

| Symbol (external, `torrust_server_lib::signals`) | Replacement                                      |
| ------------------------------------------------ | ------------------------------------------------ |
| `Halted`                                         | `CancellationToken` (injected by the caller)     |
| `shutdown_signal`                                | `cancellation_signal`                            |
| `shutdown_signal_with_message`                   | `cancellation_signal` plus a log line            |
| `global_shutdown_signal`                         | an OS-signal subscription in the executable only |

Order of work:

1. **Library release** (separate repository and issue): add `#[deprecated]`
   attributes naming the replacement and the removal release, a changelog and
   release notes, and publish a new version (the "compatible deprecation
   version"). `Started` and `cancellation_signal` stay.
2. **This issue**: bump `torrust-server-lib` in the workspace to that version,
   then deprecate the in-repo symbols listed under Scope.

This issue stays a draft until step 1 is released.

## Scope

### In scope

- Add Rust `#[deprecated]` attributes and documentation to legacy public APIs
  defined in this repository:
  - `axum-http-server`: `HttpServer<Stopped>::start`, `start_with_health_check`,
    `Running.halt_task`, `Running::stop`, `Launcher::start` (with `rx_halt`).
  - `axum-rest-api-server`: `ApiServer::start`, `Running.halt_task`,
    `Running::stop`, `Launcher::start` (with `rx_halt`).
  - `axum-health-check-api-server`: `server::start` (with `rx_halt`).
  - `udp-server`: `Server<Stopped>::start`, `Running.halt_task`,
    `Running::stop`, `Launcher::run_with_graceful_shutdown`, and the
    `rx_halt` field of the spawner.
  - `axum-server`: `graceful_shutdown`.
  - `src/bootstrap/jobs/manager.rs`: `HaltSignal`, `NestedServerTask` (public,
    used only by their own tests).
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

- SI-11 through SI-17, SI-23, and SI-24 are complete (verified 2026-11-06).
- A `torrust-server-lib` release that deprecates its legacy signal API (see
  the split above). Not yet available.
- SI-2's additive server lifecycle API is released and documented.
- #1588 revalidates the final supported-consumer inventory.
- Q5's process-wrapper verification rule is reflected in release notes and
  removal planning.

## Rollback

Remove only the deprecation attributes, migration text, and release-note entry.
Because this task does not remove or alter legacy behavior, reverting it is
source-compatible and has no runtime impact.

## Manual Verification

Record commands, output, and logs in issue-local `manual-verification-evidence.md` before closing this issue. This source-deprecation task must state whether executable-boundary verification is non-applicable; if its implementation changes a supported runtime path, follow the EPIC executable-boundary protocol.

1. Link the completed migration verification records and final task inventory.
2. Compile representative code using each deprecated API; record the expected
   warnings and confirm unchanged runtime behavior.
3. Compile migrated paths with warnings treated as errors to prove they no
   longer depend on deprecated APIs.
4. Review generated Rust documentation and release notes for accurate
   replacement and support-window guidance.
