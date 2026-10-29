---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/auth_key/responses.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/auth_key/responses.rs
    - packages/axum-rest-api-server/src/v1/context/auth_key/handlers.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-auth-key-handlers-tests.md
---

# Auth Key Responses File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/auth_key/responses.rs`, ledger row 7 (rank 4), the second file of S3.

## Current State

- **Unit-only coverage:** 0 / 26 lines at the S1 baseline. After the previous file (row 6) the file is 26 / 26 lines,
  30 / 30 regions and 7 / 7 functions (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace, `--lib`),
  measured on this branch before any change to this file.
- **Existing tests:** none in the file. Its seven functions are reached, and their output asserted, by the handler
  tests of row 6, which compare each whole response (status, content type, body) with a literal.
- **Module-owned decisions:** the `200` JSON body of an `AuthKey`; the four `500` messages (`failed to
  generate/add/delete/reload key(s): <error>`, in the legacy `Unhandled rejection` text); and the two `400`
  messages for an invalid key and an invalid duration.
- **T1 hypothesis:** test candidate. This plan challenges it: row 6 already made the file fully covered with
  literal expectations.

## Current Tests Review

No tests in the file. The tests that assert it live in `handlers.rs` and are reviewed in the row 6 plan.

## Coverage And Decision Analysis

| Function (lines) | Asserted today by | Classification | Decision |
| --- | --- | --- | --- |
| `auth_key_response` (13-21) | the `add` "the added key" case and the `generate` test: `200`, `application/json; charset=utf-8`, the literal key JSON | 3A unit, already covered | No change. |
| `failed_to_generate_key_response` (26-28) | the `generate` error table (database error, invalid key): `500`, `failed to generate key: ...` | 3A unit, already covered | No change. |
| `failed_to_add_key_response` (32-34) | the `add` "database error" case: `failed to add key: boom` | 3A unit, already covered | No change. |
| `failed_to_delete_key_response` (38-40) | the `delete` database-failure test: `failed to delete key: boom` | 3A unit, already covered | No change. |
| `failed_to_reload_keys_response` (45-47) | the `reload` failure test: `failed to reload keys: boom` | 3A unit, already covered | No change. |
| `invalid_auth_key_response` (50-52) | the `add` "an invalid key" case: the literal `400` text with the key and the reason | 3A unit, already covered | No change. |
| `invalid_auth_key_duration_response` (55-57) | the `add` "a duration overflow" case: the literal `400` text with `18446744073709551615` | 3A unit, already covered | No change. |
| The `unwrap` panic documented on `auth_key_response` | none | Not testable | Serializing a valid `AuthKey` cannot fail; no input reaches the panic. |

No production line, region or function of the file is uncovered, so there is no uncovered group to classify as
hard to test, owned elsewhere or merely unselected.

### Why direct tests would not add protection

Direct tests of the seven functions would repeat the literals the handler tests already assert, through the same
public function, so they would be duplicates, not new protection. The handler tests are the better place: they
assert the response a client really receives, which would not be true of a test that calls the response builder
alone and could pass while a handler picked the wrong builder. Writing the duplicates would also double the places
to edit when a message changes. Evidence that the existing assertions pin every builder (named mutations, below)
replaces the suspicion that "covered" means "asserted".

## Named Mutations

Each is a temporary hand-made change to this file that a test must catch; revert it exactly after the run
(`cargo test -p torrust-tracker-axum-rest-api-server --lib auth_key`, stable toolchain). All were run before
this plan was written, as evidence for the no-change decision, and each failed the named test:

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| M1 | `auth_key_response` | Status `OK` to `CREATED`. | the `add` "the added key" case, the `generate` test and a `v1/routes.rs` test |
| M2 | `auth_key_response` | Content type to `application/json`. | the same tests |
| M3 | `failed_to_generate_key_response` | Message `failed to generate: ...`. | the `generate` error table |
| M4 | `failed_to_add_key_response` | Message `failed to add: ...`. | the `add` "database error" case |
| M5 | `failed_to_delete_key_response` | Message `failed to delete: ...`. | the `delete` database-failure test |
| M6 | `failed_to_reload_keys_response` | Message `failed to reload: ...`. | the `reload` failure test |
| M7 | `invalid_auth_key_response` | Drop the word `string` from the message. | the `add` "an invalid key" case |
| M8 | `invalid_auth_key_duration_response` | Message `invalid duration: ...`. | the `add` "a duration overflow" case |

## Property-Test Decision

Not flagged and not selected: the functions format a fixed shape; the literal cases already pin it.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests in the file. | None. |
| R2 | SKIPPED | No new unit tests: covered and pinned by the handler tests of row 6. | The named mutations M1-M8 fail the handler tests (recorded below). |
| R3 | DONE | Integration increment: `none`. | The 18 auth-key integration tests keep protecting the wire contract. |
| R4 | DONE | Record before and after. | Results below and `coverage-evidence.md`. |

## Prose-First AAA Review

No test is added or changed, so there is nothing to review.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): 0 / 26 lines at the S1
  baseline; 26 / 26 lines, 30 / 30 regions and 7 / 7 functions with no change to this file (covered by the row 6
  tests).
- **Mutations:** M1-M8 were applied alone and reverted exactly (the working tree was clean afterwards).
  Every one made at least one test fail on an assertion, not on a compile error: M1 failed the add, generate and
  `v1/routes.rs` tests; M2 the add and generate tests; M3 both cases of the generate error table; M4, M5 and M7 their
  one case each; M6 the reload test; M8 the duration-overflow case.
- **Decision:** no change, recorded as documentation. The maintainer approved the no-change decision on 2026-10-29.
