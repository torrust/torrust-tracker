---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/handlers/error.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/handlers/error.rs
    - packages/udp-server/src/handlers/mod.rs
    - packages/udp-server/src/error.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/statistics/event/handler/error.rs
    - packages/udp-server/src/banning/event/handler.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Handler Error Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to `packages/udp-server/src/handlers/error.rs`.

## Phase 1 - Clean Current Tests

### Current state

`handle_error` logs an error, optionally publishes a `UdpError` event, and returns a protocol error
response. The first existing test combines two independently observable outcomes: it asserts the
returned response transaction ID and the published event. The second test asserts the zero fallback
transaction ID when no event sender is present. Unit-only evidence before this plan is 132/154 lines
(85.71%), 129/145 regions (88.97%), and 13/14 functions (92.86%).

The current combined test has two reasons to fail. A response transaction-ID regression and an
event-publication regression are owned by different behavior branches and should be separate
contracts. Its Arrange also exposes broadcaster/receiver mechanics only because it tests event
publication; those mechanics must not appear in the response-only test.

### Decision

Split the combined test before adding behavior. Keep response transaction-ID routing in a
sender-disabled test, and keep event publication in a sender-enabled test. Do not use a fixture that
hides the causal sender state, request kind, event inputs, response transaction ID, or expected
published event. A narrowly named ordinary helper is allowed only for repeated valid service-binding
or internal-error construction.

## Phase 2 - Assess Missing Behavior Tests

### Strengths to preserve

1. `handle_error` owns response construction and optional server-error event publication.
2. `handlers/mod.rs` owns deciding when a parsed or unparsed request reaches `handle_error`.
3. `error.rs` owns conversion from protocol parse errors to the server `Error` type.
4. `event.rs` owns the stable `ErrorKind` classification consumed by statistics and banning.
5. Statistics and banning handlers/listeners own event consumption and metric/policy effects.

### Problems and opportunities

#### P1 - Response construction and optional event publication are coupled in one test

**Problem.** The existing sender-enabled test asserts both a response transaction ID and a published
event. A failure cannot identify whether response routing or event publication regressed.

**Opportunity.** Split it into one response contract and one event-publication contract. The response
test uses no event sender and asserts only the explicitly supplied transaction ID. The event test
uses an enabled broadcaster and asserts only the published `UdpError` carries the independently
specified request kind and error classification. Keep the event context's client address/public URL
visible only if those fields are selected as its observable event contract.

#### P2 - Logging branches are not behavior-focused test targets

**Decision.** Do not test warn/error level selection or transaction-ID log-field branches merely to
cover lines 70-74 and 90-104. They are diagnostic implementation details, and tracing-capture tests
would couple this unit suite to logging structure rather than response or publication behavior.

#### P3 - Error event context forwarding needs assessment after cleanup

**Decision.** After the split, assess whether the event test should assert one independently relevant
context field, such as the supplied public URL. Add it only if that protects handler-owned event
construction without duplicating `ConnectionContext`, `ErrorKind::from`, or event-consumer tests.
Do not broaden the event assertion into a conversion or consumer-policy matrix.

## Proposed Refactorings

Apply items in order. Complete one approved increment—including prose-first comparison, focused
validation, review, and its mapped commit point—before beginning the next item.

### R1 - Split response and event-publication contracts

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** Phase 1, P1
- **Change:** Replace the combined test with two focused tests. One uses no sender and asserts only
  an explicitly supplied transaction ID in the returned error response. One uses an enabled sender
  and asserts only the published event's selected routing payload.
- **Guardrails:** Each test has one Act and one assertion. Keep the sender condition explicit. Do
  not assert a response in the event test or an event in the response test. Do not create generic
  broadcaster, request, or error fixtures.
- **Result:** The response contract uses no sender and retains only the supplied transaction-ID
  assertion. The publication contract uses an enabled broadcaster and retains only the published
  event assertion. The existing no-sender zero-ID fallback test remains separate.
- **Done when:** Response routing and event publication have one failure reason each.

### R2 - Review the split test designs

- **Status:** DONE
- **Priority:** High impact / low effort
- **Change:** Perform the mandatory prose-first and test-code-smell review after R1. Verify that
  sender state, transaction ID, request kind, error classification, and every selected event-context
  field are visible from Arrange into Act/Assert.
