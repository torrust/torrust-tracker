---
doc-type: test-refactor-plan
issue: 1348
package: torrust-tracker-axum-http-server
target-file: packages/axum-http-server/src/server.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/axum-http-server/src/server.rs
    - packages/axum-http-server/src/testing/environment.rs
    - packages/axum-http-server/tests/server/v1/contract/mod.rs
    - packages/axum-http-server/tests/server/v1/contract/for_all_config_modes/mod.rs
    - packages/axum-http-server/tests/server/v1/contract/using_ipv6_v6only.rs
    - packages/axum-health-check-api-server/tests/server/contract.rs
    - docs/issues/open/1348-1347-add-tests-axum-http-server/coverage-evidence.md
---

# HTTP Server Test Refactor Plan

Follow the shared [purpose, quality goals, and plan structure](README.md). This plan applies only
to `packages/axum-http-server/src/server.rs`.

## Phase 1 — Identify Problems

### Strengths to preserve

1. Health-check URL formation is fast, deterministic, and verifies both HTTP and HTTPS schemes.
2. Listener bind failure is asserted through the package-owned `Error::Bind` type, not
   platform-specific error text.
3. The direct lifecycle test covers the `Stopped → Running → Stopped` state transition.
4. The registration-failure test verifies one coupled cleanup contract: duplicate registration keeps
   its typed cause **and** the listener is released.
5. Package integration tests cover live HTTP listeners, health-check endpoint availability, route and
   protocol behavior, and IPv6-only listener behavior.
6. Cross-package health-check API tests cover registered HTTP/HTTPS service health and a stopped
   service; they are linked above because they cover a composed behavior outside this package.

### P1 — Lifecycle test names do not state their protected invariant

**Problem.** `it_should_be_able_to_start_and_stop` does not state that the test protects retention
of the launcher's original bind configuration after the full state transition. The registration test
similarly buries its cleanup purpose in a long name.

**Why it matters.** Lifecycle setup is substantial, so test names must tell a reader why that setup
exists and what a failure means.

**Opportunity.** Rename the tests to state the start/stop configuration-preservation contract and the
failed-registration listener-cleanup contract. Keep each test's current behavior scope intact.

### P2 — Direct lifecycle setup is dense

**Problem.** `it_should_be_able_to_start_and_stop` mixes configuration selection, global setup,
container composition, optional TLS configuration, registration setup, state transition, and its
final assertion.

**Why it matters.** A reader must navigate setup details before finding the controller contract.

**Opportunity.** Assess concise AAA boundaries and small local naming improvements. Do not replace
the direct `HttpServer` test with `testing::environment`, because that fixture panics on lifecycle
errors and returns a different abstraction.

### P3 — Registration cleanup uses an unavoidable but non-zero port-handoff risk

**Problem.** The registration-failure test reserves an ephemeral address, releases the listener, then
starts the server on the same address so registration can fail after binding. Another process could
claim the address in the handoff window.

**Why it matters.** This is a potential OS-level flake, even though the test asserts valuable cleanup
behavior.

**Opportunity.** Record the limitation and avoid duplicating the reservation/drop pattern. Do not add
sleeps, retries, or polling. Consider a deterministic observation seam only if this test actually
flakes or lifecycle design changes for another reason.

### P4 — The health-check job result seam lacks direct focused coverage

**Problem.** `check_fn_with_client` creates a `ServiceHealthCheckJob` that returns an HTTP status
string or a request error string. URL construction is unit-tested, and composed HTTP/HTTPS health
behavior is covered elsewhere, but this injected-client result propagation is not directly asserted
in `server.rs`.

**Why it matters.** This is a package-owned, injectable boundary that could prevent a regression in
the health job without duplicating TLS or health-check API integration coverage.

**Opportunity.** Assess one deterministic direct job-result test using an existing controlled client
or server capability. Defer if such a boundary requires an external listener, port handoff, waiting,
or new test infrastructure.

### P5 — Uncovered server paths are not all behavior gaps

**Problem.** Coverage is 85.62% for lines, 81.68% for regions, and 65.71% for functions. The
uncovered queue includes private launch mechanics, task/shutdown handling, logging, TLS setup, and
health-check helpers.

**Why it matters.** Tests written only to increase those totals would become scheduler-,
platform-, or implementation-coupled.

**Opportunity.** Defer or use existing package integration, cross-package health integration, root
application integration, or E2E tests according to the narrowest stable boundary. Do not add a
server-local test unless it proves a missing observable contract.

## Phase 2 — Proposed Refactorings

### R1 — Clarify lifecycle test names and AAA boundaries

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1, P2
- **Change:** Rename both lifecycle tests for their explicit controller/cleanup contracts and add
  concise Arrange–Act–Assert boundaries where they improve scanning.
- **Guardrails:** Do not add assertions for routes, logs, tasks, metrics, registry internals, or TLS
  state. Preserve the registration failure's two coupled cleanup assertions as one contract.
- **Done when:** a reader can identify each lifecycle contract from its name and phases without
  changing behavior.

### R2 — Document the registration-test port-handoff limitation

- **Status:** DONE
- **Priority:** Medium impact / trivial effort
- **Addresses:** P3
- **Change:** Add a concise comment beside the reservation/drop setup explaining why it is needed and
  why the test deliberately does not use retries or waiting.
- **Guardrails:** Do not change production code, add another port handoff, or mask flakes with
  sleeps, polling, or retries.
- **Done when:** the risk and intentional trade-off are clear to future maintainers.

### R3 — Assess a deterministic `check_fn_with_client` result contract

