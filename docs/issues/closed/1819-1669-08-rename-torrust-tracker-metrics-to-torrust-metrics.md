---
doc-type: issue
issue-type: task
status: closed
priority: p2
github-issue: 1819
spec-path: docs/issues/closed/1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md
branch: 1819-rename-torrust-tracker-metrics-to-torrust-metrics
related-pr: null
last-updated-utc: 2026-05-15 12:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/metrics/Cargo.toml
    - Cargo.toml
    - AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #1819 - Rename `torrust-tracker-metrics` to `torrust-metrics`

## Goal

Rename the Cargo crate `torrust-tracker-metrics` to `torrust-metrics` to reflect that it is
a generic Prometheus metrics integration that can be used by any Rust project, not only the
tracker.

## Background

The `metrics` package (folder `packages/metrics`) provides Prometheus metrics support. It
contains no tracker-specific domain logic and its usefulness extends beyond this repository
— for example, `torrust-index` could benefit from the same metrics infrastructure rather
than reinventing it.

The `torrust-tracker-` prefix implies a tracker-only scope that does not reflect the crate's
actual purpose. The rename:

- Makes the crate identity match its scope.
- Signals to downstream users that it is reusable outside the tracker.
- Prepares it for potential extraction to a standalone repository in a future cycle
  (see [1669-extract-torrust-metrics-to-standalone-repo.md](1669-extract-torrust-metrics-to-standalone-repo.md)).

The current crate name `torrust-tracker-metrics` is **not published on crates.io** (as of
May 2026), so the rename does not require handling a previously published name.

This issue is a subissue of EPIC #1669 (Overhaul: Packages).

## Scope

### In Scope

- Rename the crate `name` field in `packages/metrics/Cargo.toml`.
- Update all `Cargo.toml` files in the workspace that reference `torrust-tracker-metrics`
  as a dependency (root `Cargo.toml` + all dependent packages).
- Update all Rust source files that use the crate by its underscore-converted identifier
  (`torrust_tracker_metrics::`) to use `torrust_metrics::`.
- Update prose references in `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and the `metrics` package
  `README.md`.
- Verify the workspace builds and all tests pass.

### Out of Scope

- Moving the crate to a separate repository — see
  [1669-extract-torrust-metrics-to-standalone-repo.md](1669-extract-torrust-metrics-to-standalone-repo.md).
- Changes to the crate's API or behaviour.
- Publishing the crate on crates.io — that is a separate concern not required for the rename.
- Updating downstream repositories — that is a separate task per repository.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                | Notes / Expected Output                                                                |
| --- | ------ | --------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| T1  | DONE   | Rename `name` in `packages/metrics/Cargo.toml`                                                      | `name = "torrust-metrics"`                                                             |
| T2  | DONE   | Update root `Cargo.toml` workspace dependency key                                                   | `torrust-metrics = { version = ..., path = "packages/metrics" }`                       |
| T3  | DONE   | Update all dependent package `Cargo.toml` files (7 packages)                                        | Replace `torrust-tracker-metrics` key with `torrust-metrics`                           |
| T4  | DONE   | Update Rust source `use` / path references (`torrust_tracker_metrics::` → `torrust_metrics::`)      | Affects package sources and integration tests                                          |
| T5  | DONE   | Update prose in `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, `packages/metrics/README.md` | Crate name and any inline code snippets                                                |
| T6  | DONE   | Run `cargo build --workspace` and `cargo test --workspace`                                          | Clean build and all tests pass                                                         |
| T7  | DONE   | Run `linter all`                                                                                    | Exit code `0`                                                                          |
| T8  | DONE   | Update EPIC #1669 `Package Inventory` and `Desired Package State` tables                            | Move `torrust-metrics` from `torrust-tracker-` to `torrust-`; drop `Renamed from` note |

**Dependent packages to update in T3** (7 files):

- `packages/axum-rest-tracker-api-server/Cargo.toml`
- `packages/http-tracker-core/Cargo.toml`
- `packages/rest-tracker-api-core/Cargo.toml`
- `packages/swarm-coordination-registry/Cargo.toml`
- `packages/tracker-core/Cargo.toml`
- `packages/udp-tracker-core/Cargo.toml`
- `packages/udp-tracker-server/Cargo.toml`

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669
- 2026-05-21 UTC - josecelano - GitHub issue #1819 created; spec moved to open/
- 2026-05-21 UTC - josecelano - Implementation complete; build and tests pass; linter all passes

## Acceptance Criteria

- [x] `packages/metrics/Cargo.toml` declares `name = "torrust-metrics"`.
- [x] No `Cargo.toml` file in the workspace references `torrust-tracker-metrics`.
- [x] No Rust source file in the workspace uses `torrust_tracker_metrics::`.
- [x] `cargo build --workspace` succeeds with zero errors.
- [x] `cargo test --workspace` passes with zero failures.
- [x] `linter all` exits with code `0`.
- [x] `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and `packages/metrics/README.md` reflect the new crate name.
- [x] EPIC #1669 `Desired Package State` table lists `torrust-metrics` in the `torrust-` section.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete` (no unused dependencies)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Command/Steps                                                                                      | Expected Result | Status | Evidence |
| --- | ------------------------------------- | -------------------------------------------------------------------------------------------------- | --------------- | ------ | -------- |
| M1  | No stale references to old crate name | `grep -r "torrust-tracker-metrics\|torrust_tracker_metrics" . --include="*.toml" --include="*.rs"` | Zero matches    | TODO   |          |
