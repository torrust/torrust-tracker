---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: 1488
github-issue: 2471
spec-path: docs/issues/open/2471-1488-fix-http-environment-drop-path/ISSUE.md
branch: "2471-fix-http-environment-drop-path"
related-pr: null
last-updated-utc: "2026-10-15 19:38"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/axum-http-server/src/testing/environment.rs
    - packages/udp-server/src/testing/environment.rs
    - docs/issues/closed/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md
    - docs/issues/closed/2448-1488-si-17-migrate-standalone-udp-environment/ISSUE.md
    - docs/pr-reviews/pr-2459-review/PR-REVIEW.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2471 - Stop the HTTP Test Environment's Tasks When It Is Dropped Without `stop()`

Parent: [EPIC #1488 - Overhaul: Tracker Shutdown](../1488-overhaul-tracker-shutdown/ISSUE.md)

> A gap found on 2026-10-07 while checking SI-22's specs against what SI-17
> learned: the HTTP test environment never stopped its server when dropped
> without `stop()`, before or after SI-16 (#2412, PR #2439). SI-17 fixed the
> same gap in the UDP environment during review (PR #2459, finding F1); this
> issue aligns HTTP with UDP. An earlier version of this spec called it a
> regression from SI-16; PR #2544 review finding F5 (reviewer ID F1) showed
> that was wrong, and V4 confirms it.

## Goal

Dropping a running HTTP test environment without calling `stop()` stops its
HTTP server and statistics event listener, as dropping the UDP test
environment does, so the HTTP binding is released.

## Background

### What happens

1. A test starts `axum-http-server`'s test environment. `start()` creates a
   plain `CancellationToken`, passes clones to the HTTP server and the
   statistics event listener, and keeps one in the running state.
2. The test drops the environment without calling `stop()`, for example when
   an assertion panics before the test's own `stop()` call.
3. Dropping a `CancellationToken` does not cancel it, and dropping the task
   handles detaches the tasks.
4. The HTTP server keeps running and keeps its binding until the Tokio runtime
   shuts down. The reproduction (V1) bound the same address every 10 ms for
   10 s after the drop and never succeeded.

### Which expectation it breaks

- Before SI-16, the running state held the legacy halt sender. Dropping it
  closed the halt channel, so `torrust-server-lib`'s `shutdown_signal`
  panicked ("Failed to install stop signal") inside the spawned
  `graceful_shutdown` task. That panic happens before the task calls
  `handle.graceful_shutdown`, so the server kept serving and kept its
  binding. V4 ran the drop test on the tree before SI-16 and observed it:
  the binding was still taken after 10 s. SI-16's Design and Ownership
  Review says a drop without `stop()` "keeps today's behavior", which was
  accurate: the token migration kept the gap and no SI-16 test covered the
  drop path.
- The UDP test environment stops on drop since PR #2459: its legacy launcher
  has treated a closed halt channel as a stop since SI-14, and its running
  state now holds the token as a `DropGuard`. Its test
  `it_should_release_the_udp_socket_when_dropped_without_being_stopped`
  proves it. The two environments differ until this issue aligns HTTP with
  UDP.

### Impact

Test-only and low. Each `#[tokio::test]` has its own runtime, which stops the
leaked tasks when the test ends, so the leak lasts until the end of the
failing test, not across tests. It matters when a test drops an environment
and binds the same address again, or when SI-22 T9 restructures this
environment and relies on the drop path.

## Scope

### In Scope

- The HTTP test environment cancels its token when dropped while running.
- A regression test for the drop path, mirroring the UDP one.

### Out of Scope

- The REST API and health-check API test environments (SI-23 #2449, SI-24
  #2450): they still use the legacy halt sender. Whether dropping one stops
  its server was not checked here; V4 shows only that the HTTP environment's
  legacy drop did not. Their specs require the drop path for the migrated
  environments.
- Separate server and listener tokens (SI-22 T9).

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`
- ADRs to create: None known. The fix copies the UDP environment's decision.

## Design and Ownership Review

- **Interface**: unchanged; `start()`, `stop()`, and the public fields stay as
  they are.
- **Ownership**: the running state owns the environment token as a
  `DropGuard`. `stop()` disarms it, then cancels and joins exactly as today.
- **Drop path**: dropping a running environment cancels the token. It cannot
  await the tasks, so the binding is released shortly after the drop, not at
  once; the test polls within its deadline.
- **Deadlines**: the regression test bounds its wait with the module's
  existing `TEST_DEADLINE` (10 s).
- **Checkpoint**: after the regression test turns green (T2), stop for a
  design review before maintainer review and commit. Record the prose-first
  Arrange-Act-Assert review of the test, and check that `stop()` and the drop
  path still behave as described above.

## Bug-Fix Process

Follows [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

1. **Analysis** (done): the running state holds a plain token; see
   Background.
2. **Reproduction** (done, **Reproduced**): a temporary test dropped a started
   environment and waited for the binding; it timed out (V1). V4 ran the same
   test on the tree before SI-16 and it timed out too.
3. **Regression-test boundary**: the environment's unit tests (below).
4. **Red**: add the maintained test and record it failing before the fix.
5. **Fix**: hold the token as a `DropGuard`; disarm it in `stop()`.
6. **Green and recheck**: rerun the test and the package tests; rerun the V1
   reproduction like-for-like.

## Regression Test Strategy

A unit test in `packages/axum-http-server/src/testing/environment.rs`,
`it_should_release_the_http_binding_when_dropped_without_being_stopped`:
start an environment, drop it, and poll `TcpListener::bind` on its address
until it succeeds or `TEST_DEADLINE` expires. This is the causal seam: the
environment decides what dropping it does. It mirrors the UDP test, so the
two environments are guarded the same way.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                 | Notes / Expected Output                                                        |
| --- | ------ | -------------------- | ------------------------------------------------------------------------------ |
| T0  | DONE   | Reproduction         | V1: Reproduced; binding still taken 10 s after the drop.                       |
| T1  | DONE   | Red regression test  | The maintained test fails against the plain token; red output recorded (V2). |
| T2  | DONE   | Fix                  | Running state holds a `DropGuard`; `stop()` disarms it before cancelling.      |
| T3  | IN_PROGRESS | Green and recheck    | Test and package tests pass; V1 rerun passes; outputs recorded (V3). `linter all` and pre-push checks pending. |

## Commit Points

| Task  | Coherent change set               | Commit policy                                                     |
| ----- | --------------------------------- | ----------------------------------------------------------------- |
| T1-T2 | Regression test and the fix       | One commit after the red run is recorded, the test passes, and the design-review checkpoint is recorded. |
| T3    | Evidence and spec progress        | Commit after automatic and manual verification.                   |

Use the `write-unit-test` skill and the prose-first Arrange-Act-Assert review
for the test. Sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1488-fix-http-environment-drop-path/ISSUE.md`
- [x] Reproduction attempted and classified in `manual-verification-evidence.md` (V1: Reproduced)
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation (#2483)
- [x] Implementation completed
- [x] Design-review checkpoint recorded after the regression test turned green, before maintainer review and commit
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-07 11:09 UTC - GitHub Copilot - Drafted after checking SI-22's specs against SI-17's drop-path finding. Reproduced with a temporary test (V1), recorded verbatim and reverted; the pre-SI-16 halt-sender drop path confirmed in `torrust-server-lib` 0.3.0.
- 2026-10-07 12:45 UTC - GitHub Copilot - Maintainer approved the spec. Created #2471, linked it as a sub-issue of EPIC #1488, and moved the spec to `docs/issues/open/`. It ships in a spec-only PR with the SI-22, SI-23 and SI-24 spec refreshes; the fix gets its own PR before SI-22 starts.
- 2026-10-15 17:20 UTC - GitHub Copilot - Implemented on branch `2471-fix-http-environment-drop-path`. Added the regression test and recorded it red (V2: binding still taken after 10.02 s), then held the token as a `DropGuard` and disarmed it in `stop()` (V3: 7 environment tests, 44 unit and 62 integration tests pass; clippy clean).
- 2026-10-15 17:20 UTC - GitHub Copilot - Design-review checkpoint (prose-first Arrange-Act-Assert). Arrange: a started HTTP environment on an ephemeral port, whose binding is read before the drop. Act: drop the environment without `stop()`. Assert: within `TEST_DEADLINE` the same address can be bound again, polled because the guard only requests cancellation and the server task releases the socket when it next runs. The test fails for the defect (V2) and passes for the fix (V3), and mirrors the UDP test. Checked `stop()`: it disarms the guard, then cancels once and joins the server task, its drain controller, and the listener as before, so the drop path does not run on a normal stop; the existing stop, restart and failure-join tests still pass. The `Running` interface and public fields are unchanged.
- 2026-10-15 19:36 UTC - GitHub Copilot - PR #2544 review finding F5 (reviewer da2ce7, ID F1): the "regression from SI-16" framing was wrong. Verified in source (the spawned `graceful_shutdown` task panics on the closed halt channel before `handle.graceful_shutdown`) and by running the drop test on the tree before SI-16 (V4, `evidence/v4-pre-si16-drop-run.txt`): the binding was still taken after 10.02 s. Corrected the Background, the out-of-scope note, V1's conclusion, the EPIC roadmap row 14, the SI-23 and SI-24 drop-path notes, and the GitHub issue body.

## Acceptance Criteria

- [x] AC1: Dropping a running HTTP test environment without `stop()` releases
      its HTTP binding within the test deadline.
- [x] AC2: `stop()` keeps its behavior: it cancels once, joins every owned
      task, and reports failures as today; all existing environment tests pass.
- [x] AC3: The regression test is proven red before the fix and green after.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-axum-http-server`
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                    | Human-oriented command/steps                                                       | Expected Result                              | Status | Evidence                                     |
| --- | --------------------------- | ---------------------------------------------------------------------------------- | -------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Drop a running environment  | Temporary test from V1: start, drop, poll `TcpListener::bind` for up to 10 s       | Before fix: times out. After fix: succeeds.  | DONE | `manual-verification-evidence.md` sections V1 and V3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | V3: the maintained drop-path test passes within the deadline. |
| AC2   | DONE                   | V3: `stop()` unchanged; 7 environment tests, 44 unit and 62 integration tests pass. |
| AC3   | DONE                   | V2 red (10.02 s timeout) before the fix; V3 green after.      |

## Risks and Trade-offs

- **Drop is asynchronous**: the guard only requests cancellation; the binding
  is freed when the server task next runs. The test polls instead of asserting
  at once, as the UDP test does.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory;
  otherwise add a progress-log entry explaining why it was not needed.

## References

- Related issues: #2412 (SI-16), #2448 (SI-17), #2410 (SI-22, T9), #2449, #2450
- Related PRs: #2439, #2459 (finding F1)
- Related ADRs: `docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`
