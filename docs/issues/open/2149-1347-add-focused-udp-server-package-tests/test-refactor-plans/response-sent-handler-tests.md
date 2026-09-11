---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/event/handler/response_sent.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/event/handler/response_sent.rs
    - packages/udp-server/src/statistics/event/handler/mod.rs
    - packages/udp-server/src/statistics/metrics.rs
    - packages/udp-server/src/statistics/repository.rs
    - packages/udp-server/src/event.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Response-Sent Handler Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to
`packages/udp-server/src/statistics/event/handler/response_sent.rs`.

## Phase 1 - Clean Current Tests

### Current state

`handle_event` maps a sent response to the response-total counter and, for successful responses,
updates the request-kind-specific processing-time average. The two current tests send
`Event::UdpResponseSent` through the parent statistics dispatcher and assert the aggregate IPv4 or
IPv6 response total. Both use an `Ok { Announce }` response and repeat a large inline connection
context, an announce-request builder, one-second duration, dispatcher event wrapper, repository,
and clock setup.

The current tests preserve valuable parent-dispatcher plus IP-family aggregate-counter contracts,
but their names and assertions do not make the response handler's success-only average route
visible. Their Arrange exposes several incidental fields needed only to construct an event rather
than the behavioral difference selected by the assertion.

### Decision

Retain the current dispatcher-level IPv4 and IPv6 total-counter contracts unless their cleanup can
preserve the same boundary with clearer causal data. Do not replace them with a direct child-handler
test. Separately assess a direct `handle_event` unit contract for one successful request kind and
one processing duration, asserting only that kind's observable average. Do not add a matrix for all
request kinds, IP families, results, label construction, metric collection, or parent dispatch.

## Phase 2 - Assess Missing Behavior Tests

### Strengths to preserve

1. The parent statistics dispatcher owns routing `Event::UdpResponseSent` to this module.
2. This handler owns successful-response request-kind labeling, processing-average update, and
   response-total counter label selection.
3. `statistics/metrics.rs` owns metric aggregation/accessor implementation.
4. `statistics/repository.rs` owns metric persistence and locking.
5. `event.rs` owns the response/request kind representations.

### Problems and opportunities

#### P1 - No direct contract selects the success-only processing-time route

**Problem.** The existing total-counter assertions execute a successful announce event indirectly,
but do not assert the processing average, which is the distinct behavior selected only for
`UdpResponseKind::Ok`. An accidental removal of the average update can pass both existing tests.

**Opportunity.** Add one direct test that supplies `UdpResponseKind::Ok { Connect }` and one
readable processing duration to this module's `handle_event`, then asserts only
`udp_avg_connect_processing_time_ns_averaged()`. This is a deterministic, package-local contract:
the handler maps the successful response to the connect-labeled performance metric. The request
kind and duration remain visible from Arrange to Act to Assert.

#### P2 - Error responses intentionally have no request-kind processing average

**Decision.** Do not add an error-response companion merely to execute `LabelValue::ignore()` or
to prove the absence of an average. It would require a negative collaborator/metric assertion and
would mostly duplicate the error-response classification and generic total-counter behavior at
other boundaries. Add it only if a concrete regression demonstrates that the successful-response
contract cannot protect the branch meaningfully.

#### P3 - Per-kind and IP-family metric matrices are already represented elsewhere

**Decision.** Do not add connect/announce/scrape or IPv4/IPv6 permutations solely for coverage.
`UdpRequestKind` display/label representation belongs to `event.rs`; metric aggregation/accessors
belong to `statistics/metrics.rs`; the retained parent-dispatcher tests already select IPv4 and
IPv6 response counting. One direct connect-average route is sufficient for the selected
handler-owned behavior.

#### P4 - Counter-write error logging is not a behavior-focused target

**Decision.** Do not introduce a failing repository or tracing capture just to cover the counter
write error arm. Repository failure and logging are collaborator/diagnostic behavior, not an
observable response-metric handler contract.

## Proposed Refactorings

Apply items in order. Complete one approved increment, including prose-first comparison, focused
validation, review, and its mapped commit point, before beginning the next item.

### R1 - Add one direct successful-response processing-average contract

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Call this module's `handle_event` directly with a loopback IPv4 connection context,
  `UdpResponseKind::Ok { Connect }`, and a readable duration. Assert only the connect average
  value in the statistics repository.
- **Guardrails:** Keep the selected request kind and duration visible. Use a single assertion. Do
  not derive the expected average with production metric code, assert the response total, or add a
  collaborator mock/failing repository.
- **Done when:** A regression that stops updating the successful connect processing average fails
  a direct, deterministic handler test.

### R2 - Review existing total-counter tests for focused readability

- **Status:** DONE
- **Priority:** Medium impact / low effort
- **Change:** Perform prose-first Arrange-Act-Assert and test-smell review of the existing IPv4
  and IPv6 parent-dispatcher counter tests. Apply only a small cleanup that makes their IP-family
  and total-counter contract clearer without hiding the dispatcher Act or changing their level.
- **Guardrails:** Do not create a generic event factory, hide IP family, convert the tests into a
  table/matrix, or merge their independent IPv4/IPv6 contracts.
- **Decision:** No code change. Each retained test makes the sole relevant initial-state difference
  visible: its IPv4 or IPv6 `ConnectionContext`. Each sends one concrete `UdpResponseSent` event
  through the parent dispatcher and asserts one corresponding IP-family total. A shared event or
  context helper would hide the dispatcher input and the IP-family condition, while a matrix would
  couple two independent contracts.
