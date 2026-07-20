---
doc-type: issue
issue-type: task
status: closed
priority: p2
github-issue: 1823
spec-path: docs/issues/closed/1823-1669-10-rename-torrust-tracker-located-error-to-torrust-located-error.md
branch: 1823-rename-torrust-tracker-located-error-to-torrust-located-error
related-pr: 1824
last-updated-utc: 2026-05-22 08:09
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


# Issue #1823 - Rename `torrust-tracker-located-error` to `torrust-located-error`

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

## Pre-Implementation Review: Keep vs. Delete

Before starting the rename, we reconsidered whether the package itself should exist or be
removed. The conclusion below should be reviewed and confirmed in the PR before T1–T13 are
executed.

### Recommendation

**Keep the package and proceed with the rename to `torrust-located-error`.**

### What the package actually provides

The crate is ~110 lines in a single file (`packages/located-error/src/lib.rs`) with one
runtime dependency (`tracing`). It exports:

- `Located<E>` — newtype wrapper used as the conversion entry point.
- `LocatedError<'a, E>` — the decorated error: `Arc<E>` source + `Box<Location<'a>>`.
- `DynError` — `Arc<dyn Error + Send + Sync>` type alias.
- A `#[track_caller]` `Into` impl that captures `Location::caller()` and emits
  `tracing::debug!` on construction.

Non-trivial value vs. `std` / `thiserror` alone:

1. `#[track_caller]` capture into a stored `Location` (std has no first-class equivalent).
2. `Arc`-shared source making the error cheaply `Clone` even for `!Clone` inner errors.
3. Automatic `tracing::debug!` log on construction (single attachment point for tracing).
4. Works for both concrete `E: Error` and `dyn Error + Send + Sync`.

### Current workspace usage

Active in **5 packages**, ~20 call sites:

| Package          | Files                                                                                                      | Usage                          |
| ---------------- | ---------------------------------------------------------------------------------------------------------- | ------------------------------ |
| `configuration`  | `src/lib.rs`                                                                                               | 3 error variants (dyn)         |
| `axum-server`    | `src/tsl.rs`                                                                                               | TLS error variant (dyn)        |
| `http-protocol`  | `src/v1/requests/announce.rs`, `src/v1/requests/scrape.rs`                                                 | info_hash / peer-id conversion |
| `tracker-core`   | `src/error.rs`, `src/authentication/key/mod.rs`, `src/authentication/handler.rs`, `src/databases/error.rs` | many error variants            |
| `tracker-client` | `src/udp/mod.rs`                                                                                           | uses `DynError` alias          |

The package is also referenced from
[`.github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md`](../../../.github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md)
as the recommended pattern for diagnostics-rich errors.

### Why keep it

- **Real, non-trivial functionality.** The `#[track_caller]` + `Arc`-clone + auto-trace
  combo is not a one-liner. Replacing it everywhere would either duplicate the pattern
  across 5 packages or drop diagnostic features.
- **Stable surface, near-zero maintenance cost.** Single file, one dep, hasn't changed
  materially in a long time.
- **Crates.io alternatives are worse fits.** `error-stack` / `eyre` / `anyhow` are heavier
  and don't compose cleanly with the `thiserror`-enum policy. The error-handling skill
  explicitly disallows `anyhow` in libraries.
- **Removal cost is high, benefit is low.** Deleting would touch ~20 call sites across
  core domain packages just to swap to a less expressive pattern.
- **The rename premise still holds.** Nothing in the implementation is tracker-specific.
  `torrust-located-error` correctly reflects scope and is reusable by `torrust-index`.

### Why delete it (the alternative case)

For completeness, reasons one might prefer deletion:

- **Niche pattern.** Locating an error to a `Location` is most useful when the wrapped
  error type is `!Display`/opaque (e.g. `Box<dyn Error>`). Where call sites use concrete
  `thiserror` enums with `#[from]`, the `?` operator already propagates source-chain
  information and the `Location` adds limited extra signal.
- **Tracing overlap.** `tracing` spans / `instrument` can carry caller metadata; some of
  the value of `Located` is already available from structured logging at error sites.
- **Few real beneficiaries.** Of the ~20 call sites, several store `LocatedError<dyn ...>`
  variants that are rarely matched on; a plain `Box<dyn Error + Send + Sync>` source
  field plus a `tracing::error!` at construction may be sufficient.
- **One less crate to publish/maintain** on crates.io if the value is mostly cosmetic.

These points are weaker than the "keep" reasons above given the current usage, but they
are why this question is worth confirming with a reviewer before committing to a rename

- publish + downstream migration.

### Decision needed before implementation

