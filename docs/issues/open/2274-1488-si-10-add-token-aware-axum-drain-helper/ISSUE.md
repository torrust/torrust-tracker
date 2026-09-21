---
doc-type: issue
issue-type: task
status: open
priority: p1
epic: 1488
github-issue: 2274
spec-path: docs/issues/open/2274-1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md
branch: "2274-1488-si-10-add-token-aware-axum-drain-helper-spec"
related-pr: 2275
last-updated-utc: 2026-09-21 16:17
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-server/src/signals.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-health-check-api-server/src/server.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2274 - Add Token-Aware, Joinable Axum Drain Helper

> **EPIC position**: Roadmap step 6. Additive shared-helper task. It preserves
> the existing `Halted`-based helper while introducing a token-aware alternative.

Parent: [EPIC #1488 - Overhaul: Tracker Shutdown](../../open/1488-overhaul-tracker-shutdown/ISSUE.md)

## Goal

Add a new shared Axum graceful-shutdown helper that accepts an injected
`CancellationToken`, starts connection draining on cancellation, and can be
awaited by the server task that owns it. The existing
`graceful_shutdown(handle, rx_halt, message, address)` helper remains unchanged
in this task so all existing consumers continue to compile and behave as before.

This task creates a reusable building block; it does not migrate the HTTP
tracker, REST API, or health-check API to the new helper.

## Background

`packages/axum-server/src/signals.rs` currently spawns the shutdown path from
server implementations with `tokio::task::spawn(graceful_shutdown(...))` and
discards the returned `JoinHandle`. The helper waits on a shutdown `Halted`
channel or library-level OS signals, starts `Handle::graceful_shutdown`, and
polls connection count.

Under the Q2 target architecture, cancellation flows from the owner to a child
component token and completion flows upward through awaited handles. A detached
drain helper prevents the owner from proving that drain completed before its
server task reports completion.

## Scope

### In scope

- Add a token-aware helper beside the existing helper.
- Accept an injected `CancellationToken`, Axum `Handle`, address, message, and
  a post-cancellation drain-timeout budget suitable for later Q4 configuration.
- Have the helper await token cancellation, request Axum graceful shutdown, and
  return a typed result that distinguishes drained versus deadline-reached.
- Make the helper usable as a future that a server task can await or spawn and
  retain as a `JoinHandle`.
- Add deterministic tests that cancel a token without delivering OS signals.
- Document the ownership contract for future HTTP, REST API, and health-check
  component migrations.

### Out of scope

- Migrating any existing Axum server consumer to the new helper.
- Removing or changing `Halted`, `shutdown_signal_with_message`, or
  `global_shutdown_signal()`.
- Selecting production deadline values or configuration schema (Q4).
- Exit-code behavior (Q3) and readiness behavior (Q6).

## Architectural Decisions

- Related ADR: [Supervised cancellation tree](../../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md).
- The new API is additive. The legacy `Halted` helper and its callers remain behaviorally and source compatible until the migration and deprecation subissues are complete.
- The helper waits for injected cancellation and reports drain completion; it does not subscribe to operating-system signals or create detached tasks.
- ADRs to create: None known. Create one if implementation exposes a consequential public lifecycle API or ownership decision not covered by the existing ADR.

## Design and Ownership Review

| Collaborator | Responsibility and ownership contract |
| --- | --- |
| Application and `JobManager` | Own root cancellation and top-level component futures. They never receive the helper's nested task handle. |
| Migrating server component (SI-11 through SI-13) | Own the helper future or its retained `JoinHandle`, coordinate it with the server future, and await it before reporting component completion. |
| Token-aware drain helper | Wait for the injected token, request `Handle::graceful_shutdown`, then start the supplied post-cancellation drain timeout and wait until connections drain or that timeout expires. Return the typed outcome without spawning a task. |
| Axum `Handle` | Performs graceful connection draining after the helper requests it. |

Normal path invariant: a component awaits its drain future after cancellation and does not complete while it still owns that future. Failure and drop-path invariant: a component joins or deliberately aborts and records its retained child before returning; it never leaks a detached drain controller. `drain_timeout` bounds only the wait after cancellation is observed and `Handle::graceful_shutdown` is requested. The first passing vertical slice requires a design review confirming this API and ownership map remain accurate.

## Bug-Fix Process

Not applicable. This is an additive lifecycle API task; it preserves the existing helper unchanged.

## Regression Test Strategy

Not applicable. The task adds new behavior. Deterministic unit tests at the `axum-server` helper boundary will protect cancellation, drained, and deadline outcomes; existing consumer tests protect the legacy compatibility contract.

## Proposed API Shape

The exact type names are implementation decisions. The API must preserve this
shape of responsibility:

```rust
pub async fn graceful_shutdown_on_cancellation(
    handle: axum_server::Handle<SocketAddr>,
    cancellation_token: CancellationToken,
    message: String,
    address: SocketAddr,
    drain_timeout: Duration,
) -> GracefulShutdownOutcome
```

The caller owns the returned future or its spawned `JoinHandle`. The helper must
not spawn an unowned background task internally. A later component migration
uses a `tokio::select!` between its server future and this owned drain future,
then awaits any remaining owned child before returning the component outcome.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Establish helper contract | Inspect existing helper and Axum handle behavior; finalize public input and typed outcome without changing the legacy API. |
| T2 | TODO | Implement token-aware drain helper | Add the awaitable helper with an injected token and supplied post-cancellation drain timeout; do not spawn an unowned task. |
| T3 | TODO | Add deterministic helper tests | Cover no drain before cancellation, successful drain, and post-cancellation timeout outcome without OS signals; apply the `write-unit-test` workflow and record the test-design review. |
| T4 | TODO | Verify legacy compatibility | Run unchanged focused tests for Axum HTTP, REST API, and health-check consumers; confirm their call sites remain unchanged. |
| T5 | TODO | Perform completion review | Complete automated and manual verification, re-review acceptance criteria, and record implementation-retrospective decision. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Add the typed token-aware helper without modifying the legacy helper | Commit after focused validation and ownership review. |
| T3 | Add deterministic helper tests and test-design review evidence | Commit after focused tests and prose-first Arrange-Act-Assert review. |
| T4-T5 | Add only required documentation and verification evidence | Commit after final validation and completion review; record a no-change decision when no source artifact changes. |

Test-producing work must use the `write-unit-test` skill. After each passing test increment, record the prose-first Arrange-Act-Assert review: the fixture owns incidental mechanics, the causal initial-state difference is visible, and the production Act plus expected result remain visible. Use a signed Conventional Commit with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style specification drafted in `docs/issues/drafts/1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md`
- [x] Specification reviewed and approved by user/maintainer
- [x] GitHub issue #2274 created and issue number added to this specification
- [ ] Recommended spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks when applicable)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers receive this folder-style specification
- [x] Committer verified specification progress is up to date before commit
- [ ] Issue closed and specification moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-21 15:33 UTC - GitHub Copilot - Updated the inherited SI-10 draft with current template-required ownership, implementation, verification, and completion-review checkpoints; awaiting user/maintainer review before GitHub issue creation.
- 2026-09-21 15:37 UTC - GitHub Copilot - Created GitHub issue [#2274](https://github.com/torrust/torrust-tracker/issues/2274) with the `task` label and verified its GitHub sub-issue relationship to EPIC #1488; moved this specification to `open/`.
- 2026-09-21 15:44 UTC - GitHub Copilot - Created the documentation-only branch `2274-1488-si-10-add-token-aware-axum-drain-helper-spec`; preparing the required pre-commit gate and spec-only pull request.
- 2026-09-21 15:45 UTC - Committer - Verified branch-to-spec mapping and intended documentation-only commit scope; pre-commit hook is installed and will run during the signed commit.
- 2026-09-21 15:51 UTC - GitHub Copilot - Opened spec-only PR [#2275](https://github.com/torrust/torrust-tracker/pull/2275) targeting `develop` from this branch with `Related to #2274`; awaiting review and merge before implementation.
- 2026-09-21 16:17 UTC - GitHub Copilot - Addressed Copilot review findings in PR #2275: defined `drain_timeout` as a post-cancellation budget and repaired all discovered stable shutdown-issue references; recorded the review report before replying and resolving threads.

## Acceptance Criteria

- [ ] Existing `graceful_shutdown` behavior and public signature are unchanged.
- [ ] A new token-aware helper accepts injected cancellation without subscribing
      to an OS signal or receiving a shutdown `Halted` channel.
- [ ] The helper starts `Handle::graceful_shutdown` only after token
      cancellation.
- [ ] The helper returns an outcome that distinguishes all connections drained
  from the post-cancellation drain timeout reached.
- [ ] The helper does not create an unowned task. Its caller can await it or
      retain its join handle.
- [ ] Deterministic tests cancel an injected token and cover both drained and
  post-cancellation timeout outcomes without OS signals.
- [ ] Existing HTTP tracker, REST API, and health-check server tests still pass
      unchanged against the legacy helper.
- [ ] `linter all` passes.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- Focused `axum-server` unit tests for the token-aware helper.
- Existing focused tests for Axum HTTP tracker, REST API, and health-check server packages, proving legacy call-site compatibility.
- `linter all` on the stable Rust toolchain.
- Pre-push checks when preparing the implementation pull request.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Inspect helper ownership | Review the implemented helper and its caller-side use; confirm it takes `CancellationToken` and `drain_timeout`, returns a typed outcome, and contains no detached `tokio::spawn`. | The component can await or retain every drain task it owns, and the timeout starts only after cancellation. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Exercise graceful drain | Start a representative Axum server using the new helper, establish an in-flight connection, cancel its token, then observe the connection finish before `drain_timeout` elapses. | New connections stop, the in-flight connection drains, and the helper returns the drained outcome. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Exercise timeout outcome | Repeat M2 with a connection that remains active past a deliberately small `drain_timeout`. | The helper returns the timeout outcome without an OS signal. | TODO | `manual-verification-evidence.md` section V3 |

Manual verification is mandatory. Create `manual-verification-evidence.md` from the repository template when executing these scenarios and record actual commands, toolchain/runtime, output, relevant logs, and results. Record any failed scenario and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The behaviors belong in maintained Rust tests; manual scenarios use the actual server and no disposable script is justified.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| Existing legacy helper unchanged | TODO | Focused consumer tests and source review |
| Token-aware injected cancellation | TODO | `axum-server` unit tests and V1 |
| Graceful drain begins on cancellation | TODO | `axum-server` unit tests and V2 |
| Distinct drained and timeout outcomes | TODO | `axum-server` unit tests and V2-V3 |
| No unowned helper task | TODO | Source review and V1 |
| Linter passes | TODO | `linter all` output |

## Risks and Trade-offs

- Axum `Handle` behavior may make fully deterministic drain tests difficult. Mitigation: isolate the outcome policy in a testable helper boundary and use local server tests only for the real handle interaction.
- Prematurely selecting production timeout values would couple this helper to SI-20 policy work. Mitigation: accept a caller-provided post-cancellation timeout and defer configuration defaults.
- A convenience API that spawns internally would obscure lifecycle ownership. Mitigation: expose an awaitable future and make retained handle ownership explicit in the later consumer migrations.

## Implementation Completion Review

After implementation, compare the result with this specification; record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: Not yet assessed.
- Create `implementation-retrospective.md` from the repository template for material discoveries, design changes, or deviations; otherwise add a concise progress-log entry explaining why none was needed.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using the repository template.

## References

- Parent EPIC: #1488
- Related issue: #2234
- Related ADR: [Supervised cancellation tree](../../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md)

## Dependencies

- Follows the additive server lifecycle API from SI-2.
- Does not depend on migration of any Axum server consumer.
- Q4 later defines final deadline relationships and configuration; this task
  only provides an input for the budget.

## Rollback

This is additive. Reverting it removes only the unused new helper and tests;
all existing Axum consumers keep using the unchanged legacy helper.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Run the focused deterministic tests and record their output.
2. Confirm existing server packages compile and their existing tests pass
   without changing their call sites.
3. Confirm the new helper has no OS-signal subscription or shutdown `Halted`
   channel parameter.
4. Confirm no `tokio::spawn` inside the new helper discards a join handle.
