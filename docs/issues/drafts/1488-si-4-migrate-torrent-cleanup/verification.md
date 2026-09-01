# Verification Evidence — Torrent Cleanup Token Migration

> **Status**: Not started — record deterministic cancellation evidence before
> the end-to-end signal verification.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:

## Test Results

### Deterministic cancellation

- [ ] Unit test injects and cancels a component `CancellationToken`.
- [ ] Test awaits the torrent-cleanup task and observes normal completion.
- [ ] No test delivers an OS signal to prove the job's cancellation behavior.

### Application wiring

- [ ] `src/app.rs` derives the component token from `JobManager`.
- [ ] Source contains no direct `ctrl_c()` listener in the torrent-cleanup job.
