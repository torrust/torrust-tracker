---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: 1669
github-issue: 1910
spec-path: docs/issues/open/1910-1669-si-29-rename-udp-and-http-core-protocol-crates-to-remove-redundant-tracker.md
branch: 1910-rename-udp-and-http-core-protocol-crates
related-pr: 1923
last-updated-utc: 2026-06-19
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/http-core/Cargo.toml
    - packages/http-protocol/Cargo.toml
    - packages/udp-core/Cargo.toml
    - packages/udp-protocol/Cargo.toml
    - Cargo.toml
    - AGENTS.md
    - packages/AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
---

<!-- skill-link: create-issue -->

# Issue #1910 (SI-29) - Remove redundant `-tracker-` from HTTP and UDP crate names

## Goal

Remove the redundant `tracker` segment from four workspace crate names, so that each
crate name becomes: `torrust-tracker-{protocol}-{layer}` instead of the current
`torrust-tracker-{protocol}-tracker-{layer}`. Rename the affected folders to match
per the folder naming convention (DEC-15).

## Background

Four workspace packages have a redundant `-tracker-` segment in their crate name:

| Current crate name              | Current folder  | Proposed crate name             | Proposed folder             |
| ------------------------------- | --------------- | ------------------------------- | --------------------------- |
| `torrust-tracker-http-core`     | `http-core`     | `torrust-tracker-http-core`     | `http-core`                 |
| `torrust-tracker-http-protocol` | `http-protocol` | `torrust-tracker-http-protocol` | `http-protocol` (unchanged) |
| `torrust-tracker-udp-core`      | `udp-core`      | `torrust-tracker-udp-core`      | `udp-core`                  |
| `torrust-tracker-udp-protocol`  | `udp-protocol`  | `torrust-tracker-udp-protocol`  | `udp-protocol` (unchanged)  |

The word `tracker` appears twice in each current name: once in the prefix
(`torrust-tracker-`) and again in the middle (`-tracker-`). Since the prefix already
scopes these to the tracker workspace, the middle `-tracker-` adds no information.

The renaming also aligns the crate names with their folders per DEC-15 (folder name =
crate name without the `torrust-tracker-` prefix). Two folders must be renamed:
`http-core` → `http-core` and `udp-core` → `udp-core`. The protocol
folders already match (`http-protocol`, `udp-protocol`).

None of these packages are published on crates.io, so this is a **Rule U** rename
(unpublished crate rename) — only workspace consumers are affected, no external
migration window needed.

