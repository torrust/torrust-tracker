---
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 1488
github-issue: 2221
spec-path: docs/issues/closed/2221-1488-si-5-migrate-activity-metrics-updater/ISSUE.md
branch: 2221-migrate-activity-metrics-updater
related-pr: 2224
last-updated-utc: 2026-09-15 11:50
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs
    - packages/swarm-coordination-registry/Cargo.toml
    - src/bootstrap/jobs/activity_metrics_updater.rs
    - src/bootstrap/jobs/torrent_cleanup.rs
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
    - src/AGENTS.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/closed/2169-1488-si-4-migrate-torrent-cleanup/ISSUE.md
    - docs/issues/closed/1586-evaluate-job-manager-join-set/ISSUE.md
    - docs/issues/closed/1588-review-shutdown-process-for-all-tasks-jobs/verification.md
    - docs/analysis/20260716-shutdown-process/README.md
---

<!-- skill-link: create-issue -->

# Issue #2221 — Migrate Activity Metrics Updater to `CancellationToken`

> **Parent EPIC**: #1488 — Overhaul: Tracker Shutdown
>
> **EPIC position**: Roadmap sequence 4 (draft SI-5). This migration removes
> only `peers_inactivity_update` from the periodic-job compatibility registry.

## Goal

Make the peers activity metrics updater a directly supervised, token-aware
top-level component. It must respond to `JobManager::cancel()` without
subscribing to an OS signal, report its explicit normal terminal state, and
leave no updater task detached.

## Background

The implementation-time task inventory for #1588 confirms that
`peers_inactivity_update` is one of the pre-spawned periodic jobs retained in
`JobManager::legacy_jobs`. Its loop in
`packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs`
waits on `tokio::signal::ctrl_c()`, ignores the manager's root cancellation
token, and is therefore deliberately aborted after the shared ten-second
deadline during a SIGTERM shutdown. The #2169 SIGTERM evidence explicitly left
this job outside its claim.

Issue #1586 established that direct top-level components must be passed as
futures to `JobManager::spawn`; `JoinSet` cannot adopt an existing
`JoinHandle` without a wrapper task that could detach the actual work. Adding a
token while retaining `start_job() -> JoinHandle<()>` would therefore not
complete this migration.

Issue #2169 (SI-4) migrated torrent cleanup with exactly this shape:
`run_job(..., CancellationToken) -> impl Future<Output = Completion>`, adapted in
`src/app.rs` through `component_runner`. This task applies the same, already
reviewed pattern to the second periodic job.

## Scope

### In Scope

- Replace the pre-spawned updater starter in the
  `swarm-coordination-registry` package with an unspawned, token-aware runner
  that returns `torrust_tracker_events::shutdown::Completion`.
- Update the `src/bootstrap/jobs/activity_metrics_updater.rs` wrapper to
  return the unspawned runner.
- Move only `peers_inactivity_update` from `JobManager::legacy_jobs` to a
  direct, named `JobManager::spawn` component in `src/app.rs`.
- Preserve the existing `tracker_usage_statistics` gate, the hardcoded 15s
  interval, the immediate first tick, the metrics update logic, and the
  weak-pointer lifetime behavior for the registry and statistics repository.
- Remove the `signal` feature from the package's `tokio` dependency if no other
  code in the package uses it.
- Add deterministic lifecycle tests for token cancellation, weak-pointer
  expiry, and direct manager ownership.
- Record direct-binary SIGTERM evidence that the updater completes
  cooperatively rather than reaching the deadline abort.
- Manually verify that activity metrics are still updated during normal
  operation after the migration.

### Out of Scope

- Making the 15s interval configurable (existing `todo:` in the code).
- Changing how the inactivity cutoff is computed. The bootstrap wrapper
  computes `peer_inactivity_cutoff_timestamp` once at startup, so the gauges
  never count peers that announced after startup as inactive. This is a
  pre-existing bug that will be filed as a separate issue after this task
  merges; it is not changed here.
- Changing the UDP IP-ban cleanup registration or its lifecycle; it remains in
  the temporary compatibility registry.
- Changing the shared server lifecycle API, the `Halted` bridge, or server
  drain policies (SI-2, SI-10 onward).
- Configuring process deadlines or exit-code policy; SI-20 owns those changes.
- Removing `JobManager::register_legacy` itself; it still serves
  `udp_ban_cleanup`.

