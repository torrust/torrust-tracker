---
doc-type: issue
issue-type: task
status: done
priority: p3
github-issue: 1885
spec-path: docs/issues/closed/1885-1669-20-extract-torrust-net-primitives-to-standalone-repo.md
branch: "1885-extract-torrust-net-primitives-to-standalone-repo"
related-pr: 1893
last-updated-utc: 2026-06-10 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/net-primitives/Cargo.toml
    - Cargo.toml
    - packages/axum-health-check-api-server/Cargo.toml
    - packages/axum-http-server/Cargo.toml
    - packages/axum-rest-api-server/Cargo.toml
    - packages/http-tracker-core/Cargo.toml
    - packages/primitives/Cargo.toml
    - packages/server-lib/Cargo.toml
    - packages/tracker-client/Cargo.toml
    - packages/udp-server/Cargo.toml
    - packages/udp-tracker-core/Cargo.toml
    - AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/closed/1797-1669-05-create-torrust-net-primitives-and-move-service-binding.md
---


# Issue #1885 - Extract `torrust-net-primitives` to a standalone repository

## Goal

Move the `torrust-net-primitives` crate out of the `torrust-tracker` workspace into its own
standalone repository so that it can be maintained, versioned, and published independently
of the tracker.

## Background

The `torrust-net-primitives` package provides generic networking primitive types
(`ServiceBinding`, etc.) used across server components. It contains no tracker-specific
logic, making it a general-purpose utility crate reusable by any Torrust project
(e.g., `torrust-index`).

