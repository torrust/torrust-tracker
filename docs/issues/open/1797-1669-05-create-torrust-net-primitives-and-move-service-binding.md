---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1797
spec-path: docs/issues/open/1797-1669-05-create-torrust-net-primitives-and-move-service-binding.md
branch: 1669-05-create-torrust-net-primitives-and-move-service-binding
related-pr: null
last-updated-utc: 2026-05-19 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/net-primitives/src/service_binding.rs
    - packages/net-primitives/Cargo.toml
    - packages/primitives/src/lib.rs
    - packages/server-lib/Cargo.toml
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md
---

<!-- skill-link: create-issue -->

# Issue #1797 - Create `torrust-net-primitives` and move `ServiceBinding` from `torrust-tracker-primitives`

## Goal

Create a new `torrust-net-primitives` package containing generic networking primitives (starting
with `ServiceBinding`) and move `ServiceBinding` out of `torrust-tracker-primitives` into this
new crate. `torrust-server-lib` then depends on `torrust-net-primitives` instead of
`torrust-tracker-primitives`, breaking an unnecessary coupling.

## Background

The coupling analysis (F-04) found that `torrust-server-lib` depends on
`torrust-tracker-primitives` solely to import `ServiceBinding` — a struct representing a
network address binding (socket address at which a service listens). `torrust-server-lib` is a
generic server utility library with no tracker-specific concerns; pulling in the entire
`torrust-tracker-*` primitives crate for one generic networking type is wasteful and semantically
misleading.

`ServiceBinding` is a very generic concept that can be reused across the Torrust organisation,
not just in the tracker. Creating a dedicated `torrust-net-primitives` crate makes the type
available to any Torrust project without a `torrust-tracker-*` dependency.

Both `torrust-tracker-primitives` (source) and the new `torrust-net-primitives` (destination)
are intended to be published to crates.io. Removing `ServiceBinding` from
`torrust-tracker-primitives` is a semver breaking change; a major version bump will be needed
when the published crate is updated. Within this workspace at version `3.0.0-develop`, the
change is expected and planned.

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Create `packages/net-primitives/` with a minimal `Cargo.toml` (`name = "torrust-net-primitives"`,
  `publish = true`) and `src/lib.rs`.
- Move `ServiceBinding` (and its module `service_binding`) from `packages/primitives/` to
  `packages/net-primitives/`.
- Add `torrust-net-primitives` to the workspace `[members]` in `Cargo.toml`.
- Update `packages/server-lib/Cargo.toml` to depend on `torrust-net-primitives` instead of
  `torrust-tracker-primitives`.
- Remove `torrust-tracker-primitives` dep from `packages/server-lib/Cargo.toml` if
  `ServiceBinding` was its only reason.
- Update all workspace files that import `ServiceBinding` from `torrust_tracker_primitives` to
  import from `torrust_net_primitives`.
- Verify the workspace builds and all tests pass.

### Out of Scope

- Moving other types from `torrust-tracker-primitives` into `torrust-net-primitives`; this
  subissue focuses only on `ServiceBinding`.
- Publishing `torrust-net-primitives` to crates.io; that is handled in the release cycle.
- Removing the `#[deprecated]` re-export of `ServiceBinding` from `torrust-tracker-primitives`
  for external consumers; that requires a crates.io semver bump and is deferred.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                                      | Notes / Expected Output                                                                                                                        |
