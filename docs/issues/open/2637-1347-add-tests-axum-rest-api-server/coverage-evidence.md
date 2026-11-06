---
doc-type: coverage-evidence
issue: 2637
package: torrust-tracker-axum-rest-api-server
measured-commit: 0b4b2d0d3
measured-utc: 2026-10-26T17:50:00Z
---

# Axum REST API Server Coverage Evidence

This document records the clean package-source baseline and the complete module inventory for Issue #2637. The reports were collected on `develop` at `abce11ec8` (the merge of #2630), before any Issue #2637 test or production change. Raw JSON reports stay ignored local artifacts under `.tmp/axum-rest-api-coverage/`.

## Measurement Scope

`cargo-llvm-cov 0.6.16` on stable Rust 1.99.1 generated three clean reports. Every figure below is filtered to files under `packages/axum-rest-api-server/src/`. Totals include the in-file `#[cfg(test)]` modules (in `server.rs` and `v1/middlewares/auth.rs`) because unit tests live in the same files, and they include `src/testing/`, which is test-support code compiled into the library, so they are not production-only coverage. The files under `packages/axum-rest-api-server/tests/` are not counted. There are no benches or examples.

```text
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-axum-rest-api-server --all-features --json --output-path .tmp/axum-rest-api-coverage/aggregate.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-axum-rest-api-server --all-features --lib --json --output-path .tmp/axum-rest-api-coverage/unit.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-axum-rest-api-server --all-features --test integration --json --output-path .tmp/axum-rest-api-coverage/integration.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-axum-rest-api-server --all-features --lib
cargo llvm-cov report -p torrust-tracker-axum-rest-api-server --show-missing-lines
```

The aggregate run executed 6 unit tests and 58 integration tests (the `integration` binary of `tests/integration.rs`); the unit-only run executed the 6 unit tests; the integration-only run executed the 58 integration tests. The last two commands list the uncovered lines of the unit-only run.

`cargo llvm-cov` also reports the other workspace crates that the tests execute. Its unfiltered `TOTAL` line is therefore not a package figure: for this commit it reads lines 15.32% (unit-only) and 32.09% (aggregate) over 10,066 and 10,446 lines of the whole workspace. Those are the figures a casual `cargo llvm-cov --summary-only` shows; this document and the EPIC #1347 tables use the package-filtered figures below.

`src/lib.rs` has no instrumented entry in any report (a type alias, a constant, a derive-only newtype and an enum), so it appears with "No executable entries".

## Test-Level Baseline

### Aggregate/Global Coverage

Aggregate coverage includes the unit-test binary and the integration binary. It is broad navigation evidence only; it is not proof that a source seam has adequate unit coverage.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 865 / 1,014 (85.31%) | 1,278 / 1,578 (80.99%) | 118 / 149 (79.19%) |

### Unit-Only Coverage

Unit-only (`--lib`) coverage is the primary Issue #2637 metric.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 408 / 1,014 (40.24%) | 673 / 1,578 (42.65%) | 36 / 149 (24.16%) |

### Integration-Only Coverage

Integration-only (`--test integration`) coverage informs ownership decisions and is never added to the unit-only figure. The denominator is smaller because the `#[cfg(test)]` modules of `server.rs` and `v1/middlewares/auth.rs` are not compiled into the library for this binary.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 645 / 867 (74.39%) | 897 / 1,297 (69.16%) | 99 / 135 (73.33%) |

The 58 integration tests start a real server through `src/testing/environment.rs` and call it over HTTP with `torrust-tracker-rest-api-client`. They protect the wire contract, but they cover the handlers, responses, and the authentication middleware only through the full network stack. The unit-only column is the gap this issue closes.

## Re-Measurement Before Slice S1

