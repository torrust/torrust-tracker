---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1669-extract-torrust-tracker-contrib-bencode-to-torrust-bencode.md
branch: null
related-pr: null
last-updated-utc: 2026-05-15 12:00
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

# Issue #[To be assigned] - Extract and rename `torrust-tracker-contrib-bencode` to `torrust-bencode`

## Goal

Rename the crate `torrust-tracker-contrib-bencode` to `torrust-bencode`, extract it from
the tracker workspace into its own standalone repository, and publish it independently on
crates.io so that any Rust project can consume it without a dependency on the tracker.

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
- **`repository` already points elsewhere**: `contrib/bencode/Cargo.toml` declares
  `repository = "https://github.com/torrust/bittorrent-infrastructure-project"`,
  signalling the intent to move it out of the tracker repo.
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
- Create (or confirm and use) the target standalone repository. Two options — resolve before
  starting implementation (see Open Questions).
- Move the crate source to the new repository, preserving git history.
- Set up CI in the new repository (build, test, lint, publish workflow).
- Publish `torrust-bencode` on crates.io from the new repository.
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

## Open Questions

### Target repository

Two options:

| Option | Repository                                                                | Rationale                                                                       |
| ------ | ------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| A      | `https://github.com/torrust/torrust-bencode` (new)                        | Matches the crate name; cleanest standalone story                               |
| B      | `https://github.com/torrust/bittorrent-infrastructure-project` (existing) | Already referenced in `Cargo.toml`; may host multiple bittorrent utility crates |

**Context**: The crate was copied into this repo by @da2ce7 (Cameron Garnham) in commit
`a4ac6829` ("dev: copy bencode into local contrib folder"). The `Cargo.toml` `repository`
field already points to `bittorrent-infrastructure-project`, suggesting that was the intended
canonical home. However, it is unclear whether:

- the copy in this repo has diverged from whatever lives in `bittorrent-infrastructure-project`,
- `bittorrent-infrastructure-project` already has CI and workspace structure suitable for
  multi-crate hosting, or
- the original intent was a new standalone `torrust-bencode` repo.

**Questions for @da2ce7 before starting T3**:

1. Why was the crate copied into the tracker workspace rather than depended on as an external
   crate at the time?
2. Is there a canonical or newer version of this code in `bittorrent-infrastructure-project`,
   or is this tracker copy the authoritative source?
3. What is your preferred extraction target — new `torrust/torrust-bencode` repo (Option A)
   or `torrust/bittorrent-infrastructure-project` (Option B)?

**Recommendation**: Pending @da2ce7's input. Decide before starting T3.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                          | Notes / Expected Output                                                                   |
| --- | ------ | --------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| T1  | TODO   | Rename `name` in `contrib/bencode/Cargo.toml` to `torrust-bencode`                            | `name = "torrust-bencode"`                                                                |
| T2  | TODO   | Update `repository` URL in `contrib/bencode/Cargo.toml` to match chosen target repo           | Depends on Open Question resolution                                                       |
| T3  | TODO   | Create or confirm the target standalone repository                                            | Repo exists, README and LICENSE committed                                                 |
| T4  | TODO   | Move crate source to the new repository, preserving git history                               | `git filter-repo` or subtree split; history preserved under `contrib/bencode/`            |
| T5  | TODO   | Set up CI in the new repository (build, test, lint, publish workflow)                         | CI green on first push                                                                    |
| T6  | TODO   | Publish `torrust-bencode` on crates.io from the new repository                                | Successful `cargo publish`; crate visible at crates.io/crates/torrust-bencode             |
| T7  | TODO   | Update `packages/http-protocol/Cargo.toml`: replace path dep with published `torrust-bencode` | `torrust-bencode = "X.Y.Z"` (no path)                                                     |
| T8  | TODO   | Remove `contrib/bencode/` from tracker workspace (`members` + workspace dep in `Cargo.toml`)  | `cargo build --workspace` succeeds without the local crate                                |
| T9  | TODO   | Delete `contrib/bencode/` directory from the tracker repo                                     | Directory gone; workspace still builds                                                    |
| T10 | TODO   | Update `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and any README references       | No stale references to `torrust-tracker-contrib-bencode`                                  |
| T11 | TODO   | Run `cargo build --workspace`, `cargo test --workspace`, `linter all`                         | All green                                                                                 |
| T12 | TODO   | Yank old crates.io name `torrust-tracker-contrib-bencode` and add deprecation notice          | All versions yanked; description points to `torrust-bencode`                              |
| T13 | TODO   | Update EPIC #1669 `Package Inventory` and `Desired Package State` tables                      | Remove `torrust-tracker-contrib-bencode` from `torrust-tracker-` table; mark as extracted |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Open Question (target repository) resolved
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] `torrust-bencode` published from the new repository; old name yanked
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669

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
- [ ] The new repository has passing CI and a published release.
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
| M4  | New repository CI green                   | Check CI status on the new repository's default branch                                             | All checks pass                                  | TODO   |          |
