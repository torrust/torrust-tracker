---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/processor.rs
status: completed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/processor.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/bound_socket.rs
    - packages/udp-server/src/server/request_buffer.rs
    - packages/udp-server/src/statistics/event/listener.rs
    - packages/udp-server/src/statistics/event/handler/request_discarded.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Processor Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to `packages/udp-server/src/server/processor.rs`.

## Phase 1 - Clean Current Tests

### Current state

`Processor::process_request` has a defense-in-depth guard for source port zero. It must discard the
request before packet dispatch or response sending and, when configured, publish a
`UdpRequestDiscarded` event. The launcher has the equivalent normal-path guard; the processor
protection serves other direct callers. Standard UDP sockets cannot originate a port-zero request,
so the direct processor test constructs `RawRequest` and receives the emitted server event.

The prior tests mixed response-total, accepted-connect, and discard-count assertions. They also
polled a statistics repository using sleeps, while cancelling but not joining a listener task.
Those mechanics gave a direct processor test multiple failure reasons and unnecessary lifecycle
ownership.

### Decision

Retain direct processor coverage because it is the deepest portable boundary for the port-zero
defense. Keep one focused contract for its directly observable behavior: a parsable port-zero
request publishes `UdpRequestDiscarded`. Receive that event from the server event bus under an
absolute deadline. Response suppression and handler bypass are early-return consequences, but do
not have an independent positive processor output without indirect consumer assertions or
timing-based event absence. Do not add sleeps, retries, raw sockets, listener lifecycle, or a
scenario fixture.

## Phase 2 - Assess Missing Behavior Tests

### Strengths to preserve

1. `Processor` owns the direct-caller port-zero guard and its discard-event publication.
2. `Launcher` owns normal receive-loop admission and active-request buffering.
3. `handlers::handle_packet` owns parsing and request-specific dispatch.
4. The statistics listener and request-discarded handler own eventual event consumption and metric
   update mechanics.
5. `BoundSocket` owns socket binding and send I/O.

### Problems and opportunities

#### P1 - Prior tests conflate a direct processor fact and indirect consequences

**Problem.** The discard event is directly observable at the processor event-bus boundary.
Response-total and accepted-connect assertions require an asynchronous statistics consumer, so
they also fail for consumer scheduling or cleanup mechanics.

**Decision.** Retain one direct event-bus test asserting only `UdpRequestDiscarded`. Its parsable
connect payload remains deliberate: a guard that moves after packet handling no longer produces
the direct discard outcome. Do not assert event absence to prove response suppression or handler
bypass, because that depends on elapsed time rather than a positive observed fact.

#### P2 - Test setup hides coordinated asynchronous lifecycle details

**Problem.** The prior tuple fixture, manual cancellation, and polling helper made ownership and
cleanup hard to scan.

**Decision.** No scenario fixture is needed. The narrow setup returns only the consumed
`Processor` and direct event receiver. The source-port condition and parsable request remain at the
Act, and the test owns no listener task.

#### P3 - Send serialization, packet dispatch, and socket failure branches have other owners

**Decision.** Do not add tests for response serialization/write failure, actual UDP send failure,
normal request dispatch, trace payload logging, or absent event sender merely for coverage.
Protocol response serialization belongs to `udp-protocol`; request dispatch belongs to handlers;
transport send behavior belongs to `BoundSocket`/integration contracts; logging is diagnostic; and
sender-disabled behavior has no distinct observable output at this boundary.

#### P4 - Port-zero receive-loop and shutdown work remain deferred

**Decision.** Do not use these direct tests to redesign the launcher guard, receive-loop behavior,
request-task ownership, listener cancellation, or shutdown. Those paths remain owned by #1488 and
its UDP lifecycle subissues.

## Proposed Refactorings

Apply items in order. Complete one approved increment, including prose-first comparison, focused
validation, review, and its mapped commit point, before beginning the next item.

### R1 - Replace indirect port-zero checks with direct event observation

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Replace mixed listener-produced metric assertions with one direct event-bus contract
  for `UdpRequestDiscarded`.
- **Guardrails:** Keep the port-zero source address and parsable connect payload visible. Receive
  one event under an absolute deadline and assert it once. Do not use event absence, metrics,
  mocks, polling, sleeps, or a listener task.
- **Result:** The test directly observes the processor-owned discard event with one Act and one
  assertion, without an asynchronous consumer or cleanup resource.
- **Done when:** A failure identifies the processor's port-zero discard-event responsibility.

### R2 - Assess a state-named asynchronous scenario fixture

- **Status:** DONE
- **Priority:** Medium impact / low effort
- **Addresses:** P2
- **Change:** Compare the current tuple helpers with a scenario fixture named for the port-zero
  direct-processor state. Adopt it only when it exposes the coordinated initial state and resource
  cleanup more clearly without hiding causal source address, valid request, processor Act, or
  selected assertion value.
- **Guardrails:** Do not introduce a generic environment factory, production container factory,
  sleep/retry synchronization, or new lifecycle abstraction.
- **Decision:** No scenario fixture is needed. `setup_processor_with_event_receiver` names its
  narrow setup and exposes only the consumed `Processor` and direct event receiver.
  `connect_request_from(client_with_port_0)` keeps valid payload and causal port-zero state visible
  at the Act. A scenario fixture would add indirection without simplifying state or cleanup.
