---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/receiver.rs
status: completed
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

- **Status:** DONE
- **Priority:** High impact / trivial effort
- **Addresses:** Phase 1
- **Change:** Confirm that `receiver.rs` has no direct tests to clean and that integration coverage
  does not replace the feasible unit-test assessment.
- **Guardrails:** Do not move or refactor launcher, socket, or integration test code.
- **Decision:** `receiver.rs` has no colocated test code or concrete cleanup opportunity. The
  existing integration contracts retain their transport value but do not replace R2's feasible,
  focused unit test for the package-owned datagram-to-`RawRequest` adapter.
- **Done when:** The no-cleanup decision is recorded before adding a test.

### R2 - Cover queued loopback datagram adaptation

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Add one direct asynchronous unit test. Queue one explicit loopback datagram before
  awaiting one `Receiver::next()`, then assert that the returned `RawRequest` has the exact payload
  and independently captured client address.
- **Guardrails:** Keep the payload, expected sender address, datagram send, stream Act, and
  assertions visible. The sole timeout is an absolute diagnostic failure bound. Do not add a sleep,
  retry, polling loop, generic UDP helper, server task, event listener, or explicit teardown.
- **Prose-first review:** The temporary prose specified a receiver with a known IPv4 loopback
  datagram queued before a single stream Act returns its matching raw request. The final
  `ReceiverWithQueuedLoopbackDatagram` scenario owns only the coordinated socket binding, client
  binding, sender-address capture, and pre-Act datagram delivery. The test keeps the causal payload,
  `receiver.next()` Act, and one whole-value `RawRequest` assertion visible. `RawRequest` derives
  equality because bytes and sender address are meaningful value semantics, not merely test data.
  The timeout is an absolute diagnostic bound. Temporary prose is redundant and removed.
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

- **Status:** DONE
- **Priority:** Low impact / low effort
- **Change:** Measure unit-only coverage and record separate aggregate/global and integration-only
  evidence when it informs the decision. Retain the pending, I/O-error, and termination branches at
  their existing Tokio, platform-fault-injection, and #1488 ownership boundaries.
- **Guardrails:** Do not add a percentage-only test or a hot-path socket abstraction.
- **Decision:** Separate reports show `receiver.rs` unit-only coverage increased from 15/22 lines
  (68.18%), 18/31 regions (58.06%), and 3/3 functions (100%) to 55/56 lines (98.21%), 77/79
  regions (97.47%), and 7/7 functions (100%). Aggregate/global execution independently reports
  the same 55/56 lines, 77/79 regions, and 7/7 functions, while integration-only execution covers
  21/22 production lines (95.45%), 29/31 regions (93.55%), and 3/3 functions (100%). Do not add
  percentage-only tests for pending readiness, I/O error, or `None` termination: they require
  Tokio waker control, non-portable socket fault injection, or #1488 receive-loop lifecycle policy.
  Do not add a mock socket abstraction because it would add hot-path indirection to model those
  implementation mechanics without a distinct package contract.
- **Done when:** Each residual branch has a documented ownership decision.

## Progress Tracking

### Plan Checklist

- [x] Receiver source, `BoundSocket` boundary, loopback test feasibility, current unit-only
      coverage, integration coverage, and #1488 lifecycle ownership reviewed.
- [x] Maintainer approved R1.
- [x] R1 implemented, reviewed, validated, and committed.
- [x] Maintainer approved R2.
- [x] R2 implemented and focused validation passed.
- [x] Maintainer approved R3 design review.
- [x] R3 recorded, validated, and committed.
- [x] R4 coverage/ownership review completed and decision recorded.
- [x] Maintainer reviewed all approved changes.
- [x] Plan completed and ready for final verification.

### Progress Log

- 2026-09-11 - GitHub Copilot - Created this proposed plan after reviewing `Receiver`,
  `BoundSocket`, launcher and integration ownership, clean unit-only coverage, and the clarified
  unit-first coverage policy. No test or production change has been made.
- 2026-09-11 - User/maintainer - Approved R1. Record that `receiver.rs` has no direct test code
  to clean and retain the feasible R2 adapter unit-test assessment independently of integration
  coverage.
- 2026-09-11 - User/maintainer - Approved R2. Add the queued-loopback adapter test only, keeping
  the datagram send before the single stream Act and retaining only an absolute diagnostic timeout.
- 2026-09-11 - User/maintainer - Reviewed and approved the R2/R3 test design. Retain the focused
  `ReceiverWithQueuedLoopbackDatagram` scenario instead of generalizing it prematurely, and compare
  the whole `RawRequest` directly through its meaningful value equality.
- 2026-09-11 - User/maintainer - Approved R4. Measure aggregate/global, unit-only, and
  integration-only coverage separately; record residual pending, error, termination, and socket
  abstraction decisions without adding a percentage-only test.
- 2026-09-11 - User/maintainer - Reviewed and approved the completed receiver plan. The direct unit
  test protects the normal datagram-to-`RawRequest` adapter, while R4 records separate coverage
  evidence and retains pending, error, termination, and socket-abstraction behavior at their proper
  ownership boundaries.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | DONE | Markdown and spelling checks passed after all maintainer review changes. |
| R1 | DONE | The reviewed source has no direct test code or concrete cleanup opportunity. Existing integration coverage retains transport value but does not replace the feasible R2 unit-test assessment. |
| R2/R3 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server receiver::tests::should_yield_a_raw_request_with_the_received_datagram_and_sender_address`, and `git diff --check` passed. Prose-first and smell review replace a complex Arrange with a state-named queued-loopback scenario and two field assertions with one whole-value `RawRequest` assertion; the visible stream Act remains unchanged. |
| R4 | DONE | Separate clean reports passed. `receiver.rs` unit-only coverage increased from 15/22 lines (68.18%), 18/31 regions (58.06%), and 3/3 functions (100%) to 55/56 lines (98.21%), 77/79 regions (97.47%), and 7/7 functions (100%). Aggregate/global separately reports the same result; integration-only separately reports 21/22 production lines (95.45%), 29/31 regions (93.55%), and 3/3 functions (100%). Pending, I/O-error, termination, and mock-abstraction paths have explicit ownership decisions. |
| Plan completion | DONE | Maintainer reviewed all approved increments and evidence before the next file plan begins. |

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
