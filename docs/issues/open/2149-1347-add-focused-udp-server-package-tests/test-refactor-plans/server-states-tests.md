---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/states.rs
status: completed
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

#### D4 - Cover the remaining deterministic startup-notification mappings

Add direct tests for the two `await_startup_notification` outcomes that the existing test does not
cover: a closed startup notification with a successfully finished launcher maps to
`UdpError::StartupNotification`, and a closed startup notification with a failed launcher task join
maps to `UdpError::FailedToStartOrStopServer`. Both use the same visible closed-sender Arrange as
the current test, differ only in the task outcome, and need no socket, container, or registrar.

#### D5 - Defer bind and stop lifecycle paths

Do not add tests for bind errors, `Server::<Running>::stop`, halt signalling, receive-loop
completion, or processor-task outcomes. They require socket contention or exercise the legacy
shutdown mechanism that SI-14, SI-15, and SI-17 under #1488 are replacing. Document this ownership
in the module so future maintainers know the deferral is intentional.

## Proposed Refactorings

Apply items in order. Complete one approved increment—including prose-first comparison, focused
validation, review, and its mapped commit point—before beginning the next item.

### R1 - Record the reviewed no-change and lifecycle deferral decision

- **Status:** DONE
- **Priority:** High impact / low effort
- **Change:** Retain the current startup-error precedence test and record the representation,
  registration-cleanup, and #1488 lifecycle ownership decisions.
- **Guardrails:** Do not change production code, move existing tests, add a socket/task fixture, or
  create a percentage-only test.
- **Decision:** Retain the current direct `await_startup_notification` error-precedence test and the
  existing public `server/mod.rs` registration-error and listener-release contract. Do not add
  representation-only tests for state construction or derived types. Defer bind-error, closed-startup
  success, task join failure, stop, halt, receive-loop, and processor-task paths to #1488 SI-14,
  SI-15, and SI-17 because they require lifecycle ownership, cancellation, joining, or shutdown
  policy that this issue must not define.
- **Done when:** The plan records why no new `states.rs` test is appropriate and which existing test
  or issue owns each remaining behavior.
- **Revision:** After the maintainer asked which lines remained uncovered and whether they were
  hard to test, the two remaining `await_startup_notification` mappings were reclassified as cheap
  deterministic contracts. R2 and R3 below supersede the blanket no-change decision for those two
  branches only; the bind and `stop` deferrals stand.

### R2 - Cover the remaining startup-notification error mappings

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** D4
- **Change:** Add two direct asynchronous tests beside the existing one. The first drops the
  startup sender while the launcher task returns `Ok(Spawner)` and asserts
  `UdpError::StartupNotification`. The second drops the startup sender and aborts the task before
  awaiting it, asserting `UdpError::FailedToStartOrStopServer`.
- **Guardrails:** Keep the closed sender and the task outcome visible in each Arrange. Use no
  socket, container, registrar, or `Launcher`. Each test asserts one error variant. Do not assert the
  inner message text beyond what identifies the variant.
- **Prose-first review:** The temporary prose distinguished a closed startup sender plus either a
  successfully completed launcher or an explicitly aborted launcher task. The final tests retain
  `drop(tx_start)`, `launcher_task_with_successful_result`, and visible `task.abort()` as their
  causal input/output relationships. The direct `await_startup_notification` Act and one typed
  error-variant assertion remain visible. The helper hides only repeated incidental `Spawner`
  construction. Temporary prose is redundant and removed.
- **Done when:** Each `await_startup_notification` branch has one focused deterministic test.

### R3 - Document the module test-ownership boundary

- **Status:** TODO
- **Priority:** Medium impact / trivial effort
- **Addresses:** D5
- **Change:** Add a short module-level comment to `states.rs` stating which behavior is unit tested
  here, which is protected at the public `server/mod.rs` boundary, and which lifecycle paths are
  intentionally deferred to #1488 so future maintainers do not mistake the gap for an oversight.
- **Guardrails:** Keep the comment factual and brief; do not restate the plan or add speculative
  future design.
- **Done when:** A reader of `states.rs` can locate each behavior's test owner without opening the
  issue documents.

### R4 - Record final coverage and residual ownership

- **Status:** DONE
- **Priority:** Low impact / low effort
- **Change:** Measure unit-only coverage after R2 and record it alongside separate aggregate/global
  and integration-only figures. Confirm the bind-error and `stop` lines remain the only intentional
  gaps.
- **Guardrails:** Do not add percentage-only tests or a socket/task fixture for the deferred paths.
- **Decision:** Clean reports show aggregate/global and unit-only coverage of 72/77 lines (93.51%),
  91/102 regions (89.22%), and 16/20 functions (80.00%). Integration-only coverage separately
  reports 27/37 lines (72.97%), 16/32 regions (50.00%), and 7/11 functions (63.64%). The reports
  are not combined. Remaining unit-only executable lines are 105 (bind-error conversion), 184 and
  190-191 (`Running::stop` halt/task failure mapping), and 232 (the existing test's defensive
  fallback). Bind failure remains at the `BoundSocket` and public-start boundary; `stop` remains
  #1488 lifecycle work; the defensive fallback is not behavior to force through a test.
- **Done when:** The remaining uncovered lines are enumerated with their owners.

