---
doc-type: issue
issue-type: enhancement
status: open
priority: p2
github-issue: 889
spec-path: docs/issues/open/889-1978-new-config-option-for-logging-style.md
branch: "889-logging-style"
related-pr: null
last-updated-utc: 2026-07-13 21:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v3_0_0/
    - packages/configuration/src/v3_0_0/logging.rs
    - packages/configuration/src/logging.rs
    - src/bootstrap/
---

# Issue #889 - New config option for logging style

> **EPIC position**: Subissue #8 of 9. Independent — only modifies `Logging` struct. Can run in parallel with #1415, #1453, #1490.

## Goal

Make the tracing logging style configurable from the configuration file. Replace the hardcoded `TraceStyle::Default` with a user-selectable option, and rename `threshold` to `trace_filter` for clarity and consistency with `tracing` crate terminology.

`trace_filter` retains the existing level-only `Threshold` scope. Supporting full `tracing` filter directives (for example, per-module levels) is a separate, more complex feature and is out of scope for this issue.

## Background

After migrating from `log` to the `tracing` crate (PR #888), the codebase supports multiple tracing output styles via the `TraceStyle` enum:

```rust
#[derive(Debug)]
pub enum TraceStyle {
    Default,
    Pretty(bool),
    Compact,
    Json,
}
```

However, the style is currently hardcoded to `TraceStyle::Default`. Users cannot change it without modifying the source code.

### TraceStyle enum redesign

The current `TraceStyle` enum has two problems:

1. **`Default` is a concrete style, not a sentinel** — it's the standard human-readable format. Renamed to `Full` for clarity.
2. **`Pretty(bool)` carries a boolean** — the bool controls `display_filename` (whether file paths appear in log output). This is a cross-cutting option that applies to all styles, not just Pretty. Dropped the boolean; `display_filename` defaults to `false` (no file paths). Can be added as a separate `[logging]` field later if users request it.

New enum:

```rust
pub enum TraceStyle {
    Full,     // was Default — standard human-readable output (default)
    Pretty,   // was Pretty(false) — pretty-printed with colours
    Compact,  // compact single-line output
    Json,     // structured JSON output
}
```

### Architecture note: `logging.rs` location

Currently, the `TraceStyle` enum and `setup()`/`tracing_init()` functions live in `packages/configuration/src/logging.rs` (crate root), while the `Logging` struct and `Threshold` enum live in `packages/configuration/src/v2_0_0/logging.rs`. The crate-root code depends on versioned types via global re-exports (`pub type Logging = v2_0_0::logging::Logging`).

As part of this EPIC, each versioned module (`v2_0_0/`, `v3_0_0/`) will become **fully self-contained** — data types + behaviour. The crate-root `logging.rs` will be copied into both `v2_0_0/` and `v3_0_0/`, and the global re-exports will be removed. This is handled by subissue #1 (copy baseline) and the caller-migration subissue.

This subissue (#889) only modifies the **v3** copy of `logging.rs`.

### Proposed config changes

**Current config:**

```toml
[logging]
threshold = "info"
```

**New config:**

```toml
[logging]
trace_filter = "info"
trace_style = "full"
```

Where `trace_style` accepts one of:

| Value       | TraceStyle variant | Description                                  |
| ----------- | ------------------ | -------------------------------------------- |
| `"full"`    | `Full`             | Standard human-readable output (default)     |
| `"pretty"`  | `Pretty`           | Pretty-printed with colours                  |
| `"compact"` | `Compact`          | Compact single-line output                   |
| `"json"`    | `Json`             | Structured JSON output (for log aggregation) |

All four variants are simple unit variants — no boolean parameters. The `display_filename` option (previously the `Pretty(bool)` parameter) is dropped; it defaults to `false` and can be added as a separate `[logging]` field later if users request it.

## Scope

### In Scope

- Rename `threshold` → `trace_filter` in the `[logging]` config section
- Retain the existing level-only values for `trace_filter` through the `Threshold` enum
- Redesign `TraceStyle` enum: rename `Default` → `Full`, drop `Pretty(bool)` → `Pretty` (unit variant)
- Add `trace_style` field to the `[logging]` config section
- Wire the config value into the tracing subscriber initialization
- Update v3 generated default configuration
- Support all four `TraceStyle` variants

### Out of Scope

- Adding more tracing configuration options (e.g. per-module filter levels, `display_filename`)
- Supporting full `tracing` filter directives such as `info,torrust_tracker=debug`
- Auto-detection of terminal colour support (can be added later)

## Implementation Plan

| ID  | Status | Task                                                                       | Notes                                                                                                                           |
| --- | ------ | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| T0  | DONE   | Copy `packages/configuration/src/logging.rs` into `v3_0_0/`                | v3 logging module is self-contained with data types and behaviour                                                               |
| T1  | DONE   | Rename `threshold` → `trace_filter` in `Logging` config struct             | Implemented in `packages/configuration/src/v3_0_0/logging.rs`                                                                   |
| T2  | DONE   | Redesign `TraceStyle` enum: `Default`→`Full`, drop `Pretty(bool)`→`Pretty` | Four unit variants; no boolean parameters                                                                                       |
| T3  | DONE   | Add `trace_style: TraceStyle` field to `Logging` config struct             | Defaults to `TraceStyle::Full`                                                                                                  |
| T4  | DONE   | Implement deserialization for `TraceStyle`                                 | Supports `"full"`, `"pretty"`, `"compact"`, and `"json"`                                                                        |
| T5  | DONE   | Wire `trace_style` into tracing subscriber initialization                  | Implemented in `v3_0_0/logging.rs` `setup()`                                                                                    |
| T6  | DONE   | Update v3 generated default configuration                                  | Uses `trace_filter` and `trace_style`; shipped v2 defaults are deferred to #1980 because v3 is not yet the active global schema |
| T7  | DONE   | Run `linter all` and tests                                                 | `linter all` and the configuration crate test suite pass                                                                        |
| T8  | DONE   | Add negative test: v3 `Logging` rejects the removed `threshold` key        | Ensures the breaking rename is guarded by `#[serde(deny_unknown_fields)]`                                                       |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation
- [ ] Issue closed and spec moved to `docs/issues/open/`

### Progress Log

- 2026-07-13 21:00 UTC - josecelano - Initial spec drafted
- 2026-07-14 00:00 UTC - josecelano - Fixed field name: `log_level` → `threshold` (the field was renamed from `log_level` to `threshold` in commit 287e4842; the GitHub issue #889 description is outdated)
- 2026-07-14 00:00 UTC - josecelano - Redesigned `TraceStyle` enum: renamed `Default` → `Full`, dropped `Pretty(bool)` → `Pretty` (unit variant). The `display_filename` boolean is dropped (defaults to `false`); can be added as a separate config field later.
- 2026-07-28 00:00 UTC - josecelano - Confirmed that `trace_filter` retains the current level-only `Threshold` scope. Full tracing directives and per-module filtering are deferred to a separate feature.
- 2026-07-28 00:00 UTC - josecelano - Implemented and automatically verified the v3-only logging schema. Migration of global callers and shipped v2 defaults remains deferred to #1980.

## Acceptance Criteria

- [x] AC1: `threshold` is renamed to `trace_filter` in the config
- [x] AC2: New `trace_style` field is configurable with values `"full"`, `"pretty"`, `"compact"`, `"json"`
- [x] AC3: Default `trace_style` is `"full"` (backward-compatible behaviour)
- [x] AC4: Tracing subscriber uses the configured style
- [x] AC5: The v3 generated default configuration is updated; shipped v2 defaults are deferred to #1980 because v3 is not yet the active global schema
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

| ID  | Scenario                    | Command/Steps                               | Expected Result                 | Status | Evidence |
| --- | --------------------------- | ------------------------------------------- | ------------------------------- | ------ | -------- |
| M1  | Verify default style        | Run tracker without `trace_style` in config | Uses `"full"` style             | TODO   |          |
| M2  | Verify JSON style           | Set `trace_style = "json"`, run tracker     | Output is JSON-formatted        | TODO   |          |
| M3  | Verify compact style        | Set `trace_style = "compact"`, run tracker  | Output is compact single-line   | TODO   |          |
| M4  | Verify pretty style         | Set `trace_style = "pretty"`, run tracker   | Output is pretty-printed        | TODO   |          |
| M5  | Verify `trace_filter` works | Set `trace_filter = "warn"`, run tracker    | Only warn+ level messages shown | TODO   |          |

### Acceptance Verification

| AC ID | Status | Evidence                                                                                    |
| ----- | ------ | ------------------------------------------------------------------------------------------- |
| AC1   | PASS   | v3 `Logging` uses `trace_filter`; mandatory-option validation and fixtures use the new key. |
| AC2   | PASS   | Unit tests deserialize each supported lower-case trace style.                               |
| AC3   | PASS   | `Logging::default()` and generated TOML set `trace_style = "full"`.                         |
| AC4   | PASS   | `setup()` passes the configured style to subscriber initialization.                         |
| AC5   | PASS   | v3 generated default configuration contains the renamed filter and style.                   |

## Risks and Trade-offs

- **Breaking change**: Renaming `threshold` to `trace_filter` breaks existing configs. Mitigation: part of the v3.0.0 schema bump where breaking changes are expected.
- **`TraceStyle` enum redesign**: Renaming `Default` → `Full` and dropping `Pretty(bool)` → `Pretty` is a breaking change for any code that constructs `TraceStyle` values directly. Mitigation: the enum is internal to the configuration crate; external consumers use the TOML string values which remain stable (`"full"`, `"pretty"`, `"compact"`, `"json"`).
- **Full tracing directives deferred**: Keeping `trace_filter` as `Threshold` avoids combining a schema rename with the design, validation, and documentation required for per-module tracing filters.

## References

- Related issues: #878 (comment)
- Related PRs: #888 (log to tracing migration), #896 (enable colour in console output)
- Related: `packages/configuration/src/v2_0_0/logging.rs`
