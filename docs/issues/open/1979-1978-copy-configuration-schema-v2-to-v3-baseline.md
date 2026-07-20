---
doc-type: issue
issue-type: task
status: done
priority: p0
github-issue: 1979
spec-path: docs/issues/open/1979-1978-copy-configuration-schema-v2-to-v3-baseline.md
branch: "config-copy-v2-to-v3-baseline"
related-pr: 1999
last-updated-utc: 2026-07-20 13:21
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v2_0_0/
    - packages/configuration/src/lib.rs
    - share/default/config/
---

# Issue #1979 - Copy configuration schema v2_0_0 to v3_0_0 as baseline

> **EPIC position**: Subissue #1 of 9 in EPIC #1978. **Foundation — all other subissues depend on this.** Must be merged before any other subissue begins.

## Goal

Copy the entire `packages/configuration/src/v2_0_0/` module to `packages/configuration/src/v3_0_0/` as the starting point for all breaking changes in the Configuration Overhaul EPIC. Also copy the crate-root `logging.rs` (which contains `TraceStyle`, `setup()`, and `tracing_init()`) into both `v2_0_0/` and `v3_0_0/` so each versioned module is fully self-contained (data types + behaviour). Wire `v3_0_0` as the default schema version while keeping `v2_0_0` available for backward compatibility during the transition.

## Background

The Configuration Overhaul EPIC groups multiple breaking changes to the configuration schema. Rather than modifying `v2_0_0` in place (which would break existing consumers), we create a new `v3_0_0` module as a copy of `v2_0_0`. Each subsequent subissue in the EPIC applies its changes to the `v3_0_0` module only.

This approach:

- Keeps `v2_0_0` intact for any consumers that still need it
- Provides a clean baseline for all v3 changes
- Allows incremental migration — each subissue modifies only the v3 types
- Makes it easy to compare v2 vs v3 during review
- Makes each versioned module fully self-contained by copying the crate-root `logging.rs` (which contains `TraceStyle`, `setup()`, and `tracing_init()`) into both `v2_0_0/` and `v3_0_0/`

## Scope

### In Scope

- Copy `packages/configuration/src/v2_0_0/` → `packages/configuration/src/v3_0_0/`
- Copy `packages/configuration/src/logging.rs` into `v2_0_0/logging.rs` and `v3_0_0/logging.rs` (making each versioned module self-contained)
- Update `packages/configuration/src/lib.rs` to expose both `v2_0_0` and `v3_0_0` modules
- Wire `v3_0_0` as the default schema version used by the application
- Update `share/default/config/` files to reference `schema_version = "3.0.0"`
- Ensure all existing tests still pass (v2_0_0 unchanged)
- Add basic smoke tests for v3_0_0 deserialization

### Out of Scope

- Any functional changes to the configuration types (those come in subsequent subissues)
- Removing `v2_0_0` module (deprecated but kept for transition)
- Updating consumers outside `packages/configuration` (done in Phase 4 of the EPIC)

## Implementation Plan

