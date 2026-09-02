---
doc-type: analysis
status: draft
last-updated-utc: 2026-07-16
semantic-links:
  related-artifacts:
    - src/main.rs
    - src/app.rs
    - src/container.rs
    - src/bootstrap/jobs/manager.rs
    - src/bootstrap/jobs/torrent_cleanup.rs
    - src/bootstrap/jobs/activity_metrics_updater.rs
    - src/bootstrap/jobs/health_check_api.rs
    - src/bootstrap/jobs/http_tracker.rs
    - src/bootstrap/jobs/udp_tracker.rs
    - src/bootstrap/jobs/tracker_apis.rs
    - src/bootstrap/jobs/torrent_repository.rs
    - src/bootstrap/jobs/tracker_core.rs
    - src/bootstrap/jobs/udp_tracker_core.rs
    - src/bootstrap/jobs/udp_tracker_server.rs
    - src/bootstrap/jobs/http_tracker_core.rs
    - packages/axum-server/src/signals.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/states.rs
    - packages/udp-server/src/server/mod.rs
    - packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs
    - packages/axum-server/src/custom_axum_server.rs
    - docs/features/shutdown-process/README.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/open/1588-review-shutdown-process-for-all-tasks-jobs/ISSUE.md
    - docs/research/20260716-console-shutdown-patterns/README.md
  related-issues:
    - "https://github.com/torrust/torrust-tracker/issues/1488"
    - "https://github.com/torrust/torrust-tracker/issues/1588"
    - "https://github.com/torrust/torrust-tracker/issues/1477"
    - "https://github.com/torrust/torrust-tracker/issues/1405"
---

# Shutdown Process Analysis

## Overview

This document analyses the current shutdown process of the Torrust Tracker application.
The tracker is a multi-service BitTorrent tracker composed of several concurrent jobs
(UDP servers, HTTP servers, REST API, Health Check API, event listeners, periodic cleanup
tasks). The shutdown process must coordinate stopping all these jobs cleanly.

## 1. Entry Point

The main entry point is in `src/main.rs`:

```rust
#[tokio::main]
async fn main() {
    let (_app_container, jobs) = app::start().await;

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Torrust tracker shutting down ...");

            jobs.cancel();

            jobs.wait_for_all(Duration::from_secs(10)).await;

            tracing::info!("Torrust tracker successfully shutdown.");
        }
    }
}
```

**Key observations:**

- Only `SIGINT` (Ctrl+C) is handled at the top level.
- `SIGTERM` is **not** handled in `main.rs` (though it is handled internally by servers — see §3).
- The shutdown sequence is: `cancel()` → `wait_for_all(Duration::from_secs(10))`.
- The 10-second grace period is a **hardcoded magic number**.
- Jobs are waited **sequentially** (one by one), each with the same timeout.

## 2. The `JobManager` (`src/bootstrap/jobs/manager.rs`)

The `JobManager` is a central coordinator that holds:

- A `Vec<Job>` — each job has a `name` and a `JoinHandle<()>`.
- A shared `CancellationToken`.

### 2.1 Job Cancellation

```rust
pub fn cancel(&self) {
    self.cancellation_token.cancel();
}
```

Cancelling the `CancellationToken` signals all jobs that were registered with a
`new_cancellation_token()`. However, not all jobs use this token (see §2.2.2).

### 2.2 Waiting for All Jobs

```rust
pub async fn wait_for_all(mut self, grace_period: Duration) {
    for job in self.jobs.drain(..) {
        // ... waited sequentially with timeout(grace_period, job.handle)
    }
}
```

**Key observations:**

- Jobs are waited **sequentially**, not concurrently.
- Each job gets the **same** grace period timeout.
- If a job times out, its named top-level task is logged, aborted, and awaited.
    The manager therefore does not detach that handle, but this is forceful
    escalation rather than a graceful component outcome.
- The order of waiting is the order jobs were pushed (currently: event listeners first,
  then servers, then periodic tasks, then API servers).

## 3. Three Shutdown Mechanisms (Inconsistent)

The tracker uses **three different mechanisms** to signal shutdown to its various jobs.
This inconsistency is a primary area for improvement.

### 3.1 `CancellationToken` (used by event listeners and server wrappers)

Used by statistics event listeners:

- `swarm_coordination_registry` event listener
- `tracker_core` event listener
- `http_core` event listener
- `udp_core` event listener
- `udp_server` stats event listener
- `udp_server` banning event listener

These jobs receive a `CancellationToken` from the `JobManager` and check `token.cancelled()`
in their main loop. They respond to `jobs.cancel()`.

The UDP banning cleanup job and the HTTP tracker, REST API, health-check API,
and UDP tracker wrappers also receive the shared token. Each server wrapper
currently translates cancellation into its private `Halted::Normal` oneshot
message, then awaits its corresponding server task. This is an already-landed
token-to-halt migration bridge, not the target lifecycle contract: server
libraries still retain OS-signal behavior and some server-owned children remain
unjoined.

### 3.2 Direct `tokio::signal::ctrl_c()` (used by periodic jobs)

Used by:

- **Torrent cleanup** (`src/bootstrap/jobs/torrent_cleanup.rs`)
- **Activity metrics updater** (`packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs`)

These jobs listen for `tokio::signal::ctrl_c()` directly inside a `tokio::select!` in their
own loop. They **do not** respond to `jobs.cancel()` — they have no connection to the
`CancellationToken`. However, they will still stop when Ctrl+C is pressed because the signal
fires globally.

### 3.3 Oneshot Channel `Halted` (used by server instances)

Used by all server types:

- **UDP tracker** (`packages/udp-server/src/server/launcher.rs`)
- **HTTP tracker** (`packages/axum-http-server/src/server.rs`)
- **REST API** (`packages/axum-rest-api-server/src/server.rs`)
- **Health Check API** (`packages/axum-health-check-api-server/src/server.rs`)

Each server is started with a `oneshot::Receiver<Halted>`. The server task awaits
`shutdown_signal_with_message(rx_halt)` which internally calls `shutdown_signal(rx_halt)`.

The `shutdown_signal()` function (from `torrust_server_lib::signals`) is a `tokio::select!`
between:

1. The halt channel (receiving `Halted::Normal` from the main process)
2. The `global_shutdown_signal()` (Ctrl+C or SIGTERM)

Each server can therefore react to a library-level OS signal independently of
the application supervisor. In the current tracker application, its wrapper
also translates the manager token into that server's private halt message. The
two paths coexist during the migration bridge.

## 4. The `global_shutdown_signal()` (`torrust_server_lib::signals`)

```rust
pub async fn global_shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("...");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(SignalKind::terminate())
            .expect("...")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => { ... },
        () = terminate => { ... }
    }
}
```

**Key observations:**

- Handles both `SIGINT` (Ctrl+C) and `SIGTERM` (Unix) — but only inside servers.
- The `global_shutdown_signal()` is used **inside** each server's `shutdown_signal()`.
- This means that when the user presses Ctrl+C:
  1. `main.rs` catches it and calls `jobs.cancel()` + `jobs.wait_for_all()`.
  2. Each server ALSO catches it independently via `global_shutdown_signal()`.
  3. This creates a **double-signal** scenario — servers react to Ctrl+C both via the
     halt channel **and** via the global signal.
- The `global_shutdown_signal()` is **not** used in `main.rs` — only `ctrl_c()` is.

## 5. Graceful Shutdown Per Server

### 5.1 Axum Servers (HTTP Tracker, REST API, Health Check API)

All three Axum-based servers use the same `graceful_shutdown` function from
`packages/axum-server/src/signals.rs`:

```rust
pub async fn graceful_shutdown(handle, rx_halt, message, address) {
    shutdown_signal_with_message(rx_halt, message).await;

    let grace_period = Duration::from_secs(90);
    let max_wait = Duration::from_secs(95);

    handle.graceful_shutdown(Some(grace_period));

    loop {
        // Poll connection count every second
        // Break when: connections == 0 OR max_wait elapsed
    }
}
```

**Key observations:**

- Grace period is **90 seconds** with a **95-second** upper bound.
- The 10-second delta allows for the `graceful_shutdown` to complete before the loop times out.
- Connections are drained actively — the server waits for active HTTP connections to finish.
- **BUT**: `main.rs` waits **10 seconds per job sequentially**. A wrapper that
    exceeds that limit is aborted and joined before the manager proceeds to the
    next job. Because the Axum drain controller is detached, that abort can leave
    the wrapper unable to prove drain completion; total shutdown latency can grow
    with the number and order of blocked jobs.

