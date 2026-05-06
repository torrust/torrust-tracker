# Step 6: Primitives Module Refactor Plan

## Goal

Refactor `packages/primitives/src` so announce-related and scrape-related primitives live in
separate modules with clearer ownership boundaries, while preserving compatibility for existing
workspace consumers during the migration.

## Scope

In scope:

- Split `packages/primitives/src/core.rs` into action-oriented modules
- Introduce `announce.rs` and `scrape.rs` under `packages/primitives/src`
- Move `AnnounceData` into `announce.rs`
- Move `ScrapeData` into `scrape.rs`
- Move `packages/primitives/src/announce_event.rs` logic into `announce.rs`
- Preserve existing public API during migration through compatibility re-exports
- Keep all current workspace consumers building without behavior changes

Out of scope:

- Renaming public data structures
- Redesigning tracker-core announce/scrape domain semantics
- Large cross-package cleanup of shared primitive types
- Removing compatibility exports in the first step

## Current Layout

Current source files involved:

- `core.rs`
- `announce_event.rs`
- `lib.rs`

Current problem:

- `core.rs` mixes announce and scrape concerns in a single module.
- `announce_event.rs` is announce-specific but lives outside the announce area.
- Many workspace consumers currently import `AnnounceData` and `ScrapeData` from
  `torrust_tracker_primitives::core`, so ownership is unclear and future cleanup is harder.

## Target Layout

Planned source files:

- `announce.rs` (`AnnounceData`, `AnnounceEvent`)
- `scrape.rs` (`ScrapeData`)
- `core.rs` (temporary compatibility wrapper only)
- `lib.rs` (re-exports and module declarations)

## Final Module Map (Implemented)

- `announce.rs`: owns `AnnounceData` and `AnnounceEvent`
- `scrape.rs`: owns `ScrapeData`
- `announce_event.rs`: compatibility wrapper re-exporting `announce::AnnounceEvent`
- `core.rs`: compatibility wrapper re-exporting `announce::AnnounceData` and `scrape::ScrapeData`
- `lib.rs`: root compatibility re-exports for `AnnounceData`, `AnnounceEvent`, and `ScrapeData`

## Final Module Intent

`announce.rs` owns announce-only primitives:

- `AnnounceData`
- `AnnounceEvent`

`scrape.rs` owns scrape-only primitives:

- `ScrapeData`

`core.rs` is temporarily retained only for compatibility:

- re-export `AnnounceData`
- re-export `ScrapeData`
- avoid new concrete logic

`lib.rs` preserves root-level compatibility and exposes the new module structure.

## Migration Strategy

Follow the same strategy used for the `udp-protocol` refactor:

- move one type at a time
- re-export moved types from `lib.rs` immediately
- preserve compatibility before updating consumers
- validate after each type move before starting the next one
- use one signed commit per logical slice

This allows internal reorganization without breaking current or future consumers while the
module layout evolves.

## Constraints

- Preserve all current behavior.
- Keep `torrust_tracker_primitives::core::AnnounceData` and
  `torrust_tracker_primitives::core::ScrapeData` working during the migration.
- Keep `torrust_tracker_primitives::AnnounceEvent` working during the migration.
- Avoid unnecessary churn outside `packages/primitives` until compatibility exports are in place.

## Current Consumer Notes

Known current import patterns in the workspace:

- `torrust_tracker_primitives::core::AnnounceData`
- `torrust_tracker_primitives::core::ScrapeData`
- `torrust_tracker_primitives::AnnounceEvent`

This means the refactor should prioritize compatibility re-exports before call-site cleanup.

## Implementation Decisions (Proposed)

- Introduce `announce.rs` and `scrape.rs` first as empty/new target modules.
- Move one type at a time instead of moving all announce or scrape types in a single step.
- Re-export moved types from `lib.rs` immediately after each move.
- Keep `core.rs` as a stable compatibility wrapper during the refactor.
- Prefer delaying consumer import cleanup until after compatibility is in place.
- Use one signed commit per logical slice.

## Execution Plan

### Phase 0: Baseline and Safety Net

- [x] Record baseline:
  - [x] `cargo check --workspace`
  - [x] `cargo test --workspace`
  - [x] `linter all`
- [x] Capture current `packages/primitives/src/lib.rs` exports
- [x] Capture current workspace import usage (`rg`)

Exit criteria:

- [x] Baseline recorded and green

### Phase 1: Introduce Action-Oriented Primitive Modules

- [x] Create `packages/primitives/src/announce.rs`
- [x] Create `packages/primitives/src/scrape.rs`
- [x] Update `lib.rs` to declare and re-export the new modules

Exit criteria:

- [x] `cargo check --workspace` passes
- [x] `linter all` passes

### Phase 2: Preserve Compatibility