| ID  | Status           | Task                                                         | Notes                                                                                                                                     |
| --- | ---------------- | ------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE             | Copy `v2_0_0/` directory to `v3_0_0/`                        | `cp -r packages/configuration/src/v2_0_0/ packages/configuration/src/v3_0_0/`                                                             |
| T2  | DONE             | Update `v3_0_0/mod.rs` to use `crate::v3_0_0` internal paths | Fixed all doc links, VERSION constant, test imports, and schema_version strings                                                           |
| T3  | DONE             | Copy `logging.rs` into `v2_0_0/logging.rs`                   | Merged TraceStyle/setup/tracing_init into the versioned logging.rs; added module-level doc comment                                        |
| T4  | DONE             | Copy `logging.rs` into `v3_0_0/logging.rs`                   | Same content as T3; v3 gets its own copy                                                                                                  |
| T5  | DONE             | Update `lib.rs` to expose `pub mod v3_0_0`                   | Added alongside existing `pub mod v2_0_0`; added `Metadata::with_schema_version` helper; global re-exports stay at v2                     |
| T6  | DEFERRED → #1980 | Update default config files to `schema_version = "3.0.0"`    | Cannot be done while bootstrap still uses `v2_0_0::Configuration`; config files and bootstrap switch together in #1980                    |
| T7  | DEFERRED → #1980 | Wire application entry point to use `v3_0_0` by default      | Requires updating bootstrap + all consumers; this is exactly the scope of subissue #1980                                                  |
| T8  | DONE             | Add smoke tests: deserialize default v3 config               | Added `smoke::v3_configuration_should_load_when_schema_version_is_3_0_0` and `smoke::v3_configuration_should_reject_schema_version_2_0_0` |
| T9  | DONE             | Run `linter all` and full test suite                         | All 48 test suites pass (0 failures)                                                                                                      |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests)
- [ ] Manual verification scenarios executed and recorded
- [x] Acceptance criteria reviewed after implementation
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-13 21:00 UTC - josecelano - Initial spec drafted
- 2026-07-15 00:00 UTC - josecelano - GitHub issue #1979 created; spec moved to `docs/issues/open/1979-1978-copy-configuration-schema-v2-to-v3-baseline.md`
- 2026-07-20 00:00 UTC - agent - Implementation completed: T1–T5 and T8–T9 done; T6/T7 deferred to #1980 (consumer migration must happen atomically)
- 2026-07-20 13:21 UTC - agent - Reconciled the spec after PR #1999 merged; automatic verification and acceptance review are complete, while manual scenarios and archival remain open.

## Acceptance Criteria

- [x] AC1: `packages/configuration/src/v3_0_0/` exists as an exact copy of `v2_0_0/`
- [x] AC2: `lib.rs` exposes both `v2_0_0` and `v3_0_0` modules
- [ ] AC3: Application uses `v3_0_0` by default — **DEFERRED to #1980** (requires switching bootstrap + all consumers atomically)
- [x] AC4: All existing tests pass (v2 unchanged)
- [ ] AC5: Default config files reference `schema_version = "3.0.0"` — **DEFERRED to #1980** (config files must match the active parser)
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass (48 suites, 0 failures)

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

| ID  | Scenario                      | Command/Steps                                               | Expected Result                  | Status | Evidence |
| --- | ----------------------------- | ----------------------------------------------------------- | -------------------------------- | ------ | -------- |
| M1  | Verify v3 module exists       | `ls packages/configuration/src/v3_0_0/`                     | Lists same files as `v2_0_0/`    | TODO   |          |
| M2  | Verify default config uses v3 | `cargo run -- --help` or check default config output        | Shows `schema_version = "3.0.0"` | TODO   |          |
| M3  | Verify v2 config still loads  | Run tracker with explicit `schema_version = "2.0.0"` config | Tracker starts successfully      | TODO   |          |

### Acceptance Verification

| AC ID | Status   | Evidence                                                                         |
| ----- | -------- | -------------------------------------------------------------------------------- |
| AC1   | DONE     | `packages/configuration/src/v3_0_0/` exists with all 9 files mirroring `v2_0_0/` |
| AC2   | DONE     | `lib.rs` has `pub mod v2_0_0` and `pub mod v3_0_0`                               |
| AC3   | DEFERRED | Deferred to #1980; requires switching bootstrap and all consumers atomically     |
| AC4   | DONE     | All 48 test suites pass; v2_0_0 tests unchanged                                  |
| AC5   | DEFERRED | Deferred to #1980; config files must match the parser the bootstrap uses         |

## Risks and Trade-offs

- **Dual maintenance**: Both v2 and v3 modules exist simultaneously, meaning bug fixes may need to be applied to both. Mitigation: v2 is deprecated; only critical fixes are backported.
- **Module path confusion**: Internal `crate::v2_0_0` references in copied files need updating to `crate::v3_0_0`. Mitigation: thorough search-and-replace after copy.

## References

- EPIC: Configuration Overhaul (schema v3.0.0)
- Related: `packages/configuration/src/v2_0_0/`
- Related: `packages/configuration/src/lib.rs`
