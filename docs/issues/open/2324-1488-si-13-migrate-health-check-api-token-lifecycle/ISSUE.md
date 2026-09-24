---
schema-version: 1
doc-type: issue
issue-type: task
status: open
priority: p1
epic: 1488
github-issue: 2324
spec-path: docs/issues/open/2324-1488-si-13-migrate-health-check-api-token-lifecycle/ISSUE.md
branch: "2324-1488-si-13-migrate-health-check-api-token-lifecycle"
related-pr: null
last-updated-utc: "2026-09-24 00:00"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/app.rs
    - src/bootstrap/jobs/health_check_api.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/axum-health-check-api-server/src/handlers.rs
    - packages/axum-server/src/signals.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/open/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md
    - docs/issues/open/2274-1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2324 - Migrate Health-Check API to Token Lifecycle

Parent EPIC: #1488 - Overhaul: Tracker Shutdown

> **EPIC position**: Roadmap step 9. One independently releasable health-check
> API vertical slice after the additive server lifecycle API and Axum drain helper.

## Goal

Migrate only the health-check API component to the supervised cancellation tree.
The bootstrap derives a health-check API child `CancellationToken` from
`JobManager`; the health-check API receives it, begins Axum connection draining
through the token-aware helper, joins its server and drain-controller children,
and reports one named `health_check_api` outcome to `JobManager`.

This migration does not change HTTP tracker, REST API, UDP server, or standalone
consumers. Their legacy lifecycle paths remain supported.

## Current State

`src/bootstrap/jobs/health_check_api.rs` creates startup and shutdown oneshot
channels, spawns the server, then returns a wrapper `JoinHandle<()>` to
`JobManager`. The wrapper already receives the manager token, forwards
cancellation to its private `Halted::Normal` sender, and awaits the server task.
The server returns its drain-controller handle to the wrapper, which joins it
only on token cancellation. Independent server completion returns without
joining the controller. The legacy helper observes a `Halted` channel or
library-level OS signals.

Consequently, `JobManager` reaches the health-check API through a transitional
bridge but cannot verify drain-controller completion before `health_check_api`
reports completion.

## Scope

### In scope

- Add a health-check API start path accepting an injected component
  `CancellationToken`.
- Derive one health-check API child token from the `JobManager` root token in
  `src/app.rs`.
- Use the token-aware, joinable Axum drain helper.
- Retain and join the health-check API server and drain-controller tasks within
  the component's owned task tree.
- Report one named `health_check_api` outcome to `JobManager`.
- Add deterministic tests for injected-token cancellation, normal drain,
  unexpected server-task completion/failure, and bootstrap propagation.
- Add focused manual SIGTERM evidence after SI-1 verifies the tracker signal
  boundary and the health-check API token-driven drain path.

### Out of scope

- Returning unhealthy or HTTP 503 responses before or during shutdown. SI-21
  owns the Q6-approved readiness behavior as a separate vertical slice.
- HTTP tracker, REST API, UDP server, and standalone consumer migrations.
- Removal or deprecation of legacy `Halted`-based health-check start/stop APIs.
- Removal of `global_shutdown_signal()`, deadline configuration, and exit codes.

## Implementation Constraints

1. Existing health-check API start/stop callers remain source- and
   behavior-compatible until the separate deprecation/removal phase.
2. The token-aware path has no `SIGINT` or `SIGTERM` subscription inside the
   health-check server package.
3. The component joins its server and drain-controller children before it
   returns the top-level `health_check_api` outcome.
4. `JobManager` receives only the health-check component handle and outcome;
   it does not receive server or drain-controller handles.
5. Existing health-check request and probe behavior remains unchanged. SI-21,
   after this migration, alters shutdown readiness, response status, and probe
   fan-out behavior.
6. A cancellation race or unexpected server completion returns an explicit
   outcome; it must not panic or silently drop a drain-controller task.

## Implementation Decisions

Maintainer decisions recorded before implementation (2026-09-24):

1. **Package-owned token-aware start**: add
   `server::start_with_cancellation` in `axum-health-check-api-server`. Like
   the REST API, it binds, serves, registers the service, emits the existing
   startup logs (same target and messages, including `STARTED_ON`), and rolls
   back (cancel and join children, release the listener) when registration
   fails. The bootstrap job only supervises the returned children.
