---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1488
github-issue: 2449
spec-path: docs/issues/open/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md
branch: "2449-migrate-rest-api-test-environment"
related-pr: null
last-updated-utc: "2026-10-07 13:05"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - packages/axum-rest-api-server/src/testing/environment.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-rest-api-server/tests/server/v1/contract/context/auth_key.rs
    - packages/axum-health-check-api-server/tests/server/contract.rs
    - packages/axum-server/src/signals.rs
    - packages/axum-http-server/src/testing/environment.rs
    - docs/issues/closed/2309-1488-si-12-migrate-rest-api-token-lifecycle/ISSUE.md
    - docs/issues/closed/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md
    - docs/issues/closed/2448-1488-si-17-migrate-standalone-udp-environment/ISSUE.md
    - docs/issues/drafts/1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
    - docs/issues/drafts/1488-si-19-remove-legacy-shutdown-api/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2449 - Migrate the REST API Test Environment to the Token Lifecycle

Parent: [EPIC #1488 - Overhaul: Tracker Shutdown](../../open/1488-overhaul-tracker-shutdown/ISSUE.md)

> **EPIC position**: Roadmap sequence 16 (SI-23). Found while refreshing SI-17
> (2026-10-06): no roadmap item migrated this environment, and SI-18 and SI-19
> cannot deprecate or remove the legacy path while it depends on it.

## Goal

Make the REST API test `Environment` use the token-aware REST API server
lifecycle. `Environment::stop()` cancels its token and joins the server task
and the drain controller instead of stopping through the legacy `Halted` path.

## Background

SI-12 (#2309) added `ApiServer::start_with_cancellation`, which returns a
`CancellationRunning` with a joinable server task and drain controller; the
tracker application uses it. The package's test environment
(`packages/axum-rest-api-server/src/testing/environment.rs`) was not migrated
and is not in the EPIC roadmap:

1. **Legacy start and stop.** `start` calls `ApiServer::start` and `stop` calls
   `ApiServer::stop()`; both use the `Halted` path, which subscribes to SIGINT
   and SIGTERM inside the library until SI-19.
2. **Many callers.** About 58 `stop()` calls under
   `packages/axum-rest-api-server/tests/` use the environment.
3. **A consumer stops the server through the field.**
   `packages/axum-health-check-api-server/tests/server/contract.rs` calls
   `service.server.stop()` on the REST API `Started` environment. The `server`
   field is public; `get_connection_info()` and `bind_address()` read it.
4. **No owned listeners, no example.** Unlike SI-16 and SI-17, the environment
   spawns no event listener and the package has no example binary, so there is
   no listener-leak risk and no OS-signal boundary to move.
5. **The drain is bounded.** The shared helper force-closes connections at the
   90 s `API_GRACEFUL_DRAIN_TIMEOUT` deadline (SI-16 D7) and reports
   `TimedOut`.
6. **TLS.** The environment builds a TLS configuration when one is set; the
   token-aware start supports the same launcher.

## Scope

### In Scope

- Migrate `Environment` start and stop to `ApiServer::start_with_cancellation`,
  with a fresh token per start.
- Migrate the health-check API contract test that calls `service.server.stop()`.
- Deterministic tests for the new `stop()` guarantees.
- Update the shutdown task inventory.

### Out of Scope

- The HTTP, UDP, and health-check API test environments (SI-16, SI-17, SI-24).
- The REST API server, its drain budget, and application bootstrap.
- Deprecating or removing the legacy lifecycle (SI-18, SI-19).

## Design Decisions

- **D1 - Keep the `stop()` signature.** `stop(self) -> Environment<Stopped>`
  stays. It joins the server task and the drain controller, then panics naming
  each failing task on a join error, a server error, or a drain `TimedOut`, as
  SI-16 does.
- **D2 - One fresh token per start.** A stopped environment never reuses a
  cancelled token, so stop-then-start works.
- **D3 - Running state holds the token-aware handles.** `Environment<Running>`
  stores the token-aware running state instead of `ApiServer<Running>`, behind
  the existing `Started` alias (and a new `Unstarted` alias, as in SI-16).
  `bind_address()`, `get_connection_info()`, `registar`, and `container` stay;
  `Environment::stop()` is the only stop path.
- **D4 - No outer stop timeout.** The drain helper bounds the stop (fact 5);
  tests bound their own waits.

## Architectural Decisions

- Related ADRs:
  [Adopt a supervised cancellation tree for shutdown](../../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md).
- ADRs to create: none known.

## Design and Ownership Review

- **Owner**: `Environment<Running>` owns the server task, the drain controller,
  and the token that stops them.
- **Normal path**: `stop()` cancels the token, joins both tasks, and returns
  `Environment<Stopped>`; the binding is released when it returns.
- **Failure path**: join both tasks before panicking (D1).
- **Drop path**: dropping a running environment without `stop()` must still
  stop the server and release the binding, as the legacy halt sender does
  today. Dropping a `CancellationToken` does not cancel it, so hold it as a
  `DropGuard` and disarm it in `stop()`; SI-16 lost this behavior without a
  test (#2471), and the UDP environment has the fix (#2459). No new detached
  task.
- **Deadlines**: the drain is bounded by `API_GRACEFUL_DRAIN_TIMEOUT`; tests
  bound every start and stop with an explicit test deadline.
- **Checkpoint**: after T1, stop for a design review before migrating the
  consumer.

## Bug-Fix Process

Not applicable. This is a planned migration.

## Regression Test Strategy

Not applicable (not a bug).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                | Notes / Expected Output                                                                  |
| --- | ------ | ----------------------------------- | ---------------------------------------------------------------------------------------- |
| T0  | TODO   | Record baseline                     | Package test results and counts.                                                         |
| T1  | TODO   | Migrate `Environment` start/stop    | D1-D4, with the T1 tests below, each mutation-proven.                                    |
| T2  | TODO   | Migrate the direct field consumer   | Health-check API contract test uses `Environment::stop()`.                               |
| T3  | TODO   | Documentation                       | Task inventory records the migrated REST API test environment.                          |
| T4  | TODO   | Verification and completion review  | Automatic checks, manual scenario, AC review, pre-push checks, independent review.      |

T1 tests (use the `write-unit-test` skill, every wait bounded):

- `stop()` joins the server and the drain controller without a failure.
- `stop()` releases the binding: the same address binds right after `stop()`.
- Stop-then-start serves requests on the restarted environment.
- A drain timeout is reported after both tasks join.
- Dropping a running environment without `stop()` releases the binding
  (poll `TcpListener::bind` within a bounded deadline, as #2471's test does).

## Commit Points

| Task  | Coherent change set                                       | Commit policy                                                      |
| ----- | --------------------------------------------------------- | ------------------------------------------------------------------ |
| T0    | Baseline evidence in `manual-verification-evidence.md`    | Commit with T1 or separately.                                      |
| T1-T2 | Environment, tests, and direct field consumer migration   | Commit together after design review, since the field type changes. |
| T3    | Shutdown documentation updates                            | Commit after `linter all`.                                         |
| T4    | Verification evidence, AC review, and completion review   | Commit after all checks pass.                                      |

For test-producing work, use the `write-unit-test` skill and complete the
prose-first Arrange-Act-Assert design review after each passing test increment,
before maintainer review and commit. Sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted (moved to `docs/issues/open/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md`)
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2449)
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
- 2026-10-06 11:41 UTC - GitHub Copilot - Maintainer approved this spec. Created #2449 (sub-issue of #1488) and moved the spec to `docs/issues/open/`.
- 2026-10-07 13:05 UTC - GitHub Copilot - The drop path is no longer "unchanged": a token migration loses the halt sender's stop-on-drop, as SI-16 did (#2471). Added the `DropGuard` requirement, a T1 drop test, and AC7.

## Acceptance Criteria

- [ ] AC1: `Environment` starts the REST API server through
      `start_with_cancellation`.
- [ ] AC2: `Environment::stop()` cancels the token and joins the server task
      and the drain controller.
- [ ] AC3: `stop()` panics naming each failing task, after joining both, and
      keeps its current signature.
- [ ] AC4: A stopped environment can be started again and serves requests.
- [ ] AC5: The health-check API contract tests pass without using the REST API
      environment's `server` field.
- [ ] AC6: The shutdown task inventory reflects the migration.
- [ ] AC7: Dropping a running environment without `stop()` releases its
      binding within the test deadline.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-axum-rest-api-server`
- `cargo test -p torrust-tracker-axum-health-check-api-server`
- Pre-push checks before opening the implementation PR

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

The package has no example binary, so there is no signal scenario.

| ID  | Scenario                     | Human-oriented command/steps                                                     | Expected Result                                                     | Status | Evidence                                     |
| --- | ---------------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | No legacy stop in the env    | Read the final `environment.rs`; list every `ApiServer` method it calls.         | Only `start_with_cancellation`; no legacy `start` or `stop`.        | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Suite timing                 | Run the REST API package tests before and after; record wall time.              | Comparable time; any slowdown explained (SI-16 saw ~1 s per stop). | TODO   | `manual-verification-evidence.md` section V2 |

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
| AC7   | TODO                   |          |

## Dependencies

- Done: SI-12 (#2309) token-aware REST API lifecycle; SI-16 (#2412) as the
  reference migration and the bounded drain helper.
- Blocks: SI-18 and SI-19.

## Risks and Trade-offs

- **Slower suite.** SI-16 found each stop with an idle keep-alive connection
  takes about 1 s, because the drain helper polls the connection count every
  second. With about 58 stops, the REST API suite may slow noticeably; M2
  measures it.
- **Slow drains block tests.** A test that holds a connection makes `stop()`
  wait up to 90 s, then panic.

## Rollback

Revert the environment and consumer commits; the legacy path is still
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
- Related issues: #2309 (SI-12)
- Follow-up items: SI-18, SI-19