## Architectural Decisions

- Related ADR: `docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`
- Related completed supervisor boundary: [issue #1586](../../closed/1586-evaluate-job-manager-join-set/ISSUE.md)
- Reference migration: [issue #2169](../../closed/2169-1488-si-4-migrate-torrent-cleanup/ISSUE.md)
- ADRs to create: None known. This applies the existing supervisor boundary.

The package module must return the generic
`torrust_tracker_events::shutdown::Completion` (the package already depends on
`torrust-tracker-events`), not the application-specific `ComponentResult`.
`src/app.rs` adapts `Completion` through `component_runner`, keeping reusable
cancellation work separate from supervisor outcome reporting.

## Design and Ownership Review

**Interface:** Replace `start_job` in the package with:

```rust
pub fn run_job(
    swarms: Arc<Registry>,
    stats_repository: Arc<Repository>,
    inactivity_cutoff: DurationSinceUnixEpoch,
    cancellation_token: CancellationToken,
) -> impl Future<Output = Completion> + Send + 'static
```

The runner downgrades both `Arc`s to weak pointers and drops the strong
references before looping, as the current implementation does. The bootstrap
wrapper `src/bootstrap/jobs/activity_metrics_updater.rs` exposes a matching
`run_job(config, app_container, cancellation_token)` that returns the unspawned
runner.

**Ownership invariants:** `JobManager::spawn("peers_inactivity_update", ...)`
owns the unspawned runner directly in its `JoinSet`. There is no updater
`JoinHandle`, wrapper task, or `register_legacy` call. Token cancellation
causes the runner to log and return `Completion::Cancelled`. Independent
weak-pointer expiry of either the registry or the statistics repository causes
`Completion::Completed`. If the shared grace deadline expires, `JobManager`
aborts and joins the direct runner, recording named `Aborted`. Dropping
`JobManager` without cancellation aborts its `JoinSet` members and is not a
graceful completion path.

**Normal and failure behavior:** The runner retains its immediate first
interval tick and the regular `update_activity_metrics` work. The `select!`
uses `biased;` with the cancellation branch first, matching torrent cleanup,
so a cancelled token is observed before any further tick. The metrics update
has no fallible operation that changes the completion state; the existing
`set_gauge` results remain ignored as today.

**Deadlines:** No runner-local deadline is introduced. `JobManager`'s single
shared deadline bounds application shutdown. Deterministic unit tests inject
cancellation before awaiting the runner and, for the weak-pointer case, use
`#[tokio::test(start_paused = true)]` with `tokio::time::advance` so the
hardcoded 15s interval never delays a test.

**Design-review checkpoint:** After the focused tests pass, confirm that
`src/app.rs` no longer calls `register_legacy` for `peers_inactivity_update`,
and verify that its explicit completion state reaches the named supervisor
outcome.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                 | Notes / Expected Output                                                                                                                                     |
| --- | ------ | ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Replace pre-spawned package API      | Added unspawned `run_job` accepting `CancellationToken` and returning `Completion`; removed `start_job` and the `ctrl_c()` branch.                           |
| T2  | DONE   | Drop unused `tokio` `signal` feature | Removed `signal` from normal Tokio features; added test-only `test-util` for deterministic paused-time coverage.                                            |
| T3  | DONE   | Migrate bootstrap wrapper            | `src/bootstrap/jobs/activity_metrics_updater.rs` returns the unspawned runner and forwards the token.                                                       |
| T4  | DONE   | Migrate application registration     | `start_peers_inactivity_update` registers `peers_inactivity_update` through `JobManager::spawn` and `component_runner`; no `register_legacy` call.          |
| T5  | DONE   | Add deterministic lifecycle tests    | Added injected-token cancellation, weak-collaborator expiry, and named manager-outcome coverage.                                                            |
| T6  | DONE   | Update architecture docs             | Updated `src/AGENTS.md`; only `udp_ban_cleanup` is now documented as legacy.                                                                                |
| T7  | DONE   | Validate and record evidence         | Focused tests, `linter all`, workspace documentation tests, and pre-push checks passed; manual evidence is recorded.                                        |

## Commit Points

| Task  | Coherent change set                                             | Commit policy                                                                    |
| ----- | --------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| T1–T4 | Direct, token-aware updater runner and application registration | Commit after focused validation and complexity review.                           |
| T2    | `tokio` feature trim                                            | Fold into the T1–T4 commit if trivially safe; otherwise separate `chore` commit. |
| T5    | Runner and supervisor-outcome lifecycle tests                   | Commit after focused test-design review and validation.                          |
| T6    | Architecture documentation update                               | Commit separately as `docs(app): ...` when it improves reviewability.            |
| T7    | Final verification evidence and issue progress                  | Commit separately; no empty commit for evidence-only decisions.                  |

All commits use a narrow Conventional Commit scope and GPG signing.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1488-si-5-migrate-activity-metrics-updater/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR deliberately skipped: maintainer agreed spec and implementation share one branch because the change follows the reviewed #2169 pattern
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-15 09:04 UTC — Copilot — Rewrote the SI-5 draft to the current issue template using the #2169 (SI-4) migration as the reference design; awaiting maintainer review. — [#2169](../../closed/2169-1488-si-4-migrate-torrent-cleanup/ISSUE.md)
- 2026-09-15 09:20 UTC — Jose Celano — Approved the draft; decided on a single spec-plus-implementation branch and deferred the stale-cutoff bug to a separate follow-up issue.
- 2026-09-15 09:30 UTC — Copilot — Created GitHub issue #2221, linked it as an EPIC #1488 sub-issue, and moved the spec to `docs/issues/open/`. — https://github.com/torrust/torrust-tracker/issues/2221
- 2026-09-15 09:49 UTC — Copilot — Migrated the updater to direct token-aware supervision; focused deterministic tests and direct-binary SIGTERM plus two-interval metrics evidence passed. — `manual-verification-evidence.md`
- 2026-09-15 09:57 UTC — Task Reviewer — Approved implementation behavior and identified documentation-completion gaps; findings resolved below. — `agent-review-reports.md`
- 2026-09-15 10:00 UTC — Copilot — Completion review: no material design change or invalidated assumption occurred. The only implementation discovery was that deterministic paused Tokio time requires the `test-util` feature, now confined to package dev-dependencies; this is routine test infrastructure, not a reusable architectural lesson, so no separate retrospective is needed.
- 2026-09-15 10:04 UTC — Copilot — Final verification passed: `linter all`, workspace documentation tests, focused lifecycle tests, and all pre-push checks. — `.tmp/pre-push-*.log`
- 2026-09-15 11:50 UTC — Repository maintenance — PR #2224 merged and GitHub issue #2221 closed; archived this specification in `docs/issues/closed/2221-1488-si-5-migrate-activity-metrics-updater/`.

## Acceptance Criteria

- [x] AC1: `activity_metrics_updater.rs` in the `swarm-coordination-registry`
      package contains no `tokio::signal::ctrl_c()` call.
- [x] AC2: The updater accepts an injected `CancellationToken` and returns
      `Completion::Cancelled` after cancellation.
- [x] AC3: Weak-pointer expiry of the registry or statistics repository returns
      `Completion::Completed`.
- [x] AC4: With `tracker_usage_statistics = true`, `src/app.rs` registers one
      named direct `JoinSet` component, `peers_inactivity_update`, rather than
      a legacy handle.
- [x] AC5: `JobManager::cancel()` produces the named
      `peers_inactivity_update: JobStatus::Cancelled` outcome without an OS
      signal.
- [x] AC6: Direct-binary SIGTERM evidence shows the updater's cancellation log
      and cooperative manager outcome, not its deadline-abort outcome.
- [x] AC7: Activity metrics are still updated during normal operation
      (`Updating peers and torrents activity metrics ...` debug logs across at
      least two intervals and non-error gauge updates).
- [x] AC8: `udp_ban_cleanup` remains the only `register_legacy` registration
      and is not changed by this task.
- [x] `linter all` exits with code `0`.
- [x] Relevant tests pass.
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation is updated when behavior/workflow changes (`src/AGENTS.md`).

## Verification Plan

Define verification before implementation and execute it before closing the
issue. Record results in `manual-verification-evidence.md` in this folder.

### Automatic Checks

- `cargo test -p torrust-tracker-swarm-coordination-registry --lib statistics::activity_metrics_updater::tests`
- `cargo test -p torrust-tracker --lib app::tests::it_should_register_peers_inactivity_update_as_a_direct_cancelled_component`
- `cargo test --doc --workspace`
- `cargo machete`
- `linter all`
- `git diff --check`
- Pre-push checks before publication

T5 introduces the named tests above. The package test cancels an injected token
and awaits the unspawned runner, observing `Completion::Cancelled`; a paused-time
sibling drops the strong `Arc`s and advances past one interval, observing
`Completion::Completed`. The application test builds a `JobManager`, calls
`start_peers_inactivity_update`, cancels, awaits outcomes, and asserts
`peers_inactivity_update: JobStatus::Cancelled`.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                           | Human-oriented command/steps                                                                                                                                                                                      | Expected Result                                                                                                                                                     | Status | Evidence                                     |
| --- | ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | No direct OS-signal dependency     | `rg 'ctrl_c' packages/swarm-coordination-registry/`                                                                                                                                                               | No matches.                                                                                                                                                         | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Direct-binary SIGTERM cancellation | Build; run `./target/debug/torrust-tracker` with an isolated config where `tracker_usage_statistics = true`; wait for `Tracker shutdown signal handlers installed.`; confirm the direct PID; `kill -TERM <pid>`. | Log shows `Stopping peers activity metrics update job ...` and `Job completed after cooperative cancellation job=peers_inactivity_update`; no abort outcome for it. | DONE   | `manual-verification-evidence.md` section V2 |
| M3  | Metrics still updated              | Run the same binary with `RUST_LOG=debug` for at least 35 seconds (two 15s intervals) before stopping.                                                                                                            | At least two `Updating peers and torrents activity metrics (executed every 15 secs) ...` entries with matching `updated in ... ms` lines.                           | DONE   | `manual-verification-evidence.md` section V3 |
| M4  | Remaining migration boundary       | Inspect `src/app.rs` for `register_legacy` calls.                                                                                                                                                                 | Only `udp_ban_cleanup` remains; it may still be deadline-aborted in M2 and that is expected.                                                                        | DONE   | `manual-verification-evidence.md` section V4 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Create `manual-verification-evidence.md` from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these
  scenarios. Record actual prerequisites, actions, commands, program output,
  relevant tracker logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The scenarios above are short interactive steps; the reusable
SIGTERM procedure from #2169 applies unchanged.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | Package source search in `manual-verification-evidence.md` V1 |
| AC2   | DONE                   | Package injected-token test |
| AC3   | DONE                   | Package paused-time weak-collaborator test |
| AC4   | DONE                   | Application wiring and `manual-verification-evidence.md` V4 |
| AC5   | DONE                   | Application named-outcome test |
| AC6   | DONE                   | `manual-verification-evidence.md` V2 |
| AC7   | DONE                   | `manual-verification-evidence.md` V3 |
| AC8   | DONE                   | `manual-verification-evidence.md` V4 |

## Risks and Trade-offs

- **Incorrect ownership migration:** retaining a spawned `JoinHandle` would
  require a wrapper to enter `JoinSet`, which could detach updater work.
  **Mitigation:** use an unspawned runner and direct `JobManager::spawn`, as in
  #2169.
- **False graceful-shutdown evidence:** the process could exit after the
  manager aborts the updater at its deadline. **Mitigation:** assert both the
  updater cancellation log and the named `Cancelled` outcome; explicitly
  distinguish the expected `udp_ban_cleanup` state.
- **Interval-test delays:** the 15s interval is hardcoded, so tests cannot
  inject a long interval. **Mitigation:** cancel before awaiting the runner,
  and use paused tokio time for the weak-pointer case.
- **Feature trim regression:** removing `tokio`'s `signal` feature could break
  a use elsewhere in the package. **Mitigation:** search the package before
  removal; a compile failure is immediate and local.
- **Public API break for external users of `start_job`:** the package is
  workspace-local at version `0.1.0` and unpublished; the only consumer is the
  tracker bootstrap wrapper, which is migrated in the same change.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: No separate retrospective is needed. The token-aware direct-supervision design followed the approved #2169 pattern without material deviation. Tokio's test-only `test-util` feature was the only discovery; it is confined to dev-dependencies and does not establish a reusable architecture decision.
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue folder.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1488
- Reference migration: #2169 (SI-4)
- Related completed work: #1586, #1588, #2132
- Related analysis: `docs/analysis/20260716-shutdown-process/README.md`
