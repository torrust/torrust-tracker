---
doc-type: test-refactor-plan
issue: 1348
package: torrust-tracker-axum-http-server
target-file: packages/axum-http-server/src/v1/handlers/announce.rs
status: proposed
---

# Announce Handler Test Refactor Plan

Follow the shared [purpose, quality goals, and plan structure](README.md). This plan applies them
only to `packages/axum-http-server/src/v1/handlers/announce.rs`.

## Phase 1 — Identify Problems

### Current strengths to preserve

1. The compact-response tests use `AnnounceResponseScenario<TExpectedResponse>`. Each scenario
   owns the request selector, domain `AnnounceData`, and independently specified expected decoded
   response.
2. `decode_successful_bencoded_response` centralizes repeated response mechanics while retaining
   the `build_response` SUT call, concrete response type, and final actual-versus-expected
   assertion in each test.
3. The response fixtures are deterministic: fixed peer, addresses, protocol values, and no
   listener or clock.
4. Error tests separate authorization and client-IP behaviors into nested modules named for their
   configuration context.

### Problems and opportunities

#### P1 — Repeated HTTP service-binding setup

**Problem.** Five `handle_announce` error tests construct the same loopback HTTP
`ServiceBinding`.

**Why it matters.** The repeated setup obscures each test's behavior-specific input and creates
multiple update sites if the common binding changes.

**Evidence.** Each nested error-test module constructs
`SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)` and
`ServiceBinding::new(Protocol::HTTP, ...)`.

**Opportunity.** Extract a deterministic `sample_http_service_binding()` fixture, while retaining
each test's configuration, request, client-IP source, and key in the test body.

#### P2 — Inconsistent Arrange, Act, Assert structure

**Problem.** The error tests do not consistently separate their setup, SUT invocation, and
assertion preparation.

**Why it matters.** Readers must infer which statements describe the scenario, execute the SUT,
or prepare the assertion.

**Evidence.** The nested authorization and reverse-proxy tests have no AAA comments, unlike the
response-adapter tests.

**Opportunity.** Add concise comments around the existing logical phases only; do not add a
comment between every statement.

#### P3 — Error result has a response-like name

**Problem.** `response` holds a `Result<DomainAnnounceData, HttpAnnounceError>` that is
immediately unwrapped as an error.

**Why it matters.** The name suggests a successful HTTP response instead of the expected failure.

**Evidence.** Every nested error test assigns `.await.unwrap_err()` to `response`.

**Opportunity.** Name it `actual_error`, then map it to `actual_error_response` before asserting
the stable failure-reason contract.

#### P4 — Failure-reason helper name is too broad

**Problem.** `assert_error_response` accepts a failure-reason substring but does not state that
specific contract in its name.

**Why it matters.** A generic name can encourage vague assertions.

**Evidence.** The helper is called as `assert_error_response(&error_response, ...)`.

**Opportunity.** Rename it to `assert_failure_reason_contains`, retain the full diagnostic output,
and assert only stable behavior-relevant fragments.

#### P5 — Error-test fixture is substantial

**Problem.** Initialization builds real core and persistence services for each error test.

**Why it matters.** The tests are fast today, but this composition can become costly or fragile as
the service graph evolves.

**Evidence.** `initialize_core_tracker_services` initializes a database, event bus, repositories,
authorization, and service.

**Opportunity.** Measure focused test duration and inspect existing seams first. Do not introduce
mocks or production refactors without evidence that cost or fragility is material.

#### P6 — Uncovered handler-entry wiring

**Problem.** Public Axum handler entry points and delegation paths are uncovered despite strong
overall file coverage.

**Why it matters.** Aggregate coverage can hide missing transport wiring, but direct coverage may
duplicate router or integration coverage.

**Evidence.** `coverage-evidence.md` lists uncovered locations 25, 29, 38, 43, 53, and 59 in
`announce.rs`, even though file line coverage is 98.80%.

**Opportunity.** Inspect existing router and real-server tests. Add a focused extractor-to-handler
test only for an unobserved contract; otherwise document why direct coverage is deferred.

## Phase 2 — Proposed Refactorings

Apply items in order. Complete one increment—including review and focused validation—before
beginning the next.

### R1 — Clarify error-test setup and failure contracts

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1, P2, P3, P4
- **Change:** Apply the related readability improvements as one atomic increment:
  1. add `sample_http_service_binding()` and replace the repeated loopback binding setup in the
     five `handle_announce` error tests;
  2. rename values to `actual_error` and `actual_error_response`;
  3. rename `assert_error_response` to `assert_failure_reason_contains`; and
  4. add concise AAA comments around the existing logical phases.
- **Guardrails:** Keep each test's unique configuration and input visible. Preserve the helper's
  complete diagnostic output and stable failure-reason fragments. Add only phase comments; avoid
  comment noise.
- **Done when:** one deterministic common fixture replaces all repeated bindings, each test makes
  its error outcome explicit, and the Act/Assert flow is scannable.

### R2 — Assess fixture cost before changing it

- **Status:** DONE
- **Priority:** Medium impact / low effort
- **Addresses:** P5
- **Change:** Measure focused test duration and inspect existing seams. Record a no-change decision
  unless a smaller existing seam preserves the same observable behavior.
- **Guardrail:** Do not add mocks or refactor production based on speculation.
- **Decision:** No change. The focused `announce` test module ran 8 tests in 0.02 seconds after
  compilation. The current fixture exercises the real `AnnounceService` boundary with its required
  authorization, repository, and configuration collaborators. No smaller existing seam preserves
  those error-path contracts without replacing the behavior with mocks or adding production seams.
- **Done when:** the retained-fixture rationale and timing evidence are recorded.

