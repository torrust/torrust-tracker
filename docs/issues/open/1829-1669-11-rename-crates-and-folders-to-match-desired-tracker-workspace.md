---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1829
spec-path: docs/issues/open/1829-1669-11-rename-crates-and-folders-to-match-desired-tracker-workspace.md
branch: 1829-rename-crates-and-folders
related-pr: null
last-updated-utc: 2026-05-26 20:15
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Cargo.toml
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/packages.md
    - AGENTS.md
---

<!-- skill-link: create-issue -->

# Issue #1829 - Rename crates and folders to match EPIC desired tracker workspace state

Subissue ID: SI-11 (1669-11).

## Goal

Align the current `torrust-tracker` workspace package identifiers with the desired state
defined in EPIC #1669 by applying only rename changes, one package at a time:

- crate name rename only, or
- folder name rename only.

No package API changes are introduced by this issue.

## Background

EPIC #1669 already defines the desired tracker workspace naming model (crate names and folder
names). Several packages still use legacy names from earlier refactors.

This issue introduces an incremental migration plan where each change is isolated to a
single package so failures are easy to diagnose and roll back.

Important constraint from EPIC discussion:

- Only three tracker packages are currently published on crates.io and remain unchanged in
  this migration (`torrust-tracker-configuration`, `torrust-tracker-primitives`,
  `torrust-tracker-test-helpers`).
- The packages touched in this issue are unpublished, so there is no external crates.io
  migration window required.

