---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/event.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/error.rs
    - packages/udp-server/src/handlers/error.rs
    - packages/udp-server/src/statistics/event/handler/error.rs
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Server Event Test Refactor Plan

Follow the shared [purpose, quality goals, and plan structure](README.md). This plan applies only
to `packages/udp-server/src/event.rs`.

## Phase 1 — Identify Problems

### Strengths to preserve

1. The module explains the objective-fact event policy and links
  `docs/adrs/20260727000000_events_are_objective_facts.md` where a reader encounters the event
  schema.
2. `ErrorKind::from(Error)` is a narrow deterministic adapter from internal/server/domain errors to
   a stable event classification used by statistics and banning consumers.
3. `UdpRequestKind` keeps its wire/request data while mapping independently to stable metric labels
   and display values.
4. Existing handler tests already prove selected emitted `Event::UdpError` values. Direct tests here
   can verify the classification adapter without requiring socket, service, listener, or event-bus
   setup.

### Problems and opportunities

#### P1 — Error classification has no direct behavioral tests

**Problem.** `ErrorKind::from(Error)` has no local test module despite handling parsing, cookie,
whitelist, database, internal-server, and authentication error families.

**Why it matters.** The mapping determines both the error facts emitted by `handlers/error.rs` and
which `Event::UdpError` values allow the statistics and banning consumers to classify a failure.
A change can silently turn a connection-cookie error into a non-cookie classification or collapse a
stable consumer-facing category.

**Opportunity.** Add table-oriented unit cases using independently constructed source errors and
expected `ErrorKind` values. Use each test case only where it represents a distinct output category;
do not duplicate every wrapper path that maps to the same variant.

#### P2 — Request-kind metric representations have no local contract

**Problem.** The conversion and display implementations for `UdpRequestKind::{Connect, Announce,
Scrape}` have no direct tests.

**Why it matters.** These values become request-kind labels in server metrics. An accidental spelling
or mapping change affects observability without necessarily breaking protocol behavior.

**Opportunity.** Add a small table-driven unit test that independently expects `connect`, `announce`,
and `scrape` for `LabelValue` and `Display`. Construct only the minimum valid announce request
fixture required by the enum variant; do not test announce protocol parsing here.

#### P3 — The test boundary must not duplicate adjacent ownership

**Problem.** Error construction can tempt tests to reproduce UDP-core cookie validation, tracker-core
whitelist/database logic, or the event consumers' metric increments.

**Why it matters.** Those tests would be slower, more coupled, and duplicate coverage at a lower or
later boundary.

**Opportunity.** Keep all new cases synchronous and adapter-focused. Assert the exact `ErrorKind`
variant plus an independently specified stable message or message fragment. Retain
`handlers/error.rs` for emitted-event/wire-error behavior and
`statistics/event/handler/error.rs` for counter-consumption behavior.

## Phase 2 — Proposed Refactorings

Apply items in order. Complete one approved increment—including review, focused validation, and the
mapped commit point—before beginning the next item.

### R1 — Cover distinct `ErrorKind` classifications

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1, P3
- **Change:** Add direct, deterministic unit tests for one representative of each distinct output:
  request parse, connection cookie, whitelist, database, internal server, and tracker
  authentication. Group table cases only when their source setup remains readable and each expected
  classification is visible.
- **Guardrails:** Use concrete error values and independently specified expected `ErrorKind` values.
  Do not assert log text, create event-bus fixtures, invoke handlers, or re-test UDP-core/tracker-core
  behavior. Keep exact expected values adjacent to their assertions, rather than placing them in
  Arrange. Do not add a generic error factory with optional unrelated error families.
- **Done when:** each stable event error category has one readable adapter contract, and equivalent
  announce/scrape wrapper paths are covered only where they produce a distinct classification.

### R2 — Cover request-kind label and display mappings