- **Guardrails:** Hide only ordinary service-binding, UUID, and broadcaster mechanics. Do not hide
  a value that selects response routing or published-event meaning.
- **Prose-first review:** The temporary prose specified one response-routing contract and one event
  publication contract. The response test visibly retains its disabled sender and supplied
  transaction ID. The event test visibly retains its enabled sender, `Connect` request kind, and
  internal error. Each directly calls `handle_error` and has one assertion for its selected
  behavior. Temporary prose is redundant and removed. Event-context forwarding remains an explicit
  R3 assessment rather than an accidental wildcard assertion.
- **Done when:** Both tests communicate one behavior and one reason to fail without hidden data
  coupling.

### R3 - Assess one event-context forwarding contract

- **Status:** TODO
- **Priority:** Medium impact / low effort
- **Addresses:** P3
- **Change:** Decide whether one direct assertion for a selected event context field adds distinct
  handler-owned value after R1. Record a no-change decision when the existing event payload contract
  is sufficient.
- **Guardrails:** Do not test `ErrorKind` conversion, full `ConnectionContext` construction,
  statistics, banning, or listener behavior.
- **Done when:** The event-context test boundary is explicit.

### R4 - Record residual ownership and coverage

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Change:** Measure aggregate/global, unit-only, and integration-only coverage separately when it
  informs a decision. Record ownership for logging branches, lower-level error conversion,
  dispatcher routing, event consumers, and sender-disabled behavior not selected by R1.
- **Guardrails:** Do not add percentage-only logging or collaborator-matrix tests.
- **Done when:** Residual lines and behavior have documented owners.

## Progress Tracking

### Plan Checklist

- [x] Handler responsibility, current local tests, error conversion, dispatcher routing, event
      consumers, and unit-only coverage reviewed.
- [x] Maintainer approved R1.
- [x] R1 implemented and focused validation passed.
- [x] Maintainer approved R2 design review.
- [x] R2 recorded, validated, and committed.
- [ ] R3 event-context assessment completed and decision recorded.
- [ ] R4 coverage/ownership review completed and decision recorded.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

### Progress Log

- 2026-09-11 - GitHub Copilot - Created this proposed plan after reviewing
  `handlers/error.rs`, its local tests, dispatcher/error-conversion boundaries, event consumers, and
  unit-only line coverage. The existing combined response-and-event test has two independent failure
  reasons; no test or production change has been made.
- 2026-09-11 - User/maintainer - Approved R1. Split the combined response transaction-ID and
  published error-event contract into focused tests before adding any behavior.
- 2026-09-11 - User/maintainer - Reviewed and approved R2. The split tests retain visible sender,
  transaction-ID, request-kind, and error-classification values with one Act and one assertion each.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1/R2 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server handlers::error::tests`, and `git diff --check` passed. The combined test was split into one sender-disabled transaction-ID response contract and one sender-enabled event-publication contract; prose-first review confirms one reason to fail per test. |
| R3 | TODO | Awaiting approved cleanup. |
| R4 | TODO | Awaiting approved increments. |

## Non-Goals

- Do not change production error handling, protocol response serialization, event classification,
  dispatcher routing, statistics/banning behavior, logging format/level, or listener lifecycle.
- Do not test lower-level parse-error conversion, client-software classification, metrics/gauges,
  banning policy, sockets, tasks, or root composition.
- Do not add tracing-capture, mock-repository, mock-sender, generic broadcaster, or percentage-only
  tests.

## Validation Per Approved Increment

- Apply mandatory prose-first Arrange-Act-Assert and test-code-smell review before maintainer
  review.
- Run `cargo test -p torrust-tracker-udp-server handlers::error::tests`.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Measure aggregate/global, unit-only, and integration-only coverage separately whenever coverage
  informs a decision.

## Completion Criteria

- Response transaction-ID routing and optional event publication have separate focused tests.
- Every retained assertion observes handler-owned behavior rather than logging, conversion, or event
  consumer behavior.
- The plan records whether event-context forwarding has one independently valuable contract.
- Residual logging, conversion, routing, and event-consumer behavior remains at its existing owner.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
