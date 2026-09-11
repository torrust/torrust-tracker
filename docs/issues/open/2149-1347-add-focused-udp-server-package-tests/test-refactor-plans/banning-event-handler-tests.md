---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/banning/event/handler.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/banning/event/handler.rs
    - packages/udp-server/src/banning/event/listener.rs
    - packages/udp-core/src/services/banning.rs
    - packages/udp-server/src/statistics/repository.rs
    - packages/udp-server/src/statistics/metrics.rs
    - packages/udp-server/tests/server/contract.rs
    - tests/banning/udp_metrics_disabled_port_zero.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Banning Event-Handler Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to `packages/udp-server/src/banning/event/handler.rs`.

## Phase 1 - Clean Current Tests

### Current state

`handler.rs` has no colocated tests. Its direct `handle_event` function consumes an already
classified `Event::UdpError` with `ErrorKind::ConnectionCookie`, records the context client IP in
`BanService`, then publishes the resulting number of distinct tracked client IPs to
`udp_tracker_server_ips_banned_total`. The clean unit-only inventory reports 28/29 lines (96.55%),
but that indirect execution is not a focused handler contract.

Existing tests remain at their proper boundaries: `banning/event/listener.rs` owns event reception
and lifecycle; `udp-core` `BanService` tests own counters, thresholds, banning, and resets; the
statistics repository/metrics own gauge mechanics; package contracts own real UDP behavior; and
root tests own multi-listener metrics and banning composition.

### Decision

No cleanup increment is proposed because this handler has no direct test code. Do not move listener,
BanService, repository, integration, or root tests. The feasible focused unit contract must assess
this handler's event-to-IP-and-gauge orchestration independently of indirect coverage.

## Phase 2 - Add Missing Behavior Tests

### Strengths to preserve

1. The handler owns translating an already classified connection-cookie error event into a
   `BanService` counter update and an updated distinct-client-IP gauge.
2. `BanService` owns counter storage, ban threshold semantics, `is_banned`, and reset behavior.
3. Statistics repository and metrics modules own gauge storage, aggregation, conversion, and time
   handling.
4. The listener owns event reception, lag handling, cancellation, and task lifetime.
5. Root tests own listener configuration, cross-listener sharing, REST aggregation, and externally
   observable policy.

### Problems and opportunities

#### P1 - Cookie-error orchestration has no direct handler contract

**Problem.** Existing tests exercise this handler only through its listener or higher-level
transport/composition paths. They do not directly state that the event context's client IP is
recorded and that the post-update distinct tracked-IP total is published.

**Why it matters.** A refactor can use the wrong source IP, fail to record the event, publish a stale
total, or publish an event-count/threshold value instead of the post-update distinct-IP total.
Higher-level failures would be less local and less diagnostic.

**Opportunity.** Add two direct, focused tests. First, send one already-classified connection-cookie
event from a visible client IP and assert only that the handler records its error. Second, start
with `BanService` tracking one unrelated IP, send an event from a visible second IP, and assert only
that the domain-oriented gauge accessor reports two distinct tracked IPs. The nonempty second-test
state prevents a hard-coded `1` or a mistaken event-count gauge from passing.

#### P2 - Non-cookie events and collaborator failure paths have no additional handler value

**Decision.** Do not add a non-cookie event matrix: listener tests already prove the ignored-event
boundary, while a local test would only repeat the classification guard. Do not mock or inject a
repository to induce its logging-only failure path; that would test repository/logging mechanics or
add an unjustified production abstraction.

## Proposed Refactorings

Apply items in order. Complete one approved increment—including prose-first comparison, focused
validation, review, and its mapped commit point—before beginning the next item.

### R1 - Record the Phase 1 no-change decision

- **Status:** DONE
- **Priority:** High impact / trivial effort
- **Addresses:** Phase 1
- **Change:** Confirm that `handler.rs` has no direct tests to clean and that existing listener,
  BanService, repository, integration, and root contracts remain at their current boundaries.
- **Guardrails:** Do not move or refactor collaborator tests.
- **Decision:** `handler.rs` has no colocated test code or concrete cleanup opportunity. Listener,
  `BanService`, repository, integration, and root contracts remain at their existing boundaries.
  R2 separately assesses the feasible direct handler orchestration contract.
- **Done when:** The no-cleanup decision is recorded before adding a test.

### R2 - Cover cookie-error event orchestration

- **Status:** TODO
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Add two direct asynchronous tests with direct
  `Event::UdpError { ErrorKind::ConnectionCookie }` inputs. The first starts with an empty
  `BanService` and asserts only the event client IP count. The second begins with an unrelated IP
  tracked by a small state-named helper, then asserts only the post-update distinct-IP gauge total.
- **Guardrails:** Construct classified events directly and use a fixed timestamp. Keep each causal
  client IP, local handler Act, and one observable assertion visible. Do not assert threshold/banning
  status, labels, timestamps, listener behavior, protocol conversion, or repository internals.
