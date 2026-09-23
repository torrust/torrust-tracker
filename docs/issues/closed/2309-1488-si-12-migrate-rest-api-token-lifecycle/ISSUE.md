---
doc-type: issue
issue-type: task
status: done
priority: p1
epic: 1488
github-issue: 2309
spec-path: docs/issues/closed/2309-1488-si-12-migrate-rest-api-token-lifecycle/ISSUE.md
branch: "2309-1488-si-12-migrate-rest-api-token-lifecycle"
related-pr: 2316
last-updated-utc: 2026-09-23
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - src/app.rs
    - src/bootstrap/jobs/tracker_apis.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-server/src/signals.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - docs/issues/open/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md
    - docs/issues/open/2274-1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
---

<!-- skill-link: create-issue -->

# Issue #2309 - Migrate REST API to Token Lifecycle

Parent EPIC: #1488 - Overhaul: Tracker Shutdown

> **EPIC position**: Roadmap step 8. One independently releasable REST API
> vertical slice after the additive server lifecycle API and Axum drain helper.

## Goal

Migrate only the tracker management REST API component to the supervised
cancellation tree. The bootstrap derives a REST API component child
`CancellationToken` from `JobManager`; the REST API receives it, starts
connection draining through the token-aware Axum helper, joins its server and
drain-controller children, and reports one named `http_api` outcome to
`JobManager`.

This migration does not change the HTTP tracker, health-check API, UDP server,
or standalone consumers. Their legacy lifecycle paths remain supported.

## Current State

The REST API now has an additive `ApiServer::start_with_cancellation` path.
It retains a server task and a token-aware drain-controller task in
`CancellationRunning`. `tracker_apis` supervises and joins those children, and
`start_the_http_api` passes a child token from `JobManager`. The legacy
`ApiServer::start` / `ApiServer::stop` path remains unchanged.

Deterministic tests cover cancellation, startup rollback, independent
completion, runtime failure, legacy compatibility, and bootstrap propagation.
Direct-process SIGTERM and listener-rebind evidence is recorded in
`manual-verification-evidence.md`.

## Scope

### In scope

- Add a REST API start path that accepts an injected component
  `CancellationToken`.
- Derive a REST API child token from the `JobManager` root token in `src/app.rs`.
- Use the token-aware, joinable Axum drain helper.
- Retain and join the REST API server task and drain-controller task within the
  REST API component's owned task tree.
- Report one named `http_api` outcome to `JobManager`.
- Add deterministic tests for injected-token cancellation and unexpected
  server-task completion/failure, without OS signals.
- Add focused manual SIGTERM evidence after SI-1 verifies the tracker signal
  boundary and the REST API token-driven drain path.

### Out of scope

- HTTP tracker, health-check API, UDP server, and standalone consumer changes.
- Readiness behavior during shutdown; SI-21 owns Q6's approved behavior after
  the health-check API lifecycle migration.
- Removal or deprecation of legacy `Halted`-based REST API start/stop APIs.
- Removal of `global_shutdown_signal()`, deadline configuration, and exit codes.

## Implementation Constraints

1. Existing `ApiServer::start` / `ApiServer::stop` callers remain source- and
   behavior-compatible until migration and deprecation are complete.
2. The new REST API path does not subscribe to `SIGINT` or `SIGTERM` in the
   server package.
3. The REST API component joins its server and drain-controller children before
   returning its top-level outcome.
4. `JobManager` receives only the `http_api` top-level handle and outcome, not
   internal REST server task handles.
5. A cancellation race or unexpected server completion yields an explicit
   outcome; it must not panic or silently discard the drain controller.

## Bug-Fix Process

Not applicable. This is an additive lifecycle migration, not a repair of a
reported defect.

## Regression Test Strategy

