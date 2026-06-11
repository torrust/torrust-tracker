---
doc-type: spec
issue-type: task
status: draft
priority: p2
epic: 1669
spec-path: docs/issues/drafts/1669-relocate-udp-server-test-environment.md
last-updated-utc: 2026-06-11
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-06-10.md
---

# Relocate `udp-server` Test Environment Infrastructure

## Subissue of EPIC #1669 — Overhaul: Packages

**Part of the test environment relocation series:**

1. [1669-relocate-rest-api-core-from-udp-internals.md](./1669-relocate-rest-api-core-from-udp-internals.md) (prerequisite)
2. [1669-relocate-axum-rest-api-server-test-environment.md](./1669-relocate-axum-rest-api-server-test-environment.md)
3. **This subissue**
4. [1669-relocate-http-server-test-environment.md](./1669-relocate-http-server-test-environment.md)

## Problem

Same pattern as `axum-rest-api-server`: `packages/udp-server/src/environment.rs`
lives in production code but is only consumed by tests and examples:

- `packages/udp-server/tests/`
- `packages/udp-server/examples/`
- `packages/axum-health-check-api-server/tests/`

It depends on the full tracker stack for test convenience, forcing unnecessary
runtime dependencies.

## Scope

### 1. Relocate `environment.rs`

**Option A** (recommended): Move to `src/testing/environment.rs`.
**Option B**: Move to `tests/common/`. Not importable by external packages.

### 2. Update consumers

- `packages/axum-health-check-api-server/tests/` — update import paths
- `packages/udp-server/examples/` — update import paths

### 3. Clean up

- Run `cargo machete`
- Update `Cargo.toml` if any deps can be demoted
- Verify `linter all` and `cargo test --workspace`

## Acceptance Criteria

1. `udp-server/src/environment.rs` no longer exists (moved to `src/testing/`).
2. `cargo test --workspace` passes.
3. `cargo machete` passes.
4. `linter all` passes.

## Verification

- [ ] `environment.rs` moved to `src/testing/environment.rs`
- [ ] External consumers updated
- [ ] `cargo test --workspace` — pass
- [ ] `cargo machete` — pass
- [ ] `linter all` — pass