### 5.2 UDP Server

The UDP server (`packages/udp-server/src/server/launcher.rs`) has a different approach:

```rust
select! {
    _ = running => { ... },
    () = shutdown_signal_with_message(rx_halt, ...) => { ... }
}
running.abort();  // Force-abort the main loop
```

**Key observations:**

- The UDP server **cannot** drain connections gracefully — it simply aborts the main loop.
- The launcher directly awaits the halt channel or library-level OS signal in
    its `select!`; it does not spawn a separate halt-signal task.
- There is no connection draining mechanism for UDP.
- After abort, `tokio::task::yield_now().await` gives other tasks a chance to complete.

## 6. Startup and Shutdown Architecture

The public startup boundary in `src/app.rs` is `app::start()`, which completes
configuration loading and application bootstrap before returning the application
container and `JobManager`. Its bootstrap path starts jobs in this order:

```rust
pub async fn start() -> Result<(Arc<AppContainer>, JobManager), Error> {
    let (config, app_container) = bootstrap::app::setup()
        .await
        .map_err(|source| Error::Setup { source })?;
    let app_container = Arc::new(app_container);
    run_after_setup(&config, &app_container).await
}
```

Jobs are started in this order:

1. Event listeners (swarm, core, http-core, udp-core, UDP-server stats, UDP-server banning)
2. UDP IP-ban cleanup
3. UDP tracker instances
4. HTTP tracker instances
5. Torrent cleanup (periodic)
6. Peers inactivity update (periodic)
7. REST API
8. Health Check API

The shutdown (waiting) order follows the push order — jobs pushed first are
waited first:

1. Event listeners (swarm, core, http-core, udp-core, UDP-server stats, banning)
2. UDP IP-ban cleanup
3. UDP tracker instances
4. HTTP tracker instances
5. Torrent cleanup
6. Peers inactivity update
7. REST API
8. Health Check API (waited last)

> This was confirmed experimentally in §8.2.

## 7. Identified Issues

### 7.1 Inconsistent Shutdown Mechanisms

Jobs use three different mechanisms (`CancellationToken`, direct `ctrl_c`, halt channel).
This makes it hard to reason about the shutdown process and hard to add new job types.

### 7.2 Torrent Cleanup and Activity Metrics Ignore `CancellationToken`

These two jobs listen for `ctrl_c` directly instead of using the shared `CancellationToken`.
They will still stop on Ctrl+C because the signal fires globally, but they won't respond
to `jobs.cancel()`.

### 7.3 Grace Period Mismatch

The `JobManager` waits **10 seconds per job sequentially**, while Axum servers have a
**90-second grace period**. This means:

A wrapper can be force-aborted before the Axum server's 90-second drain finishes.
The detached drain controller can then continue independently until runtime
teardown, while the manager continues its sequential waits.

### 7.4 No SIGTERM in `main.rs`

Only `SIGINT` (Ctrl+C) is handled at the top level. SIGTERM (used by container orchestrators
like Docker/Podman) is only handled inside each server via `global_shutdown_signal()`. If
a container runtime sends SIGTERM, the servers will react, but `jobs.cancel()` will never
be called, and `jobs.wait_for_all()` will never execute.

### 7.5 Sequential Job Waiting

Jobs are waited one by one, and every job receives the full 10-second grace
period. Consequently, each blocked job can add another 10 seconds to total
shutdown time. This should be replaced by concurrent waiting under one shared
process deadline.

### 7.6 Hardcoded Grace Periods

Both the `JobManager`'s 10-second timeout and the Axum's 90-second grace period are
hardcoded magic numbers. They are not configurable.

### 7.7 Double-Signal on Ctrl+C

When Ctrl+C is pressed:

1. `main.rs` catches it and starts the shutdown sequence.
2. Each server's `shutdown_signal()` also catches it via `global_shutdown_signal()`.
3. This creates a race: the main process calls `jobs.cancel()`, server wrappers
    forward that cancellation to their private `Halted` channels, and servers may
    already be shutting down from the global signal.

### 7.8 UDP Server Has No Graceful Shutdown

The UDP server simply aborts its main loop. There is no mechanism to wait for in-flight
UDP requests to complete before stopping.

### 7.9 Profiling Binary Shutdown Difference

