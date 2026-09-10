---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/tests/server/contract.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/tests/server/contract.rs
    - packages/udp-server/tests/server/asserts.rs
    - packages/udp-server/src/server/receiver.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/processor.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Server Contract Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to `packages/udp-server/tests/server/contract.rs`.

## Phase 1 - Clean Current Tests

### Current state

`contract.rs` contains the package's real-loopback UDP contracts. Those tests are the appropriate
boundary for wire behavior that a unit test cannot express, including datagram serialization,
response decoding, and listener configuration. They currently repeat environment/client bootstrap,
manual `match`-and-`panic!` error handling, and teardown calls. The first empty-request contract
also constructs and decodes the transport exchange inline, so its intended BEP 15 error-response
contract is less prominent than its mechanics.

`src/server/receiver.rs` is exercised by this real-loopback suite. Its only direct wrapper behavior
is converting a bound socket receive into `RawRequest`; a new direct test would require the same
UDP I/O boundary and would be less readable than the existing integration coverage. No receiver
plan is proposed unless this contract review exposes a receiver-specific regression gap.

### Decision

Begin with one prose-first refactor of
`should_return_a_bad_request_response_when_the_client_sends_an_empty_request`. Its temporary prose
must distinguish the causal empty UDP datagram, the real loopback exchange, and the independently
specified error response. Extract only repeated, non-behavioral mechanics that remain useful to an
adjacent contract; do not introduce a general integration-test framework or refactor the entire
file in one increment.

## Phase 2 - Add Missing Behavior Tests

### Strengths to preserve

1. Real-loopback contracts cover actual UDP client/server serialization and the receive/send
   transport boundary.
2. Unit tests own internal adapter, admission, event, and normal-operation buffer behavior first.
3. Existing contract tests exercise connect, announce, scrape, invalid packet, IPv6, and selected
   connection-ID-validation behaviors.

### Problems and opportunities

#### P1 - The empty-datagram wire contract has a readability opportunity

**Problem.** The test's Arrange and Act mix server bootstrap, client bootstrap, datagram send,
response receive, and protocol decoding.

**Opportunity.** Use prose-first AAA verification to expose “an empty datagram receives the
protocol error response” while retaining the actual client/server transport call and independently
specified error response assertion.

#### P2 - Integration behavior must remain distinct from unit seams

**Decision.** Do not add an integration test for the launcher admission decisions, parse-error
routing, or event classification covered by #2149 unit tests. Add a contract only when real UDP
datagram transport gives clearer or unique regression value.

#### P3 - Broad integration-fixture extraction is premature

**Decision.** Do not introduce a configurable server/client builder or shared lifecycle abstraction.
Extract one helper only after the first prose-first refactor demonstrates repeated non-behavioral
mechanics and keeps each test's causal state, Act, and expected result visible.

## Proposed Refactorings

Apply items in order. Complete one approved increment—including prose-first comparison, focused
validation, review, and its mapped commit point—before beginning the next item.

### R1 - Clarify the empty-datagram error contract

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1, P3
- **Change:** Write temporary Arrange-Act-Assert prose for the empty-request contract. Refactor
  only enough to make the empty datagram, direct UDP exchange, and expected error response clearly
  visible. Keep server lifecycle setup and teardown correct.
- **Guardrails:** Do not assert logging, internal parser implementation, event delivery, or
  statistics. Do not use sleeps, polling, or a new generic fixture. Keep response parsing and the
  error assertion in the test or a narrowly named decoding helper that does not derive expectations.
- **Prose-first review:** The temporary Arrange prose was “a running UDP tracker and a real
  loopback client send an empty datagram.” `start_ephemeral_udp_tracker` names the coherent
  non-behavioral lifecycle setup, even with one caller, because it keeps the test at the same
  abstraction level as its real UDP interaction. `empty_udp_datagram` makes the causal input
  visible. The Act retains send, receive, and protocol decode steps; the Assert independently
  specifies the missing-protocol-identifier error. Temporary prose is redundant and removed.
- **Done when:** the code expresses the wire contract without redundant prose and has one clear
  behavioral reason to fail.

### R2 - Assess one adjacent real-loopback contract improvement

- **Status:** IN_PROGRESS
- **Priority:** Medium impact / low effort
- **Addresses:** P1-P3
- **Change:** After R1, inspect the nearby connect-response contract. Record whether a small
  repeated transport helper improves both tests without hiding their causal input, real UDP Act, or
  expected response. Do not add behavior merely to increase integration coverage.
- **Guardrails:** Preserve distinct unit-test ownership. A no-change decision is preferred to a
  broad fixture extraction.
