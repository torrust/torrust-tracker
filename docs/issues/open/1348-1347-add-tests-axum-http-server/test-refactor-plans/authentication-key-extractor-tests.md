---
doc-type: test-refactor-plan
issue: 1348
package: torrust-tracker-axum-http-server
target-file: packages/axum-http-server/src/v1/extractors/authentication_key.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/axum-http-server/src/v1/extractors/authentication_key.rs
    - packages/axum-http-server/tests/server/v1/contract/configured_as_private.rs
    - docs/issues/open/1348-1347-add-tests-axum-http-server/coverage-evidence.md
---

# Authentication-Key Extractor Test Refactor Plan

Follow the shared [purpose, quality goals, and plan structure](README.md). This plan applies only
to `packages/axum-http-server/src/v1/extractors/authentication_key.rs`.

## Phase 1 — Identify Problems

### Strengths to preserve

1. The current `parse_key` failure test is fast and deterministic: it needs no router, listener,
   clock, database, or mock.
2. The HTTP-protocol error response is asserted through its stable failure-reason text rather than
   unstable caller-location data.
3. Existing real-server private-mode tests exercise malformed path keys through both announce and
   scrape routes, as recorded in the related-artifact link.

### P1 — The failure test does not name its protocol classification

**Problem.** The current test name says a key cannot be parsed, but not that malformed path text
maps to the invalid-key-format authentication failure.

**Why it matters.** A syntactically valid but unregistered key is a different contract. Naming the
protocol classification prevents those two failure modes from being conflated.

**Opportunity.** Rename the test to state the invalid-key-format mapping while retaining the direct
`parse_key` seam and stable failure-reason assertion.

### P2 — The valid key branch lacks a direct test

**Problem.** The module verifies invalid input but not that a syntactically valid path key produces
a `Key`.

**Why it matters.** The success branch is a small, deterministic contract that complements the
existing invalid-format mapping test.

**Opportunity.** Add one fixed valid-key test that asserts the parsed key equals the expected domain
value. Do not test `KeyParam::value()` cloning as a separate behavior.

### P3 — The local helper name does not state its partial assertion

**Problem.** `assert_error_response` checks that a failure reason contains a stable fragment.

**Why it matters.** The generic helper name hides the intentionally partial contract and can
encourage vague assertions.

**Opportunity.** Rename it to `assert_failure_reason_contains`, retaining the current full debug
diagnostic when the assertion fails.

### P4 — Extractor wire behavior is covered only generically at a higher level

**Problem.** The `FromRequestParts` implementation turns extraction failure into a bencoded HTTP
`200 OK` response. Real-server private-mode tests prove a generic authentication failure through
both routes, but do not explicitly retain the extractor's invalid-key-format classification.

**Why it matters.** The package owns this HTTP response boundary, while general bencode serialization
belongs to `http-protocol`.

**Opportunity.** Add one minimal in-process router test only if maintainer review accepts the
specific wire-level contract: malformed `{key}` becomes HTTP `200 OK` and a valid bencoded error
whose failure reason contains the stable invalid-format classification.

### P5 — Module documentation contradicts the implementation

**Problem.** The documentation says the extractor returns a `500` response, but the implementation
returns `StatusCode::OK`; the same module later documents HTTP `200` for authentication failures.

**Why it matters.** It misstates the BitTorrent wire contract and conflicts with the protocol error
response documentation.

**Opportunity.** Correct the stale `500` text as a documentation-only increment. Avoid copying
sample messages containing unstable source locations.

### P6 — Low coverage should not require synthetic Axum rejection tests

**Problem.** The coverage report identifies lower coverage in this file, including the Axum trait
entry point and rejection mapping.

**Why it matters.** Constructing every `PathRejection` variant would test Axum internals more than a
tracker-owned observable contract.

**Opportunity.** Assess direct `custom_error` tests only when a concrete route-parameter regression
identifies a stable missing classification. Otherwise record the deferral.

## Phase 2 — Proposed Refactorings

### R1 — Correct the extractor response-status documentation

- **Status:** DONE
- **Priority:** High impact / trivial effort
- **Addresses:** P5
- **Change:** Replace the stale `500` wording with the actual bencoded HTTP `200 OK` failure
  response contract.
- **Guardrails:** Documentation only. Do not assert or document unstable caller-location strings as
  response contracts.
- **Done when:** module documentation agrees with its implementation and the protocol error type.

### R2 — Make malformed-key mapping explicit

