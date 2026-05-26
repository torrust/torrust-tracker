---
doc-type: issue
issue-type: task
status: draft
priority: p3
github-issue: null
spec-path: docs/issues/drafts/1669-extract-torrust-clock-to-standalone-repo.md
branch: null
related-pr: null
last-updated-utc: 2026-05-15 12:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/clock/Cargo.toml
    - Cargo.toml
    - docs/packages.md
    - AGENTS.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/closed/1821-1669-09-rename-torrust-tracker-clock-to-torrust-clock.md
    - docs/issues/closed/1790-move-duration-since-unix-epoch-to-torrust-tracker-clock.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Extract `torrust-clock` to a standalone repository

## Goal

Move the `torrust-clock` crate out of the `torrust-tracker` workspace into its own
standalone repository so that it can be maintained, versioned, and published independently
of the tracker.

## Background

The `torrust-clock` package provides a mockable time abstraction for deterministic testing.
It contains no tracker-specific logic, making it a general-purpose utility reusable by any
Rust project (e.g., `torrust-index` already contains a local copy of equivalent clock
code). Keeping it inside the tracker workspace couples its release cycle to the tracker's
and limits its visibility to potential consumers.

After the preceding subissues are complete (`torrust-tracker-clock` renamed to
`torrust-clock` and `DurationSinceUnixEpoch` moved from `torrust-tracker-primitives` to
`torrust-clock`), the crate has **zero workspace-path dependencies** — all its runtime
deps (`chrono`, `tracing`) are published crates. Extraction is therefore unblocked.

**Prerequisites**:

1. Clock rename subissue
   ([1821-1669-09-rename-torrust-tracker-clock-to-torrust-clock.md](../closed/1821-1669-09-rename-torrust-tracker-clock-to-torrust-clock.md))
   must be complete — in particular T8 (publish `torrust-clock` on crates.io).
2. `DurationSinceUnixEpoch` move subissue
   ([1790-move-duration-since-unix-epoch-to-torrust-tracker-clock.md](../closed/1790-move-duration-since-unix-epoch-to-torrust-tracker-clock.md))
   must be complete — in particular T4 (`torrust-tracker-primitives` dep removed from
   `packages/clock/Cargo.toml`).

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Create a new standalone repository `torrust/torrust-clock` in the Torrust GitHub
  organization.
- Move `packages/clock/` to the new repository, preserving git history (using
  `git filter-repo`).
- Verify the standalone repository builds and tests pass independently.
- Set up CI in the new repository (mirror the relevant CI workflows from the tracker repo).
- Update all 11 workspace consumers (root `Cargo.toml` + 10 packages) to reference
  `torrust-clock` as a crates.io version dependency instead of a path dependency.
- Remove `packages/clock` from the workspace `members` list in root `Cargo.toml`.
- Delete the `packages/clock/` directory from the tracker repository.
- Update prose references in `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md`
  (move `torrust-clock` to the "Extracted" section).

### Out of Scope

- Changes to the crate's API or behaviour.
- Yanking the old crates.io name `torrust-tracker-clock` (that is handled by the rename
  subissue T11, after `torrust-index` migration).

### Workspace consumers to migrate in T5

The following 11 files must have their `torrust-clock` dep changed from a path dep to a
crates.io version dep:

