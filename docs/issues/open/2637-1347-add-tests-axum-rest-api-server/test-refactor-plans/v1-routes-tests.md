---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/routes.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/routes.rs
    - src/bootstrap/persistence.rs
    - src/bootstrap/app.rs
    - docs/adrs/20260825193119_make_persistence_an_optional_application_composition_capability.md
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
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

### Two Lines Of Defense

The "private or listed set but no persistence" state is guarded twice, and this plan links both:

- **First line (startup, owned by the root crate):** `validate_persistence_requirements` in
  `src/bootstrap/persistence.rs` rejects `core.listed`, `core.private` and persistent completed statistics
  without `[core.database]`, and `setup` in `src/bootstrap/app.rs` calls it before any container is built
  (ADR `20260825193119`, agreement 3). Its own tests are in those two files.
- **Second line (this file):** `v1::routes::add` receives a container whose `core_config.private` and
  `persistence` are independent fields, so the combination is representable. The handler treats it as
  "capability off" and answers `409` instead of building a service or panicking. The "no persistence" cases below
  protect that second line. They are defensive: a real tracker never reaches them, because the first line stops
  startup. The decision to keep them is the maintainer's (2026-10-28): when the contract of a layer allows a state,
  the layer's defense is tested even if an earlier layer validates.

Whether the state could be made unrepresentable in the types is a design question outside this issue; it is
recorded in `lessons.md`.

### Boundary And Arrange

The test calls `v1::routes::add("/api", Router::new(), &container)` and sends requests with
`tower::ServiceExt::oneshot`. It does **not** go through `routes::router`, so the authentication layer, request
id, compression and timeout (row 3, `routes.rs`) are not involved. The container is built by the package's
`TrackerHttpApiCoreContainer::initialize` (through the crate-private helper below) from a `torrust-tracker-test-helpers` configuration whose `core.private`,
`core.listed` and `core.database` are set by the test. `ephemeral()` already gives each call its own temporary
SQLite file, and `core.database = None` composes no persistence.

### Cases For R2 (literal expectations)

| State | Request | Expected |
| --- | --- | --- |
| `private`, persistence | `POST /api/v1/key/60` | `200` (a generated key; only the status is asserted) |
| not `private`, persistence | `POST /api/v1/key/60` | `409`, `application/json`, `{"status":"err","reason":"private capability is disabled by configuration"}` |
| `private`, no persistence (second line) | `POST /api/v1/key/60` | the same `409` |
| `listed`, persistence | `GET /api/v1/whitelist/reload` | `200` |
| not `listed`, persistence | `GET /api/v1/whitelist/reload` | `409`, `{"status":"err","reason":"listed capability is disabled by configuration"}` |
| `listed`, no persistence (second line) | `GET /api/v1/whitelist/reload` | the same `409` |
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
| R2 | DONE | Add `src/test_support.rs` (the crate-private container helper) and the in-file test module with the gating and registration tables. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | DONE | Integration increment: `none` expected. | The 58 integration tests keep the wire contract. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Named Mutations

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| G1 | auth-key gate | Remove the `private` condition (always build the service when persistence exists). | "not `private`, persistence" |
| G2 | auth-key gate | Make the `private` branch return `None` (the service is never built). | "`private`, persistence" |
| G3 | whitelist gate | Swap `listed` for `private`. | the two `listed` cases |
| G4 | whitelist gate | Always `None`. | "`listed`, persistence" |
| G5 | auth-key routes | Do not register the auth-key routes when the service is absent. | the "no persistence" `409` cases and the route table |
| G6 | stats service | Skip `stats::routes::add`. | the `stats` cases and the route table |

## Maintainer Decisions

Recorded on 2026-10-28.

1. **Container helper:** a private `#[cfg(test)] mod test_support` in this crate (`src/test_support.rs`), as
   rule C of ADR `20261021190318` says for a helper used only by the crate's own unit tests. It is not placed in
   `src/testing/`, which holds test support shared outside the crate (rules A and D) and is gated by #2482. It is
   reused by `routes.rs` (row 3) and by `server.rs` in S5, whose private copy it replaces then.
2. **Cases with `private` or `listed` set but no persistence:** kept, labelled as the second line of defense and
   linked to the first line (above).
