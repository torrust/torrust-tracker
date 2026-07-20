---
doc-type: issue
issue-type: task
status: in_progress
priority: p1
github-issue: 1981
spec-path: docs/issues/open/1981-1978-fix-tsl-config-tls-config-typo.md
branch: "1981-fix-tsl-config-typo"
related-pr: null
last-updated-utc: 2026-07-20 13:21
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v3_0_0/http_tracker.rs
    - packages/configuration/src/v3_0_0/tracker_api.rs
    - packages/configuration/src/v3_0_0/mod.rs
    - packages/configuration/src/v3_0_0/tls.rs
    - packages/axum-server/src/tls.rs
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
    - docs/issues/open/1640-1978-per-http-tracker-on-reverse-proxy-setting.md
---

# Issue #1981 - Fix `tsl_config` → `tls_config` typo

> **EPIC position**: Subissue #2 of 11 in EPIC #1978. Depends on #1979. Must be implemented **before #1640** (#3) to avoid merge conflicts on `http_tracker.rs`.

## Goal

Fix the `tsl_config` → `tls_config` typo in configuration schema v3 and in schema-neutral TLS module naming. Preserve the typo in the supported v2 compatibility contract until consumers migrate to v3 in #1980.

## Background

The active v2 schema uses `tsl_config` instead of `tls_config`:

```rust
// packages/configuration/src/v2_0_0/http_tracker.rs
pub tsl_config: Option<TslConfig>,

// packages/configuration/src/v2_0_0/tracker_api.rs
pub tsl_config: Option<TslConfig>,

// packages/configuration/src/lib.rs
pub struct TslConfig { ... }
```

The v3 struct name and fields should be `TlsConfig` / `tls_config`. The schema-neutral Axum helper module should likewise be named `tls`.

### Compatibility Boundary

Subissue #1979 established that `v2_0_0` remains available for backward compatibility while v3 evolves. On 2026-07-20, the maintainer confirmed that #1981 must preserve that contract:

- Keep `v2_0_0::HttpTracker::tsl_config`, `v2_0_0::HttpApi::tsl_config`, and the crate-root `TslConfig` unchanged.
- Add a v3-owned `TlsConfig` type and use `tls_config` only in v3 DTOs.
- Rename schema-neutral module and local identifier spellings from `tsl` to `tls` now.
- Defer active configuration consumer field migration to #1980, when the application switches atomically from v2 to v3.
- Preserve closed issue specs and dated reports as historical evidence; correct current v3 documentation and open implementation specs only.

Old spellings are therefore expected to remain under `v2_0_0`, in the crate-root v2 compatibility type, in active v2 field consumers, and in historical documentation until their owning migration or archival policy says otherwise.

## Scope

### In Scope

- Add `v3_0_0::tls::TlsConfig`
- Rename `tsl_config` → `tls_config` in v3 `HttpTracker` and `HttpApi`
- Update v3 schema documentation and tests
- Rename `packages/axum-server/src/tsl.rs` → `packages/axum-server/src/tls.rs`
- Update schema-neutral module imports and local identifiers referencing the old `tsl` spelling
- Update open EPIC implementation specs that describe the future v3 contract

### Out of Scope

