---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1793
spec-path: docs/issues/open/1793-1669-03-define-per-package-default-timeout-constants.md
branch: 1793-1669-03-define-per-package-default-timeout-constants
related-pr: null
last-updated-utc: 2026-05-19 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/lib.rs
    - packages/tracker-client/Cargo.toml
    - packages/axum-http-tracker-server/src/v1/routes.rs
    - packages/udp-tracker-server/tests/server/contract.rs
    - console/tracker-client/Cargo.toml
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #1793 - Define per-package default timeout constants and remove `DEFAULT_TIMEOUT` from `torrust-tracker-configuration`

## Goal

Replace the shared `DEFAULT_TIMEOUT` constant in `packages/configuration` with per-package
timeout constants, each named to reflect the specific operation context of its package.
Remove `DEFAULT_TIMEOUT` from `packages/configuration` entirely once all consumers have
defined their own constant.

## Background

`DEFAULT_TIMEOUT` is a `Duration` constant (`Duration::from_secs(5)`), defined in
`packages/configuration/src/lib.rs`. It is not used within the `configuration` package
itself — it exists solely for other packages to import.

A single generic timeout shared across the entire workspace is too coarse-grained. Each
package performs a different kind of network operation:

- `packages/tracker-client`: UDP socket connect/send/receive
- `packages/axum-http-tracker-server`: HTTP request processing via Tower's `TimeoutLayer`
- `packages/udp-tracker-server` (tests): UDP client connections in contract tests
- `console/tracker-client`: network checking (UDP, HTTP, health checks) in a CLI tool

Each package should own its timeout default with a name that reflects its specific context.
Sharing a constant from the configuration crate creates an unnecessary coupling — packages
that have no other reason to depend on `torrust-tracker-configuration` are forced to do so
solely for a timeout value.

This issue is a subissue of EPIC #1669 (Overhaul: Packages).

## Scope

### In Scope

For each of the 4 consumer packages, in order:

1. **`packages/tracker-client`**: evaluate usage, define local constant(s), update the one
   import site, drop `torrust-tracker-configuration` if it is the only remaining reason for
   the dep.
2. **`packages/axum-http-tracker-server`**: evaluate usage, define local constant(s), update
   the one import site. Verify whether `torrust-tracker-configuration` can be dropped; drop it
   if so.
3. **`packages/udp-tracker-server`** (test file): evaluate usage, define local constant(s) in
   the test module, update all 4 inline import sites. Verify whether
   `torrust-tracker-configuration` can be dropped from `dev-dependencies`; drop it if so.
4. **`console/tracker-client`**: evaluate usage, define local constant(s) at crate level,
   update all 6 import sites, drop `torrust-tracker-configuration`.
5. **`packages/configuration`**: once `DEFAULT_TIMEOUT` has zero consumers across the
   workspace, remove the constant and its associated `use std::time::Duration;` import if
   it becomes unused.
6. **Regenerate** the workspace coupling report (`docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md`)
   by running `cargo run -p workspace-coupling`.