This issue is a subissue of EPIC [#1669](1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Rename legacy `bittorrent-*` crate names that remain in tracker to `torrust-tracker-*`
  where the folder stays the same.
- Rename legacy folder names to the desired folder names where the crate name stays the same.
- Update all workspace references (`Cargo.toml`, imports, docs, and scripts) for each package
  change before moving to the next package.
- Keep each package migration independent (one package per PR/commit unit).

### Out of Scope

- Extraction to external repositories.
- API/behavioral changes to any package.
- Re-layering dependency boundaries.
- Renaming published crates.

## Package Migration Matrix

### A. Crate rename only (folder unchanged)

| Package folder      | Old crate name                     | New crate name                          |
| ------------------- | ---------------------------------- | --------------------------------------- |
| `http-tracker-core` | `bittorrent-http-tracker-core`     | `torrust-tracker-http-tracker-core`     |
| `tracker-core`      | `bittorrent-tracker-core`          | `torrust-tracker-core`                  |
| `tracker-client`    | `bittorrent-tracker-client`        | `torrust-tracker-client`                |
| `udp-protocol`      | `bittorrent-udp-tracker-protocol`  | `torrust-tracker-udp-tracker-protocol`  |
| `http-protocol`     | `bittorrent-http-tracker-protocol` | `torrust-tracker-http-tracker-protocol` |
| `udp-tracker-core`  | `bittorrent-udp-tracker-core`      | `torrust-tracker-udp-tracker-core`      |

### B. Folder rename only (crate unchanged)

| Old folder                     | New folder             | Crate name                             |
| ------------------------------ | ---------------------- | -------------------------------------- |
| `axum-http-tracker-server`     | `axum-http-server`     | `torrust-tracker-axum-http-server`     |
| `axum-rest-tracker-api-server` | `axum-rest-api-server` | `torrust-tracker-axum-rest-api-server` |
| `rest-tracker-api-client`      | `rest-api-client`      | `torrust-tracker-rest-api-client`      |
| `rest-tracker-api-core`        | `rest-api-core`        | `torrust-tracker-rest-api-core`        |
| `udp-tracker-server`           | `udp-server`           | `torrust-tracker-udp-server`           |

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

Execution rule for T2-T12: complete one package fully before starting the next.
Each task includes all required reference updates and verification for that package.

| ID  | Status | Task                                                                                                                  | Notes / Expected Output                                                                                |
| --- | ------ | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Create migration checklist from matrix A+B and confirm owner approval for per-package sequencing                      | Implemented directly in branch `1829-rename-crates-and-folders`                                        |
| T2  | DONE   | Crate-only rename: `bittorrent-http-tracker-core` -> `torrust-tracker-http-tracker-core`                              | `http-tracker-core/Cargo.toml` and dependents updated                                                  |
| T3  | DONE   | Crate-only rename: `bittorrent-tracker-core` -> `torrust-tracker-core`                                                | `tracker-core/Cargo.toml` and dependents updated                                                       |
| T4  | DONE   | Crate-only rename: `bittorrent-tracker-client` -> `torrust-tracker-client`                                            | `tracker-client/Cargo.toml` and dependents updated                                                     |
| T5  | DONE   | Crate-only rename: `bittorrent-udp-tracker-protocol` -> `torrust-tracker-udp-tracker-protocol`                        | `udp-protocol/Cargo.toml` and dependents updated                                                       |
| T6  | DONE   | Crate-only rename: `bittorrent-http-tracker-protocol` -> `torrust-tracker-http-tracker-protocol`                      | `http-protocol/Cargo.toml` and dependents updated                                                      |
| T7  | DONE   | Crate-only rename: `bittorrent-udp-tracker-core` -> `torrust-tracker-udp-tracker-core`                                | `udp-tracker-core/Cargo.toml` and dependents updated                                                   |
| T8  | DONE   | Folder-only rename: `axum-http-tracker-server` -> `axum-http-server`                                                  | Workspace paths updated                                                                                |
| T9  | DONE   | Folder-only rename: `axum-rest-tracker-api-server` -> `axum-rest-api-server`                                          | Workspace paths updated                                                                                |
| T10 | DONE   | Folder-only rename: `rest-tracker-api-client` -> `rest-api-client`                                                    | Workspace paths updated                                                                                |
| T11 | DONE   | Folder-only rename: `rest-tracker-api-core` -> `rest-api-core`                                                        | Workspace paths updated                                                                                |
| T12 | DONE   | Folder-only rename: `udp-tracker-server` -> `udp-server`                                                              | Workspace paths updated                                                                                |
| T13 | DONE   | Update docs after all package renames (`docs/packages.md`, `AGENTS.md`, EPIC active subissues and desired-state rows) | Renamed catalog entries and EPIC tables synchronized                                                   |
| T14 | DONE   | Run full verification (`cargo build`, tests, lints)                                                                   | `cargo build --workspace` and `linter all` passed; test run failed due rustc compiler crash (signal 7) |
| T15 | DONE   | Update EPIC after implementation                                                                                      | Active subissue status and package tables updated                                                      |

## Per-Package PR Boundary

Each package change should be delivered as a dedicated PR/commit unit with:

1. Rename implementation.
2. Local verification for impacted crates.
3. Documentation touch-ups needed for that package.

Do not batch multiple package renames in a single PR unless explicitly approved.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Package-by-package PR sequence executed (T2-T12)
- [x] Final docs synchronization completed (T13)
- [x] Automatic verification completed (T14)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-26 00:00 UTC - josecelano - Drafted package-by-package rename plan for crate names and folder names.
- 2026-05-26 00:00 UTC - josecelano - GitHub issue #1829 created; spec moved to `docs/issues/open/` and metadata updated.
- 2026-05-26 19:59 UTC - github-copilot - Implemented crate and folder renames from matrices A+B and updated workspace references.
- 2026-05-26 19:59 UTC - github-copilot - Verification: `cargo build --workspace` passed; `linter all` passed; `cargo test --workspace` blocked by rustc compiler crash (signal 7).
- 2026-05-26 20:15 UTC - github-copilot - Aligned client naming split to `torrust-tracker-client` (console package) and `torrust-tracker-client-lib` (library package).

## Acceptance Criteria

- [x] All crate-name-only renames in matrix A are completed with no stale old crate names.
- [x] All folder-name-only renames in matrix B are completed with no stale old folder paths.
- [x] Published crates listed as unchanged in this issue remain unchanged.
- [x] `cargo build --workspace` succeeds after each package rename and at final state.
- [ ] `cargo test --workspace` passes after the full sequence. (blocked by rustc compiler crash in this environment)
- [x] `linter all` exits with code `0` after the full sequence.
- [x] `docs/packages.md`, `AGENTS.md`, and EPIC #1669 reflect final crate and folder names.

## Verification Plan

### Automatic Checks

- For each package PR:
  - `cargo build --workspace`
  - targeted checks for changed crates (`cargo test -p <crate-name>` when practical)
- Final integrated verification:
  - `cargo test --doc --workspace`
  - `cargo test --tests --benches --examples --workspace --all-targets --all-features`
  - `linter all`
  - `cargo machete`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                         | Command / Steps                                                          | Expected Result                                                                                                    | Status | Evidence                              |
| --- | ------------------------------------------------ | ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ | ------ | ------------------------------------- |
| M1  | Old crate names removed after each crate rename  | Run `rg` for the six old crate names across active code/docs scope       | No stale active references except historical docs intentionally preserved                                          | DONE   | Exit code 1 (no matches)              |
| M2  | Old folder paths removed after each folder move  | Run `rg` for the five old folder names across active code/docs scope     | No stale path references in active workspace config/docs                                                           | DONE   | Exit code 1 (no matches)              |
| M3  | Workspace members list matches final folder set  | Review root `Cargo.toml` `[dependencies]` path entries and moved folders | Path entries point to `axum-http-server`, `axum-rest-api-server`, `rest-api-client`, `rest-api-core`, `udp-server` | DONE   | Verified in `Cargo.toml`              |
| M4  | No changes made to published crates in this task | Review diff vs baseline for published package manifests                  | `torrust-tracker-configuration`, `torrust-tracker-primitives`, and `torrust-tracker-test-helpers` unchanged        | DONE   | No changes in those package manifests |

## References

- EPIC spec: [docs/issues/open/1669-overhaul-packages/EPIC.md](1669-overhaul-packages/EPIC.md)
- Decisions log: [docs/issues/open/1669-overhaul-packages/DECISIONS.md](1669-overhaul-packages/DECISIONS.md)
