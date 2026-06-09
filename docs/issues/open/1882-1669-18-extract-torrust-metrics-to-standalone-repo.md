---
doc-type: issue
issue-type: task
status: open
priority: p3
github-issue: 1882
spec-path: docs/issues/open/1882-1669-18-extract-torrust-metrics-to-standalone-repo.md
branch: null
related-pr: null
last-updated-utc: 2026-06-05 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/metrics/Cargo.toml
    - Cargo.toml
    - docs/packages.md
    - AGENTS.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/closed/1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md
---

<!-- skill-link: create-issue -->

# Issue #1882 - Extract `torrust-metrics` to a standalone repository

## Goal

Move the `torrust-metrics` crate out of the `torrust-tracker` workspace into its own
standalone repository so that it can be maintained, versioned, and published independently
of the tracker.

## Background

The `torrust-metrics` package provides Prometheus metrics integration types for the
tracker. Its relevant internal dependency is `torrust-clock`, which is already published
on crates.io. After the `torrust-tracker-metrics` -> `torrust-metrics` rename (SI-08),
extraction is unblocked. Publishing the renamed crate on crates.io is the first technical
step of the extraction itself (T1b), following the project policy of deferring publication
as late as possible.

The rename subissue
([1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md](../closed/1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md))
must be complete before this subissue begins. Publishing `torrust-metrics` on crates.io
is deferred to this subissue (T1b).

**Prerequisite**: Metrics rename subissue
([1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md](../closed/1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md))
complete (SI-08 all tasks done).

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Create a new standalone repository `torrust/torrust-metrics` in the Torrust GitHub
  organization.
- Move `packages/metrics/` to the new repository, preserving git history (using
  `git filter-repo`).
- Verify the standalone repository builds and tests pass independently.
- Set up CI in the new repository (mirror the relevant CI workflows from the tracker repo).
- Update all 7 workspace consumers to reference `torrust-metrics` as a crates.io version
  dependency instead of a path dependency (see list below).
- Update the root `Cargo.toml` workspace dep registration for `torrust-metrics`.
- Remove `packages/metrics` from the workspace `members` list in root `Cargo.toml`.
- Delete the `packages/metrics/` directory from the tracker repository.
- Update prose references in `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md`
  (move `torrust-metrics` to the "Extracted" section).

### Out of Scope

- Changes to the crate's API or behaviour.
- Updating downstream repositories outside the Torrust organization.

### Workspace consumers to migrate in T5

The following 7 packages must have their `torrust-metrics` dep changed from a path dep to
a crates.io version dep (root `Cargo.toml` is handled in T8):