**Per-package evaluation rule**: before defining the local constant(s), review how
`DEFAULT_TIMEOUT` is used within the package. If it is used for two or more semantically
distinct operations (for example, "sending/receiving data" vs. "waiting for a socket to
become readable or writable"), define a separate named constant for each distinct purpose
rather than a single generic timeout. Document the chosen name(s) in the implementation
plan as the work progresses.

### Out of Scope

- Moving `DEFAULT_TIMEOUT` to `packages/clock` — superseded by this approach.
- Any API or behaviour changes beyond replacing the import source.
- Changing timeout values — all local constants use the same `Duration::from_secs(5)`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                         | Notes / Expected Output                                                                                                  |
| --- | ------ | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| T1  | DONE   | **`packages/tracker-client`**: evaluate `DEFAULT_TIMEOUT` usage; define local constant(s)                    | Review all use sites; if multiple distinct purposes, define one constant per purpose; candidates: `DEFAULT_UDP_TIMEOUT`  |
| T2  | DONE   | **`packages/tracker-client`**: remove `use torrust_tracker_configuration::DEFAULT_TIMEOUT`                   | Use local constant(s) instead; `cargo build -p bittorrent-tracker-client` succeeds                                       |
| T3  | DONE   | **`packages/tracker-client`**: drop `torrust-tracker-configuration` from `Cargo.toml`                        | No other imports from that crate; `cargo machete` confirms clean                                                         |
| T4  | DONE   | **`packages/axum-http-tracker-server`**: evaluate `DEFAULT_TIMEOUT` usage; define local constant(s)          | Review all use sites; candidates: `DEFAULT_REQUEST_TIMEOUT`                                                              |
| T5  | DONE   | **`packages/axum-http-tracker-server`**: remove `use torrust_tracker_configuration::DEFAULT_TIMEOUT`         | Use local constant(s); verify whether `torrust-tracker-configuration` can be dropped; drop if so                         |
| T6  | DONE   | **`packages/udp-tracker-server`** (tests): evaluate `DEFAULT_TIMEOUT` usage; define local constant(s)        | Review 4 use sites; candidates: `DEFAULT_UDP_TIMEOUT`                                                                    |
| T7  | DONE   | **`packages/udp-tracker-server`** (tests): remove all 4 `use torrust_tracker_configuration::DEFAULT_TIMEOUT` | Use local constant(s); verify whether dep can be dropped from `dev-dependencies`; drop if so                             |
| T8  | DONE   | **`console/tracker-client`**: evaluate `DEFAULT_TIMEOUT` usage; define local constant(s)                     | Review 6 use sites across UDP, HTTP, health-check contexts; candidates: `DEFAULT_NETWORK_TIMEOUT` or per-operation names |
| T9  | DONE   | **`console/tracker-client`**: update all 6 import sites to use the local constant(s)                         | Remove all `use torrust_tracker_configuration::DEFAULT_TIMEOUT` imports                                                  |
| T10 | DONE   | **`console/tracker-client`**: drop `torrust-tracker-configuration` from `Cargo.toml`                         | `cargo build -p torrust-tracker-client` succeeds; `cargo machete` confirms clean                                         |
| T11 | DONE   | **`packages/configuration`**: remove `DEFAULT_TIMEOUT` and its `Duration` import if unused                   | Zero consumers remaining; `cargo build --workspace` succeeds; `cargo machete` confirms clean                             |
| T12 | DONE   | Run `cargo build --workspace` and `cargo test --workspace`                                                   | Clean build; all tests pass                                                                                              |
| T13 | DONE   | Run `linter all`                                                                                             | Exit code `0`                                                                                                            |
| T14 | DONE   | Regenerate workspace coupling report                                                                         | `cargo run -p workspace-coupling`; updates `docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md`        |

**Source files updated** (12 files across 5 packages):

- `packages/tracker-client/src/udp/client.rs` (T1–T2)
- `packages/axum-http-tracker-server/src/v1/routes.rs` (T4–T5)
- `packages/axum-rest-tracker-api-server/src/routes.rs` (discovered during implementation; `DEFAULT_REQUEST_TIMEOUT` added)
- `packages/udp-tracker-server/src/environment.rs` (discovered during implementation; `DEFAULT_SERVER_LIFECYCLE_TIMEOUT` added)
- `packages/udp-tracker-server/tests/server/contract.rs` (T6–T7; `DEFAULT_UDP_TIMEOUT` added)
- `console/tracker-client/src/lib.rs` (T8; `DEFAULT_NETWORK_TIMEOUT` defined)
- `console/tracker-client/src/console/clients/unified/udp.rs` (T9)
- `console/tracker-client/src/console/clients/unified/check.rs` (T9)
- `console/tracker-client/src/console/clients/unified/http.rs` (T9)
- `console/tracker-client/src/console/clients/http/app.rs` (T9)
- `console/tracker-client/src/console/clients/checker/service.rs` (T9)
- `console/tracker-client/src/console/clients/udp/app.rs` (T9)

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [x] Manual verification scenarios executed and recorded
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669; identified as
  prerequisite for the clock rename subissue.
- 2026-05-19 00:00 UTC - josecelano - Revised approach: instead of moving `DEFAULT_TIMEOUT`
  to `torrust-tracker-clock`, define per-package constants with context-specific names in all
  4 consumer packages and remove `DEFAULT_TIMEOUT` from `packages/configuration` entirely.
  Spec file renamed to `1669-03-define-per-package-default-timeout-constants.md`.
  SI-09 (clock rename) no longer depends on this issue. EPIC updated accordingly.

## Acceptance Criteria

- [x] `packages/tracker-client` defines local timeout constant(s); no import from `torrust_tracker_configuration`; `torrust-tracker-configuration` removed from its `Cargo.toml`.
- [x] `packages/axum-http-tracker-server` defines local timeout constant(s); no import from `torrust_tracker_configuration`.
- [x] `packages/udp-tracker-server` test file defines local timeout constant(s); no import from `torrust_tracker_configuration` in tests.
- [x] `console/tracker-client` defines local timeout constant(s); no file in that package imports `DEFAULT_TIMEOUT` from `torrust_tracker_configuration`; `torrust-tracker-configuration` removed from its `Cargo.toml`.
- [x] `packages/configuration/src/lib.rs` no longer defines `DEFAULT_TIMEOUT`.
- [x] `grep -r "torrust_tracker_configuration::DEFAULT_TIMEOUT" . --include="*.rs"` returns zero matches.
- [x] `cargo build --workspace` succeeds with zero errors.
- [x] `cargo test --workspace` passes with zero failures.
- [x] `linter all` exits with code `0`.
- [x] Workspace coupling report regenerated and committed.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete`
- `cargo run -p workspace-coupling`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                  | Command/Steps                                                                 | Expected Result | Status | Evidence |
| --- | --------------------------------------------------------- | ----------------------------------------------------------------------------- | --------------- | ------ | -------- |
| M1  | No stale imports from configuration for timeout           | `grep -r "torrust_tracker_configuration::DEFAULT_TIMEOUT" . --include="*.rs"` | Zero matches    | DONE   | Verified 2026-05-19 |
| M2  | tracker-client no longer depends on configuration         | `grep "torrust-tracker-configuration" packages/tracker-client/Cargo.toml`     | Zero matches    | DONE   | Verified 2026-05-19 |
| M3  | console/tracker-client no longer depends on configuration | `grep "torrust-tracker-configuration" console/tracker-client/Cargo.toml`      | Zero matches    | DONE   | Verified 2026-05-19 |
| M4  | DEFAULT_TIMEOUT removed from configuration package        | `grep "DEFAULT_TIMEOUT" packages/configuration/src/lib.rs`                    | Zero matches    | DONE   | Verified 2026-05-19 |
| M5  | Workspace coupling report up to date                      | `cargo run -p workspace-coupling` produces output matching committed report   | Clean run       | DONE   | Regenerated 2026-05-19 |
