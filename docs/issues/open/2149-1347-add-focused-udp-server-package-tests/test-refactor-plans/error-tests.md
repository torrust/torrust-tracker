---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/error.rs
status: completed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/error.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/handlers/mod.rs
    - packages/udp-server/src/handlers/error.rs
    - packages/udp-protocol/src/request.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Server Parse-Error Adapter Test Plan

Follow the shared [purpose, quality goals, and plan structure](README.md). This plan applies only
to `packages/udp-server/src/error.rs`.

## Phase 1 — Identify Problems

### Strengths to preserve

1. `SendableRequestParseError` preserves connection and transaction identifiers only when a
   malformed UDP packet can receive an error response.
2. The adapter cleanly separates protocol parsing ownership in `udp-protocol` from UDP server
   response-routing metadata.
3. `Error::from(RequestParseError)` consistently wraps the converted server representation as
   `Error::InvalidRequest`.
4. `event.rs` and `handlers/error.rs` already consume this server error at their appropriate
   classification and wire-response boundaries.

### Problems and opportunities

#### P1 — Parse-error routing metadata has no direct contract

**Problem.** `From<RequestParseError> for SendableRequestParseError` is not tested directly.

**Why it matters.** Losing a sendable error's connection or transaction identifier prevents the UDP
server from addressing its error response correctly. Conversely, preserving invented identifiers on
an unsendable parse error would be incorrect.

**Opportunity.** Add two direct deterministic tests: one sendable protocol error must preserve both
identifiers and message; one unsendable protocol error must preserve its message while clearing both
optional identifiers.

#### P2 — Outer error wrapping is not directly asserted

**Problem.** `From<RequestParseError> for Error` is only covered incidentally through later handler
and event behavior.

**Why it matters.** The wrapper is the explicit server boundary used by `handlers/mod.rs` before
constructing a UDP error response and event fact.

**Opportunity.** Add one focused test that converts a sendable parse error through `Error` and
asserts its `Error::InvalidRequest` payload retains the converted identifiers and message.

#### P3 — Display formatting is not an independent behavior target

**Problem.** `SendableRequestParseError::fmt` is used by the error event classification, but a
separate formatting test could duplicate the event-plan request-parse test.

**Decision.** Do not add a standalone `Display` test unless a consumer requires a distinct stable
operator-facing message. `event.rs` already verifies the resulting request-parse classification
contains the full adapter display representation.

#### P4 — Protocol parser variants are out of scope

**Problem.** It would be easy to use parser byte inputs to obtain source errors.

**Why it matters.** That would duplicate `udp-protocol` parser tests and obscure the UDP server
adapter contract.

**Opportunity.** Construct `RequestParseError::sendable_text` and `RequestParseError::unsendable_text`
directly. Use fixed protocol identifier values; do not invoke `Request::parse_bytes`.

## Phase 2 — Proposed Refactorings

Apply items in order. Complete one approved increment—including review, focused validation, and the
mapped commit point—before beginning the next item.

### R1 — Cover sendable and unsendable parse-error conversion

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1, P4
- **Change:** Add one test for `RequestParseError::sendable_text` and one test for
  `RequestParseError::unsendable_text`. Assert the message and both optional identifiers explicitly.
- **Guardrails:** Keep source errors, numeric identifiers, and expected adapter values visible.
  Do not parse bytes, create sockets, or add generic error fixtures.
- **Done when:** sendable errors retain both response-routing identifiers and unsendable errors have
  no identifiers.

### R2 — Cover outer invalid-request wrapping

- **Status:** DONE
- **Priority:** Medium impact / low effort
- **Addresses:** P2
- **Change:** Convert one sendable `RequestParseError` directly into `Error` and assert the
  `InvalidRequest` payload preserves its message and identifiers.
- **Guardrails:** Assert the typed `Error::InvalidRequest` variant. Do not test final response
  serialization or event classification here.
- **Done when:** the outer UDP server error boundary has one readable typed conversion contract.

### R3 — Assess residual adapter coverage

- **Status:** DONE
- **Priority:** Low impact / low effort
- **Addresses:** P3
- **Change:** Review remaining coverage after R1/R2. Record a no-change decision for wrapper
  conversions already exercised by handlers, `ConnectionCookie` display, or display formatting
  already protected at a distinct consumer boundary.