If the reviewer agrees with **Keep**, T1–T13 proceed as planned.

If the reviewer prefers **Delete**, this subissue is closed and replaced by a new
subissue with scope: remove `packages/located-error`, migrate ~20 call sites to a
simpler pattern (likely `Box<dyn Error + Send + Sync>` + explicit `tracing::error!` at
construction sites), yank `torrust-tracker-located-error` from crates.io with a final
deprecation note.

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

| ID  | Status | Task                                                                                                       | Notes / Expected Output                                                     |
| --- | ------ | ---------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| T1  | DONE   | Rename `name` in `packages/located-error/Cargo.toml`                                                       | `name = "torrust-located-error"`                                            |
| T2  | N/A    | Update root `Cargo.toml` workspace dependency key                                                          | No workspace-level dep existed; all 5 packages reference the crate directly |
| T3  | DONE   | Update all 5 dependent package `Cargo.toml` files (excluding root — see T2)                                | Replace `torrust-tracker-located-error` key with `torrust-located-error`    |
| T4  | DONE   | Update Rust source `use` / path references (`torrust_tracker_located_error::` → `torrust_located_error::`) | Affects package sources and integration tests                               |
| T5  | DONE   | Update prose in `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, `packages/located-error/README.md`  | Crate name and any inline code snippets                                     |
| T6  | DONE   | Run `cargo build --workspace` and `cargo test --workspace`                                                 | Clean build and all tests pass                                              |
| T7  | DONE   | Run `linter all`                                                                                           | Exit code `0`                                                               |
| T8  | TODO   | Publish `torrust-located-error` on crates.io                                                               | Successful `cargo publish -p torrust-located-error`                         |
| T9  | TODO   | Add deprecation notice to `torrust-tracker-located-error` on crates.io                                     | README / description points to `torrust-located-error`; do **not** yank yet |
| T10 | TODO   | Check and migrate any downstream Torrust repositories using `torrust-tracker-located-error`                | Companion PRs in downstream repos merged; must be complete before T11       |
| T11 | TODO   | Yank all versions of `torrust-tracker-located-error` on crates.io                                          | All versions yanked; T10 must be complete first                             |
| T12 | TODO   | Update EPIC #1669 `Package Inventory` and `Desired Package State` tables                                   | Move `torrust-located-error` from `torrust-tracker-` to `torrust-` prefix   |

**Dependent packages to update in T3** (5 files; root `Cargo.toml` is handled in T2):

- `packages/configuration/Cargo.toml`
- `packages/axum-server/Cargo.toml`
- `packages/http-protocol/Cargo.toml`
- `packages/tracker-core/Cargo.toml`
- `packages/tracker-client/Cargo.toml`

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
- [ ] `torrust-located-error` published on crates.io; deprecation notice added to old name
- [ ] Downstream Torrust repositories migrated to `torrust-located-error` (T10 companion PRs merged)
- [ ] `torrust-tracker-located-error` yanked on crates.io (T11)
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669
- 2026-05-21 17:00 UTC - josecelano - GitHub issue #1823 created and linked as sub-issue of #1669; spec moved to `docs/issues/open/`
- 2026-05-21 17:15 UTC - josecelano - Added pre-implementation "Keep vs. Delete" analysis; awaiting reviewer decision before T1 starts
- 2026-05-22 08:09 UTC - josecelano - Rename implemented: T1 (Cargo.toml name), T3 (5 dependent Cargo.toml dep keys), T4 (10 Rust source use statements), T5 (README, AGENTS.md, deployment.yaml, release_process.md, 2 skills); T2 is N/A (no workspace-level dep existed). T6 (`cargo build --workspace`, `cargo test --workspace`) and T7 (`linter all`) all pass. Draft PR #1824 open.

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

| ID  | Scenario                              | Command / Steps                                                                                                | Expected Result                            | Status | Evidence                          |
| --- | ------------------------------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------ | ------ | --------------------------------- |
| M1  | No stale references to old crate name | `grep -r "torrust-tracker-located-error\|torrust_tracker_located_error" . --include="*.toml" --include="*.rs"` | Zero matches                               | DONE   | Zero matches confirmed 2026-05-22 |
| M2  | New crate name visible on crates.io   | Visit `https://crates.io/crates/torrust-located-error`                                                         | Crate page exists and shows latest version | TODO   |                                   |
| M3  | Old crate name yanked                 | Visit `https://crates.io/crates/torrust-tracker-located-error`                                                 | All versions show "yanked"                 | TODO   |                                   |
| M4  | Downstream Torrust repositories clean | Check `torrust-index` and other Torrust repos for `torrust-tracker-located-error` dependency                   | No references found after T10              | TODO   |                                   |
