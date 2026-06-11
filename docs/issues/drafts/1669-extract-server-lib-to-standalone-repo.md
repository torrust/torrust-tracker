---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1669-extract-server-lib-to-standalone-repo.md
branch: null
related-pr: null
last-updated-utc: 2026-06-11
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/server-lib/Cargo.toml
    - packages/server-lib/README.md
    - Cargo.toml
    - packages/axum-health-check-api-server/Cargo.toml
    - packages/axum-http-server/Cargo.toml
    - packages/axum-rest-api-server/Cargo.toml
    - packages/axum-server/Cargo.toml
    - packages/udp-server/Cargo.toml
    - packages/AGENTS.md
    - AGENTS.md
    - docs/packages.md
    - docs/templates/README.template.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Extract `torrust-server-lib` to a standalone repository

## Goal

Move the `torrust-server-lib` crate out of the `torrust-tracker` workspace into its own
standalone repository so that it can be maintained, versioned, and published independently
of the tracker.

## Background

The `torrust-server-lib` package (folder `packages/server-lib`) is a shared utility crate
for all Torrust HTTP servers. Key facts:

- **Generic utility, not tracker-specific**: it provides common Axum server infrastructure
  (compression, CORS, request tracing, logging) that is reused across all Torrust HTTP
  servers — the tracker's axum-based servers, and potentially the index or other Torrust
  projects.
- **Independent dependency tree**: its only Torrust dependency is `torrust-net-primitives`
  (version `0.1.0`), which is already published on crates.io in its own standalone
  repository. All other dependencies are external crates (`tokio`, `tower-http`, etc.).
- **Five workspace consumers**: `axum-health-check-api-server`, `axum-http-server`,
  `axum-rest-api-server`, `axum-server`, and `udp-server` all import from `server-lib`.
- **Already published on crates.io**: the crate is already published as `torrust-server-lib`.
  No additional publication step is needed — this issue only moves the source to a standalone
  repository and updates consumers.

The crate has **zero workspace-path dependencies**. All consumers currently use path
dependencies, but `torrust-server-lib` itself depends only on published crates.
Extraction is therefore unblocked.

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Create a new standalone repository `torrust/torrust-server-lib` in the Torrust GitHub
  organisation.
- Move `packages/server-lib/` to the new repository, preserving git history (using
  `git filter-repo`).
- Make `Cargo.toml` self-contained (remove workspace inheritance).
- Verify the standalone repository builds and tests pass independently.
- Set up CI in the new repository (mirror the relevant CI workflows from the tracker repo).
- Update all 5 workspace consumers (see list below) to reference `torrust-server-lib` as a
  crates.io version dependency instead of a path dependency.
- Update the root `Cargo.toml` workspace dep registration for `torrust-server-lib`.
- Remove `packages/server-lib` from the workspace `members` list in root `Cargo.toml`.
- Delete the `packages/server-lib/` directory from the tracker repository.
- Update prose references in `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md`
  (move `torrust-server-lib` to the "Extracted" section).

### Out of Scope

- Changes to the crate's API or behaviour.
- Renaming the crate (`torrust-server-lib` is appropriate for its role).
- Extracting any other utility crate.

### Prerequisites

None. `torrust-net-primitives` (the only Torrust dependency) is already published as
`0.1.0` on crates.io. The crate is already published on crates.io.

### Workspace consumers to migrate

The following 6 files must have their `torrust-server-lib` dep changed from a path dep to
a crates.io version dep (root `Cargo.toml` has the workspace dep registration, and 5
packages consume it):

- `Cargo.toml` (root — workspace dep registration)
- `packages/axum-health-check-api-server/Cargo.toml`
- `packages/axum-http-server/Cargo.toml`
- `packages/axum-rest-api-server/Cargo.toml`
- `packages/axum-server/Cargo.toml`
- `packages/udp-server/Cargo.toml`

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                               | Notes / Expected Output                                                              |
| --- | ------ | -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| T1  | TODO   | Verify crate has no workspace path dependencies                                                    | `packages/server-lib/Cargo.toml` lists only external crates + `torrust-net-primitives` (published) ✅ |
| T2  | TODO   | Create standalone repository `torrust/torrust-server-lib`                                          | Repo created at https://github.com/torrust/torrust-server-lib                        |
| T3  | TODO   | Copy `packages/server-lib/` to the new repository (history preservation where practical)           | Files copied to new repo                                                             |
| T4  | TODO   | Make `Cargo.toml` self-contained (remove workspace inheritance; pin explicit values)               | All fields explicit; no `workspace = true` entries                                   |
| T5  | TODO   | Verify standalone repository: `cargo build` and `cargo test` pass with no path deps                | Build and tests pass; no path deps remain                                            |
| T6  | TODO   | Set up CI in the new repository                                                                    | CI workflow with `linter all` + `cargo test`                                         |
| T7  | TODO   | Update all 6 workspace consumers (see list above): path dep → crates.io version dep                | `torrust-server-lib = "X.Y.Z"` in all 6 files; no path deps remain                  |
| T8  | TODO   | Remove `packages/server-lib` entry from workspace `members` in root `Cargo.toml`                   | `packages/server-lib` absent from `[workspace]` members list                         |
| T9  | TODO   | Delete `packages/server-lib/` directory from the tracker repository                                | Directory removed via `git rm -r`                                                     |
| T10 | TODO   | Update `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`                                       | `torrust-server-lib` moved to an "Extracted packages" section                        |
| T11 | TODO   | Run `cargo build --workspace` and `cargo test --workspace`                                         | Clean build and all tests pass                                                       |
| T12 | TODO   | Run `linter all`                                                                                   | Exit code `0`                                                                         |
| T13 | TODO   | Update EPIC #1669 tables                                                                           | Package inventory and desired state tables updated; subissue row set to `DONE`       |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Standalone repository created
- [ ] Source moved with history preserved
- [ ] CI set up and passing in new repository
- [ ] Workspace consumers migrated to crates.io version dep
- [ ] `packages/server-lib/` removed from tracker workspace
- [ ] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-06-11 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669

## Acceptance Criteria

- [ ] A standalone repository `torrust/torrust-server-lib` exists on GitHub.
- [ ] The repository contains the crate source (history preservation where practical).
- [ ] CI in the new repository passes.
- [ ] No `Cargo.toml` in the tracker workspace references `torrust-server-lib` with a path dep.
- [ ] `packages/server-lib` is absent from the `[workspace]` members list in root `Cargo.toml`.
- [ ] The `packages/server-lib/` directory no longer exists in the tracker repository.
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

| ID  | Scenario                                                   | Command / Steps                                              | Expected Result                         | Status | Evidence |
| --- | ---------------------------------------------------------- | ------------------------------------------------------------ | --------------------------------------- | ------ | -------- |
| M1  | No path dep on `torrust-server-lib` remains in workspace   | `grep -r "path.*packages/server-lib" . --include="*.toml"`   | Zero matches                            | TODO   |          |
| M2  | `packages/server-lib/` directory is gone                   | `ls packages/server-lib`                                     | `No such file or directory`             | TODO   |          |
| M3  | Standalone repo builds and tests pass independently        | In new repo: `cargo build && cargo test --workspace`         | Clean build; all tests pass             | TODO   |          |
| M4  | `torrust-server-lib` CI green in new repository            | Check GitHub Actions on `torrust/torrust-server-lib`         | All workflows green                     | TODO   |          |