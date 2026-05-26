---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1829
spec-path: docs/issues/open/1829-1669-11-rename-crates-and-folders-to-match-desired-tracker-workspace.md
branch: null
related-pr: null
last-updated-utc: 2026-05-26 00:00
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

| ID  | Status | Task                                                                                                                  | Notes / Expected Output                                                                                                                  |
| --- | ------ | --------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Create migration checklist from matrix A+B and confirm owner approval for per-package sequencing                      | Checklist committed in issue comment or PR description                                                                                   |
| T2  | TODO   | Crate-only rename: `bittorrent-http-tracker-core` -> `torrust-tracker-http-tracker-core`                              | `http-tracker-core/Cargo.toml` updated; all workspace references compile                                                                 |
| T3  | TODO   | Crate-only rename: `bittorrent-tracker-core` -> `torrust-tracker-core`                                                | `tracker-core/Cargo.toml` updated; dependent crates updated                                                                              |
| T4  | TODO   | Crate-only rename: `bittorrent-tracker-client` -> `torrust-tracker-client`                                            | `tracker-client/Cargo.toml` updated; dependent crates updated                                                                            |
| T5  | TODO   | Crate-only rename: `bittorrent-udp-tracker-protocol` -> `torrust-tracker-udp-tracker-protocol`                        | `udp-protocol/Cargo.toml` updated; dependent crates updated                                                                              |
| T6  | TODO   | Crate-only rename: `bittorrent-http-tracker-protocol` -> `torrust-tracker-http-tracker-protocol`                      | `http-protocol/Cargo.toml` updated; dependent crates updated                                                                             |
| T7  | TODO   | Crate-only rename: `bittorrent-udp-tracker-core` -> `torrust-tracker-udp-tracker-core`                                | `udp-tracker-core/Cargo.toml` updated; dependent crates updated                                                                          |
| T8  | TODO   | Folder-only rename: `axum-http-tracker-server` -> `axum-http-server`                                                  | Workspace members and paths updated                                                                                                      |
| T9  | TODO   | Folder-only rename: `axum-rest-tracker-api-server` -> `axum-rest-api-server`                                          | Workspace members and paths updated                                                                                                      |
| T10 | TODO   | Folder-only rename: `rest-tracker-api-client` -> `rest-api-client`                                                    | Workspace members and paths updated                                                                                                      |
| T11 | TODO   | Folder-only rename: `rest-tracker-api-core` -> `rest-api-core`                                                        | Workspace members and paths updated                                                                                                      |
| T12 | TODO   | Folder-only rename: `udp-tracker-server` -> `udp-server`                                                              | Workspace members and paths updated                                                                                                      |
| T13 | TODO   | Update docs after all package renames (`docs/packages.md`, `AGENTS.md`, EPIC active subissues and desired-state rows) | No stale crate/folder names in tracked package catalog docs                                                                              |
| T14 | TODO   | Run full verification (`cargo build`, tests, lints)                                                                   | Green checks on the final integrated state                                                                                               |
| T15 | TODO   | Update EPIC after implementation                                                                                      | Update Active Subissues progress and EPIC sections: Package Inventory, Desired Package State, Torrust Dependency Lists (Direct, Non-dev) |

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
- [ ] Package-by-package PR sequence executed (T2-T12)
- [ ] Final docs synchronization completed (T13)
- [ ] Automatic verification completed (T14)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-26 00:00 UTC - josecelano - Drafted package-by-package rename plan for crate names and folder names.
- 2026-05-26 00:00 UTC - josecelano - GitHub issue #1829 created; spec moved to `docs/issues/open/` and metadata updated.

## Acceptance Criteria

- [ ] All crate-name-only renames in matrix A are completed with no stale old crate names.
- [ ] All folder-name-only renames in matrix B are completed with no stale old folder paths.
- [ ] Published crates listed as unchanged in this issue remain unchanged.
- [ ] `cargo build --workspace` succeeds after each package rename and at final state.
- [ ] `cargo test --workspace` passes after the full sequence.
- [ ] `linter all` exits with code `0` after the full sequence.
- [ ] `docs/packages.md`, `AGENTS.md`, and EPIC #1669 reflect final crate and folder names.

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

| ID  | Scenario                                         | Command / Steps                                                                                                                                                                                                | Expected Result                                                                                             | Status | Evidence |
| --- | ------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Old crate names removed after each crate rename  | `rg -e "bittorrent-http-tracker-core" -e "bittorrent-tracker-core" -e "bittorrent-tracker-client" -e "bittorrent-udp-tracker-protocol" -e "bittorrent-http-tracker-protocol" -e "bittorrent-udp-tracker-core"` | No stale references except historical docs intentionally preserved                                          | TODO   |          |
| M2  | Old folder paths removed after each folder move  | `rg -e "axum-http-tracker-server" -e "axum-rest-tracker-api-server" -e "rest-tracker-api-client" -e "rest-tracker-api-core" -e "udp-tracker-server"`                                                           | No stale path references in active workspace config/docs                                                    | TODO   |          |
| M3  | Workspace members list matches final folder set  | Review root `Cargo.toml` `[workspace].members`                                                                                                                                                                 | Members point to final desired folder names                                                                 | TODO   |          |
| M4  | No changes made to published crates in this task | Review diff vs baseline for published package manifests                                                                                                                                                        | `torrust-tracker-configuration`, `torrust-tracker-primitives`, and `torrust-tracker-test-helpers` unchanged | TODO   |          |

## References

- EPIC spec: [docs/issues/open/1669-overhaul-packages/EPIC.md](1669-overhaul-packages/EPIC.md)
- Decisions log: [docs/issues/open/1669-overhaul-packages/DECISIONS.md](1669-overhaul-packages/DECISIONS.md)
