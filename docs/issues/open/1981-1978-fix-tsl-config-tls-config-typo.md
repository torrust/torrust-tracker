---
doc-type: issue
issue-type: task
status: open
priority: p1
github-issue: 1981
spec-path: docs/issues/open/1981-1978-fix-tsl-config-tls-config-typo.md
branch: "config-fix-tsl-typo"
related-pr: null
last-updated-utc: 2026-07-14 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v3_0_0/http_tracker.rs
    - packages/configuration/src/v3_0_0/tracker_api.rs
    - packages/configuration/src/v3_0_0/mod.rs
    - packages/configuration/src/lib.rs
    - packages/axum-server/src/tsl.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-http-server/src/testing/environment.rs
    - packages/axum-http-server/examples/http_only_public_tracker.rs
    - packages/axum-rest-api-server/src/lib.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-rest-api-server/src/testing/environment.rs
    - packages/test-helpers/src/configuration.rs
    - src/bootstrap/jobs/http_tracker.rs
    - src/bootstrap/jobs/tracker_apis.rs
    - docs/containers.md
    - docs/issues/open/1640-per-http-tracker-on-reverse-proxy-setting.md
---

<!-- skill-link: create-issue -->

# Issue #1981 - Fix `tsl_config` → `tls_config` typo

> **EPIC position**: Subissue #2 of 9 in EPIC #1978. Depends on #1979. Must be implemented **before #1640** (#3) to avoid merge conflicts on `http_tracker.rs`.

## Goal

Fix the pervasive typo `tsl_config` → `tls_config` across the entire codebase. This is a pre-existing typo (TLS, not TSL) that has propagated into ~13 Rust source files and ~8 documentation files. Since we are releasing config schema v3.0.0, this is the right time to fix it.

## Background

The codebase consistently uses `tsl_config` instead of `tls_config`:

```rust
// packages/configuration/src/v2_0_0/http_tracker.rs
pub tsl_config: Option<TslConfig>,

// packages/configuration/src/v2_0_0/tracker_api.rs
pub tsl_config: Option<TslConfig>,

// packages/configuration/src/lib.rs
pub struct TslConfig { ... }
```

The struct name `TslConfig` and all field names `tsl_config` should be `TlsConfig` / `tls_config`. This is a purely mechanical rename with no behavioural change.

## Scope

### In Scope

- Rename `TslConfig` → `TlsConfig` in `packages/configuration/src/lib.rs`
- Rename `tsl_config` → `tls_config` in all config struct fields (`HttpTracker`, `HttpApi`)
- Rename `tsl_config` → `tls_config` in all consumers (~13 Rust files)
- Rename `tsl_config` → `tls_config` in all documentation (~8 markdown files)
- Rename `packages/axum-server/src/tsl.rs` → `packages/axum-server/src/tls.rs`
- Update all `use` imports referencing the old module path

### Out of Scope

- Any functional changes to TLS configuration
- Changing the TLS implementation itself

## Implementation Plan

| ID  | Status | Task                                      | Notes                                                                     |
| --- | ------ | ----------------------------------------- | ------------------------------------------------------------------------- |
| T1  | TODO   | Rename `TslConfig` → `TlsConfig` struct   | In `packages/configuration/src/lib.rs`                                    |
| T2  | TODO   | Rename `tsl_config` → `tls_config` fields | In `HttpTracker` and `HttpApi` config structs                             |
| T3  | TODO   | Rename `tsl.rs` → `tls.rs`                | In `packages/axum-server/src/`; update `mod.rs`                           |
| T4  | TODO   | Update all Rust consumers (~13 files)     | Search-and-replace `tsl_config` → `tls_config`, `TslConfig` → `TlsConfig` |
| T5  | TODO   | Update all documentation (~8 files)       | Search-and-replace in markdown files                                      |
| T6  | TODO   | Run `linter all` and full test suite      |                                                                           |

## Consumer Files

