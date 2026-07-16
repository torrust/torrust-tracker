---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-6-align-grace-periods/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-07-16
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/main.rs
    - src/bootstrap/jobs/manager.rs
    - packages/axum-server/src/signals.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/features/shutdown-process/open-questions.md
---

<!-- skill-link: create-issue -->

# Draft SI-6 — Align `JobManager` Grace Period with Axum Server Timeout

> **EPIC position**: SI-6 of #1488.
> **Blocked**: Q4 must be resolved — the correct target grace period depends
> on the Docker/Kubernetes deployment context decision.

## Goal

Fix the mismatch between the `JobManager`'s per-job timeout (10s) and the Axum
server's graceful shutdown grace period (90s) so that the main process actually
waits long enough for HTTP connections to drain before exiting.

## Background

**Current state** (confirmed experimentally):

- `main.rs` calls `jobs.wait_for_all(Duration::from_secs(10))` — 10 seconds per
  job, sequentially.
- Axum servers (HTTP tracker, REST API, Health Check API) have an internal grace
  period of 90 seconds (`graceful_shutdown(Some(Duration::from_secs(90)))`).

Because the `JobManager` times out after 10s per job, the main process exits
before the Axum 90s drain period completes. The Axum drain task keeps running
as an orphan but the process has already exited — connections are force-dropped.

See [analysis §5.1 and §7.3](../../analysis/20260716-shutdown-process/README.md).

**The tension with Docker**:

Docker's default `stop_grace_period` is 10s. If we raise the `JobManager` timeout
to match the Axum 90s, Docker will SIGKILL the container before the tracker
finishes draining. Operators must configure `stop_grace_period` appropriately.
See Q4 in open-questions.md.

## Implementation

**Step 1**: Reduce the Axum server grace period to a value shorter than the
`JobManager` total timeout. A good target:

```rust
// packages/axum-server/src/signals.rs
let grace_period = Duration::from_secs(25);  // was 90s
let max_wait = Duration::from_secs(30);       // was 95s
```

**Step 2**: Raise the `JobManager` grace period to give enough time for all jobs
(including the Axum drain):

```rust
// src/main.rs
jobs.wait_for_all(Duration::from_secs(30)).await;  // was 10s
```

**Step 3**: Change `wait_for_all` to wait concurrently rather than sequentially
(each job currently gets 10s regardless of what others are doing):

```rust
// src/bootstrap/jobs/manager.rs
pub async fn wait_for_all(mut self, grace_period: Duration) {
    let handles: Vec<_> = self.jobs.drain(..).collect();
    let futures = handles.into_iter().map(|job| {
        let name = job.name.clone();
        async move {
            match timeout(grace_period, job.handle).await {
                Ok(Ok(())) => info!(job = %name, "Job completed gracefully"),
                Ok(Err(e)) => warn!(job = %name, "Job returned an error: {:?}", e),
                Err(_) => warn!(job = %name, "Job did not complete in time"),
            }
        }
    });
    futures::future::join_all(futures).await;
}
```

## Acceptance Criteria

- [ ] Q4 is resolved and the target grace period is agreed.
- [ ] `jobs.wait_for_all()` waits concurrently, not sequentially.
- [ ] The Axum internal grace period is ≤ `JobManager` total timeout.
- [ ] `main.rs` uses the agreed total timeout.
- [ ] When `kill -INT <pid>` is sent, all Axum servers drain their connections
      (log shows "All connections closed") before `main.rs` exits.
- [ ] Deployment documentation (`docs/containers.md`) notes the recommended
      Docker `stop_grace_period` configuration.

## Dependencies

- Q4 (grace period decision) must be resolved.
- Can land after SI-1 or independently; the grace period alignment is correct
  behavior regardless of which signal triggered the shutdown.

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Setup

```bash
cargo build --release
RUST_LOG=info ./target/release/torrust-tracker
```

### Test 1: All Axum servers drain connections before main exits

With the tracker running, open one or more idle HTTP connections (e.g., with
`curl` or `telnet`) and then send `kill -INT <pid>`.

```bash
# Keep a connection alive in background
curl -v --no-progress-meter http://127.0.0.1:7070/health_check &

# Shutdown
kill -INT <pid>
```

**Expected log sequence**:

```text
INFO  graceful_shutdown: !! Shutting down HTTP server ... in 25 seconds !!
INFO  graceful_shutdown: All connections closed, shutting down server in address ...
INFO  Job completed gracefully job=http_instance_0_0.0.0.0:7070
INFO  Torrust tracker successfully shutdown.
```

The key requirement is that `All connections closed` appears **before**
`successfully shutdown` — the main process waited for HTTP drain to complete.

**Record in `verification.md`**: timestamped log output showing the sequence.

### Test 2: Jobs are waited concurrently

Confirm that the shutdown log does **not** show sequential one-by-one waits
but instead shows jobs completing in parallel. Compare timestamps:

```text
# Sequential (WRONG): timestamps are separated by ~10s each
2026-07-16T10:00:00 INFO  Waiting for job ... job=health_check_api
2026-07-16T10:00:10 INFO  Waiting for job ... job=http_api

# Concurrent (CORRECT): timestamps are clustered together
2026-07-16T10:00:00 INFO  Waiting for 9 jobs to finish (timeout: 30s)
2026-07-16T10:00:01 INFO  Job completed gracefully job=health_check_api
2026-07-16T10:00:01 INFO  Job completed gracefully job=http_api
```

**Record in `verification.md`**: log output with timestamps showing concurrent completion.

### Test 3: Docker stop completes without SIGKILL

```bash
docker run -d --name torrust-test torrust/tracker:dev
# Make one HTTP request to ensure a connection is open
curl http://localhost:7070/health_check &
docker stop torrust-test  # default 10s timeout
docker logs torrust-test | tail -5
```

**Expected**: `docker stop` returns before the timeout. The last log line shows
`successfully shutdown`, not a killed/aborted message.

**Note**: If the default 10s is too short, document the observed timing and the
recommended `stop_grace_period` in `verification.md`.
