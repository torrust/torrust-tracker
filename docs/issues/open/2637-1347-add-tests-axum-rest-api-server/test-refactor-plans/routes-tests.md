---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/routes.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/routes.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-routes-tests.md
---

# Router Layers File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/axum-rest-api-server/src/routes.rs`,
ledger row 3 (rank 2), the second file of S2.

## Current State

- **Unit-only coverage:** 58 / 67 lines (86.57%), 103 / 117 regions, 5 / 8 functions (S2 head, `cargo-llvm-cov 0.6.16`,
  stable Rust 1.99.1). The covered lines run through the test-environment unit tests, which start the real router.
  Nothing asserts the file's behavior. Uncovered: lines 100 and 102 (the server-error response log arm) and
  124-129 and 136 (the `on_failure` closure).
- **Existing tests:** none in the file. The integration tests exercise the router over HTTP; one asserts that an
  authentication failure logs an `ERROR` line with the request id.
- **Module-owned decisions:** `router(container, access_tokens, binding)` composes the v1 routes, puts the
  authentication layer on them **before** adding `/api/health_check`, so the health check stays public, then wraps
  everything in compression, request-id setting and propagation, request tracing and a 5 s timeout that answers
  `408`. The log events are structured and use the `API` target.
- **T1 hypothesis:** test candidate.

## Current Tests Review

No tests in the file, so R1 is skipped.

