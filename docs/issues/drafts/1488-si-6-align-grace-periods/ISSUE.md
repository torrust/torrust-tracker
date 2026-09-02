---
doc-type: issue
issue-type: task
status: superseded
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-6-align-grace-periods/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/main.rs
    - src/bootstrap/jobs/manager.rs
    - packages/axum-server/src/signals.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
---

<!-- skill-link: create-issue -->

# Superseded Draft SI-6 — Use Issue #1586 for Supervisor Ownership

> **Status**: Superseded for implementation planning. Existing issue
> [#1586](../../open/1586-evaluate-job-manager-join-set/ISSUE.md) replaces this
> draft because it requires direct `JoinSet` ownership rather than wrapping
> already-spawned handles.

## Why This Draft Is Superseded

Issue #1586 requires `JobManager` to evaluate direct task ownership through
`JoinSet` (or an explicitly justified alternative). This draft instead retains
a `Vec<Job>` of already-spawned handles, which can require wrapper tasks solely
for registration. Do not implement this draft; use the local issue #1586 spec.

## Original Goal

## Goal

Replace sequential per-job waits with concurrent waiting for existing direct
handles and structured, named outcomes. This issue does not change server
cancellation APIs, component child ownership, numeric deployment policy, or
exit codes; those follow in independent work items.

## Background

**Current state** (confirmed experimentally):

- `main.rs` calls `jobs.wait_for_all(Duration::from_secs(10))` — 10 seconds per
  job, sequentially.
- Axum servers (HTTP tracker, REST API, Health Check API) have an internal grace
  period of 90 seconds (`graceful_shutdown(Some(Duration::from_secs(90)))`).

Because the `JobManager` times out after 10s per job, the main process exits
before the Axum 90s drain period completes. The Axum drain task keeps running
as an orphan but the process has already exited — connections are force-dropped.

See [analysis §5.1 and §7.3](../../../analysis/20260716-shutdown-process/README.md).

**The tension with Docker**:

Docker's default `stop_grace_period` is 10s. If we raise the `JobManager` timeout
to match the Axum 90s, Docker will SIGKILL the container before the tracker
finishes draining. Operators must configure `stop_grace_period` appropriately.
See Q4 in questions.md.

## Implementation

Change `wait_for_all` to wait concurrently and return named outcomes. Use an
existing temporary overall deadline until Q4 specifies configurable final policy:

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

- [ ] `jobs.wait_for_all()` waits concurrently, not sequentially.
- [ ] Each registered job has a named structured outcome: completed, failed,
      timed out, or deliberately aborted.
- [ ] The implementation preserves current job registration and server APIs.
- [ ] Unit tests exercise completion, failure, and timeout without OS signals.
- [ ] The temporary deadline and its limitations are documented for Q4.

## Dependencies

- No hard prerequisite. It is additive and may precede component migrations.
- Q4 supplies the final deadline hierarchy and configuration after this outcome
  foundation exists.

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Setup

```bash
cargo build --release
RUST_LOG=info ./target/release/torrust-tracker
```

### Test 1: Jobs are waited concurrently

Confirm that the shutdown log does **not** show sequential one-by-one waits
but instead shows jobs completing in parallel. Compare timestamps:

```text
# Sequential (WRONG): timestamps are separated by ~10s each
2026-07-16T10:00:00 INFO  Waiting for job ... job=health_check_api
2026-07-16T10:00:10 INFO  Waiting for job ... job=http_api

# Concurrent (CORRECT): timestamps are clustered together
2026-07-16T10:00:00 INFO  Waiting for 9 jobs to finish (temporary overall deadline)
2026-07-16T10:00:01 INFO  Job completed gracefully job=health_check_api
2026-07-16T10:00:01 INFO  Job completed gracefully job=http_api
```

**Record in `verification.md`**: log output with timestamps showing concurrent completion.

### Test 2: Structured outcomes are named

Use deterministic completed, failed, and blocked tasks. Confirm the supervisor
returns the corresponding named outcomes. SI-20 owns the approved 25s/20s/5s
budgets and configured Docker/Podman validation.