- **Prose-first review:** The temporary prose specified that one IPv4 or IPv6 sent-response event
  increments the matching aggregate total through the parent dispatcher. The final tests visibly
  provide their concrete event/context, retain the dispatcher Act, and have one typed total
  assertion. Temporary prose is redundant and removed.
- **Done when:** Retained parent-dispatcher tests each communicate their one selected total-counter
  behavior and one reason to fail.

### R3 - Record residual ownership and coverage

- **Status:** DONE
- **Priority:** Low impact / low effort
- **Change:** Measure aggregate/global, unit-only, and integration-only coverage separately when
  it informs a decision. Record ownership for error responses, other request kinds, label
  representation, metric aggregation, counter-write logging, parent dispatcher routing, and
  listener lifecycle.
- **Guardrails:** Do not add percentage-only, negative-absence, tracing-capture, or
  collaborator-matrix tests.
- **Coverage evidence:** At the uncommitted R1 increment, clean reports measure this file at
  129/130 lines (99.23%), 172/174 regions (98.85%), and 8/8 functions (100.00%)
  aggregate/global; 122/130 lines (93.85%), 149/174 regions (85.63%), and 8/8 functions
  (100.00%) unit-only; and 40/41 lines (97.56%), 91/93 regions (97.85%), and 2/2 functions
  (100.00%) integration-only. The reports have different denominators because they include
  different test binaries and test-only code; they must not be combined into one percentage.
- **Residual ownership:** Error-response `LabelValue::ignore()` and the absence of a
  processing-average update are not selected as negative collaborator/metric contracts. Announce
  and scrape request-kind label mapping is represented by `event.rs`, while metric
  aggregation/accessors belong to `statistics/metrics.rs`. Counter-write failure logging is
  diagnostic repository-collaborator behavior. Parent routing is owned by the statistics event
  dispatcher and listener lifecycle by its listener. Existing integration contracts retain their
  real-loopback boundary without replacing the direct successful-connect unit contract.
- **Done when:** Residual lines and behavior have documented owners.

## Progress Tracking

### Plan Checklist

- [x] Handler responsibility, existing local tests, parent dispatcher, metric repository, and
      event representation reviewed.
- [x] Maintainer approved R1.
- [x] R1 implemented and focused validation passed.
- [x] R2 design review completed.
- [x] R2 completed as a no-change decision.
- [x] R3 coverage/ownership review completed and decision recorded.
- [x] Maintainer reviewed all approved changes.
- [x] Plan completed and ready for final verification.

### Progress Log

- 2026-09-11 - GitHub Copilot - Created this proposed plan after confirming the existing
  parent-dispatcher IPv4/IPv6 total-counter tests and the direct successful-response
  processing-average seam. No test or production change has been made.
- 2026-09-11 - User/maintainer - Approved R1: add one direct successful-connect processing-average
  contract while retaining existing parent-dispatcher counter contracts.
- 2026-09-11 - GitHub Copilot - Implemented R1. The direct handler call makes `Connect` and its
  one-second processing duration visible and asserts only the observable `1_000_000_000`
  nanosecond connect average. Focused formatting, tests, and diff checks passed.
- 2026-09-11 - GitHub Copilot - Completed R2 as a no-change decision. The existing IPv4/IPv6
  parent-dispatcher total-counter tests retain visible causal IP-family context, one Act, and one
  assertion; a helper or matrix would obscure that contract.
- 2026-09-11 - GitHub Copilot - Completed R3 with clean aggregate/global, unit-only, and
  integration-only reports. Error-response absence, request-kind representation, metric
  aggregation, counter-write logging, parent routing, and listener lifecycle retain their existing
  owners; no coverage-only test is justified.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server statistics::event::handler::response_sent::tests`, and `git diff --check` passed. The direct successful-connect contract asserts only the `1_000_000_000` nanosecond processing average. |
| R2 | DONE | Prose-first Arrange-Act-Assert and test-smell review completed. The existing parent-dispatcher IPv4/IPv6 response-total tests require no cleanup. |
| R3 | DONE | Clean `cargo llvm-cov` aggregate/global, `--lib`, and `--test integration` reports were collected after R1; see `coverage-evidence.md` for figures and scope interpretation. |

## Non-Goals

- Do not change production response handling, dispatcher routing, metric definitions, repository
  behavior, event representations, counter-write logging, or listener lifecycle.
- Do not duplicate request-kind label conversion, metric aggregation/accessors, IP-family counter
  matrices, error classification, transport, or root composition tests.
- Do not add tracing-capture, mock repository, generic event factory, property/matrix, or
  percentage-only tests.

## Validation Per Approved Increment

- Apply mandatory prose-first Arrange-Act-Assert and test-code-smell review before maintainer
  review.
- Run `cargo test -p torrust-tracker-udp-server statistics::event::handler::response_sent::tests`.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Measure aggregate/global, unit-only, and integration-only coverage separately whenever coverage
  informs a decision.

## Completion Criteria

- Existing parent-dispatcher IP-family total-counter protection remains clear and valuable.
- At most one direct successful response processing-average route is added when approved.
- Every retained assertion observes handler-owned behavior, rather than representation,
  aggregation, repository internals, logging, or listener lifecycle.
- Residual error, label, aggregation, routing, and lifecycle behavior remains at its existing
  owner.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