## Progress Tracking

### Plan Checklist

- [x] State transition responsibilities, existing unit/public-server/integration tests, unit-only
      evidence, and #1488 lifecycle ownership reviewed.
- [x] Maintainer approved R1.
- [x] R1 decision recorded, validated, and committed.
- [x] Maintainer approved R2.
- [x] R2 implemented and focused validation passed.
- [x] Maintainer approved R3.
- [x] R3 design review recorded, validated, and committed.
- [x] R4 coverage/ownership review completed and decision recorded.
- [x] Maintainer reviewed all approved changes.
- [x] Plan completed and ready for final verification.

### Progress Log

- 2026-09-11 - GitHub Copilot - Created this proposed plan after reviewing state transition
  responsibilities, the colocated startup-error precedence test, public registration cleanup,
  integration boundaries, and #1488 lifecycle ownership. No test or production change has been
  made.
- 2026-09-11 - User/maintainer - Approved R1. Record the reviewed no-change decision: retain the
  existing startup-error precedence and public registration-cleanup contracts, decline
  representation-only tests, and defer all remaining task/channel/shutdown behavior to #1488.
- 2026-09-11 - User/maintainer - Asked which lines remained uncovered and whether they were hard to
  test. Fresh unit-only coverage listed lines 95, 149, 152, 174, 180, 181, and 215. Lines 149 and
  152 are cheap deterministic `await_startup_notification` mappings using the existing test pattern;
  the blanket no-change decision was too conservative for them. Lines 174/180/181 (`stop`) and 95
  (bind failure) remain deferred to #1488 and the `BoundSocket` boundary. Line 215 is the existing
  test's defensive `panic!` arm.
- 2026-09-11 - User/maintainer - Requested the plan be reopened to add those tests and a module
  comment documenting the testing strategy for future maintainers.
- 2026-09-11 - User/maintainer - Approved R2. Add only the two deterministic closed-startup
  notification mappings; do not introduce socket, registrar, container, or `Launcher` setup.
- 2026-09-11 - User/maintainer - Reviewed and approved the R3 test design. Retain
  `launcher_task_with_successful_result` because it hides duplicated incidental `Spawner`
  construction while the successful versus aborted task outcome remains visible in each test.
- 2026-09-11 - User/maintainer - Approved R4. Measure aggregate/global, unit-only, and
  integration-only coverage separately and record each remaining executable line with its owner;
  do not add a coverage-only socket or lifecycle test.
- 2026-09-11 - User/maintainer - Reviewed and approved the completed server-states plan. Direct
  tests now cover each deterministic startup-notification mapping, while the module documents the
  public transition and #1488 lifecycle ownership of all remaining paths.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | DONE | Markdown and spelling checks passed after all maintainer review changes. |
| R1 | DONE | `cargo test -p torrust-tracker-udp-server states::tests` and `cargo test -p torrust-tracker-udp-server server::tests::it_should_preserve_registration_error_and_release_listener_when_registration_fails` retain the focused existing unit and public-transition contracts. The no-change conclusion was subsequently narrowed by R2. |
| R2/R3 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server states::tests`, and `git diff --check` passed. Prose-first and smell review retain visible closed-sender/task-outcome causal state, direct startup-notification Act, and one error-variant assertion per test. |
| R4 | DONE | Separate clean reports: aggregate/global and unit-only are 72/77 lines (93.51%), 91/102 regions (89.22%), and 16/20 functions (80.00%); integration-only is 27/37 lines (72.97%), 16/32 regions (50.00%), and 7/11 functions (63.64%). Remaining unit-only lines are the `BoundSocket`/public-start bind conversion, #1488-owned `stop` mappings, and defensive test fallback. |
| Plan completion | DONE | Maintainer reviewed all approved increments and evidence before the next file plan begins. |

## Non-Goals

- Do not change state transitions, socket binding, registration, task spawning, error mapping, or
  shutdown behavior.
- Do not test derived representations, field assignment, aliases, or macro-generated display.
- Do not duplicate registration-error cleanup, real-loopback transport, or standalone environment
  coverage.
- Do not test `Server::<Running>::stop`, halt cancellation, receive-loop completion, active-request
  draining, or shutdown policy before #1488's SI-14, SI-15, and SI-17 work is complete. Aborting a
  test-owned task to exercise `await_startup_notification`'s join-failure mapping is in scope; it
  does not touch the production shutdown path.

## Validation Per Approved Increment

- Run `cargo test -p torrust-tracker-udp-server states::tests`.
- Run `cargo test -p torrust-tracker-udp-server server::tests::it_should_preserve_registration_error_and_release_listener_when_registration_fails`.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Measure aggregate/global, unit-only, and integration-only coverage separately only when a new
  measurement materially informs an ownership decision.

## Completion Criteria

- Every `await_startup_notification` branch has one focused deterministic unit test.
- The module documents which behavior is unit tested locally, which is protected at the public
  `server/mod.rs` boundary, and which lifecycle paths are deferred to #1488.
- Each remaining state-layer behavior has a documented representation, public-transition, or #1488
  lifecycle owner.
- No fixture, mock, abstraction, or percentage-only test is introduced without a distinct
  package-owned behavioral reason.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
