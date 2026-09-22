---
schema-version: 1
doc-type: issue
issue-type: task
status: open
priority: p1
epic: 1488
github-issue: 2289
spec-path: docs/issues/open/2289-1488-si-11-migrate-http-tracker-token-lifecycle/ISSUE.md
branch: "2289-1488-si-11-migrate-http-tracker-token-lifecycle-spec"
related-pr: null
last-updated-utc: "2026-09-22 07:55"
semantic-links:
 skill-links: [create-issue, write-unit-test]
 related-artifacts: [.github/skills/dev/planning/create-issue/SKILL.md, .github/skills/dev/testing/write-unit-test/SKILL.md, src/app.rs, src/bootstrap/jobs/http_tracker.rs, packages/axum-http-server/src/server.rs, packages/axum-server/src/signals.rs, docs/features/shutdown-process/README.md, docs/features/shutdown-process/task-inventory.md, docs/features/shutdown-process/shutdown-architecture-examples.md, docs/issues/open/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md, docs/issues/open/2274-1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md, docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md]
---

<!-- skill-link: create-issue -->

# Issue #2289 - Migrate HTTP Tracker to Token Lifecycle

Parent EPIC: #1488 - Overhaul: Tracker Shutdown

> **EPIC position**: Roadmap step 7. One independently releasable HTTP tracker vertical slice after the additive server lifecycle API and Axum drain helper.

## Goal

Migrate only the HTTP tracker component to the supervised cancellation tree.
The tracker bootstrap derives a component child `CancellationToken` from
`JobManager`; the HTTP tracker receives it, starts graceful Axum draining on
cancellation through the new helper, joins its server and drain-controller
children, and reports one named component outcome to `JobManager`.

This migration does not change REST API, health-check API, UDP, or standalone
HTTP environment consumers. Their legacy lifecycle paths remain supported.

## Background

`src/bootstrap/jobs/http_tracker.rs` starts `HttpServer` then returns a wrapper
`JoinHandle<()>` to `JobManager`. The wrapper already receives the manager
token, forwards cancellation to its private `Halted::Normal` sender, and awaits
the server task. In `packages/axum-http-server`, the server spawns
`graceful_shutdown(...)` and discards that drain controller's handle. The
legacy helper observes a `Halted` channel or library-level OS signals.

Consequently, the existing bridge requests HTTP tracker shutdown but does not
give the component a direct token-aware lifecycle API or prove that the HTTP
drain controller completed before the wrapper task ends.

## Scope

### In scope

- Add an HTTP tracker start path that accepts an injected component
  `CancellationToken`.
- Derive one child token per configured HTTP tracker instance in `src/app.rs`.
- Use the token-aware Axum drain helper introduced by the preceding shared-helper
  task.
- Retain and join the HTTP server task and its drain-controller task inside the
  HTTP component's owned task tree.
- Report one named `http_instance_<index>_<address>` outcome to `JobManager`.
- Add deterministic tests that cancel an injected token and await the HTTP
  component completion without an OS signal.
- Add focused manual verification using the tracker binary after SI-1 is
  available, confirming SIGTERM reaches `main()` and the migrated HTTP
  component drains through its token path.

### Out of scope

- Changes to REST API, health-check API, UDP server, or their consumers.
- Removal or deprecation of legacy `Halted`-based HTTP start/stop APIs.
- Removal of `global_shutdown_signal()` or other shared legacy APIs.
- Final component/process deadline values, configuration, and exit codes.

## Implementation Constraints

1. The existing `HttpServer::start` / `HttpServer::stop` lifecycle remains
   source- and behavior-compatible for consumers that have not migrated.
2. The token-aware path must not subscribe to `SIGINT` or `SIGTERM` inside the
   HTTP server package.
3. The HTTP component owns its direct children. It must await both the server
   future and drain-controller future before it returns its outcome.
4. `JobManager` receives only the HTTP component's top-level handle and outcome;
   it does not receive nested HTTP handles.
5. If cancellation races with unexpected server completion, the component must
   return an explicit completed or failed outcome rather than panic or silently
   dropping the drain controller.

## Bug-Fix Process

Not applicable. This is a migration to an additive lifecycle API, not a repair
of a reported defect.

## Regression Test Strategy