- **Design revision:** The initial candidate combined client-IP forwarding and gauge publication in
  one test. Review identified two independent failure reasons, so it was split into two focused
  contracts. `BanningHandlerTestContext::with_one_tracked_client` names only the coordinated
  pre-existing tracked-IP state for the gauge test; it does not perform the production Act or
  interpret its result.
- **Prose-first review:** The temporary prose distinguished the two handler responsibilities. The
  first test visibly carries `cookie_error_client_ip` from its event context to its one counter
  assertion. The second visibly carries `unrelated_client_ip` into the one-tracked-client state,
  carries a second `cookie_error_client_ip` into the event context, and specifies
  `expected_distinct_client_ip_total` before the Act and in its one gauge assertion.
  `BanningHandlerTestContext` hides only ordinary `Arc<RwLock<BanService>>` and `Repository`
  construction. Temporary prose is redundant and removed.
- **Done when:** Wrong-IP forwarding, missing counter update, or stale/wrong distinct-IP gauge value
  yields a focused direct deterministic handler-contract failure.

### R3 - Review the test design after the vertical slice

- **Status:** TODO
- **Priority:** High impact / low effort
- **Change:** Complete and record mandatory prose-first Arrange-Act-Assert and test-code-smell
  review. Use a scenario fixture only if several coordinated operations obscure the two-IP initial
  state; otherwise keep the seeded and causal IPs visible inline.
- **Guardrails:** Each test must retain one observable result and one reason to fail. The first test
  owns client-IP forwarding; the second owns post-update distinct-IP gauge publication.
- **Done when:** The test has maintainer-reviewed AAA structure and a clear coordinated outcome.

### R4 - Record residual handler ownership decisions

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Change:** Measure unit-only coverage and retain aggregate/global and integration-only evidence
  separately when it informs a decision. Record deferrals for non-cookie events, repository failure,
  threshold policy, listener lifecycle, and root composition.
- **Guardrails:** Do not add percentage-only tests or use higher-level coverage to substitute for the
  direct unit contract.
- **Done when:** Each residual branch has a documented owner.

## Progress Tracking

### Plan Checklist

- [x] Handler responsibility, `BanService`, repository, listener, package-contract, root-composition,
      and current unit-only coverage boundaries reviewed.
- [x] Maintainer approved R1.
- [x] R1 implemented, reviewed, validated, and committed.
- [x] Maintainer approved R2.
- [x] R2 implemented and focused validation passed.
- [x] Maintainer approved R3 design review.
- [x] R3 recorded, validated, and committed.
- [ ] R4 coverage/ownership review completed and decision recorded.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

### Progress Log

- 2026-09-11 - GitHub Copilot - Created this proposed plan after reviewing direct handler behavior,
  listener tests, `BanService` ownership, repository/metric ownership, package contracts, root
  composition tests, and the unit-only inventory. No test or production change has been made.
- 2026-09-11 - User/maintainer - Approved R1. Record that `handler.rs` has no direct test code to
  clean and retain all collaborator contracts at their existing boundaries before assessing R2.
- 2026-09-11 - User/maintainer - Approved the revised R2 design. Split the initial combined
  assertion into client-IP forwarding and distinct-IP gauge publication contracts, then refine the
  Arrange sections so behavioral data remains visible and ordinary collaborator mechanics are
  hidden in a focused test context.
- 2026-09-11 - User/maintainer - Reviewed and approved the R3 design review. Retain the local
  `sample_connection_context` helper because client IP is its visible causal variation; do not add
  a premature cross-module fixture abstraction.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1 | DONE | The reviewed source has no colocated test code or concrete cleanup opportunity. Listener, `BanService`, repository, integration, and root tests retain their current ownership boundaries. |
| R2/R3 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server banning::event::handler::tests`, and `git diff --check` passed. The initial multi-assertion test was split into two focused contracts. Prose-first and smell review keep client IPs and expected gauge total visible across Arrange, Act, and Assert while the focused test context hides only ordinary collaborator mechanics. |
| R4 | TODO | Awaiting approved increments. |

## Non-Goals

- Do not change production banning, event classification, `BanService`, repository/metrics,
  listener lifecycle, package transport contracts, or root application composition.
- Do not test ban thresholds, `is_banned`, resets, counter storage, gauge labels/timestamps,
  event-bus reception, cancellation, logging, or repository failure behavior.
- Do not add sockets, server tasks, event listeners, sleeps, polling, mocks, or a generic fixture.

## Validation Per Approved Increment

- Apply mandatory prose-first Arrange-Act-Assert and test-code-smell review before maintainer
  review.
- Run the focused `banning::event::handler` test target and then the package `--lib` target when an
  increment is approved for broader validation.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Measure aggregate/global, unit-only, and integration-only coverage separately whenever coverage
  informs a decision.

## Completion Criteria

- A direct unit test protects the connection-cookie event's client-IP forwarding and post-update
  distinct-IP gauge publication as one handler-owned orchestration result.
- The test keeps its seeded IP, causal client IP, direct classified event, local handler Act, and
  observable result visible without duplicating collaborator implementation details.
- `BanService`, repository/metrics, listener, transport, and root composition remain at their
  existing ownership boundaries.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