- **Guardrails:** Do not add percentage-only tests or reproduce `udp-protocol` parser matrices.
- **Decision:** No test added. Current `error.rs` coverage is 57/62 lines (91.94%), 79/82 regions
  (96.34%), and 7/8 functions (87.50%) at commit `441a7512`. The direct parse-error conversion
  boundary is now covered by R1 and R2. `handlers/announce.rs` and `handlers/scrape.rs` cover
  wrapping UDP-core service errors at their handler boundary; `handlers/mod.rs` owns protocol parse
  failure routing; `handlers/error.rs` owns wire-response and emitted-event behavior; and `event.rs`
  owns stable event classification. `ConnectionCookie` display and residual wrapper conversions do
  not expose a distinct UDP-server behavior beyond those boundaries.
- **Done when:** each residual direct gap has a documented ownership/boundary decision.

## Progress Tracking

### Plan Checklist

- [x] Phase 1 findings reviewed against the adapter, protocol source error, and downstream consumers.
- [x] Phase 2 refactorings ordered by impact and effort.
- [x] Maintainer approved implementation of R1.
- [x] R1 implemented, reviewed, validated, and committed.
- [x] Maintainer approved implementation of R2.
- [x] R2 implemented, reviewed, validated, and committed.
- [x] R3 assessment completed and decision recorded.
- [x] Maintainer reviewed all approved changes.
- [x] Plan completed and ready for final verification.

### Progress Log

- 2026-09-08 11:31 UTC - GitHub Copilot - Created this proposed plan from `error.rs`, the direct
  `RequestParseError` source variants in `packages/udp-protocol/src/request.rs`, and existing event
  and handler boundaries. No test or production change has been made.
- 2026-09-08 10:35 UTC - User/maintainer - Approved R1. Construct sendable and unsendable protocol
  errors directly, then assert the message and response-routing identifier presence or absence.
  Commit the plan update before test implementation.
- 2026-09-08 10:45 UTC - User/maintainer - Reviewed and approved R1. The two direct adapter tests
  preserve the protocol/server boundary and explicitly assert response-routing identifier retention
  or absence without parsing packets or constructing handlers.
- 2026-09-08 11:13 UTC - User/maintainer - Approved R2. Convert one fixed sendable protocol error
  through `Error` and assert its typed `InvalidRequest` payload retains the message and both
  response-routing identifiers. Commit the plan update before test implementation.
- 2026-09-08 11:20 UTC - User/maintainer - Reviewed and approved R2. The direct typed conversion
  test retains the source error, conversion Act, and `Error::InvalidRequest` message/identifier
  assertions without crossing into response serialization or event classification.
- 2026-09-08 11:30 UTC - GitHub Copilot - Completed R3 assessment. The refreshed package-source
  report gives `error.rs` 91.94% lines, 96.34% regions, and 87.50% function coverage. The remaining
  direct paths are `ConnectionCookie` display or wrapper conversions already protected by their
  handler, error-response, or event-classification boundaries; no additional adapter test is
  justified.
- 2026-09-08 11:34 UTC - User/maintainer - Reviewed and approved the completed parse-error adapter
  plan. R1/R2 cover the server-owned response-routing conversion and typed wrapper, while R3 records
  the justified no-change decision for residual display and downstream-boundary paths.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after plan review changes. |
| R1 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server error::tests`, and `git diff --check` passed. Two direct adapter tests cover sendable and unsendable parse-error conversion. |
| R2 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server error::tests`, and `git diff --check` passed. One typed conversion test covers `Error::InvalidRequest` wrapping. |
| R3 | DONE | No change: 91.94% lines, 96.34% regions, and 87.50% functions. Residual display/wrapper paths are covered at handler, response, or event boundaries. |
| Plan completion | DONE | Maintainer reviewed all approved increments and decisions before the next file plan begins. |

## Non-Goals

- Do not change UDP server errors, protocol errors, response serialization, event classification, or
  event payloads.
- Do not test protocol byte parsing, connection-cookie validation, socket behavior, handler services,
  event buses, or logging.
- Do not add an error builder or generic fixture; direct protocol error construction is clearer.

## Validation Per Approved Increment

- Run focused `error::tests`.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Review that each test keeps source error, conversion Act, and exact typed adapter result visible.

## Completion Criteria

- Every approved test is deterministic and adapter-focused.
- Tests preserve the boundary: `udp-protocol` owns parsing and `udp-server` owns response-routing
  metadata conversion.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
