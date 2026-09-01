---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-4-migrate-torrent-cleanup/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
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

> **EPIC position**: Roadmap step 3. One independently releasable periodic
> component migration after the supervisor token convention is documented.

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

See [analysis §3.2 and §7.2](../../../analysis/20260716-shutdown-process/README.md).

## Implementation

Pass a component `CancellationToken` into `start_job` and use it in the loop.
The job is a named top-level component: it reports completion to `JobManager`
only after its owned loop has stopped. It does not subscribe to OS signals.

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
- [ ] The job receives a component `CancellationToken` as a parameter.
- [ ] `src/app.rs` passes a token derived from the `JobManager` root token when
      starting the job.
- [ ] Unit tests cancel an injected token and await job completion without
      delivering an OS signal.
- [ ] `cargo test` passes.
- [ ] `linter all` passes.

## Dependencies

- The shared component-token convention must be established first.
- SI-1 allows end-to-end SIGTERM verification after this migration.

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
