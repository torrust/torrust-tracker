---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/auth_key/routes.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/auth_key/routes.rs
    - packages/axum-rest-api-server/src/v1/routes.rs
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-routes-tests.md
---

# Auth Key Routes File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/auth_key/routes.rs`, ledger row 8 (rank 4), the third file of S3.

## Current State

- **Unit-only coverage:** 16 / 16 lines, 27 / 27 regions, 1 / 1 functions (stable Rust 1.99.1,
  `cargo-llvm-cov 0.6.16`, clean workspace, `--lib`), measured on this branch. The function is `add`, which only
  registers routes; the `v1/routes.rs` tests (S2) run it for every router they build.
- **Existing tests:** none in the file. The route table is asserted in `v1/routes.rs` (reviewed in the
  [`v1/routes.rs` plan](v1-routes-tests.md)): one test per capability state, and a table of every route of the four
  contexts that fails on `404` or `405`. The 18 auth-key integration tests exercise each route and its method with a
  real server.
- **Module-owned decision:** the registered paths and methods (`POST /key/{seconds_valid_or_key}`,
  `DELETE /key/{seconds_valid_or_key}`, `GET /keys/reload`, `POST /keys`) and which handler and service state each
  one gets. The behavior behind each route belongs to `handlers.rs` (row 6).
- **T1 hypothesis:** probably no change. This plan challenges it with mutations.

## Current Tests Review

No tests in the file. The tests that assert it are in `v1/routes.rs` and `tests/server/v1/contract/context/auth_key.rs`.

## Coverage And Decision Analysis

| Group (lines) | Asserted today by | Classification | Decision |
| --- | --- | --- | --- |
| `add` and the four routes (16-37) | the `v1/routes.rs` route table (path and method registered, even without persistence); the `v1/routes.rs` capability tests; the auth-key integration tests (each route reaches its handler with the real service) | 3A unit for the table, 3B integration for the handler wiring | No change. |

No line, region or function is uncovered, so there is nothing to classify as hard to test, owned elsewhere or merely
unselected.

### Why direct tests would not add protection

A test of `add` alone would have to build the same router and call the same four routes as the `v1/routes.rs`
table, with the same assertions; it would duplicate that table. The state wiring (which service reaches which
handler) is only observable with a real service, which the integration tests have. The mutations below show that
each registered path, method and state is pinned by a test that exists today.

## Named Mutations

Each is a temporary hand-made change to this file that a test must catch, applied alone and reverted exactly
(`cargo test -p torrust-tracker-axum-rest-api-server --all-features`, stable toolchain):

| ID | Site | Temporary change | Killed by |
| --- | --- | --- | --- |
| M1 | generate and delete route | Path `/key/` with a letter changed. | 5 `v1/routes.rs` tests (capability, table) |
| M2 | reload route | Path `/keys/reload` with a suffix added. | the `v1/routes.rs` table, reload case |
| M3 | add route | Path `/keys` with a letter changed. | the `v1/routes.rs` table, add case |
| M4 | delete method | `.delete(...)` to `.put(...)`. | the `v1/routes.rs` table, delete case |
| M5 | add route | Service state `None`. | 4 auth-key integration tests |
| M6 | reload route | Service state `None`. | 2 auth-key integration tests (reload) |
| M7 | generate route | Service state `None`. | the `v1/routes.rs` "serve the auth key routes" test |
| M8 | delete route | Service state `None`. | 3 auth-key integration tests (delete) |

Not tried: swapping one handler for another on a route, because it leaves an import unused and the crate denies
`unused`; the add, generate and delete integration tests assert each handler's own behavior and body on its route.

## Property-Test Decision

Not flagged and not selected: registration is a fixed table.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests in the file. | None. |
| R2 | SKIPPED | No new unit tests: covered and pinned by existing tests. | Mutations M1-M8 are killed (above). |
| R3 | DONE | Integration increment: `none`. | The 18 auth-key integration tests keep protecting the wiring. |
| R4 | DONE | Record before and after. | Results below and `coverage-evidence.md`. |

## Prose-First AAA Review

No test is added or changed, so there is nothing to review.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): 16 / 16 lines, 27 / 27
  regions and 1 / 1 functions before and after (no change to the file).
- **Mutations:** M1-M8 were each applied alone and reverted exactly (the working tree was clean afterwards); every
  one made at least one existing test fail on an assertion, not on a compile error.
- **Decision:** no change, recorded as documentation. The maintainer approved the no-change decision on 2026-10-29.
