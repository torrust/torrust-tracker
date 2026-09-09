---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/handlers/mod.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/handlers/mod.rs
    - packages/udp-server/src/handlers/error.rs
    - packages/udp-server/src/server/processor.rs
    - packages/udp-server/src/error.rs
    - packages/udp-protocol/src/request.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Handler Dispatch Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to `packages/udp-server/src/handlers/mod.rs`.

## Phase 1 - Clean Current Tests

### Current state

`handlers/mod.rs` has no direct test cases. Its `#[cfg(test)]` module provides narrowly scoped
service construction, sample network values, and mock sender types used by the individual handler
modules. The test-bearing handler modules already exercise their own business rules at the service
boundary.

### Decision

No cleanup increment is proposed. Moving or generalizing the existing support would create a
cross-module fixture change without an observed readability, duplication, or determinism problem.
The proposed direct tests must use only the smallest existing support required by the orchestration
boundary; they must not turn this support module into a generic server fixture.

## Phase 2 - Add Missing Behavior Tests

### Strengths to preserve

1. `handle_packet` owns the package boundary between raw datagrams, protocol parsing, request
  dispatch, and error response routing.
2. `handle_request` dispatches `Connect`, `Announce`, and `Scrape` requests, while each concrete
  handler owns its respective tracker behavior.
3. `error.rs` directly covers parse-error metadata conversion and `handlers/error.rs` directly
  covers error-response serialization and error-event emission.
4. `server/processor.rs` owns the source-port-zero defensive guard before it delegates to
  `handle_packet`.

### Problems and opportunities

#### P1 - Parse-failure routing has no direct orchestration contract

**Problem.** No test calls `handle_packet` with a malformed raw payload. The direct adapter and
error-response tests prove their individual behavior, but neither proves that the dispatcher
preserves a sendable parse error's transaction identifier while reporting that no request kind was
parsed.

**Why it matters.** A refactor can accidentally discard the transaction identifier before error
routing, or report an invented request kind to the caller. Either regression breaks the UDP server
response/metrics boundary without being a protocol-parser or error-serializer defect.

**Opportunity.** Construct one minimal malformed payload that produces a sendable
`RequestParseError` with a fixed transaction identifier. Call `handle_packet` with deterministic
containers and assert an error response carries that identifier and the returned request kind is
`None`.

#### P2 - Handler-error routing is adjacent but risks duplicating handler behavior

**Problem.** The successful-parse / failed-handler branch is untested directly here.

**Why it matters.** This branch must preserve the parsed request kind for the caller while routing
the handler's error through `handle_error`.

**Opportunity.** Assess whether an existing deterministic invalid request can exercise this branch
without testing connection-cookie validation, whitelist policy, database behavior, or final
error-event serialization. Add a test only if its fixture makes the dispatch/routing distinction
clearer than the existing handler and error tests.

#### P3 - Success dispatch belongs primarily to individual handlers

**Decision.** Do not add connect, announce, or scrape success-dispatch matrices. The individual
handler tests own those behavioral outcomes, and a dispatcher matrix would only repeat their
service setup and protocol response assertions.

## Proposed Refactorings

Apply items in order. Complete one approved increment, including its review and focused validation,
before beginning the next item.

### R1 - Record the Phase 1 no-change decision

- **Status:** TODO
- **Priority:** High impact / trivial effort
- **Change:** Confirm that `handlers/mod.rs` has no direct tests to clean and that its existing
  support remains local to the handler modules.
- **Done when:** Phase 1 is explicitly complete without a cross-module fixture refactor.

### R2 - Cover sendable parse-failure routing

- **Status:** TODO
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Add one direct asynchronous `handle_packet` test using a fixed malformed payload with
  a sendable transaction identifier. Assert the independently specified `Response::Error`
  transaction identifier and `None` request-kind result.
- **Guardrails:** Construct no parser matrix and do not call `Error::from` to derive expected
  metadata. Use disabled/non-listening event infrastructure unless the exact dispatch contract
  requires observing an event. Do not assert error response text, logging, latency, UUIDs, or
  handler service effects.
- **Done when:** the raw-payload to error-response-routing contract is protected while protocol
  parsing and final error serialization remain owned by their existing tests.

### R3 - Assess failed-handler routing without duplicate business behavior

- **Status:** DONE
- **Priority:** Medium impact / low effort
- **Addresses:** P2, P3
- **Change:** Review the parsed-request error branch after R2. Record a no-change decision unless
  one existing deterministic request produces a handler error with a visible request-kind routing
  distinction and no duplicated handler-policy assertion.
- **Guardrails:** Do not introduce mocks or production dependency injection solely for this test.
  Do not use clocks, retries, sockets, databases, or lifecycle fixtures beyond existing minimal
  test support.
