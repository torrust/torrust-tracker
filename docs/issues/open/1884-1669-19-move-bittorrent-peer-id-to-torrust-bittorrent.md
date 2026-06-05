---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1884
spec-path: docs/issues/open/1884-1669-19-move-bittorrent-peer-id-to-torrust-bittorrent.md
branch: 1884-1669-move-bittorrent-peer-id-to-torrust-bittorrent
related-pr: null
last-updated-utc: 2026-06-05 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/peer-id/Cargo.toml
    - Cargo.toml
    - packages/http-protocol/Cargo.toml
    - packages/primitives/Cargo.toml
    - packages/udp-protocol/Cargo.toml
    - AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #1884 - Move `packages/peer-id` to `torrust/torrust-bittorrent` as `torrust-peer-id`

## Goal

Rename the crate `bittorrent-peer-id` to `torrust-peer-id`, and move it from the tracker
workspace (`packages/peer-id`) into `torrust/torrust-bittorrent` (`packages/peer-id`).

## Background

The `packages/peer-id` package is a pure BitTorrent peer-ID parsing and client-identification
library with no tracker-specific logic. Several facts confirm it is ready for extraction:

- **No workspace dependencies**: its only dependencies are external crates (`compact_str`,
  `hex`, `quickcheck`, `regex`, `serde`, `zerocopy`). The extraction can proceed without
  publishing any other workspace package first. _(Verified June 2026.)_
- **No crates.io publication blockers**: the crate has never been published, so there is no
  migration window or old-name yank required.
- **`torrust/torrust-bittorrent` is the agreed destination**: the EPIC #1669 "Desired Package
  State" table already lists `torrust-peer-id` as an incoming package in that workspace.
  This is the first package in the `bittorrent-*` extraction sequence.
- **Three workspace consumers**: `packages/http-protocol`, `packages/primitives`, and
  `packages/udp-protocol` all depend on `bittorrent-peer-id` via a local path dep. After
  extraction each dependency becomes a normal crates.io dependency — no other workspace
  packages change.
- **Naming alignment**: the `bittorrent-` prefix is a working name carried over from an
  earlier refactoring cycle. Renaming to `torrust-peer-id` aligns with the `torrust-`
  organisation prefix adopted for all packages landing in `torrust/torrust-bittorrent`.

This issue is a subissue of EPIC #1669 (Overhaul: Packages).

## Scope

### In Scope

- Rename the crate `name` in `packages/peer-id/Cargo.toml` from `bittorrent-peer-id` to
  `torrust-peer-id`.
- Move the crate source to `packages/peer-id` in `torrust/torrust-bittorrent`, preserving
  relevant history.
- Update `repository` URL and crate metadata in `Cargo.toml` to point to
  `https://github.com/torrust/torrust-bittorrent`.
- Ensure CI passes in the destination repository after migration.
- Publish `torrust-peer-id` on crates.io from the destination repository.
- Update the three consumers in the tracker workspace to depend on the published
  `torrust-peer-id` crate (remove all local path dependencies):
  - `packages/http-protocol/Cargo.toml`
  - `packages/primitives/Cargo.toml`
  - `packages/udp-protocol/Cargo.toml`
- Remove `packages/peer-id/` from the tracker workspace:
  - Remove from `members` in the root `Cargo.toml`.
  - Remove the workspace dependency entry for `bittorrent-peer-id`.
- Delete `packages/peer-id/` directory from the tracker repo.
- Update `packages/AGENTS.md`, `AGENTS.md` Package Catalog, and `docs/packages.md`.

### Out of Scope

