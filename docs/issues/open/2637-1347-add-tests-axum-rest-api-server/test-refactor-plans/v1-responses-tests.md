---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/responses.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/responses.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
---

# Shared Response Contract File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/responses.rs`, ledger row 5 (rank 3), the second and last file of S1.

## Current State

- **Unit-only coverage:** 8 / 40 lines (20.00%), 6 / 31 regions, 1 / 6 functions
  ([coverage evidence](../coverage-evidence.md#re-measurement-before-slice-s1), `0b4b2d0d3`). Only
  `unhandled_rejection_response` runs, through the authentication tests of the previous file. The 40 / 40
  integration-only lines go through a real server.
- **Existing tests:** none in the file.
- **Module-owned decision:** the status, `content-type` header and body of every response shared by the
  contexts: `ok_response` (`200`, JSON `{"status":"ok"}`), `bad_request_response` and its two wrappers
  (`400`, plain text), `disabled_by_configuration_response` (`409`, JSON `{"status":"error","reason":...}`),
  and `unhandled_rejection_response` (`500`, plain text with the Rust `Debug` form of the status, kept for
  backward compatibility with the old API). Clients and `rest-api-client` depend on these values.
- **T1 hypothesis:** test candidate.

## Current Tests Review

No tests in the file, so R1 is skipped.

## Coverage And Decision Analysis

| Behavior group (lines at `0b4b2d0d3`) | Classification | Decision |
| --- | --- | --- |
| `ok_response` (41-48) | 3A unit | R2: status `200`, `content-type: application/json`, body `{"status":"ok"}`. |
| `invalid_info_hash_param_response`, `invalid_auth_key_param_response` and `bad_request_response` (53-72) | 3A unit | R2: a table over the three functions: `400`, `text/plain; charset=utf-8`, the exact message with the given value interpolated. The two wrappers are tested through their own public function, since each owns a message. |
| `disabled_by_configuration_response` (80-87) | 3A unit | R2: `409`, `application/json`, body `{"status":"error","reason":"<reason>"}`; one case with a quote in the reason, to pin the JSON escaping. |
| `unhandled_rejection_response` (the covered function) | 3A unit | R2: `500`, `text/plain; charset=utf-8`, body `Unhandled rejection: Err { reason: "<reason>" }`; one case with a quote, to pin the `Debug` escaping the old API produced. Already exercised indirectly by the authentication tests, which assert the same text. |
| The `unwrap` panics documented on `ok_response` and `disabled_by_configuration_response` | Not testable | Serializing these fixed shapes cannot fail; no input reaches it. |

Observation for the record, not a finding: `ok_response` and `disabled_by_configuration_response` send
`application/json` without a charset, while the auth-key resource response sends
`application/json; charset=utf-8`. The tests pin each as it is; `rest-api-client` does not depend on the
difference.

### Cases For R2 (literal expectations, one table per shape)

| Function | Input | Expected status | Expected `content-type` | Expected body |
| --- | --- | --- | --- | --- |
| `ok_response` | none | `200` | `application/json` | `{"status":"ok"}` |
| `invalid_info_hash_param_response` | `abc` | `400` | `text/plain; charset=utf-8` | `Invalid URL: invalid infohash param: string "abc", expected a 40 character long string` |
| `invalid_auth_key_param_response` | `bad-key` | `400` | `text/plain; charset=utf-8` | `Invalid auth key id param "bad-key"` |
| `bad_request_response` | `custom` | `400` | `text/plain; charset=utf-8` | `custom` |
| `disabled_by_configuration_response` | `capability is off` | `409` | `application/json` | `{"status":"error","reason":"capability is off"}` |
| `disabled_by_configuration_response` | `say "no"` | `409` | `application/json` | `{"status":"error","reason":"say \"no\""}` |
| `unhandled_rejection_response` | `failed` | `500` | `text/plain; charset=utf-8` | `Unhandled rejection: Err { reason: "failed" }` |
| `unhandled_rejection_response` | `say "no"` | `500` | `text/plain; charset=utf-8` | `Unhandled rejection: Err { reason: "say \"no\"" }` |

Each test calls the function and reads the response through one helper that returns the whole
`(status, content type, body)` value; the expected value is a literal in the test.

## Property-Test Decision

Not flagged and not selected: the functions are formatting with a fixed shape; the cases above and the two
escaping cases already pin it. No `quickcheck`.

## Dependencies

None. `rstest` was added for the previous file; `tower` is not needed (the functions are called directly, and
the body is read with `axum::body::to_bytes`).

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests. | None. |
| R2 | TODO | Add the in-file `#[cfg(test)]` module with one `rstest` table per response shape and the helper. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | TODO | Integration increment: `none` expected. | The functions are pure; the 58 integration tests keep protecting the wire contract. |
| R4 | TODO | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change that a test must catch; revert it exactly after the run.

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| E1 | `ok_response` | Status `OK` to `CREATED`. | the `ok_response` case |
| E2 | `ok_response` | Content type to `application/json; charset=utf-8`. | the `ok_response` case |
| E3 | `invalid_info_hash_param_response` | Drop the word `expected` from the message (`a 40 character long string`). | the info-hash case |
| E4 | `invalid_auth_key_param_response` | Drop the quotes around the key. | the auth-key case |
| E5 | `bad_request_response` | Status `BAD_REQUEST` to `UNPROCESSABLE_ENTITY`. | all three `400` cases |
| E6 | `disabled_by_configuration_response` | Status `CONFLICT` to `FORBIDDEN`. | the two `409` cases |
| E7 | `disabled_by_configuration_response` | Use the `Debug` form for the body, as the `500` response does. | the two `409` cases |
| E8 | `unhandled_rejection_response` | Drop the `Unhandled rejection:` prefix and its space from the body. | the two `500` cases |

## Prose-First AAA Review

To be written for every added test before the completed-file review.

## Results

Pending.
