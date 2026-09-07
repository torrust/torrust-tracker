# Verification Evidence — Issue #1586: `JoinSet` Evaluation

> **Status**: Final validation passed.

## Environment

- Date: 2026-09-07
- OS: Linux
- Rust version (`rustc --version`): `1.100.0-nightly (f248f4038 2026-09-05)`
- Tracker commit/branch: `1586-evaluate-job-manager-join-set` (uncommitted implementation)

## Architecture Decision

- [x] Record whether `JoinSet` is adopted or rejected.
- [x] Link the selected cancellation-tree architecture and explain how the
      chosen implementation preserves top-level versus nested ownership.
- [ ] If rejected, record the explicitly justified alternative.

**Evidence:**

`JobManager` adopts `tokio::task::JoinSet` and accepts only direct component
futures through `spawn(name, future)`. Each registered runner returns the
explicit `ComponentResult` contract: completed, cooperatively cancelled, or a
contextual component failure. Panics and deadline aborts remain named
supervisor outcomes; cancellation is never inferred from the global token.
After one concurrent grace deadline it calls `abort_all`, drains every JoinSet
result, and returns a named `Aborted` outcome.

The separately sequenced SI-4/SI-5 periodic-job token migrations are excluded.
Those periodic jobs retain their existing starters and Ctrl-C/token behavior.
Their pre-spawned handles are registered in `JobManager`'s compatibility
registry, where they share the direct components' single process-wide deadline
and are then joined and escalated. They are intentionally not claimed as direct
`JoinSet` components because Tokio cannot adopt such a handle without the
forbidden wrapper task. This transitional registry is expected to be removed by
SI-4/SI-5.

## Deterministic Supervisor Tests

- [x] Completion order is observed without sequential waits.
- [x] Completed, failed/panicked, timed-out, cancelled, and deliberately
      aborted top-level components retain their names in outcomes.
- [x] The process-wide deadline covers all top-level components concurrently.
- [x] Unfinished work follows explicit escalation and is not silently detached.

**Evidence:**

Focused manager tests pass. They cover causally ordered completion, explicit
component failure with context, named panic, explicit cooperative cancellation,
one global zero-duration deadline over two blocked direct tasks, and the same
one-second deadline over blocked direct and two legacy tasks with Tokio's paused
time. The latter confirms the elapsed time is exactly one grace period rather
than one grace period per legacy task. The nested-server and halt-signal
deadline tests use readiness channels, so the deadline starts only after the
outer component has constructed its cleanup guard. The escalation cases confirm
dropped oneshot senders reject sends after `wait_for_all` returns, proving every
task was joined rather than detached.

## Ownership Review

- [x] Direct component futures are registered without a wrapper task that only
      awaits an existing handle.
- [x] Component-owned child handles are not registered with `JobManager`.
- [ ] The final #1588 inventory supports the documented ownership boundary.

**Evidence:**

`src/app.rs` passes server/API component futures and six unspawned event-listener
runners directly to `JobManager::spawn`; no listener `JoinHandle` is created
or dropped in the application. The listener dispatch loops themselves return
`Completion::Cancelled` or `Completion::Completed`, avoiding redundant outer
cancellation `select!` wrappers. Bootstrap-level spawned listener helpers have
been removed, and the application listener test uses the direct runner. Startup
handshakes still occur before a server runner is returned and registered. Server
runtime handles remain inside their component runner and their join failures are
returned as `ComponentError`, rather than being logged as successful completion. The torrent-cleanup,
activity-metrics, and UDP ban-cleanup periodic jobs deliberately retain their
baseline starters. Their handles remain under `JobManager` compatibility supervision and are not
direct `JoinSet` components.

Server runners retain a drop-safe nested-task owner. If the manager aborts the
outer component at the shared deadline while it awaits an uncooperative server,
the owner sends normal halt and aborts the nested `JoinHandle`; focused manager
tests prove both the nested server and a nested shutdown-controller receiver are
gone when shutdown returns. The health-check server now returns its running
future together with its spawned graceful-shutdown controller. Its component
owns both: it joins the controller after cooperative shutdown and aborts it if
the outer component is deadline-aborted.

## Summary

| Check                                                                 | Result | Evidence link or note                   |
| --------------------------------------------------------------------- | ------ | --------------------------------------- |
| `JoinSet` decision                                                    | Pass   | Direct named top-level ownership        |
| Concurrent outcomes                                                   | Pass   | `join_next_with_id` under one timeout   |
| Named failure paths                                                   | Pass   | Explicit result, panic, and abort tests |
| Deadline and escalation                                               | Pass   | `abort_all`, then drain/join            |
| Ownership boundary                                                    | Pass   | Direct runners in `src/app.rs`          |
| `cargo fmt --all`                                                     | Pass   | Formatting applied before validation    |
| `cargo test -p torrust-tracker-axum-health-check-api-server`          | Pass   | Health-check package tests              |
| `cargo test -p torrust-tracker --lib bootstrap::jobs::manager::tests` | Pass   | Focused manager tests                   |
| `cargo test -p torrust-tracker --lib`                                 | Pass   | Tracker library tests                   |
| `cargo test -p torrust-tracker-http-core`                             | Pass   | HTTP listener package tests             |
| `cargo test -p torrust-tracker-swarm-coordination-registry`           | Pass   | Registry listener package tests         |
| `cargo test -p torrust-tracker-core`                                  | Pass   | Tracker-core listener package tests     |
| `cargo test -p torrust-tracker-udp-core`                              | Pass   | UDP-core listener package tests         |
| `cargo test -p torrust-tracker-udp-server`                            | Pass   | UDP-server listener package tests       |
| `cargo test --test lifecycle-signals`                                 | Pass   | Lifecycle signal integration tests      |
| `cargo clippy --all-targets --all-features -- -D warnings`            | Pass   | Workspace clippy gate                   |
| `linter all`                                                          | Pass   | Repository linter gate                  |
| `git diff --check`                                                    | Pass   | No whitespace errors                    |