- **Assessment:** A narrow cleanup is justified. The adjacent connect contract repeats the complete
  ephemeral tracker bootstrap that R1 moved into `start_ephemeral_udp_tracker`, proving the helper
  names a coherent shared lifecycle action rather than hiding one caller's mechanics. Reuse that
  helper and replace the manual client `match` branches with expectation messages. Keep the causal
  `ConnectRequest`, direct client send/receive Act, expected transaction ID, and explicit tracker
  shutdown visible. Do not extract a generic send/receive helper because the connect request and
  response assertion are the contract's relevant behavior.
- **Prose-first review:** The temporary Arrange prose was “a running ephemeral tracker and a
  loopback client have a connect request with transaction ID 123.” The final code names tracker
  startup, client connection, and the transaction ID/request directly. The Act retains the real
  client send/receive exchange and the Assert independently specifies the response transaction ID.
  `start_ephemeral_udp_tracker` owns only coherent ordinary lifecycle setup; no generic transport
  helper hides the contract. The temporary prose is redundant and removed.
- **Done when:** the next contract cleanup or no-change boundary decision is recorded.

### R3 - Review residual integration coverage and ownership

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Change:** Measure integration-only coverage for selected production seams and compare it with
  separate unit-only evidence. Record only unique loopback behavior; assign internal logic to its
  existing unit boundary or lifecycle work to #1488.
- **Guardrails:** Do not use combined coverage to claim either boundary and do not add
  percentage-only tests.
- **Done when:** the plan identifies whether another real-loopback contract has unique value.

## Progress Tracking

### Plan Checklist

- [x] Existing real-loopback contracts, receiver boundary, unit-first policy, and candidate seams reviewed.
- [x] Maintainer approved R1.
- [x] R1 implemented, reviewed, validated, and committed.
- [x] R2 assessment completed and proposed cleanup recorded.
- [x] Maintainer approved R2 cleanup.
- [x] R2 cleanup implemented, reviewed, validated, and committed.
- [ ] R3 coverage/ownership review completed and decision recorded.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

### Progress Log

- 2026-09-09 - GitHub Copilot - Created this proposed plan after completing the launcher plan and
  reviewing `contract.rs`, `receiver.rs`, and the issue's unit-first coverage policy. No contract
  test or production change has been made.
- 2026-09-10 - User/maintainer - Approved R1. Apply the prose-first Arrange-Act-Assert cleanup to
  the empty-datagram contract only; commit this plan update before modifying the integration test.
- 2026-09-10 - User/maintainer - Confirmed that a helper is justified by its meaningful name and
  coherent abstraction level, not by having multiple callers. Retained
  `start_ephemeral_udp_tracker` because it names a cohesive setup action and keeps the contract test
  focused on real UDP behavior.
- 2026-09-10 - GitHub Copilot - Completed R2 assessment. The adjacent connect contract repeats
  R1's tracker bootstrap, so `start_ephemeral_udp_tracker` is a justified shared named action. A
  narrow cleanup is proposed; it retains the connect request, client exchange, expected transaction
  ID, and tracker shutdown in the test rather than introducing a generic transport helper.
- 2026-09-10 - User/maintainer - Approved the R2 cleanup. Reuse the named ephemeral-tracker
  setup, improve client error messages, and retain the visible connect request, UDP exchange,
  transaction-ID assertion, and explicit shutdown.
- 2026-09-10 - User/maintainer - Reviewed and approved R2. The shared tracker-start helper keeps
  both adjacent loopback contracts at one abstraction level; the visible connect request, transport
  Act, expected transaction ID, and shutdown preserve the test's behavior-specific contract.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server --test integration should_return_a_bad_request_response_when_the_client_sends_an_empty_request`, and `git diff --check` passed. Prose-first review retains named tracker setup, causal empty datagram, visible UDP exchange, and independent protocol-error assertion. |
| R2 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server --test integration receiving_a_connection_request::should_return_a_connect_response`, and `git diff --check` passed. Prose-first review retains named tracker setup, visible connect request/exchange, independent transaction-ID assertion, and explicit shutdown. |
| R3 | TODO | Awaiting R2 review. |

## Non-Goals

- Do not replace package integration tests with unit tests or duplicate unit-owned behavior at the
  wire boundary.
- Do not redesign server shutdown, receive-loop ownership, listener teardown, client timeout, or
  task lifecycle; those are governed by #1488.
- Do not add sleeps, polling, uncontrolled external networking, a generic integration fixture, or
  a percentage-only test.

## Validation Per Approved Increment

- Apply the mandatory prose-first Arrange-Act-Assert comparison before maintainer review.
- Run only the selected contract test, then the package integration target when the increment is
  approved for broader validation.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Record unit-only and integration-only coverage separately whenever coverage informs a decision.

## Completion Criteria

- Each retained integration test has a real UDP boundary reason that makes it more appropriate or
  clearer than a unit test.
- Refactors expose causal state, real transport Act, and independent expected response without
  hiding them in broad helpers.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
