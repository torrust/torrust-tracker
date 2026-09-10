---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/event/handler/error.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/event/handler/error.rs
    - packages/udp-server/src/statistics/metrics.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/statistics/event/handler/mod.rs
    - packages/udp-server/src/handlers/announce.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Error-Metric Handler Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to
`packages/udp-server/src/statistics/event/handler/error.rs`.

## Phase 1 - Clean Current Tests

### Current state

The handler has one direct asynchronous test. It verifies that an IPv4 UDP error event increments
the aggregate IPv4 error metric, but it mixes a full inline connection context, event construction,
repository setup, event handling, and metric assertion without Arrange-Act-Assert headings or
named ordinary setup. At commit `23889a84`, unit-only coverage is 71/106 lines (66.98%), 70/173
regions (40.46%), and 10/11 functions (90.91%).

### Decision

Start with a mandatory prose-first Arrange-Act-Assert cleanup of the existing general-error metric
test. Use helpers only when they name coherent ordinary event/context setup and maintain a single
abstraction level. Keep the causal error/request-kind input, direct `handle_event` Act, and one
metric assertion visible. Do not create a general metrics fixture or derive expected metric values
through production code.

## Phase 2 - Add Missing Behavior Tests

### Strengths to preserve

1. `error::handle_event` owns routing one `Event::UdpError` payload into general and conditional
   connection-ID metric updates.
2. `event.rs` owns conversion of internal errors into `ErrorKind`; these tests must construct the
   classification directly rather than reproduce conversion behavior.
3. `statistics/event/handler/mod.rs` owns dispatch from the event enum; these tests call the
   local error handler directly.
4. `statistics/metrics.rs` and `statistics/repository.rs` own metric aggregation/query behavior.
5. `torrust-peer-id` owns peer-client classification. A fixed QBitTorrent-style peer ID may select
   an already-known client label, but tests must not reproduce the parser's variant matrix.

### Problems and opportunities

#### P1 - General-error request-kind label routing is not directly protected

**Problem.** The existing test covers an IPv4 event without a request kind, but not the handler's
`request_kind` label insertion for parsed requests.

**Opportunity.** Add one direct error event with `UdpRequestKind::Connect` and assert only the
general error metric query for the `connect` request-kind label. Do not duplicate event
classification or metric collection arithmetic.

#### P2 - Announce connection-cookie errors have an untested client-software metric route

**Problem.** The conditional branch increments the connection-ID-error counter only when a
connection-cookie error belongs to an announce request, labelling it by client software name and
version.

**Opportunity.** Add one direct announce `UdpRequestKind` with a fixed QBitTorrent-style peer ID
and `ErrorKind::ConnectionCookie`. Assert only the connection-ID-error metric associated with its
independently specified client-software labels. Do not test non-announce suppression, peer-ID
parsing, or the general-error counter in the same test.

#### P3 - Peer-client mapping variants are not this handler's responsibility

**Decision.** Do not create a table for every `PeerClient` variant. The handler's metric-routing
contract needs one representative known client and can defer unknown/other classification to the
peer-ID library and a future targeted observability need.

## Proposed Refactorings

Apply items in order. Complete one approved increment—including prose-first comparison, focused
validation, review, and its mapped commit point—before beginning the next item.

### R1 - Clarify the general IPv4 error metric contract

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** Phase 1
- **Change:** Write temporary Arrange-Act-Assert prose for the existing IPv4 error metric test.
  Refactor until a named ordinary IPv4 connection context, direct error-handler Act, and one
  aggregate IPv4 error assertion express that prose.
- **Guardrails:** Do not add a behavior case, listener, socket, clock abstraction, or broad fixture.
  Keep the independently constructed request-parse error visible.
- **Prose-first review:** The temporary Arrange prose was “an IPv4 request-parse error has no
  parsed request kind and uses an empty metrics repository.”
  `sample_ipv4_connection_context` names ordinary context construction, while the test retains the
  direct request-parse classification. The Act now calls this file's local `error::handle_event`,
  rather than the parent event router, and the Assert has one aggregate IPv4 error-metric fact.
  Temporary prose is redundant and removed.
