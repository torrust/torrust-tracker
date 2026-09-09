---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: 1488
github-issue: 2169
spec-path: docs/issues/open/2169-1488-si-4-migrate-torrent-cleanup/ISSUE.md
branch: 2169-migrate-torrent-cleanup
related-pr: 2181
last-updated-utc: 2026-09-09 08:35
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/bootstrap/jobs/torrent_cleanup.rs
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/open/1586-evaluate-job-manager-join-set/ISSUE.md
    - docs/issues/open/1588-review-shutdown-process-for-all-tasks-jobs/verification.md
    - docs/analysis/20260716-shutdown-process/README.md
---

<!-- skill-link: create-issue -->

# Issue #2169 — Migrate Torrent Cleanup to `CancellationToken`

> **Parent EPIC**: #1488 — Overhaul: Tracker Shutdown
>
> **EPIC position**: Roadmap sequence 3. This migration removes only torrent
> cleanup from the periodic-job compatibility registry.

## Goal

Make torrent cleanup a directly supervised, token-aware top-level component.
It must respond to `JobManager::cancel()` without subscribing to an OS signal,
report its explicit normal terminal state, and leave no cleanup task detached.

## Background

The implementation-time task inventory for #1588 confirms that
`torrent_cleanup` is currently one of three pre-spawned periodic jobs retained
in `JobManager::legacy_jobs`. It waits on `tokio::signal::ctrl_c()`, ignores the
manager's root cancellation token, and therefore is deliberately aborted after
the shared ten-second deadline during a SIGTERM shutdown.

Issue #1586 established that direct top-level components must be passed as futures to
`JobManager::spawn`; `JoinSet` cannot adopt an existing `JoinHandle` without a
wrapper task that could detach the actual work. Adding a token while retaining
`start_job() -> JoinHandle<()>` would therefore not complete this migration.

## Scope

### In Scope

- Replace the pre-spawned torrent-cleanup starter with an unspawned,
  token-aware runner.
- Move only `torrent_cleanup` from `JobManager::legacy_jobs` to a direct,
  named `JobManager::spawn` component.
- Preserve the existing configuration gate, interval behavior, cleanup logic,
  and weak `TorrentsManager` lifetime behavior.
- Add deterministic lifecycle tests for token cancellation and direct manager
  ownership.
- Record direct-binary SIGTERM evidence that torrent cleanup completes
  cooperatively rather than reaching the deadline abort.
- Manually verify that the migrated job still removes an inactive peer and
  document the reusable procedure as a testing skill in a separate commit.

### Out of Scope

- Migrating `peers_inactivity_update`; SI-5 owns that separate periodic job.
- Changing the UDP IP-ban cleanup registration or its lifecycle; it remains in
  the temporary compatibility registry.
- Changing torrent cleanup policy, interval configuration, or the underlying
  `TorrentsManager::cleanup_torrents` behavior.
- Changing the shared server lifecycle API, the `Halted` bridge, or server
  drain policies.
- Configuring process deadlines or exit-code policy; SI-20 owns those changes.

## Architectural Decisions

