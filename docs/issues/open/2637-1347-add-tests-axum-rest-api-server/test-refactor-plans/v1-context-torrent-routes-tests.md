---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/torrent/routes.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/torrent/routes.rs
    - packages/axum-rest-api-server/src/v1/routes.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-routes-tests.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-whitelist-routes-tests.md
---

# Torrent Routes File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/torrent/routes.rs`, ledger row 14 (rank 5), the third file of S4. It
repeats the no-change decision approved for the whitelist routes ([plan](v1-context-whitelist-routes-tests.md)).

## Current State

- **Unit-only coverage:** 9 / 9 lines, 15 / 15 regions, 1 / 1 functions (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`,
  clean workspace, `--lib`), measured on this branch. The function is `add`, which only registers routes; the
  `v1/routes.rs` tests (S2) run it for every router they build.
- **Existing tests:** none in the file. The route table is asserted in `v1/routes.rs` (reviewed in the
  [`v1/routes.rs` plan](v1-routes-tests.md)): a table of every route of the four contexts that fails on `404` or
  `405`, and a test that the read routes answer `200` without persistence. The 14 torrent integration tests exercise
  each route with a real server.
- **Module-owned decision:** the registered paths and methods (`GET /torrent/{info_hash}` and `GET /torrents`) and which
  handler each one gets. The service is not optional here, so there is no capability gating (unlike auth keys and the
  whitelist). The behavior behind each route belongs to `handlers.rs` (row 12).
- **T1 hypothesis:** probably no change. This plan challenges it with mutations.

## Current Tests Review

No tests in the file. The tests that assert it are in `v1/routes.rs` and `tests/server/v1/contract/context/torrent.rs`.

## Coverage And Decision Analysis

| Group | Asserted today by | Classification | Decision |
| --- | --- | --- | --- |
| `add` and the two routes | the `v1/routes.rs` route table and the read-routes test (path and method registered, even without persistence); the torrent integration tests (each route reaches its handler with the real service) | 3A unit for the table, 3B integration for the handler wiring | No change. |

No line, region or function is uncovered, so there is nothing to classify as hard to test, owned elsewhere or merely
unselected.

### Why direct tests would not add protection

A test of `add` alone would build the same router and call the same routes as the `v1/routes.rs` table, so it would
duplicate it. Which handler serves which path is observable only through a real service, which the integration tests
have; the read-routes test of `v1/routes.rs` also kills the handler swap below.

## Named Mutations

Each is a temporary hand-made change to this file that a test must catch, applied alone and reverted exactly
(`cargo test -p torrust-tracker-axum-rest-api-server --all-features`, stable toolchain):

| ID | Site | Temporary change | Killed by |
| --- | --- | --- | --- |
| M1 | single torrent route | Path `/torrent/` to `/torrents/` (letter added). | the `v1/routes.rs` table, case `get_a_torrent` |
| M2 | list route | Path `/torrents` with a letter changed. | the `v1/routes.rs` table, case `get_the_torrents`, and the read-routes test |
| M3 | both routes | The two handlers swapped (both imports stay used). | the read-routes test of `v1/routes.rs` |
| M4 | list route | Method `GET` to `POST`. | the `v1/routes.rs` table and the read-routes test |

A first attempt at M3 pointed one route at a service that does not exist and did not compile; it was replaced by the
swap above. A change of the service state is not possible here: the state type is a required `Arc`, so the only
wrong state would be a different service, which no code path builds.

## Property-Test Decision

Not flagged and not selected: registration is a fixed table.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests in the file. | None. |
| R2 | SKIPPED | No new unit tests: covered and pinned by existing tests. | Mutations M1-M4 are killed (above). |
| R3 | DONE | Integration increment: `none`. | The 14 torrent integration tests keep protecting the wiring. |
| R4 | DONE | Record before and after. | Results below and `coverage-evidence.md`. |

## Prose-First AAA Review

No test is added or changed, so there is nothing to review.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): 9 / 9 lines, 15 / 15 regions
  and 1 / 1 functions before and after (no change to the file).
- **Mutations:** M1-M4 were each applied alone and reverted exactly (the working tree was clean afterwards); every one
  made at least one existing test fail on an assertion, not on a compile error.
- **Decision:** no change, recorded as documentation. The maintainer approved the no-change decision on 2026-11-04.
