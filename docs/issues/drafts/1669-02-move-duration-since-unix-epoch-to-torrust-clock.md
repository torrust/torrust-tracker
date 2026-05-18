---
doc-type: issue
issue-type: task
status: draft
priority: p3
github-issue: null
spec-path: docs/issues/drafts/1669-02-move-duration-since-unix-epoch-to-torrust-clock.md
branch: null
related-pr: null
last-updated-utc: 2026-05-18 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/primitives/src/lib.rs
    - packages/clock/Cargo.toml
    - packages/clock/src/clock/mod.rs
    - packages/clock/src/conv/mod.rs
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/drafts/1669-09-rename-torrust-tracker-clock-to-torrust-clock.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Move `DurationSinceUnixEpoch` from `torrust-tracker-primitives` to `torrust-clock`

## Goal

Move the `DurationSinceUnixEpoch` type alias from `torrust-tracker-primitives` into
`torrust-clock` — where it semantically belongs — and update all workspace consumers to
import it from `torrust-clock`. This removes the only `torrust-tracker-*` dependency from
`torrust-clock`, making the crate fully self-contained and ready for future extraction to a
standalone repository.

## Background

`DurationSinceUnixEpoch` is defined in `packages/primitives/src/lib.rs` as:

```rust
pub type DurationSinceUnixEpoch = Duration;
```

It is a trivial alias for `std::time::Duration` with no tracker-specific logic. The
`torrust-clock` package is the primary user of this type: it appears in the `Clock` trait
itself (`fn now() -> DurationSinceUnixEpoch`) and in the conversion helpers
(`packages/clock/src/conv/mod.rs`). Having it live in `torrust-tracker-primitives` is an
accident of history, not a design intent.

After the clock rename (see
[1669-09-rename-torrust-tracker-clock-to-torrust-clock.md](1669-09-rename-torrust-tracker-clock-to-torrust-clock.md)),
`torrust-clock` still carries a `torrust-tracker-primitives` dependency solely for this
type alias. A generic `torrust-clock` crate depending on a `torrust-tracker-*` package is
semantically inconsistent and would block future extraction to a standalone repository.

**Key implementation note**: Since `DurationSinceUnixEpoch` is a trivial type alias (both
the old and new definitions are `= std::time::Duration`), there is no type incompatibility
between `torrust_tracker_primitives::DurationSinceUnixEpoch` and
`torrust_clock::DurationSinceUnixEpoch`. All 80+ workspace files that currently import the
type from `torrust-tracker-primitives` need only a trivial import path change.

**Circular dep constraint**: `torrust-tracker-primitives` must **not** re-export the type
from `torrust-clock`. That would introduce a circular dependency (since `torrust-clock`
previously depended on primitives). Instead, `torrust-tracker-primitives` retains its own
independent `pub type DurationSinceUnixEpoch = Duration` definition. Once all workspace
consumers have been migrated to `torrust_clock::DurationSinceUnixEpoch`, the copy in
`torrust-tracker-primitives` can be deprecated and removed in a future cleanup.

**Prerequisite**: SI-09 technical steps must be complete before this subissue begins: the
crate must be renamed (`name = "torrust-clock"` in `packages/clock/Cargo.toml`), all
dependent `Cargo.toml` files updated to use the new key, and all `use`-path references
migrated to `torrust_clock::` (SI-09 T1–T4). The EPIC table update (SI-09 T12) is not a
blocker.

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
  `torrust_tracker_primitives` to import it from `torrust_clock` instead.
- Verify the workspace builds and all tests pass.
- Update `torrust-tracker-metrics` to import `DurationSinceUnixEpoch` from `torrust-clock`
  instead of `torrust-tracker-primitives`, eliminating that dependency edge entirely (see F-02).

### Out of Scope

- Removing `DurationSinceUnixEpoch` from `torrust-tracker-primitives`: that requires a
  crates.io version bump to signal the breaking change; deferred to a separate cleanup
  subissue once all consumers have migrated.
