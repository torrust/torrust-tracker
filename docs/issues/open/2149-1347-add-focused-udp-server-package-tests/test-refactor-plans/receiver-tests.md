---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/receiver.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/receiver.rs
    - packages/udp-server/src/server/bound_socket.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/tests/server/contract.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

# UDP Server Receiver Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to `packages/udp-server/src/server/receiver.rs`.

## Phase 1 - Clean Current Tests

### Current state

`receiver.rs` has no colocated test module. `Receiver` is a thin `Stream` adapter over a concrete
`BoundSocket`: it delegates readiness to `poll_recv_from`, copies the filled bytes to a
`RawRequest`, and preserves the sender address. Clean unit-only coverage is 15/22 lines (68.18%),
while existing package integration tests execute 21/22 lines (95.45%).

### Decision

No cleanup increment is proposed because no local test code exists. Do not refactor launcher or
integration tests while assessing this adapter. Existing integration coverage retains its distinct
transport value, but it does not replace the feasible focused `--lib` unit contract required by the
issue's unit-first policy.

## Phase 2 - Add Missing Behavior Tests

### Strengths to preserve

1. `Receiver::poll_next` owns adapting one received UDP datagram into `RawRequest`.
2. `BoundSocket` owns socket binding and endpoint metadata; reuse its normal IPv4 loopback
   construction without duplicating its own contracts.
3. `Launcher` owns repeated receiving, admission, request processing, and task lifecycle.
4. #1488 owns receive-loop cancellation, termination, joining, and shutdown policy.

### Problems and opportunities

#### P1 - Datagram-to-raw-request adaptation has no direct unit contract

**Problem.** Existing real-loopback integration tests execute normal reception, but no local unit
test directly specifies that one datagram yielded by `Receiver` preserves its payload and sender
address.

**Why it matters.** A change in buffer handling, `ReadBuf::filled()` extraction, or sender-address
propagation can break this adapter while a higher-level failure is less local and less diagnostic.

**Opportunity.** Bind a normal IPv4 loopback `BoundSocket` on port zero, construct `Receiver`, and
send one short datagram from an ephemeral Tokio `UdpSocket` before the Act. Await exactly one
`StreamExt::next()` under an absolute failure deadline, then assert the independently captured
client address and explicit payload. The queued-before-Act ordering avoids a readiness race, sleep,
retry, polling loop, server task, listener, or lifecycle fixture.

#### P2 - Pending, receive-error, and stream-termination branches lack a stable unit boundary

**Decision.** Do not directly test `Poll::Pending`, receive errors, or `None`. Pending requires
manual waker/readiness orchestration and tests Tokio implementation mechanics; a valid live UDP
socket has no portable receive-error injection; and termination is receive-loop lifecycle behavior
owned by #1488. Do not add a mock socket abstraction: it would add hot-path indirection and model
polling mechanics while weakening the concrete Tokio socket contract.

#### P3 - Bound socket address forwarding has no separate value

**Decision.** Do not add an independent `bound_socket_address` forwarding test. It simply delegates
to the already directly tested `BoundSocket::address` behavior.

## Proposed Refactorings

Apply items in order. Complete one approved increment—including prose-first comparison, focused
validation, review, and its mapped commit point—before beginning the next item.

### R1 - Record the Phase 1 no-change decision

- **Status:** TODO
- **Priority:** High impact / trivial effort
- **Addresses:** Phase 1
- **Change:** Confirm that `receiver.rs` has no direct tests to clean and that integration coverage
  does not replace the feasible unit-test assessment.
- **Guardrails:** Do not move or refactor launcher, socket, or integration test code.
- **Done when:** The no-cleanup decision is recorded before adding a test.

### R2 - Cover queued loopback datagram adaptation

- **Status:** TODO
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Add one direct asynchronous unit test. Queue one explicit loopback datagram before
  awaiting one `Receiver::next()`, then assert that the returned `RawRequest` has the exact payload
  and independently captured client address.
- **Guardrails:** Keep the payload, expected sender address, datagram send, stream Act, and
  assertions visible. The sole timeout is an absolute diagnostic failure bound. Do not add a sleep,
  retry, polling loop, generic UDP helper, server task, event listener, or explicit teardown.
- **Done when:** A regression in normal UDP datagram adaptation has one direct, deterministic
  unit-test failure.

### R3 - Review the test design after the vertical slice

- **Status:** TODO
- **Priority:** High impact / low effort
- **Change:** Complete and record the mandatory prose-first Arrange-Act-Assert comparison after
  R2. Remove redundant temporary prose only after the final code makes the queued datagram, single
  stream Act, and exact `RawRequest` assertion clear.
- **Guardrails:** The test must retain one behavioral contract. Do not hide the causal datagram,
  stream Act, or expected payload/address in a fixture.
- **Done when:** The test has maintainer-reviewed readable AAA structure and one reason to fail.

### R4 - Record residual receiver ownership decisions

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Change:** Measure unit-only coverage and record separate aggregate/global and integration-only
  evidence when it informs the decision. Retain the pending, I/O-error, and termination branches at
  their existing Tokio, platform-fault-injection, and #1488 ownership boundaries.
- **Guardrails:** Do not add a percentage-only test or a hot-path socket abstraction.
- **Done when:** Each residual branch has a documented ownership decision.

## Progress Tracking

### Plan Checklist

- [x] Receiver source, `BoundSocket` boundary, loopback test feasibility, current unit-only
      coverage, integration coverage, and #1488 lifecycle ownership reviewed.
- [ ] Maintainer approved R1.
- [ ] R1 implemented, reviewed, validated, and committed.
- [ ] Maintainer approved R2.
- [ ] R2 implemented and focused validation passed.
- [ ] Maintainer approved R3 design review.
- [ ] R3 recorded, validated, and committed.
- [ ] R4 coverage/ownership review completed and decision recorded.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

### Progress Log

- 2026-09-11 - GitHub Copilot - Created this proposed plan after reviewing `Receiver`,
  `BoundSocket`, launcher and integration ownership, clean unit-only coverage, and the clarified
  unit-first coverage policy. No test or production change has been made.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1 | TODO | Awaiting maintainer approval. |
| R2 | TODO | Awaiting R1 completion and maintainer approval. |
| R3 | TODO | Awaiting R2 review. |
| R4 | TODO | Awaiting approved increments. |

## Non-Goals

- Do not change `Receiver`, `BoundSocket`, event publication, request admission, packet dispatch,
  response sending, or production lifecycle code.
- Do not test pending readiness, I/O errors, stream termination, socket teardown, or receive-loop
  cancellation; these belong to Tokio/platform fault injection or #1488.
- Do not replace existing package integration contracts, claim unit coverage from them, or create a
  mock socket abstraction solely for coverage.

## Validation Per Approved Increment

- Apply the mandatory prose-first Arrange-Act-Assert comparison before maintainer review.
- Run the focused `receiver::tests` target and then the package `--lib` target when the increment
  is approved for broader validation.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Measure aggregate/global, unit-only, and integration-only coverage separately whenever coverage
  informs a decision.

## Completion Criteria

- The normal datagram-to-`RawRequest` adapter has one direct, deterministic unit contract.
- The test makes the queued datagram, single stream Act, and exact payload/sender assertions visible
  without a generic fixture.
- Remaining pending, error, and termination branches have explicit ownership decisions rather than
  percentage-only tests.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
