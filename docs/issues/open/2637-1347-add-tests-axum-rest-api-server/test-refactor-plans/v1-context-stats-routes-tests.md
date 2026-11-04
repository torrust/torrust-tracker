---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/stats/routes.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/stats/routes.rs
    - packages/axum-rest-api-server/src/v1/routes.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-routes-tests.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-torrent-routes-tests.md
---

# Stats Routes File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/stats/routes.rs`, ledger row 17 (rank 5), the last file of S4. It repeats
the no-change decision approved for the torrent routes ([plan](v1-context-torrent-routes-tests.md)).

## Current State

- **Unit-only coverage:** 9 / 9 lines, 15 / 15 regions, 1 / 1 functions (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`,
  clean workspace, `--lib`), measured on this branch. The function is `add`, which only registers routes; the
  `v1/routes.rs` tests (S2) run it for every router they build.
- **Existing tests:** none in the file. The route table is asserted in `v1/routes.rs` (reviewed in the
  [`v1/routes.rs` plan](v1-routes-tests.md)): a table of every route of the four contexts that fails on `404` or `405`,
  and a test that the read routes answer `200` without persistence. The four stats integration tests exercise both
  routes with a real server.
- **Module-owned decision:** the registered paths and methods (`GET /stats` and `GET /metrics`) and which handler each
  one gets. The service is not optional here, so there is no capability gating. The behavior behind each route belongs
  to `handlers.rs` (row 15).
- **T1 hypothesis:** probably no change. This plan challenges it with mutations.

## Current Tests Review

No tests in the file. The tests that assert it are in `v1/routes.rs` and `tests/server/v1/contract/context/stats.rs`.

## Coverage And Decision Analysis

| Group | Asserted today by | Classification | Decision |
| --- | --- | --- | --- |
| `add` and the two routes | the `v1/routes.rs` route table and read-routes test (path and method registered, even without persistence); the stats integration tests (each route reaches its own handler with the real service) | 3A unit for the table, 3B integration for the handler wiring | No change. |

No line, region or function is uncovered, so there is nothing to classify as hard to test, owned elsewhere or merely
unselected.

### Why direct tests would not add protection

A test of `add` alone would build the same router and call the same routes as the `v1/routes.rs` table, so it would
duplicate it. Which handler serves which path is observable only with a real service that answers differently for each
route, which the integration tests have (the JSON statistics and the labeled Prometheus metrics). A unit test of that
swap would need the fake service of row 15 plus a router, which is a new integration-style test and not a gap of this
file.

## Named Mutations

Each is a temporary hand-made change to this file that a test must catch, applied alone and reverted exactly
(`cargo test -p torrust-tracker-axum-rest-api-server --all-features`, stable toolchain):

| ID | Site | Temporary change | Killed by |
| --- | --- | --- | --- |
| M1 | statistics route | Path `/stats` with a letter changed. | the `v1/routes.rs` table (case `get_the_statistics`), the read-routes test and `routes.rs` tests |
| M2 | metrics route | Path `/metrics` with a letter changed. | the `v1/routes.rs` table, case `get_the_metrics` |
| M3 | both routes | The two handlers swapped (both imports stay used). | **only the stats integration tests**: the unit tests check "not `404` or `405`" and cannot tell the handlers apart |
| M4 | metrics route | Method `GET` to `POST`. | the `v1/routes.rs` table, case `get_the_metrics` |

A first attempt at M3 that replaced one handler by the other dropped the last use of an import and did not compile; the
swap above replaced it. This is the same boundary as in the whitelist and torrent routes: the unit table pins path and
method, the integration tests pin which handler answers.

## Property-Test Decision

Not flagged and not selected: registration is a fixed table.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests in the file. | None. |
| R2 | SKIPPED | No new unit tests: covered and pinned by existing tests. | Mutations M1-M4 are killed (above). |
| R3 | DONE | Integration increment: `none`. | The four stats integration tests keep protecting the handler wiring. |
| R4 | DONE | Record before and after. | Results below and `coverage-evidence.md`. |

## Prose-First AAA Review

No test is added or changed, so there is nothing to review.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): 9 / 9 lines, 15 / 15 regions
  and 1 / 1 functions before and after (no change to the file).
- **Mutations:** M1-M4 were each applied alone and reverted exactly (the working tree was clean afterwards); every one
  made at least one existing test fail on an assertion, not on a compile error. M3 is killed only at integration level.
- **Decision:** no change, recorded as documentation. The maintainer approved the no-change decision on 2026-11-04.