2. **Drain timeout**: a package constant of 5 seconds for the token-aware
   drain. Health-check requests are short-lived probe fan-outs, and the value
   stays inside the current 10-second `JobManager` deadline in `main()`. SI-20
   owns configurable budgets.
3. **Component supervisor**: a local supervisor in
   `src/bootstrap/jobs/health_check_api.rs`, consistent with SI-11 and SI-12.
   Consolidating the three copies is deferred to the legacy-removal phase.
4. **Legacy owner type**: keep `NestedServerTask::with_shutdown_controller`
   until SI-19 so the rollback story remains a call-site revert.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Map health-check lifecycle ownership | Confirm `JobManager` owns `health_check_api`; the component owns its runtime and drain-controller children, including startup rollback. |
| T2 | TODO | Add owned token-aware health-check lifecycle | Add an additive token-aware server start path while preserving legacy start/stop APIs and startup logs. |
| T3 | TODO | Supervise health-check component children | Join runtime and drain-controller children for cancellation, independent completion, and failure before reporting one named outcome. |
| T4 | TODO | Add deterministic lifecycle coverage | Cover token drain, registration rollback/listener release, legacy compatibility, independent completion, failure, and bootstrap propagation. |
| T5 | TODO | Review first passing vertical slice | Review ownership, drop paths, named outcomes, readiness invariants, and absolute deadlines after focused tests pass. |
| T6 | TODO | Complete executable-boundary verification | Capture direct tracker-binary SIGTERM, health-check token-driven drain, and immediate listener rebind evidence. |
| T7 | TODO | Complete acceptance and implementation review | Re-review acceptance criteria against observed behavior and record the completion review. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Ownership-map or test-plan correction, only if it materially clarifies the migration | Record a no-change decision in issue evidence when no standalone documentation commit is warranted. |
| T2-T3 | Additive health-check server lifecycle and owned component supervision | Commit after focused compilation and lifecycle validation. |
| T4 | One reviewed deterministic lifecycle-test increment | Use the `write-unit-test` workflow. Commit after focused validation and test-design review before another test area. |
| T5 | Material ownership or documentation correction | Commit only substantive corrections separately. |
| T6-T7 | Final evidence and completion review | Commit with final implementation evidence after direct-process verification succeeds. |

## Test Development Loop

For each test-producing increment, use the `write-unit-test` skill and make one
behavior-focused change. Write temporary prose for Arrange, Act, and Assert;
refactor until the test body expresses that prose; remove redundant prose; and
record the result in issue evidence. The test must expose its one causal
initial-state difference, keep the production Act visible, and independently
specify the expected result. Run focused validation and complete the test-design
review before beginning another test area or committing.

## Acceptance Criteria

- [ ] The health-check API receives a component child `CancellationToken`
      derived from the `JobManager` root token.
- [ ] Token cancellation starts health-check API graceful draining through the
      new Axum helper without a library-level OS-signal subscription.
- [ ] The component awaits its server and drain-controller children before
      reporting the named `health_check_api` outcome to `JobManager`.
- [ ] Legacy health-check API start/stop callers compile and preserve behavior.
- [ ] Deterministic tests cover injected-token cancellation, normal drain, and
      unexpected server-task completion/failure without OS signals.
- [ ] A focused bootstrap integration test proves root-token cancellation
      reaches the health-check API without delivering an OS signal.
- [ ] Existing health-check response and readiness semantics are unchanged in
      this migration; SI-21 applies the separately approved shutdown behavior.
- [ ] Manual SIGTERM verification records the `main()` signal event followed
      by the health-check API's token-driven drain completion.
- [ ] `linter all` passes.

## Dependencies

- The additive token-aware server lifecycle API from SI-2 is available and
  released.
- The token-aware, joinable Axum drain helper is available.
- SI-1 is required only for manual SIGTERM verification.
- SI-21 follows this migration to apply Q6's readiness-before-drain behavior.

## Rollback

Restore only the health-check API bootstrap and server call sites to the legacy
lifecycle path. The additive lifecycle API and helper remain available but
unused; HTTP, REST, UDP, and standalone consumers are unaffected.

## Architectural Decisions

No new ADR is planned. This task applies the existing supervised
cancellation-tree ADR to the health-check component. Create and link an ADR if
the work changes established component ownership, deadline, readiness, or
compatibility policy.

## Progress Tracking

### Workflow Checkpoints