- Any functional changes to TLS configuration
- Changing the TLS implementation itself
- Renaming v2 types, fields, or TOML keys
- Migrating active configuration consumers from v2 fields to v3 fields (tracked in #1980)
- Rewriting closed issue specs or dated reports
- Updating current v2 deployment examples before v3 becomes active (tracked in #1980)

## Implementation Plan

| ID  | Status | Task                                                 | Notes                                                   |
| --- | ------ | ---------------------------------------------------- | ------------------------------------------------------- |
| T1  | TODO   | Add the v3-owned `TlsConfig` struct                  | New `packages/configuration/src/v3_0_0/tls.rs`          |
| T2  | TODO   | Rename v3 `tsl_config` fields to `tls_config`        | In v3 `HttpTracker` and `HttpApi` only                  |
| T3  | TODO   | Rename schema-neutral `tsl.rs` to `tls.rs`           | Update module imports and local identifiers             |
| T4  | TODO   | Update v3 docs, open implementation specs, and tests | Preserve v2 and historical spellings intentionally      |
| T5  | TODO   | Record remaining old spellings by ownership          | Verify each belongs to v2, #1980, or historical records |
| T6  | TODO   | Run `linter all` and full test suite                 |                                                         |

## Implementation Files

### Rust source files

| File                                                  | Change                                           |
| ----------------------------------------------------- | ------------------------------------------------ |
| `packages/configuration/src/v3_0_0/tls.rs`            | Add v3 `TlsConfig`                               |
| `packages/configuration/src/v3_0_0/http_tracker.rs`   | Rename field, default method, type import        |
| `packages/configuration/src/v3_0_0/tracker_api.rs`    | Rename field, default method, type import        |
| `packages/configuration/src/v3_0_0/mod.rs`            | Export module and correct v3 docs                |
| `packages/axum-server/src/tsl.rs` → `tls.rs`          | Rename schema-neutral module and local variables |
| Current imports of `torrust_tracker_axum_server::tsl` | Update module path to `tls`                      |

### Documentation files

| File                                                                      | Change                                    |
| ------------------------------------------------------------------------- | ----------------------------------------- |
| `packages/configuration/src/v3_0_0/mod.rs`                                | Correct v3 schema examples and prose      |
| `docs/issues/open/1640-1978-per-http-tracker-on-reverse-proxy-setting.md` | Correct future v3 field/type references   |
| `docs/issues/open/1978-configuration-overhaul-epic.md`                    | Track progress and compatibility boundary |

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
- 2026-07-15 00:00 UTC - josecelano - GitHub issue #1981 created; spec moved to `docs/issues/open/1981-1978-fix-tsl-config-tls-config-typo.md`
- 2026-07-20 13:21 UTC - josecelano/agent - Started implementation on branch `1981-fix-tsl-config-typo`; maintainer chose to preserve v2 and historical artifacts, apply the rename to v3 and schema-neutral naming, and defer active field migration to #1980.

## Acceptance Criteria

- [ ] AC1: Schema v3 exposes `TlsConfig` and no v3 Rust/TOML identifier uses the `tsl` typo
- [ ] AC2: Schema v2 public types, fields, and TOML keys remain unchanged
- [ ] AC3: `packages/axum-server/src/tsl.rs` is renamed to `tls.rs`, including imports and local identifiers
- [ ] AC4: Remaining old spellings are limited to v2 compatibility, active v2 field consumers awaiting #1980, and historical artifacts
- [ ] AC5: All tests pass
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`
- `rg "tsl_config|TslConfig" packages/configuration/src/v3_0_0 packages/axum-server/src` — should return zero matches
- `rg -w "tsl" packages/configuration/src/v3_0_0 packages/axum-server/src` — should return zero matches
- Review repository-wide old-spelling matches and classify each under the approved compatibility boundary

### Manual Verification Scenarios

| ID  | Scenario                           | Command/Steps                               | Expected Result                       | Status | Evidence |
| --- | ---------------------------------- | ------------------------------------------- | ------------------------------------- | ------ | -------- |
| M1  | Verify v3 corrected names          | Search v3 and Axum TLS module for old names | No old spelling remains in that scope | TODO   |          |
| M2  | Verify v2 compatibility            | Run v2 configuration tests                  | Existing v2 TOML still deserializes   | TODO   |          |
| M3  | Verify v3 TLS TOML deserialization | Deserialize v3 `tls_config` examples        | v3 TLS values deserialize correctly   | TODO   |          |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   |          |
| AC2   | TODO   |          |
| AC3   | TODO   |          |
| AC4   | TODO   |          |
| AC5   | TODO   |          |

## Risks and Trade-offs

- **Split migration vocabulary**: old and corrected names coexist temporarily. Mitigation: confine old names to the documented v2, active-consumer, and historical boundaries; #1980 removes active v2 usage.
- **Merge conflicts with other EPIC subissues**: Other subissues modify the same files (e.g., #1640 touches `http_tracker.rs`). Mitigation: implement this subissue early (before #1640) to avoid conflicts.

## References

- EPIC: Configuration Overhaul (schema v3.0.0)
- Related: `packages/configuration/src/lib.rs` (TslConfig definition)
- Related: `packages/axum-server/src/tsl.rs`
