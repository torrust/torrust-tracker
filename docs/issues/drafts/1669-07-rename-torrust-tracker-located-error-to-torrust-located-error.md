---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1669-07-rename-torrust-tracker-located-error-to-torrust-located-error.md
branch: null
related-pr: null
last-updated-utc: 2026-05-15 12:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/located-error/Cargo.toml
    - Cargo.toml
    - AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Rename `torrust-tracker-located-error` to `torrust-located-error`

## Goal

Rename the Cargo crate `torrust-tracker-located-error` to `torrust-located-error` to reflect
that it is a generic, tracker-independent error decoration utility that can be used in any
Rust project (e.g., `torrust-index`).

## Background

The `located-error` package (folder `packages/located-error`) provides an error decorator
that attaches source-location information to errors — a generic debugging utility with no
tracker-specific logic. Its only runtime dependency is `tracing`, a general-purpose
structured logging crate. There is nothing in the implementation that ties it to the
BitTorrent tracker.

The `torrust-tracker-` prefix implies a tracker-only scope that does not reflect the crate's
actual purpose. The rename:

- Makes the crate identity match its scope.
- Signals to downstream users that it is reusable outside the tracker.
- Prepares it for potential extraction to a standalone repository in a future cycle.

The current crate name `torrust-tracker-located-error` is **published on crates.io** (as of
May 2026). The rename requires publishing the new name `torrust-located-error` and handling
the old published name (deprecation notice, then yank after downstream migration).

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Rename the `name` field in `packages/located-error/Cargo.toml`.
- Update all `Cargo.toml` files in the workspace that reference `torrust-tracker-located-error`
  as a dependency (root `Cargo.toml` + all 5 dependent packages — see T3).
- Update all Rust source files that use the crate by its underscore-converted identifier
  (`torrust_tracker_located_error::`) to use `torrust_located_error::`.
- Update prose references in `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and the
  `located-error` package `README.md`.
- Verify the workspace builds and all tests pass.
- Publish `torrust-located-error` on crates.io.
- Handle the old crates.io name `torrust-tracker-located-error`: first add a deprecation
  notice / README update pointing to `torrust-located-error`; yank all versions only after
  any known downstream Torrust repositories are migrated (see Companion work).

### Out of Scope

- Moving the crate to a separate repository (a future extraction subissue).
- Changes to the crate's API or behaviour.

### Companion Work (other repositories)

After `torrust-located-error` is published, check all Torrust repositories (e.g.,
`torrust-index`) that may depend on the published `torrust-tracker-located-error`. Companion
PRs must be merged in those repos before yanking the old name. Yanking (T11) must happen
only after T10 is complete.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                       | Notes / Expected Output                                                      |
| --- | ------ | ---------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| T1  | TODO   | Rename `name` in `packages/located-error/Cargo.toml`                                                       | `name = "torrust-located-error"`                                             |
| T2  | TODO   | Update root `Cargo.toml` workspace dependency key                                                          | `torrust-located-error = { version = ..., path = "packages/located-error" }` |
| T3  | TODO   | Update all 5 dependent package `Cargo.toml` files (excluding root — see T2)                                | Replace `torrust-tracker-located-error` key with `torrust-located-error`     |
| T4  | TODO   | Update Rust source `use` / path references (`torrust_tracker_located_error::` → `torrust_located_error::`) | Affects package sources and integration tests                                |
| T5  | TODO   | Update prose in `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, `packages/located-error/README.md`  | Crate name and any inline code snippets                                      |
| T6  | TODO   | Run `cargo build --workspace` and `cargo test --workspace`                                                 | Clean build and all tests pass                                               |
| T7  | TODO   | Run `linter all`                                                                                           | Exit code `0`                                                                |
| T8  | TODO   | Publish `torrust-located-error` on crates.io                                                               | Successful `cargo publish -p torrust-located-error`                          |
| T9  | TODO   | Add deprecation notice to `torrust-tracker-located-error` on crates.io                                     | README / description points to `torrust-located-error`; do **not** yank yet  |
| T10 | TODO   | Check and migrate any downstream Torrust repositories using `torrust-tracker-located-error`                | Companion PRs in downstream repos merged; must be complete before T11        |
| T11 | TODO   | Yank all versions of `torrust-tracker-located-error` on crates.io                                          | All versions yanked; T10 must be complete first                              |
| T12 | TODO   | Update EPIC #1669 `Package Inventory` and `Desired Package State` tables                                   | Move `torrust-located-error` from `torrust-tracker-` to `torrust-` prefix    |

**Dependent packages to update in T3** (5 files; root `Cargo.toml` is handled in T2):

- `packages/configuration/Cargo.toml`
- `packages/axum-server/Cargo.toml`
- `packages/http-protocol/Cargo.toml`
- `packages/tracker-core/Cargo.toml`
- `packages/tracker-client/Cargo.toml`

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
- [ ] `torrust-located-error` published on crates.io; deprecation notice added to old name
- [ ] Downstream Torrust repositories migrated to `torrust-located-error` (T10 companion PRs merged)
- [ ] `torrust-tracker-located-error` yanked on crates.io (T11)
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669

## Acceptance Criteria

- [ ] `packages/located-error/Cargo.toml` declares `name = "torrust-located-error"`.
- [ ] No `Cargo.toml` file in the workspace references `torrust-tracker-located-error`.
- [ ] No Rust source file in the workspace uses `torrust_tracker_located_error::`.
- [ ] `cargo build --workspace` succeeds with zero errors.
- [ ] `cargo test --workspace` passes with zero failures.
- [ ] `linter all` exits with code `0`.
- [ ] `torrust-located-error` is published and visible on crates.io.
- [ ] `torrust-tracker-located-error` has a deprecation notice pointing to `torrust-located-error`.
- [ ] All known downstream Torrust repositories using `torrust-tracker-located-error` have been
      migrated to `torrust-located-error` (T10 complete).
- [ ] `torrust-tracker-located-error` is yanked on crates.io (only after T10 is complete).
- [ ] `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and `packages/located-error/README.md`
      reflect the new crate name.
- [ ] EPIC #1669 `Desired Package State` table lists `torrust-located-error` in the `torrust-`
      prefix section.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete` (no unused dependencies)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Command / Steps                                                                                                | Expected Result                            | Status | Evidence |
| --- | ------------------------------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------ | ------ | -------- |
| M1  | No stale references to old crate name | `grep -r "torrust-tracker-located-error\|torrust_tracker_located_error" . --include="*.toml" --include="*.rs"` | Zero matches                               | TODO   |          |
| M2  | New crate name visible on crates.io   | Visit `https://crates.io/crates/torrust-located-error`                                                         | Crate page exists and shows latest version | TODO   |          |
| M3  | Old crate name yanked                 | Visit `https://crates.io/crates/torrust-tracker-located-error`                                                 | All versions show "yanked"                 | TODO   |          |
| M4  | Downstream Torrust repositories clean | Check `torrust-index` and other Torrust repos for `torrust-tracker-located-error` dependency                   | No references found after T10              | TODO   |          |
