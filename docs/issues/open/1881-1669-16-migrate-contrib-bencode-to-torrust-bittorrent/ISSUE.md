---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1881
spec-path: docs/issues/open/1881-1669-16-migrate-contrib-bencode-to-torrust-bittorrent/ISSUE.md
branch: 1881-1669-16-migrate-contrib-bencode-to-torrust-bittorrent
related-pr: null
last-updated-utc: 2026-06-05 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - contrib/bencode/Cargo.toml
    - Cargo.toml
    - packages/http-protocol/Cargo.toml
    - AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #1881 - Migrate `contrib/bencode` to `torrust/torrust-bittorrent` as `torrust-bencode`

## Goal

Rename the crate `torrust-tracker-contrib-bencode` to `torrust-bencode`, and migrate it from
the tracker workspace (`contrib/bencode`) back into `torrust/torrust-bittorrent`
(`packages/bencode`) replacing the legacy copy there.

## Background

The `contrib/bencode` package is a pure bencode encode/decode library with no
tracker-specific logic. Several facts confirm it is ready for independent life:

- **No tracker dependencies**: its only runtime dependency is `thiserror`.
- **No crates.io publication blockers**: all runtime dependencies are external crates already
  on crates.io. The extraction can proceed without publishing any other workspace package
  first. _(Publication blocker analysis reviewed May 2026.)_
- **Separate license**: Apache-2.0, unlike the tracker's AGPL-3.0-only. Having it in the
  same workspace creates a mixed-license surface that confuses downstream users.
- **Already published on crates.io** as `torrust-tracker-contrib-bencode` (verified May 2026).
- **Destination is now explicit in EPIC #1669**: `torrust/torrust-bittorrent` is the target
  workspace for this migration, and the tracker copy is treated as the newer lineage that
  replaces legacy `packages/bencode` there.
- **Only one internal consumer**: `packages/http-protocol` depends on it. After extraction
  that dependency becomes a normal crates.io dependency — no other workspace packages change.
- **`contrib/` is the wrong home**: the `contrib/` prefix signals community-contributed
  code living temporarily in the workspace; this crate has been here long enough to graduate.

The rename drops the `torrust-tracker-contrib-` prefix:

- `torrust-tracker-` scopes it to the tracker — wrong.
- `-contrib-` marks it as transient community code — no longer accurate.
- `torrust-bencode` is the shortest accurate name: Torrust-namespace, bencode purpose.

This issue is a subissue of EPIC #1669 (Overhaul: Packages).

## Scope

### In Scope

- Rename the crate `name` in `contrib/bencode/Cargo.toml` to `torrust-bencode`.
- Use `torrust/torrust-bittorrent` as the destination workspace.
- Move the crate source to `packages/bencode` in `torrust/torrust-bittorrent` as a clean copy
  (no cross-repo history transplant; commit history in the tracker repo is sufficient context).
- Ensure CI passes in the destination repository after migration.
- Publish `torrust-bencode` from the destination repository.
- Update `packages/http-protocol/Cargo.toml` to depend on the published `torrust-bencode`
  crate (remove the local path dependency).
- Remove `contrib/bencode/` from the tracker workspace:
  - Remove from `members` in the root `Cargo.toml`.
  - Remove the workspace dependency entry for `torrust-tracker-contrib-bencode`.
- Update `packages/AGENTS.md`, `AGENTS.md` Package Catalog, and `docs/packages.md`.
- Handle the old crates.io name `torrust-tracker-contrib-bencode`: yank all versions and
  publish a notice pointing to `torrust-bencode`.

### Out of Scope

