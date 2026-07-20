---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md
branch: null
related-pr: null
last-updated-utc: 2026-05-15 12:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - console/tracker-client/Cargo.toml
    - Cargo.toml
    - AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---


# Issue #[To be assigned] - Extract `torrust-tracker-client` to standalone repository

## Goal

Extract the `torrust-tracker-client` CLI tool from the tracker workspace into its own
standalone repository so that it can evolve independently, be installed without the full
tracker source tree, and follow its own versioning and release cadence.

## Background

The `torrust-tracker-client` package (folder `console/tracker-client`) is a collection of
console clients for making requests to BitTorrent trackers. Key facts:

- **CLI tool, not a library**: its primary artefact is a binary. It is not consumed as a
  library dependency by other crates in the workspace.
- **Separate license**: LGPL-3.0, unlike the tracker's AGPL-3.0-only workspace license.
  Having a differently licensed binary in the same workspace creates a mixed-license surface
  that is harder to communicate to contributors and downstream users.
- **Independent evolution**: the CLI tool's feature set and release cadence are driven by
  user interaction needs, not by tracker server internals. Tying its version to the tracker
  workspace version is unnecessary coupling.
- **Extraction was always the intent**: the package README states _"We're currently
  extracting and refining common functionality from the Torrust Tracker"_, confirming that
  moving it to its own repository is the designed direction.

The extraction is currently **blocked** by two unpublished workspace dependencies:

| Dependency                                           | Current status             |
| ---------------------------------------------------- | -------------------------- |
| `torrust-tracker-udp-tracker-protocol`               | Not published on crates.io |
| `torrust-tracker-client` (`packages/tracker-client`) | Not published on crates.io |

The third workspace dependency (`torrust-tracker-configuration`) is already published.
Do not start T3 or later tasks until T1 is satisfied.

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Create (or confirm) the target standalone repository for the CLI tool.
- Move the `console/tracker-client/` source to the new repository, preserving git history.
- Update the crate's `Cargo.toml` in the new repo: replace workspace path dependencies with
  published crates.io version dependencies once the blocking crates are published.
- Set up CI in the new repository (build, test, lint, publish/release workflow).
- Remove `console/tracker-client/` from the tracker workspace:
  - Remove from the `members` list in the root `Cargo.toml`.
  - Remove the workspace dependency entry from the root `Cargo.toml`.
  - Delete the `console/tracker-client/` directory from the tracker repo.
- Update `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md`.

### Out of Scope

- Changes to the CLI tool's features or behaviour.
- Publishing `torrust-tracker-udp-tracker-protocol` or the library crate
  `torrust-tracker-client` (`packages/tracker-client`) on crates.io
  — those are separate subissues.
- Renaming the crate: `torrust-tracker-client` is an appropriate name and is kept.

### Prerequisites

This issue is **blocked** until the following crates are published on crates.io:

1. `torrust-tracker-udp-tracker-protocol`
2. `torrust-tracker-client` (`packages/tracker-client`)

Do not begin T3 or later until both are available.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status  | Task                                                                                                        | Notes / Expected Output                                                                   |
| --- | ------- | ----------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| T1  | BLOCKED | Confirm `torrust-tracker-udp-tracker-protocol` and the library crate `torrust-tracker-client` are published | Prerequisite; unblocks T3 and all later tasks                                             |
| T2  | TODO    | Create (or confirm) the target standalone repository                                                        | Repo exists with README and LICENSE committed                                             |
| T3  | TODO    | Move crate source to the new repository, preserving git history                                             | Use `git filter-repo` or subtree split; history preserved under `console/tracker-client/` |
| T4  | TODO    | Update `Cargo.toml` in the new repo: replace path deps with published crates.io version deps                | `torrust-tracker-udp-tracker-protocol = "X.Y.Z"`, `torrust-tracker-client = "X.Y.Z"`      |
| T5  | TODO    | Set up CI in the new repository (build, test, lint, release workflow)                                       | CI green on first push                                                                    |
| T6  | TODO    | Remove `console/tracker-client/` from workspace members and workspace dep in root `Cargo.toml`              | `cargo build --workspace` succeeds without the local crate                                |
| T7  | TODO    | Delete `console/tracker-client/` directory from the tracker repo                                            | Directory gone; workspace still builds                                                    |
| T8  | TODO    | Update `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and any README references                     | No stale references to the console client remain in the tracker docs                      |
| T9  | TODO    | Run `cargo build --workspace`, `cargo test --workspace`, `linter all`                                       | All green                                                                                 |
| T10 | TODO    | Update EPIC #1669 `Package Inventory` and `Desired Package State` tables                                    | Mark `torrust-tracker-client` as extracted; remove from workspace member list             |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] Blocking dependencies (`torrust-tracker-udp-tracker-protocol`, library crate `torrust-tracker-client`) published on crates.io
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669

## Acceptance Criteria

- [ ] `console/tracker-client/` directory no longer exists in the tracker workspace.
- [ ] Root `Cargo.toml` does not list `console/tracker-client` as a workspace member.
- [ ] No `Cargo.toml` in the tracker workspace references `torrust-tracker-client` as a path dep.
- [ ] `cargo build --workspace` succeeds with zero errors after the removal.
- [ ] `cargo test --workspace` passes with zero failures after the removal.
- [ ] `linter all` exits with code `0`.
- [ ] The new repository has passing CI and a clean `cargo build`.
- [ ] `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md` no longer list
      `torrust-tracker-client` as a workspace package.
- [ ] EPIC #1669 `Package Inventory` and `Desired Package State` tables are updated to
      reflect the extraction.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete` (no unused dependencies)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                  | Command / Steps                                                                                                   | Expected Result              | Status | Evidence |
| --- | ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ---------------------------- | ------ | -------- |
| M1  | No stale workspace reference to old crate | `grep -r "torrust-tracker-client\|console/tracker-client" . --include="*.toml" --include="*.rs" --include="*.md"` | Zero matches in tracker repo | TODO   |          |
| M2  | New repository CI passes                  | Check CI status on the new repository's default branch                                                            | All checks pass              | TODO   |          |
| M3  | Crate builds from new repository          | Clone new repo; `cargo build`                                                                                     | Clean build                  | TODO   |          |
