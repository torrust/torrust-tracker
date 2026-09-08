---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/bound_socket.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/bound_socket.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/tests/server/contract.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/testing/refactoring-patterns/README.md
---

# UDP Bound Socket Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to `packages/udp-server/src/server/bound_socket.rs`.

## Phase 1 — Clean Current Tests

### Current state

`bound_socket.rs` has no colocated test module. Existing tests only exercise it incidentally through
the launcher, processor, and real-loopback package contracts.

### Decision

No refactoring increment is needed before new tests. There is no test code in this target file to
clean, and changing distant consumer tests would obscure the wrapper's own socket/metadata
contract. Record this no-change decision before Phase 2 begins.

## Phase 2 — Add Missing Behavior Tests

### Strengths to preserve

1. `BoundSocket::bind` establishes the package-owned invariant that every returned local port is
   non-zero, including port-zero requests delegated to the OS.
2. `address`, `url`, and `service_binding` are small public metadata adapters derived from the same
   bound socket.
3. `create_socket` deliberately leaves `IPV6_V6ONLY` unset when `ipv6_v6only` is false, preserving
   OS defaults rather than claiming a cross-platform dual-stack contract.

### Problems and opportunities

#### P1 — The port-zero invariant has no direct contract

**Problem.** No test proves that `BoundSocket::bind` returns a non-zero port when asked to bind
IPv4 loopback port zero.

**Opportunity.** Bind `127.0.0.1:0` and assert `address().port() != 0`.

#### P2 — Metadata adapters have no direct consistency contract

**Problem.** No test proves that `address`, `url`, and `service_binding` describe the same
successfully bound endpoint.

**Opportunity.** From one IPv4 loopback port-zero binding, independently assert UDP protocol, the
same bind address, and the expected `udp://<address>` URL representation.

#### P3 — Platform-specific dual-stack behavior must not be inferred

**Problem.** An IPv6 socket with `ipv6_v6only = false` has OS-dependent behavior.

**Decision.** Do not add a dual-stack reachability test. Existing package integration covers an
IPv6-only listener where supported. A future explicit IPv6-only metadata test needs a portability
review and availability guard.

## Proposed Refactorings

### R1 — Record the Phase 1 no-change decision

- **Status:** DONE
- **Priority:** High impact / trivial effort
- **Change:** Confirm this file has no existing tests to refactor and that adjacent tests remain at
  their established integration/consumer boundaries.
- **Done when:** Phase 1 is recorded complete with no cleanup code change.

### R2 — Cover port-zero binding

- **Status:** DONE
- **Priority:** High impact / low effort
- **Change:** Add one direct test that binds IPv4 loopback port zero and asserts the resulting port
  is non-zero.
- **Guardrails:** Use an OS-assigned port; do not reserve/release a port, sleep, retry, or make a
  real client request.
- **Done when:** the non-zero port invariant is asserted at the wrapper boundary.

### R3 — Cover endpoint metadata consistency

- **Status:** DONE
- **Priority:** High impact / low effort
- **Change:** Add one direct test from a bound IPv4 loopback socket asserting its address,
  `ServiceBinding` UDP protocol/address, and URL are consistent.
- **Guardrails:** Keep expected protocol/address/URL values independent and visible. Do not derive
  expected values using `BoundSocket::url` or `service_binding`, and do not test dual-stack policy.
- **Done when:** all public endpoint representations agree for one bound socket.

### R4 — Review Phase 2 test design

- **Status:** TODO
- **Priority:** Medium impact / low effort
- **Change:** After each added test, review Arrange–Act–Assert structure, fixture choice, and
  portability. Record no-change or an approved focused cleanup before beginning the next test.
- **Done when:** the final added test remains direct, deterministic, and free of unnecessary helper
  abstractions.

### R5 — Assess residual coverage

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Change:** Measure residual coverage and document why debug formatting, `Deref`, impossible
  OS-port-zero failure injection, or platform-specific dual-stack branches are covered elsewhere or
  intentionally deferred.
- **Done when:** remaining gaps have an ownership/portability rationale.

## Progress Tracking

### Plan Checklist

- [x] Target source and adjacent integration coverage reviewed.
- [x] Two-phase sequence applied; Phase 1 has no test code to refactor.
- [x] Maintainer approved R1.
- [x] R1 no-change decision recorded and committed.
- [x] Maintainer approved R2.
- [x] R2 implemented, reviewed, validated, and committed.
- [x] Maintainer approved R3.
- [x] R3 implemented, reviewed, validated, and committed.
- [ ] R4 design reviews completed and recorded.
- [ ] R5 assessment completed and decision recorded.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

### Progress Log

- 2026-09-08 11:38 UTC - GitHub Copilot - Created this two-phase proposed plan from
  `bound_socket.rs`, its launcher/processor consumers, and existing real-loopback integration
  coverage. Phase 1 has no target-file tests to clean; no test or production change has been made.
- 2026-09-08 11:42 UTC - User/maintainer - Reviewed and approved the Phase 1 no-change decision.
  `bound_socket.rs` has no existing colocated tests to refactor, so the next work may proceed to
  the separately approved Phase 2 port-zero behavior test.
- 2026-09-08 11:49 UTC - User/maintainer - Approved R2. Bind IPv4 loopback on port zero and assert
  only the non-zero returned port invariant; commit this plan update before test implementation.
- 2026-09-08 11:55 UTC - User/maintainer - Reviewed and approved R2. The direct test uses a Tokio
  runtime only because `BoundSocket::bind` constructs a Tokio UDP socket; it retains the narrow
  IPv4 loopback port-zero contract without client traffic, retries, sleeps, or dual-stack behavior.
- 2026-09-08 12:16 UTC - User/maintainer - Approved R3. Bind IPv4 loopback on port zero, retain the
  bound address as the independently observed endpoint, and assert that public URL and service
  binding representations use that same UDP endpoint. Commit the plan update before implementation.
- 2026-09-08 12:22 UTC - User/maintainer - Reviewed and approved R3. The direct test observes one
  bound IPv4 endpoint and independently verifies its URL and UDP service-binding representations,
  without client traffic, dual-stack assumptions, or production changes.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after plan review changes. |
| R1 | DONE | Maintainer approved the explicit no-change decision: there is no target-file test code to clean before Phase 2. |
| R2 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server bound_socket::tests`, and `git diff --check` passed. One Tokio-bound direct test covers the non-zero port invariant. |
| R3 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server bound_socket::tests`, and `git diff --check` passed. One direct test covers URL and UDP service-binding endpoint consistency. |
| R4 | TODO | Documented review after each Phase 2 test increment. |
| R5 | TODO | Coverage measurement or documented no-change decision. |

## Non-Goals

- Do not test UDP packet reception, processing, listener lifecycle, or application registration.
- Do not assert platform-default dual-stack reachability, add port handoff/retry logic, or inject an
  impossible OS-assigned port-zero error.
- Do not change `BoundSocket` production behavior or create a generic socket-test factory.

## Validation Per Approved Increment

- Run focused `bound_socket` unit tests.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- After each Phase 2 behavior test, review causal state, visible Act, independent expected value,
  and portability before the next increment.

## Completion Criteria

- Phase 1 no-change decision is explicit and justified.
- Phase 2 tests protect stable wrapper invariants without crossing into listener or dual-stack
  integration behavior.
- Every behavior test has a recorded post-test design review.
- The maintainer reviews all approved increments before the next file plan begins.
