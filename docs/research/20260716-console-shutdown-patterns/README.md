---
doc-type: research
status: draft
last-updated-utc: 2026-07-16
semantic-links:
  related-artifacts:
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/features/shutdown-process/README.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

# Console Shutdown Patterns: SIGINT vs SIGTERM

## Status

Draft — research conducted on 2026-07-16.

## Summary

This document investigates how console applications, particularly long-running
network services written in Rust, handle OS signals for graceful shutdown. It
focuses on the differences between `SIGINT` (Ctrl+C) and `SIGTERM` (default
`kill` signal), and how real-world projects like Vector (Datadog) implement
their shutdown logic.

## 1. OS Signals Overview

### 1.1 SIGINT (Signal Interrupt)

| Property                  | Value                                                  |
| ------------------------- | ------------------------------------------------------ |
| **Signal number**         | 2                                                      |
| **Default action**        | Terminate process                                      |
| **Can be caught/ignored** | Yes                                                    |
| **How sent**              | Ctrl+C in terminal, `kill -2 <pid>`, `kill -INT <pid>` |
| **Typical meaning**       | "User requested interrupt"                             |

**Characteristics:**

- Typically sent by the user from a terminal (Ctrl+C).
- The process is expected to stop **promptly** but can clean up.
- Some programs treat it as a "soft" shutdown (print status and continue).
- When caught by a Tokio runtime, it can be received by **multiple** tasks if
  they all call `tokio::signal::ctrl_c()`. However, only **one** task will
  actually receive it — the signal is consumed by the first listener.

### 1.2 SIGTERM (Signal Terminate)

| Property                  | Value                                                                                                              |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| **Signal number**         | 15                                                                                                                 |
| **Default action**        | Terminate process                                                                                                  |
| **Can be caught/ignored** | Yes                                                                                                                |
| **How sent**              | `kill <pid>`, `kill -15 <pid>`, `kill -TERM <pid>`, Docker/Podman `stop`, Kubernetes pre-stop hook, systemd `stop` |
| **Typical meaning**       | "Please terminate gracefully"                                                                                      |

**Characteristics:**

- This is the **default** signal sent by `kill`, Docker/Podman `stop`,
  Kubernetes, systemd, and most process managers.
- The process is expected to perform a **graceful shutdown** (drain connections,
  flush data, close files) and then exit.
- If the process does not exit within a grace period, a `SIGKILL` (signal 9) is
  sent, which cannot be caught and force-terminates the process.
- The `tokio::signal::ctrl_c()` function does **not** handle SIGTERM. You must
  use `tokio::signal::unix::signal(SignalKind::terminate())` on Unix.

### 1.3 SIGQUIT (Signal Quit)

| Property                  | Value                                                   |
| ------------------------- | ------------------------------------------------------- |
| **Signal number**         | 3                                                       |
| **Default action**        | Terminate with core dump                                |
| **Can be caught/ignored** | Yes                                                     |
| **How sent**              | Ctrl+\ in terminal, `kill -3 <pid>`, `kill -QUIT <pid>` |
| **Typical meaning**       | "Quit and dump core"                                    |

Used by Vector to trigger a **quick/forced quit** (no graceful shutdown).

### 1.4 SIGHUP (Signal Hangup)

| Property                  | Value                                                |
| ------------------------- | ---------------------------------------------------- |
| **Signal number**         | 1                                                    |
| **Default action**        | Terminate                                            |
| **Can be caught/ignored** | Yes                                                  |
| **How sent**              | Closing terminal, `kill -1 <pid>`, `kill -HUP <pid>` |
| **Typical meaning**       | Traditionally "hang up" — reload configuration       |

Used by Vector (and many daemons) to trigger a **configuration reload**.

## 2. Signal Handling in Tokio

### 2.1 `tokio::signal::ctrl_c()`

```rust
pub async fn ctrl_c() -> Result<()>
```

- Only handles `SIGINT` (signal 2).
- Available on all platforms (Unix + Windows).
- On Windows, it uses `SetConsoleCtrlHandler` to catch Ctrl+C, Ctrl+Break,
  and console close events.
- **Important**: Only one task can successfully wait for `ctrl_c()`. If multiple
  tasks call `ctrl_c()`, only one receives the signal. The others will hang
  indefinitely.

### 2.2 `tokio::signal::unix::signal()`

```rust
pub fn signal(kind: SignalKind) -> Result<Signal>
```