- Changes to the crate's API or behaviour.
- Updating other downstream repositories (e.g., `torrust-index`) — separate task per repo.
- Extracting other `bittorrent-*` or `contrib/` crates — each gets its own subissue.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                                           | Notes / Expected Output                                                                   |
| --- | ------ | ------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| T1  | DONE   | Rename `name` in `contrib/bencode/Cargo.toml` to `torrust-bencode`                                                             | `name = "torrust-bencode"`                                                                |
| T2  | DONE   | Update `repository` URL in `contrib/bencode/Cargo.toml` and destination crate metadata                                         | Point to `https://github.com/torrust/torrust-bittorrent`                                  |
| T3  | DONE   | Confirm destination workspace `torrust/torrust-bittorrent` migration path                                                      | Target path agreed: `packages/bencode`                                                    |
| T4  | DONE   | Copy crate source into destination workspace as a clean copy (no cross-repo history transplant)                                | `packages/bencode` replaced by tracker lineage                                            |
| T5  | DONE   | Set up/adjust CI in destination repository if needed                                                                           | CI green after migration                                                                  |
| T6  | DONE   | Publish `torrust-bencode` on crates.io from destination repository (same version as current `torrust-tracker-contrib-bencode`) | Successful `cargo publish`; crate visible at crates.io/crates/torrust-bencode             |
| T7  | DONE   | Update `packages/http-protocol/Cargo.toml`: replace path dep with published `torrust-bencode`                                  | `torrust-bencode = "X.Y.Z"` (no path)                                                     |
| T8  | DONE   | Remove `contrib/bencode/` from tracker workspace (`members` + workspace dep in `Cargo.toml`)                                   | `cargo build --workspace` succeeds without the local crate                                |
| T9  | DONE   | Delete `contrib/bencode/` directory from the tracker repo                                                                      | Directory gone; workspace still builds                                                    |
| T10 | DONE   | Update `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and any README references                                        | No stale references to `torrust-tracker-contrib-bencode`                                  |
| T11 | DONE   | Run `cargo build --workspace`, `cargo test --workspace`, `linter all`                                                          | All green                                                                                 |
| T12 | TODO   | Handle old crates.io name `torrust-tracker-contrib-bencode`                                                                    | Yank and/or deprecate old name with redirect to `torrust-bencode`                         |
| T13 | DONE   | Update EPIC #1669 `Package Inventory` and `Desired Package State` tables                                                       | Remove `torrust-tracker-contrib-bencode` from `torrust-tracker-` table; mark as extracted |

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
- [x] `torrust-bencode` published from `torrust/torrust-bittorrent`; old name yanked
- [x] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669
- 2026-06-05 00:00 UTC - josecelano - Spec reviewed; GitHub issue #1881 created; moved to open/
- 2026-06-05 00:00 UTC - josecelano - Implementation complete: torrust-bencode 3.0.0 published; contrib/bencode removed from tracker workspace; T12 (yank old crate) pending

## Acceptance Criteria

- [ ] `contrib/bencode/` directory no longer exists in the tracker workspace.
- [ ] Root `Cargo.toml` does not list `contrib/bencode` as a workspace member.
- [ ] No `Cargo.toml` in the tracker workspace references `torrust-tracker-contrib-bencode`.
- [ ] `packages/http-protocol/Cargo.toml` depends on the published `torrust-bencode`.
- [ ] `cargo build --workspace` succeeds without the local bencode crate.
- [ ] `cargo test --workspace` passes with zero failures.
- [ ] `linter all` exits with code `0`.
- [ ] `torrust-bencode` is published and visible on crates.io.
- [ ] `torrust-tracker-contrib-bencode` is yanked or carries a deprecation notice.
- [ ] Destination repository (`torrust/torrust-bittorrent`) has passing CI and a published release.
- [ ] `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md` no longer list `torrust-tracker-contrib-bencode`.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                  | Command/Steps                                                                                      | Expected Result                                  | Status | Evidence |
| --- | ----------------------------------------- | -------------------------------------------------------------------------------------------------- | ------------------------------------------------ | ------ | -------- |
| M1  | No stale workspace reference to old crate | `grep -r "torrust-tracker-contrib-bencode\|contrib/bencode" . --include="*.toml" --include="*.rs"` | Zero matches in tracker repo                     | TODO   |          |
| M2  | New crate visible on crates.io            | Visit `https://crates.io/crates/torrust-bencode`                                                   | Crate page exists, latest version shown          | TODO   |          |
| M3  | Old crate yanked                          | Visit `https://crates.io/crates/torrust-tracker-contrib-bencode`                                   | All versions show "yanked" or deprecation notice | TODO   |          |
| M4  | Destination repository CI green           | Check CI status on `torrust/torrust-bittorrent` default branch                                     | All checks pass                                  | TODO   |          |
