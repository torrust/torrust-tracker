---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/auth_key/handlers.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/auth_key/handlers.rs
    - packages/rest-api-application/src/v1/ports/auth_key.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-routes-tests.md
---

# Auth Key Handlers File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/auth_key/handlers.rs`, ledger row 6 (rank 4), the first file of S3.

## Current State

- **Unit-only coverage:** 0 / 56 lines at the S1 baseline, and now covered only as a side effect: the gating tests of
  `v1/routes.rs` reach the "service absent" arm of the handlers and the one happy path of `generate_auth_key_handler`.
  The 48 / 56 integration-only lines go through a real server and the real persistence. No test asserts a handler's
  mapping of a service outcome to a response at unit level.
- **Existing tests:** none in the file. `tests/server/v1/contract/context/auth_key.rs` has 18 integration tests.
- **Module-owned decisions:** each of the four handlers (`add`, `generate`, `delete`, `reload`) answers `409` when
  its service is absent, forwards its input to the service, and maps the service outcome to a response: `add` maps
  `DurationOverflow` and `InvalidKey` to `400` messages, `DisabledByConfiguration` to the disabled `409` and anything
  else to a `500`; `generate` maps every error to a `500`; `delete` maps `InvalidKey` to a `400` that names the path
  parameter (not the service's key) and anything else to a `500`; `reload` maps an error to a `500`.
- **T1 hypothesis:** test candidate.

## Current Tests Review

No tests in the file, so R1 is skipped.

## Coverage And Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Service absent: every handler answers the disabled `409` | 3A unit | R2: one table over the four handlers. |
| `add_auth_key_handler` outcome mapping (`Ok`, `DurationOverflow`, `InvalidKey`, `DisabledByConfiguration`, `Database`) | 3A unit | R2: a table of five outcomes with literal status, content type and body. |
| `generate_auth_key_handler` outcome mapping (`Ok`, any error) | 3A unit | R2: the success case, a `Database` error and an `InvalidKey` error that must both give the same `500` shape. |
| `delete_auth_key_handler` mapping (`Ok`, `InvalidKey`, other error) | 3A unit | R2: three cases; the `InvalidKey` case uses a service key different from the path parameter, to pin which one the message names. |
| `reload_keys_handler` mapping (`Ok`, error) | 3A unit | R2: two cases. |
| Forwarding of the input (form, seconds, key) | 3A unit | R2: the fake records each call; one test per handler asserts what the service received. |
| The wildcard arm of `add_auth_key_handler` for a future `AuthKeyError` variant | Collaborator-owned | Not reachable: `AuthKeyError` is `#[non_exhaustive]`, so nothing outside `rest-api-protocol` can build a fifth variant. The arm is exercised through `Database`, which it shares. |
| `AuthKeyErrorDisplay`'s `Debug` implementation | Not tested | It exists only to satisfy the `Error` bound of the response functions, and no caller prints it. A test of `format!("{:?}")` would add a line of coverage, not behavior. |
| Path and body extraction (`Path`, `Json`) | Not here | Axum's. Routing of the paths is pinned by the registration table of `v1/routes.rs`. |
| Persistence, key validity, durations | Collaborator-owned | `rest-api-application`, `rest-api-runtime-adapter`, `tracker-core`. |

### Boundary And Arrange

The tests call the handler functions directly with their extractors built by hand (`State(Some(Arc<service>))`,
`Json(form)`, `Path(60)`); no router and no socket. The service is `AuthKeyApiService::new(Box::new(fake))` over a
hand-written `AuthKeyPort` fake in the test module.

### The Fake (a shared helper within this file)

`FakeAuthKeyPort` has one responsibility: answer each port call with a scripted outcome and record the call. It holds
two plain function pointers (one returning `Result<AuthKey, AuthKeyError>` for `add` and `generate`, one returning
`Result<(), AuthKeyError>` for `delete` and `reload`) because `AuthKeyError` is not `Clone`, and an
`Arc<Mutex<Vec<String>>>` call log, for example `generate_key(60)` or `delete_key("abc")`. Its contract is exercised by
the forwarding tests; mutation F1 (the fake records nothing) proves it.

### Cases For R2 (literal expectations)

| Handler | Service outcome | Expected |
| --- | --- | --- |
| each of the four | service absent | `409`, `application/json`, `{"status":"err","reason":"private capability is disabled by configuration"}` |
| add | `Ok(key)` | `200`, `application/json; charset=utf-8`, the key as JSON (`{"key":"k1","valid_until":null,"expiry_time":null}`) |
| add | `DurationOverflow` (18446744073709551615) | `400`, `text/plain; charset=utf-8`, `Invalid URL: invalid auth key duration: "18446744073709551615"` |
| add | `InvalidKey { key: "bad key", reason: "too short" }` | `400`, `Invalid URL: invalid auth key: string "bad key", too short` |
| add | `DisabledByConfiguration` | the same disabled `409` |
| add | `Database("boom")` | `500`, `Unhandled rejection: Err { reason: "failed to add key: boom" }` |
| generate | `Ok(key)` | `200`, the key as JSON; the fake saw `generate_key(60)` |
| generate | `Database("boom")` and `InvalidKey` | `500`, `... "failed to generate key: boom" ...` and `... "failed to generate key: invalid key: \"x\", y" ...` |
| delete | `Ok(())` | `200`, `application/json`, `{"status":"ok"}`; the fake saw `delete_key("abc")` |
| delete | `InvalidKey { key: "service-key", .. }` with path `abc` | `400`, `Invalid auth key id param "abc"` |
| delete | `Database("boom")` | `500`, `... "failed to delete key: boom" ...` |
| reload | `Ok(())` | `200`, `{"status":"ok"}` |
| reload | `Database("boom")` | `500`, `... "failed to reload keys: boom" ...` |
| add | the form `{ key: Some("k1"), seconds_valid: Some(5) }` | the fake saw `add_key(Some("k1"), Some(5))` |

`AuthKey` has a deprecated field (`valid_until`); constructing it in a test needs an `#[allow(deprecated)]` with a
reason, as `stats/responses.rs` already does.

## Property-Test Decision

Not flagged and not selected: discrete outcome mapping, served by `rstest` tables.

## Dependencies

`async-trait = "0.1"` as a dev-dependency, the latest compatible stable line, matching `rest-api-application`. Reason:
implementing the `AuthKeyPort` trait, which is declared with `#[async_trait]`, requires the same attribute. No default
features exist to trim. `rstest` and `tokio` exist.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests. | None. |
| R2 | TODO | Add the dev-dependency and the in-file test module with the fake and the tables above. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `cargo machete --with-metadata`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | TODO | Integration increment: `none`. | The 18 integration tests keep the wire and persistence contract. |
| R4 | TODO | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Named Mutations

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| A1 | `add` | Swap the responses of the `DurationOverflow` and `InvalidKey` arms. | the two `400` cases |
| A2 | `add` | Map `DisabledByConfiguration` to `failed_to_add_key_response`. | the disabled case |
| A3 | `add` | Return `ok_response()` when the service is absent. | the absent case |
| A4 | `generate` | Forward `seconds_valid + 1`. | the forwarding case |
| A5 | `delete` | Name the service's key in the `400` message instead of the path parameter. | the `InvalidKey` case |
| A6 | `delete` | Answer `invalid_auth_key_param_response` for every error. | the `Database` case |
| A7 | `reload` | Report `failed to delete keys` in the error message. | the reload error case |
| A8 | `generate` | Map every error to `failed_to_add_key_response`. | both `generate` error cases |
| F1 | the fake | Record no calls. | the four forwarding assertions |

## Prose-First AAA Review

To be written for every added test before the completed-file review.

## Results

Pending.