- **Status:** IN_PROGRESS
- **Priority:** Medium impact / low effort
- **Addresses:** P2
- **Change:** Add table-driven test cases for `Connect`, `Announce`, and `Scrape` label/display
  values. Use a local minimal `AnnounceRequest` fixture only for the `Announce` variant; an
  inline case table must retain each concrete request-kind input and exact string value visibly.
- **Guardrails:** The announce fixture must be minimal and local. Do not derive expected labels by
  calling production conversion code or make metric-repository assertions.
- **Done when:** all three request kinds have exact independently specified `LabelValue` and display
  contracts.

### R3 — Assess residual event-schema coverage

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Addresses:** P1–P3
- **Change:** Review uncovered lines/functions after R1 and R2 against existing handler and consumer
  tests. Record a no-change decision for enum derives, sender/receiver type aliases, event-bus
  aliases, or paths already covered at the event-emission/consumption boundary.
- **Guardrails:** Do not add tests solely to increase a percentage and do not modify the event schema
  or ADR-defined objective-fact policy.
- **Done when:** remaining direct coverage gaps have a documented ownership/boundary decision.

## Progress Tracking

### Plan Checklist

- [x] Phase 1 findings reviewed against current source, event ADR, handlers, and consumers.
- [x] Phase 2 refactorings ordered by impact and effort.
- [x] Maintainer approved implementation of R1.
- [x] R1 implemented, reviewed, validated, and committed.
- [x] Maintainer approved implementation of R2.
- [ ] R2 implemented, reviewed, validated, and committed.
- [ ] R3 assessment completed and decision recorded.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

### Progress Log

- 2026-09-08 08:13 UTC - GitHub Copilot - Created this proposed plan from `event.rs`, its direct
  error sources, the existing error-event emitter/consumer tests, and
  `docs/adrs/20260727000000_events_are_objective_facts.md`. No test or production change has been
  made.
- 2026-09-08 08:19 UTC - User/maintainer - Approved R1 and requested repository-relative paths
  whenever an ADR is mentioned in this plan. Commit the approved plan update before implementing
  event-classification tests.
- 2026-09-08 08:53 UTC - User/maintainer - Approved a readability refinement for R1: keep source
  error construction in Arrange and place exact expected `ErrorKind` values directly beside their
  assertions. Commit the plan update before applying the test-only refactor.
- 2026-09-08 09:01 UTC - User/maintainer - Reviewed and approved R1. The six direct classification
  tests retain concrete source errors in Arrange, exact expected values beside assertions, and a
  stable-fragment whitelist assertion because its message includes source-location information.
- 2026-09-08 09:13 UTC - User/maintainer - Approved R2. Use a small local announce fixture and
  table cases that retain the request kind and independent expected `connect`, `announce`, or
  `scrape` representation visibly. Commit the plan update before test implementation.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after plan review changes. |
| R1 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server event::tests`, and `git diff --check` passed. Six classification tests are deterministic and test-only; the public test info hash has a narrow DevSkim suppression. |
| R2 | TODO | Focused event unit tests, formatting, and diff checks. |
| R3 | TODO | Test or documented no-change decision. |

## Non-Goals

- Do not change event variants, error classifications, event payloads, or the objective-fact policy.
- Do not duplicate `handlers/error.rs` response/event-emission tests, error-counter consumer tests,
  UDP-core cookie validation, tracker-core whitelist/database behavior, or UDP-protocol parsing.
- Do not add listener, socket, database, clock, or random-data setup for this adapter-level work.
- Do not create a broad cross-module error fixture or builder.

## Validation Per Approved Increment

- Run focused `event` unit tests.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Review that each test's source error and independently specified `ErrorKind` expectation remain
  visible before beginning the next increment.

## Completion Criteria

- Every approved test is deterministic and adapter-focused.
- Error classifications preserve the objective-fact event contract without duplicating lower-layer
  error behavior or later event-consumer behavior.
- Metric label/display tests specify expected values independently of the production conversion.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
