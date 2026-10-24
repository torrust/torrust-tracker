---
doc-type: coverage-evidence
issue: 2637
package: torrust-tracker-axum-rest-api-server
measured-commit: abce11ec8
measured-utc: 2026-10-24T19:50:00Z
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
