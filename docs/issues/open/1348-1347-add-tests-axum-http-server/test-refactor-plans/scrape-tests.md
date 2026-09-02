---
doc-type: test-refactor-plan
issue: 1348
package: torrust-tracker-axum-http-server
target-file: packages/axum-http-server/src/v1/handlers/scrape.rs
status: proposed
---

# Scrape Handler Test Refactor Plan

Follow the shared [purpose, quality goals, and plan structure](README.md). This plan applies them
only to `packages/axum-http-server/src/v1/handlers/scrape.rs`. The separate
[draft shared-bootstrap plan](drafts/shared-handler-test-bootstrap.md) owns cross-file setup
questions and must not be implemented through this plan.

## Phase 1 — Identify Problems

### Strengths to preserve

1. The response-mapping test decodes the bencoded body and compares complete, downloaded, and
   incomplete values against an independently specified protocol response.
2. Authentication, whitelist, and client-IP scenarios use fresh in-memory dependencies and fixed
   inputs.
3. Existing real-server tests cover multiple info hashes, private/whitelisted behavior, and TCP4/
   TCP6 statistics separately from focused handler tests.

### P1 — Handler module tests mainly exercise the service directly

**Problem.** Six mode and client-IP tests call `ScrapeService::handle_scrape` directly. They live
beside the Axum handler but do not exercise the handler's service-result-to-HTTP-response mapping.

**Why it matters.** The local test boundary is unclear, and `handle`'s error-response conversion is
not directly verified.

**Opportunity.** Extract a narrow `handle_scrape` service-delegation seam, as in `announce.rs`, and
add a focused test for the handler's observable bencoded failure response.

### P2 — Service construction is repeated at test call sites

**Problem.** Tests receive two dependency bundles and repeatedly construct `ScrapeService`, using
two constructor variants.

**Why it matters.** Each test must know constructor argument order, obscuring the key, IP, and
configuration variation that actually defines its behavior.

**Opportunity.** Make local setup return a ready-to-call `Arc<ScrapeService>` configured from the
selected HTTP tracker configuration. Keep the cross-file bootstrap question out of this plan.

### P3 — Common fixtures and error-test flow are inconsistent

**Problem.** Five tests reconstruct the same loopback `ServiceBinding`; error tests use generic
`response` and `error_response` names and lack AAA boundaries.

**Why it matters.** Repeated mechanics and unclear value roles reduce scanning speed and create
unnecessary update sites.

**Opportunity.** Add local deterministic `sample_http_service_binding()` and
`missing_client_ip_sources()` fixtures; rename error values to `actual_error` and
`actual_error_response`; rename `assert_error_response` to `assert_failure_reason_contains`; and
add concise AAA boundaries.

### P4 — The response mapper has only a single-file focused example

**Problem.** The local `build_response` test covers one info hash, while
`to_protocol_scrape_data` loops over all requested files.

**Why it matters.** Endpoint tests cover multiple info hashes, but a focused two-file mapping
example would protect the local adapter loop directly.

**Opportunity.** Add one concrete two-file response-mapping test with independently specified
protocol output. Do not add a generic scenario type or parameterized matrix.

### P5 — Statistics-listener ownership is unclear

**Problem.** Setup may start a statistics listener and discards its task and cancellation handle;
the local tests do not assert statistics.

**Why it matters.** Detached background work weakens lifecycle clarity and may become a source of
cost or flakiness.

**Opportunity.** First verify whether the selected configurations require the listener for these
contracts. Remove it only if it is unnecessary, or retain explicit ownership if required.

### P6 — Module documentation names the wrong protocol request

**Problem.** The module documentation describes `announce` requests.

**Why it matters.** It misleads readers navigating the scrape handler and its tests.

**Opportunity.** Correct it to `scrape` as a separate documentation-only change.

## Phase 2 — Proposed Refactorings

### R1 — Correct the scrape module documentation

- **Status:** DONE
- **Priority:** High impact / trivial effort
- **Addresses:** P6
- **Change:** Replace the module-documentation reference to `announce` with `scrape`.
- **Guardrail:** Documentation only; do not combine with handler behavior changes.
- **Done when:** the module-level description correctly identifies scrape requests.

### R2 — Clarify local test setup and error contracts

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P3
- **Change:** Add the two narrow fixtures; use explicit actual-error names; rename the
  failure-reason assertion helper; and add concise AAA boundaries.
- **Guardrail:** Keep keys, configuration modes, client-IP sources, and expected contracts visible in
  each test. Do not introduce a broad scenario fixture.
- **Done when:** repeated mechanics are local helpers and each error test visibly expresses its
  behavior-specific contract.

### R3 — Re-establish the handler error-mapping boundary

- **Status:** DONE
- **Priority:** High impact / medium effort
- **Addresses:** P1
- **Change:** Extract `handle_scrape` as the service-delegation seam and add a focused test that an
  `HttpScrapeError` becomes HTTP `200 OK` plus the expected bencoded failure response.
- **Guardrail:** Preserve scrape's zeroed-data behavior for unauthenticated/private and
  non-whitelisted cases. Assert protocol-visible output, not incidental error internals.
- **Done when:** direct local coverage proves the handler's error-response conversion separately
  from `ScrapeService` outcomes.

### R4 — Return a ready-to-call service from local setup

- **Status:** DONE
- **Priority:** Medium impact / medium effort
- **Addresses:** P2
- **Change:** Make the file-local setup return an `Arc<ScrapeService>` configured with the selected
  HTTP tracker configuration, then remove repeated constructor calls.