Not applicable as a bug-fix strategy. Deterministic REST component and bootstrap
integration tests must protect token propagation, owned-child completion,
ordinary drain, and unexpected server completion. Existing legacy start/stop
coverage protects callers that remain on the compatibility API.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Map current REST lifecycle ownership | `JobManager` owns the named `http_api` component; that component owns and joins its REST runtime and drain controller. Registration failure cancels and joins unpublished children. |
| T2 | DONE | Add owned token-aware REST lifecycle | Added an additive `ApiServer::start_with_cancellation` path that preserves legacy APIs and startup logs. |
| T3 | DONE | Supervise REST component children | The token-aware runner joins both children for cancellation, independent completion, and failure before reporting its top-level outcome. |
| T4 | DONE | Add deterministic lifecycle coverage | Added token drain, registration rollback/listener release, legacy compatibility, independent completion, failure, and bootstrap propagation tests. Controller-completion tests hold the controller after it observes cancellation. |
| T5 | DONE | Review first passing vertical slice | Ownership review completed: listener setup is local and synchronous; the only deferred drain has the existing 90-second absolute timeout. `TokenAwareServerTask` aborts owned children on drop/escalation. |
| T6 | DONE | Complete executable-boundary verification | Two direct tracker-binary SIGTERM runs exited `0`, logged the REST token-aware drain, and immediately rebound the REST listener. See `manual-verification-evidence.md`. |
| T7 | DONE | Complete acceptance and implementation review | Independent review found and the implementation corrected runtime-error outcome propagation. See `implementation-retrospective.md`. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Ownership map or test-plan correction, only if it materially clarifies the migration | Record a no-change decision in issue evidence when no standalone documentation commit is warranted. |
| T2-T3 | Additive REST server lifecycle and owned component supervision | Commit after focused compilation and lifecycle validation. |
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

- [x] The REST API receives a component child `CancellationToken` derived from
      the `JobManager` root token.
- [x] Token cancellation starts REST API graceful draining through the new Axum
      helper without a library-level OS-signal subscription.
- [x] The REST API component joins its server and drain-controller children
      before reporting its named `http_api` outcome to `JobManager`.
- [x] Legacy REST API start/stop callers compile and preserve their behavior.
- [x] Deterministic REST API tests cover injected-token cancellation, normal
      drain completion, and unexpected server-task completion/failure.
- [x] A focused bootstrap integration test proves root-token cancellation
      reaches the REST API without delivering an OS signal.
- [x] Manual SIGTERM verification records the `main()` signal-boundary event
      followed by the REST API component's token-driven drain completion.
- [x] The first passing vertical slice completes a design review of ownership,
   drop paths, named outcomes, and absolute deadlines.
- [x] Acceptance criteria are re-reviewed after implementation and reflect
   observed behavior.
- [x] `linter all` passes.

## Dependencies

- The additive token-aware server lifecycle API from SI-2 is available and
  released.
- The token-aware, joinable Axum drain helper is available.
- SI-1 is required only for manual SIGTERM verification.

## Rollback

Restore only REST API bootstrap and server call sites to the legacy lifecycle
path. The additive lifecycle API and helper remain available but unused; HTTP,
health-check, UDP, and standalone consumers are unaffected.

## Architectural Decisions

No new ADR is planned. This task applies the existing supervised
cancellation-tree ADR to the REST API component. Create and link an ADR if the
work changes the established component ownership, deadline, or compatibility
policy.

## Progress Tracking

### Workflow Checkpoints

- [x] #1488 roadmap, #2234, #2274, and completed SI-11 reviewed.
- [x] Current REST bootstrap and server ownership path mapped.
- [x] Draft expanded with implementation, verification, and completion-review controls.
- [x] Draft reviewed and approved by user/maintainer.
- [x] GitHub issue #2309 created and issue number added to this specification.
- [x] Spec-only PR #2311 merged into `develop` before implementation.
- [x] Implementation completed.
- [x] Automatic verification completed with toolchain-qualified evidence.
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`.
- [x] Acceptance criteria reviewed after implementation and updated with evidence.
- [x] Evidence-based implementation completion review recorded.
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`.

### Progress Log

- 2026-09-23 UTC - GitHub Copilot - Prepared the SI-12 draft after SI-11 merged.
  The current REST path derives a `JobManager` token but bridges cancellation to
  legacy `Halted`; `ApiServer::Launcher::start` detaches its drain controller.
  The planned migration retains the REST component as the owner of its server
  and token-aware drain-controller children while `JobManager` retains only the
  named `http_api` component.