- Unix-only.
- Can handle any signal: `SIGTERM`, `SIGINT`, `SIGHUP`, `SIGQUIT`, and user-defined signals, etc.
- Returns a `Signal` stream that yields `()` each time the signal is received.
- **Important**: Like `ctrl_c()`, only one instance of a particular signal
  handler can be created. Creating a second handler for the same signal will
  overwrite the first.

### 2.3 `tokio_util::sync::CancellationToken`

```rust
pub struct CancellationToken { ... }
impl CancellationToken {
    pub fn new() -> Self;
    pub fn cancel(&self);
    pub fn cancelled(&self) -> WaitForCancellationFuture;
    pub fn child_token(&self) -> Self;
    pub fn drop_guard(&self) -> DropGuard;
}
```

- Not a signal mechanism, but a **coordination** mechanism.
- Used to propagate shutdown signals from a central coordinator to many tasks.
- Tasks check `token.cancelled()` or await `token.cancelled()` in their loops.
- Child tokens inherit parent cancellation.
- `DropGuard` auto-cancels on drop (useful for RAII-style shutdown).

## 3. How Real-World Projects Handle Shutdown

### 3.1 Vector (Datadog) — `src/signal.rs`

Vector is a high-performance observability data pipeline written in Rust. It has
a sophisticated signal handling system:

**Unix signal handling** (`src/signal.rs`):

```rust
#[cfg(unix)]
fn os_signals(runtime: &Runtime) -> impl Stream<Item = SignalTo> + use<> {
    runtime.block_on(async {
        let mut sigint = signal(SignalKind::interrupt()).expect("...");
        let mut sigterm = signal(SignalKind::terminate()).expect("...");
        let mut sigquit = signal(SignalKind::quit()).expect("...");
        let mut sighup = signal(SignalKind::hangup()).expect("...");

        async_stream::stream! {
            loop {
                let signal = tokio::select! {
                    _ = sigint.recv() => {
                        info!(message = "Signal received.", signal = "SIGINT");
                        SignalTo::Shutdown(None)
                    },
                    _ = sigterm.recv() => {
                        info!(message = "Signal received.", signal = "SIGTERM");
                        SignalTo::Shutdown(None)
                    },
                    _ = sigquit.recv() => {
                        info!(message = "Signal received.", signal = "SIGQUIT");
                        SignalTo::Quit
                    },
                    _ = sighup.recv() => {
                        info!(message = "Signal received.", signal = "SIGHUP");
                        SignalTo::ReloadFromDisk
                    },
                };
                yield signal;
            }
        }
    })
}
```

**Windows signal handling**:

```rust
#[cfg(windows)]
fn os_signals() -> impl Stream<Item = SignalTo> {
    async_stream::stream! {
        loop {
            let signal = tokio::signal::ctrl_c().map(|_| SignalTo::Shutdown(None)).await;
            yield signal;
        }
    }
}
```

**Key observations from Vector:**

- Both `SIGINT` and `SIGTERM` produce the **same** `SignalTo::Shutdown` action.
- `SIGQUIT` produces a **different** action (`SignalTo::Quit`) — immediate exit
  without graceful shutdown.
- `SIGHUP` triggers a **configuration reload** (`SignalTo::ReloadFromDisk`).
- Vector uses a **broadcast channel** (`SignalTx`/`SignalRx`) to propagate
  signals from the handler to all interested components.
- The shutdown flow is: `Application::start()` → `StartedApplication::main()`
  (event loop) → `FinishedApplication::shutdown()`.
- Graceful shutdown has a configurable timeout (`--graceful-shutdown-limit-secs`,
  default 60s). After the timeout, force shutdown occurs.
- The `stop()` method has a **two-phase shutdown**: first mark the API as
  unavailable (for Kubernetes readiness probes), then drain the topology.

### 3.2 Axum (Tokio) — `Handle::graceful_shutdown()`

Axum provides a built-in mechanism for graceful HTTP shutdown:

```rust
use axum_server::Handle;

let handle = Handle::new();

// Spawn the graceful shutdown watcher
tokio::spawn(async move {
    // Wait for signal
    signal::ctrl_c().await.unwrap();
    // Start graceful shutdown
    handle.graceful_shutdown(Some(Duration::from_secs(30)));
});

// Pass the handle to the server
axum_server::from_tcp(listener)
    .handle(handle)
    .serve(app.into_make_service())
    .await
    .unwrap();
```

**Key observations:**