- **Done when:** redundant prose can be removed and the test has one metric assertion.

### R2 - Cover general-error request-kind metric routing

- **Status:** IN_PROGRESS
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Add one unit test for a connect-kind error event and assert only its general-error
  metric route labelled `request_kind=connect`.
- **Guardrails:** Do not also assert aggregate IPv4/IPv6 totals, connection-ID-error metrics, event
  conversion, listener dispatch, or metric arithmetic.
- **Done when:** a regression in request-kind label routing has one direct, deterministic failure.

### R3 - Cover announce cookie-error client-software metric routing

- **Status:** TODO
- **Priority:** High impact / low effort
- **Addresses:** P2, P3
- **Change:** Add one unit test with a direct `ConnectionCookie` classification and minimal announce
  request using a fixed QBitTorrent-style peer ID. Assert only the client-software-labelled
  connection-ID-error metric.
- **Guardrails:** Keep the selected client label/version independently specified. Do not test the
  peer-ID parser, general error metric, ban counter, or event emission.
- **Done when:** the conditional announce-cookie route has one readable contract.

### R4 - Review residual metric-routing coverage

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Change:** Apply prose-first review after each test and measure unit-only coverage. Record why
  unselected peer-client variants, repository failures, or metric aggregation remain at their
  existing ownership boundaries.
- **Guardrails:** Do not add percentage-only cases or broaden the peer-client variant matrix.
- **Done when:** each residual branch has an ownership decision.

## Progress Tracking

### Plan Checklist

- [x] Handler, current test, metric ownership, event classification, and unit-only coverage reviewed.
- [x] Maintainer approved R1.
- [x] R1 implemented, reviewed, validated, and committed.
- [x] Maintainer approved R2.
- [ ] R2 implemented, reviewed, validated, and committed.
- [ ] Maintainer approved R3.
- [ ] R3 implemented, reviewed, validated, and committed.
- [ ] R4 coverage/ownership review completed and decision recorded.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

### Progress Log

- 2026-09-10 - GitHub Copilot - Created this proposed plan after reviewing the error-metric
  handler, current local test, event/router ownership, metric query boundaries, representative
  announce fixture support, and unit-only coverage. No test or production change has been made.
- 2026-09-10 - User/maintainer - Approved R1. Apply the prose-first cleanup to the existing IPv4
  general-error metric test only; commit this plan update before modifying the test.
- 2026-09-10 - User/maintainer - Reviewed and approved R1. The cleaned test directly exercises the
  error-metric handler with a visible request-parse classification and one IPv4 aggregate error
  metric assertion; ordinary connection context setup is named locally.
- 2026-09-10 - User/maintainer - Approved R2. Add one direct unit test for a connect-kind
  request-parse error and assert only the general error metric labelled `request_kind=connect`.
  Do not assert aggregate totals, client-software metrics, conversion, routing, or metric arithmetic.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server statistics::event::handler::error::tests::should_increase_the_udp4_errors_counter_when_it_receives_a_udp4_error_event`, and `git diff --check` passed. Prose-first review keeps request-parse classification, local handler Act, and one aggregate IPv4 metric assertion visible. |
| R2 | IN_PROGRESS | Maintainer approved the single `request_kind=connect` general-error metric route. |
| R3 | TODO | Awaiting R2 review. |
| R4 | TODO | Awaiting approved increments. |

## Non-Goals

- Do not change event classification, listener dispatch, metrics repository behavior, peer-ID
  parsing, ban policy, or production error-metric logic.
- Do not create sockets, event buses, listeners, databases, sleeps, polling, or generic fixtures.
- Do not test every client-software variant or combine general-error and connection-ID-error
  assertions in one test.

## Validation Per Approved Increment

- Apply the mandatory prose-first Arrange-Act-Assert comparison before maintainer review.
- Run focused `statistics::event::handler::error` tests.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Measure unit-only coverage when coverage informs a decision.

## Completion Criteria

- The existing aggregate-error test has a clear causal input, direct handler Act, and one metric
  assertion.
- Each new test protects exactly one handler-owned metric-routing decision.
- Event classification, peer-client parsing, metric aggregation, and event dispatch remain at their
  existing ownership boundaries.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
