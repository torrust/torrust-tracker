---
doc-type: issue
issue-type: task
status: done
priority: p2
github-issue: 1884
spec-path: docs/issues/closed/1884-1669-19-move-bittorrent-peer-id-to-torrust-bittorrent.md
branch: 1884-1669-move-bittorrent-peer-id-to-torrust-bittorrent
related-pr: 1887
last-updated-utc: 2026-06-10 00:00
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
- **Change the license from AGPL-3.0 to Apache-2.0**: the tracker workspace inherits AGPL-3.0
  globally, but this was never an intentional choice for this standalone library crate. The
  upstream source (`aquatic_peer_id`) is Apache-2.0, the existing `LICENSE-APACHE` file already
  preserves that attribution, and all packages in `torrust/torrust-bittorrent` are uniformly
  Apache-2.0. The AGPL-3.0 `LICENSE` file from the tracker workspace is dropped; the package
  inherits `license = "Apache-2.0"` from the `torrust-bittorrent` workspace.
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

| ID  | Status | Task                                                                                                                   | Notes / Expected Output                                                                                 |
| --- | ------ | ---------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Rename `name` in `packages/peer-id/Cargo.toml` to `torrust-peer-id`                                                    | `name = "torrust-peer-id"`                                                                              |
| T2  | DONE   | Update `repository` URL in `packages/peer-id/Cargo.toml` and crate metadata                                            | Point to `https://github.com/torrust/torrust-bittorrent`                                                |
| T2b | DONE   | Drop AGPL-3.0 `LICENSE` from the package; inherit Apache-2.0 from the destination workspace                            | `LICENSE` file removed; `license.workspace = true`; `LICENSE-APACHE` attribution kept                   |
| T3  | DONE   | Confirm destination workspace `torrust/torrust-bittorrent` migration path                                              | Target path agreed: `packages/peer-id`                                                                  |
| T4  | DONE   | Move/merge crate source into destination workspace, preserving history where practical                                 | `packages/peer-id` added to `torrust/torrust-bittorrent`                                                |
| T5  | DONE   | Set up/adjust CI in destination repository if needed                                                                   | CI green after migration                                                                                |
| T6  | DONE   | Publish `torrust-peer-id` on crates.io from destination repository                                                     | Successful `cargo publish`; crate visible at crates.io/crates/torrust-peer-id                           |
| T7  | DONE   | Update `packages/http-protocol/Cargo.toml`: replace path dep with published `torrust-peer-id`                          | `torrust-peer-id = "0.1.0"` (no path)                                                                   |
| T8  | DONE   | Update `packages/primitives/Cargo.toml`: replace path dep with published `torrust-peer-id`                             | `torrust-peer-id = "0.1.0"` (no path)                                                                   |
| T9  | DONE   | Update `packages/udp-protocol/Cargo.toml`: replace path dep with published `torrust-peer-id` (keep `zerocopy` feature) | `torrust-peer-id = { version = "0.1.0", features = ["zerocopy"] }` (no path)                            |
| T10 | DONE   | Remove `packages/peer-id/` from tracker workspace (`members` + workspace dep in `Cargo.toml`)                          | `cargo build --workspace` succeeds without the local crate                                              |
| T11 | DONE   | Delete `packages/peer-id/` directory from the tracker repo                                                             | Directory gone; workspace still builds                                                                  |
| T12 | DONE   | Update `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and any README references                                | No stale references to `bittorrent-peer-id`                                                             |
| T13 | DONE   | Run `cargo build --workspace`, `cargo test --workspace`, `linter all`                                                  | All green                                                                                               |
| T14 | DONE   | Update EPIC #1669 `Package Inventory` and `Desired Package State` tables                                               | `bittorrent-` prefix section removed; Desired Package State table updated; Active Subissues marked DONE |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [x] Manual verification scenarios executed and recorded
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] `torrust-peer-id` published from `torrust/torrust-bittorrent`
- [x] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-06-05 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669
- 2026-06-05 00:00 UTC - josecelano - GitHub issue #1884 created; spec promoted to docs/issues/open/
- 2026-06-08 00:00 UTC - josecelano - T1-T4: Copied crate to torrust-bittorrent, renamed to torrust-peer-id, switched to Apache-2.0
- 2026-06-08 00:00 UTC - josecelano - T6: torrust-peer-id 0.1.0 published to crates.io
- 2026-06-08 00:00 UTC - josecelano - T7-T13: Replaced path deps with crates.io dep; removed packages/peer-id/; updated AGENTS.md; all quality gates pass

## Acceptance Criteria

- [x] `packages/peer-id/` directory no longer exists in the tracker workspace.
- [x] Root `Cargo.toml` does not list `packages/peer-id` as a workspace member.
- [x] No `Cargo.toml` in the tracker workspace references `bittorrent-peer-id`.
- [x] `packages/http-protocol/Cargo.toml` depends on the published `torrust-peer-id`.
- [x] `packages/primitives/Cargo.toml` depends on the published `torrust-peer-id`.
- [x] `packages/udp-protocol/Cargo.toml` depends on the published `torrust-peer-id` with the `zerocopy` feature.
- [x] `cargo build --workspace` succeeds without the local peer-id crate.
- [x] `cargo test --workspace` passes with zero failures.
- [x] `linter all` exits with code `0`.
- [x] `torrust-peer-id` is published and visible on crates.io.
- [x] `torrust-peer-id` is published under the Apache-2.0 license; no AGPL-3.0 `LICENSE` file is present in the package.
- [x] Destination repository (`torrust/torrust-bittorrent`) has passing CI and a published release.
- [x] `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md` no longer list `bittorrent-peer-id`.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                  | Command/Steps                                                                          | Expected Result                         | Status | Evidence                                                   |
| --- | ----------------------------------------- | -------------------------------------------------------------------------------------- | --------------------------------------- | ------ | ---------------------------------------------------------- |
| M1  | No stale workspace reference to old crate | `grep -r "bittorrent-peer-id\|packages/peer-id" . --include="*.toml" --include="*.rs"` | Zero matches in tracker repo            | DONE   | Only in docs/issues/ historical specs; zero in source/toml |
| M2  | New crate visible on crates.io            | Visit `https://crates.io/crates/torrust-peer-id`                                       | Crate page exists, latest version shown | DONE   | `torrust-peer-id 0.1.0` published and visible              |
| M3  | Destination repository CI green           | Check CI status on `torrust/torrust-bittorrent` default branch                         | All checks pass                         | DONE   | Published and deployed from torrust-bittorrent             |