This issue is a subissue of EPIC [#1669](../1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Rename all 4 crate `name` fields in their `Cargo.toml` files.
- Rename the two folder paths (`http-core/` → `http-core/`, `udp-core/` → `udp-core/`).
- Update all workspace `Cargo.toml` dependency references (root `Cargo.toml` + consumer packages).
- Update all Rust `use` imports referencing the snake_case versions of the crate names.
- Update all documentation files that reference the crate names or folder paths.
- Update READMEs with the new crate name and docs.rs URL.

### Out of Scope

- Any changes to the packages' API, behaviour, or public types.
- Renaming any other packages (other renames are separate subissues).
- Publishing the renamed crates on crates.io (not currently planned).

### Prerequisites

None. These are unpublished crates (Rule U), and no other subissue depends on
the current name.

## Files to Update

This section lists every file type and location that must be updated.

### Package Cargo.toml files (crate names + dependency keys)

| File                                           | Current reference                         | Change                                    |
| ---------------------------------------------- | ----------------------------------------- | ----------------------------------------- |
| `packages/http-core/Cargo.toml` (after rename) | `name = "torrust-tracker-http-core"`      | `name = "torrust-tracker-http-core"`      |
| Same file                                      | `torrust-tracker-http-protocol = { ... }` | `torrust-tracker-http-protocol = { ... }` |
| `packages/http-protocol/Cargo.toml`            | `name = "torrust-tracker-http-protocol"`  | `name = "torrust-tracker-http-protocol"`  |
| `packages/udp-core/Cargo.toml` (after rename)  | `name = "torrust-tracker-udp-core"`       | `name = "torrust-tracker-udp-core"`       |
| Same file                                      | `torrust-tracker-udp-protocol = { ... }`  | `torrust-tracker-udp-protocol = { ... }`  |
| `packages/udp-protocol/Cargo.toml`             | `name = "torrust-tracker-udp-protocol"`   | `name = "torrust-tracker-udp-protocol"`   |

### Root workspace Cargo.toml

| Line                | Current reference                                                 | Change                                                            |
| ------------------- | ----------------------------------------------------------------- | ----------------------------------------------------------------- |
| Workspace `members` | `"packages/http-core"`                                            | `"packages/http-core"`                                            |
| Workspace `members` | `"packages/udp-core"`                                             | `"packages/udp-core"`                                             |
| Workspace dep       | `torrust-tracker-http-core = { ... path = "packages/http-core" }` | `torrust-tracker-http-core = { ... path = "packages/http-core" }` |
| Workspace dep       | `torrust-tracker-udp-core = { ... path = "packages/udp-core" }`   | `torrust-tracker-udp-core = { ... path = "packages/udp-core" }`   |

### Consumer Cargo.toml files (dependency keys)

| File                                         | Current dep key                                                                            | Change to                                                                          |
| -------------------------------------------- | ------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------- |
| `packages/axum-http-server/Cargo.toml`       | `torrust-tracker-http-core`                                                                | `torrust-tracker-http-core`                                                        |
| Same file                                    | `torrust-tracker-http-protocol`                                                            | `torrust-tracker-http-protocol`                                                    |
| Same file                                    | `torrust_tracker_udp_tracker_protocol = { package = "torrust-tracker-udp-protocol", ... }` | `torrust_tracker_udp_protocol = { package = "torrust-tracker-udp-protocol", ... }` |
| `packages/axum-rest-api-server/Cargo.toml`   | `torrust-tracker-http-core`                                                                | `torrust-tracker-http-core`                                                        |
| Same file                                    | `torrust-tracker-udp-core`                                                                 | `torrust-tracker-udp-core`                                                         |
| `packages/rest-api-core/Cargo.toml`          | `torrust-tracker-http-core`                                                                | `torrust-tracker-http-core`                                                        |
| Same file                                    | `torrust-tracker-udp-core`                                                                 | `torrust-tracker-udp-core`                                                         |
| `packages/udp-server/Cargo.toml`             | `torrust_tracker_udp_tracker_protocol = { package = "torrust-tracker-udp-protocol", ... }` | `torrust_tracker_udp_protocol = { package = "torrust-tracker-udp-protocol", ... }` |
| Same file                                    | `torrust-tracker-udp-core`                                                                 | `torrust-tracker-udp-core`                                                         |
| `packages/tracker-client/Cargo.toml`         | `torrust-tracker-udp-protocol`                                                             | `torrust-tracker-udp-protocol`                                                     |
| `packages/http-core/src/...` (all .rs files) | Internal `crate::` references                                                              | Internal — no change needed (crate rename doesn't affect internal `crate::` paths) |
| `console/tracker-client/Cargo.toml`          | `torrust-tracker-udp-protocol`                                                             | `torrust-tracker-udp-protocol`                                                     |

### Rust source files — `use` imports

**Warning**: these use the snake_case version of the crate name as a Rust `extern crate`
identifier. When the crate is renamed, all `use` statements importing from it must be
updated.

#### `torrust_tracker_http_tracker_core` → `torrust_tracker_http_core`

Files in `packages/http-core/benches/helpers/`:

- `sync.rs` — `use torrust_tracker_http_tracker_core::services::announce::AnnounceService;`
- `util.rs` — multiple imports

Files consuming `http-core` from other packages:

- `packages/rest-api-core/src/` — various imports
- `packages/axum-rest-api-server/src/` — various imports
- `packages/axum-http-server/src/` — various imports

#### `torrust_tracker_http_tracker_protocol` → `torrust_tracker_http_protocol`

Files in `packages/http-core/src/`:

- `src/services/announce.rs` — multiple imports
- `src/services/error_mapping.rs` — import
- `benches/helpers/util.rs` — multiple imports

Files in `packages/axum-http-server/src/` — various imports

#### `torrust_tracker_udp_tracker_core` → `torrust_tracker_udp_core`

Files in `packages/udp-server/src/` — various imports
Files in `packages/rest-api-core/src/` — various imports
Files in `packages/axum-rest-api-server/src/` — various imports

#### `torrust_tracker_udp_tracker_protocol` → `torrust_tracker_udp_protocol`

Files in `packages/udp-server/src/` — various imports
Files in `packages/axum-http-server/src/` — various imports
Files in `packages/udp-core/src/` — various imports
Files in `packages/tracker-client/src/` — various imports
Files in `console/tracker-client/src/` — various imports

### Package READMEs

| File                                          | Change                                                                |
| --------------------------------------------- | --------------------------------------------------------------------- |
| `packages/http-core/README.md` (after rename) | Update docs.rs URL to `https://docs.rs/torrust-tracker-http-core`     |
| `packages/http-protocol/README.md`            | Update docs.rs URL to `https://docs.rs/torrust-tracker-http-protocol` |
| `packages/udp-core/README.md` (after rename)  | Update docs.rs URL to `https://docs.rs/torrust-tracker-udp-core`      |
| `packages/udp-protocol/README.md`             | Update docs.rs URL to `https://docs.rs/torrust-tracker-udp-protocol`  |

### Documentation files

| File                                                                              | Notes                                                                 |
| --------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `AGENTS.md`                                                                       | Package Catalog table — 4 crate names to update                       |
| `packages/AGENTS.md`                                                              | Architecture diagram + Package Catalog — crate names and folder names |
| `src/AGENTS.md`                                                                   | Package Catalog table — folder references                             |
| `docs/packages.md`                                                                | File listing, architecture diagram, Package Catalog                   |
| `docs/issues/open/1669-overhaul-packages/EPIC.md`                                 | Many tables, dependency lists, and sections                           |
| `docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-06-10.md` | Many section headers referencing crate names                          |
| `docs/issues/open/1669-overhaul-packages/readme-audit.md`                         | Audit table rows                                                      |
| `docs/issues/drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md`    | References to `torrust-tracker-udp-protocol`                          |

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                     | Notes / Expected Output                                |
| --- | ------ | ---------------------------------------------------------------------------------------- | ------------------------------------------------------ | --- |
| T1  | DONE   | Rename `packages/http-tracker-core/` folder to `packages/http-core/`                     | `git mv packages/http-tracker-core packages/http-core` |
| T2  | DONE   | Rename `packages/udp-tracker-core/` folder to `packages/udp-core/`                       | `git mv packages/udp-tracker-core packages/udp-core`   |
| T3  | DONE   | Update crate `name` fields in all 4 Cargo.toml files                                     | http-core, http-protocol, udp-core, udp-protocol       |
| T4  | DONE   | Update all dependency references in root + consumer Cargo.toml files                     | See "Consumer Cargo.toml files" table above            |
| T5  | DONE   | Update all Rust `use` imports across the workspace                                       | See "Rust source files" section above                  |
| T6  | DONE   | Update folder references in root `Cargo.toml` workspace `members`                        | `packages/http-core`, `packages/udp-core`              |
| T7  | DONE   | Update package READMEs (docs.rs URLs, crate names)                                       | See "Package READMEs" table above                      |
| T8  | DONE   | Update `AGENTS.md`, `packages/AGENTS.md`, `src/AGENTS.md`                                | Crate names + folder names                             |
| T9  | DONE   | Update `docs/packages.md`                                                                | File listing + Package Catalog                         |
| T10 | DONE   | Update `docs/issues/open/1669-overhaul-packages/EPIC.md`                                 | Package inventory, desired state, dependency lists     |
| T11 | DONE   | Update `docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-06-10.md` | Section headers + crate name references                |
| T12 | DONE   | Run `cargo build --workspace`                                                            | All compilation succeeds                               |
| T13 | DONE   | Run `cargo test --workspace`                                                             | All tests pass                                         |
| T14 | DONE   | Run `cargo machete`                                                                      | No unused dependencies                                 |
| T15 | DONE   | Run `linter all`                                                                         | Exit code `0`                                          |
| T16 | DONE   | Update EPIC #1669 tables to mark this subissue DONE                                      |                                                        |     |

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
- [x] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-06-11 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669
- 2026-06-19 00:00 UTC - josecelano - Implementation completed: folders renamed, Cargo.toml files updated, Rust imports updated, READMEs updated, AGENTS.md files updated, docs updated, `cargo build --workspace` succeeds, all tests pass, `linter all` passes

## Acceptance Criteria

- [x] `packages/http-tracker-core/` renamed to `packages/http-core/`.
- [x] `packages/udp-tracker-core/` renamed to `packages/udp-core/`.
- [x] All 4 crate `name` fields use the new names.
- [x] No `Cargo.toml` in the workspace references the old crate names or old folder paths.
- [x] No Rust `use` import references the old snake_case crate names.
- [x] All package READMEs use the new docs.rs URLs.
- [x] `AGENTS.md`, `packages/AGENTS.md`, `src/AGENTS.md` use the new names.
- [x] `docs/packages.md` uses the new folder and crate names.
- [x] EPIC #1669 spec uses the new crate names throughout.
- [x] `cargo build --workspace` succeeds with zero errors.
- [x] `cargo test --workspace` passes with zero failures.
- [x] `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete` (no unused dependencies)

### Manual Verification Scenarios

| ID  | Scenario                                | Command / Steps                                                            | Expected Result                               | Status |
| --- | --------------------------------------- | -------------------------------------------------------------------------- | --------------------------------------------- | ------ |
| M1  | No stale crate name in Cargo.toml files | `grep -r "http-tracker-core\|udp-tracker-core" --include="*.toml"`         | Zero matches (except `http-core`, `udp-core`) | DONE   |
| M2  | No stale crate name in Rust imports     | `grep -r "http_tracker_core\|udp_tracker_core" --include="*.rs" packages/` | Zero matches (except `http_core`, `udp_core`) | DONE   |
| M3  | Old folders removed                     | `ls -d packages/http-tracker-core packages/udp-tracker-core 2>&1`          | `No such file or directory`                   | DONE   |
| M4  | New folders exist                       | `ls -d packages/http-core packages/udp-core`                               | Directories exist                             | DONE   |
