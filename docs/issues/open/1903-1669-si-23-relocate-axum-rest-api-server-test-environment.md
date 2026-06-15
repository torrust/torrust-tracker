---
doc-type: issue
issue-type: task
status: completed
priority: p2
epic: 1669
github-issue: 1903
spec-path: docs/issues/open/1903-1669-si-23-relocate-axum-rest-api-server-test-environment.md
branch: 1903-relocate-axum-rest-api-server-test-environment
related-pr: null
last-updated-utc: 2026-06-15
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-06-10.md
    - docs/issues/drafts/1669-decouple-rest-api-core-from-udp-internals.md
---

<!-- skill-link: create-issue -->

# Issue #1903 (SI-23) - Relocate `axum-rest-api-server` Test Environment Infrastructure

## Subissue of EPIC #1669 — Overhaul: Packages

**Part of the test environment relocation series:**

1. [1669-relocate-rest-api-core-from-udp-internals.md](./1669-relocate-rest-api-core-from-udp-internals.md) (production decoupling — prerequisite)
2. **This subissue** (test env relocation)
3. [1669-relocate-udp-server-test-environment.md](./1669-relocate-udp-server-test-environment.md)
4. [1669-relocate-http-server-test-environment.md](./1669-relocate-http-server-test-environment.md)

## Problem

`packages/axum-rest-api-server/src/environment.rs` lives in **production code** (`src/`)
but is only used by **test code**:

- Internal tests: `packages/axum-rest-api-server/tests/`
- External tests: `packages/axum-health-check-api-server/tests/`

The module depends on `UdpTrackerServerContainer`, `UdpTrackerCoreContainer`,
`initialize_static()`, and `BanService` — all purely for test convenience.
Despite being in `src/`, it is never used in production startup (the root
`src/container.rs` does its own wiring directly). It forces runtime dependencies
on both UDP packages solely for test infrastructure.

## Scope

### 1. Relocate `environment.rs` to a proper test location

Two options:

- **Option A**: Move to `packages/axum-rest-api-server/src/testing/environment.rs`
  (a `src/testing` module). This keeps it importable by external packages like
  `axum-health-check-api-server` while clearly marking it as test-only.
- **Option B**: Move to `packages/axum-rest-api-server/tests/common/`. Not importable
  by external packages — consumers would need to duplicate the setup logic.

Recommended: **Option A**, consistent with packages that already use this pattern
(e.g. `tracker-core/src/test_helpers.rs`).

### 2. Update `Cargo.toml`

Move `udp-server` and `udp-tracker-core` from runtime dependencies to
dev-dependencies. They are only needed by the relocated test infrastructure.

### 3. Update external consumers

Update import paths in packages that use `Started` from the current location:

- `packages/axum-health-check-api-server/tests/`

### 4. Clean up

- Run `cargo machete` to verify no unused deps
- Update `Cargo.toml` files
- Verify `linter all` and `cargo test --workspace`

## Acceptance Criteria

1. `axum-rest-api-server/Cargo.toml` has no `udp-server` or `udp-tracker-core` **runtime** dependency.
2. `axum-rest-api-server/src/environment.rs` no longer exists (moved to `src/testing/`).
3. `cargo test --workspace` passes.
4. `cargo machete` passes.
5. `linter all` passes.

## Out of Scope

- Decoupling `rest-api-core` from concrete UDP types (separate subissue — prerequisite).
- Moving other server packages' test environments (separate subissues in the series).
- Changing the main tracker orchestrator (`src/container.rs`).

## Verification

- [x] DEC-13 added to `docs/issues/open/1669-overhaul-packages/DECISIONS.md`
- [x] `environment.rs` moved to `src/testing/environment.rs`
- [ ] ~`axum-rest-api-server/Cargo.toml`: UDP deps demoted to dev-dependencies~ — **Blocked**: production handlers in `src/v1/context/stats/handlers.rs` still reference `BanService` and UDP stats repository types directly. Full demotion requires prerequisite decoupling in `rest-api-core` (separate subissue).
- [x] External consumers updated (`axum-health-check-api-server`)
- [x] `cargo test --workspace` — pass
- [x] `cargo machete` — pass
- [ ] `linter all` — pass (pending full CI pipeline)
