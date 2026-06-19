---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: 1669
github-issue: 1908
spec-path: docs/issues/open/1908-1669-si-27-move-driver-enum-to-primitives.md
branch: null
related-pr: null
last-updated-utc: 2026-06-10
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-06-10.md
---

<!-- skill-link: create-issue -->

# Issue #1908 (SI-27) - Move `Driver` Enum from `configuration` to `primitives`

## Subissue of EPIC #1669 — Overhaul: Packages

## Problem

The `Driver` enum (`Sqlite3`, `MySQL`, `PostgreSQL`) is currently defined in
`torrust-tracker-configuration` as a TOML deserialization type. However, it is
a cross-cutting domain concept used by multiple packages:

- `configuration` — to deserialize `database.driver` from `tracker.toml`
- `tracker-core` — to select which DB driver to initialize (with a _duplicate_ copy
  of the same enum and a pointless mapping between the two)
- `persistence-benchmark` — to set up per-driver benchmarks

The current duplication in `tracker-core` (see `packages/tracker-core/src/databases/driver/mod.rs`)
is a symptom of misplaced ownership. The enum sits in `configuration` because that is where
it is deserialized, but it leaks into inner layers that should not depend on the full
configuration package solely for a stable, cross-cutting concept.

## Scope

### 1. Add a decision to DECISIONS.md

Record a new decision (DEC-10 or next available) with the rationale for moving `Driver`
to `primitives` — this is not about TslConfig-style acceptance but about recognizing
that cross-cutting domain types belong in a shared home.

### 2. Move `Driver` enum

- Move the `Driver` enum definition from `packages/configuration/src/v2_0_0/database.rs`
  to `packages/primitives/src/` (e.g. `packages/primitives/src/driver.rs`)
- Re-export it from `packages/primitives/src/lib.rs`
- Remove the duplicate `Driver` enum from `packages/tracker-core/src/databases/driver/mod.rs`

### 3. Update consumers

| Package                 | Change                                                                                                                                      |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `configuration`         | Import `torrust_tracker_primitives::Driver` and re-export as `type Driver = torrust_tracker_primitives::Driver;` for backward compatibility |
| `tracker-core`          | Remove the duplicate enum; remove the mapping in `setup.rs`; use `primitives::Driver` directly                                              |
| `persistence-benchmark` | Import `TorrustTrackerPrimitives::Driver` directly (already depends on `primitives`)                                                        |

### 4. Remove `configuration` dependency from `tracker-core`

After the move, `tracker-core` no longer needs to import `configuration::Driver`.
If importing `configuration::Core` is still needed (for `config.database.path` etc.),
keep that dependency. But the coupling for `Driver` specifically is eliminated.

### 5. Clean up

- Run `cargo machete` to verify no unused deps remain
- Update any existing `use` paths across the workspace
- Verify `linter all` and `cargo test --workspace`

## Acceptance Criteria

1. `Driver` is defined once in `torrust-tracker-primitives` and used by all consumers.
2. No duplicate `Driver` enum exists in any package.
3. No mapping code converts between two identical enums.
4. `cargo test --workspace` passes.
5. `cargo machete` passes (no unused deps).
6. `linter all` passes.
7. A decision (DEC-XX) is recorded in `DECISIONS.md`.

## Verification

- [x] DEC-14 added to `docs/issues/open/1669-overhaul-packages/DECISIONS.md`
- [x] `Driver` defined in `primitives`, all consumers import it directly
- [x] Duplicate in `tracker-core` removed
- [x] Mapping in `setup.rs` simplified
- [x] `cargo test --workspace` — pass
- [x] `cargo machete` — pass (no new unused deps)
- [x] `linter all` — pass
