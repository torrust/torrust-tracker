---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/responses.rs
status: done
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
  (`400`, plain text), `disabled_by_configuration_response` (`409`, JSON `{"status":"err","reason":...}`),
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
| `disabled_by_configuration_response` (80-87) | 3A unit | R2: `409`, `application/json`, body `{"status":"err","reason":"<reason>"}`; one case with a quote in the reason, to pin the JSON escaping. |
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
| `disabled_by_configuration_response` | `capability is off` | `409` | `application/json` | `{"status":"err","reason":"capability is off"}` |
| `disabled_by_configuration_response` | `say "no"` | `409` | `application/json` | `{"status":"err","reason":"say \"no\""}` |
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
| R2 | DONE | Add the in-file `#[cfg(test)]` module with one `rstest` table per response shape and the helper. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | DONE | Integration increment: `none` expected. | The functions are pure; the 58 integration tests keep protecting the wire contract. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

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

Temporary prose, written before comparing with the code.

| Test (cases) | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_answer_ok_with_a_json_status_ok_body` | Nothing: the response takes no input. | Call `ok_response`. | The whole `(status, content type, body)` equals `200`, `application/json`, `{"status":"ok"}`. | Kept. |
| `it_should_answer_a_bad_request_with_the_message_as_plain_text` (3 cases) | The `400` response built by one of the three functions, and the message it must carry. | Read the response through `sent`. | `400`, plain text, the literal message. | The case builds the response in the table (the Act of each function), so the response under test stays visible in its case. Kept. |
| `it_should_answer_conflict_with_the_reason_in_a_json_error_body_when_a_capability_is_disabled` (2) | A reason, plain or with quotes. | Call `disabled_by_configuration_response`. | `409`, JSON, the literal body, including the escaped quotes. | Kept. |
| `it_should_answer_an_internal_server_error_with_the_legacy_debug_text_of_the_reason` (2) | A reason, plain or with quotes. | Call `unhandled_rejection_response`. | `500`, plain text, the literal `Unhandled rejection: Err { reason: "..." }`, with the `Debug` escaping. | Kept. |

`sent` is the only helper: it hides the mechanics of reading the header and the body, and the expected value is a
literal in each test.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): `v1/responses.rs`
  8 / 40 lines (20.00%) before; 66 / 66 lines (100.00%), 69 / 69 regions and 10 / 10 functions after (the counts
  include the test module). No production line of the file is uncovered. Package unit-only lines after S1:
  921 / 1,305 (70.57%), regions 1,393 / 2,006 (69.44%), functions 114 / 196 (58.16%), 46 unit tests.
- **Tests:** three tables (3, 2 and 2 cases) and one single test: 8 test cases. No
  integration test was added or changed.
- **A correction made while writing the tests:** the plan expected the `409` body to read
  `{"status":"error",...}`. The code serializes `ActionStatus::Err` with `rename_all = "snake_case"`, so the
  value is `"err"`; the first run failed on that. The tests and this plan now pin `"err"`, which is the
  behavior clients get today. It is not a finding.
- **Mutations** (each applied alone, stable toolchain, then reverted; the file was identical to the tested one
  afterwards): E1 killed by the `ok_response` test, E2 by the same test, E3 by the info-hash case, E4 by the
  auth-key case, E5 by all three `400` cases, E6, E7 and E8 by both cases of their table. Every named mutation is
  killed. The first E6 attempt matched two places (the test module also names `StatusCode::CONFLICT`), so it
  was anchored on the production line and rerun.
- **Validation:** `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `cargo clippy -p
  torrust-tracker-axum-rest-api-server --all-targets` (stable), `cargo machete --with-metadata`, `git diff
  --check` and `cargo test -p torrust-tracker-axum-rest-api-server` (46 unit and 58 integration tests) pass.
- **Review outcome:** the maintainer approved the completed file on 2026-10-27.
