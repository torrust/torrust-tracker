---
doc-type: issue
issue-type: task
status: done
priority: p3
github-issue: 1894
spec-path: docs/issues/closed/1894-1669-22-extract-torrust-located-error-to-standalone-repo.md
branch: "1894-extract-torrust-located-error-to-standalone-repo"
related-pr: 1897
last-updated-utc: 2026-06-10 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/located-error/Cargo.toml
    - Cargo.toml
    - packages/configuration/Cargo.toml
    - packages/http-protocol/Cargo.toml
    - packages/axum-server/Cargo.toml
    - packages/tracker-core/Cargo.toml
    - packages/tracker-client/Cargo.toml
    - AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/closed/1823-1669-10-rename-torrust-tracker-located-error-to-torrust-located-error.md
---

<!-- skill-link: create-issue -->

# Issue #1894 - SI-22: Extract `torrust-located-error` to a standalone repository

## Goal

Move the `torrust-located-error` crate out of the `torrust-tracker` workspace into its own
standalone repository so that it can be maintained, versioned, and published independently
of the tracker.

## Background

The `torrust-located-error` package provides an error decorator that attaches
source-location information to errors — a generic debugging utility with no tracker-specific
logic. Its only runtime dependency is `tracing`, a general-purpose structured logging crate.