The package was created by SI-05 ([#1797](https://github.com/torrust/torrust-tracker/issues/1797))
which moved `ServiceBinding` from `torrust-tracker-primitives` and established
`torrust-net-primitives` as the right home for generic networking types. Standalone
extraction was flagged as the intended next step at the time.

The crate has **zero workspace-path dependencies** — all its runtime deps (`serde`,
`thiserror`, `url`) are published crates. Extraction is therefore unblocked.

The crate is **not yet published on crates.io**; publication from the standalone repository
is part of this issue's scope.

This issue is a subissue of EPIC [#1669](1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Create a new standalone repository `torrust/torrust-net-primitives` in the Torrust GitHub
  organisation.
- Move `packages/net-primitives/` to the new repository, preserving git history
  (using `git filter-repo`).
- Make `Cargo.toml` self-contained (remove workspace inheritance).
- Verify the standalone repository builds and tests pass independently.
- Set up CI in the new repository (mirror the relevant CI workflows from the tracker repo).
- Publish `torrust-net-primitives` on crates.io from the new standalone repository.
- Update all 10 workspace consumers (root `Cargo.toml` + 9 packages) to reference
  `torrust-net-primitives` as a crates.io version dependency instead of a path dependency.
- Remove `packages/net-primitives` from the workspace `members` list in root `Cargo.toml`.
- Delete the `packages/net-primitives/` directory from the tracker repository.
- Update prose references in `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md`
  (move `torrust-net-primitives` to the "Extracted" section).

### Out of Scope

- Changes to the crate's API or behaviour.
- Updating other downstream repositories (e.g., `torrust-index`) — separate task per repo.
- Extracting other crates from this workspace — each gets its own subissue.

### Workspace consumers to migrate

The following 10 files must have their `torrust-net-primitives` dep changed from a path dep
to a crates.io version dep:

- `Cargo.toml` (root — workspace dep registration)
- `packages/axum-health-check-api-server/Cargo.toml`
- `packages/axum-http-server/Cargo.toml`
- `packages/axum-rest-api-server/Cargo.toml`
- `packages/http-tracker-core/Cargo.toml`
- `packages/primitives/Cargo.toml`
- `packages/server-lib/Cargo.toml`
- `packages/tracker-client/Cargo.toml`
- `packages/udp-server/Cargo.toml`
- `packages/udp-tracker-core/Cargo.toml`

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                         | Notes / Expected Output                                                                          |
| --- | ------ | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Verify crate has no workspace path dependencies                                              | `packages/net-primitives/Cargo.toml` lists only external crates (`serde`, `thiserror`, `url`) ✅ |
| T2  | DONE   | Create standalone repository `torrust/torrust-net-primitives`                                | Repo created at https://github.com/torrust/torrust-net-primitives                                |
| T3  | DONE   | Copy `packages/net-primitives/` to the new repository (history preservation where practical) | Files copied to new repo                                                                         |
| T4  | DONE   | Make `Cargo.toml` self-contained (remove workspace inheritance; pin explicit values)         | All fields explicit; no `workspace = true` entries                                               |
| T5  | DONE   | Verify standalone repository: `cargo build` and `cargo test` pass with no path deps          | Build and tests pass; no path deps remain                                                        |
| T6  | DONE   | Set up CI in the new repository                                                              | CI workflow with `linter all` + `cargo test`                                                     |
| T7  | DONE   | Publish `torrust-net-primitives` on crates.io from the standalone repository                 | Published v0.1.0                                                                                 |
| T8  | DONE   | Update all 10 workspace consumers (see list above): path dep → crates.io version dep         | `torrust-net-primitives = "0.1.0"` in all 10 files; no path deps remain                          |
| T9  | DONE   | Remove `packages/net-primitives` entry from workspace `members` in root `Cargo.toml`         | `packages/net-primitives` absent from `[workspace]` members list                                 |
| T10 | DONE   | Delete `packages/net-primitives/` directory from the tracker repository                      | Directory removed via `git rm -r`                                                                |
| T11 | DONE   | Update `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`                                 | `torrust-net-primitives` moved to an "Extracted packages" section                                |
| T12 | DONE   | Run `cargo build --workspace` and `cargo test --workspace`                                   | Clean build and all tests pass                                                                   |
| T13 | DONE   | Run `linter all`                                                                             | Exit code `0`                                                                                    |
| T14 | TODO   | Update EPIC #1669 tables                                                                     | Package inventory and desired state tables updated; subissue row set to `DONE`                   |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Standalone repository created
- [ ] Source moved with history preserved
- [ ] CI set up and passing in new repository
- [ ] `torrust-net-primitives` published on crates.io
- [ ] Workspace consumers migrated to crates.io version dep
- [ ] `packages/net-primitives/` removed from tracker workspace
- [ ] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-06-05 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669; follows
  net-primitives creation in SI-05 (#1797)
- 2026-06-05 00:00 UTC - josecelano - GitHub issue #1885 created; spec promoted to docs/issues/open/

> **Note — deferred setup for `torrust/torrust-net-primitives`**: The following work may be
> intentionally deferred to a follow-up, to be done when the first change or publication is
> needed:
>
> - GitHub Actions CI workflows (build, test, lint, publish)
> - AI agent configuration (AGENTS.md, `.github/skills/`)
> - Release and versioning process definition

## Acceptance Criteria

- [ ] A standalone repository `torrust/torrust-net-primitives` exists on GitHub.
- [ ] The repository contains the crate source (history preservation where practical).
- [ ] CI in the new repository passes _(may be deferred — see note below)_.
- [ ] `torrust-net-primitives` is published and visible on crates.io.
- [ ] No `Cargo.toml` in the tracker workspace references `torrust-net-primitives` with a path dep.
- [ ] `packages/net-primitives` is absent from the `[workspace]` members list in root `Cargo.toml`.
- [ ] The `packages/net-primitives/` directory no longer exists in the tracker repository.
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

| ID  | Scenario                                                     | Command / Steps                                                | Expected Result                         | Status | Evidence |
| --- | ------------------------------------------------------------ | -------------------------------------------------------------- | --------------------------------------- | ------ | -------- |
| M1  | No path dep on `torrust-net-primitives` remains in workspace | `grep -r "path.*packages/net-primitives" . --include="*.toml"` | Zero matches                            | TODO   |          |
| M2  | `packages/net-primitives/` directory is gone                 | `ls packages/net-primitives`                                   | `No such file or directory`             | TODO   |          |
| M3  | Standalone repo builds and tests pass independently          | In new repo: `cargo build && cargo test --workspace`           | Clean build; all tests pass             | TODO   |          |
| M4  | New crate visible on crates.io                               | Visit `https://crates.io/crates/torrust-net-primitives`        | Crate page exists; latest version shown | TODO   |          |
| M5  | `torrust-net-primitives` CI green in new repository          | Check GitHub Actions on `torrust/torrust-net-primitives`       | All workflows green                     | TODO   |          |
