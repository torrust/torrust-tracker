---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1488-si-1-add-sigterm-to-main/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-07-16
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/main.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/open-questions.md
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/research/20260716-console-shutdown-patterns/README.md
---

<!-- skill-link: create-issue -->

# Draft SI-1 — Add `SIGTERM` Handler to `main.rs`

> **EPIC position**: SI-1 of #1488. Highest priority. No prerequisites.

## Goal

Handle `SIGTERM` in `src/main.rs` alongside the existing `SIGINT` handler so that
`kill <pid>`, `docker stop`, `systemctl stop`, and Kubernetes pod termination all
trigger the same graceful shutdown as Ctrl+C.

This is the single most impactful change in the EPIC: a few lines of code that fix
the tracker's compliance with the Unix, Docker, Kubernetes, and systemd process
lifecycle contracts.

## Background

`main.rs` currently only listens for `SIGINT` (Ctrl+C) via
`tokio::signal::ctrl_c()`. `SIGTERM` (signal 15) — sent by default by `kill`,
`docker stop`, `systemctl stop`, and Kubernetes — is silently ignored by `main.rs`.

This was confirmed experimentally on 2026-07-16 (see Phase 1 evidence in
[verification.md](./verification.md)).

**Exact behaviour observed (commit 49d8117f)**:

- `kill <pid>` sent SIGTERM to the binary (PID 955797).
- Each server's internal `global_shutdown_signal()` **did** catch SIGTERM and
  began draining its own connections — this is the per-server signal handler
  inside `torrust_server_lib::signals`, **not** `main.rs`.
- `main.rs`'s `tokio::select!` did **not** fire. `jobs.cancel()` and
  `jobs.wait_for_all()` were never called.
- After the servers shut themselves down, the swarm coordination registry
  continued emitting periodic metrics, proving `main.rs` was still running.
- The process had to be killed with `kill -9` (exit code 137).
- The log contained **none** of the normal graceful shutdown messages
  (`Torrust tracker shutting down`, `Job completed gracefully`,
  `Torrust tracker successfully shutdown.`).

See also [analysis §7.4 and §8.1](../../analysis/20260716-shutdown-process/README.md)
and [research §4.2](../../research/20260716-console-shutdown-patterns/README.md).

## Implementation

Change `src/main.rs` from:

```rust
tokio::select! {
    _ = tokio::signal::ctrl_c() => {
        tracing::info!("Torrust tracker shutting down ...");
        jobs.cancel();
        jobs.wait_for_all(Duration::from_secs(10)).await;
        tracing::info!("Torrust tracker successfully shutdown.");
    }
}
```

To:

```rust
#[cfg(unix)]
let mut sigterm = tokio::signal::unix::signal(
    tokio::signal::unix::SignalKind::terminate()
).expect("failed to install SIGTERM handler");

tokio::select! {
    _ = tokio::signal::ctrl_c() => {
        tracing::info!("Torrust tracker shutting down (SIGINT) ...");
    }
    #[cfg(unix)]
    _ = sigterm.recv() => {
        tracing::info!("Torrust tracker shutting down (SIGTERM) ...");
    }
}

jobs.cancel();
jobs.wait_for_all(Duration::from_secs(10)).await;
tracing::info!("Torrust tracker successfully shutdown.");
```

**Important nuance from the experimental baseline**: after this change there
will be a **redundant double-signal** for SIGTERM: `main.rs` catches it _and_
each server's `global_shutdown_signal()` also catches it. In practice, `main.rs`
reacts first (since it is the top-level `tokio::select!`), calls `jobs.cancel()`,
and sends halt messages to all servers. The servers' own SIGTERM handlers then
fire as a redundant no-op. This is harmless but produces extra log noise
(duplicate `caught interrupt signal (terminate)` messages from each server).
The clean removal of `global_shutdown_signal()` is tracked in SI-2.

## Acceptance Criteria

- [ ] `kill <pid>` against the tracker binary starts a graceful shutdown.
- [ ] `kill -TERM <pid>` starts a graceful shutdown.
- [ ] Ctrl+C still works as before.
- [ ] The tracker logs distinguish the signal source: `(SIGINT)` vs `(SIGTERM)`.
- [ ] After SIGTERM, `main.rs` logs `Torrust tracker shutting down (SIGTERM) ...`.
- [ ] `JobManager` logs `Waiting for job to finish` for each job.
- [ ] All jobs report `Job completed gracefully`.
- [ ] Final log line is `Torrust tracker successfully shutdown.`
- [ ] No periodic metrics log lines appear after the final shutdown message.
- [ ] Process exits with code 0 (not 137).
- [ ] `cargo test` passes.
- [ ] `linter all` passes.
- [ ] Phase 2 of [verification.md](./verification.md) is fully completed.

## Open Questions Affecting This Sub-issue

- [Q1](../../features/shutdown-process/open-questions.md#q1): The
  double-signal for SIGTERM after this change is harmless but should be noted.
  SI-2 must follow to clean it up.

## Dependencies

- No hard prerequisites. Can land independently.
- SI-2 should follow to remove the `global_shutdown_signal()` redundancy.

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Setup

```bash
# Build the release binary
cargo build --release

# Start the tracker in a terminal
RUST_LOG=info ./target/release/torrust-tracker
```

Wait for all services to report "Started on" in the log output.

### Test 1: `kill <pid>` triggers graceful shutdown (SIGTERM)

```bash
# In a second terminal, get the binary PID (not the cargo PID)
pgrep -x torrust-tracker
kill <pid>
```

**Expected**:

- Log shows: `Torrust tracker shutting down (SIGTERM) ...`
- Log shows each job completing gracefully: `Job completed gracefully job=...`
- Log shows: `Torrust tracker successfully shutdown.`
- Process exits with code 0.
- No `SIGKILL` is needed.
- Verify exit code: `echo $?` in the terminal running the tracker (should be 0).

**Record in `verification.md`**: full log output from shutdown start to exit.

### Test 2: `kill -TERM <pid>` (explicit SIGTERM)

Repeat Test 1 using `kill -TERM <pid>`. Expected outcome is identical.

### Test 3: Ctrl+C still works (SIGINT)

```bash
# Start the tracker, then press Ctrl+C in its terminal
```

**Expected**:

- Log shows: `Torrust tracker shutting down (SIGINT) ...`
- Same graceful shutdown sequence as Test 1.
- Log **does not** say `(SIGTERM)`.

### Test 4: Signal source is distinguishable in logs

Confirm that the log message text differs between SIGINT and SIGTERM shutdowns
(i.e., the log says "SIGINT" vs "SIGTERM" respectively).

### Test 5: `docker stop` triggers graceful shutdown

```bash
# Run tracker in a container
docker run -d --name torrust-test torrust/tracker:dev
docker stop torrust-test
docker logs torrust-test
```

**Expected**:

- Logs show the graceful shutdown sequence ending with `successfully shutdown`.
- `docker stop` returns within 10 seconds (Docker default).
- No "Killed" or forced exit message from Docker.

### Test 6: `kill -9 <pid>` still works (SIGKILL, unchanged behavior)

Verify that `kill -9` still terminates the process immediately (this is OS behavior
and cannot be changed, but should be confirmed as still functional).
