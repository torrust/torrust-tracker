---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/auth_key/handlers.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/auth_key/handlers.rs
    - packages/rest-api-application/src/v1/ports/auth_key.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-routes-tests.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-whitelist-handlers-tests.md
    - packages/axum-rest-api-server/src/v1/context/whitelist/handlers.rs
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

Cross-context difference, documented: this handler maps a service-reported `DisabledByConfiguration` to the disabled
`409`, while the whitelist handlers map the equivalent `WhitelistError` to a `500` (not reachable today). The
whitelist plan pins that behavior and explains it in
[Documented inconsistency](v1-context-whitelist-handlers-tests.md#documented-inconsistency-a-service-reported-capability-disabled-error-decision-pin-it).

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
| R2 | DONE | Add the dev-dependency and the in-file test module with the fake and the tables above. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `cargo machete --with-metadata`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | DONE | Integration increment: `none`. | The 18 integration tests keep the wire and persistence contract. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

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
| F1 | the fake | Record no calls. | the three forwarding assertions |

## Prose-First AAA Review

Temporary prose, written before comparing with the code.

| Test (cases) | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| Four `it_should_answer_conflict_when_..._the_private_capability_is_not_available` (one per handler) | No service: `State(None)` and the handler's input. | Call the handler. | The whole response equals the disabled `409`. | Four explicit tests, not a table over a dispatch enum, so each Act is a plain handler call. Kept. |
| `it_should_map_the_outcome_of_adding_a_key_to_a_response` (5) | A service whose fake answers the scripted outcome (a key, a duration overflow, an invalid key, a disabled capability, a database error). | `add_auth_key_handler` with a form. | The whole response equals the literal status, content type and body of that case. | The scripted outcome and the expected response sit side by side in each case. Kept. |
| `it_should_forward_the_form_to_the_service_when_adding_a_key` | A fake that records calls. | `add_auth_key_handler` with a form. | The call log equals `add_key(Some("k1"), Some(5))`. | Kept. |
| `it_should_answer_the_generated_key_and_forward_the_seconds_to_the_service_when_generating_a_key` | A recording fake that answers a key. | `generate_auth_key_handler` with `Path(60)`. | The response is the key JSON and the call log is `generate_key(60)`. | Response and forwarding are one behavior of one call, asserted as one value. Kept. |
| `it_should_answer_an_internal_server_error_whatever_the_error_when_generating_a_key` (2) | A fake that fails with a database error, or with an invalid key. | `generate_auth_key_handler`. | The same `500` shape with each error's text. | The second case pins that `generate` has no special mapping, unlike `add`. Kept. |
| `it_should_answer_ok_and_forward_the_key_to_the_service_when_deleting_a_key` | A recording fake that succeeds. | `delete_auth_key_handler` with the key `abc`. | `ok` and the call log `delete_key("abc")`. | Kept. |
| `it_should_name_the_path_parameter_when_the_service_rejects_the_key_to_delete` | A fake that rejects with a key different from the path parameter. | `delete_auth_key_handler` with `abc`. | The `400` names `abc`. | The two different keys make the pinned choice visible. Kept. |
| `it_should_answer_an_internal_server_error_when_deleting_a_key_fails_in_the_database` | A fake that fails with a database error. | `delete_auth_key_handler`. | The `500` with `failed to delete key: boom`. | Kept. |
| `it_should_answer_ok_when_the_keys_are_reloaded` and `it_should_answer_an_internal_server_error_when_reloading_the_keys_fails` | A fake that succeeds, or fails with a database error. | `reload_keys_handler`. | `ok`, or the `500` with `failed to reload keys: boom`. | Kept. |

Helpers: `disabled`, `ok`, `unhandled` and `bad_request` build the literal expected `Observed` values (one
visible literal each, with the shared `Observed::new`); the shared `observed` reads a response with the bounded body limit; `FakeAuthKeyPort`, `state_with`,
`state_answering` and `a_form` are the Arrange mechanics. The fake's contract (scripted answers and a call log) is
exercised by every forwarding test, and mutation F1 breaks it.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): `auth_key/handlers.rs` 0 / 56
  lines at the S1 baseline (a few lines ran as a side effect of the S2 gating tests); 261 / 264 lines (98.86%), 373 / 378
  regions and 62 / 63 functions after (the counts include the test module). The three uncovered lines (153-155) are
  `AuthKeyErrorDisplay`'s `Debug` implementation, which the plan decided not to test. Package unit-only lines after this
  file: 1,563 / 1,770 (88.31%), regions 2,315 / 2,708 (85.49%), functions 245 / 284 (86.27%), 94 unit tests.
- **Tests:** 13 test functions for the handlers (four absent-service tests, a table of 5, a forwarding test, a generate
  test, a table of 2, three delete tests and two reload tests), 18 test cases counting the tables; the package went from
  76 to 94 unit tests. No integration test was added or changed.
- **Mutations** (each applied alone, stable toolchain, then reverted; the file was identical to the tested one
  afterwards): A1 killed by the duration overflow case, A2 by the disabled case, A3 by the absent-service test of `add`,
  A4 by the generate forwarding test, A5 by the path-parameter test, A6 by the delete database test, A7 by the reload
  error test, A8 by both generate error cases, and F1 (the fake records nothing) by the three forwarding tests. A6 and
  A7 did not compile at first because they left an import unused; they were redone as mutants that drop the response
  of the original call, and then compiled and were killed.
- **Dependency:** `async-trait = "0.1"` as a dev-dependency (latest line `0.1.92`), as `rest-api-application` declares it;
  needed to implement the `#[async_trait]` port.
- **Clippy allows:** two `#[allow(clippy::unnecessary_wraps, reason = ...)]` on the scripted outcomes, which must have the
  fake's function-pointer type, and one `#[allow(deprecated, reason = ...)]` for the deprecated `valid_until` field.
- **Validation:** `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `cargo clippy -p
  torrust-tracker-axum-rest-api-server --all-targets` (stable), `cargo machete --with-metadata`, `git diff --check` and
  `cargo test -p torrust-tracker-axum-rest-api-server` (94 unit and 58 integration tests) pass.
- **Review outcome:** pending the maintainer's completed-file review.