## Coverage And Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| The authentication layer protects `/api/v1` routes and is built from the tokens passed in; the health check is outside it | 3A unit | R2: call `router` in-process; the health check answers `200` without a token, a v1 route answers the `unauthorized` failure without one and `200` with a valid one. The middleware's own cases are done in S1; here only the wiring (applied to v1, not to the health check, fed the given tokens). |
| Request id: generated when absent, kept when provided, copied to the response | 3A unit | R2: a request without `x-request-id` gets a response with a UUID in that header; a request with `x-request-id: my-request-1` gets the same value back. |
| Compression | 3A unit | R2: with `Accept-Encoding: gzip` a v1 response has `content-encoding: gzip`; without it, none. Only the header is asserted (decoding the body is `tower-http`'s). |
| The `408` timeout (`HandleErrorLayer` closure, line 144, plus the 5 s constant) | 3A unit after a seam | R0 (below) splits `router` into the route composition and a private `with_layers(routes, access_tokens, binding, timeout)`. R2: pass a test route that never answers, a 50 ms timeout and a valid token; expect `408` and an empty body. The 5 s default is not asserted (see the mutation list). |
| Request, response and failure log events (lines 67-136) | Not selected at unit level | Maintainer decision 2026-10-28: do not assert log events here; no new dev-dependency. The integration test asserts the `ERROR` line with the request id. |
| The server-error log arm and the failure closure (100, 102, 124-129, 136) | Coverage side effect | They run for any `5xx` response, so the unauthorized request of the first group executes them. The plan does not claim they are asserted: their content is observability, and the log text is asserted by an integration test (`tests/server/v1/contract/authentication.rs`, the `ERROR ... API ... <request id>` line). |

### R0: The Testability Refactor

Maintainer decision 2026-10-28 (Question 1, option b): a small, behavior-preserving refactor of `routes.rs`, done in this
issue and in its own commit before any test, so the `408` handler can be reached.

- Add `const API_URL_PREFIX: &str = "/api";` and keep `DEFAULT_REQUEST_TIMEOUT` (5 s).
- `router` keeps its signature and its `#[instrument]`: it composes `v1::routes::add(API_URL_PREFIX, Router::new(),
  container)` and calls `with_layers(routes, access_tokens, binding, DEFAULT_REQUEST_TIMEOUT)`.
- A private `with_layers(routes: Router, access_tokens: Arc<AccessTokens>, binding: &ServiceBinding, timeout: Duration)
  -> Router` holds the existing chain unchanged: the authentication layer, then the public health check route, then
  compression, request id, propagation, tracing and the timeout, in the same order, with `timeout` replacing the
  constant.
- No other file changes. Validation that it preserves behavior: the 58 integration tests and the unit tests pass
  unchanged, plus the new tests for the layers.

### Boundary And Arrange

The tests call `routes::router(&container, tokens, &service_binding)` (and, for the timeout only, the private
`with_layers` with a test route and a short timeout) and send requests with `tower::ServiceExt::oneshot`. The container comes from `test_support::api_container(&ephemeral())`; the tokens are a
literal `AccessTokens` (`test-token`) built in the test, not read from the configuration; the binding is
`ServiceBinding::new(Protocol::HTTP, 127.0.0.1:1212)`. No socket is opened.

### Cases For R2 (literal expectations)

| Request | Expected |
| --- | --- |
| `GET /api/health_check`, no token | `200` |
| `GET /api/v1/stats`, no token | `500`, `Unhandled rejection: Err { reason: "unauthorized" }` (the content type is asserted by the S1 middleware tests) |
| `GET /api/v1/stats?token=wrong` | `500`, the `token not valid` body |
| `GET /api/v1/stats?token=test-token` | `200` |
| `GET /api/health_check`, no `x-request-id` | response header `x-request-id` is a valid UUID (parsed with `uuid`) |
| `GET /api/health_check`, `x-request-id: my-request-1` | response header `x-request-id` equals `my-request-1` |
| `GET /api/v1/stats?token=test-token`, `Accept-Encoding: gzip` | `content-encoding: gzip` |
| the same without `Accept-Encoding` | no `content-encoding` header |
| `with_layers` with a route that never answers, a 50 ms timeout, `GET /api/v1/slow?token=test-token`, awaited under an absolute 5 s deadline | `408`, empty body |

## Property-Test Decision

Not flagged and not selected: the layers are wiring with a handful of enumerable outcomes.

## Dependencies

None new: `uuid` (already a dev-dependency) parses the generated id, and `rstest`, `tower` and the helper exist.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R0 | DONE | The behavior-preserving testability refactor above, in its own `refactor` commit. | The existing unit and integration tests pass unchanged, `cargo +nightly fmt --all -- --check`, `cargo clippy`. |
| R1 | SKIPPED | No existing tests. | None. |
| R2 | DONE | Add the in-file test module with the cases above. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | DONE | Integration increment: `none` expected. | The integration tests keep the wire and log contract. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Named Mutations

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| L1 | Authentication layer | Add the health check route before `.layer(auth)` so the layer covers it. | the public health check case |
| L2 | Authentication layer | Remove the `.layer(middleware::from_fn_with_state(state, auth))` call. | the `unauthorized` and `token not valid` cases |
| L3 | Authentication state | Build `State` with no tokens (`AccessTokens::default()`). | the valid-token case |
| L4 | Request-id propagation | Remove `PropagateHeaderLayer`. | the provided-id case and the generated-id case |
| L5 | Compression | Remove `CompressionLayer`. | the gzip case |
| L6 | Request-id setting (first layer) | Remove the inner `SetRequestIdLayer`. | expected to **survive**: the second layer already sets the header. |
| L7 | Request-id setting (second layer) | Remove the outer `SetRequestIdLayer`. | the generated-id case |
| L8 | Timeout layer | Remove the `TimeoutLayer`. | the timeout case (a bounded await fails instead of hanging) |
| L9 | Timeout error mapping | Answer `INTERNAL_SERVER_ERROR` in the `HandleErrorLayer` closure. | the timeout case |
| L10 | Default timeout | Change `DEFAULT_REQUEST_TIMEOUT` to 50 s. | expected to **survive**: the default is not asserted, because that would need a 5 s wait or a tautology; recorded as a known gap. |

`router` adds `SetRequestIdLayer::x_request_id(MakeRequestUuid)` twice. If L6 survives, the inner one is
redundant; that is an observation for `lessons.md`, not a finding.

## Maintainer Decisions

Recorded on 2026-10-28.

1. **The `408` timeout:** a small behavior-preserving testability refactor (R0), done in this issue. The spec's
   scope is extended for it (see the specification's Scope).
2. **Asserting log events:** not selected; the integration test asserts the `ERROR` line.
3. **Plan approved** with these answers.

## Prose-First AAA Review

Temporary prose, written before comparing with the code.

| Test (cases) | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_serve_the_health_check_without_a_token` | A request to the health check with no token. | Send it through the router. | `200`. | Only the status: the body belongs to the health check handler (row 4). Kept. |
| `it_should_reject_a_request_to_an_api_route_without_a_valid_token` (2) | A v1 request with no token, or with a wrong one. | Same. | `500` and the literal `unauthorized` or `token not valid` body. | The middleware's own cases were done in S1; this pins that the router applies it to v1 and feeds it the tokens. Kept. |
| `it_should_serve_an_api_route_when_the_request_carries_a_token_given_to_the_router` | A v1 request with the literal token that the test gave the router. | Same. | `200`. | The token is a literal in the test, not read from the configuration. Kept. |
| `it_should_generate_a_request_id_when_the_request_has_none` | A request with no `x-request-id`. | Same. | The response header parses as a UUID. | The value is random, so the property is the format. Kept. |
| `it_should_return_the_request_id_provided_by_the_client` | A request with `x-request-id: my-request-1`. | Same. | The response carries exactly that value. | Kept. |
| `it_should_compress_the_response_only_as_the_client_asks` (2) | A v1 request with `Accept-Encoding: gzip`, or without it. | Same. | `content-encoding: gzip`, or none. | Only the header: decoding is `tower-http`'s. Kept. |
| `it_should_answer_request_timeout_when_an_api_route_does_not_answer_within_the_timeout` | A route that never answers, a 50 ms timeout, a token. | `with_layers` through `oneshot`, awaited under an absolute 5 s deadline. | `408` and an empty body. | The deadline makes a missing timeout fail instead of hang (L8 fails after 5 s). Kept. |

Helpers: `received` reads the status, headers and body; `through_the_router` builds the container and the router for a
default tracker; `access_tokens` and `service_binding` hide only construction. The expected values are literals.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): `routes.rs` 58 / 67 lines
  (86.57%) before; 169 / 169 lines (100.00%), 277 / 277 regions and 26 / 26 functions after (the counts include
  the test module). An earlier version of this entry said 174 / 174 and 280 / 280: those were measured before the
  timeout assertion was simplified, which removed 5 lines; the figures here come from the S2 slice measurement.
  The package figures after S2 are in the slice measurement of `coverage-evidence.md`.
- **Honest note on coverage:** the request, response and failure log closures (lines 67-136) are now executed, because
  the unauthorized request goes through the router and a `5xx` response runs the server-error and failure arms. No
  test asserts their content (maintainer decision: no unit-level log assertions). An integration test
  (`tests/server/v1/contract/authentication.rs`) asserts one `ERROR ... API ... <request id>` line, which most likely
  matches only the server-error response event, because the `on_failure` event has no request id field. So the
  response arm has an integration-level assertion, and the `on_failure` closure has no assertion at any level; at unit
  level both are covered, not protected.
- **Tests:** 9 test cases in seven tests (two `rstest` tables of 2). No integration test was added or changed.
- **R0 refactor:** commit `refactor(axum-rest-api-server): [#2637] separate the router layers from the routes`. A pure
  move: the chain and its order are unchanged, `timeout` replaces the constant, and the prefix is a constant. The
  58 integration tests and the 65 unit tests of the moment passed unchanged before the tests were added.
- **Mutations** (each applied alone, stable toolchain, then reverted; the file was identical to the tested one
  afterwards): L1 killed by the public health check test, L2 by both rejection cases, L3 by the valid-token and the
  timeout tests, L4 by both request-id tests, L5 by the gzip case, L7 by the generated-id test, L8 by the timeout test
  (after its 5 s deadline) and L9 by the timeout test. **L6 survived, as predicted:** removing the inner
  `SetRequestIdLayer` changes nothing a client can see, because the outer one already sets the header, so `router`
  carries a redundant layer. **L10 survived:** the 5 s default is not asserted, because that would need a 5 s wait
  or a tautology. Both are recorded in `lessons.md`; neither is a finding. L8 and L9 were run again after the
  timeout assertion was simplified to status and body.
- **Validation:** `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `cargo clippy -p
  torrust-tracker-axum-rest-api-server --all-targets` (stable), `cargo machete --with-metadata`, `git diff
  --check` and `cargo test -p torrust-tracker-axum-rest-api-server` (74 unit and 58 integration tests) pass.
- **Review outcome:** the maintainer approved the completed file on 2026-10-28, including the R0 refactor.