- Changes to the type itself — it stays `= std::time::Duration`.
- Extracting `torrust-clock` to a standalone repository (a separate, later subissue).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status  | Task                                                                                                                                                                                                                   | Notes / Expected Output                                                                                              |
| --- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| T1  | BLOCKED | Confirm SI-09 technical steps complete (T1–T4: crate renamed to `torrust-clock`, dep keys updated, `use`-paths migrated workspace-wide)                                                                                | `name = "torrust-clock"` in `packages/clock/Cargo.toml`; workspace builds cleanly                                    |
| T2  | TODO    | Define `DurationSinceUnixEpoch` in `packages/clock/src/lib.rs`                                                                                                                                                         | `pub type DurationSinceUnixEpoch = std::time::Duration;`                                                             |
| T3  | TODO    | Update `packages/clock/src/clock/mod.rs` and `packages/clock/src/conv/mod.rs` to use the local definition                                                                                                              | Replace `use torrust_tracker_primitives::DurationSinceUnixEpoch` with local import                                   |
| T4  | TODO    | Remove `torrust-tracker-primitives` dep from `packages/clock/Cargo.toml`                                                                                                                                               | Dep entry removed; workspace build still passes                                                                      |
| T5  | TODO    | Update all 80+ workspace files to import `DurationSinceUnixEpoch` from `torrust_clock` instead of `torrust_tracker_primitives`                                                                                         | Use M1 grep to find the full file list; one-line change per file                                                     |
| T6  | TODO    | Run `cargo build --workspace` and `cargo test --workspace`                                                                                                                                                             | Clean build and all tests pass                                                                                       |
| T7  | TODO    | Run `linter all`                                                                                                                                                                                                       | Exit code `0`                                                                                                        |
| T8  | TODO    | Update EPIC #1669 extraction ordering table: note that `torrust-clock` has no `torrust-tracker-*` deps                                                                                                                 | `torrust-clock` row: unpublished runtime workspace deps column set to `None`                                         |
| T9  | TODO    | Update `torrust-tracker-metrics`: replace import of `DurationSinceUnixEpoch` from `torrust_tracker_primitives` with `torrust_clock`; remove `torrust-tracker-primitives` dep from its `Cargo.toml` if no longer needed | `cargo build -p torrust-tracker-metrics` succeeds; `cargo machete -p torrust-tracker-metrics` reports no unused deps |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] Clock rename subissue complete (prerequisite)
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669 following
  Option A decision in clock rename spec. `DurationSinceUnixEpoch` has 80+ workspace
  consumers; all import from `torrust-tracker-primitives` today.

## Acceptance Criteria

- [ ] `packages/clock/src/lib.rs` (or a submodule) exports `pub type DurationSinceUnixEpoch = std::time::Duration`.
- [ ] `packages/clock/Cargo.toml` does not list `torrust-tracker-primitives` as a dependency.
- [ ] No file in `packages/clock/src/` imports `DurationSinceUnixEpoch` from `torrust_tracker_primitives`.
- [ ] No other workspace file imports `DurationSinceUnixEpoch` from `torrust_tracker_primitives`
      (all migrated to `torrust_clock`).
- [ ] `torrust-tracker-metrics` no longer lists `torrust-tracker-primitives` as a dependency
      (or only lists it for non-`DurationSinceUnixEpoch` reasons).
- [ ] `cargo build --workspace` succeeds with zero errors.
- [ ] `cargo test --workspace` passes with zero failures.
- [ ] `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test --doc --workspace`
- `cargo test --tests --workspace --all-targets --all-features`
- `linter all`
- `cargo machete` (no unused dependencies)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                            | Command / Steps                                                                   | Expected Result                         | Status | Evidence |
| --- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------- | --------------------------------------- | ------ | -------- |
| M1  | No workspace import from `torrust_tracker_primitives` for this type | `grep -r "torrust_tracker_primitives::DurationSinceUnixEpoch" . --include="*.rs"` | Zero matches                            | TODO   |          |
| M2  | `torrust-clock` dep list is clean                                   | `grep "torrust-tracker-primitives" packages/clock/Cargo.toml`                     | No output                               | TODO   |          |
| M3  | `torrust-clock` exports `DurationSinceUnixEpoch`                    | `grep "DurationSinceUnixEpoch" packages/clock/src/lib.rs`                         | `pub type DurationSinceUnixEpoch` found | TODO   |          |