- [x] #1488 roadmap, #2234, #2274, and completed SI-11/SI-12 reviewed.
- [x] Current health-check bootstrap and server ownership path mapped.
- [x] Draft expanded with implementation, verification, and completion-review controls.
- [x] Draft reviewed and approved by user/maintainer.
- [x] GitHub issue #2324 created and issue number added to this specification.
- [x] Spec-only PR #2326 merged into `develop` before implementation.
- [ ] Implementation completed.
- [ ] Automatic verification completed with toolchain-qualified evidence.
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria reviewed after implementation and updated with evidence.
- [ ] Evidence-based implementation completion review recorded.
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`.

### Progress Log

- 2026-09-23 UTC - GitHub Copilot - Expanded the SI-13 draft after SI-12
   merged and was archived. The current health-check component bridges the
   manager token to `Halted`; it retains the drain-controller handle but does
   not join it when the server completes independently. The planned migration
   gives the component explicit ownership of both children and preserves SI-21's
   separate readiness-before-drain behavior.
- 2026-09-23 UTC - GitHub Copilot - User/maintainer approved the draft.
  Created GitHub issue #2324 and promoted this specification to its numbered
  open-issue folder. The next workflow step is a spec-only pull request before
  implementation.
- 2026-09-24 UTC - GitHub Copilot - Spec-only PR #2326 merged into `develop`.
  Created the implementation branch from the merge result. Maintainer answered
  pre-implementation questions; decisions are recorded in
  [Implementation Decisions](#implementation-decisions).

## Verification Plan

Define verification before implementation starts and execute it before closing
this issue. Every recorded command result must identify the Rust toolchain or
runtime when it affects behavior.

### Automatic Checks

- Focused `axum-health-check-api-server`, `axum-server`, health-check component,
   and bootstrap lifecycle tests.
- Legacy health-check API start/stop compatibility coverage.
- `linter all`.
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`.
- Pre-push checks when applicable.

### Manual Verification Scenarios

Record commands, output, and relevant logs in issue-local
`manual-verification-evidence.md`. Follow the EPIC executable-boundary protocol:
signal the direct tracker-binary PID, capture bounded completion and exit status,
and prove affected bindings are released.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Token-driven health-check API shutdown | Start a configured `target/debug/torrust-tracker` with the health-check API enabled, establish readiness, send `SIGTERM` to the direct binary PID, and capture bounded exit and logs. | `main()` cancels the component token; the health-check API records one token-driven drain path and exits cleanly. | TODO | `manual-verification-evidence.md` |
| M2 | Health-check API listener release | Restart the configured tracker on the same health-check API binding after M1. | The listener rebinds immediately and becomes ready. | TODO | `manual-verification-evidence.md` |
| M3 | Legacy health-check lifecycle compatibility | Exercise an unchanged legacy health-check API start/stop call path. | The legacy consumer compiles and retains its supported stop behavior. | TODO | Automated test evidence; the migrated binary has no legacy-path switch. |

Manual verification is real interaction with the built tracker. Running
automated tests alone does not satisfy M1 or M2.

### Disposable Verification Scripts

No disposable verification script is planned. If a repeatable direct-PID harness
becomes necessary, place it in this issue directory and record why a maintained
Rust test cannot cover the scenario, its owner, and its removal or retention
decision.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Focused bootstrap cancellation test. |
| AC2 | TODO | Token-drain test and source review confirming no token-aware-path signal subscription. |
| AC3 | TODO | Component tests that hold the drain controller after cancellation observation for cancellation, completion, and failure. |
| AC4 | TODO | Legacy server start/stop compatibility test. |
| AC5 | TODO | Focused lifecycle tests and prose-first test-design review. |
| AC6 | TODO | Focused bootstrap cancellation test. |
| AC7 | TODO | Existing response and readiness behavior test coverage plus source review. |
| AC8 | TODO | Direct-PID SIGTERM, token-aware drain, and listener-rebind evidence. |
| AC9 | TODO | `linter all` output. |

## Implementation Completion Review

Before closing the issue, an independent reviewer must verify the acceptance
criteria, child-task ownership, startup rollback, deterministic test design,
manual evidence, and required validation. Create an issue-local
`implementation-retrospective.md` for reusable lessons, material design changes,
or meaningful deviations; otherwise record why no separate retrospective is
needed in the progress log.
