---
doc-type: issue
issue-type: task
status: open
priority: p3
github-issue: 1790
spec-path: docs/issues/open/1790-move-duration-since-unix-epoch-to-torrust-tracker-clock.md
branch: 1790-move-duration-since-unix-epoch
related-pr: 1791
last-updated-utc: 2026-05-18 20:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/primitives/src/lib.rs
    - packages/clock/Cargo.toml
    - packages/clock/src/clock/mod.rs
    - packages/clock/src/conv/mod.rs
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #1790 - Move `DurationSinceUnixEpoch` from `torrust-tracker-primitives` to `torrust-tracker-clock`

## Goal

Move the `DurationSinceUnixEpoch` type alias from `torrust-tracker-primitives` into
`torrust-tracker-clock` — where it semantically belongs — and update all workspace consumers
to import it from `torrust-tracker-clock`. This removes the `torrust-tracker-primitives`
dependency from `torrust-tracker-clock`, preparing the crate for future extraction to a
standalone repository.

## Background

`DurationSinceUnixEpoch` is defined in `packages/primitives/src/lib.rs` as:

```rust
pub type DurationSinceUnixEpoch = Duration;
```

It is a trivial alias for `std::time::Duration` with no tracker-specific logic. The
`torrust-tracker-clock` package is the primary user of this type: it appears in the `Clock`
trait itself (`fn now() -> DurationSinceUnixEpoch`) and in the conversion helpers
(`packages/clock/src/conv/mod.rs`). Having it live in `torrust-tracker-primitives` is an
accident of history, not a design intent.

`torrust-tracker-clock` currently carries a `torrust-tracker-primitives` dependency solely
for this type alias. Removing it makes `torrust-tracker-clock` dependency-lighter and
prepares it for future rename/extraction (SI-09, SI-13).

**Key implementation note**: Since `DurationSinceUnixEpoch` is a trivial type alias (both
the old and new definitions are `= std::time::Duration`), there is no type incompatibility
between `torrust_tracker_primitives::DurationSinceUnixEpoch` and
`torrust_tracker_clock::DurationSinceUnixEpoch`. All 80+ workspace files that currently
import the type from `torrust-tracker-primitives` need only a trivial import path change.

**Backward compatibility and deprecation**: Now that `torrust-tracker-clock` no longer
depends on `torrust-tracker-primitives`, there is no circular dependency, and
`torrust-tracker-primitives` can safely depend on `torrust-tracker-clock`. Rather than
leaving a stale independent copy, `torrust-tracker-primitives` now re-exports the type
from `torrust-tracker-clock` via `#[deprecated] pub use torrust_tracker_clock::DurationSinceUnixEpoch`.
This preserves backward compatibility for external consumers while actively signalling that
they should migrate to the `torrust_tracker_clock` import path. Removal of the re-export
is deferred to a follow-up cleanup subissue of EPIC #1669.

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Add `pub type DurationSinceUnixEpoch = std::time::Duration;` to `packages/clock/src/lib.rs`
  (or a dedicated `types.rs` module), exported as part of the public API.
- Update `packages/clock/src/clock/mod.rs` and `packages/clock/src/conv/mod.rs` to use the
  local definition instead of importing from `torrust-tracker-primitives`.
- Remove the `torrust-tracker-primitives` dependency from `packages/clock/Cargo.toml`
  (it was added only for this type alias).
- Update all 80+ workspace files that import `DurationSinceUnixEpoch` from
  `torrust_tracker_primitives` to import it from `torrust_tracker_clock` instead.
- Verify the workspace builds and all tests pass.
- Update `torrust-tracker-metrics` to import `DurationSinceUnixEpoch` from
  `torrust-tracker-clock` instead of `torrust-tracker-primitives`, eliminating that
  dependency edge entirely (see F-02).

### Out of Scope

- Removing `DurationSinceUnixEpoch` from `torrust-tracker-primitives`: that requires a
  crates.io version bump to signal the breaking change; deferred to a separate cleanup
  subissue once all consumers have migrated.
