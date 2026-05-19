---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1795
spec-path: docs/issues/open/1795-1669-04-move-announce-policy-to-torrust-tracker-primitives.md
branch: 1669-04-move-announce-policy-to-torrust-tracker-primitives
related-pr: null
last-updated-utc: 2026-05-18 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/lib.rs
    - packages/primitives/src/lib.rs
    - packages/primitives/Cargo.toml
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - https://github.com/torrust/torrust-tracker/issues/1795
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md
---

<!-- skill-link: create-issue -->

# Issue #1795 - Move `AnnouncePolicy` from `torrust-tracker-configuration` to `torrust-tracker-primitives`

## Goal

Move the `AnnouncePolicy` struct from `torrust-tracker-configuration` into
`torrust-tracker-primitives`, reversing an inverted dependency where a `primitives` package
depends on a `configuration` package. After the move, `torrust-tracker-configuration` depends
on `torrust-tracker-primitives` for `AnnouncePolicy`, which is the natural direction.

## Background

`AnnouncePolicy` (min/max announce intervals) is a domain concept — it describes the peer
communication policy for the BitTorrent announce cycle. Domain concepts belong in `primitives`,
not in `configuration`, which should be concerned only with config-file parsing and environment
variable wiring.

The coupling analysis (F-03) found that `torrust-tracker-primitives` imports
`torrust_tracker_configuration::AnnouncePolicy` — meaning a `primitives` package depends on a
`configuration` package. This is an inverted dependency: `primitives` should sit at the bottom
of the dependency graph, with `configuration` depending on it, not the reverse.

Moving `AnnouncePolicy` to `primitives` fixes the inversion:

- Before: `primitives` → `configuration` (for `AnnouncePolicy`)
- After: `configuration` → `primitives` (for `AnnouncePolicy`, among other types)

Both packages (`torrust-tracker-primitives` and `torrust-tracker-configuration`) are published
to crates.io. Removing `AnnouncePolicy` from `torrust-tracker-configuration` is a semver
breaking change for that crate; it will require a major version bump when published. Within
this workspace, at version `3.0.0-develop`, the change is expected and planned.

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Move the `AnnouncePolicy` struct (and any directly associated types or impl blocks) from
  `packages/configuration/src/` to `packages/primitives/src/`.
- Add `torrust-tracker-configuration` as a dependency of `torrust-tracker-primitives`
  is removed; `torrust-tracker-primitives` must not depend on `torrust-tracker-configuration`.
- Update `packages/configuration` to import `AnnouncePolicy` from `torrust-tracker-primitives`.
- Update all other workspace files that import `AnnouncePolicy` from
  `torrust_tracker_configuration` to import it from `torrust_tracker_primitives`.
- Verify the workspace builds and all tests pass.

### Out of Scope

- Any rename of `AnnouncePolicy` or changes to its fields.
- Publishing a new crates.io version; the semver bump is handled in the release cycle.
- Extracting `torrust-tracker-primitives` to a standalone repository (a later subissue).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                                             | Notes / Expected Output                                                                         |
| --- | ------ | -------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| T1  | DONE   | Locate all definition and usage sites of `AnnouncePolicy` across the workspace                                                   | `grep -r "AnnouncePolicy" . --include="*.rs"` — build a full consumer list                      |
| T2  | DONE   | Move `AnnouncePolicy` definition to `packages/primitives/src/` (e.g. `primitives/src/announce_policy.rs`)                        | Public module exported from `packages/primitives/src/lib.rs`                                    |
| T3  | DONE   | Remove `AnnouncePolicy` from `packages/configuration/src/`                                                                       | Definition gone; re-export or direct dep on `torrust-tracker-primitives` added to configuration |
| T4  | DONE   | Add `torrust-tracker-primitives` as a dep of `packages/configuration/Cargo.toml` if not already present                          | `torrust-tracker-primitives` in `[dependencies]`                                                |
| T5  | DONE   | Remove `torrust-tracker-configuration` dep from `packages/primitives/Cargo.toml` if `AnnouncePolicy` was its sole reason         | `cargo machete` reports no unused dep                                                           |
| T6  | DONE   | Update all workspace files that import `AnnouncePolicy` from `torrust_tracker_configuration` to use `torrust_tracker_primitives` | One-line change per file                                                                        |
| T7  | DONE   | Run `cargo build --workspace` and `cargo test --workspace`                                                                       | Clean build; all tests pass                                                                     |
| T8  | DONE   | Run `linter all`                                                                                                                 | Exit code `0`                                                                                   |

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
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-18 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669, addressing F-03
  from the coupling analysis report.
- 2026-05-19 UTC - josecelano - Implementation completed: moved `AnnouncePolicy` to
  `primitives/src/announce.rs`, removed inverted dep, added deprecated re-export in
  `configuration`, updated all workspace consumers. All checks pass.

## Acceptance Criteria

- [x] `packages/primitives/src/` defines `AnnouncePolicy` and exports it publicly.
- [x] `packages/primitives/Cargo.toml` does not list `torrust-tracker-configuration` as a dependency.
- [x] `packages/configuration/src/` no longer defines `AnnouncePolicy`; it imports from `torrust-tracker-primitives`.
- [x] No workspace file imports `AnnouncePolicy` from `torrust_tracker_configuration`
      (all migrated to `torrust_tracker_primitives` or re-exported through it).
- [x] `cargo build --workspace` succeeds with zero errors.
- [x] `cargo test --workspace` passes with zero failures.
- [x] `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                     | Command / Steps                                                              | Expected Result         | Status | Evidence                                                           |
| --- | ------------------------------------------------------------ | ---------------------------------------------------------------------------- | ----------------------- | ------ | ------------------------------------------------------------------ |
| M1  | No workspace import of `AnnouncePolicy` from `configuration` | `grep -r "torrust_tracker_configuration::AnnouncePolicy" . --include="*.rs"` | Zero matches            | DONE   | `grep` returned zero matches                                       |
| M2  | `primitives` exports `AnnouncePolicy`                        | `grep "AnnouncePolicy" packages/primitives/src/lib.rs`                       | `pub` declaration found | DONE   | `pub use announce::{AnnounceData, AnnounceEvent, AnnouncePolicy};` |
| M3  | `primitives` dep list does not include `configuration`       | `grep "torrust-tracker-configuration" packages/primitives/Cargo.toml`        | Zero matches            | DONE   | `grep` returned zero matches                                       |