### Rust source files (~13)

| File                                                             | Change                            |
| ---------------------------------------------------------------- | --------------------------------- |
| `packages/configuration/src/lib.rs`                              | `TslConfig` → `TlsConfig`         |
| `packages/configuration/src/v3_0_0/http_tracker.rs`              | Field + default method            |
| `packages/configuration/src/v3_0_0/tracker_api.rs`               | Field + default method            |
| `packages/configuration/src/v3_0_0/mod.rs`                       | Doc comments                      |
| `packages/axum-server/src/tsl.rs` → `tls.rs`                     | File rename + function signatures |
| `packages/axum-http-server/src/server.rs`                        | Field access                      |
| `packages/axum-http-server/src/testing/environment.rs`           | Field access                      |
| `packages/axum-http-server/examples/http_only_public_tracker.rs` | Field access + comment            |
| `packages/axum-rest-api-server/src/lib.rs`                       | Doc comments                      |
| `packages/axum-rest-api-server/src/server.rs`                    | Field access                      |
| `packages/axum-rest-api-server/src/testing/environment.rs`       | Field access                      |
| `packages/test-helpers/src/configuration.rs`                     | Field access                      |
| `src/bootstrap/jobs/http_tracker.rs`                             | Field access                      |
| `src/bootstrap/jobs/tracker_apis.rs`                             | Field access                      |

### Documentation files (~8)

| File                                                                               | Change                       |
| ---------------------------------------------------------------------------------- | ---------------------------- |
| `docs/containers.md`                                                               | TOML examples                |
| `docs/issues/open/1640-per-http-tracker-on-reverse-proxy-setting.md`               | Code examples + design notes |
| `docs/issues/closed/1860-1669-evaluate-tslconfig-move-to-axum-server/ISSUE.md`     | References                   |
| `docs/issues/open/1669-overhaul-packages/DECISIONS.md`                             | References                   |
| `docs/issues/open/1669-overhaul-packages/workspace-coupling-report-*.md` (3 files) | References                   |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation
- [ ] Issue closed and spec moved to `docs/issues/open/`

### Progress Log

- 2026-07-14 00:00 UTC - josecelano - Initial spec drafted
- 2026-07-15 00:00 UTC - josecelano - GitHub issue #1981 created; spec moved to `docs/issues/open/1981-configuration-overhaul-fix-tsl-typo.md`

## Acceptance Criteria

- [ ] AC1: `TslConfig` is renamed to `TlsConfig` everywhere
- [ ] AC2: `tsl_config` is renamed to `tls_config` everywhere
- [ ] AC3: `packages/axum-server/src/tsl.rs` is renamed to `tls.rs`
- [ ] AC4: All tests pass
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`
- `rg "tsl_config\|TslConfig"` — should return zero matches

### Manual Verification Scenarios

| ID  | Scenario                     | Command/Steps                | Expected Result             | Status | Evidence |
| --- | ---------------------------- | ---------------------------- | --------------------------- | ------ | -------- |
| M1  | Verify no tsl_config remains | `rg "tsl_config\|TslConfig"` | Zero matches                | TODO   |          |
| M2  | Verify tracker starts        | `cargo run`                  | Tracker starts successfully | TODO   |          |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   |          |
| AC2   | TODO   |          |
| AC3   | TODO   |          |
| AC4   | TODO   |          |

## Risks and Trade-offs

- **Large diff**: ~21 files changed. Mitigation: all changes are mechanical search-and-replace; no behavioural change.
- **Merge conflicts with other EPIC subissues**: Other subissues modify the same files (e.g., #1640 touches `http_tracker.rs`). Mitigation: implement this subissue early (before #1640) to avoid conflicts.

## References

- EPIC: Configuration Overhaul (schema v3.0.0)
- Related: `packages/configuration/src/lib.rs` (TslConfig definition)
- Related: `packages/axum-server/src/tsl.rs`
