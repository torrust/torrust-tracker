---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1488
github-issue: 2450
spec-path: docs/issues/open/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md
branch: "2450-migrate-health-check-api-test-environment"
related-pr: null
last-updated-utc: "2026-10-06 12:23"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - packages/axum-health-check-api-server/src/environment.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/axum-health-check-api-server/tests/server/contract.rs
    - packages/axum-server/src/signals.rs
    - src/bootstrap/jobs/health_check_api.rs
    - docs/issues/closed/2324-1488-si-13-migrate-health-check-api-token-lifecycle/ISSUE.md
    - docs/issues/closed/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md
    - docs/issues/open/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md
    - docs/issues/drafts/1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
    - docs/issues/drafts/1488-si-19-remove-legacy-shutdown-api/ISSUE.md
    - docs/issues/drafts/1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2450 - Migrate the Health-Check API Test Environment to the Token Lifecycle

Parent: [EPIC #1488 - Overhaul: Tracker Shutdown](../../open/1488-overhaul-tracker-shutdown/ISSUE.md)

> **EPIC position**: Roadmap sequence 16 (SI-24). Found while refreshing SI-17
> (2026-10-06): no roadmap item migrated this environment, and SI-18 and SI-19
> cannot deprecate or remove the legacy path while it depends on it.

## Goal

Make the health-check API test `Environment` use the token-aware health-check
server lifecycle. `Environment::stop()` cancels its token and joins the server
task and the drain controller instead of sending a `Halted` message.

## Background

SI-13 (#2324) added `server::start_with_cancellation`, which returns a
`CancellationRunning` with a joinable server task and drain controller; the
tracker application uses it. The package's environment
(`packages/axum-health-check-api-server/src/environment.rs`) was not migrated
and is not in the EPIC roadmap:

1. **Legacy start and stop.** `start` spawns a task that calls legacy
   `server::start` with `Started`/`Halted` channels; `stop` sends
   `Halted::Normal` and awaits that task. The legacy path subscribes to SIGINT
   and SIGTERM inside the library until SI-19.
2. **Public module and fields.** The environment lives in `src/environment.rs`,
   not a `testing` module, and its `state` field is public: the contract tests
   read `env.state.binding` 8 times.
3. **`stop()` returns a `Result`.** Unlike the HTTP, UDP, and REST API
   environments, `stop()` returns `Result<Environment<Stopped>, Error>`, with a
   single string variant `Error::Error(String)`; 8 contract-test calls use it.
4. **Caller-provided registar.** `new` takes the registar of the services under
   check, so the environment does not own the services it reports on.
5. **No owned listeners, no example.** No event listener, no example binary.
6. **The drain is bounded.** The shared helper force-closes at the 5 s
   `HEALTH_CHECK_API_GRACEFUL_DRAIN_TIMEOUT` deadline (SI-16 D7).

## Scope

### In Scope

- Migrate `Environment` start and stop to `server::start_with_cancellation`,
  with a fresh token per start.
- Give the environment a `bind_address()` accessor and migrate the 8
  `env.state.binding` reads to it.
- Deterministic tests for the new `stop()` guarantees.
- Update the shutdown task inventory.

### Out of Scope

- The other test environments (SI-16, SI-17, SI-23).
- Readiness during shutdown (SI-21).
- Moving the environment into a `testing` module: a public path change for no
  shutdown benefit.
- Deprecating or removing the legacy lifecycle (SI-18, SI-19).

## Design Decisions

- **D1 - Keep the `stop()` signature.** `stop` keeps returning
  `Result<Environment<Stopped>, Error>`, so the 8 callers do not change. It
  joins both tasks, then returns an error naming each failing task (join
  error, server error, or drain `TimedOut`. Approved at spec review
  (2026-10-06) over the panic contract of the other environments.
- **D2 - One fresh token per start.** A stopped environment never reuses a
  cancelled token.
- **D3 - Private running state.** `Running` holds the token-aware handles and
  stops being public; callers use `bind_address()`. `Stopped` keeps its bind
  address.
- **D4 - Registration metadata.** The token-aware start needs
  `RuntimeServiceMetadata`; use the health-check API role with instance 0, as
  the application does.
- **D5 - No outer stop timeout.** The drain helper bounds the stop (fact 6).

## Architectural Decisions

- Related ADRs:
  [Adopt a supervised cancellation tree for shutdown](../../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md).
- ADRs to create: none known.

## Design and Ownership Review

- **Owner**: `Environment<Running>` owns the server task, the drain controller,
  and the token that stops them. It does not own the services in its registar.
- **Normal path**: `stop()` cancels the token, joins both tasks, and returns
  `Environment<Stopped>`; the binding is released when it returns.
- **Failure path**: join both tasks before returning an error (D1).
- **Drop path**: unchanged; no new detached task.
- **Deadlines**: the drain is bounded by
  `HEALTH_CHECK_API_GRACEFUL_DRAIN_TIMEOUT`; tests bound every start and stop.
- **Checkpoint**: after T1, stop for a design review before migrating the
  field reads.

## Bug-Fix Process

Not applicable. This is a planned migration.

## Regression Test Strategy

Not applicable (not a bug).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                | Notes / Expected Output                                                             |
| --- | ------ | ----------------------------------- | ----------------------------------------------------------------------------------- |
| T0  | TODO   | Record baseline                     | Package test results.                                                               |
| T1  | TODO   | Migrate `Environment` start/stop    | D1-D5, with the T1 tests below, each mutation-proven.                               |
| T2  | TODO   | Migrate the field reads             | Contract tests use `bind_address()`.                                                |
| T3  | TODO   | Documentation                       | Task inventory records the migrated health-check API environment.                  |
| T4  | TODO   | Verification and completion review  | Automatic checks, manual scenario, AC review, pre-push checks, independent review. |

T1 tests (use the `write-unit-test` skill, every wait bounded):

- `stop()` joins the server and the drain controller without an error.
- `stop()` releases the binding: the same address binds right after `stop()`.
- Stop-then-start serves the health check on the restarted environment.
- A drain timeout is returned as an error after both tasks join.

## Commit Points

| Task  | Coherent change set                                     | Commit policy                                                        |
| ----- | ------------------------------------------------------- | -------------------------------------------------------------------- |
| T0    | Baseline evidence in `manual-verification-evidence.md`  | Commit with T1 or separately.                                        |
| T1-T2 | Environment, tests, and field-read migration            | Commit together after design review, since `Running` becomes private. |
| T3    | Shutdown documentation updates                          | Commit after `linter all`.                                           |
| T4    | Verification evidence, AC review, and completion review | Commit after all checks pass.                                        |

For test-producing work, use the `write-unit-test` skill and complete the
prose-first Arrange-Act-Assert design review after each passing test increment,
before maintainer review and commit. Sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted (moved to `docs/issues/open/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md`)
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2450)
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-06 11:34 UTC - GitHub Copilot - Drafted after the SI-17 refresh found this environment still on the legacy path and missing from the roadmap; the maintainer chose one item per environment.
- 2026-10-06 11:41 UTC - GitHub Copilot - Maintainer approved this spec, including D1 (keep the `Result` signature). Created #2450 (sub-issue of #1488) and moved the spec to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: `Environment` starts the health-check API through
      `start_with_cancellation`.
- [ ] AC2: `Environment::stop()` cancels the token and joins the server task
      and the drain controller.
- [ ] AC3: `stop()` returns an error naming each failing task, after joining
      both, and keeps its current signature (D1).
- [ ] AC4: A stopped environment can be started again and serves the health
      check.
- [ ] AC5: No caller reads the environment's running state directly.
- [ ] AC6: The shutdown task inventory reflects the migration.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-axum-health-check-api-server`
- Pre-push checks before opening the implementation PR

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

The package has no example binary, so there is no signal scenario.

| ID  | Scenario                  | Human-oriented command/steps                                               | Expected Result                                                 | Status | Evidence                                     |
| --- | ------------------------- | -------------------------------------------------------------------------- | --------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | No legacy stop in the env | Read the final `environment.rs`; list every `server` function it calls.    | Only `start_with_cancellation`; no `Halted` or `Started` usage. | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Suite timing              | Run the package tests before and after; record wall time.                 | Comparable time; any slowdown explained.                        | TODO   | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record the toolchain for every command result that it can affect.
- Create `manual-verification-evidence.md` from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` at T0.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Dependencies

- Done: SI-13 (#2324) token-aware health-check API lifecycle; SI-16 (#2412) as
  the reference migration and the bounded drain helper.
- Best after SI-17 and SI-23: the contract tests also stop UDP and REST API
  environments, so migrating those first keeps this diff to the health-check
  environment.
- Blocks: SI-18 and SI-19.

## Risks and Trade-offs

- **Public API change.** `Running` and the `state` field are public in a
  non-`testing` module; external users of them break. The module is test
  support, so the risk is low; SI-19 is the breaking release anyway.

## Rollback

Revert the environment and field-read commits; the legacy path is still
available.

## Implementation Completion Review

- Retrospective: `TODO`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory;
  otherwise add a progress-log entry explaining why it was not needed.
- Independent reviewers record results in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1488
- Reference migration: #2412 (SI-16), PR #2439
- Related issues: #2324 (SI-13)
- Follow-up items: SI-18, SI-19, SI-21
