---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/routes.rs
status: proposed
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
| The `408` timeout (`HandleErrorLayer` closure, line 144, plus the 5 s constant) | **Question 1** | See below. |
| Request, response and failure log events (lines 67-136) | **Question 2** | See below. |
| The server-error log arm and the failure closure (100, 102, 124-129, 136) | Coverage side effect | They run for any `5xx` response, so the unauthorized request of the first group executes them. The plan does not claim they are asserted: their content is observability, and the log text is asserted by an integration test (`tests/server/v1/contract/authentication.rs`, the `ERROR ... API ... <request id>` line). |

### Boundary And Arrange

The tests call `routes::router(&container, tokens, &service_binding)` and send requests with
`tower::ServiceExt::oneshot`. The container comes from `test_support::api_container(&ephemeral())`; the tokens are a
literal `AccessTokens` (`test-token`) built in the test, not read from the configuration; the binding is
`ServiceBinding::new(Protocol::HTTP, 127.0.0.1:1212)`. No socket is opened.

### Cases For R2 (literal expectations)

| Request | Expected |
| --- | --- |
| `GET /api/health_check`, no token | `200` |
| `GET /api/v1/stats`, no token | `500`, `text/plain; charset=utf-8`, `Unhandled rejection: Err { reason: "unauthorized" }` |
| `GET /api/v1/stats?token=wrong` | `500`, the `token not valid` body |
| `GET /api/v1/stats?token=test-token` | `200` |
| `GET /api/health_check`, no `x-request-id` | response header `x-request-id` is a valid UUID (parsed with `uuid`) |
| `GET /api/health_check`, `x-request-id: my-request-1` | response header `x-request-id` equals `my-request-1` |
| `GET /api/v1/stats?token=test-token`, `Accept-Encoding: gzip` | `content-encoding: gzip` |
| the same without `Accept-Encoding` | no `content-encoding` header |

## Property-Test Decision

Not flagged and not selected: the layers are wiring with a handful of enumerable outcomes.

## Dependencies

None new: `uuid` (already a dev-dependency) parses the generated id, and `rstest`, `tower` and the helper exist.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests. | None. |
| R2 | TODO | Add the in-file test module with the cases above. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | TODO | Integration increment: `none` expected. | The integration tests keep the wire and log contract. |
| R4 | TODO | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

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

`router` adds `SetRequestIdLayer::x_request_id(MakeRequestUuid)` twice. If L6 survives, the inner one is
redundant; that is an observation for `lessons.md`, not a finding.

## Maintainer Questions

1. **The `408` timeout.** The timeout is a private 5 s constant and every production route answers quickly, so a unit
   test has no way to make a request outlast it. Options: (a) record it as not testable at unit level (the
   closure at line 144 and the constant stay uncovered by an assertion; the reason and owner go in the plan),
   (b) a small testability refactor that lets a test pass the base routes and the timeout (`router` would call it
   with the real routes and the 5 s default), which touches production code and needs your approval, or (c) a
   package integration test with a deliberately slow route (it would still need the same seam). Which one?
2. **Asserting the log events.** The global capture helper of `test-helpers` installs a process-wide subscriber and
   is shared with other tests; in a unit-test binary that already sets one up, a second `init()` would panic. A
   scoped capture needs `tracing-subscriber` as a new dev-dependency. Options: (a) do not assert logs at unit level
   (observability; the integration test asserts the `ERROR` line), (b) add `tracing-subscriber` as a dev-dependency
   and assert the request, response and failure events. Which one?

## Prose-First AAA Review

To be written for every added test before the completed-file review.

## Results

Pending.
