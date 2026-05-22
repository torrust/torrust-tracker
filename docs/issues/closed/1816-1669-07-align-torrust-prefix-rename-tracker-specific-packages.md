---
doc-type: issue
issue-type: task
status: closed
priority: p2
github-issue: 1816
spec-path: docs/issues/closed/1816-1669-07-align-torrust-prefix-rename-tracker-specific-packages.md
branch: 1816-1669-07-align-torrust-prefix-rename-tracker-specific-packages
related-pr: null
last-updated-utc: 2026-05-20 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Cargo.toml
    - AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #1816 - Align `torrust-` prefix: rename tracker-specific packages to `torrust-tracker-`

## Goal

Rename the seven crate names that currently carry the bare `torrust-` prefix but contain
tracker-specific logic or depend on tracker-specific crates, so that the `torrust-tracker-`
prefix accurately marks their scope. Where the old name already contains the word "tracker"
in the middle (redundant once it is in the prefix), remove it to produce cleaner names.

## Background

The workspace currently has three crate-name prefixes:

| Prefix             | Intended scope                                       |
| ------------------ | ---------------------------------------------------- |
| `bittorrent-`      | Generic BitTorrent protocol / community reusable     |
| `torrust-`         | Reusable across Torrust projects (tracker, index, …) |
| `torrust-tracker-` | Torrust Tracker only                                 |

Seven crates carry the `torrust-` prefix but belong in the `torrust-tracker-` group:

| Current crate name                             | Why it is tracker-specific                                                                            |
| ---------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `torrust-tracker-axum-health-check-api-server` | Depends on `torrust-tracker-configuration` and `torrust-tracker-primitives`                           |
| `torrust-tracker-axum-http-server`             | Implements the BitTorrent HTTP tracker; depends on all tracker-core packages                          |
| `torrust-tracker-axum-rest-api-server`         | Implements the tracker management REST API; deep tracker dependencies                                 |
| `torrust-tracker-axum-server`                  | Axum wrapper configured via `torrust-tracker-configuration`; not generic                              |
| `torrust-tracker-rest-api-client`              | HTTP client for this tracker's REST API; no torrust deps but implements tracker-specific API contract |
| `torrust-tracker-rest-api-core`                | Core logic for tracker REST API; depends on all three tracker-core packages                           |
| `torrust-tracker-udp-server`                   | Implements the BitTorrent UDP tracker; deep tracker dependencies                                      |

**None of these crates are published on crates.io** (verified May 2026). The rename has no
external consumers to migrate and does not require any crates.io handling.

This issue is a subissue of EPIC #1669 (Overhaul: Packages).

### Proposed name mapping

Where the old name contained a redundant middle `tracker` segment (already covered by the
new prefix), that segment is removed to produce a shorter, cleaner name.

| Current name                                   | Proposed new name                              | Rust identifier change                                                                          |
| ---------------------------------------------- | ---------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `torrust-tracker-axum-health-check-api-server` | `torrust-tracker-axum-health-check-api-server` | `torrust_tracker_axum_health_check_api_server` → `torrust_tracker_axum_health_check_api_server` |
| `torrust-tracker-axum-http-server`             | `torrust-tracker-axum-http-server`             | `torrust_tracker_axum_http_server` → `torrust_tracker_axum_http_server`                         |
| `torrust-tracker-axum-rest-api-server`         | `torrust-tracker-axum-rest-api-server`         | `torrust_tracker_axum_rest_api_server` → `torrust_tracker_axum_rest_api_server`                 |
| `torrust-tracker-axum-server`                  | `torrust-tracker-axum-server`                  | `torrust_tracker_axum_server` → `torrust_tracker_axum_server`                                   |
| `torrust-tracker-rest-api-client`              | `torrust-tracker-rest-api-client`              | `torrust_tracker_rest_api_client` → `torrust_tracker_rest_api_client`                           |
| `torrust-tracker-rest-api-core`                | `torrust-tracker-rest-api-core`                | `torrust_tracker_rest_api_core` → `torrust_tracker_rest_api_core`                               |
| `torrust-tracker-udp-server`                   | `torrust-tracker-udp-server`                   | `torrust_tracker_udp_server` → `torrust_tracker_udp_server`                                     |

### Note on `torrust-server-lib`

`torrust-server-lib` is described as "Common functionality used in all Torrust HTTP
servers", implying it was intended to be reusable beyond the tracker (e.g., `torrust-index`).
Its only tracker-specific dependency is `torrust-tracker-primitives`, used solely for the
`ServiceBinding` type in `signals.rs` and `registar.rs`.

**Decision (see Open Questions)**: `torrust-server-lib` is **excluded from this rename**.
The `torrust-` prefix correctly reflects its intended cross-project reuse scope. The
dependency on `torrust-tracker-primitives` should be resolved separately — either by moving
`ServiceBinding` into `torrust-server-lib` itself or into a more neutral crate. A future
issue will cover that design decision.

## Scope

### In Scope

- Rename the `name` field in each of the 7 package `Cargo.toml` files.
- Update the root `Cargo.toml` workspace dependency keys.
- Update all `Cargo.toml` files in the workspace that reference the old names as
  dependencies.
- Update all Rust source files that use the crate identifiers (176 occurrences across
  `src/`, `packages/`, and `tests/`).