`packages/axum-rest-api-server/` changed on `develop` after `abce11ec8`, so the package was measured again at `0b4b2d0d3` before any S1 test change (same commands, stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`; clean workspace before each report). Two merged changes explain the difference:

- `refactor(axum-rest-api-server): [#2449] migrate the REST API test environment to the token lifecycle` added 8 unit tests inside `src/testing/environment.rs` (371 changed lines). They start the real router, so unit-only coverage of `routes.rs`, `v1/routes.rs`, `v1/responses.rs` and the health check rose without a package-owned unit test of those files; this is incidental coverage through test support, not a behavior contract.
- `fix(axum-rest-api-server): [#2653] reject an unreadable Authorization header` (merged through #2656) changed `v1/middlewares/auth.rs` and added the first middleware test.

The package now has 15 unit tests (3 in `server.rs`, 4 in `v1/middlewares/auth.rs`, 8 in `testing/environment.rs`) and the same 58 integration tests. The figures below replace the baseline tables above for slice work; the EPIC #1347 tables and the specification carry them. The measurement before this one is kept above as history.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 15 unit + 58 integration | 1,091 / 1,240 (87.98%) | 1,618 / 1,924 (84.10%) | 156 / 187 (83.42%) |
| Unit-only (`--lib`) | 15 unit | 792 / 1,240 (63.87%) | 1,231 / 1,924 (63.98%) | 96 / 187 (51.34%) |
| Integration-only (`--test integration`) | 58 integration | 674 / 908 (74.23%) | 941 / 1,352 (69.60%) | 97 / 137 (70.80%) |

The unit-only gain is mostly test support: `src/testing/environment.rs` alone is 301 of its 332 lines (the file is out of scope and its test module is counted). Excluding it, the unit-only lines are 491 / 908 (54.07%); judge the S1 slice by the production lines of its files, not by the total.

Per-file unit-only lines for the rows that changed (all other files have the counts in the inventory):

| Source module | Unit-only lines before | Unit-only lines now | Integration-only lines now | Aggregate lines now |
| --- | ---: | ---: | ---: | ---: |
| `v1/middlewares/auth.rs` | 27 / 83 (32.53%) | 71 / 103 (68.93%) | 54 / 60 | 98 / 103 |
| `v1/responses.rs` | 0 / 40 (0.00%) | 8 / 40 (20.00%) | 40 / 40 | 40 / 40 |
| `routes.rs` | 31 / 67 (46.27%) | 58 / 67 (86.57%) | 61 / 67 | 62 / 67 |
| `v1/context/health_check/handlers.rs` | 0 / 4 (0.00%) | 4 / 4 (100.00%) | 4 / 4 | 4 / 4 |
| `testing/environment.rs` (out of scope) | 0 / 126 (0.00%) | 301 / 332 (90.66%) | 156 / 171 | 325 / 332 |

Unit-only uncovered lines of the S1 files at `0b4b2d0d3`:

- `v1/middlewares/auth.rs`: 82, 86, 88-94, 97-99, 101 (the token sources, precedence and the missing and invalid exits of `auth`), 108, 115, 117-118, 120, 122-123, 125-127 (the `Bearer`, empty-token and wrong-scheme paths of the header extraction), 144-145 and 170-172, 176-178 (the `AuthError` conversions and two of the three failure responses). The unreadable-header rejection is covered by the #2653 test.
- `v1/responses.rs`: 41-48, 53-57, 60-62, 65-72 and 80-87, that is the success, invalid info-hash, invalid key, bad-request and disabled responses (the `409` builder); only the unhandled-rejection response runs.

The inventory rows above keep their original (pre-S1) unit-only columns; the "Per-File Results" section records each file after its slice.

## Module Inventory And T1 Hypotheses

Every one of the 31 `.rs` files under `packages/axum-rest-api-server/src/` (`git ls-files`) appears exactly once, in processing order. The test count is the number of `#[test]` and `#[tokio::test]` functions in the file. Unit-only, integration-only and aggregate columns are line counts. The T1 hypothesis determines processing order but is not a terminal decision: every file still receives its own file test plan, and only that plan can conclude no test change. A property candidate must be assessed by the plan; it does not require a property-testing dependency.

| # | Source module | Tests | Unit-only lines | Unit regions | Unit functions | Integration-only lines | Aggregate lines | Property candidate | Selected behavior or ownership boundary | T1 hypothesis |
| ---: | --- | ---: | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `v1/middlewares/auth.rs` | 3 | 27 / 83 (32.53%) | 51 / 132 | 6 / 14 | 58 / 64 (90.63%) | 77 / 83 (92.77%) | Yes: `authenticate` is equivalent to "the token equals some configured token" for arbitrary tokens and token sets (the constant-time fold must keep that meaning); the bearer-header extraction as a parse of `Bearer <token>` | Token sources and precedence (header over query), the empty token, a header without the `Bearer` scheme prefix, the missing-token case, the three failure responses, and the constant-time comparison (security record `docs/security/analysis/reports/2026-09-04_rest-api-token-timing.md`). Only `authenticate` has unit tests today. | Property candidate |
| 2 | `v1/routes.rs` | 0 | 27 / 45 (60.00%) | 51 / 73 | 1 / 3 | 45 / 45 (100.00%) | 45 / 45 (100.00%) | No | Capability gating: the auth-key and whitelist routes are always registered, and each handler receives its service only when `core.private` (respectively `core.listed`) is set and persistence exists, otherwise it answers `409`; the torrent and stats services are always present. Collaborator adapters and use cases are owned by `rest-api-runtime-adapter` and `rest-api-application`. | Test candidate |
| 3 | `routes.rs` | 0 | 31 / 67 (46.27%) | 57 / 117 | 1 / 8 | 61 / 67 (91.04%) | 61 / 67 (91.04%) | No | Router composition: the authentication layer protects `/api/v1` routes but not `/api/health_check`, the request-id header is set and propagated, the request timeout answers `408`, and the structured request, response and failure log events. Logging is observability; the layer order and the timeout are package-owned decisions. | Test candidate |
| 4 | `v1/context/health_check/handlers.rs` | 0 (1 integration) | 0 / 4 (0.00%) | 0 / 4 | 0 / 2 | 4 / 4 (100.00%) | 4 / 4 (100.00%) | No | The unauthenticated health-check response body and status. | Test candidate (small) |
| 5 | `v1/responses.rs` | 0 | 0 / 40 (0.00%) | 0 / 31 | 0 / 6 | 40 / 40 (100.00%) | 40 / 40 (100.00%) | No | The shared response contract of every context: status, `content-type` and body of the success, `400`, `409` and `500` responses, including the legacy `Unhandled rejection: Err { reason: ... }` debug body kept for backward compatibility. | Test candidate |
| 6 | `v1/context/auth_key/handlers.rs` | 0 (18 integration) | 0 / 56 (0.00%) | 0 / 83 | 0 / 11 | 48 / 56 (85.71%) | 48 / 56 (85.71%) | No | The mapping of each `AuthKeyError` variant to a response for add, generate, delete and reload, the disabled response when the service is absent, and path and body extraction. Key validity, duration limits and persistence are owned by `tracker-core`, `rest-api-application` and `rest-api-runtime-adapter`. | Test candidate |
| 7 | `v1/context/auth_key/responses.rs` | 0 | 0 / 26 (0.00%) | 0 / 30 | 0 / 7 | 23 / 26 (88.46%) | 23 / 26 (88.46%) | No | The key resource response and the four `500` failure messages. | Test candidate |
| 8 | `v1/context/auth_key/routes.rs` | 0 | 16 / 16 (100.00%) | 27 / 27 | 1 / 1 | 16 / 16 (100.00%) | 16 / 16 (100.00%) | No | Route paths and methods, covered through the router tests. | Probably no change (paths pinned by rows 2 and 6) |
| 9 | `v1/context/whitelist/handlers.rs` | 0 (13 integration) | 0 / 37 (0.00%) | 0 / 50 | 0 / 7 | 35 / 37 (94.59%) | 35 / 37 (94.59%) | No | Invalid info-hash rejection, the disabled response, and the failure responses of add, remove and reload. | Test candidate |
| 10 | `v1/context/whitelist/responses.rs` | 0 | 0 / 9 (0.00%) | 0 / 12 | 0 / 3 | 9 / 9 (100.00%) | 9 / 9 (100.00%) | No | The three `500` failure messages. | Test candidate (small) |
| 11 | `v1/context/whitelist/routes.rs` | 0 | 14 / 14 (100.00%) | 25 / 25 | 1 / 1 | 14 / 14 (100.00%) | 14 / 14 (100.00%) | No | Route paths and methods. | Probably no change |
| 12 | `v1/context/torrent/handlers.rs` | 0 (13 integration) | 0 / 50 (0.00%) | 0 / 67 | 0 / 7 | 50 / 50 (100.00%) | 50 / 50 (100.00%) | Yes (weak): `parse_info_hashes` keeps order and fails on the first invalid hash; `empty_string_as_none` | Info-hash validation, pagination query parsing (empty strings as absent, unparsable values rejected), the `info_hash` list query, and the not-known response. | Test candidate |
| 13 | `v1/context/torrent/responses.rs` | 0 | 0 / 9 (0.00%) | 0 / 10 | 0 / 3 | 9 / 9 (100.00%) | 9 / 9 (100.00%) | No | The JSON list, info and "torrent not known" bodies. DTO serialization is owned by `rest-api-protocol`. | Test candidate (small) |
| 14 | `v1/context/torrent/routes.rs` | 0 | 9 / 9 (100.00%) | 15 / 15 | 1 / 1 | 9 / 9 (100.00%) | 9 / 9 (100.00%) | No | Route paths. | Probably no change |
| 15 | `v1/context/stats/handlers.rs` | 0 (4 integration) | 0 / 20 (0.00%) | 0 / 30 | 0 / 8 | 14 / 20 (70.00%) | 14 / 20 (70.00%) | No | The `format` query selecting JSON or Prometheus text for `/stats` and `/metrics`; the Prometheus arms are not exercised at any level. | Test candidate |
| 16 | `v1/context/stats/responses.rs` | 0 | 0 / 45 (0.00%) | 0 / 116 | 0 / 4 | 6 / 45 (13.33%) | 6 / 45 (13.33%) | No | The Prometheus text of `/stats` (one line per field, deprecated fields included) and of the labeled `/metrics`. The text is package-owned; the `Stats` DTO and the metrics serializer are collaborators'. Largest gap at every level. | Test candidate |
| 17 | `v1/context/stats/routes.rs` | 0 | 9 / 9 (100.00%) | 15 / 15 | 1 / 1 | 9 / 9 (100.00%) | 9 / 9 (100.00%) | No | Route paths. | Probably no change |
| 18 | `server.rs` | 3 | 275 / 349 (78.80%) | 432 / 567 | 24 / 44 | 74 / 221 (33.48%) | 275 / 349 (78.80%) | No | Start and stop, token-aware start and its drain, registration-failure cleanup, the listener errors, the `Launcher` display, the health-check job, and TLS startup. The legacy `Halted` start and stop path is scheduled for deprecation and removal by #1488 (SI-18, SI-19); lifecycle policy is owned there. Test-module lines are counted. | Test candidate (TLS, errors, `check_fn`) with legacy path owned by #1488 |
| 19 | `testing/environment.rs` | 0 | 0 / 126 (0.00%) | 0 / 174 | 0 / 18 | 121 / 126 (96.03%) | 121 / 126 (96.03%) | No | Test-support code that starts the server for the integration tests and for `axum-health-check-api-server`. The EPIC does not test test code; gating it behind a `testing` feature is #2482's draft, and its lifecycle migration is #2449. | Out of scope (test support) |
| 20 | `testing/mod.rs` | 0 | No executable entries | - | - | - | - | No | Module declaration and a stale note, covered by the same #2482 draft. | Out of scope (test support) |
| 21 | `lib.rs` | 0 | No executable entries | - | - | - | - | No | Crate documentation, the clock aliases, the log target, `InfoHashParam` and `Version`. | Wiring only |
| 22 | `v1/mod.rs` | 0 | No executable entries | - | - | - | - | No | Module declarations and the context table. | Wiring only |
| 23 | `v1/context/mod.rs` | 0 | No executable entries | - | - | - | - | No | Module declarations. | Wiring only |
| 24 | `v1/middlewares/mod.rs` | 0 | No executable entries | - | - | - | - | No | Module declaration. | Wiring only |
| 25 | `v1/context/auth_key/mod.rs` | 0 | No executable entries | - | - | - | - | No | Module declarations and the endpoint documentation. | Wiring only |
| 26 | `v1/context/health_check/mod.rs` | 0 | No executable entries | - | - | - | - | No | Module declaration and documentation. | Wiring only |
| 27 | `v1/context/stats/mod.rs` | 0 | No executable entries | - | - | - | - | No | Module declarations and the endpoint documentation. | Wiring only |
| 28 | `v1/context/torrent/mod.rs` | 0 | No executable entries | - | - | - | - | No | Module declarations and the endpoint documentation. | Wiring only |
| 29 | `v1/context/torrent/resources/mod.rs` | 0 | No executable entries | - | - | - | - | No | Module declaration. | Wiring only |
| 30 | `v1/context/torrent/resources/torrent.rs` | 0 | No executable entries | - | - | - | - | No | Documentation comment only; the DTOs live in `rest-api-protocol`. | Wiring only |
| 31 | `v1/context/whitelist/mod.rs` | 0 | No executable entries | - | - | - | - | No | Module declarations and the endpoint documentation. | Wiring only |

Rows 1-18 account for every uncovered unit-only line outside test support. The endpoint documentation in the `mod.rs` files is prose with `console` and `json` blocks that no test runs; a wiring plan checks it against the observed behavior and records any mismatch.

## Uncovered Lines By Group

Unit-only uncovered lines from `cargo llvm-cov report --show-missing-lines`, grouped by behavior. Each group is a hypothesis for its file plan, which makes the final decision. Classifications follow the per-file workflow: **3A** a package-owned decision testable at the module's API; **3B** observable only through a package integration boundary; **owner** not testable here, with the owner named; **finding** looks wrong and is handed off; **logging** observability only. Line numbers are at `abce11ec8` and shift when tests are added.

| File | Lines | Behavior | Hypothesis |
| --- | --- | --- | --- |
| `v1/middlewares/auth.rs` | 73-100 | The `auth` middleware: header token, query token, precedence, the missing-token and invalid-token exits | 3A through a minimal router with a fixed `State` and a trivial handler (no network) |
| `v1/middlewares/auth.rs` | 102-127 | `extract_bearer_token_from_header`: absent header, `Bearer` alone (an empty token), a wrong scheme, a valid token | 3A |
| `v1/middlewares/auth.rs` | 141-147, 169-177, 184-186 | `AuthError` to response conversion and the three failure responses | 3A |
| `v1/routes.rs` | 26-33 | Building the auth-key service in private mode with persistence | 3A with a container built from a private configuration |
| `v1/routes.rs` | 56-63 | Building the whitelist service in listed mode with persistence | 3A with a container built from a listed configuration |
| `routes.rs` | 67-79, 87-102, 111-113 | The request-log closure and the response-log closure with its server-error and non-error arms (they run only when a request is served) | 3A through a request on the router; the logged content is observability |
| `routes.rs` | 122-129, 136 | The failure-log closure (`on_failure`; line 136 closes it) | logging (observability only) |
| `routes.rs` | 144 (not listed by the unit-only report) | The `HandleErrorLayer` closure that answers `408` after a timeout. The line itself runs when the router is built, so the report counts it as covered, but the closure body runs only when a request times out. | 3A with a slow handler and a short timeout, if the timeout can be injected; otherwise record the reason |
| `v1/context/health_check/handlers.rs` | 8-10 | The health-check response | 3A |
| `v1/responses.rs` | 41-99 | Every shared response builder | 3A (pure functions) |
| `v1/context/auth_key/handlers.rs` | 30-155 | The four handlers: service absent, each `AuthKeyError` arm, and success | 3A with a fake `AuthKeyPort` |
| `v1/context/auth_key/responses.rs` | 16-60 | The key response and four failure responses | 3A |
| `v1/context/whitelist/handlers.rs` | 27-94 | The three handlers: service absent, invalid info hash, failure and success | 3A with a fake `WhitelistCommandPort` |
| `v1/context/whitelist/responses.rs` | 11-25 | Three failure responses | 3A |
| `v1/context/torrent/handlers.rs` | 29-138 | `get_torrent_handler`, `get_torrents_handler`, `parse_info_hashes` and `empty_string_as_none` | 3A with a fake `TorrentQueryPort` |
| `v1/context/torrent/responses.rs` | 10-25 | The three response builders | 3A |
| `v1/context/stats/handlers.rs` | 28-51 | The `format` selection for both endpoints | 3A with a fake `StatsQueryPort` |
| `v1/context/stats/responses.rs` | 9-82 | The JSON and Prometheus bodies of `/stats` and `/metrics` | 3A |
| `server.rs` | 200, 244, 247 | The failure arm of the startup notification and the log arm when a public URL is configured | 3A (public URL), owner #1488 (legacy startup notification) |
| `server.rs` | 285-322 | The token-aware start: TLS protocol selection, the listener error mappings, and the runtime error | 3A for the protocol selection through a TLS launcher; the runtime-error arm needs a server that fails after start (3A if reachable, otherwise a recorded reason) |
| `server.rs` | 359-364 | The error arms of the legacy `stop` (closed channel, join error) | owner #1488 (legacy path removed by SI-19) |
| `server.rs` | 380-392 | `check_fn`: the health-check job against a live and against a dead address | 3A with an ephemeral server and a closed port |
| `server.rs` | 409-415 | `Launcher` `Display` for the TLS and plain variants | 3A |
| `server.rs` | 452-453, 460-473, 483 | The legacy `Launcher::start` TLS path, its listener errors and the server-stopped log arms | 3A for TLS startup and the listener errors; the log arms are logging |
| `server.rs` (test module) | 564, 670-671 | The TLS branch of the start test and two `panic!` arms | test support; the plan reviews the fixture |
| `testing/environment.rs` | 36-232 | The whole test environment | out of scope (test support); #2482 gates it, #2449 migrates it |

## Prioritized Gaps

1. **Authentication middleware (row 1).** Only the constant-time comparison has unit tests. The token sources, precedence and failure exits, which decide who may call a management API, are protected only by the 9 integration tests in `tests/server/v1/contract/authentication.rs`.
2. **Route composition (rows 2-3).** Whether the key and whitelist services are present for a configuration (the routes are always registered; an absent service answers `409`), and that the health check is outside the authentication layer, are protected only by integration tests. The unit run never serves a request.
3. **Shared and per-context responses (rows 5, 7, 10, 13, 16).** No unit test checks a status, header or body. The Prometheus text of `/stats` (row 16) and the `format` selection (row 15) have no test at any level for 39 of 45 lines.
4. **Handlers (rows 6, 9, 12, 15).** Every `AuthKeyError` arm and every failure response is reached only through the network stack. The application ports are traits, so a fake port reaches each arm without a socket.
5. **Server lifecycle (row 18).** TLS startup, the listener-error arms, `check_fn` and the `Launcher` display are uncovered at every level.

## Test-Technique Assessment

EPIC #1347 requires every package subissue to assess each test level. T1 records the initial assessment; each file plan and the final assessment in T8 refine it.

| Technique | Assessment |
| --- | --- |
| Unit tests | Selected. Primary objective, as in-file `#[cfg(test)]` modules next to the code. Handlers take `State<Arc<...ApiService>>` and the services wrap `Box<dyn ...Port>` traits, so a hand-written port fake and an in-process `tower` request reach every handler arm with no socket. Pure functions (responses, bearer extraction, `parse_info_hashes`) are called directly. |
| Package integration tests | Existing and retained: 58 tests in `tests/` start the real server and call it with `rest-api-client`. They protect the wire contract over HTTP and the `rest-api-client` compatibility. They are not reviewed, moved or deleted here (an integration-test review is a separate subissue, as #2140 was for `axum-http-server`). A new integration test is added only if a file plan records why a unit seam cannot protect the behavior; TLS startup is the likely case and is decided in the `server.rs` plan. |
| Runnable examples | Not applicable. The package has no `examples/` and its crate documentation consists of `toml`, `bash`, `json` and `console` blocks that are not Rust. |
| Root `tests/` or `packages/e2e-tools/` | Not selected. Their coverage of the REST API is exercised through the same endpoints and is not evidence for a package-owned unit seam. |
| Mutation testing | Selected as a bounded whole-package run in T6 (the package has few functions), with `RUSTFLAGS="--cap-lints=warn"` and a stated reason for every missed, timed-out and unviable mutant (#1348, #1349 lessons). |
| Property-based testing | Assessed per seam. Candidates: `authenticate` against a model (any token set, any token), the bearer-header extraction, and weakly `parse_info_hashes`. A property is kept only if it protects state that examples miss; if kept, `quickcheck` is added as a dev-dependency following the `add-rust-dependency` skill, as in #1348. Discrete branch policy (the `AuthKeyError` arms, the capability gating by configuration) is better served by `rstest` tables. |
| Fuzzing | Not selected. Untrusted input reaches this package as header values, query strings and path segments already parsed by Axum and serde; the package-owned parsers are small and covered by properties and examples. Record as deferred if the authentication plan finds a reason. |
| Benchmarks | Out of scope. The package has none and benchmarks are not tests of correctness. |

## Per-File Results

Added by each slice as its files complete.

### `v1/middlewares/auth.rs` (slice S1)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 71 / 103 (68.93%) before, 142 / 142 (100.00%) after; regions 124 / 179 before, 223 / 223 after; functions 13 / 17 before, 22 / 22 after. The counts include the in-file test module. 27 test cases replace the four tests; eight named mutations are killed (see [the plan](test-refactor-plans/v1-middlewares-auth-tests.md)).

### `v1/responses.rs` (slice S1)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 8 / 40 (20.00%) before, 66 / 66 (100.00%) after; regions 6 / 31 before, 69 / 69 after; functions 1 / 6 before, 10 / 10 after. The counts include the in-file test module. Eight test cases; eight named mutations killed (see [the plan](test-refactor-plans/v1-responses-tests.md)).

### Slice S1 Measurement

After both S1 files (`v1/middlewares/auth.rs` and `v1/responses.rs`), on the S1 branch rebased onto `develop` (the only package change since the S1 baseline is a `torrust-server-lib` 0.4.0 bump); stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, a clean workspace before each report, the commands of the Measurement Scope section:

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 46 unit + 58 integration | 1,161 / 1,305 (88.97%) | 1,710 / 2,006 (85.24%) | 167 / 196 (85.20%) |
| Unit-only (`--lib`) | 46 unit | 921 / 1,305 (70.57%) | 1,393 / 2,006 (69.44%) | 114 / 196 (58.16%) |
| Integration-only (`--test integration`) | 58 integration | 674 / 908 (74.23%) | 941 / 1,352 (69.60%) | 97 / 137 (70.80%) |

Unit-only lines move 792 / 1,240 (63.87%) at the S1 baseline to 921 / 1,305 (70.57%). Both files count their new test modules, so the denominator grew by 65 lines; the production lines of the two files are now fully covered by unit tests. The integration-only figure is unchanged because the integration binary does not compile the in-file test modules.

### `v1/routes.rs` (slice S2)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 27 / 45 (60.00%) before, 111 / 111 (100.00%) after; regions 51 / 73 before, 174 / 174 after; functions 1 / 3 before, 13 / 13 after. The counts include the in-file test module. The new crate-private `test_support.rs` helper is 31 / 31 lines. Nineteen test cases; six named mutations killed (see [the plan](test-refactor-plans/v1-routes-tests.md)).

### `routes.rs` (slice S2)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 58 / 67 (86.57%) before, 169 / 169 (100.00%) after; regions 103 / 117 before, 277 / 277 after; functions 5 / 8 before, 26 / 26 after. The counts include the in-file test module. The request, response and failure log closures run but their content is not asserted at unit level (an integration test asserts the `ERROR` line). Nine test cases; eight of ten named mutations killed, two survive by design (see [the plan](test-refactor-plans/routes-tests.md)).

### `v1/context/health_check/handlers.rs` (slice S2)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 4 / 4 (100.00%) before, 16 / 16 (100.00%) after; regions 4 / 4 before, 25 / 25 after; functions 2 / 2 before, 4 / 4 after. The file was already fully covered; the single test adds protection (nothing asserted the answer), and the counts grow only by the test module. The mutation (report `Status::Error`) is killed (see [the plan](test-refactor-plans/v1-context-health-check-handlers-tests.md)).

### Slice S2 Measurement

After the four S2 files (`v1/routes.rs`, `routes.rs`, `v1/context/health_check/handlers.rs`, and the crate-private `test_support.rs` helper), on the S2 branch rebased onto `develop`. Stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, a clean workspace before each report, the commands of the Measurement Scope section. `develop` changed the package while S2 ran: `feat(axum-rest-api-server): apply the configured drain timeout to the REST API` (in `server.rs`, which is the S5 file, and the out-of-scope `testing/environment.rs`) added a unit test and 40 lines, so part of the rise below is not from S2.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 76 unit + 58 integration | 1,421 / 1,556 (91.32%) | 2,122 / 2,405 (88.23%) | 205 / 230 (89.13%) |
| Unit-only (`--lib`) | 76 unit | 1,305 / 1,556 (83.87%) | 1,938 / 2,405 (80.58%) | 182 / 230 (79.13%) |
| Integration-only (`--test integration`) | 58 integration | 682 / 916 (74.45%) | 949 / 1,360 (69.78%) | 98 / 138 (71.01%) |

Unit-only lines move 921 / 1,305 (70.57%) at the S2 baseline (the S1 measurement) to 1,305 / 1,556 (83.87%). The files S2 changed are fully covered by unit tests, counting their test modules: `v1/routes.rs` 111 / 111, `routes.rs` 169 / 169, `v1/context/health_check/handlers.rs` 16 / 16 and `test_support.rs` 31 / 31. The remaining uncovered production lines are in `server.rs` (S5), the context handlers and responses (S3 and S4), and the out-of-scope `testing/environment.rs`.

### `v1/context/auth_key/handlers.rs` (slice S3)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 0 / 56 (0.00%) before, 261 / 264 (98.86%) after; regions 0 / 83 before, 373 / 378 after; functions 0 / 11 before, 62 / 63 after. The counts include the in-file test module. The three uncovered lines (153-155) are `AuthKeyErrorDisplay`'s `Debug` implementation, deliberately not tested. 13 test functions (18 cases); nine named mutations killed (see [the plan](test-refactor-plans/v1-context-auth-key-handlers-tests.md)). Package unit-only after this file: lines 1,563 / 1,770 (88.31%), regions 2,315 / 2,708 (85.49%), functions 245 / 284 (86.27%), 94 unit tests.

### `v1/context/auth_key/responses.rs` (slice S3)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 0 / 26 (0.00%) at the S1 baseline, 26 / 26 (100.00%) after row 6 with no change to this file; regions 30 / 30; functions 7 / 7. The file has no test module of its own: the row 6 handler tests assert each of its seven response builders with literal status, content type and body. Decision: no change. Eight named mutations (one per builder, plus the status and content type) each fail an assertion of those tests (see [the plan](test-refactor-plans/v1-context-auth-key-responses-tests.md)).

### `v1/context/auth_key/routes.rs` (slice S3)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 16 / 16 (100.00%), regions 27 / 27, functions 1 / 1, before and after; the file has no test module and is covered by the `v1/routes.rs` tests. Decision: no change. Eight named mutations (three path changes, a method change and `None` state on the add, reload, generate and delete routes) each fail an existing unit or integration test on an assertion (see [the plan](test-refactor-plans/v1-context-auth-key-routes-tests.md)).

### `v1/context/whitelist/handlers.rs` (slice S3)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 0 / 37 (0.00%) at the S1 baseline (26 / 37 before this file, from the S2 gating tests), 198 / 198 (100.00%) after; regions 334 / 334; functions 47 / 47. The counts include the in-file test module. 12 test functions (13 cases); ten named mutations killed (see [the plan](test-refactor-plans/v1-context-whitelist-handlers-tests.md)). One case pins a documented difference with the auth-key handlers (a service-reported disabled capability is a `500` here, a `409` there; not reachable today). Package unit-only after this file: lines 1,744 / 1,931 (90.32%), regions 2,638 / 2,992 (88.17%), functions 288 / 324 (88.89%), 106 unit tests.

### `v1/context/whitelist/responses.rs` (slice S3)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 0 / 9 (0.00%) at the S1 baseline, 9 / 9 (100.00%) after row 9 with no change to this file; regions 12 / 12; functions 3 / 3. The file has no test module of its own: the row 9 handler tests assert each of its three messages with a literal. Decision: no change. Three named mutations (one per message) each fail the named handler test (see [the plan](test-refactor-plans/v1-context-whitelist-responses-tests.md)).

### `v1/context/whitelist/routes.rs` (slice S3)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 14 / 14 (100.00%), regions 25 / 25, functions 1 / 1, before and after; the file has no test module and is covered by the `v1/routes.rs` tests. Decision: no change. Seven named mutations (prefix, reload path, the add and remove methods, and `None` state on the add, remove and reload routes) each fail an existing unit or integration test on an assertion. Replacing a method with a dummy handler (M3, M4) is killed only by the integration tests, while dropping the method (M3b, M4b) is killed by the `v1/routes.rs` table (see [the plan](test-refactor-plans/v1-context-whitelist-routes-tests.md)).

### Slice S3 Measurement

After the six S3 files (`v1/context/auth_key/{handlers,responses,routes}.rs` and `v1/context/whitelist/{handlers,responses,routes}.rs`), on the S3 branch rebased onto `develop` (the package and its `rest-api-*` collaborators are unchanged since the S2 head; `develop` changed other packages and documentation only). Stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, a clean workspace before each report, the commands of the Measurement Scope section:

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 106 unit + 58 integration | 1,806 / 1,931 (93.53%) | 2,722 / 2,992 (90.98%) | 300 / 324 (92.59%) |
| Unit-only (`--lib`) | 106 unit | 1,744 / 1,931 (90.32%) | 2,638 / 2,992 (88.17%) | 288 / 324 (88.89%) |
| Integration-only (`--test integration`) | 58 integration | 682 / 916 (74.45%) | 949 / 1,360 (69.78%) | 98 / 138 (71.01%) |

Unit-only lines move 1,305 / 1,556 (83.87%) at the S3 baseline (the S2 measurement) to 1,744 / 1,931 (90.32%). Two files gained test modules (`auth_key/handlers.rs` and `whitelist/handlers.rs`), so the denominator grew by 375 lines; the other four S3 files are unchanged and were already fully covered by tests of earlier rows. The integration-only figure is unchanged because the integration binary does not compile the in-file test modules. The remaining uncovered unit-only lines are in `server.rs` (74, S5), `v1/context/stats/responses.rs` (39), `testing/environment.rs` (31, out of scope), `v1/context/torrent/handlers.rs` (29), `v1/context/stats/handlers.rs` (8), `auth_key/handlers.rs` (3, the `Debug` implementation of `AuthKeyErrorDisplay`) and `v1/context/torrent/responses.rs` (3); the torrent and stats files are S4.

### Slice S3 Re-Measurement After The Review Of #2702

Copilot's review of #2702 led to two test-only changes: the response-reading helper became shared in `test_support.rs` (the seven modules that each had a copy now use it), and the two port fakes record typed `Call` values. Both remove duplicated test code, so the denominator falls and the percentages move slightly; no test was added or removed. Measured on the branch after both changes and the merge of `develop` (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, a clean workspace before each report, the commands of the Measurement Scope section). This is the latest S3 figure; the slice measurement above is the one taken before the review.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 106 unit + 58 integration | 1,730 / 1,855 (93.26%) | 2,619 / 2,889 (90.65%) | 298 / 322 (92.55%) |
| Unit-only (`--lib`) | 106 unit | 1,668 / 1,855 (89.92%) | 2,535 / 2,889 (87.75%) | 286 / 322 (88.82%) |
| Integration-only (`--test integration`) | 58 integration | 682 / 916 (74.45%) | 949 / 1,360 (69.78%) | 98 / 138 (71.01%) |

Unit-only lines per file, before and after the two changes (the per-file sections above record the figure at each file's completed-file review): `routes.rs` 169 / 169 to 167 / 167, `test_support.rs` 35 / 35 to 63 / 63, `v1/context/auth_key/handlers.rs` 261 / 264 to 236 / 239, `v1/context/health_check/handlers.rs` 18 / 18 to 15 / 15, `v1/context/whitelist/handlers.rs` 198 / 198 to 175 / 175, `v1/middlewares/auth.rs` 142 / 142 to 133 / 133, `v1/responses.rs` 66 / 66 to 47 / 47 and `v1/routes.rs` 111 / 111 to 88 / 88. The uncovered lines are unchanged (the three `Debug` lines of `AuthKeyErrorDisplay` and the S4, S5 and out-of-scope files listed above). Mutations of the shared `observed` fail 48 to 62 unit tests, and the forwarding mutations of both fakes still fail their tests.

### `v1/context/torrent/handlers.rs` (slice S4)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 21 / 50 (42.00%) at the S4 baseline, 194 / 194 (100.00%) after; regions 20 / 67 before, 248 / 248 after; functions 4 / 7 before, 30 / 30 after. The counts include the in-file test module. 10 test functions (34 cases); 13 named mutations killed, two of them only after a correction (a palindromic order test, and a dead empty-string arm now tested directly; see [the plan](test-refactor-plans/v1-context-torrent-handlers-tests.md)). `torrent/responses.rs` (row 13) became 9 / 9 lines through the "not known" test. Package unit-only after this file: lines 1,844 / 1,999 (92.25%), regions 2,766 / 3,070 (90.10%), functions 313 / 345 (90.72%), 140 unit tests.

### `v1/context/torrent/responses.rs` (slice S4)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 0 / 9 (0.00%) at the S1 baseline and 6 / 9 at the S4 baseline, 9 / 9 (100.00%) after row 12 with no change to this file; regions 10 / 10; functions 3 / 3. The file has no test module of its own: the row 12 handler tests assert each of its three answers with a literal. Decision: no change. Three named mutations each fail the named handler test (see [the plan](test-refactor-plans/v1-context-torrent-responses-tests.md)).

### `v1/context/torrent/routes.rs` (slice S4)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 9 / 9 (100.00%), regions 15 / 15, functions 1 / 1, before and after; the file has no test module and is covered by the `v1/routes.rs` tests. Decision: no change. Four named mutations (two path changes, the two handlers swapped and the list method) each fail an existing unit test on an assertion (see [the plan](test-refactor-plans/v1-context-torrent-routes-tests.md)).

### `v1/context/stats/handlers.rs` (slice S4)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 12 / 20 (60.00%) at the S4 baseline, 40 / 40 (100.00%) after; regions 61 / 61; functions 14 / 14. The counts include the in-file test module. 4 test functions (12 cases); 8 of 9 named mutations killed, and the ninth is an equivalent mutant recorded in advance (see [the plan](test-refactor-plans/v1-context-stats-handlers-tests.md)). The two shared fixtures `a_stats` and `a_labeled_stats` were added to `test_support.rs` (109 / 109 lines). `stats/responses.rs` (row 16) now reads 45 / 45 lines only because the Prometheus handler case runs all its metric lines while asserting the first two, so row 16 still has to assert them. Package unit-only after this file: lines 1,957 / 2,065 (94.77%), regions 2,932 / 3,116 (94.09%), functions 325 / 353 (92.07%), 152 unit tests.

### `v1/context/stats/responses.rs` (slice S4)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 6 / 45 (13.33%) at the S4 baseline, 45 / 45 after row 15 (run but not asserted: only two of the 32 metric lines were checked), 155 / 155 (100.00%) after this file, where every line of the legacy text is now asserted; regions 168 / 168; functions 12 / 12. The counts include the in-file test module. 4 test functions; 10 named mutations killed (see [the plan](test-refactor-plans/v1-context-stats-responses-tests.md)). The legacy text is pinned as it is, with a `TODO` that whether its non-numeric `completed_persisted_enabled true` line is a bug is pending re-evaluation (a separate investigation, recorded in `lessons.md`). Package unit-only after this file: lines 2,067 / 2,175 (95.03%), regions 2,984 / 3,168 (94.19%), functions 333 / 361 (92.24%), 156 unit tests.

### `v1/context/stats/routes.rs` (slice S4)

Unit-only (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): lines 9 / 9 (100.00%), regions 15 / 15, functions 1 / 1, before and after; the file has no test module and is covered by the `v1/routes.rs` tests. Decision: no change. Four named mutations (two path changes, the metrics method and the two handlers swapped) each fail an existing test on an assertion; the handler swap is killed only by the integration tests (see [the plan](test-refactor-plans/v1-context-stats-routes-tests.md)).

### Slice S4 Measurement

After the six S4 files (`v1/context/torrent/{handlers,responses,routes}.rs` and `v1/context/stats/{handlers,responses,routes}.rs`, plus the two shared fixtures added to `test_support.rs`), on the S4 branch rebased onto `develop` (the package and its `rest-api-*` collaborators are unchanged since the S3 head; `develop` changed `Cargo.lock`, other packages and documentation only). Stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, a clean workspace before each report, the commands of the Measurement Scope section:

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 156 unit + 58 integration | 2,091 / 2,175 (96.14%) | 3,011 / 3,168 (95.04%) | 339 / 361 (93.91%) |
| Unit-only (`--lib`) | 156 unit | 2,067 / 2,175 (95.03%) | 2,984 / 3,168 (94.19%) | 333 / 361 (92.24%) |
| Integration-only (`--test integration`) | 58 integration | 682 / 916 (74.45%) | 949 / 1,360 (69.78%) | 98 / 138 (71.01%) |

Unit-only lines move 1,668 / 1,855 (89.92%) at the S4 baseline (the S3 re-measurement) to 2,067 / 2,175 (95.03%). Two files gained substantial test modules and `test_support.rs` gained two fixtures, so the denominator grew by 320 lines; the four route and response files recorded as no change are unchanged. The integration-only figure is unchanged because the integration binary does not compile the in-file test modules. The only uncovered unit-only lines left are in `server.rs` (74, S5), `testing/environment.rs` (31, out of scope, test support owned by #2482 and #2449) and `auth_key/handlers.rs` (3, the `Debug` implementation of `AuthKeyErrorDisplay`), 108 in total; everything else, including every file of the torrent, stats, whitelist and health-check contexts, is fully covered by unit tests, and the auth-key contexts apart from those three `Debug` lines. Two files read as covered by an earlier row but needed their own assertions (`torrent/responses.rs` and `stats/responses.rs`); the second was 45 / 45 lines after row 15 with only two of its 32 metric lines asserted.

### Slice S4 Re-Measurement After The Review Of #2718

Copilot's review of #2718 led to one test-only change: the stats `format` test maps the parsed value to a small local enum with an exhaustive `match` instead of comparing its `Debug` text, which adds a few test lines. No test was added or removed. Measured on the branch after the change (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, a clean workspace before each report, the commands of the Measurement Scope section). This is the latest S4 figure; the slice measurement above is the one taken before the review.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 156 unit + 58 integration | 2,097 / 2,181 (96.15%) | 3,017 / 3,174 (95.05%) | 340 / 362 (93.92%) |
| Unit-only (`--lib`) | 156 unit | 2,073 / 2,181 (95.05%) | 2,990 / 3,174 (94.20%) | 334 / 362 (92.27%) |
| Integration-only (`--test integration`) | 58 integration | 682 / 916 (74.45%) | 949 / 1,360 (69.78%) | 98 / 138 (71.01%) |

`v1/context/stats/handlers.rs` is now 46 / 46 lines (it was 40 / 40), 67 / 67 regions and 15 / 15 functions; the other files are unchanged. The uncovered lines are the same 108 as before.

### Slice S5 Baseline After #2719 And #2726

Measured on the S5 branch rebased onto `torrust/develop` (it contains the fixes of #2719 and #2726, and `server.rs` grew by about 108 lines), stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, a clean workspace before each report, the commands of the Measurement Scope section. This supersedes the S4 figures as the baseline of S5; the S4 sections above stay as written. The stats responses text they call "pinned pending re-evaluation" was fixed by #2719 (PR #2727) and the stats responses tests were updated there.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 159 unit + 58 integration | 2,196 / 2,261 (97.13%) | 3,188 / 3,311 (96.29%) | 351 / 371 (94.61%) |
| Unit-only (`--lib`) | 159 unit | 2,172 / 2,261 (96.06%) | 3,161 / 3,311 (95.47%) | 345 / 371 (92.99%) |
| Integration-only (`--test integration`) | 58 integration | 682 / 927 (73.57%) | 949 / 1,376 (68.97%) | 98 / 140 (70.00%) |

Unit-only files below 100% lines: `server.rs` 403 / 458 (87.99%; 666 / 767 regions; 35 / 53 functions), `testing/environment.rs` 301 / 332 (out of scope, owned by #2482 and #2449) and `v1/context/auth_key/handlers.rs` 236 / 239 (the three deliberately untested `Debug` lines). `server.rs` is 403 / 458 in the aggregate as well: the integration tests add nothing to it (74 / 233 lines alone). Uncovered unit-only line ranges of `server.rs` at this baseline: 198, 242, 244-245, 277, 279-280, 289-290, 294, 296, 301-302, 310, 312, 318-319, 358-359, 362-363, 379-381, 396, 428-430, 432, 434, 455-457, 469, 471-472, 478-483, 487-488, 490-492, 494, 496, 502, 514, 583-584 and 742-743 (55 lines). The row 18 plan re-derives and classifies these groups.

### Row 18 (`server.rs`) Measurement

Measured on the S5 branch after the row 18 tests (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, a clean workspace before each report, the commands of the Measurement Scope section). The S5 slice measurement is taken again at the slice close; this is the figure after the only file of S5.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 164 unit + 58 integration | 2,350 / 2,405 (97.71%) | 3,455 / 3,557 (97.13%) | 370 / 387 (95.61%) |
| Unit-only (`--lib`) | 164 unit | 2,326 / 2,405 (96.72%) | 3,428 / 3,557 (96.37%) | 364 / 387 (94.06%) |
| Integration-only (`--test integration`) | 58 integration | 682 / 927 (73.57%) | 949 / 1,376 (68.97%) | 98 / 140 (70.00%) |

`server.rs` is 533 / 578 lines unit-only (it was 403 / 458), 901 / 981 regions and 50 / 65 functions. Its 45 uncovered lines (198, 244-245, 279-280, 289-290, 294, 296, 301-302, 310, 312, 318-319, 358-359, 362-363, 455-457, 469, 471-472, 478-483, 487-488, 490-492, 494, 496, 502, 514, 744-745 and 862-863) are classified in the [row 18 plan](test-refactor-plans/server-tests.md): logging-only, platform faults, runtime error arms owned by `torrust-tracker-axum-server`, the legacy `Halted` path owned by #1488, and `panic!` arms of tests. Files below 100% unit-only lines: `server.rs`, `testing/environment.rs` (out of scope, #2482 and #2449) and `v1/context/auth_key/handlers.rs` (236 / 239, the three untested `Debug` lines).

### Slice S5 Measurement After The Rebase Onto `develop`

The unpushed S5 branch was rebased onto `torrust/develop` at the slice close. `develop` meanwhile fixed #2733 (`check_fn` reports an error for a non-success status, with two tests in `server.rs`) and changed the stats files, so the row 18 figures above are superseded by these (same commands, clean workspace before each report). This is the latest S5 figure.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 167 unit + 58 integration | 2,345 / 2,400 (97.71%) | 3,484 / 3,586 (97.16%) | 380 / 397 (95.72%) |
| Unit-only (`--lib`) | 167 unit | 2,321 / 2,400 (96.71%) | 3,457 / 3,586 (96.40%) | 374 / 397 (94.21%) |
| Integration-only (`--test integration`) | 58 integration | 682 / 903 (75.53%) | 949 / 1,299 (73.06%) | 98 / 140 (70.00%) |

`server.rs` is 576 / 621 lines unit-only (981 / 1,061 regions, 58 / 73 functions); its 45 uncovered lines are the same groups as in the row 18 plan, shifted by one line. Files below 100% unit-only lines are `server.rs`, `testing/environment.rs` (out of scope) and `v1/context/auth_key/handlers.rs` (236 / 239).

### Slice S5 Measurement After The Review Fix

The review of S5 added an assertion to the registration-failure test (a few test lines), so the figures moved slightly. This is the latest S5 figure (same commands, clean workspace before each report, rebased onto `develop`).

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 167 unit + 58 integration | 2,348 / 2,403 (97.71%) | 3,490 / 3,592 (97.16%) | 380 / 397 (95.72%) |
| Unit-only (`--lib`) | 167 unit | 2,324 / 2,403 (96.71%) | 3,463 / 3,592 (96.41%) | 374 / 397 (94.21%) |
| Integration-only (`--test integration`) | 58 integration | 682 / 903 (75.53%) | 949 / 1,299 (73.06%) | 98 / 140 (70.00%) |

`server.rs` is 579 / 624 lines unit-only (987 / 1,067 regions, 58 / 73 functions); its 45 uncovered lines are the same groups as in the row 18 plan (the two test `panic!` arms are now 747-748 and 869-870).

### Final Measurement (slice S6)

`cargo-llvm-cov 0.6.16` on stable Rust 1.99.1, the same three commands as above with a clean workspace before each report, on the S6 branch after the duplicate-token test (`authenticate`, the only test added in S6). Figures are filtered to `packages/axum-rest-api-server/src/`; the totals include the in-file test modules.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 170 unit + 58 integration | 2,348 / 2,404 (97.67%) | 3,489 / 3,592 (97.13%) | 380 / 398 (95.48%) |
| Unit-only (`--lib`) | 170 unit | 2,324 / 2,404 (96.67%) | 3,462 / 3,592 (96.38%) | 374 / 398 (93.97%) |
| Integration-only (`--test integration`) | 58 integration | 682 / 903 (75.53%) | 949 / 1,299 (73.06%) | 98 / 140 (70.00%) |

The added test function counts as one more function and one more line in `test_support.rs`'s JSON summary (126 lines, 125 covered, with no zero-count line in the text report: a counting artefact, the file was not changed), so the percentages fall by about 0.04 pp against the S5 final figures while the covered counts are unchanged. Against the T1 baselines the deltas are: aggregate lines +9.69 pp, regions +13.03 pp, functions +12.06 pp; unit-only lines +32.80 pp, regions +32.40 pp, functions +42.63 pp.

Every uncovered line is classified in the row plans: `server.rs` 45 (logging-only, unreachable, or owned by #1488 and #2733), `testing/environment.rs` 31 (test support owned by #2482 and #2449, out of scope), `test_support.rs` 1 (the counting artefact), and `v1/context/auth_key/handlers.rs` 3 (the `Debug` impl decided as untested).
