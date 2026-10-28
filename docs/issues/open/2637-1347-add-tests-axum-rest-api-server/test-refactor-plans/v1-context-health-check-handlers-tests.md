---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/health_check/handlers.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/health_check/handlers.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/routes-tests.md
---

# Health Check Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/health_check/handlers.rs`, ledger row 4 (rank 2), the last file of S2.

## Current State

- **Unit-only coverage:** 4 / 4 lines (100.00%), 4 / 4 regions, 2 / 2 functions. The lines run through the
  test-environment unit tests and the router tests of row 3, but **no test asserts the handler's answer**: the router
  tests assert only that the health check answers `200` without a token.
- **Existing tests:** none in the file. An integration test asserts the health check over HTTP.
- **Module-owned decision:** `health_check_handler` answers with a JSON `Report` whose status is `Ok`. The type and
  its serialization belong to `rest-api-protocol`; the package owns that the handler reports `Ok` and wraps it as
  JSON. The module documentation shows the response `{"status": "Ok"}`.
- **T1 hypothesis:** test candidate (small).

## Current Tests Review

No tests in the file, so R1 is skipped.

## Coverage And Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| The handler answers `200`, `application/json`, body `{"status":"Ok"}` | 3A unit | R2: call `health_check_handler().await`, convert it into a response, and assert the three parts as literals. |
| `Status::Error` | Not reachable | The handler never returns it, so there is no behavior to test; the variant belongs to the DTO. |

Coverage is already 100%, so this test adds protection, not lines. The plan says so to avoid claiming a coverage gain.

### Case For R2

| Call | Expected |
| --- | --- |
| `health_check_handler().await.into_response()` | `200`, `content-type: application/json`, body `{"status":"Ok"}` |

## Property-Test Decision

Not flagged and not selected: a constant answer.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests. | None. |
| R2 | TODO | Add the in-file test module with the one case. | Focused test, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | TODO | Integration increment: `none`. | The integration test keeps the wire contract. |
| R4 | TODO | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Named Mutations

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| H1 | `health_check_handler` | Report `Status::Error` instead of `Status::Ok`. | the one case (body) |

## Prose-First AAA Review

To be written for the added test before the completed-file review.

## Results

Pending.