| --- | ------ | ------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Locate all usage sites of `ServiceBinding` in the workspace                                                               | `grep -r "ServiceBinding" . --include="*.rs"` — build full consumer list                                                                       |
| T2  | DONE   | Create `packages/net-primitives/Cargo.toml` and `src/lib.rs`                                                              | `name = "torrust-net-primitives"`, `publish = true`; inherits workspace `edition`/`rust-version`                                               |
| T3  | DONE   | Add `packages/net-primitives` to workspace `[members]` in root `Cargo.toml`                                               | `cargo build -p torrust-net-primitives` succeeds                                                                                               |
| T4  | DONE   | Move `service_binding` module to `packages/net-primitives/src/`                                                           | Module exported from `packages/net-primitives/src/lib.rs`                                                                                      |
| T5  | DONE   | Remove `service_binding` module definition from `packages/primitives/src/` and replace with a `#[deprecated]` re-export   | `packages/primitives` re-exports `ServiceBinding` via `#[deprecated]` from `torrust_net_primitives` (same pattern as `DurationSinceUnixEpoch`) |
| T6  | DONE   | Update `packages/server-lib/Cargo.toml`: replace `torrust-tracker-primitives` dep with `torrust-net-primitives`           | `cargo build -p torrust-server-lib` succeeds; `cargo machete` clean                                                                            |
| T7  | DONE   | Update all other workspace files importing `ServiceBinding` from `torrust_tracker_primitives` to `torrust_net_primitives` | One-line change per file (35 source files updated)                                                                                             |
| T8  | DONE   | Run `cargo build --workspace` and `cargo test --workspace`                                                                | Clean build; all tests pass                                                                                                                    |
| T9  | DONE   | Run `linter all`                                                                                                          | Exit code `0` (via pre-commit hook)                                                                                                            |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] Package name confirmed: `torrust-net-primitives`
- [x] Backwards-compat strategy confirmed: `#[deprecated]` re-export in `torrust-tracker-primitives` (same pattern as `DurationSinceUnixEpoch`)
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [x] Manual verification scenarios executed and recorded
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-18 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669, addressing F-04
  from the coupling analysis report. Package name `torrust-net-primitives` is a proposal pending
  confirmation.
- 2026-05-19 00:00 UTC - josecelano - Spec updated: `#[deprecated]` re-export strategy confirmed
  (same pattern as `DurationSinceUnixEpoch`). GitHub issue #1797 created. Spec moved to
  `docs/issues/open/`.
- 2026-05-19 00:00 UTC - josecelano - Implementation complete. `torrust-net-primitives` package
  created; `ServiceBinding` moved from `torrust-tracker-primitives` to `torrust-net-primitives`;
  `#[deprecated]` re-export added in `torrust-tracker-primitives`; all 35 consumer import paths
  updated; `cargo build --workspace` and `linter all` pass.

## Acceptance Criteria

- [x] `packages/net-primitives/` exists and is a member of the workspace.
- [x] `torrust-net-primitives` exports `ServiceBinding` publicly.
- [x] `packages/primitives/src/` no longer defines `ServiceBinding` (only re-exports it via `#[deprecated]`
      from `torrust_net_primitives` for external crates.io consumer backwards compatibility).
- [x] `packages/server-lib/Cargo.toml` does not list `torrust-tracker-primitives` as a dependency
      (replaced by `torrust-net-primitives`).
- [x] No workspace file imports `ServiceBinding` from `torrust_tracker_primitives` directly
      (workspace consumers use `torrust_net_primitives::service_binding`).
- [x] `cargo build --workspace` succeeds with zero errors.
- [x] `cargo test --workspace` passes with zero failures.
- [x] `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                          | Command / Steps                                                             | Expected Result    | Status | Evidence                                                                                |
| --- | ----------------------------------------------------------------- | --------------------------------------------------------------------------- | ------------------ | ------ | --------------------------------------------------------------------------------------- |
| M1  | No workspace import of `ServiceBinding` from `tracker_primitives` | `grep -r "torrust_tracker_primitives::.*ServiceBinding" . --include="*.rs"` | Zero matches       | DONE   | 0 matches confirmed                                                                     |
| M2  | `torrust-net-primitives` exports `ServiceBinding`                 | `grep "ServiceBinding" packages/net-primitives/src/service_binding.rs`      | `pub struct` found | DONE   | `pub struct ServiceBinding` present in `packages/net-primitives/src/service_binding.rs` |
| M3  | `server-lib` no longer depends on `tracker-primitives`            | `grep "torrust-tracker-primitives" packages/server-lib/Cargo.toml`          | Zero matches       | DONE   | 0 matches confirmed                                                                     |