The crate was renamed from `torrust-tracker-located-error` to `torrust-located-error` by
SI-10 ([#1823](https://github.com/torrust/torrust-tracker/issues/1823)) which was completed
in May 2026.

The crate has **zero workspace-path dependencies** — its only runtime dep (`tracing`) is an
external published crate. Extraction is therefore unblocked.

The crate under its new name (`torrust-located-error`) was **published on crates.io as v3.0.0**
as part of this issue. The old name (`torrust-tracker-located-error` v3.0.0) remains
published and can be yanked after downstream consumers have migrated.

This issue is a subissue of EPIC [#1669](1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Create a new standalone repository `torrust/torrust-located-error` in the Torrust GitHub
  organisation.
- Move `packages/located-error/` to the new repository via file copy (no history preservation).
- Make `Cargo.toml` self-contained (remove workspace inheritance).
- Verify the standalone repository builds and tests pass independently.
- Set up CI in the new repository (mirror the relevant CI workflows from the tracker repo).
- Publish `torrust-located-error` on crates.io from the new standalone repository.
- Update all 5 workspace consumers (see list below) to reference `torrust-located-error` as
  a crates.io version dependency instead of a path dependency.
- Remove the path-based dep registration from root `Cargo.toml` if present.
- Remove `packages/located-error` from the workspace if listed.
- Delete the `packages/located-error/` directory from the tracker repository.
- Update prose references in `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md`
  (move `torrust-located-error` to the "Extracted" section).
- Yank the old `torrust-tracker-located-error` crate on crates.io after workspace consumers
  have been migrated.

### Out of Scope

- Changes to the crate's API or behaviour.
- Updating downstream repositories outside the Torrust organisation.

### Workspace consumers to migrate

The following 5 files must have their `torrust-located-error` dep changed from a path dep
to a crates.io version dep:

- `packages/configuration/Cargo.toml`
- `packages/http-protocol/Cargo.toml`
- `packages/axum-server/Cargo.toml`
- `packages/tracker-core/Cargo.toml`
- `packages/tracker-client/Cargo.toml`

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                          | Notes / Expected Output                                                                                                                  |
| --- | ------ | --------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Verify crate has no workspace path dependencies                                               | `packages/located-error/Cargo.toml` lists only external crates (`tracing`), plus `thiserror` as dev-dep ✅                               |
| T2  | DONE   | Create standalone repository `torrust/torrust-located-error`                                  | Repo created at https://github.com/torrust/torrust-located-error - initialized with no commits on `main`                                 |
| T3  | DONE   | Copy `packages/located-error/` to the new repository (no history preservation)                | Files copied via `cp -r`; new repo contains Cargo.toml, LICENSE, README.md, src/                                                         |
| T4  | DONE   | Make `Cargo.toml` self-contained (remove workspace inheritance; pin explicit values)          | All fields explicit; no `workspace = true` entries; version set to 3.0.0 (matching old published version); authors/categories/etc pinned |
| T5  | DONE   | Verify standalone repository: `cargo build` and `cargo test` pass with no path deps           | `cargo build` success + 1 unit test + 1 doctest pass (both OK)                                                                           |
| T6  | DONE   | Set up CI in the new repository                                                               | CI workflow with `linter all` + `cargo test`; pushed to main; linter config files copied from tracker repo                               |
| T7  | DONE   | Publish `torrust-located-error` on crates.io from the standalone repository                   | Published v3.0.0 — crate page at https://crates.io/crates/torrust-located-error                                                          |
| T8  | DONE   | Update all 5 workspace consumers (see list above): path dep → crates.io version dep           | `torrust-located-error = "3.0.0"` in all 5 files; no path deps remain                                                                    |
| T9  | DONE   | Remove `packages/located-error` entry from workspace if listed                                | Not present in `[workspace]` members list; no action needed                                                                              |
| T10 | DONE   | Delete `packages/located-error/` directory from the tracker repository                        | Directory removed via `git rm -r` — 4 files removed                                                                                      |
| T11 | DONE   | Update `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`                                  | `torrust-located-error` moved to an "Extracted packages" section in all three files                                                      |
| T12 | TODO   | Yank old `torrust-tracker-located-error` from crates.io (optional, after downstream migrated) | `cargo yank torrust-tracker-located-error@3.0.0`                                                                                         |
| T13 | DONE   | Run `cargo build --workspace` and `cargo test --workspace`                                    | Clean build and all tests pass (0 failures)                                                                                              |
| T14 | DONE   | Run `linter all`                                                                              | Exit code `0` — all linters passed                                                                                                       |
| T15 | DONE   | Update EPIC #1669 tables                                                                      | Package inventory and desired state tables updated; subissue row set to `DONE`                                                           |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Standalone repository created
- [x] Source moved via file copy to new repository
- [x] CI set up and passing in new repository
- [x] `torrust-located-error` published on crates.io
- [x] Workspace consumers migrated to crates.io version dep
- [x] `packages/located-error/` removed from tracker workspace
- [x] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [x] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Old `torrust-tracker-located-error` yanked (optional)
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-06-09 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669; follows
  located-error rename in SI-10 (#1823)
- 2026-06-09 18:50 UTC - josecelano - Standalone repository `torrust/torrust-located-error` created on GitHub (empty, no commits yet)
- 2026-06-09 19:15 UTC - josecelano - Source copied via `cp -r`; Cargo.toml made self-contained (v3.0.0); build + tests verified; CI workflow + linter configs pushed to main

## Acceptance Criteria

- [x] A standalone repository `torrust/torrust-located-error` exists on GitHub.
- [x] The repository contains the crate source (file copy).
- [x] CI in the new repository passes.
- [x] `torrust-located-error` is published and visible on crates.io.
- [ ] No `Cargo.toml` in the tracker workspace references `torrust-located-error` with a path dep.
- [ ] `packages/located-error` is absent from the `[workspace]` members list in root `Cargo.toml`.
- [ ] The `packages/located-error/` directory no longer exists in the tracker repository.
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

| ID  | Scenario                                                    | Command / Steps                                               | Expected Result                         | Status | Evidence |
| --- | ----------------------------------------------------------- | ------------------------------------------------------------- | --------------------------------------- | ------ | -------- |
| M1  | No path dep on `torrust-located-error` remains in workspace | `grep -r "path.*packages/located-error" . --include="*.toml"` | Zero matches                            | TODO   |          |
| M2  | `packages/located-error/` directory is gone                 | `ls packages/located-error`                                   | `No such file or directory`             | TODO   |          |
| M3  | Standalone repo builds and tests pass independently         | In new repo: `cargo build && cargo test --workspace`          | Clean build; all tests pass             | TODO   |          |
| M4  | New crate visible on crates.io                              | Visit `https://crates.io/crates/torrust-located-error`        | Crate page exists; latest version shown | TODO   |          |
