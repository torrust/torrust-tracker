---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/whitelist/handlers.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/whitelist/handlers.rs
    - packages/rest-api-application/src/v1/ports/whitelist.rs
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - packages/axum-rest-api-server/src/v1/context/auth_key/handlers.rs
    - packages/rest-api-protocol/src/v1/context/whitelist/resources/whitelist.rs
    - packages/rest-api-protocol/src/v1/context/auth_key/resources/auth_key.rs
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/lessons.md
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-auth-key-handlers-tests.md
---

# Whitelist Handlers File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/whitelist/handlers.rs`, ledger row 9 (rank 4), the fourth file of S3. It
repeats the pattern the maintainer approved for the auth-key handlers
([plan](v1-context-auth-key-handlers-tests.md)): the handlers are called directly, over a hand-written fake of the
application port.

## Current State

- **Unit-only coverage:** 26 / 37 lines (70.27%), 23 / 50 regions, 7 / 7 functions (stable Rust 1.99.1,
  `cargo-llvm-cov 0.6.16`, clean workspace, `--lib`), measured on this branch. The `v1/routes.rs` tests reach the
  "service absent" arm of each handler. The uncovered lines are the invalid-info-hash arm,
  the service call and its success and failure arms of the three handlers.
- **Existing tests:** none in the file. `tests/server/v1/contract/context/whitelist.rs` has 13 integration tests that
  go through a real server and the real persistence.
- **Module-owned decisions:** each handler (`add`, `remove`, `reload`) answers `409` when its service is absent;
  `add` and `remove` reject an invalid info-hash parameter with a `400` before calling the service, parse the valid
  one and forward the parsed value; every service failure becomes a `500` in the legacy `Unhandled rejection` text
  with an operation prefix; success is `200 {"status":"ok"}`. Whether an info hash is valid is owned by
  `torrust-info-hash`; the whitelist rules and persistence are owned by `tracker-core` and the runtime adapter.
- **T1 hypothesis:** test candidate.

## Current Tests Review

No tests in the file, so R1 is skipped.

## Coverage And Decision Analysis

| Behavior group (lines) | Classification | Decision |
| --- | --- | --- |
| Service absent in the three handlers | 3A unit | R2: one test per handler: `409`, JSON, `{"status":"err","reason":"listed capability is disabled by configuration"}`, and the fake is not reachable. |
| Invalid info hash in `add` and `remove` | 3A unit | R2: one test per handler: `400`, plain text, the literal message with the given string, and no service call recorded. |
| Success in `add` and `remove` | 3A unit | R2: `200`, JSON, `{"status":"ok"}`, and the service receives the parsed hash (call log). |
| Failure in `add`, `remove` and `reload` | 3A unit | R2: a table of cases per handler: `500`, plain text, `Unhandled rejection: Err { reason: "<prefix>: boom" }`. The `add` table also pins what a service-reported `DisabledByConfiguration` becomes (see the documented inconsistency). |
| Reload success and failure | 3A unit | R2: `200` ok; `500` with `failed to reload whitelist: boom`. |
| The real service and persistence behind the port | Collaborator-owned | Not tested here: `rest-api-application` and `rest-api-runtime-adapter`; the 13 integration tests keep protecting the wiring. |

### Documented inconsistency: a service-reported "capability disabled" error (decision: pin it)

Maintainer decision on 2026-11-03: option (a), pin the current behavior with one `add` failure case and link every
place that explains it, so the difference between the two contexts stays documented in one traceable chain.

- **Whitelist (this file):** `add_torrent_to_whitelist_handler` and `remove_torrent_from_whitelist_handler` turn every
  service failure into a `500`, including `WhitelistError::DisabledByConfiguration`, which becomes
  `failed to whitelist torrent: listed capability is disabled by configuration`. Source:
  [`whitelist/handlers.rs`](../../../../../packages/axum-rest-api-server/src/v1/context/whitelist/handlers.rs) (the `Err(e)` arms) and
  [`WhitelistError`](../../../../../packages/rest-api-protocol/src/v1/context/whitelist/resources/whitelist.rs).
- **Auth keys (row 6):** `add_auth_key_handler` maps `AuthKeyError::DisabledByConfiguration` to the disabled `409`. Source:
  [`auth_key/handlers.rs`](../../../../../packages/axum-rest-api-server/src/v1/context/auth_key/handlers.rs) (the
  `DisabledByConfiguration` arm), pinned by the "a capability disabled in the service" case of
  `it_should_map_the_outcome_of_adding_a_key_to_a_response` ([plan](v1-context-auth-key-handlers-tests.md)), and
  [`AuthKeyError`](../../../../../packages/rest-api-protocol/src/v1/context/auth_key/resources/auth_key.rs).