- **Prose-first review:** The temporary prose specified that a parsable request from port zero
  produces a discard event. The final test visibly passes the port-zero address to the valid
  connect request, calls `processor.process_request`, and asserts the received event. Temporary
  prose is redundant and removed.
- **Done when:** Fixture ownership, if kept, makes the test's state and cancellation explicit.

### R3 - Record residual ownership and coverage

- **Status:** DONE
- **Priority:** Low impact / low effort
- **Change:** Measure aggregate/global, unit-only, and integration-only coverage separately when
  it informs a decision. Record ownership for serialization, socket send, handler dispatch,
  event consumption, logging, sender absence, launcher admission, and lifecycle.
- **Guardrails:** Do not add percentage-only, raw-socket, tracing-capture, or collaborator-matrix
  tests.
- **Coverage evidence:** Before the final direct-event cleanup, clean reports measured this file at
  122/122 lines (100.00%), 175/175 regions (100.00%), and 19/19 functions (100.00%)
  aggregate/global; 109/122 lines (89.34%), 168/175 regions (96.00%), and 15/19 functions
  (78.95%) unit-only; and 34/34 lines (100.00%), 20/20 regions (100.00%), and 7/7 functions
  (100.00%) integration-only. The reports have different denominators because they include
  different test binaries and test-only code; they must not be combined into one percentage.
- **Residual ownership:** Normal packet handling and response sending are exercised by existing
  integration contracts. Response serialization belongs to `udp-protocol`; actual socket send
  success/failure belongs to `BoundSocket` and real-loopback tests; parsed packet dispatch belongs
  to handlers; and event consumption belongs to statistics handlers and listeners. Trace payload
  and failure logging are diagnostic detail. The sender-disabled branch has no distinct observable
  contract. Launcher admission and asynchronous task/listener shutdown remain owned by #1488 and
  its UDP lifecycle subissues.
- **Done when:** Residual lines and behavior have documented owners.

## Progress Tracking

### Plan Checklist

- [x] Processor responsibility, existing direct tests, launcher, handlers, socket, and listener
      boundaries reviewed.
- [x] Maintainer approved R1.
- [x] R1 implemented and focused validation passed.
- [x] R2 design review completed.
- [x] R2 completed as a no-change decision.
- [x] R3 coverage/ownership review completed and decision recorded.
- [x] Maintainer reviewed all approved changes.
- [x] Plan completed and ready for final verification.

### Progress Log

- 2026-09-11 - GitHub Copilot - Created this proposed plan after confirming direct portable
  port-zero guard coverage and mixed indirect assertion contracts. No test or production change has
  been made.
- 2026-09-11 - User/maintainer - Approved R1: split the portable direct port-zero guard tests by
  the direct processor event and its indirect consequences.
- 2026-09-11 - GitHub Copilot - Implemented R1 with a direct event-bus discard contract;
  formatting, focused tests, and diff checks passed.
- 2026-09-14 - GitHub Copilot - Corrected the test after final review found polling, sleeps, and
  unjoined listener ownership. The final direct event observation uses one bounded receive and one
  assertion; response suppression and handler bypass remain unselected indirect consequences.
- 2026-09-14 - GitHub Copilot - Completed R2 as a no-change decision. The direct receiver setup
  and request helper preserve visible port-zero state and the production Act; a scenario fixture
  would introduce indirection without clarifying state or cleanup.
- 2026-09-11 - GitHub Copilot - Completed R3 with clean aggregate/global, unit-only, and
  integration-only reports. Packet handling, socket transport, protocol serialization, event
  consumption, logging, sender absence, launcher admission, and lifecycle behavior retain their
  existing owners; no coverage-only test is justified.

### Validation Evidence

> Formatting claims recorded before 2026-09-14 are stable-rustfmt results; see the
> [formatting validation correction](README.md#formatting-validation-correction-2026-09-14).

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server server::processor::tests`, and `git diff --check` passed after direct event observation replaced polling and listener ownership. The valid connect payload and source-port-zero state remain visible. |
| R2 | DONE | Prose-first Arrange-Act-Assert and test-smell review completed. The direct receiver setup needs no scenario-fixture refactor. |
| R3 | DONE | Clean `cargo llvm-cov` aggregate/global, `--lib`, and `--test integration` reports were collected after R1; see `coverage-evidence.md` for figures and scope interpretation. |

## Non-Goals

- Do not change processor, launcher, socket, protocol, handler, listener, repository, event, or
  lifecycle production behavior.
- Do not add raw-socket port-zero, serialization, socket-failure, normal-dispatch, logging,
  sender-absence, or shutdown tests.
- Do not introduce generic test factories, mocks, sleeps, retries, or percentage-only tests.

## Validation Per Approved Increment

- Apply mandatory prose-first Arrange-Act-Assert and test-code-smell review before maintainer
  review.
- Run `cargo test -p torrust-tracker-udp-server server::processor::tests`.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Measure aggregate/global, unit-only, and integration-only coverage separately whenever coverage
  informs a decision.

## Completion Criteria

- The retained port-zero test observes one processor-owned guard behavior with one assertion.
- The valid request, port-zero causal state, processor Act, bounded direct receive, and selected
  observable output remain readable.
- Socket, protocol, handler, listener, launcher, and lifecycle behavior remains at its current
  owner.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
