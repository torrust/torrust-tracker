---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/states.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/states.rs
    - packages/udp-server/src/server/mod.rs
    - packages/udp-server/src/server/bound_socket.rs
    - packages/udp-server/src/server/spawner.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/testing/environment.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/drafts/1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
    - docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md
    - docs/issues/drafts/1488-si-17-migrate-standalone-udp-environment/ISSUE.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Server States Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to `packages/udp-server/src/server/states.rs`.

## Phase 1 - Clean Current Tests

### Current state

`states.rs` has one colocated test for `await_startup_notification`. It closes the startup sender,
returns a `BrokenPipe` launcher error from a task, and asserts that the resulting `UdpError::Launcher`
preserves that error. The Arrange makes the causal closed sender and launcher error visible, while
the task and channels are essential mechanics of the helper's error-precedence behavior. Clean
unit-only evidence before this assessment is 45/54 lines (83.33%), 45/57 regions (78.95%), and
10/14 functions (71.43%).

### Decision

No cleanup change is proposed. The sole direct test already expresses one deterministic
error-precedence contract. Do not extract a fixture: the channels and task are required by the SUT
and an abstraction would hide the causal task outcome or closed startup notification.

## Phase 2 - Assess Missing Behavior Tests

### Strengths to preserve

1. `Server::<Stopped>::start` composes socket binding, launcher startup notification, registration,
   registration-failure cleanup, and the typed transition to `Running`.
2. `await_startup_notification` owns the narrow precedence rule that a launcher error is retained
   when startup notification is closed.
3. `Server::<Running>::stop` owns halt signalling and launcher-task joining, but its lifecycle
   semantics are governed by #1488.
4. `server/mod.rs` owns the public transition test for registration-error preservation and actual
   listener release.
5. `BoundSocket`, `Spawner`, `Launcher`, the registrar, and the standalone environment own their
   respective binding, task, registration, and lifecycle concerns.

### Decisions

#### D1 - Retain the deterministic startup-error precedence test

Keep the existing `await_startup_notification` test. It is the narrowest direct test for the useful
non-socket behavior: a concrete launcher `BrokenPipe` takes precedence over a closed startup
notification.

#### D2 - Do not test representation-only state construction

Do not add tests for `Server::<Stopped>::new`, state aliases, derived constructors, or derived
`Display`. These tests would restate field assignment or macro-generated representation without a
package-owned behavior.

#### D3 - Do not duplicate registration-failure cleanup

Do not add a `states.rs` registration-failure test. The existing public `server/mod.rs` contract
asserts both `UdpError::Registration` source preservation and actual UDP listener release. Moving or
repeating it here would duplicate socket binding, task spawning, registration, and cleanup behavior.

#### D4 - Defer remaining startup and stop lifecycle paths

Do not add tests for bind errors, successful closed-startup notification, task join failures,
`Server::<Running>::stop`, halt signalling, receive-loop completion, or processor-task outcomes.
They require socket contention, task cancellation/abortion, joining, or shutdown policy. SI-14,
SI-15, and SI-17 under #1488 own the successor lifecycle design and tests.

## Proposed Refactorings

This plan has no test-producing increment. Record and validate the no-change decisions after
maintainer approval; do not add a fixture, mock, or production abstraction solely to increase
coverage.

### R1 - Record the reviewed no-change and lifecycle deferral decision

- **Status:** TODO
- **Priority:** High impact / low effort
- **Change:** Retain the current startup-error precedence test and record the representation,
  registration-cleanup, and #1488 lifecycle ownership decisions.
- **Guardrails:** Do not change production code, move existing tests, add a socket/task fixture, or
  create a percentage-only test.
- **Done when:** The plan records why no new `states.rs` test is appropriate and which existing test
  or issue owns each remaining behavior.

## Progress Tracking

### Plan Checklist

- [x] State transition responsibilities, existing unit/public-server/integration tests, unit-only
      evidence, and #1488 lifecycle ownership reviewed.
- [ ] Maintainer approved R1.
- [ ] R1 decision recorded, validated, and committed.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

### Progress Log

- 2026-09-11 - GitHub Copilot - Created this proposed plan after reviewing state transition
  responsibilities, the colocated startup-error precedence test, public registration cleanup,
  integration boundaries, and #1488 lifecycle ownership. No test or production change has been
  made.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1 | TODO | Awaiting maintainer approval. |

## Non-Goals

- Do not change state transitions, socket binding, registration, task spawning, error mapping, or
  shutdown behavior.
- Do not test derived representations, field assignment, aliases, or macro-generated display.
- Do not duplicate registration-error cleanup, real-loopback transport, or standalone environment
  coverage.
- Do not test halt cancellation, task abortion/joining, receive-loop completion, active-request
  draining, or shutdown policy before #1488's SI-14, SI-15, and SI-17 work is complete.

## Validation Per Approved Increment

- Run `cargo test -p torrust-tracker-udp-server states::tests`.
- Run `cargo test -p torrust-tracker-udp-server server::tests::it_should_preserve_registration_error_and_release_listener_when_registration_fails`.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Measure aggregate/global, unit-only, and integration-only coverage separately only when a new
  measurement materially informs an ownership decision.

## Completion Criteria

- The existing direct startup-error precedence test retains its focused deterministic contract.
- Each remaining state-layer behavior has a documented representation, public-transition, or #1488
  lifecycle owner.
- No fixture, mock, abstraction, or percentage-only test is introduced without a distinct
  package-owned behavioral reason.
- The maintainer reviews the no-change decision before the next file plan begins.
