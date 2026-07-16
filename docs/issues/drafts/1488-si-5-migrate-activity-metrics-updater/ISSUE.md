---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-5-migrate-activity-metrics-updater/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-07-16
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs
    - src/bootstrap/jobs/activity_metrics_updater.rs
    - src/bootstrap/jobs/manager.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/analysis/20260716-shutdown-process/README.md
---

<!-- skill-link: create-issue -->

# Draft SI-5 — Migrate Activity Metrics Updater to `CancellationToken`

> **EPIC position**: SI-5 of #1488. Independent — no blockers.

## Goal

Replace the direct `tokio::signal::ctrl_c()` listener in the peers activity
metrics updater with the shared `CancellationToken` from `JobManager`. This makes
the job respond to `jobs.cancel()` and removes a direct signal dependency.

## Background

`packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs`
currently uses:

```rust
tokio::select! {
    _ = tokio::signal::ctrl_c() => {
        tracing::info!("Stopping peers activity metrics update job (ctrl-c signal received) ...");
        break;
    }
    _ = interval.tick() => { ... }
}
```

This job does **not** respond to `jobs.cancel()`. The interval is also hardcoded
at 15 seconds (a separate known issue noted in the code with a `todo:`).

See [analysis §3.2 and §7.2](../../analysis/20260716-shutdown-process/README.md).

## Implementation

The `start_job` function in the `swarm-coordination-registry` package currently
has this signature:

```rust
pub fn start_job(
    swarms: &Arc<Registry>,
    stats_repository: &Arc<Repository>,
    inactivity_cutoff: DurationSinceUnixEpoch,
) -> JoinHandle<()>
```

Add a `CancellationToken` parameter:

```rust
pub fn start_job(
    swarms: &Arc<Registry>,
    stats_repository: &Arc<Repository>,
    inactivity_cutoff: DurationSinceUnixEpoch,
    cancellation_token: CancellationToken,  // new parameter
) -> JoinHandle<()>
```

In the loop, replace `ctrl_c()` with:

```rust
tokio::select! {
    _ = cancellation_token.cancelled() => {
        tracing::info!("Stopping peers activity metrics update job ...");
        break;
    }
    _ = interval.tick() => { ... }
}
```

Update the call site in `src/bootstrap/jobs/activity_metrics_updater.rs` to
pass `job_manager.new_cancellation_token()`.

## Acceptance Criteria

- [ ] The `ctrl_c()` call is removed from `activity_metrics_updater.rs`.
- [ ] The job stops when `jobs.cancel()` is called.
- [ ] The `CancellationToken` is passed from `JobManager` through
      `src/bootstrap/jobs/activity_metrics_updater.rs`.
- [ ] `cargo test` passes.
- [ ] `linter all` passes.

## Dependencies

- No hard prerequisites. Can land independently.
- Benefits from SI-1 being landed first (SIGTERM will then also stop this job).
- Note: the hardcoded 15s interval has a `TODO` comment — that is a separate
  concern and not in scope for this sub-issue.

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Setup

```bash
cargo build --release
RUST_LOG=info ./target/release/torrust-tracker
```

### Test 1: No direct `ctrl_c()` call in source

```bash
grep -rn 'ctrl_c' packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs
```

**Expected**: no matches.

### Test 2: Activity metrics updater stops on graceful shutdown

Send Ctrl+C (or `kill -INT <pid>`) to the tracker. Look for the updater's stop
message in the logs.

**Expected log**:

```text
INFO  Stopping peers activity metrics update job ...
```

If this message does not appear, the job was aborted rather than cancelled.

**Record in `verification.md`**: the log line confirming graceful stop.

### Test 3: Activity metrics updater stops on SIGTERM (requires SI-1)

If SI-1 has been merged, send `kill <pid>` (SIGTERM).

**Expected**: same stop message appears.

### Test 4: Activity metrics are still collected during normal operation

Run the tracker for at least 30 seconds (two 15s intervals) and confirm that
metrics update log messages appear:

```bash
RUST_LOG=debug ./target/release/torrust-tracker 2>&1 | grep 'activity_metrics'
```

Verify that activity metrics are still being computed and logged before shutdown.