Not applicable as a bug-fix strategy. Deterministic HTTP-component and bootstrap
integration tests protect token propagation, owned-child completion, ordinary
drain, and unexpected server completion. Existing legacy-consumer coverage
protects compatibility for consumers not yet migrated.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Map existing HTTP lifecycle ownership | Identify the bootstrap wrapper, `HttpServer` tasks, legacy `Halted` bridge, and drain-controller ownership. Confirm the narrow additive token-aware start surface. |
| T2 | TODO | Add owned token-aware HTTP lifecycle | Derive one component child token per configured HTTP instance, start the token-aware drain helper, and await server plus drain-controller tasks before the component reports one named outcome. |
| T3 | TODO | Add deterministic lifecycle coverage | Cover injected-token cancellation, normal drain, unexpected server completion or failure, bootstrap propagation without OS signals, and legacy API compatibility. |
| T4 | TODO | Review the first passing vertical slice | Recheck ownership boundaries, drop paths, named outcomes, and absolute deadlines before changing another lifecycle area. |
| T5 | TODO | Complete executable-boundary verification | Record direct tracker-binary PID `SIGTERM`, bounded exit, token-driven HTTP drain log ordering, legacy compatibility, and listener rebind evidence. |
| T6 | TODO | Complete acceptance and implementation review | Run final checks, re-review acceptance criteria, and record a retrospective or a reasoned no-retrospective decision. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Ownership map or test-plan documentation, only if it materially clarifies the migration | Record a no-change decision in evidence when a standalone documentation commit is not warranted. |
| T2 | Token-aware HTTP component lifecycle and owned-child joining | Commit after focused compilation and lifecycle validation. |
| T3 | One reviewed deterministic lifecycle-test increment | Use the `write-unit-test` workflow, record prose-first Arrange-Act-Assert review, and commit after focused validation before another test area. |
| T4 | Design-review findings | Commit only material design or documentation corrections separately. |
| T5 | Manual verification evidence | Commit with final implementation evidence after the direct-process scenario succeeds. |
| T6 | Acceptance and completion-review evidence | Commit after maintainer review and final validation. |

## Test Development Loop

For each test-producing increment, use the `write-unit-test` skill and make one
behavior-focused change. Write temporary prose for Arrange, Act, and Assert;
refactor until the test body expresses that prose; remove redundant prose; and
record the result in issue evidence. The test must expose the one causal
initial-state difference, keep the production Act visible, and independently
specify the expected result. Run focused validation and complete the test-design
review before beginning another test area or committing.

## Progress Tracking

### Workflow Checkpoints