- `Cargo.toml` (root — workspace dep registration)
- `packages/axum-health-check-api-server/Cargo.toml`
- `packages/axum-http-tracker-server/Cargo.toml`
- `packages/axum-rest-tracker-api-server/Cargo.toml`
- `packages/http-protocol/Cargo.toml`
- `packages/http-tracker-core/Cargo.toml`
- `packages/swarm-coordination-registry/Cargo.toml`
- `packages/tracker-core/Cargo.toml`
- `packages/torrent-repository-benchmarking/Cargo.toml`
- `packages/udp-tracker-core/Cargo.toml`
- `packages/udp-tracker-server/Cargo.toml`

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                             | Notes / Expected Output                                                        |
| --- | ------ | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------ |
| T1  | TODO   | Verify clock rename completion state (T8 of rename spec: `torrust-clock` published on crates.io) | `packages/clock/Cargo.toml` has `name = "torrust-clock"`                       |
| T2  | TODO   | Verify `DurationSinceUnixEpoch` move completion state (T4 of move spec)                          | `packages/clock/Cargo.toml` does not list `torrust-tracker-primitives`         |
| T3  | TODO   | Create standalone repository `torrust/torrust-clock`                                             | Empty repo with license and basic README                                       |
| T4  | TODO   | Move `packages/clock/` to the new repository, preserving git history (`git filter-repo`)         | New repo contains full history for `packages/clock/`                           |
| T5  | TODO   | Verify standalone repository: `cargo build` and `cargo test` pass with no path deps              | Clean build in the new repo; Cargo.toml has only external (non-path) deps      |
| T6  | TODO   | Set up CI in the new repository                                                                  | Copy/adapt relevant GitHub Actions workflows; CI passes                        |
| T7  | TODO   | Update all 11 workspace consumers (see list above): path dep → crates.io version dep             | `torrust-clock = "X.Y.Z"` (or workspace dep) in each Cargo.toml                |
| T8  | TODO   | Remove `packages/clock` entry from workspace `members` in root `Cargo.toml`                      | `packages/clock` absent from `[workspace]` members list                        |
| T9  | TODO   | Delete `packages/clock/` directory from the tracker repository                                   | Directory removed; `git status` shows deletions                                |
| T10 | TODO   | Update `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`                                     | `torrust-clock` moved to an "Extracted packages" section                       |
| T11 | TODO   | Run `cargo build --workspace` and `cargo test --workspace`                                       | Clean build and all tests pass                                                 |
| T12 | TODO   | Run `linter all`                                                                                 | Exit code `0`                                                                  |
| T13 | TODO   | Update EPIC #1669 tables                                                                         | Package inventory and desired state tables updated; subissue row set to `DONE` |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] Clock rename subissue complete (prerequisite 1)
- [ ] `DurationSinceUnixEpoch` move subissue complete (prerequisite 2)
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Standalone repository created
- [ ] Source moved with history preserved
- [ ] CI set up and passing in new repository
- [ ] Workspace consumers migrated to crates.io version dep
- [ ] `packages/clock/` removed from tracker workspace
- [ ] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669; follows
  clock rename and DurationSinceUnixEpoch move subissues

## Acceptance Criteria

- [ ] A standalone repository `torrust/torrust-clock` exists on GitHub.
- [ ] The repository contains the full git history for `packages/clock/`.
- [ ] CI in the new repository passes.
- [ ] No `Cargo.toml` in the tracker workspace references `torrust-clock` with a path dep.
- [ ] `packages/clock` is absent from the `[workspace]` members list in root `Cargo.toml`.
- [ ] The `packages/clock/` directory no longer exists in the tracker repository.
- [ ] `cargo build --workspace` in the tracker repository succeeds with zero errors.
- [ ] `cargo test --workspace` in the tracker repository passes with zero failures.
- [ ] `linter all` exits with code `0`.
- [ ] `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md` reflect the extraction.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete` (no unused dependencies)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                | Command / Steps                                       | Expected Result             | Status | Evidence |
| --- | ------------------------------------------------------- | ----------------------------------------------------- | --------------------------- | ------ | -------- |
| M1  | No path dep on `torrust-clock` remains in the workspace | `grep -r "path.*packages/clock" . --include="*.toml"` | Zero matches                | TODO   |          |
| M2  | `packages/clock/` directory is gone                     | `ls packages/clock`                                   | `No such file or directory` | TODO   |          |
| M3  | Standalone repo builds and tests pass independently     | In new repo: `cargo build && cargo test --workspace`  | Clean build; all tests pass | TODO   |          |
| M4  | `torrust-clock` CI green in new repository              | Check GitHub Actions on `torrust/torrust-clock`       | All workflows green         | TODO   |          |