- **Guardrail:** Keep this helper local. Do not generalize common bootstrap infrastructure or claim
  coverage for the alternate `ScrapeService::new` constructor.
- **Done when:** mode tests vary request, key, and client IP without repeating service construction.

### R5 — Add a concrete two-file response-mapping contract

- **Status:** TODO
- **Priority:** Medium impact / low effort
- **Addresses:** P4
- **Change:** Add one `build_response` test with two fixed info hashes and distinct metadata,
  decoding the response and comparing it with independently specified expected protocol data.
- **Guardrail:** Do not derive the expected value through production mapping/serialization and do
  not parameterize without new meaningful behavior variants.
- **Done when:** the local mapper loop has one behavior-focused multi-file contract.

### R6 — Assess statistics-listener necessity and lifecycle

- **Status:** TODO
- **Priority:** Medium impact / medium effort
- **Addresses:** P5
- **Change:** Verify configuration and sender/receiver requirements. Record either a no-change
  lifecycle rationale or an approved focused cleanup.
- **Guardrail:** Do not remove required event wiring or add sleeps, retries, polling, or shared state.
- **Done when:** listener necessity and ownership are explicit.

### R7 — Reassess the cross-file bootstrap draft after local changes

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Addresses:** Cross-file bootstrap duplication
- **Change:** Revisit `drafts/shared-handler-test-bootstrap.md` after R2–R6. Promote it only when a
  cohesive shared responsibility remains.
- **Guardrail:** Do not implement cross-file extraction as part of this file-local plan.
- **Done when:** the draft is retained, revised, promoted, or closed with rationale.

## Progress Tracking

### Plan Checklist

- [x] Phase 1 findings reviewed against the current file
- [x] Phase 2 refactorings ordered by impact and effort
- [x] Maintainer approved implementation of R1
- [x] R1 implemented, reviewed, and validated
- [x] Maintainer approved implementation of R2
- [x] R2 implemented, reviewed, and validated
- [x] Maintainer approved implementation of R3
- [x] R3 implemented, reviewed, and validated
- [x] Maintainer approved implementation of R4
- [x] R4 implemented, reviewed, and validated
- [ ] Maintainer approved implementation of R5
- [ ] R5 implemented, reviewed, and validated
- [ ] R6 assessment completed and decision recorded
- [ ] R7 draft assessment completed and decision recorded
- [ ] Maintainer reviewed all approved changes
- [ ] Plan completed and ready for commit

### Progress Log

- 2026-09-02 - GitHub Copilot - Created the proposed plan from the current `scrape.rs` tests,
  existing integration coverage, and the completed announce-handler test plan. No refactoring has
  been implemented.
- 2026-09-02 - User/maintainer - Approved R1 after reviewing the isolated module-documentation
  correction.
- 2026-09-02 - GitHub Copilot - Completed R1 by correcting the module documentation to identify
  scrape requests.
- 2026-09-02 - User/maintainer - Approved R2 after reviewing the deterministic local fixtures,
  clearer error-value names, explicit failure-reason assertion, and AAA structure.
- 2026-09-02 - GitHub Copilot - Completed R2. Replaced repeated binding and missing-client-IP
  setup with narrow local fixtures, made actual values explicit, and added concise AAA boundaries.
- 2026-09-02 - User/maintainer - Approved implementation of R3.
- 2026-09-02 - GitHub Copilot - Completed R3. Extracted the `handle_scrape` service-delegation
  seam and added a focused contract proving an unresolved reverse-proxy client IP becomes a
  bencoded BitTorrent failure response with HTTP `200 OK`.
- 2026-09-02 - User/maintainer - Approved R4 after reviewing the local ready-to-call service
  fixture.
- 2026-09-02 - GitHub Copilot - Completed R4. Local setup now returns a configuration-aware
  `Arc<ScrapeService>`, removing repeated constructor wiring while keeping test inputs explicit.

### Validation Evidence

| Increment          | Status | Evidence                                                                                                                                             |
| ------------------ | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plan documentation | DONE   | `linter markdown`, `linter cspell`, and `git diff --check` passed after plan creation.                                                               |
| R1                 | DONE   | `cargo fmt --all -- --check`, package library tests, `linter markdown`, `linter cspell`, and `git diff --check` passed.                              |
| R2                 | DONE   | Editor diagnostics, `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-axum-http-server --lib`, and `git diff --check` passed.             |
| R3                 | DONE   | Editor diagnostics, `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-axum-http-server --lib` (31 passed), and `git diff --check` passed. |
| R4                 | DONE   | Editor diagnostics, `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-axum-http-server --lib` (31 passed), and `git diff --check` passed. |
| R5                 | TODO   | Not started.                                                                                                                                         |
| R6                 | TODO   | Not started.                                                                                                                                         |
| R7                 | TODO   | Not started.                                                                                                                                         |

## Non-Goals

- Do not implement the shared-bootstrap draft through this plan.
- Do not replace scrape's zeroed-data contracts with announce-style error contracts.
- Do not introduce a generic response scenario where one inline expected response is clearer.
- Do not add tests merely to raise an aggregate coverage percentage.
- Do not add retries, sleeps, broad timeouts, or shared state.
- Do not change production behavior except the explicitly isolated documentation correction.

## Validation Per Approved Increment

- `cargo fmt --all -- --check`
- `cargo test -p torrust-tracker-axum-http-server --lib`
- `git diff --check`
- `linter markdown` when this plan changes
- Refresh `coverage-evidence.md` only after an approved behavior-adding test.