- Update prose references in `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and each package's
  `README.md`.
- Verify the workspace builds and all tests pass.

### Out of Scope

- Moving any crate to a separate repository.
- Changes to any crate's API or behaviour.
- Deciding the final scope of `torrust-server-lib` / `ServiceBinding` — that is a
  follow-up design discussion.
- Publishing any crate on crates.io.

## Open Questions

### Should `torrust-server-lib` stay `torrust-` scoped?

If `ServiceBinding` is moved out of `torrust-tracker-primitives` into a more neutral location
(or into `server-lib` itself), `torrust-server-lib` would have zero tracker-specific
dependencies and could legitimately serve `torrust-index` and other Torrust servers without
pulling in tracker logic. In that case, renaming it to `torrust-tracker-server-lib` now
would be a mistake.

| Option | Action                                                           | Trade-off                                                    |
| ------ | ---------------------------------------------------------------- | ------------------------------------------------------------ |
| A      | Rename to `torrust-tracker-server-lib` now                       | Consistent; can always rename back if dep is removed         |
| B      | Leave as `torrust-server-lib` until `ServiceBinding` is resolved | Preserves future intent; leaves naming inconsistency for now |

**Decision**: Option B. `torrust-server-lib` is excluded from this rename. The `torrust-`
prefix correctly reflects its intended cross-project reuse scope. The `ServiceBinding` dep
resolution is deferred to a separate issue. See the Note on `torrust-server-lib` in
Background.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                | Notes / Expected Output                                                                     |
| --- | ------ | --------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| T1  | DONE   | Rename `name` field in each of the 7 package `Cargo.toml` files                                     | See proposed name mapping above                                                             |
| T2  | DONE   | Update root `Cargo.toml` workspace dependency keys (7 entries)                                      | Replace old key names with new key names; `path` values stay unchanged                      |
| T3  | DONE   | Update dependency references in consumer `Cargo.toml` files (6 files)                               | See consumer file list below                                                                |
| T4  | DONE   | Update Rust source `use` / path references (176 occurrences)                                        | See identifier mapping in proposed name table; affects `src/`, `packages/`, `tests/`        |
| T5  | DONE   | Update prose in `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and each package `README.md` | Crate names and any inline code snippets referencing old names                              |
| T6  | DONE   | Run `cargo build --workspace` and `cargo test --workspace`                                          | Clean build; all tests pass                                                                 |
| T7  | DONE   | Run `linter all`                                                                                    | Exit code `0`                                                                               |
| T8  | DONE   | Update EPIC #1669 `Package Inventory` and `Desired Package State` tables                            | Move 7 entries from `torrust-` table to `torrust-tracker-` table; drop `Renamed from` notes |

**Consumer `Cargo.toml` files to update in T3** (6 files; some also appear in T1):

- `Cargo.toml` (root — workspace dependencies section)
- `packages/axum-health-check-api-server/Cargo.toml` — references `torrust-tracker-axum-server`
  (dep); `torrust-tracker-axum-health-check-api-server` (self, dev-dep),
  `torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server`,
  `torrust-tracker-udp-server` (dev-deps)
- `packages/axum-http-tracker-server/Cargo.toml` — references `torrust-tracker-axum-server`
- `packages/axum-rest-tracker-api-server/Cargo.toml` — references `torrust-tracker-axum-server`,
  `torrust-tracker-rest-api-client`, `torrust-tracker-rest-api-core`,
  `torrust-tracker-udp-server` (deps + dev-deps)
- `packages/rest-tracker-api-core/Cargo.toml` — references `torrust-tracker-udp-server`
- `packages/tracker-core/Cargo.toml` — references `torrust-tracker-rest-api-client`

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Open Question on `torrust-server-lib` resolved; decision recorded in spec
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [x] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669; all 7 packages
  confirmed unpublished on crates.io (no external migration required). `torrust-server-lib`
  excluded (Option B decision).
- 2026-05-20 00:00 UTC - josecelano - GitHub issue #1816 created; spec moved to
  `docs/issues/open/` with issue number prefix. SI-05 confirmed done: `server-lib` now
  depends on `torrust-net-primitives` (not `torrust-tracker-primitives`), validating the
  Option B exclusion decision.
- 2026-05-20 18:00 UTC - josecelano - Implementation complete. T1–T5 applied via sed across
  workspace (all 7 packages renamed in Cargo.toml name fields, workspace deps, consumer deps,
  Rust source identifiers, and prose). Fixed rand version constraint in udp-tracker-server and
  axum-http-tracker-server (rand = "0" → rand = "0.9") to resolve resolution regression caused
  by Cargo.lock regeneration after rename. T6: `cargo test --tests --workspace --all-targets
--all-features` passes. T7: `linter all` exits 0. T8: EPIC tables updated.

## Acceptance Criteria

- [x] No `Cargo.toml` in the workspace declares any of the 7 old crate names.
- [x] No Rust source file in the workspace uses any of the 7 old Rust identifiers.
- [x] `cargo build --workspace` succeeds with zero errors.
- [x] `cargo test --workspace` passes with zero failures.
- [x] `linter all` exits with code `0`.
- [x] `packages/AGENTS.md`, `AGENTS.md`, `docs/packages.md`, and each renamed package's `README.md` reflect the
      new crate names.
- [x] EPIC #1669 `Package Inventory` and `Desired Package State` tables are updated.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                 | Command/Steps                                                                                                                                                                                         | Expected Result                                                               | Status | Evidence |
| --- | ---------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- | ------ | -------- |
| M1  | No stale references to old names in TOML | `grep -r "torrust-axum-health-check\|torrust-axum-http-tracker\|torrust-axum-rest-tracker\|torrust-tracker-axum-server\b\|torrust-rest-tracker-api\|torrust-tracker-udp-server" . --include="*.toml"` | Zero matches (except own `name =` fields before rename, which should be gone) | TODO   |          |
| M2  | No stale identifiers in Rust source      | `grep -r "torrust_tracker_axum_http_server\|torrust_tracker_axum_rest_api_server\|torrust_rest_tracker_api\|torrust_tracker_udp_server\b" . --include="*.rs"`                                         | Zero matches                                                                  | TODO   |          |