- **Status:** TODO
- **Priority:** Medium impact / trivial effort
- **Addresses:** P1, P3
- **Change:** Rename the failure test for the invalid-key-format mapping and rename the local helper
  to `assert_failure_reason_contains`.
- **Guardrails:** Preserve the direct `parse_key` seam; assert only stable semantic text, not full
  dynamic failure messages.
- **Done when:** the test and helper names reveal the actual failure contract.

### R3 — Add a valid-key parsing contract

- **Status:** TODO
- **Priority:** Medium impact / trivial effort
- **Addresses:** P2
- **Change:** Add one deterministic test with a fixed syntactically valid key, asserting the parsed
  domain `Key` equals the expected value.
- **Guardrails:** Do not use generated values, test clone implementation details, or duplicate
  authentication-service tests for unregistered/expired keys.
- **Done when:** valid and invalid format branches each have one focused direct test.

### R4 — Assess the in-process malformed-key wire contract

- **Status:** TODO
- **Priority:** Medium impact / medium effort
- **Addresses:** P4
- **Change:** Determine whether one minimal router test adds value beyond the real-server announce
  and scrape contracts. If it does, assert HTTP `200 OK`, valid bencode, and the stable
  invalid-key-format failure reason for one malformed path key.
- **Guardrails:** Use `oneshot`; do not create a listener, database, service mock, timeout, or
  log-capture assertion. Do not test both routes because the extractor is shared.
- **Done when:** the plan records either one behavior-justified test or a concrete no-change
  rationale referencing the higher-level coverage.

### R5 — Assess direct Axum path-rejection coverage

- **Status:** TODO
- **Priority:** Low impact / medium effort
- **Addresses:** P6
- **Change:** Assess whether a concrete tracker-owned path-rejection contract remains untested after
  R4.
- **Guardrails:** Do not manufacture Axum rejection variants merely to improve coverage.
- **Done when:** the plan records a behavior-justified test or a deferral rationale.

## Progress Tracking

### Plan Checklist

- [x] Phase 1 findings reviewed against the current file and package coverage evidence
- [x] Phase 2 refactorings ordered by impact and effort
- [x] Maintainer approved implementation of R1
- [x] R1 implemented, reviewed, and validated
- [ ] Maintainer approved implementation of R2
- [ ] R2 implemented, reviewed, and validated
- [ ] Maintainer approved implementation of R3
- [ ] R3 implemented, reviewed, and validated
- [ ] R4 assessment completed and decision recorded
- [ ] R5 assessment completed and decision recorded
- [ ] Maintainer reviewed all approved changes
- [ ] Plan completed and ready for commit

### Progress Log

- 2026-09-02 - GitHub Copilot - Created the proposed plan from the extractor tests, current package
  coverage evidence, HTTP-protocol response contract, and existing private-mode real-server tests.
  No refactoring has been implemented.
- 2026-09-02 - User/maintainer - Approved R1 after reviewing the documentation correction.
- 2026-09-02 - GitHub Copilot - Completed R1. The extractor documentation now states the actual
  bencoded HTTP `200 OK` failure-response contract.

### Validation Evidence

| Increment          | Status | Evidence                                                                                                                            |
| ------------------ | ------ | ----------------------------------------------------------------------------------------------------------------------------------- |
| Plan documentation | DONE   | `linter markdown`, `linter cspell`, and `git diff --check` passed after plan creation.                                              |
| R1                 | DONE   | `cargo fmt --all -- --check`, package library tests (32 passed), `linter markdown`, `linter cspell`, and `git diff --check` passed. |
| R2                 | TODO   | Not started.                                                                                                                        |
| R3                 | TODO   | Not started.                                                                                                                        |
| R4                 | TODO   | Not started.                                                                                                                        |
| R5                 | TODO   | Not started.                                                                                                                        |

## Non-Goals

- Do not test every Axum `PathRejection` variant or Axum path-extraction internals.
- Do not duplicate real-server malformed-key, missing-key, unregistered-key, or scrape-zeroed-data
  contracts.
- Do not add a listener, database, mocks, wall-clock waits, retries, or log assertions.
- Do not share the tiny local assertion helper across extractor files solely to remove duplication.
- Do not refactor `KeyParam::value()` or other production implementation details through this test
  plan.

## Validation Per Approved Increment

- `cargo fmt --all -- --check`
- `cargo test -p torrust-tracker-axum-http-server --lib`
- `git diff --check`
- `linter markdown` when this plan changes
- Refresh `coverage-evidence.md` only after an approved behavior-adding test.