- [x] Convert `core.rs` into a compatibility wrapper module
- [x] Re-export `AnnounceData` and `ScrapeData` from `core.rs`
- [x] Preserve `torrust_tracker_primitives::AnnounceEvent` via `lib.rs` re-export
- [x] Verify existing consumers still compile unchanged

Exit criteria:

- [x] Existing import paths continue to work
- [x] No workspace build regressions

### Phase 3: Type-by-Type Migration

- [x] Move `AnnounceData` into `announce.rs`
- [x] Re-export `AnnounceData` from `lib.rs`
- [x] Validate after the `AnnounceData` move
- [x] Move `AnnounceEvent` from `announce_event.rs` into `announce.rs`
- [x] Preserve root `AnnounceEvent` re-export from `lib.rs`
- [x] Validate after the `AnnounceEvent` move
- [x] Move `ScrapeData` into `scrape.rs`
- [x] Re-export `ScrapeData` from `lib.rs`
- [x] Validate after the `ScrapeData` move

Exit criteria:

- [x] Each moved type remains available through compatibility exports
- [x] Each per-type move passes validation before the next move starts

### Phase 4: Optional Consumer Cleanup

- [x] Decide whether internal consumers should migrate from `core::*` to `announce::*` / `scrape::*`
- [ ] Update internal imports only where it improves clarity
- [x] Keep compatibility re-exports until a separate cleanup/removal task

Exit criteria:

- [x] New ownership boundaries are clear
- [x] Compatibility strategy is documented

### Phase 5: Final Documentation

- [x] Document final module map
- [x] Record any follow-up work for eventual compatibility wrapper removal

Exit criteria:

- [x] Final module structure documented
- [x] Remaining follow-up work explicitly listed

## Tracking Checklist

### Deliverables

- [x] `announce.rs` added
- [x] `scrape.rs` added
- [x] `AnnounceData` moved
- [x] `ScrapeData` moved
- [x] `AnnounceEvent` moved
- [x] `core.rs` reduced to compatibility exports
- [x] `lib.rs` updated
- [x] Docs updated

### Type-by-Type Progress Tracker

- [x] `AnnounceData`
  - [x] moved to `announce.rs`
  - [x] re-exported from `lib.rs`
  - [x] compatibility preserved
  - [x] consumers validated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `ScrapeData`
  - [x] moved to `scrape.rs`
  - [x] re-exported from `lib.rs`
  - [x] compatibility preserved
  - [x] consumers validated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `AnnounceEvent`
  - [x] moved to `announce.rs`
  - [x] re-exported from `lib.rs`
  - [x] root re-export preserved
  - [x] consumers validated
  - [x] validated (`cargo check --workspace`, `linter all`)

### Per-Type Migration Workflow

For each type, execute this sequence before starting the next one:

1. Move one type to its target module.
2. Add or adjust the `pub use` re-export in `lib.rs`.
3. Preserve compatibility exports before touching consumers.
4. Run validation gate for that single move:
   - `cargo check --workspace`
   - `linter all`
5. Mark the type row/checklist as validated.

## Validation Gate

- [x] `cargo check --workspace`
- [x] `cargo test --workspace`
- [x] `cargo test --doc --workspace`
- [x] `linter all`

## Risk Register

### Risk 1: Breaking `core::*` imports

Impact: high

Mitigation:

- Keep `core.rs` as a compatibility wrapper first
- Validate all current consumers with workspace-wide checks

### Risk 2: Incomplete announce ownership move

Impact: medium

Mitigation:

- Keep announce-related primitives co-located by the end of the refactor
- Still move one type at a time so validation remains narrow and reversible

### Risk 3: Over-scoping the refactor

Impact: medium

Mitigation:

- Limit this task to module boundaries and compatibility
- Defer deeper domain redesign or wrapper removal to future work

## Review Checklist

- [x] Announce-related primitives are co-located
- [x] Scrape-related primitives are isolated
- [x] Compatibility exports preserve current consumers
- [x] No unnecessary behavior changes introduced
- [x] Follow-up cleanup work is documented

## Suggested Commit Slicing

1. [x] `refactor(primitives): add announce and scrape modules`
2. [x] `refactor(primitives): move AnnounceData to announce module`
3. [x] `refactor(primitives): move AnnounceEvent to announce module`
4. [x] `refactor(primitives): move ScrapeData to scrape module`
5. [x] `refactor(primitives): keep core module as compatibility wrapper`
6. [ ] `docs(issue-1732): document final primitives module layout`

## Follow-Up Work

- Optionally migrate internal consumers from `core::*` imports to `announce::*` and `scrape::*`
  where that improves clarity.
- Keep compatibility re-exports in place until a separate cleanup task explicitly removes them.