- **Status:** DONE
- **Priority:** Medium impact / medium effort
- **Addresses:** P4
- **Change:** Identify an existing deterministic controlled HTTP-client boundary. If one can execute
  the returned health job without listener timing or new infrastructure, add one focused assertion
  for status-string or request-error propagation.
- **Guardrails:** Test one behavior-focused result path; do not duplicate URL-only, HTTP endpoint,
  trusted TLS, or health API registration tests. Do not use external networking, a port handoff,
  sleeps, retries, or log assertions.
- **Decision:** Deferred. `check_fn_with_client` accepts a concrete `reqwest::Client`, and no
  existing deterministic in-process client transport or mock server seam can execute its spawned
  request without a listener. The package and cross-package integration tests already cover HTTP
  health success, trusted HTTPS health success, and post-stop request failure. Adding another
  listener-based test here would duplicate those contracts and create the port/timing concerns that
  this focused plan excludes.
- **Done when:** the deferred rationale and existing coverage boundaries are recorded.

### R4 — Assess external lifecycle coverage boundaries

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Addresses:** P5
- **Change:** Record why remaining server behavior belongs to one of these existing boundaries:
  package integration for listener/endpoint/IPv6 behavior, health-check API integration for
  registered HTTP/HTTPS health behavior, root `tests/` for application composition, or
  `packages/e2e-tools/` for containerized interoperability.
- **Guardrails:** Link only high-signal external test artifacts under this plan's semantic links; do
  not add markers to external source files just to describe coverage overlap.
- **Done when:** the plan states the retained or deferred boundary for each relevant behavior class.

### R5 — Consider a deterministic listener-cleanup seam only on evidence

- **Status:** DEFERRED
- **Priority:** Low impact / high effort
- **Addresses:** P3, P5
- **Change:** Revisit only if the registration-failure test flakes or a production lifecycle change
  creates a meaningful deterministic cleanup-observation seam.
- **Guardrails:** Any future design must preserve public behavior and expose a lifecycle capability,
  not a test-only task-scheduling hook.
- **Done when:** a concrete flake or lifecycle change justifies separate design review.

## Progress Tracking

### Plan Checklist

- [x] Phase 1 findings reviewed against current code, coverage evidence, and external boundaries
- [x] Phase 2 refactorings ordered by impact and effort
- [x] Maintainer approved implementation of R1
- [x] R1 implemented, reviewed, and validated
- [x] Maintainer approved implementation of R2
- [x] R2 implemented, reviewed, and validated
- [x] R3 assessment completed and decision recorded
- [ ] R4 assessment completed and decision recorded
- [ ] Maintainer reviewed all approved changes
- [ ] Plan completed and ready for commit

### Progress Log

- 2026-09-03 - GitHub Copilot - Created the proposed plan from `server.rs`, current package coverage
  evidence, package integration contracts, cross-package health-check integration contracts, root
  integration scope, and E2E tooling. No refactoring has been implemented.
- 2026-09-03 - User/maintainer - Approved R1 after reviewing the explicit lifecycle contract names
  and start/stop AAA structure.
- 2026-09-03 - GitHub Copilot - Completed R1. Renamed the lifecycle tests for launcher
  configuration preservation and duplicate-registration listener cleanup without changing their
  behavior.
- 2026-09-03 - User/maintainer - Approved R2 after reviewing the documented port-handoff
  limitation.
- 2026-09-03 - GitHub Copilot - Completed R2. Documented why duplicate registration must occur
  after listener binding and why retries or waiting would conceal the OS-level handoff risk.
- 2026-09-03 - GitHub Copilot - Completed R3 assessment. Deferred a direct health-job result test:
  the injected client has no existing deterministic transport seam, while package and health-check
  API integration tests already cover HTTP, trusted HTTPS, and post-stop health outcomes.

### Validation Evidence

| Increment          | Status   | Evidence                                                                                                                                                      |
| ------------------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plan documentation | DONE     | `linter markdown`, `linter cspell`, and `git diff --check` passed after plan creation.                                                                        |
| R1                 | DONE     | Editor diagnostics, `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-axum-http-server --lib` (34 passed), and `git diff --check` passed.          |
| R2                 | DONE     | `cargo fmt --all -- --check`, package library tests (34 passed), `linter markdown`, `linter cspell`, and `git diff --check` passed.                           |
| R3                 | DONE     | Inspected `check_fn_with_client`, existing test helpers, package health contracts, and health-check API contracts; no deterministic non-listener seam exists. |
| R4                 | TODO     | Not started.                                                                                                                                                  |
| R5                 | DEFERRED | Awaiting a concrete flake or lifecycle-design trigger.                                                                                                        |

## Non-Goals

- Do not add tests for logs, `BoxFuture` construction, internal graceful-shutdown task mechanics,
  synthetic `JoinHandle`/oneshot failures, or impossible normal-address `ServiceBinding` failures.
- Do not add a TLS lifecycle test that duplicates package integration or trusted health-check API
  coverage.
- Do not add retries, sleeps, polling, or further port-handoff tests.
- Do not move `initialize_container` or create a generic server-test builder solely for aesthetics.
- Do not use root integration or E2E tests as a substitute for a missing deterministic package-local
  lifecycle-controller contract.

## Validation Per Approved Increment

- `cargo fmt --all -- --check`
- `cargo test -p torrust-tracker-axum-http-server --lib`
- `git diff --check`
- `linter markdown` when this plan changes
- Refresh `coverage-evidence.md` only after an approved behavior-adding test.