### R3 — Assess missing handler-entry behavior

- **Status:** DONE
- **Priority:** Medium impact / medium effort
- **Addresses:** P6
- **Change:** Compare the uncovered paths with existing router and real-server tests. Add one
  focused transport-wiring test only if it proves an unobserved contract.
- **Guardrail:** Prefer an in-process router seam; do not add tests merely to turn lines green or
  duplicate integration coverage.
- **Decision:** No change. `v1::routes::router` registers `handle_without_key` on `/announce` and
  `handle_with_key` on `/announce/{key}`. The existing real-server announce contract suite invokes
  those routes across public, private, and whitelisted configurations, including successful,
  malformed, missing-key, invalid-key, and client-IP failure behavior. A new direct
  extractor-to-handler test would exercise the same route wiring while adding a second, less
  representative fixture path. The uncovered entry-point locations are therefore not a missing
  observable contract for this issue.
- **Done when:** the router and real-server coverage assessment and no-change rationale are
  recorded.

### R4 — Consider an error scenario only if needed

- **Status:** DONE
- **Priority:** Low impact / medium effort
- **Addresses:** Remaining duplication after R1
- **Change:** Extract a small scenario fixture only if R1 leaves meaningful duplication.
- **Guardrail:** A scenario must own configuration selector, request, client-IP sources, key, and
  expected failure contract. It must not accumulate optional variants or hide the behavior.
- **Decision:** No change. R1 removed the only repeated mechanical fixture. The remaining test
  statements express different configuration modes, distinct inputs, and different failure-reason
  contracts. A shared scenario would need optional configuration, key, client-IP, and expected
  error fields, which would hide the behavior each nested module currently makes explicit.
- **Done when:** the no-change decision is recorded.

## Progress Tracking

### Plan Checklist

- [x] Phase 1 findings reviewed against the current file
- [x] Phase 2 refactorings ordered by impact and effort
- [x] Maintainer approved implementation of R1
- [x] R1 implemented, reviewed, and validated
- [x] R2 assessment completed and decision recorded
- [x] R3 assessment completed and decision recorded
- [x] R4 assessment completed and decision recorded
- [x] Maintainer reviewed all approved changes
- [x] Plan completed and ready for commit

### Progress Log

- 2026-09-02 - GitHub Copilot - Created the proposed plan from the current
  `announce.rs` tests and the issue's coverage evidence. No refactoring has been implemented.
- 2026-09-02 - User/maintainer - Approved R1 after reviewing the shared binding fixture, explicit
  error-path names, failure-reason assertion name, and AAA structure.
- 2026-09-02 - GitHub Copilot - Completed R1. The five error tests now share
  `sample_http_service_binding()`, use explicit actual-error names, and retain their distinct
  configuration and input in visible Arrange sections.
- 2026-09-02 - GitHub Copilot - Completed R2 assessment. Focused `announce` tests passed 8 tests
  in 0.02 seconds after compilation. Retained the real-service fixture because no smaller existing
  seam preserves the authorization and client-IP error contracts without speculative mocks or
  production changes.
- 2026-09-02 - GitHub Copilot - Completed R3 assessment. Retained existing route and real-server
  coverage: the router binds the public handlers to `/announce` and `/announce/{key}`, and the
  integration suite exercises those routes across configuration and error contracts. No additional
  direct wiring test would add an unobserved behavior.
- 2026-09-02 - GitHub Copilot - Completed R4 assessment. After R1, no repeated mechanical setup
  remains. Retained the explicit error tests because a shared scenario would require optional,
  behavior-hiding fields across unrelated configuration and failure contracts.
- 2026-09-02 - User/maintainer - Reviewed and approved the R4 no-change decision and completion
  of the announce-handler test refactor plan.

### Validation Evidence

| Increment          | Status | Evidence                                                                                                                                         |
| ------------------ | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Plan documentation | DONE   | `linter markdown`, `linter cspell`, and `git diff --check` passed after plan creation.                                                           |
| R1                 | DONE   | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-axum-http-server --lib`, `git diff --check`, and editor diagnostics passed.         |
| R2                 | DONE   | `cargo test -p torrust-tracker-axum-http-server --lib v1::handlers::announce::tests -- --nocapture`: 8 passed in 0.02 seconds after compilation. |
| R3                 | DONE   | Inspected `v1::routes::router` and the existing real-server announce contract suite; no unique observable wiring contract was found.             |
| R4                 | DONE   | Inspected the post-R1 error tests; their remaining differences are behavior-specific, so a scenario would add optional, opaque fields.           |

## Non-Goals

- Do not replace direct full-value `assert_eq!` in the announce-response tests with a custom
  assertion helper.
- Do not merge compact and non-compact response tests into a parameterized test if doing so hides
  their different decoded response types or expected wire representations.
- Do not add retries, sleeps, or broad test-timeout increases to mask failures.
- Do not pursue uncovered lines that correspond only to generated trait glue or already-covered
  integration wiring without a missing observable behavior.
- Do not change production behavior as part of this plan.

## Validation Per Approved Increment

- `cargo fmt --all -- --check`
- `cargo test -p torrust-tracker-axum-http-server --lib`
- `git diff --check`
- Run `linter markdown` if this plan changes.
- Review the updated coverage evidence only after a behavior-adding test, not for a readability-only
  refactor.

## Completion Criteria

- Every approved item preserves deterministic, behavior-focused tests.
- Common mechanics are reduced without hiding scenarios, the SUT call, expected type, or final
  behavior assertion.
- Any coverage addition is justified by a missing observable contract, not an aggregate percentage.
- The maintainer reviews the completed increment before a commit or the next file's plan.
