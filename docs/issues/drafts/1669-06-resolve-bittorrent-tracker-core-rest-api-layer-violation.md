---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1669-06-resolve-bittorrent-tracker-core-rest-api-layer-violation.md
branch: null
related-pr: null
last-updated-utc: 2026-05-18 12:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/tracker-core/Cargo.toml
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Resolve `bittorrent-tracker-core` ↔ `torrust-rest-tracker-api-client` layer violation

## Goal

Remove the stale dev dependency from `bittorrent-tracker-core` on
`torrust-rest-tracker-api-client`. A pre-implementation audit revealed that the dependency is
declared in `packages/tracker-core/Cargo.toml` but is never imported or used anywhere in
`src/` or `tests/`. The fix is a one-line `Cargo.toml` deletion.

## Background

The coupling analysis (F-05) found:

> `bittorrent-tracker-core` → `torrust-rest-tracker-api-client` [dev]

The entry was listed in `[dev-dependencies]` of `packages/tracker-core/Cargo.toml` (line 48),
which caused the coupling tool to report it as a layer violation. However, auditing
`packages/tracker-core/tests/` and `packages/tracker-core/src/` shows **zero uses** of
`torrust_rest_tracker_api_client` anywhere in the crate. The dependency is dead — left over
from a previous refactor.

No code movement or extraction is needed. `cargo machete` would also flag this as an unused
dependency.

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Remove `torrust-rest-tracker-api-client` from `packages/tracker-core/Cargo.toml`
  `[dev-dependencies]`.
- Verify the workspace builds and all tests pass.

### Out of Scope

- Extracting `bittorrent-tracker-core` to a standalone repository (a separate, later subissue).
- Any code movement or refactoring — the dependency is unused, so no consumers need updating.

## Open Questions

None. Pre-implementation audit confirmed the dependency is unused.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                  | Notes / Expected Output     |
| --- | ------ | ----------------------------------------------------------------------------------------------------- | --------------------------- |
| T1  | TODO   | Remove `torrust-rest-tracker-api-client` from `packages/tracker-core/Cargo.toml` `[dev-dependencies]` | One-line deletion           |
| T2  | TODO   | Run `cargo build --workspace` and `cargo test --workspace`                                            | Clean build; all tests pass |
| T3  | TODO   | Run `linter all`                                                                                      | Exit code `0`               |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-18 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669, addressing F-05
  from the coupling analysis report. Initially assumed code extraction was needed.
- 2026-05-18 12:00 UTC - josecelano - Audit confirmed the dependency is unused (zero imports
  in `src/` and `tests/`). Spec revised: no extraction required; fix is a one-line `Cargo.toml`
  deletion.

## Acceptance Criteria

- [ ] `packages/tracker-core/Cargo.toml` does not list `torrust-rest-tracker-api-client` in
      `[dev-dependencies]`.
- [ ] All `bittorrent-tracker-core` integration tests still compile and pass.
- [ ] `cargo build --workspace` succeeds with zero errors.
- [ ] `cargo test --workspace` passes with zero failures.
- [ ] `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                  | Command / Steps                                                           | Expected Result | Status | Evidence |
| --- | --------------------------------------------------------- | ------------------------------------------------------------------------- | --------------- | ------ | -------- |
| M1  | No dev dep on `rest-tracker-api-client` in `tracker-core` | `grep "torrust-rest-tracker-api-client" packages/tracker-core/Cargo.toml` | Zero matches    | TODO   |          |
| M2  | `bittorrent-tracker-core` integration tests pass          | `cargo test -p bittorrent-tracker-core --tests`                           | All pass        | TODO   |          |
