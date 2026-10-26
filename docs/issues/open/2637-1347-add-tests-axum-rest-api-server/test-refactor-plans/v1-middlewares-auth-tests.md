---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/middlewares/auth.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/middlewares/auth.rs
    - docs/security/analysis/reports/2026-09-04_rest-api-token-timing.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
---

# Authentication Middleware File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/middlewares/auth.rs`, ledger row 1 (rank 1, the highest risk).

## Current State

- **Unit-only coverage:** 71 / 103 lines (68.93%), 124 / 179 regions, 13 / 17 functions
  ([coverage evidence](../coverage-evidence.md#re-measurement-before-slice-s1), `0b4b2d0d3`). It was 27 / 83
  before #2653, which rewrote `extract_bearer_token_from_header` and added a middleware test.
- **Existing tests:** four unit tests: three of `authenticate`, the constant-time comparison, and the one added
  by #2653 for an unreadable `Authorization` header sent with a valid `token` query. The nine
  integration tests in `tests/server/v1/contract/authentication.rs` cover the middleware through a
  real server and are retained.
- **Module-owned decisions:** which token source wins (header over query), the shape of an accepted
  `Authorization` value (`Bearer <token>`, `Bearer` alone as an empty token, anything else rejected),
  which of the three failures answers, that an unreadable header is rejected (fixed by #2653), that a
  failure never reaches the handler, and that every configured token is compared without early exit.
- **T1 hypothesis:** property candidate.

## Current Tests Review

| Test | Smell | Conclusion |
| --- | --- | --- |
| `it_authenticates_a_configured_token` | Name lacks `should`; Act and Assert are one line, otherwise clean. | refactor (rename) |
| `it_rejects_tokens_that_differ_at_any_position_or_length` | A loop with the assertion inside: a failure does not name the token, and one test has five reasons to fail. | refactor (an `rstest` table, one case per token) |
| `it_authenticates_each_configured_token` | Name lacks `should`; covers only the second token, the first is covered by the first test. | refactor (rename; fold into one table of accepted tokens) |
| `it_rejects_an_unreadable_authorization_header_even_with_a_valid_token_query` (#2653) | Name lacks `should`; the router and the request are built inline and the response is read inline, which every R2 case repeats. The Act and the literal body assertion are visible. | refactor (rename; reuse the helper that R2 extracts for building the router and calling it) |

R1 therefore renames the tests to `it_should_*` and turns the loop and the pair of accepted-token tests into
two `rstest` tables; the #2653 test keeps its behavior and moves onto the shared helper once R2 extracts it. It
needs `rstest` as a dev-dependency (see Dependencies).

## Coverage And Decision Analysis

| Behavior group (lines at `abce11ec8`) | Classification | Decision |
| --- | --- | --- |
| `auth` token sources, precedence and the three exits (lines 82-101 at `0b4b2d0d3`) | 3A unit | R2: call the middleware through a minimal `Router` with a fixed `State` and a handler that answers `reached`, with `tower::ServiceExt::oneshot`. Cases below. |
| `extract_bearer_token_from_header` (108-127): absent, `Bearer` alone, wrong scheme, valid | 3A unit | Same router tests; the function is private and its outcomes are observable as the three failures. |
| `AuthError` to response and the three failure responses (144-145, 170-178) | 3A unit | Covered by the failure cases; each case asserts status, `content-type` and body as literals. |
| Query rejection before authentication (duplicate `token`) | 3A unit | One case: `?token=a&token=b` answers `400` with Axum's text, without reaching the handler. The package owns the order (the `Query` extractor runs first); the text is Axum's. |
| Non-UTF-8 `Authorization` value | 3A unit, already covered | Handed off as #2653 and fixed (rejected as `unknown token provided`, merged through #2656). The existing #2653 test covers it; R1 keeps it as a case of the same table. |

### Cases For R2 (one `rstest` table per outcome, literal expectations)

| Outcome | Header | Query | Expected |
| --- | --- | --- | --- |
| Reaches the handler | `Bearer <valid>` | none | `200`, body `reached` |
| Reaches the handler | none | `token=<valid>` | `200`, `reached` |
| Reaches the handler (header wins) | `Bearer <valid>` | `token=<invalid>` | `200`, `reached` |
| `token not valid` | `Bearer <invalid>` | none | `500`, `text/plain; charset=utf-8`, `Unhandled rejection: Err { reason: "token not valid" }` |
| `token not valid` | none | `token=<invalid>` and `token=` (empty) | same |
| `token not valid` (header wins) | `Bearer <invalid>` | `token=<valid>` | same |
| `token not valid` (empty token) | `Bearer` and `Bearer` followed by a space | none | same |
| `unknown token provided` | `Basic <valid>`, `bearer <valid>` (lower case), `Bearer<valid>` | none | same shape, `reason: "unknown token provided"` |
| `unknown token provided` (unreadable header, #2653) | `Bearer` followed by a non-ASCII byte | `token=<valid>` | same shape (existing test, moved onto the helper) |
| `unauthorized` | none | none | same shape, `reason: "unauthorized"` |
| `400` before authentication | none | `token=<valid>&token=<valid>` | `400`, Axum's text |

The handler is an expectation-free double: it records that it ran (an `AtomicBool`), so every rejected
case also asserts that the handler never ran, which proves the order "authenticate, then handle".

## Property-Test Decision

Flagged candidate: `authenticate(token, tokens)` equals "the token equals some configured token".
**Decision: not selected (maintainer, 2026-10-24).** The input space is a small set of discrete outcomes and the examples
above and in R1 already kill every plausible mutant of the fold (named below): a token that differs at
the first position, the last position, by a shorter or longer length, the empty token, and each of two
configured tokens. A `quickcheck` model would add a dependency for the same kills.

## Dependencies

- `rstest` (dev-dependency, approved 2026-10-24; latest stable `0.28.0` as in `http-core`): tables over tokens and headers.
- `tower` with the `util` feature: already a dev-dependency since #2653; nothing to add.
- No `quickcheck` (property decision above).

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | TODO | Rename the three tests to `it_should_*` and turn them into two `rstest` tables; add the `rstest` dev-dependency. | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, `cargo machete --with-metadata`, prose-first AAA review. |
| R2 | TODO | Add the middleware tests above with a minimal router and a recording handler (the `tower` `util` dev-dependency exists). | Same validation plus the named mutations. |
| R3 | TODO | Integration increment: `none` expected (the nine integration tests are retained). | The package has no boundary to add here. |
| R4 | TODO | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change that a test must catch; revert it exactly after the run and record
the result in Results.

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| M1 | `auth`, both-tokens arm | Use the query token instead of the header token. | "header wins" cases |
| M2 | `authenticate` | Combine the comparison results with a bitwise AND instead of OR. | accepted-token table |
| M3 | `authenticate` | Start the fold from `Choice::from(1)`. | every invalid-token case |
| M4 | `extract_bearer_token_from_header` | `Bearer` alone returns `Ok(None)` instead of an empty token. | `Bearer` alone case (`unauthorized` instead of `token not valid`) |
| M5 | `extract_bearer_token_from_header` | `starts_with("Bearer ")` to `starts_with("Bearer")`. | `Bearer<valid>` case |
| M6 | `AuthError::into_response` | Swap the `TokenNotValid` and `UnknownTokenProvided` arms. | the two reasons |
| M7 | `auth` | Return the handler's response after a failed authentication (call `next` anyway). | the recording handler |
| M8 | `extract_bearer_token_from_header` | Restore the #2653 behavior: map an unreadable value to "no header" (`Ok(None)`). | the unreadable-header case |

## Prose-First AAA Review

To be written for every added or changed test before the completed-file review.

## Results

Pending.

## Maintainer Decisions

Recorded on 2026-10-24 after the first plan review; the plan itself still needs the maintainer's explicit
approval before any test change (step 0 gate), which is requested again after the 2026-10-26 update.

1. **Non-UTF-8 `Authorization` header with a valid `token` query:** handed off as a finding; resolved by #2653
   (rejected as `unknown token provided`), which also added its test.
2. **Duplicate `token` query parameter:** pin the `400` (the `400` case above stays in R2).
3. **Property test for `authenticate`:** none; examples are enough.
4. **Dev-dependencies:** `rstest` 0.28.0 is approved; `tower` with `util` is already present.

## Update 2026-10-26

Issues #2653 and #2449 merged. The branch was rebased onto `develop` at `0b4b2d0d3`, the package was measured again
(see the evidence), and this plan was adjusted: current state, the existing-test review (a fourth test), the
unreadable-header case (covered), the dependency list (only `rstest` is new) and mutation M8.