- **Why it is not a finding:** nothing in the workspace returns `WhitelistError::DisabledByConfiguration` from the
  service; the only use is the handler's own absent-service response, and the router answers that `409` before any
  service exists. The state "a whitelist service that reports the capability disabled" is not reachable today, so the
  `500` cannot reach a client.
- **Why pin it anyway:** the port contract allows the variant (`WhitelistError` is a public `#[non_exhaustive]` enum
  returned by `WhitelistCommandPort`), and the handler reacts in a defensible, deterministic way. The test is the
  second line of defense, as in the routes plan. If the two contexts are ever aligned, the maintainer changes this
  case on purpose.
- **Where it is recorded:** this plan, the in-file doc comment of the pinning test (which names the auth-key case),
  the `lessons.md` follow-up row, and the same references from the auth-key handlers plan.

### Cases for R2 (literal expectations)

| Handler | Situation | Expected status | Content type | Body |
| --- | --- | --- | --- | --- |
| add / remove / reload | service absent | `409` | `application/json` | `{"status":"err","reason":"listed capability is disabled by configuration"}` |
| add / remove | info hash `abc` | `400` | `text/plain; charset=utf-8` | `Invalid URL: invalid infohash param: string "abc", expected a 40 character long string` |
| add / remove / reload | success | `200` | `application/json` | `{"status":"ok"}` |
| add | database failure `boom` | `500` | `text/plain; charset=utf-8` | `Unhandled rejection: Err { reason: "failed to whitelist torrent: boom" }` |
| add | service reports the capability disabled (pins the documented inconsistency with auth keys) | `500` | `text/plain; charset=utf-8` | `Unhandled rejection: Err { reason: "failed to whitelist torrent: listed capability is disabled by configuration" }` |
| remove | database failure `boom` | `500` | `text/plain; charset=utf-8` | `Unhandled rejection: Err { reason: "failed to remove torrent from whitelist: boom" }` |
| reload | database failure `boom` | `500` | `text/plain; charset=utf-8` | `Unhandled rejection: Err { reason: "failed to reload whitelist: boom" }` |

## Test Design

- **Fake:** a `FakeWhitelistPort` implementing `WhitelistCommandPort`, with one scripted outcome (a function pointer
  returning `Result<(), WhitelistError>`) and an `Arc<Mutex<Vec<Call>>>` call log, as in the auth-key tests.
  `Call` is a test-local enum (`AddTorrent(InfoHash)`, `RemoveTorrent(InfoHash)`, `Reload`), typed after Copilot's review
  of #2702 (finding F1) instead of formatted strings. Forwarding is asserted through the log. The fake
  needs only one outcome type, so it is smaller than the auth-key one.
- **Helpers:** the `Observed` value (status, content type, body) and a bounded body read. This plan first wrote its
  own copy, the seventh in the package; Copilot's review of #2702 (finding F2) asked for a shared one, and the
  maintainer decided on 2026-11-03 to extract it in the same pull request: `test_support` now holds `read_body`,
  `Observed` and `observed`, and this file and the six others use them (see `lessons.md`).
- **Lint allows:** the scripted success outcome needs one `#[allow(clippy::unnecessary_wraps, reason = ...)]`.

## Property-Test Decision

Not flagged and not selected: info-hash parsing is owned by `torrust-info-hash`; the handlers only choose a response.

## Dependencies

None: `async-trait` and `rstest` are already dev-dependencies.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests in the file. | None. |
| R2 | DONE | Add the in-file `#[cfg(test)]` module with the fake, the helpers and the cases above. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | DONE | Integration increment: `none` expected. | The 13 whitelist integration tests keep protecting the wire contract. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change a test must catch; revert it exactly after the run.

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| W1 | `add` | Success arm answers the disabled response. | the `add` success test |
| W2 | `add` | Failure arm uses the remove-failure response. | the `add` failure cases |
| W3 | `add` | Invalid-hash arm answers `ok_response`. | the `add` invalid-hash test |
| W4 | `add` | Forward a different hash to the service. | the `add` forwarding assertion |
| W5 | `remove` | Call `add_torrent` instead of `remove_torrent`. | the `remove` forwarding assertion |
| W6 | `remove` | Failure arm uses the whitelist-failure response. | the `remove` failure case |
| W7 | `remove` | Invalid-hash arm answers `ok_response`. | the `remove` invalid-hash test |
| W8 | `reload` | Failure arm uses the remove-failure response. | the `reload` failure case |
| W9 | `reload` | Success arm answers the disabled response. | the `reload` success test |
| W10 | `disabled_response` | Capability `"listed"` to `"private"`. | the three absent-service tests |