- `graceful_shutdown(Some(duration))` stops accepting new connections and waits
  for existing connections to finish, up to the given duration.
- After the grace period, remaining connections are forcibly closed.
- The `Handle` also provides `connection_count()` to monitor active connections.

### 3.3 Torrust Tracker (Current Implementation)

The current implementation is covered in detail in the
[shutdown analysis](../../analysis/20260716-shutdown-process/README.md). Key points:

- `main.rs` only handles `SIGINT` via `tokio::signal::ctrl_c()`.
- `SIGTERM` is not handled at the top level — only inside each server via
  `torrust_server_lib::signals::global_shutdown_signal()`.
- The `global_shutdown_signal()` handles both `SIGINT` and `SIGTERM` via
  `tokio::signal::unix::signal(SignalKind::terminate())`.
- This creates a **double-signal** problem on Ctrl+C: both `main.rs` and each
  server's `global_shutdown_signal()` catch the same signal independently.

## 4. Common Patterns and Best Practices

### 4.1 Centralized Signal Handling (Recommended)

```text
┌─────────────────────────────────────────────────┐
│                   main.rs                       │
│                                                 │
│   tokio::select! {                              │
│       _ = sigint() => shutdown().await,         │
│       _ = sigterm() => shutdown().await,        │
│       _ = sigquit() => quit().await,            │
│       _ = sighup() => reload().await,           │
│   }                                             │
│                                                 │
│   async fn shutdown() {                         │
│       token.cancel();                           │
│       send_halt_to_all_servers().await;         │
│       wait_for_all_jobs(timeout).await;         │
│   }                                             │
└─────────────────────────────────────────────────┘
         │          │           │
         ▼          ▼           ▼
   ┌─────────┐ ┌─────────┐ ┌─────────┐
   │  Job 1  │ │  Job 2  │ │  Job 3  │
   │(token)  │ │(token)  │ │(channel)│
   └─────────┘ └─────────┘ └─────────┘
```

**Benefits:**

- Single source of truth for shutdown decisions.
- Predictable shutdown order.
- Can differentiate between signals (e.g., SIGQUIT → immediate exit).
- No double-signal problem.

### 4.2 Signal Differentiation

Most projects treat `SIGINT` and `SIGTERM` the same way: graceful shutdown.
However, some projects differentiate:

| Signal                | Typical Action                      | Notes                    |
| --------------------- | ----------------------------------- | ------------------------ |
| `SIGINT`              | Graceful shutdown                   | User pressed Ctrl+C      |
| `SIGTERM`             | Graceful shutdown (possibly faster) | Container orchestrator   |
| `SIGQUIT`             | Immediate/dirty shutdown            | User wants to force quit |
| `SIGHUP`              | Reload configuration                | Reload without restart   |
| `SIGUSR1` / `SIGUSR2` | Toggle debug/log level              | Custom behavior          |

For the Torrust Tracker, there is likely **no need to differentiate** between
`SIGINT` and `SIGTERM` — both should trigger the same graceful shutdown
sequence. The key missing piece is simply that `SIGTERM` is not handled at the
top level.

### 4.3 Grace Period Configuration

Production services should make the shutdown grace period configurable:

```toml
[shutdown]
# Maximum time to wait for jobs to finish before force-exiting.
# Kubernetes terminationGracePeriodSeconds should be set higher than this.
grace_period_secs = 30

# How long each Axum server waits for connections to drain.
# This must be <= grace_period_secs.
connection_drain_secs = 25
```

**Reference values from real projects:**

| Project                   | Grace Period  | Configurable?                          |
| ------------------------- | ------------- | -------------------------------------- |
| Vector                    | 60s           | Yes (`--graceful-shutdown-limit-secs`) |
| Kubernetes pod            | 30s (default) | Yes (`terminationGracePeriodSeconds`)  |
| Docker/Podman stop        | 10s (default) | Yes (`--time`)                         |
| systemd                   | 90s (default) | Yes (`TimeoutStopSec`)                 |
| Torrust Tracker (current) | 10s per job   | No (hardcoded)                         |

### 4.4 Observable Shutdown

During shutdown, the application should log which jobs are still running:

```text
2026-07-16T12:00:00Z INFO  Shutting down ...
2026-07-16T12:00:00Z INFO  Waiting for jobs to finish (timeout: 30s)...
2026-07-16T12:00:05Z INFO  Still waiting for: HTTP tracker (0.0.0.0:7070), Torrent cleanup
2026-07-16T12:00:10Z INFO  Still waiting for: HTTP tracker (0.0.0.0:7070) — 3 active connections
2026-07-16T12:00:12Z INFO  HTTP tracker (0.0.0.0:7070) — done
2026-07-16T12:00:12Z INFO  All jobs finished. Shutdown complete.
```

Vector does this by printing which components won't shutdown gracefully when
the deadline is reached:

```rust
if let Some(deadline) = deadline {
    let mut check_handles2 = check_handles.clone();
    Box::pin(async move {
        sleep_until(deadline).await;
        check_handles2.retain(|_key, handles| {
            retain(handles, |handle| handle.peek().is_none());
            !handles.is_empty()
        });
        // Log remaining handles that haven't finished
        if !check_handles2.is_empty() {
            warn!(...);
        }
    })
}
```

### 4.5 Two-Phase Shutdown for Network Services

Vector implements a two-phase shutdown pattern that is useful for services
behind load balancers:

1. **Phase 1**: Mark the service as unhealthy (Kubernetes readiness probe fails).
   This stops new traffic from being routed to this instance.
2. **Phase 2**: Drain existing connections gracefully within the timeout.

```rust
impl TopologyController {
    pub async fn stop(mut self) {
        // Phase 1: Mark the API as unavailable
        #[cfg(feature = "api")]
        if let Some(server) = self.api_server.as_mut() {
            server.set_not_serving().await;
        }

        // Phase 2: Drain the topology
        self.topology.stop().await;
    }
}
```

### 4.6 Windows Considerations

On Windows, `tokio::signal::ctrl_c()` handles Ctrl+C, Ctrl+Break, and console
close events. There is no equivalent of `SIGTERM` on Windows. The standard
approach is:

```rust
#[cfg(windows)]
let terminate = std::future::pending::<()>();

#[cfg(unix)]
let terminate = async {
    tokio::signal::unix::signal(SignalKind::terminate())
        .expect("...")
        .recv()
        .await;
};
```

This is exactly what the current `torrust_server_lib::signals::global_shutdown_signal()`
does.

## 5. Recommendations for the Torrust Tracker

### 5.1 Handle SIGTERM in `main.rs`

Add `SIGTERM` handling alongside the existing `SIGINT` handler:

```rust
#[cfg(unix)]
use tokio::signal::unix::{SignalKind, signal};

#[tokio::main]
async fn main() {
    let (_app_container, jobs) = app::run().await;

    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    let mut sigterm = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Torrust tracker shutting down (SIGINT) ...");
        }
        #[cfg(unix)]
        _ = sigterm.recv() => {
            tracing::info!("Torrust tracker shutting down (SIGTERM) ...");
        }
    }

    jobs.cancel();
    jobs.wait_for_all(Duration::from_secs(30)).await;
    tracing::info!("Torrust tracker successfully shutdown.");
}
```

### 5.2 Remove `global_shutdown_signal()` from Servers

Once `main.rs` handles both signals, the duplicate `global_shutdown_signal()`
inside each server's `shutdown_signal()` should be removed. The halt channel
alone is sufficient — `main.rs` sends the halt signal to all servers during
shutdown.

### 5.3 Make Grace Periods Configurable

Add a `[shutdown]` configuration section (tracked in the EPIC as a draft
sub-issue).

### 5.4 Consider Concurrent Job Waiting

Change `JobManager::wait_for_all()` to wait for all jobs **concurrently**
with a shared timeout, rather than sequentially.

### 5.5 Consider SIGQUIT for Immediate Exit

Optionally, add `SIGQUIT` handling for an immediate, non-graceful exit (useful
for developers who want to force-stop the tracker without waiting).

## 6. References

- [Tokio Signal Documentation](https://docs.rs/tokio/latest/tokio/signal/index.html)
- [Tokio Signal Unix](https://docs.rs/tokio/latest/tokio/signal/unix/index.html)
- [Tokio Util CancellationToken](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html)
- [Vector Signal Handling](https://github.com/vectordotdev/vector/blob/master/src/signal.rs)
- [Vector Graceful Shutdown CLI Options](https://github.com/vectordotdev/vector/blob/master/src/cli.rs)
- [Axum Server Graceful Shutdown](https://docs.rs/axum-server/latest/axum_server/struct.Handle.html#method.graceful_shutdown)
- [Torrust Tracker Shutdown Analysis](../../analysis/20260716-shutdown-process/README.md)