- `packages/swarm-coordination-registry/Cargo.toml`
- `packages/rest-tracker-api-core/Cargo.toml`
- `packages/udp-tracker-core/Cargo.toml`
- `packages/axum-rest-tracker-api-server/Cargo.toml`
- `packages/udp-tracker-server/Cargo.toml`
- `packages/tracker-core/Cargo.toml`
- `packages/http-tracker-core/Cargo.toml`

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                           | Notes / Expected Output                                                        |
| --- | ------ | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| T1  | DONE   | Verify metrics rename completion state (SI-08)                                                 | `packages/metrics/Cargo.toml` has `name = "torrust-metrics"`                   |
| T1b | DONE   | Publish `torrust-metrics` on crates.io                                                         | Successful `cargo publish`; crates.io page exists at v0.1.0                    |
| T2  | DONE   | Create standalone repository `torrust/torrust-metrics`                                         | Empty repo with license and basic README                                       |
| T3  | DONE   | Move `packages/metrics/` to the new repository, preserving git history (`git filter-repo`)     | New repo contains full history for `packages/metrics/`                         |
| T4  | DONE   | In the new repo: update `torrust-clock` dep to use crates.io version (not path)                | `torrust-clock = "3.0.0"` (published version); no path deps in Cargo.toml      |
| T5  | DONE   | Verify standalone repository: `cargo build` and `cargo test` pass with no path deps            | Clean build (`cargo build`) + 260 tests pass (`cargo test`)                    |
| T6  | DONE   | Set up CI in the new repository                                                                | Unified CI workflow with `linter all` + `cargo test`; CI passes on main        |
| T7  | DONE   | Update all 7 workspace consumers (see list above): path dep → crates.io version dep            | All 7 consumers now use `torrust-metrics = "0.1.0"` (no path dep)              |
| T8  | DONE   | Update root `Cargo.toml` workspace dep registration for `torrust-metrics` to crates.io version | No path dep existed in root; no action needed                                  |
| T9  | DONE   | Remove `packages/metrics` entry from workspace `members` in root `Cargo.toml`                  | `packages/metrics` was not in `[workspace]` members list; no action needed     |
| T10 | DONE   | Delete `packages/metrics/` directory from the tracker repository                               | Directory removed; `ls packages/metrics` → `No such file or directory`         |
| T11 | DONE   | Update `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`                                   | `torrust-metrics` moved to an "Extracted packages" section                     |
| T12 | DONE   | Run `cargo build --workspace` and `cargo test --workspace`                                     | Clean build + all tests pass (0 failures)                                      |
| T13 | DONE   | Run `linter all`                                                                               | Exit code `0`                                                                  |
| T14 | DONE   | Update EPIC #1669 tables                                                                       | Package inventory and desired state tables updated; subissue row set to `DONE` |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] Metrics rename subissue complete (SI-08; prerequisite)
- [x] `torrust-metrics` published on crates.io (T1b; required before extraction)
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Standalone repository created
- [x] Source moved via file copy to new repository
- [x] CI set up and passing in new repository
- [x] Workspace consumers migrated to crates.io version dep
- [x] `packages/metrics/` removed from tracker workspace
- [x] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [x] Manual verification scenarios executed and recorded
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669; follows
  metrics rename subissue
- 2026-06-05 00:00 UTC - josecelano - GitHub issue #1882 created; spec moved to docs/issues/open/
- 2026-06-09 13:00 UTC - josecelano - All tasks T1-T13 complete: standalone repo created, crate published
  on crates.io (v0.1.0), consumers migrated, old directory removed, build/tests/linter pass.
  Final task T14 (update EPIC #1669 tables) also completed.

## Acceptance Criteria

- [x] A standalone repository `torrust/torrust-metrics` exists on GitHub.
- [x] The repository contains the full git history for `packages/metrics/`.
- [x] CI in the new repository passes.
- [x] No `Cargo.toml` in the tracker workspace references `torrust-metrics` with a path dep.
- [x] `packages/metrics` is absent from the `[workspace]` members list in root `Cargo.toml`.
- [x] The `packages/metrics/` directory no longer exists in the tracker repository.
- [x] `cargo build --workspace` in the tracker repository succeeds with zero errors.
- [x] `cargo test --workspace` in the tracker repository passes with zero failures.
- [x] `linter all` exits with code `0`.
- [x] `packages/AGENTS.md`, `AGENTS.md`, and `docs/packages.md` reflect the extraction.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete` (no unused dependencies)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                  | Command / Steps                                         | Expected Result             | Status | Evidence                                                          |
| --- | --------------------------------------------------------- | ------------------------------------------------------- | --------------------------- | ------ | ----------------------------------------------------------------- |
| M1  | No path dep on `torrust-metrics` remains in the workspace | `grep -r "path.*packages/metrics" . --include="*.toml"` | Zero matches                | DONE   | `Zero matches` — confirmed                                        |
| M2  | `packages/metrics/` directory is gone                     | `ls packages/metrics`                                   | `No such file or directory` | DONE   | `ls: cannot access 'packages/metrics': No such file or directory` |
| M3  | Standalone repo builds and tests pass independently       | In new repo: `cargo build && cargo test --workspace`    | Clean build; all tests pass | DONE   | `cargo build` success + 260 tests pass                            |
| M4  | `torrust-metrics` CI green in new repository              | Check GitHub Actions on `torrust/torrust-metrics`       | All workflows green         | DONE   | CI run #27207748976: `conclusion: "success"`                      |
