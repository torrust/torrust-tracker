# Verification Evidence — Activity Metrics Token Migration

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
- [ ] Test awaits the activity-metrics task and observes normal completion.
- [ ] No test delivers an OS signal to prove the job's cancellation behavior.

### Application wiring

- [ ] The bootstrap adapter receives a token derived from `JobManager`.
- [ ] Source contains no direct `ctrl_c()` listener in the activity-metrics job.
