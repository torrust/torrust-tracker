---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/whitelist/routes.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/whitelist/routes.rs
    - packages/axum-rest-api-server/src/v1/routes.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-routes-tests.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-auth-key-routes-tests.md
---

# Whitelist Routes File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/whitelist/routes.rs`, ledger row 11 (rank 4), the last file of S3. It
repeats the no-change decision the maintainer approved for the auth-key routes
([plan](v1-context-auth-key-routes-tests.md)).

## Current State

- **Unit-only coverage:** 14 / 14 lines, 25 / 25 regions, 1 / 1 functions (stable Rust 1.99.1,
  `cargo-llvm-cov 0.6.16`, clean workspace, `--lib`), measured on this branch. The function is `add`, which only
  registers routes; the `v1/routes.rs` tests (S2) run it for every router they build.
- **Existing tests:** none in the file. The route table is asserted in `v1/routes.rs` (reviewed in the
  [`v1/routes.rs` plan](v1-routes-tests.md)): one test per capability state, and a table of every route of the four
  contexts that fails on `404` or `405`. The 13 whitelist integration tests exercise each route and its method with a
  real server.
- **Module-owned decision:** the registered paths and methods (`POST /whitelist/{info_hash}`,
  `DELETE /whitelist/{info_hash}`, `GET /whitelist/reload`) and which handler and service state each gets. The
  behavior behind each route belongs to `handlers.rs` (row 9).
- **T1 hypothesis:** probably no change. This plan challenges it with mutations.

## Current Tests Review

No tests in the file. The tests that assert it are in `v1/routes.rs` and `tests/server/v1/contract/context/whitelist.rs`.

## Coverage And Decision Analysis

| Group | Asserted today by | Classification | Decision |
| --- | --- | --- | --- |
| `add` and the three routes | the `v1/routes.rs` route table (path and method registered, even without persistence); the `v1/routes.rs` capability tests; the whitelist integration tests (each route reaches its handler with the real service) | 3A unit for the table, 3B integration for the handler wiring | No change. |

No line, region or function is uncovered, so there is nothing to classify as hard to test, owned elsewhere or merely
unselected.

### Why direct tests would not add protection

A test of `add` alone would build the same router and call the same routes as the `v1/routes.rs` table, so it would
duplicate it. The state wiring (which service reaches which handler) is only observable with a real service, which
the integration tests have.

## Named Mutations

Each is a temporary hand-made change to this file that a test must catch, applied alone and reverted exactly
(`cargo test -p torrust-tracker-axum-rest-api-server --all-features`, stable toolchain):

| ID | Site | Temporary change | Killed by |
| --- | --- | --- | --- |
| M1 | all routes | Prefix `/whitelist` with a letter added. | at least 5 `v1/routes.rs` tests (capability, table) |
| M2 | reload route | Path `/reload` with a letter added. | the `v1/routes.rs` table, reload case, and 3 more `v1/routes.rs` tests |
| M3 | add route | Method `POST` to `PUT` (the `POST` answers an empty `200`). | the whitelist integration tests of `add` |
| M4 | remove route | Method `DELETE` to `PUT` (the `DELETE` answers an empty `200`). | the whitelist integration tests of `remove` |
| M3b | add route | Method `POST` to `PUT`, with no dummy handler (the `POST` answers `405`). | the `v1/routes.rs` table, case `whitelist_a_torrent` |
| M4b | remove route | Method `DELETE` to `PUT`, with no dummy handler (the `DELETE` answers `405`). | the `v1/routes.rs` table, case `remove_a_torrent_from_the_whitelist` |
| M5 | add route | Service state `None`. | 4 whitelist integration tests (add) |
| M6 | remove route | Service state `None`. | 4 whitelist integration tests (remove) |
| M7 | reload route | Service state `None`. | the `v1/routes.rs` "serve the whitelist routes" test |

Notes: M3 and M4 keep the method registered with a dummy handler, so the table of `v1/routes.rs` (which checks only
that a route is not `404` or `405`) does not kill them; the integration tests do. M3b and M4b remove the method with
no dummy handler, so the route answers `405` and the table kills them. The independent review of S3 found M3b and
M4b, and they were reproduced afterwards. A first attempt to change a method with the unused import did not
compile (`unused = "deny"`), and a second (`DELETE` to `POST`) made the router panic on a duplicate `POST`
registration; both were replaced by the `PUT` mutants above.

## Property-Test Decision

Not flagged and not selected: registration is a fixed table.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests in the file. | None. |
| R2 | SKIPPED | No new unit tests: covered and pinned by existing tests. | Mutations M1-M7 are killed (above). |
| R3 | DONE | Integration increment: `none`. | The 13 whitelist integration tests keep protecting the wiring. |
| R4 | DONE | Record before and after. | Results below and `coverage-evidence.md`. |

## Prose-First AAA Review

No test is added or changed, so there is nothing to review.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): 14 / 14 lines, 25 / 25
  regions and 1 / 1 functions before and after (no change to the file).
- **Mutations:** M1-M7, M3b and M4b were each applied alone and reverted exactly (the working tree was clean afterwards); every
  one made at least one existing test fail on an assertion, not on a compile error.
- **Decision:** no change, recorded as documentation. The maintainer approved the no-change decision on 2026-11-03.
