---
doc-type: issue
issue-type: task
status: open
priority: p1
github-issue: 1889
spec-path: docs/issues/open/1889-1669-21-migrate-from-bittorrent-primitives-to-torrust-info-hash.md
branch: "1889-migrate-from-bittorrent-primitives-to-torrust-info-hash"
related-pr: null
last-updated-utc: 2026-06-09 11:00
semantic-links:
  skill-links:
    - create-issue
    - add-rust-dependency
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/maintenance/add-rust-dependency/SKILL.md
    - AGENTS.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #1889 - Migrate from `bittorrent-primitives` to `torrust-info-hash`

## Goal

Replace the `bittorrent-primitives` crate dependency with the new `torrust-info-hash` crate (v0.1.0) across the entire workspace. The `InfoHash` type originally came from `bittorrent-primitives` and has now been published as a standalone crate `torrust-info-hash` from the `torrust/torrust-bittorrent` monorepo (see torrust/torrust-bittorrent#87 / #88).

## Background

The `bittorrent-primitives` crate (v0.2.0) is a single-package repository whose sole public type is `InfoHash`. As part of the broader workspace overhaul (EPIC #1669), the `InfoHash` type has been migrated to the `torrust/torrust-bittorrent` workspace as `torrust-info-hash` v0.1.0 and published to crates.io.

This workspace (torrust/torrust-tracker) currently depends on `bittorrent-primitives` in **14 Cargo.toml files** (13 packages + the root crate for dev-dependencies) — exclusively for the `InfoHash` type. Replacing it with `torrust-info-hash` reduces the dependency footprint and moves toward deprecating/archiving the `torrust/bittorrent-primitives` repository.

Note: the `udp-protocol` package (`torrust-tracker-udp-tracker-protocol`) defines its **own** local `InfoHash` struct (a newtype over `[u8; 20]`) and does NOT use `bittorrent-primitives`. It is not in scope for this migration.

## Scope

### In Scope

- Replace `bittorrent-primitives = "0.2.0"` with `torrust-info-hash = "=0.1.0"` in all workspace `Cargo.toml` files that use it for `InfoHash`
- Update all Rust source files: `use bittorrent_primitives::info_hash::InfoHash` → `use torrust_info_hash::InfoHash`
- Update doc comments that reference the old import path (`bittorrent_primitives::info_hash::InfoHash`)
- Remove `bittorrent-primitives` from root `Cargo.toml` dev-dependencies if no longer needed
- Run `cargo machete` to verify no unused dependencies remain
- Run `linter all` and full test suite to validate
- Update `AGENTS.md` if the package table requires changes
- Update `project-words.txt` with any new technical terms

### Out of Scope

- The `udp-protocol` package's local `InfoHash` struct — it is unrelated to `bittorrent-primitives` (see background)
- Migrating any other types — `torrust-info-hash` only contains `InfoHash`
- Publishing new crates to crates.io
- Archiving the `torrust/bittorrent-primitives` repository

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                        | Notes / Expected Output                                    |
| --- | ------ | --------------------------------------------------------------------------- | ---------------------------------------------------------- |
| T1  | DONE   | Add `torrust-info-hash` to root workspace `Cargo.toml` dependencies section | Add `torrust-info-hash` version pin for workspace-wide use |
| T2  | DONE   | Replace dependency + imports in `packages/http-tracker-core`                | Cargo.toml + all `.rs` imports and doc comments            |
| T3  | DONE   | Replace dependency + imports in `packages/http-protocol`                    | Cargo.toml + all `.rs` imports and doc comments            |
| T4  | DONE   | Replace dependency + imports in `packages/primitives`                       | Cargo.toml + all `.rs` imports and doc comments            |
| T5  | DONE   | Replace dependency + imports in `packages/tracker-core`                     | Cargo.toml + all `.rs` imports and doc comments            |
| T6  | DONE   | Replace dependency + imports in `packages/tracker-client`                   | Cargo.toml + all `.rs` imports and doc comments            |
| T7  | DONE   | Replace dependency + imports in `packages/udp-tracker-core`                 | Cargo.toml + all `.rs` imports and doc comments            |
| T8  | DONE   | Replace dependency + imports in `packages/udp-server`                       | Cargo.toml + all `.rs` imports and doc comments            |
| T9  | DONE   | Replace dependency + imports in `packages/axum-rest-api-server`             | Cargo.toml + all `.rs` imports and doc comments            |
| T10 | DONE   | Replace dependency + imports in `packages/axum-http-server`                 | Cargo.toml + all `.rs` imports and doc comments            |
| T11 | DONE   | Replace dependency + imports in `packages/swarm-coordination-registry`      | Cargo.toml + all `.rs` imports and doc comments            |
| T12 | DONE   | Replace dependency + imports in `packages/torrent-repository-benchmarking`  | Cargo.toml + all `.rs` imports and doc comments            |
| T13 | DONE   | Replace dependency + imports in `packages/persistence-benchmark`            | Cargo.toml + all `.rs` imports and doc comments            |
| T14 | DONE   | Replace dependency + imports in `console/tracker-client`                    | Cargo.toml + all `.rs` imports and doc comments            |
| T15 | DONE   | Replace root dev-dependency + update `tests/` imports                       | Root `Cargo.toml` + `tests/servers/` files                 |
| T16 | DONE   | Remove `bittorrent-primitives` from all `Cargo.toml` files                  | After confirming no remaining references                   |
| T17 | DONE   | Run `cargo check --workspace`                                               | Verify compilation                                         |
| T18 | DONE   | Run `cargo machete`                                                         | Verify no unused dependencies                              |
| T19 | DONE   | Run `linter all`                                                            | Verify linting passes                                      |
| T20 | DONE   | Run `cargo test --workspace`                                                | Verify tests pass (2520/2520)                              |
| T21 | N/A    | Update `project-words.txt`                                                  | No new terms needed                                     |
| T22 | N/A    | Update `AGENTS.md` if needed                                                | No references to either crate found                     |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-09 00:00 UTC - User - Initial specification draft. Issue #1889 created. Linked as SI-21 under EPIC #1669.
- 2026-06-09 11:00 UTC - Agent - Implementation completed: all 14 Cargo.toml files migrated, all `.rs` imports updated, `linter all` passes, 2520/2520 tests pass

## Acceptance Criteria

- [x] AC1: All `Cargo.toml` files use `torrust-info-hash = "=0.2.0"` instead of `bittorrent-primitives = "0.2.0"` for InfoHash
- [x] AC2: All Rust source imports use `use torrust_info_hash::InfoHash` instead of `use bittorrent_primitives::info_hash::InfoHash`
- [x] AC3: No remaining references to `bittorrent-primitives` or `bittorrent_primitives` except the comment in `udp-protocol/src/common.rs` (which is out of scope)
- [x] AC4: `cargo check --workspace` exits with code `0`
- [x] AC5: `cargo machete` exits with code `0`
- [x] AC6: `linter all` exits with code `0`
- [x] AC7: `cargo test --workspace` passes (2520/2520)
- [ ] AC8: `project-words.txt` is up to date (N/A)
- [ ] AC9: Documentation is updated when behavior/workflow changes
- [ ] AC10: Manual verification scenarios are executed and documented (status + evidence)
- [ ] AC11: Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- `cargo check --workspace`
- `cargo machete`
- `cargo test --workspace`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                            | Command/Steps                                                                                   | Expected Result                          | Status | Evidence                     |
| --- | --------------------------------------------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------------- | ------ | ---------------------------- |
| M1  | Verify no `bittorrent-primitives` references remain | `grep -r "bittorrent-primitives" --include="*.toml" --include="*.rs"`                           | No matches (except udp-protocol comment) | DONE   | `grep` shows only udp-protocol comment |
| M2  | Verify all imports use new crate                    | `sed -i` bulk replacement across all files                                                      | All files updated                        | DONE   | `cargo check --workspace` passes |
| M3  | Full workspace build                                | `cargo check --workspace`                                                                       | Exit code 0                              | DONE   | `Finished dev profile`       |
| M4  | Full workspace test                                 | `cargo nextest run --workspace`                                                                 | Exit code 0                              | DONE   | 2520/2520 passed             |