The profiling binary in `src/console/profiling.rs` has a different shutdown path:

- It does not call `jobs.cancel()` before `jobs.wait_for_all()`.
- It uses a timed shutdown instead of Ctrl+C.
- This means the `CancellationToken` is never triggered for the profiling binary.

> **Note**: The profiling binary is a developer-only tool for profiling
> (valgrind/callgrind), not a user-facing entry point. It is out of scope for
> the shutdown process feature (EPIC #1488) and can be updated independently
> as needed.

## 8. Experimental Validation

The findings in this analysis were validated by running the tracker locally on
2026-07-16 using the default development configuration (`cargo run`).

### 8.1 SIGTERM Test

**Command**: `kill <pid>` (sends `SIGTERM` by default)

**Result**: The tracker **kept running**. No shutdown sequence was initiated.
The logs continued normally with periodic metrics output and torrent cleanup
tasks. The process had to be force-killed with `kill -9`.

**Conclusion**: Confirms that `SIGTERM` is not handled at the top level. The
`main.rs` entry point only listens for `SIGINT`. This is the most critical gap
for container orchestration and AI agents.

### 8.2 SIGINT Test (Ctrl+C simulation)

**Command**: `kill -INT <pid>` (sends `SIGINT`, same as Ctrl+C)

**Result**: The tracker shut down gracefully. The logs showed:

```text
1. `main.rs` caught SIGINT and called `jobs.cancel()` + `jobs.wait_for_all()`.
2. JobManager waited for each job sequentially with a 10s timeout.
3. All jobs completed gracefully within the timeout.
4. Final message: "Torrust tracker successfully shutdown."
```

**Key observation — double-signal confirmed**: The logs also showed that each
server's `global_shutdown_signal()` caught the same SIGINT independently:

```text
WARN  torrust_server_lib::signals: caught interrupt signal (ctrl-c), halting...
```

This confirms the **double-signal problem** identified in §7.7: both `main.rs`
and each server's internal signal handler catch the same Ctrl+C.

**Key observation — shutdown order**: The JobManager waited jobs in this order:

1. Event listeners (swarm, tracker-core, http-core, udp-core, udp-server stats,
   udp-server banning)
2. UDP instances (6868, 6969)
3. HTTP instances (7070, 7171)
4. Torrent cleanup
5. Peers inactivity update
6. HTTP API
7. Health Check API

All completed within the 10s timeout.

### 8.3 Graceful shutdown works

Despite the signal-handling issues, the internal shutdown mechanism is solid:

- Axum servers drain connections (the log shows "All connections closed").
- Event listeners respect the `CancellationToken`.
- The `JobManager` reports each job's status during shutdown.

### 8.4 `cargo run` vs actual binary PID

When starting with `cargo run`, the PID of `cargo` itself is different from the
PID of the final `torrust-tracker` binary. This means:

- `kill <cargo-pid>` sends the signal to `cargo`, not the tracker.
- The tracker binary runs as a child process.
- If `cargo` is killed, the tracker becomes orphaned but continues running.

This is relevant for development workflows but not for production deployments
(where the binary runs directly or via a container entrypoint).

## 9. Summary Table

| Aspect                       | Current Implementation                                | Status                             |
| ---------------------------- | ----------------------------------------------------- | ---------------------------------- |
| Top-level signal handling    | Only `SIGINT` in `main.rs`                            | ⚠️ Missing `SIGTERM`               |
| Server shutdown mechanism    | Manager token forwarded to `Halted` + `global_shutdown_signal()` | ⚠️ Transitional bridge |
| Event listener shutdown      | `CancellationToken` from `JobManager`                 | ✅ Functional                      |
| Periodic job shutdown        | Direct `tokio::signal::ctrl_c()`                      | ⚠️ Inconsistent                    |
| Axum connection draining     | 90s grace period, polls connection count              | ✅ Functional but timeout mismatch |
| UDP connection draining      | None — `abort()`                                      | ❌ Not graceful                    |
| Job waiting strategy         | Sequential per-job timeout                            | ⚠️ Sequential + timeout mismatch   |
| Grace period configurability | Hardcoded everywhere                                  | ❌ Not configurable                |
| Double-signal on Ctrl+C      | Both `main.rs` and servers catch it                   | ⚠️ Potential race                  |
