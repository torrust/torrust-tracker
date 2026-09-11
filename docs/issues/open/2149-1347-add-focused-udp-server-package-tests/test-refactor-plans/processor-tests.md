---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/processor.rs
status: proposed
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
so the current direct processor tests construct `RawRequest` and use a bounded event-listener
fixture.

Both current tests use the same port-zero scenario. The first asserts two response-family totals
and accepted-connect count; the second asserts discarded count and accepted-connect count. Each
therefore has multiple reasons to fail, and the valid connect payload, listener, repository, and
bounded synchronization obscure which fact each assertion protects.

### Decision

Retain direct processor coverage because it is the deepest portable boundary for the port-zero
defense. Split the mixed assertions into focused contracts: no matching IPv4 response event,
exactly one discarded event, and no connect-handler acceptance. Use a narrowly named port-zero
scenario fixture only if it makes the coordinated processor/listener lifecycle explicit without
hiding the source-port condition, valid payload, processor Act, or observed metric. Keep the
absolute timeout; do not add sleeps, retries, raw sockets, or lifecycle redesign.

## Phase 2 - Assess Missing Behavior Tests

### Strengths to preserve

1. `Processor` owns the direct-caller port-zero guard and its discard-event publication.
2. `Launcher` owns normal receive-loop admission and active-request buffering.
3. `handlers::handle_packet` owns parsing and request-specific dispatch.
4. The statistics listener and request-discarded handler own eventual event consumption and metric
   update mechanics.
5. `BoundSocket` owns socket binding and send I/O.

### Problems and opportunities

#### P1 - Current tests conflate three observable processor contracts

**Problem.** Response suppression, discard-event publication, and handler bypass are independent
regression facts. Their combined assertions make it unclear which responsibility failed.

**Opportunity.** Make each contract a separately named test with one assertion. The response test
uses an IPv4 port-zero client and asserts only `udp4_responses_sent_total() == 0`; the discard test
asserts only `udp_requests_discarded_total() == 1`; the handler-bypass test asserts only
`udp4_connect_requests_accepted_total() == 0`. The valid connect payload remains deliberate: it
ensures a moved or removed early guard cannot pass merely because parsing failed.

#### P2 - Test setup hides coordinated asynchronous lifecycle details

**Problem.** The current tuple fixture, manual cancellation, and polling helper make ownership and
cleanup hard to scan.

**Opportunity.** Assess a state-named scenario fixture that owns the ephemeral environment,
listener cancellation, a port-zero client, and a valid raw connect request. Keep the test Act as
`processor.process_request(scenario.request).await`; expose only the one selected metric for the
assertion. Add it only if its name and fields clarify the coordinated condition better than the
current helpers.

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

### R1 - Split port-zero contracts by observable behavior

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Split the two existing tests into focused response-suppression, discard-event, and
  handler-bypass tests. Each has one Act and one assertion.
- **Guardrails:** Keep the port-zero source address and parsable connect payload visible. Assert
  only the matching IPv4 response total, not an irrelevant IPv6 total. Do not replace metrics with
  mocks or remove bounded settling before reading listener-produced metrics.
- **Done when:** Each failure identifies one processor guard responsibility.

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
- **Decision:** No code change. `setup_processor_with_stats_listener` names its narrow setup
  responsibility and exposes the consumed `Processor`, stats-owning `EnvContainer`, and explicit
  cancellation token. `connect_request_from(client_with_port_0)` keeps both the valid payload and
  port-zero causal state visible at each Act. A scenario fixture would make those fields indirect
  without simplifying listener cleanup or the bounded event-settling wait.
- **Prose-first review:** The temporary prose specified one port-zero request and one observable
  guard consequence per test: no IPv4 response, one discard event, or no accepted connect request.
  The final tests visibly pass the port-zero address to the valid-connect request, call
  `processor.process_request`, wait for the listener-produced metric to settle, and assert one
  selected counter. Temporary prose is redundant and removed.
- **Done when:** Fixture ownership, if kept, makes the test's state and cancellation explicit.

### R3 - Record residual ownership and coverage

- **Status:** DONE
- **Priority:** Low impact / low effort
- **Change:** Measure aggregate/global, unit-only, and integration-only coverage separately when
  it informs a decision. Record ownership for serialization, socket send, handler dispatch,
  event consumption, logging, sender absence, launcher admission, and lifecycle.
- **Guardrails:** Do not add percentage-only, raw-socket, tracing-capture, or collaborator-matrix
  tests.
- **Coverage evidence:** At the uncommitted R1 increment, clean reports measure this file at
  122/122 lines (100.00%), 175/175 regions (100.00%), and 19/19 functions (100.00%)
  aggregate/global; 109/122 lines (89.34%), 168/175 regions (96.00%), and 15/19 functions
  (78.95%) unit-only; and 34/34 lines (100.00%), 20/20 regions (100.00%), and 7/7 functions
  (100.00%) integration-only. The reports have different denominators because they include
  different test binaries and test-only code; they must not be combined into one percentage.
- **Residual ownership:** The normal packet-handler and response-send path is exercised by
  existing integration contracts. Response serialization belongs to `udp-protocol`; actual socket
  send success/failure belongs to `BoundSocket` and real-loopback tests; parsed packet dispatch
  belongs to handlers; and response/discard event consumption belongs to statistics handlers and
  listeners. Trace payload and failure logging are diagnostic implementation detail. The
  sender-disabled branch has no distinct observable contract. Launcher admission and asynchronous
  task/listener shutdown remain owned by #1488 and its UDP lifecycle subissues.
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
  port-zero guard coverage and three mixed assertion contracts. No test or production change has
  been made.
- 2026-09-11 - User/maintainer - Approved R1: split the portable direct port-zero guard tests by
  their independently observable processor behavior.
- 2026-09-11 - GitHub Copilot - Implemented R1. Focused tests now separately assert IPv4 response
  suppression, discard-event publication, and connect-handler bypass; formatting, focused tests,
  and diff checks passed.
- 2026-09-11 - GitHub Copilot - Completed R2 as a no-change decision. The existing narrow setup
  and request helpers preserve visible port-zero state and the direct processor Act; a scenario
  fixture would introduce indirection without clarifying resource ownership.
- 2026-09-11 - GitHub Copilot - Completed R3 with clean aggregate/global, unit-only, and
  integration-only reports. Packet handling, socket transport, protocol serialization, event
  consumption, logging, sender absence, launcher admission, and lifecycle behavior retain their
  existing owners; no coverage-only test is justified.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server server::processor::tests`, and `git diff --check` passed. The three focused contracts retain a valid connect payload and source-port-zero causal state. |
| R2 | DONE | Prose-first Arrange-Act-Assert and test-smell review completed. The existing narrow helpers require no scenario-fixture refactor. |
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

- Each retained port-zero test observes one processor-owned guard behavior with one assertion.
- The valid request, port-zero causal state, processor Act, bounded synchronization, and selected
  observable output remain readable.
- Socket, protocol, handler, listener, launcher, and lifecycle behavior remains at its current
  owner.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