- Changes to the crate's API or behaviour.
- Updating other downstream repositories — separate task per repo.
- Extracting other `bittorrent-*` or `contrib/` crates — each gets its own subissue.
- Setting up CI from scratch in `torrust/torrust-bittorrent` if it is already in place.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                                   | Notes / Expected Output                                                               |
| --- | ------ | ---------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| T1  | TODO   | Rename `name` in `packages/peer-id/Cargo.toml` to `torrust-peer-id`                                                    | `name = "torrust-peer-id"`                                                            |
| T2  | TODO   | Update `repository` URL in `packages/peer-id/Cargo.toml` and crate metadata                                            | Point to `https://github.com/torrust/torrust-bittorrent`                              |
| T3  | TODO   | Confirm destination workspace `torrust/torrust-bittorrent` migration path                                              | Target path agreed: `packages/peer-id`                                                |
| T4  | TODO   | Move/merge crate source into destination workspace, preserving history where practical                                 | `packages/peer-id` added to `torrust/torrust-bittorrent`                              |
| T5  | TODO   | Set up/adjust CI in destination repository if needed                                                                   | CI green after migration                                                              |
| T6  | TODO   | Publish `torrust-peer-id` on crates.io from destination repository                                                     | Successful `cargo publish`; crate visible at crates.io/crates/torrust-peer-id         |
| T7  | TODO   | Update `packages/http-protocol/Cargo.toml`: replace path dep with published `torrust-peer-id`                          | `torrust-peer-id = "X.Y.Z"` (no path)                                                 |
| T8  | TODO   | Update `packages/primitives/Cargo.toml`: replace path dep with published `torrust-peer-id`                             | `torrust-peer-id = "X.Y.Z"` (no path)                                                 |
| T9  | TODO   | Update `packages/udp-protocol/Cargo.toml`: replace path dep with published `torrust-peer-id` (keep `zerocopy` feature) | `torrust-peer-id = { version = "X.Y.Z", features = ["zerocopy"] }` (no path)          |
| T10 | TODO   | Remove `packages/peer-id/` from tracker workspace (`members` + workspace dep in `Cargo.toml`)                          | `cargo build --workspace` succeeds without the local crate                            |
| T11 | TODO   | Delete `packages/peer-id/` directory from the tracker repo                                                             | Directory gone; workspace still builds                                                |
| T12 | TODO   | Update `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and any README references                                | No stale references to `bittorrent-peer-id`                                           |
| T13 | TODO   | Run `cargo build --workspace`, `cargo test --workspace`, `linter all`                                                  | All green                                                                             |
| T14 | TODO   | Update EPIC #1669 `Package Inventory` and `Desired Package State` tables                                               | Remove `bittorrent-peer-id` from tracker table; mark as extracted in bittorrent table |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] `torrust-peer-id` published from `torrust/torrust-bittorrent`
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-06-05 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669
- 2026-06-05 00:00 UTC - josecelano - GitHub issue #1884 created; spec promoted to docs/issues/open/

## Acceptance Criteria

- [ ] `packages/peer-id/` directory no longer exists in the tracker workspace.
- [ ] Root `Cargo.toml` does not list `packages/peer-id` as a workspace member.
- [ ] No `Cargo.toml` in the tracker workspace references `bittorrent-peer-id`.
- [ ] `packages/http-protocol/Cargo.toml` depends on the published `torrust-peer-id`.
- [ ] `packages/primitives/Cargo.toml` depends on the published `torrust-peer-id`.
- [ ] `packages/udp-protocol/Cargo.toml` depends on the published `torrust-peer-id` with the `zerocopy` feature.
- [ ] `cargo build --workspace` succeeds without the local peer-id crate.
- [ ] `cargo test --workspace` passes with zero failures.
- [ ] `linter all` exits with code `0`.
- [ ] `torrust-peer-id` is published and visible on crates.io.
- [ ] Destination repository (`torrust/torrust-bittorrent`) has passing CI and a published release.
- [ ] `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md` no longer list `bittorrent-peer-id`.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                  | Command/Steps                                                                          | Expected Result                         | Status | Evidence |
| --- | ----------------------------------------- | -------------------------------------------------------------------------------------- | --------------------------------------- | ------ | -------- |
| M1  | No stale workspace reference to old crate | `grep -r "bittorrent-peer-id\|packages/peer-id" . --include="*.toml" --include="*.rs"` | Zero matches in tracker repo            | TODO   |          |
| M2  | New crate visible on crates.io            | Visit `https://crates.io/crates/torrust-peer-id`                                       | Crate page exists, latest version shown | TODO   |          |
| M3  | Destination repository CI green           | Check CI status on `torrust/torrust-bittorrent` default branch                         | All checks pass                         | TODO   |          |