- [x] Existing HTTP tracker lifecycle, #2234, #2274, and the #1488 roadmap reviewed.
- [x] Folder-style draft updated for the current issue-specification workflow.
- [x] Draft specification reviewed and approved by user/maintainer.
- [x] GitHub issue #2289 created and issue number added to this specification.
- [ ] Spec-only PR merged into `develop` before implementation.
- [ ] Implementation completed.
- [ ] Automatic verification completed with toolchain-qualified evidence.
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria reviewed after implementation and updated with evidence.
- [ ] Evidence-based implementation completion review recorded.
- [ ] Reviewer validated acceptance criteria and updated checkboxes.
- [x] Committer verified spec progress is up to date before commit.
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`.

### Progress Log

- 2026-09-22 07:45 UTC - GitHub Copilot - Updated the existing SI-11 draft with
   the required implementation plan, commit points, test-development loop,
   verification matrix, and completion-review checkpoints after #2234 and #2274
   became available. User/maintainer approved promotion to an issue and spec-only PR.
- 2026-09-22 07:30 UTC - GitHub Copilot - Created GitHub issue #2289 and moved
   this specification to its numbered open-issue directory. The next workflow
   step is a spec-only pull request before implementation.

## Acceptance Criteria

- [ ] One configured HTTP tracker instance receives one component child
      `CancellationToken` derived from the `JobManager` root token.
- [ ] Token cancellation starts HTTP graceful draining through the new Axum
      helper without a library-level OS-signal subscription.
- [ ] The HTTP component awaits its server and drain-controller tasks before
      reporting its named outcome to `JobManager`.
- [ ] Legacy HTTP start/stop API consumers still compile and preserve behavior.
- [ ] HTTP component tests deterministically cancel an injected token and cover
      normal drain completion and unexpected server-task completion/failure.
- [ ] A focused integration test proves a cancellation request reaches the HTTP
      tracker through bootstrap wiring without delivering an OS signal.
- [ ] Manual SIGTERM verification confirms the migrated HTTP component logs one
      token-driven shutdown path; legacy server signal logs are not required to
      disappear until all consumers migrate and the legacy API is removed.
- [ ] The first passing vertical slice completes a design review of ownership,
   drop paths, named outcomes, and absolute deadlines.
- [ ] `linter all` passes.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect
   actual behavior.

## Dependencies

- Additive token-aware server lifecycle API (SI-2) is available and released.
- Token-aware, joinable Axum drain helper is available.
- SI-1 is required only for the manual SIGTERM check; deterministic tests do
  not require it.

## Rollback

The migration is reversible without an API rollback: restore the HTTP tracker
bootstrap and server call sites to the unchanged legacy lifecycle path. The
additive token-aware APIs remain available but unused; REST, health-check, UDP,
and standalone HTTP consumers are unaffected.

## Verification Plan

Define verification before implementation starts and execute it before closing
this issue. Every recorded command result identifies the Rust toolchain or
runtime when it affects behavior.

### Automatic Checks

- Focused `axum-http-server`, `axum-server`, and bootstrap lifecycle tests
- Legacy HTTP start/stop compatibility coverage
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
- Pre-push checks when applicable

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Token-driven HTTP tracker shutdown | Start a configured `target/debug/torrust-tracker` with one HTTP binding, establish readiness, send `SIGTERM` to the direct binary PID, and capture bounded exit and logs. | `main()` cancels the component token; the HTTP component records one token-driven drain path and exits cleanly. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | HTTP listener release | Restart the configured tracker on the same HTTP binding after M1. | The listener rebinds immediately and becomes ready. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Legacy HTTP lifecycle compatibility | Exercise an unchanged legacy HTTP start/stop call path. | The legacy consumer compiles and retains its supported stop behavior. | TODO | `manual-verification-evidence.md` section V3 |

Manual verification is real interaction with the built tracker. Running
automated tests alone does not satisfy these scenarios.

### Disposable Verification Scripts

No disposable verification script is planned. If a repeatable direct-PID harness
becomes necessary, place it in this issue directory and record why a maintained
Rust test cannot cover the scenario, its owner, and its removal or retention
decision.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Bootstrap and component lifecycle tests |
| AC2 | TODO | Focused token-cancellation test and code review |
| AC3 | TODO | Component task-ownership test and shutdown logs |
| AC4 | TODO | Legacy HTTP consumer compilation and test output |
| AC5 | TODO | Deterministic lifecycle test output and test-design review |
| AC6 | TODO | Focused bootstrap integration test output |
| AC7 | TODO | `manual-verification-evidence.md` |
| AC8 | TODO | Design-review progress-log entry |
| AC9 | TODO | `linter all` output |
| AC10 | TODO | Post-implementation acceptance review |

## Risks and Trade-offs

- The first migration may accidentally leave the legacy bridge and token path
   both active. Mitigate by mapping ownership before code changes and asserting
   one token-driven path in focused tests and manual logs.
- Coordinating server and drain futures can create completion races. Mitigate
   with explicit component-owned handles, deterministic unexpected-completion
   coverage, and absolute deadlines for readiness and drain waits.
- A broad refactor could disturb REST, health-check, or standalone consumers.
   Mitigate by keeping the new HTTP path additive and preserving legacy APIs and
   call sites.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from
   `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` when the migration yields a
   material discovery; otherwise add a concise progress-log entry explaining why
   one was unnecessary.
- When an independent reviewer receives this folder-style specification, record
   its result in `agent-review-reports.md` using
   `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1488
- Prerequisites: #2234 and #2274
- Related ADR:
   `docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`

### Manual Verification Procedure

1. Run focused HTTP component and bootstrap integration tests that cancel an
   injected token, recording their output.
2. Run the tracker with one HTTP binding, then send SIGTERM to the tracker
   binary after SI-1. Record the `main()` signal-boundary log and HTTP drain
   completion in the correct order.
3. Start an HTTP tracker through an unchanged legacy start/stop call path and
   confirm it still compiles and stops using its legacy behavior.
4. Review the migrated token-aware path to confirm it has no OS-signal listener
   and retains every drain-controller handle it creates.