## Prose-First AAA Review

Temporary prose, written before comparing with the code.

| Test (cases) | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| Three `it_should_answer_conflict_when_..._the_listed_capability_is_not_available` (one per handler) | No service: `State(None)` and the handler's input. | Call the handler. | The whole response equals the disabled `409`. | Three explicit tests, as for the auth-key handlers. Kept. |
| Two `it_should_reject_an_invalid_info_hash_without_calling_the_service_when_...` (add, remove) | A recording fake that would succeed, and the parameter `abc`. | Call the handler. | The whole response is the `400` with `abc`, and the call log is empty. | The empty log is what proves the service is not reached. Kept. |
| Two `it_should_answer_ok_and_forward_the_parsed_info_hash_to_the_service_when_...` (add, remove) | A recording fake that succeeds, and a valid 40-character hash. | Call the handler. | `ok`, and the log holds the call with the hash. | Response and forwarding are one behavior of one call. Kept. |
| `it_should_answer_ok_when_the_whitelist_is_reloaded` | A recording fake that succeeds. | `reload_whitelist_handler`. | `ok` and the log `reload`. | Kept. |
| `it_should_answer_an_internal_server_error_whatever_the_error_when_whitelisting_a_torrent` (2) | A fake that fails with a database error, or reports the capability disabled. | `add_torrent_to_whitelist_handler`. | The same `500` shape with each error's text. | The second case pins the documented inconsistency with the auth-key handlers; its doc comment names the plan. Kept. |
| `it_should_answer_an_internal_server_error_when_removing_a_torrent_fails` | A fake that fails with a database error. | `remove_torrent_from_whitelist_handler`. | The `500` with `failed to remove torrent from whitelist: boom`. | Kept. |
| `it_should_answer_an_internal_server_error_when_reloading_the_whitelist_fails` | A fake that fails with a database error. | `reload_whitelist_handler`. | The `500` with `failed to reload whitelist: boom`. | Kept. |

Helpers: `disabled`, `ok`, `unhandled` and `invalid_info_hash` build the literal expected values with `Observed::new`;
the shared `observed` reads a response with the bounded limit; `FakeWhitelistPort`, `state_with`, `state_answering`, `param` and
`recorded` are the Arrange and Assert mechanics.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): `whitelist/handlers.rs`
  0 / 37 lines at the S1 baseline (26 / 37 as a side effect of the S2 gating tests before this file); 198 / 198 lines
  (100.00%), 334 / 334 regions and 47 / 47 functions after (the counts include the test module). Package unit-only
  lines after this file: 1,744 / 1,931 (90.32%), regions 2,638 / 2,992 (88.17%), functions 288 / 324 (88.89%),
  106 unit tests.
- **Tests:** 12 test functions (three absent-service, two invalid-hash, three success with forwarding, a table of 2
  for `add` failures, one `remove` failure and one `reload` failure), 13 test cases counting the table; the package went
  from 94 to 106 unit tests. No integration test was added or changed.
- **Mutations** (each applied alone, stable toolchain, then reverted; the file was identical to the tested one
  afterwards): W1 killed by the `add` success test, W2 by both `add` failure cases, W3 by the `add` invalid-hash test,
  W4 by the `add` forwarding assertion, W5 by the `remove` forwarding assertion, W6 by the `remove` failure test, W7 by
  the `remove` invalid-hash test, W8 by the `reload` failure test, W9 by the `reload` success test, W10 by the three
  absent-service tests. W2, W4, W6 and W8 did not compile at first (an unused import or variable under
  `unused = "deny"`); they were redone as mutants that keep the original call (`drop` of the response, or `black_box`
  of the hash) and were then killed.
- **Documented inconsistency:** pinned as agreed, linked from this plan, the auth-key handlers plan and `lessons.md`
  (see [Documented inconsistency](#documented-inconsistency-a-service-reported-capability-disabled-error-decision-pin-it)).
- **Dependency:** none new. **Clippy allows:** one `#[allow(clippy::unnecessary_wraps, reason = ...)]` on the scripted
  success outcome. A first clippy run flagged `assert!(... .is_empty())`; it became a comparison with an empty `Vec`.
- **Validation:** `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `cargo clippy -p
  torrust-tracker-axum-rest-api-server --all-targets` (stable), `cargo machete --with-metadata`, `git diff --check` and
  `cargo test -p torrust-tracker-axum-rest-api-server` (106 unit and 58 integration tests) pass.
- **Review outcome:** the maintainer approved the plan with option (a) on 2026-11-03 and the completed file on 2026-11-03.