- Changes to the type itself — it stays `= std::time::Duration`.
- Extracting `torrust-tracker-clock` to a standalone repository (a separate, later subissue).
- Renaming `torrust-tracker-clock` to `torrust-clock` (tracked in SI-09, a separate subissue).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                                                                                                                                           | Notes / Expected Output                                                                                              |
| --- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Define `DurationSinceUnixEpoch` in `packages/clock/src/lib.rs`                                                                                                                                                                 | `pub type DurationSinceUnixEpoch = std::time::Duration;`                                                             |
| T2  | DONE   | Update `packages/clock/src/clock/mod.rs` and `packages/clock/src/conv/mod.rs` to use the local definition                                                                                                                      | Replace `use torrust_tracker_primitives::DurationSinceUnixEpoch` with local import                                   |
| T3  | DONE   | Remove `torrust-tracker-primitives` dep from `packages/clock/Cargo.toml`                                                                                                                                                       | Dep entry removed; workspace build still passes                                                                      |
| T4  | DONE   | Update all 80+ workspace files to import `DurationSinceUnixEpoch` from `torrust_tracker_clock` instead of `torrust_tracker_primitives`                                                                                         | Use M1 grep to find the full file list; one-line change per file                                                     |
| T5  | DONE   | Run `cargo build --workspace` and `cargo test --workspace`                                                                                                                                                                     | Clean build and all tests pass                                                                                       |
| T6  | DONE   | Run `linter all`                                                                                                                                                                                                               | Exit code `0`                                                                                                        |
| T7  | DONE   | Update EPIC #1669 extraction ordering table: note that `torrust-tracker-clock` has no `torrust-tracker-primitives` dep                                                                                                         | `torrust-tracker-clock` row: `torrust-tracker-primitives` dep removed                                                |
| T8  | DONE   | Update `torrust-tracker-metrics`: replace import of `DurationSinceUnixEpoch` from `torrust_tracker_primitives` with `torrust_tracker_clock`; remove `torrust-tracker-primitives` dep from its `Cargo.toml` if no longer needed | `cargo build -p torrust-tracker-metrics` succeeds; `cargo machete -p torrust-tracker-metrics` reports no unused deps |

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
- [ ] PR merged
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669 following
  Option A decision in clock rename spec. `DurationSinceUnixEpoch` has 80+ workspace
  consumers; all import from `torrust-tracker-primitives` today.
- 2026-05-18 00:00 UTC - josecelano - Spec updated to target current crate name
  `torrust-tracker-clock` (Option A: proceed without SI-09 prerequisite). SI-09 prerequisite
  removed; type will land as `torrust_tracker_clock::DurationSinceUnixEpoch`.
- 2026-05-18 18:30 UTC - josecelano - Implementation complete. All 77 workspace files
  updated. `torrust-tracker-clock` no longer depends on `torrust-tracker-primitives`.
  `torrust-tracker-metrics` now imports from `torrust-tracker-clock`.
  `cargo build --workspace`, `cargo test --workspace`, and `linter all` all pass.
- 2026-05-18 20:00 UTC - josecelano - `torrust-tracker-primitives` re-export added as
  `#[deprecated] pub use torrust_tracker_clock::DurationSinceUnixEpoch` for backward
  compatibility. `peer.rs` migrated to import directly from `torrust_tracker_clock`.
  PR #1791 opened against `develop`.

## Acceptance Criteria

- [x] `packages/clock/src/lib.rs` (or a submodule) exports `pub type DurationSinceUnixEpoch = std::time::Duration`.
- [x] `packages/clock/Cargo.toml` does not list `torrust-tracker-primitives` as a dependency.
- [x] No file in `packages/clock/src/` imports `DurationSinceUnixEpoch` from `torrust_tracker_primitives`.
- [x] No other workspace file imports `DurationSinceUnixEpoch` from `torrust_tracker_primitives`
      (all migrated to `torrust_tracker_clock`).
- [x] `torrust-tracker-metrics` no longer lists `torrust-tracker-primitives` as a dependency
      (or only lists it for non-`DurationSinceUnixEpoch` reasons).
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

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                            | Command / Steps                                                                   | Expected Result                         | Status | Evidence                                                                                |
| --- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------- | --------------------------------------- | ------ | --------------------------------------------------------------------------------------- |
| M1  | No workspace import from `torrust_tracker_primitives` for this type | `grep -r "torrust_tracker_primitives::DurationSinceUnixEpoch" . --include="*.rs"` | Zero matches                            | DONE   | Zero matches (only `primitives/` defines the type; no consumer imports it from there)   |
| M2  | `torrust-tracker-clock` dep list is clean                           | `grep "torrust-tracker-primitives" packages/clock/Cargo.toml`                     | No output                               | DONE   | No output confirmed                                                                     |
| M3  | `torrust-tracker-clock` exports `DurationSinceUnixEpoch`            | `grep "DurationSinceUnixEpoch" packages/clock/src/lib.rs`                         | `pub type DurationSinceUnixEpoch` found | DONE   | `pub type DurationSinceUnixEpoch = std::time::Duration;` in `packages/clock/src/lib.rs` |