- 2026-09-23 UTC - GitHub Copilot - User/maintainer approved the draft. Created
  GitHub issue #2309 and promoted this specification to its numbered open-issue
  folder. The next workflow step is a spec-only pull request before implementation.
- 2026-09-23 09:10 UTC - GitHub Copilot - Spec-only PR #2311 merged into
  `develop`. Created the reserved implementation branch from the merge result;
  implementation begins with the REST server lifecycle ownership path.
- 2026-09-23 UTC - GitHub Copilot - Completed the deterministic REST vertical
  slice on `2309-1488-si-12-migrate-rest-api-token-lifecycle`. The additive
  runtime retains its drain controller, startup rollback cancels and joins
  unpublished children, and component supervision joins children before a
  named outcome. Prose-first review found the server tests own runtime
  behavior, supervisor tests own child-join behavior, and the application test
  owns root-token propagation. Manual direct-process evidence and final gates
  were recorded subsequently.
- 2026-09-23 09:33 UTC - GitHub Copilot - Direct tracker-binary verification
  started two isolated processes on the same REST binding. Each exact binary
  PID received SIGTERM, exited `0` within the 20-second bound, and logged both
  the main signal boundary and REST token-aware drain. The second run proved
  immediate listener rebind. See `manual-verification-evidence.md`.
- 2026-09-23 UTC - GitHub Copilot - PR #2316 merged and GitHub issue #2309
  closed as completed. Archived this completed specification and its evidence
  under `docs/issues/closed/`.

## Verification Plan

Define verification before implementation starts and execute it before closing
this issue. Every recorded command result must identify the Rust toolchain or
runtime when it affects behavior.

### Automatic Checks

- Focused `axum-rest-api-server`, `axum-server`, REST component, and bootstrap
  lifecycle tests.
- Legacy REST API start/stop compatibility coverage.
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
| M1 | Token-driven REST API shutdown | Start a configured `target/debug/torrust-tracker` with the REST API enabled, establish readiness, send `SIGTERM` to the direct binary PID, and capture bounded exit and logs. | `main()` cancels the component token; the REST API records one token-driven drain path and exits cleanly. | DONE | `manual-verification-evidence.md` V1 |
| M2 | REST API listener release | Restart the configured tracker on the same REST API binding after M1. | The listener rebinds immediately and becomes ready. | DONE | `manual-verification-evidence.md` V2 |
| M3 | Legacy REST lifecycle compatibility | Exercise an unchanged legacy REST API start/stop call path. | The legacy consumer compiles and retains its supported stop behavior. | DONE | Automated test evidence; the migrated binary has no legacy-path switch. |

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
| AC1 | DONE | `app::tests::it_should_cancel_the_rest_api_component_through_the_job_manager`. |
| AC2 | DONE | `server::tests::it_should_drain_the_token_aware_rest_api_when_its_cancellation_token_is_cancelled`; source review confirms the new path has no signal subscription. |
| AC3 | DONE | `tracker_apis` completion and panic tests hold the controller after cancellation observation; the token-cancellation test covers the normal outcome. |
| AC4 | DONE | `server::tests::it_should_be_able_to_start_and_stop` exercises the unchanged compatibility path. |
| AC5 | DONE | Focused lifecycle tests and the 2026-09-23 prose-first review entry above. |
| AC6 | DONE | `app::tests::it_should_cancel_the_rest_api_component_through_the_job_manager`. |
| AC7 | DONE | `manual-verification-evidence.md` V1-V2 records direct-PID SIGTERM, clean exit, token-aware drain, and listener rebind. |
| AC8 | DONE | T5 ownership review and `implementation-retrospective.md`. |
| AC9 | DONE | `linter all` passed on nightly Rust `1.100.0-nightly`. |

## Implementation Completion Review

Before closing the issue, an independent reviewer must verify the acceptance
criteria, child-task ownership, startup rollback, deterministic test design,
manual evidence, and required validation. Create an issue-local
`implementation-retrospective.md` for reusable lessons, material design changes,
or meaningful deviations; otherwise record why no separate retrospective is
needed in the progress log.
