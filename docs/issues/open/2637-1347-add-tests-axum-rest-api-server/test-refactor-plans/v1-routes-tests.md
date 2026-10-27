---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/routes.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/routes.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
---

# v1 Route Composition File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/axum-rest-api-server/src/v1/routes.rs`,
ledger row 2 (rank 2), the first file of S2.

## Current State

- **Unit-only coverage:** 27 / 45 lines (60.00%), 51 / 73 regions, 1 / 3 functions
  ([coverage evidence](../coverage-evidence.md#slice-s1-measurement); the package is identical to the S1 head).
  The covered lines run through the server and test-environment unit tests, which start the real router with the
  default (public, not listed) configuration, so only the "capability off" outcome runs. No test is in the file.
- **Existing tests:** none in the file. The 58 integration tests exercise every endpoint over HTTP and are kept.
- **Module-owned decision:** `add` registers the four contexts on one router. Stats and torrent services are
  always present. The auth-key service exists only when `core.private` is set **and** persistence is composed;
  the whitelist service only when `core.listed` is set **and** persistence is composed. The auth-key and
  whitelist routes are registered in every case (see the capability-gating wording in the spec); an absent
  service makes their handlers answer `409`.
- **T1 hypothesis:** test candidate.

## Current Tests Review

No tests in the file, so R1 is skipped.

## Coverage And Decision Analysis

| Behavior group (lines at the S1 head) | Classification | Decision |
| --- | --- | --- |
| Auth-key service built in private mode with persistence (26-33) | 3A unit | R2: call `add` and send one request to a key route for each state of (`private`, persistence). |
| Whitelist service built in listed mode with persistence (56-63) | 3A unit | R2: the same for (`listed`, persistence). |
| Stats and torrent services are always present | 3A unit | R2: with the least capable composition (public, no persistence) both read endpoints answer `200`. |
| Every route of the four contexts is registered | 3A unit | R2: a table of method and path pairs; a registered route never answers `404`. This also pins the paths of the four `*/routes.rs` files (ledger rows 8, 11, 14, 17). |
| What the services do (key validity, persistence, DTOs) | Collaborator-owned | `rest-api-application`, `rest-api-runtime-adapter` and `tracker-core`; asserted only as "not the disabled `409`". |

### Boundary And Arrange

The test calls `v1::routes::add("/api", Router::new(), &container)` and sends requests with
`tower::ServiceExt::oneshot`. It does **not** go through `routes::router`, so the authentication layer, request
id, compression and timeout (row 3, `routes.rs`) are not involved. The container is built by the package's
`TrackerHttpApiCoreContainer::initialize` from a `torrust-tracker-test-helpers` configuration whose `core.private`,
`core.listed` and `core.database` are set by the test. `ephemeral()` already gives each call its own temporary
SQLite file, and `core.database = None` composes no persistence.

### Cases For R2 (literal expectations)

| State | Request | Expected |
| --- | --- | --- |
| `private`, persistence | `POST /api/v1/key/60` | `200` (a generated key; only the status is asserted) |
| not `private`, persistence | `POST /api/v1/key/60` | `409`, `application/json`, `{"status":"err","reason":"private capability is disabled by configuration"}` |
| `private`, no persistence | `POST /api/v1/key/60` | the same `409` |
| `listed`, persistence | `GET /api/v1/whitelist/reload` | `200` |
| not `listed`, persistence | `GET /api/v1/whitelist/reload` | `409`, `{"status":"err","reason":"listed capability is disabled by configuration"}` |
| `listed`, no persistence | `GET /api/v1/whitelist/reload` | the same `409` |
| public, no persistence | `GET /api/v1/stats` and `GET /api/v1/torrents` | `200` |

Route registration table (the least capable composition, so a gated route answers `409` and an open one `200` or
a handler error, never `404`): `POST /key/60`, `DELETE /key/abc`, `POST /keys` (empty body), `GET /keys/reload`,
`POST` and `DELETE /whitelist/<40 hex>`, `GET /whitelist/reload`, `GET /stats`, `GET /metrics`,
`GET /torrents`, `GET /torrent/<40 hex>`. Expected: status is not `404` and not `405`.

## Property-Test Decision

Not flagged and not selected: the outcome is a small enumerable policy (two booleans and a persistence flag),
better served by an `rstest` table.

## Dependencies

None new. `rstest` and `tower` (`util`) exist; `torrust-tracker-test-helpers` is already a dev-dependency.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests. | None. |
| R2 | TODO | Add the in-file test module with the gating and registration tables and a helper that builds the container. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | TODO | Integration increment: `none` expected. | The 58 integration tests keep the wire contract. |
| R4 | TODO | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Named Mutations

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| G1 | auth-key gate | Remove the `private` condition (always build the service when persistence exists). | "not `private`, persistence" |
| G2 | auth-key gate | Make the `private` branch return `None` (the service is never built). | "`private`, persistence" |
| G3 | whitelist gate | Swap `listed` for `private`. | the two `listed` cases |
| G4 | whitelist gate | Always `None`. | "`listed`, persistence" |
| G5 | auth-key routes | Do not register the auth-key routes when the service is absent. | the "no persistence" `409` cases and the route table |
| G6 | stats service | Skip `stats::routes::add`. | the `stats` cases and the route table |

## Maintainer Questions

1. **Shared test container helper.** Row 3 (`routes.rs`) and later `server.rs` (S5) need the same container
   builder, and `server.rs` already has a private copy inside its test module. Proposal: add one
   `#[cfg(test)] pub(crate)` module in this package (for example `src/test_support.rs`) with
   `api_container(&Configuration) -> Arc<TrackerHttpApiCoreContainer>`, used from here, row 3 and S5. It does
   not depend on `testing::environment` (which #2482 gates). Alternative: a private helper in each test module.
   Which do you prefer?
2. **Persistence "none" case.** It relies on `TrackerCoreContainer::initialize_from` accepting
   `core.database = None` with `private` or `listed` set. If a configuration rule forbids that combination in
   production, the case still protects the route's behavior, because `add` only reads the composed
   persistence. Confirm the case stays.

## Prose-First AAA Review

To be written for every added test before the completed-file review.

## Results

Pending.
