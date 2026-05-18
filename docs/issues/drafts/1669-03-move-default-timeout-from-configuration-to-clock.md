---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1669-03-move-default-timeout-from-configuration-to-clock.md
branch: null
related-pr: null
last-updated-utc: 2026-05-18 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/lib.rs
    - packages/clock/src/lib.rs
    - packages/tracker-client/Cargo.toml
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/drafts/1669-09-rename-torrust-tracker-clock-to-torrust-clock.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Move `DEFAULT_TIMEOUT` from `torrust-tracker-configuration` to `torrust-tracker-clock`

## Goal

Move the `DEFAULT_TIMEOUT` constant from `packages/configuration` to `packages/clock`,
so that packages needing only a default timeout value do not have to depend on the full
tracker configuration crate.

## Background

`DEFAULT_TIMEOUT` is a `Duration` constant (`Duration::from_secs(5)`), defined in
`packages/configuration/src/lib.rs`. It is a time concept — a default duration used as
a network timeout. It does not belong in `configuration`, which is concerned with
tracker configuration structs and their parsing.

The immediate motivation is `packages/tracker-client`: its `Cargo.toml` lists
`torrust-tracker-configuration` as a dependency, but the only thing it imports from that
crate is `DEFAULT_TIMEOUT` (one import site: `packages/tracker-client/src/udp/client.rs`).
Moving the constant to `clock` removes an unnecessary heavyweight dependency from a
client library.

Placing `DEFAULT_TIMEOUT` in `clock` also makes semantic sense: `clock` already owns the
mockable time abstraction; default timeout durations are a natural sibling.

**Side effect (F-01)**: two client packages (`bittorrent-tracker-client` and
`torrust-tracker-client` in `console/tracker-client`) depend on `torrust-tracker-configuration`
solely for `DEFAULT_TIMEOUT`. After this move both clients can drop that dependency entirely,
eliminating a layer violation where client packages depend on the tracker configuration crate.

**This issue is a prerequisite** for renaming `torrust-tracker-clock` to `torrust-clock`
(see linked spec). It must be completed and merged first so that the constant travels with
the `clock` package when it is eventually renamed and extracted.

This issue is a subissue of EPIC #1669 (Overhaul: Packages).

## Scope

### In Scope

- Add `pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);` to `packages/clock`
  at an appropriate public location.
- Remove `DEFAULT_TIMEOUT` from `packages/configuration/src/lib.rs`.
- Update all 9 source files that use `use torrust_tracker_configuration::DEFAULT_TIMEOUT`
  to use `use torrust_tracker_clock::DEFAULT_TIMEOUT`.
- Drop `torrust-tracker-configuration` from `packages/tracker-client/Cargo.toml` (it was
  the only reason that dependency existed).
- Verify that `console/tracker-client/Cargo.toml` also no longer needs `torrust-tracker-configuration`
  after the import update; drop it if confirmed.
- Verify the workspace builds and all tests pass.

### Out of Scope

- Renaming `torrust-tracker-clock` to `torrust-clock` — that is the next subissue.
- Removing `torrust-tracker-configuration` from other packages that imported `DEFAULT_TIMEOUT`
  but also use configuration for other purposes — those packages still need the dep.
- Changes to any other constant or API in either crate.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                | Notes / Expected Output                                                                 |
| --- | ------ | --------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| T1  | TODO   | Add `pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);` to `packages/clock`             | Choose an appropriate public module (e.g., top of `lib.rs` or a `timeout` mod)          |
| T2  | TODO   | Remove `DEFAULT_TIMEOUT` from `packages/configuration/src/lib.rs`                                   | Constant no longer in `configuration`                                                   |
| T3  | TODO   | Update all 9 import sites to `use torrust_tracker_clock::DEFAULT_TIMEOUT`                           | See file list below                                                                     |
| T4  | TODO   | Remove `torrust-tracker-configuration` from `packages/tracker-client/Cargo.toml`                    | No longer a dependency; `cargo build -p bittorrent-tracker-client` succeeds             |
| T5  | TODO   | Verify `console/tracker-client/Cargo.toml` no longer needs `torrust-tracker-configuration`; drop it | `cargo build -p torrust-tracker-client` succeeds; `cargo machete` reports no unused dep |
| T6  | TODO   | Run `cargo build --workspace` and `cargo test --workspace`                                          | Clean build; all tests pass                                                             |
| T7  | TODO   | Run `linter all`                                                                                    | Exit code `0`                                                                           |

**Source files to update in T3** (9 files):

- `packages/tracker-client/src/udp/client.rs`
- `packages/axum-http-tracker-server/src/v1/routes.rs`
- `packages/udp-tracker-server/tests/server/contract.rs`
- `console/tracker-client/src/console/clients/unified/udp.rs`
- `console/tracker-client/src/console/clients/unified/check.rs`
- `console/tracker-client/src/console/clients/unified/http.rs`
- `console/tracker-client/src/console/clients/http/app.rs`
- `console/tracker-client/src/console/clients/checker/service.rs`
- `console/tracker-client/src/console/clients/udp/app.rs`

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669; identified as
  prerequisite for the clock rename subissue.

## Acceptance Criteria

- [ ] `packages/clock` exports `DEFAULT_TIMEOUT` as a public constant.
- [ ] `packages/configuration` no longer defines `DEFAULT_TIMEOUT`.
- [ ] No source file in the workspace uses `torrust_tracker_configuration::DEFAULT_TIMEOUT`.
- [ ] `packages/tracker-client/Cargo.toml` no longer lists `torrust-tracker-configuration`.
- [ ] `console/tracker-client/Cargo.toml` no longer lists `torrust-tracker-configuration`
      (confirmed: `DEFAULT_TIMEOUT` was its only use).
- [ ] `cargo build --workspace` succeeds with zero errors.
- [ ] `cargo test --workspace` passes with zero failures.
- [ ] `linter all` exits with code `0`.
- [ ] `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md` are reviewed; no sections reference `DEFAULT_TIMEOUT` as belonging to `torrust-tracker-configuration`.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                          | Command/Steps                                                                 | Expected Result | Status | Evidence |
| --- | ------------------------------------------------- | ----------------------------------------------------------------------------- | --------------- | ------ | -------- |
| M1  | No stale imports from configuration for timeout   | `grep -r "torrust_tracker_configuration::DEFAULT_TIMEOUT" . --include="*.rs"` | Zero matches    | TODO   |          |
| M2  | tracker-client no longer depends on configuration | `grep "torrust-tracker-configuration" packages/tracker-client/Cargo.toml`     | Zero matches    | TODO   |          |
