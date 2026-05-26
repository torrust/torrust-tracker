---
doc-type: issue
issue-type: task
status: closed
priority: p2
github-issue: 1821
spec-path: docs/issues/closed/1821-1669-09-rename-torrust-tracker-clock-to-torrust-clock.md
branch: 1821-rename-torrust-tracker-clock-to-torrust-clock
related-pr: 1822
last-updated-utc: 2026-05-21 16:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/clock/Cargo.toml
    - Cargo.toml
    - AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #1821 - Rename `torrust-tracker-clock` to `torrust-clock`

## Goal

Rename the Cargo crate `torrust-tracker-clock` to `torrust-clock` to reflect that it is a
generic, tracker-independent utility that can be used in any Rust project (e.g.,
`torrust-index`).

## Background

The `clock` package (folder `packages/clock`) provides a mockable time abstraction for
deterministic testing. It contains no tracker-specific logic and its usefulness extends
beyond this repository — for example, `torrust-index` already contains copied clock code
(<https://github.com/torrust/torrust-index/blob/843aafff6b459a9ade4097273fbc430b7ecb959e/src/utils/clock.rs>).

The `torrust-tracker-` prefix implies a tracker-only scope that does not reflect the
crate's actual purpose. The rename:

- Makes the crate identity match its scope.
- Signals to downstream users that it is reusable outside the tracker.
- Prepares it for potential extraction to a standalone repository in a future cycle
  (see [1669-extract-torrust-clock-to-standalone-repo.md](1669-extract-torrust-clock-to-standalone-repo.md)).

The current crate name `torrust-tracker-clock` is **published on crates.io** (as of
May 2026). Publishing the new name `torrust-clock` and handling the old published name
(yank or deprecation notice) are **deferred to SI-17** (extract `torrust-clock` to
standalone repository). This issue covers only the in-workspace rename.

**This issue has a prerequisite**: the `DEFAULT_TIMEOUT` constant must be moved from
`torrust-tracker-configuration` to `torrust-tracker-clock` before this rename is started,
so that the constant travels with the `clock` package. See
[1669-03-move-default-timeout-from-configuration-to-clock.md](1669-03-move-default-timeout-from-configuration-to-clock.md).

**Residual tracker-namespaced dep**: After the rename, `torrust-clock` will still depend on
`torrust-tracker-primitives` for `DurationSinceUnixEpoch`. That type is a plain
`pub type DurationSinceUnixEpoch = Duration` — a trivial alias for `std::time::Duration`
with no tracker-specific logic. A generic `torrust-clock` crate depending on a
`torrust-tracker-*` package is semantically inconsistent.

**Decision — Option A**: Move `DurationSinceUnixEpoch` from `torrust-tracker-primitives`
into `torrust-clock`. The primitives dep does **not block publishing `torrust-clock`** (the
crate is already published), so this move can happen as a dedicated follow-up after the
rename is complete. A separate draft subissue covers the migration of the 80+ workspace
consumers currently importing the type from `torrust-tracker-primitives`:
see [1669-02-move-duration-since-unix-epoch-to-torrust-clock.md](1669-02-move-duration-since-unix-epoch-to-torrust-clock.md).

This issue is a subissue of EPIC #1669 (Overhaul: Packages).

## Scope

### In Scope

- Rename the crate `name` field in `packages/clock/Cargo.toml`.
- Update all `Cargo.toml` files in the workspace that reference `torrust-tracker-clock`
  as a dependency (root `Cargo.toml` + all dependent packages).
- Update all Rust source files that use the crate by its underscore-converted identifier
  (`torrust_tracker_clock::`) to use `torrust_clock::`.
- Update prose references in `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and the `clock` package
  `README.md`.
- Verify the workspace builds and all tests pass.

### Out of Scope

- Publishing `torrust-clock` on crates.io — deferred to SI-17.
- Deprecating or yanking `torrust-tracker-clock` on crates.io — deferred to SI-17.
- Updating `torrust-index` to use `torrust-clock` — deferred to SI-17; an issue will be
  opened on `torrust/torrust-index` once the crate is published under the new name.
- Moving the crate to a separate repository — see
  [1669-extract-torrust-clock-to-standalone-repo.md](../drafts/1669-extract-torrust-clock-to-standalone-repo.md).
- Changes to the crate's API or behaviour.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status   | Task                                                                                              | Notes / Expected Output                                                              |
| --- | -------- | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| T1  | DONE     | Rename `name` in `packages/clock/Cargo.toml`                                                      | `name = "torrust-clock"`                                                             |
| T2  | DONE     | Update root `Cargo.toml` workspace dependency key                                                 | `torrust-clock = { version = ..., path = "packages/clock" }`                         |
| T3  | DONE     | Update all dependent package `Cargo.toml` files (10 packages, excluding root — see T2)            | Replace `torrust-tracker-clock` key with `torrust-clock` in each                     |
| T4  | DONE     | Update Rust source `use` / path references (`torrust_tracker_clock::` → `torrust_clock::`)        | Affects `src/`, package sources, and integration tests                               |
| T5  | DONE     | Update prose in `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, `packages/clock/README.md` | Crate name and any inline code snippets                                              |
| T6  | DONE     | Run `cargo build --workspace` and `cargo test --workspace`                                        | Clean build and all tests pass                                                       |
| T7  | DONE     | Run `linter all`                                                                                  | Exit code `0`                                                                        |
| T8  | DEFERRED | Publish `torrust-clock` on crates.io                                                              | Deferred to SI-17                                                                    |
| T9  | DEFERRED | Add deprecation notice to `torrust-tracker-clock` on crates.io                                    | Deferred to SI-17                                                                    |
| T10 | DEFERRED | Update `torrust-index`: replace copied clock code with `torrust-clock` dep                        | Deferred to SI-17; open issue on `torrust/torrust-index` after crate is published    |
| T11 | DEFERRED | Yank all versions of `torrust-tracker-clock` on crates.io                                         | Deferred to SI-17                                                                    |
| T12 | DONE     | Update EPIC #1669 `Package Inventory` and `Desired Package State` tables                          | Move `torrust-clock` from `torrust-tracker-` to `torrust-`; drop `Renamed from` note |

**Dependent packages to update in T3** (10 files; root `Cargo.toml` is handled in T2):

- `packages/axum-health-check-api-server/Cargo.toml`
- `packages/axum-http-tracker-server/Cargo.toml` (appears in both `[dependencies]` and `[dev-dependencies]`)
- `packages/axum-rest-tracker-api-server/Cargo.toml`
- `packages/http-protocol/Cargo.toml`
- `packages/http-tracker-core/Cargo.toml`
- `packages/swarm-coordination-registry/Cargo.toml`
- `packages/tracker-core/Cargo.toml`
- `packages/torrent-repository-benchmarking/Cargo.toml`
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
- [ ] `torrust-clock` published on crates.io; deprecation notice added to old name (deferred to SI-17)
- [ ] `torrust-index` migrated to `torrust-clock` (companion PR merged) (deferred to SI-17)
- [ ] `torrust-tracker-clock` yanked on crates.io (deferred to SI-17)
- [x] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669
- 2026-05-21 12:00 UTC - josecelano - GitHub issue #1821 created; spec moved to `docs/issues/open/`; branch `1821-rename-torrust-tracker-clock-to-torrust-clock` created; crates.io tasks deferred to SI-17
- 2026-05-21 15:50 UTC - josecelano - Implementation complete: T1–T7 + T12 done; `cargo build --workspace`, `cargo test --workspace`, `linter all` all pass; EPIC updated

## Acceptance Criteria

- [ ] `packages/clock/Cargo.toml` declares `name = "torrust-clock"`.
- [ ] No `Cargo.toml` file in the workspace references `torrust-tracker-clock`.
- [ ] No Rust source file in the workspace uses `torrust_tracker_clock::`.
- [ ] `cargo build --workspace` succeeds with zero errors.
- [ ] `cargo test --workspace` passes with zero failures.
- [ ] `linter all` exits with code `0`.
- [ ] `torrust-clock` is published and visible on crates.io (deferred to SI-17).
- [ ] `torrust-tracker-clock` has a deprecation notice pointing to `torrust-clock` (deferred to SI-17).
- [ ] `torrust-index` no longer contains a local copy of clock code; it depends on `torrust-clock` (deferred to SI-17).
- [ ] `torrust-tracker-clock` is yanked on crates.io (only after `torrust-index` migration is merged) (deferred to SI-17).
- [ ] `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and `packages/clock/README.md` reflect the new crate name.
- [ ] EPIC #1669 `Desired Package State` table lists `torrust-clock` in the `torrust-` section.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete` (no unused dependencies)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Command/Steps                                                                                  | Expected Result                            | Status | Evidence |
| --- | ------------------------------------- | ---------------------------------------------------------------------------------------------- | ------------------------------------------ | ------ | -------- |
| M1  | No stale references to old crate name | `grep -r "torrust-tracker-clock\|torrust_tracker_clock" . --include="*.toml" --include="*.rs"` | Zero matches                               | TODO   |          |
| M2  | New crate name visible on crates.io   | Visit `https://crates.io/crates/torrust-clock`                                                 | Crate page exists and shows latest version | TODO   |          |
| M3  | Old crate name yanked                 | Visit `https://crates.io/crates/torrust-tracker-clock`                                         | All versions show "yanked"                 | TODO   |          |
| M4  | `torrust-index` migration merged      | Check `torrust/torrust-index` for `torrust-clock` dep; no local clock copy                     | PR merged; no copied clock code present    | TODO   |          |
