---
doc-type: test-refactor-plan
issue: 1348
package: torrust-tracker-axum-http-server
target-file: packages/axum-http-server/src/v1/routes.rs
status: proposed
---

# Routes Test Refactor Plan

Follow the shared [purpose, quality goals, and plan structure](README.md). This plan applies them
only to `packages/axum-http-server/src/v1/routes.rs`.

## Phase 1 — Identify Problems

### Strengths to preserve

1. The request-ID tests use an in-process Axum router, exercising real request layers without a
   listener, external I/O, background task, or time dependency.
2. Each test has concise Arrange–Act–Assert structure and a distinct contract: supplied request IDs
   are propagated; generated request IDs are valid UUIDs.
3. The generated-ID test correctly checks a deterministic property rather than an exact random UUID.
4. Existing real-server tests cover `router()` registrations and protocol behavior for announce,
   scrape, health-check, configuration modes, and reverse-proxy client-IP handling.

### P1 — Request-ID header access is inconsistent

**Problem.** One test uses the literal `"x-request-id"`; the other constructs
`HeaderName::from_static("x-request-id")`.

**Why it matters.** The same protocol name appears through two forms, which adds small but needless
visual variation in a compact test module.

**Opportunity.** Use one private `REQUEST_ID_HEADER` constant for request construction and response
lookup. Keep client-provided ID values visible in the test.

### P2 — Minimal router helper has a generic name

**Problem.** `test_router()` does not state that its purpose is to expose request-layer behavior.

**Why it matters.** A reader can mistake it for a representative tracker router.

**Opportunity.** Assess whether `router_with_request_layers()` is clearer, while keeping its minimal
successful endpoint and in-process `oneshot` boundary.

### P3 — Remaining coverage is middleware composition, not a known missing contract

**Problem.** Current coverage evidence records 93.43% line coverage and 82.35% function coverage
for `v1/routes.rs`, with uncovered locations near request-layer composition and instrumentation.

**Why it matters.** Compression, timeout, and tracing branches may be uncovered, but adding tests
only to raise aggregate coverage would create framework-coupled or timing-sensitive tests.

**Opportunity.** Assess each candidate only for an explicit stable HTTP contract. In particular,
avoid a five-second timeout test, log-text assertions, and layer-order tests without a documented
regression or versioned observability requirement.

## Phase 2 — Proposed Refactorings

### R1 — Standardize the request-ID header name

- **Status:** TODO
- **Priority:** Medium impact / trivial effort
- **Addresses:** P1
- **Change:** Introduce a private `REQUEST_ID_HEADER` constant and use it in both tests.
- **Guardrails:** Keep `client_request_id` visible. Do not add a one-use assertion helper or assert a
  generated UUID value.
- **Done when:** the header name has one local representation and both existing contracts remain
  explicit.

### R2 — Assess the minimal router helper name

- **Status:** TODO
- **Priority:** Low impact / trivial effort
- **Addresses:** P2
- **Change:** Decide whether renaming `test_router()` to `router_with_request_layers()` materially
  improves intent.
- **Guardrails:** Do not add routes, tracker services, listeners, or production setup. Retain the
  minimal successful endpoint and direct `oneshot` call.
- **Done when:** a rename or no-change decision is recorded.

### R3 — Assess residual middleware coverage by contract value

- **Status:** TODO
- **Priority:** Low impact / medium effort
- **Addresses:** P3
- **Change:** Compare potentially uncovered compression, timeout, and trace branches with existing
  package tests and public requirements. Add a test only for a stable unobserved HTTP contract.
- **Guardrails:** Do not wait for the default five-second timeout; test logs, closure execution,
  framework defaults, layer order, UUID uniqueness, or compression merely to improve coverage.
- **Done when:** the plan records a behavior-justified test or a concrete no-change/defer rationale.

## Progress Tracking

### Plan Checklist

- [x] Phase 1 findings reviewed against the current file and package coverage evidence
- [x] Phase 2 refactorings ordered by impact and effort
- [ ] Maintainer approved implementation of R1
- [ ] R1 implemented, reviewed, and validated
- [ ] R2 assessment completed and decision recorded
- [ ] R3 assessment completed and decision recorded
- [ ] Maintainer reviewed all approved changes
- [ ] Plan completed and ready for commit

### Progress Log

- 2026-09-02 - GitHub Copilot - Created the proposed plan from the current request-layer tests,
  package coverage evidence, and existing real-server contract coverage. No refactoring has been
  implemented.

### Validation Evidence

| Increment          | Status | Evidence                     |
| ------------------ | ------ | ---------------------------- |
| Plan documentation | TODO   | Not run after plan creation. |
| R1                 | TODO   | Not started.                 |
| R2                 | TODO   | Not started.                 |
| R3                 | TODO   | Not started.                 |

## Non-Goals

- Do not add direct `router()` tests for tracker routes already covered by real-server contracts.
- Do not duplicate reverse-proxy client-IP or health-check composition coverage.
- Do not test generated UUID uniqueness, exact random values, trace-log text, layer order, or Axum
  framework defaults.
- Do not trigger a timeout by sleeping or add production configurability solely for this plan.
- Do not move these small fixtures to a shared module.

## Validation Per Approved Increment

- `cargo fmt --all -- --check`
- `cargo test -p torrust-tracker-axum-http-server --lib`
- `git diff --check`
- `linter markdown` when this plan changes
- Refresh `coverage-evidence.md` only after an approved behavior-adding test.