3. **Plan approved** with the edits above.
4. **Helper location, confirmed 2026-10-28 after questioning the `src/testing/` convention:** the helper stays a
   private `#[cfg(test)] mod test_support` at the crate root, and the file also carries the inner attribute
   `#![cfg(test)]` so that it states it is test code. Evidence weighed: the `src/environment.rs` to
   `src/testing/environment.rs` moves were meant to mark test-only infrastructure but moved shared server
   environments; ADR `20261021190318` rule C does not say where a crate-private helper's file lives; and
   `tracker-core` keeps `src/test_helpers.rs` at the crate root with only `#[cfg(test)]` content. The
   convention is recorded in `lessons.md` for a separate ADR clarification after this issue, together with the
   maintainer's note that files containing only test code are prefixed `test_`.

## Prose-First AAA Review

Temporary prose, written before comparing with the code.

| Test (cases) | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_serve_the_auth_key_routes_when_the_tracker_is_private_and_has_persistence` | A private tracker with a database. | `POST /api/v1/key/60` through the composed routes. | `200`. | Only the status is asserted: the body is a random key owned by `tracker-core`. Kept. |
| `it_should_answer_conflict_on_the_auth_key_routes_when_the_private_capability_is_not_available` (2) | A public tracker with a database, or a private tracker without one (the second line of defense). | Same request. | `409`, `application/json`, the literal disabled body. | The two states share one outcome, so one table; the case names say which state each is. Kept. |
| `it_should_serve_the_whitelist_routes_when_the_tracker_is_listed_and_has_persistence` | A listed tracker with a database. | `GET /api/v1/whitelist/reload`. | `200`, JSON, `{"status":"ok"}`. | Kept. |
| `it_should_answer_conflict_on_the_whitelist_routes_when_the_listed_capability_is_not_available` (2) | A public tracker with a database, or a listed tracker without one (the second line of defense). | Same request. | `409`, JSON, the literal disabled body. | Kept. |
| `it_should_serve_the_read_routes_even_when_the_tracker_has_no_persistence` (2) | The least capable tracker (public, no database). | `GET` of the statistics and the torrent list. | `200`. | Status only: the bodies are owned by the stats and torrent services. Kept. |
| `it_should_register_every_route_of_the_four_contexts_even_for_the_least_capable_tracker` (11) | The least capable tracker. | One request per method and path of the four contexts. | The status is neither `404` (path not registered) nor `405` (method not registered). | Deliberately weak: it pins the route table, not the behavior of each handler, which is the job of the handler plans (S3 and S4). Kept. |

Helpers: `private_tracker`, `listed_tracker` and `without_database` name the three configuration states; `call`
builds the container, composes the routes and reads the response, and hides only mechanics. `test_support::api_container`
(in `src/test_support.rs`, rule C of ADR `20261021190318`, with an inner `#![cfg(test)]`) builds the container as the application does.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): `v1/routes.rs` 27 / 45
  lines (60.00%) before; 111 / 111 lines (100.00%), 174 / 174 regions and 13 / 13 functions after (the counts include
  the test module). The new `test_support.rs` is 31 / 31. The four context `routes.rs` files stay at 100%
  (their paths and methods are now pinned by the registration table). Package unit-only lines after this file:
  1,142 / 1,402 (81.46%), regions 1,661 / 2,142 (77.54%), functions 157 / 208 (75.48%), 65 unit tests; the
  denominator grew by the test module and the helper.
- **Tests:** 19 test cases in six tests (single tests and `rstest` tables of 2, 2, 2 and 11 cases). No
  integration test was added or changed.
- **Mutations** (each applied alone, stable toolchain, then reverted; the file was identical to the tested one
  afterwards): G1 killed by the public-tracker `409` case, G2 by the private-with-persistence test, G3 and G4 by the
  listed-with-persistence test, G5 by six cases (both auth-key `409` cases and the key rows of the registration
  table), G6 by three cases (the statistics and metrics registration rows and the statistics read case). The
  first G6 attempt left `stats` unused and did not compile under `unused = "deny"`; it was redone as a clone that
  is dropped, which compiles.
- **Validation:** `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `cargo clippy -p
  torrust-tracker-axum-rest-api-server --all-targets` (stable), `cargo machete --with-metadata`, `git diff
  --check` and `cargo test -p torrust-tracker-axum-rest-api-server` (65 unit and 58 integration tests) pass.
- **Review outcome:** the maintainer approved the completed file on 2026-10-28.