- Related ADR: `docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`
- Related completed supervisor boundary: [issue #1586](../../open/1586-evaluate-job-manager-join-set/ISSUE.md)
- ADRs to create: None known. This applies the existing supervisor boundary.

The cleanup module must return the generic
`torrust_tracker_events::shutdown::Completion`, not the application-specific
`ComponentResult`. `src/app.rs` already adapts `Completion` through
`component_runner`, retaining the separation between reusable cancellation work
and supervisor outcome reporting.

## Design and Ownership Review

**Interface:** Replace `start_job` with:

```rust
pub fn run_job(
  config: Core,
  torrents_manager: Arc<TorrentsManager>,
    cancellation_token: CancellationToken,
) -> impl Future<Output = Completion> + Send + 'static
```

**Ownership invariants:** `JobManager::spawn("torrent_cleanup", ...)` owns the
unspawned runner directly in its `JoinSet`. There is no cleanup `JoinHandle`,
wrapper task, or `register_legacy` call. Token cancellation causes the runner
to log and return `Completion::Cancelled`. Independent weak-manager expiry
causes `Completion::Completed`. If the shared grace deadline expires,
`JobManager` aborts and joins the direct runner, recording named `Aborted`.
Dropping `JobManager` without cancellation aborts its `JoinSet` members and is
not a graceful completion path.

**Normal and failure behavior:** The runner retains its existing immediate first
interval tick and regular cleanup work. It has no expected fallible cleanup
operation; existing torrent-manager behavior tests remain responsible for
cleanup-policy coverage. Startup failure after registration uses the existing
`start_jobs` cancellation-and-wait path, so cleanup must return cooperatively.

**Deadlines:** No runner-local deadline is introduced. `JobManager`'s existing
single shared deadline bounds application shutdown. Deterministic unit tests use
injected cancellation and a long interval; they do not wait for wall-clock
cleanup intervals.

**Design-review checkpoint:** After the focused tests pass, confirm the direct
component is not registered through `legacy_jobs`, and verify that its explicit
completion state reaches the named supervisor outcome.

**Functional regression evidence:** M4 uses a local, isolated configuration
with `remove_peerless_torrents = false`. This preserves the torrent after its
only peer is removed, so the REST API can distinguish an empty peer list from
a missing torrent. It verifies both state transitions and the cleanup job's
observable logs. After this scenario is complete, T6 captures the exact,
reusable procedure in the `manual-torrent-cleanup-e2e` skill; that documentation
commit is deliberately separate from the migration implementation.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                              | Notes / Expected Output                                                                                     |
| --- | ------ | --------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Replace pre-spawned API           | Added unspawned `run_job` accepting `CancellationToken` and returning `Completion`.                         |
| T2  | DONE   | Migrate loop cancellation         | Replaced `ctrl_c()` with `cancellation_token.cancelled()` and preserved interval and weak-manager behavior. |
| T3  | DONE   | Migrate application registration  | Registered `torrent_cleanup` directly with `JobManager::spawn` through `component_runner`.                  |
| T4  | DONE   | Add deterministic lifecycle tests | Added injected-token, weak-manager expiry, and named manager-outcome coverage.                              |
| T5  | DONE   | Manually verify cleanup behavior  | Verified the announced peer is removed while the configured peerless torrent remains.                       |
| T6  | DONE   | Add manual-cleanup test skill     | Added a reusable manual verification procedure under `.github/skills/dev/testing/` in a separate commit.    |
| T7  | DONE   | Validate and record evidence      | Passed root-library tests and `linter all`; recorded direct-binary SIGTERM and cleanup evidence.            |

## Commit Points

| Task  | Coherent change set                                             | Commit policy                                                              |
| ----- | --------------------------------------------------------------- | -------------------------------------------------------------------------- |
| T1–T3 | Direct, token-aware cleanup runner and application registration | Commit after focused validation and complexity review.                     |
| T4    | Runner and supervisor-outcome lifecycle tests                   | Commit after focused test-design review and validation.                    |
| T5    | Manual inactive-peer cleanup evidence                           | Record in issue-local `verification.md`; no empty commit.                  |
| T6    | Reusable manual-cleanup verification skill                      | Commit separately as `docs(skills): add manual torrent cleanup e2e skill`. |
| T7    | Final verification evidence and issue progress                  | Commit separately when it improves reviewability.                          |

All commits use a narrow Conventional Commit scope and GPG signing.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec reviewed and approved by user/maintainer
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-08 10:31 UTC — Copilot — Revised the existing SI-4 draft using #1586 direct-supervisor and #1588 inventory evidence. — [#1586](../../open/1586-evaluate-job-manager-join-set/ISSUE.md), [#1588](../../open/1588-review-shutdown-process-for-all-tasks-jobs/verification.md)
- 2026-09-08 11:15 UTC — Jose Celano — Approved the revised specification; GitHub issue #2169 created and linked as an EPIC #1488 sub-issue. — https://github.com/torrust/torrust-tracker/issues/2169
- 2026-09-08 16:26 UTC — Copilot — Migrated torrent cleanup to direct token-aware supervision; deterministic tests, isolated inactive-peer cleanup, and direct-binary SIGTERM verification passed. — `verification.md`
- 2026-09-09 08:22 UTC — Copilot — Completion review: no material design change and no invalidated assumption, so no `implementation-retrospective.md` is needed; the only discovery was the `metadata.purpose = "configuration"` schema requirement for the isolated config. Independent Task Reviewer approved AC1–AC9; its low-severity documentation findings were applied. — `agent-review-reports.md`

## Acceptance Criteria

- [x] AC1: `torrent_cleanup.rs` contains no `tokio::signal::ctrl_c()` call.
- [x] AC2: Torrent cleanup accepts an injected `CancellationToken` and returns
      `Completion::Cancelled` after cancellation.
- [x] AC3: Weak `TorrentsManager` expiry returns `Completion::Completed`.
- [x] AC4: With cleanup enabled, `src/app.rs` registers one named direct
      `JoinSet` component, `torrent_cleanup`, rather than a legacy handle.
- [x] AC5: `JobManager::cancel()` produces the named
      `torrent_cleanup: JobStatus::Cancelled` outcome without an OS signal.
- [x] AC6: Direct-binary SIGTERM evidence shows torrent cleanup's cancellation
      log and cooperative manager outcome, not its deadline-abort outcome.
- [x] AC7: SI-5's `peers_inactivity_update` remains outside this scope and may
      remain deadline-aborted in the same scenario.
- [x] AC8: A local tracker run proves torrent cleanup removes an inactive peer;
      REST API observations and cleanup log evidence are recorded.
- [x] AC9: A reusable manual torrent-cleanup verification skill is added under
      `.github/skills/dev/testing/` in a commit separate from the migration.
- [x] `linter all` exits with code `0`.
- [x] Relevant tests pass.
- [x] Manual verification scenarios are executed and documented.
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation is updated when behavior changes.

## Verification Plan

Define verification before implementation and execute it before closing the
issue. Record results in `verification.md` in this folder.

### Automatic Checks

- `cargo test -p torrust-tracker --lib bootstrap::jobs::torrent_cleanup::tests`
- Focused application/manager registration and outcome test
- `linter all`
- `git diff --check`
- Pre-push checks before publication
- `cargo test -p torrust-tracker --lib bootstrap::jobs::torrent_cleanup::tests::it_should_return_cancelled_when_the_token_is_cancelled`
- `cargo test -p torrust-tracker --lib app::tests::it_should_register_torrent_cleanup_as_a_direct_cancelled_component`

The T4 tests introduce the named tests above. The first uses a long cleanup
interval and injected token; the second cancels `JobManager`, awaits its
outcomes, and asserts `torrent_cleanup: JobStatus::Cancelled`.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                           | Command / Steps                                                                                                                                                                                                                                                    | Expected Result                                                                                                                                                     | Status | Evidence                                                                           |
| --- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ---------------------------------------------------------------------------------- |
| M1  | No direct OS-signal dependency     | Search `src/bootstrap/jobs/torrent_cleanup.rs` for `ctrl_c`.                                                                                                                                                                                                       | No matches.                                                                                                                                                         | DONE   | `verification.md`                                                                  |
| M2  | Direct-binary SIGTERM cancellation | Build and run `./target/debug/torrust-tracker`; wait for `Tracker shutdown signal handlers installed.`; verify its direct PID; send `SIGTERM`.                                                                                                                     | Torrent cleanup logs cancellation and manager reports it as cooperatively cancelled, not deadline-aborted.                                                          | DONE   | `verification.md`                                                                  |
| M3  | Remaining migration boundary       | The isolated run disables `peers_inactivity_update`; inspect its `register_legacy` call in `src/app.rs` instead.                                                                                                                                                   | `peers_inactivity_update` may remain deadline-aborted while SI-5 is pending; no conclusion about SI-5 completion.                                                   | DONE   | `verification.md`; boundary verified by its `register_legacy` call in `src/app.rs` |
| M4  | Inactive-peer cleanup regression   | Run an isolated local tracker with `inactive_peer_cleanup_interval = 1`, `max_peer_timeout = 1`, and `remove_peerless_torrents = false`; announce a fixed info hash; read it through `GET /api/v1/torrent/{info_hash}`; wait at least five seconds; read it again. | First response contains the announced peer; second response keeps the torrent but has an empty `peers` array. Logs show the cleanup run and post-cleanup `peers=0`. | DONE   | `verification.md`; reusable skill created by T6                                    |

## Risks and Trade-offs

- **Incorrect ownership migration:** retaining a spawned `JoinHandle` would
  require a wrapper to enter `JoinSet`, which could detach cleanup work.
  **Mitigation:** use an unspawned runner and direct `JobManager::spawn`.
- **False graceful-shutdown evidence:** a process could exit after the manager
  aborts cleanup at its deadline. **Mitigation:** assert both the cleanup
  cancellation log and named `Cancelled` outcome; explicitly distinguish SI-5's
  expected abort.
- **Interval-test delays:** waiting for periodic work makes tests slow and
  flaky. **Mitigation:** inject a token, use a long interval, and cancel before
  awaiting the runner.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: No material design change occurred. The only runtime discovery was that the isolated configuration metadata requires `purpose = "configuration"`; the implementation itself followed the approved direct-supervisor design.
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue folder.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1488
- Related completed work: #1586, #1588, #2132
- Related analysis: `docs/analysis/20260716-shutdown-process/README.md`