- **Decision:** No test added. `handle_announce` and `handle_scrape` construct the
  `(Error, TransactionId, UdpRequestKind)` tuple from their parsed request at the handler boundary.
  `handle_error` directly verifies that a supplied transaction ID and request kind become the error
  response/event routing result. The real-loopback contract suite exercises invalid-cookie request
  behavior at the outer boundary. A direct `handle_packet` failure case would need to configure an
  invalid cookie, whitelist, or other tracker policy merely to reproduce that tuple and call
  `handle_error`; its assertions would duplicate the handler cause or the direct error-routing
  contract rather than reveal a distinct dispatcher behavior.
- **Done when:** the branch either has one distinct dispatcher contract or a documented reason it
  remains protected at the handler/error boundaries.

### R4 - Review Phase 2 test design and residual coverage

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Change:** After each added test, review Arrange-Act-Assert visibility, fixture scope, and
  ownership. Measure residual coverage only to decide whether another distinct orchestration
  behavior exists.
- **Guardrails:** Do not add percentage-only tests or change production dispatch behavior.
- **Done when:** every remaining gap is assigned to the dispatcher, a concrete handler, the
  protocol parser, error serializer, or processor boundary.

## Progress Tracking

### Plan Checklist

- [x] Phase 1 and Phase 2 boundaries reviewed against current handler, error, parser, and processor tests.
- [x] Maintainer approved R1.
- [x] R1 no-change decision recorded and committed.
- [x] Maintainer approved R2.
- [x] R2 implemented, reviewed, validated, and committed.
- [x] R3 assessment completed and decision recorded.
- [ ] R4 design/coverage review completed and decision recorded.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

### Progress Log

- 2026-09-08 16:32 UTC - GitHub Copilot - Created this proposed two-phase plan after reviewing
  `handlers/mod.rs`, its current shared test support, the processor guard tests, direct
  parse-error adapter tests, and direct error-response tests. No test or production change has
  been made.
- 2026-09-09 - User/maintainer - Approved R1's Phase 1 no-change decision and R2's focused
  sendable parse-failure routing test. Commit this approval record and the proposed plan before
  implementing the test.
- 2026-09-09 - User/maintainer - Reviewed and approved R2 after its Arrange section was reduced
  to `SendableParseErrorPacketScenario`. The scenario names the causal sendable-parse-error state
  and the test keeps `handle_packet` plus its independently specified transaction-ID/request-kind
  contract visible.
- 2026-09-09 - User/maintainer - Requested a further simplification because the scenario still
  moved complexity rather than making the initial state directly readable. Applied a prose-first
  Arrange-Act-Assert loop: temporarily state each section in normal prose, then refactor until the
  code expresses that prose and remove redundant comments. The final test separates ordinary
  `initialize_udp_handler_environment` mechanics from the causal
  `scrape_request_without_info_hashes(transaction_id)` input; its transaction ID, dispatcher Act,
  and expected outputs remain directly visible.
- 2026-09-09 - GitHub Copilot - Completed R3 assessment. No failed-handler routing test is added:
  handler tests own construction of error/request-kind metadata, `handlers/error.rs` owns its
  routing to a response/event, and real-loopback contracts own invalid-cookie behavior. A direct
  dispatcher case would duplicate one of those boundaries to reach the same call.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1 | DONE | No change: `handlers/mod.rs` has no direct test cases to clean. Its existing local support remains focused on individual handler modules, so a cross-module fixture refactor is not justified. |
| R2 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server handlers::tests::it_should_preserve_the_transaction_id_for_a_sendable_parse_error_without_a_request_kind`, and `git diff --check` passed. A prose-first Arrange-Act-Assert comparison replaced the scenario with named ordinary-environment and causal-empty-scrape helpers; the transaction ID, dispatcher Act, and expected outputs remain visible. |
| R3 | DONE | No change: handler-error metadata is created and covered at the announce/scrape boundary, `handlers/error.rs` directly covers supplied error routing, and real-loopback contracts cover invalid-cookie behavior. A `handle_packet` failure test would duplicate one of those boundaries. |
| R4 | TODO | Awaiting Phase 2 completion. |

## Non-Goals

- Do not change dispatcher, parser, handler, error-response, event, or processor production behavior.
- Do not duplicate `udp-protocol` parsing matrices, concrete handler business rules, or
  `handlers/error.rs` error serialization/event tests.
- Do not create a generic test container, mock a concrete handler, or introduce a socket, retry,
  sleep, clock, database, or shutdown-lifecycle test.

## Validation Per Approved Increment

- Run focused handler-dispatch tests.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- After each Phase 2 behavior test, review causal state, visible Act, independently specified
  expected result, and ownership before the next increment.

## Completion Criteria

- Phase 1 makes an explicit no-change or cleanup decision for existing target-file test code.
- Each approved direct test protects a unique raw-packet dispatch contract.
- Individual handler, protocol parser, error serializer, and processor contracts remain at their
  existing ownership boundaries.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
