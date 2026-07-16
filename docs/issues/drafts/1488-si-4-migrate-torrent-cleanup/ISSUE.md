---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-4-migrate-torrent-cleanup/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-07-16
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/bootstrap/jobs/torrent_cleanup.rs
    - src/bootstrap/jobs/manager.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/analysis/20260716-shutdown-process/README.md
---

<!-- skill-link: create-issue -->

# Draft SI-4 — Migrate Torrent Cleanup Job to `CancellationToken`

> **EPIC position**: SI-4 of #1488. Independent — no blockers.

## Goal

Replace the direct `tokio::signal::ctrl_c()` listener in the torrent cleanup job
with the shared `CancellationToken` from `JobManager`. This makes the job respond
to `jobs.cancel()` and removes a direct signal dependency that bypasses the
centralized shutdown coordinator.

## Background

`src/bootstrap/jobs/torrent_cleanup.rs` currently uses:

```rust
tokio::select! {
    _ = tokio::signal::ctrl_c() => {
        tracing::info!("Stopping torrent cleanup job ...");
        break;
    }
    _ = interval.tick() => { ... }
}
```

This job does **not** respond to `jobs.cancel()` — it only stops when Ctrl+C is
pressed. After SI-1 adds `SIGTERM` to `main.rs`, this job will still not stop
on SIGTERM because it listens for Ctrl+C directly.

See [analysis §3.2 and §7.2](../../analysis/20260716-shutdown-process/README.md).

## Implementation

Pass the `CancellationToken` into `start_job` and use it in the loop:

```rust
pub fn start_job(
    config: &Core,
    torrents_manager: &Arc<TorrentsManager>,
    cancellation_token: CancellationToken,  // new parameter
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    tracing::info!("Stopping torrent cleanup job ...");
                    break;
                }
                _ = interval.tick() => { ... }
            }
        }
    })
}
```

In `src/app.rs`, pass `job_manager.new_cancellation_token()` when calling
`start_torrent_cleanup`.

## Acceptance Criteria

- [ ] The `ctrl_c()` call is removed from `torrent_cleanup.rs`.
- [ ] The job stops when `jobs.cancel()` is called (i.e., responds to SIGTERM
      after SI-1, not just SIGINT).
- [ ] The job receives the `CancellationToken` as a parameter.
- [ ] `src/app.rs` passes the token when starting the job.
- [ ] `cargo test` passes.
- [ ] `linter all` passes.

## Dependencies

- No hard prerequisites. Can land independently.
- Benefits from SI-1 being landed first (SIGTERM will then also stop this job).

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
grep -rn 'ctrl_c' src/bootstrap/jobs/torrent_cleanup.rs
```

**Expected**: no matches.

### Test 2: Torrent cleanup job stops on graceful shutdown

Send `kill -INT <pid>` (or Ctrl+C if SI-1 is not yet landed) to the tracker.

**Expected log** (in RUST_LOG=info output):

```text
INFO  Stopping torrent cleanup job ...
```

This message confirms the job's cancellation path ran. If it is absent, the job
was aborted rather than cancelled.

**Record in `verification.md`**: the log line showing the torrent cleanup job
stopped gracefully.

### Test 3: Torrent cleanup stops on SIGTERM (requires SI-1)

If SI-1 has been merged, send `kill <pid>` (SIGTERM).

**Expected**: same `Stopping torrent cleanup job ...` message appears.

### Test 4: `ctrl_c()` removal does not break other jobs

After Ctrl+C, confirm all other jobs also shut down as expected (no regressions).
Verify that the `JobManager` reports all jobs completing gracefully.
