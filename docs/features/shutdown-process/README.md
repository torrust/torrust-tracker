---
doc-type: feature
status: draft
last-updated-utc: 2026-09-01
semantic-links:
  related-artifacts:
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/research/20260716-console-shutdown-patterns/README.md
---

# Feature: Shutdown Process

## Status

Draft — planning complete; implementation has not started.

## Summary

Make the Torrust Tracker conform to the **Unix and container process lifecycle
contracts** — the well-proven, widely-adopted standards that govern how a
long-running service is expected to stop. The tracker should respond correctly
to every conventional shutdown mechanism (`kill`, Ctrl+C, `docker stop`,
`systemctl stop`, Kubernetes pod termination) and behave predictably for human
operators, container runtimes, and automated agents alike.

This is not about adding new features. It is about **not surprising the operator**
— implementing the behavior they already expect based on decades of Unix and
container conventions.

## The Problem in One Sentence

`kill <pid>` — the most basic Unix way to ask a process to stop — silently does
nothing to the Torrust Tracker today. The tracker ignores `SIGTERM`.

> **A note on `kill`**: Despite its name, `kill` does not force-terminate a
> process by default. It sends `SIGTERM` (signal 15), which is simply a polite
> request to stop gracefully. The word "kill" sounds brutal, but the mechanism
> is not — it is the standard Unix way of asking a process to exit. The truly
> forceful command is `kill -9` (SIGKILL), which cannot be caught or ignored.
> The tracker currently treats `kill <pid>` as if it were silent — surprising
> and wrong.

## Motivation

The current shutdown process has several pain points:

1. **Container orchestration**: Docker/Podman send `SIGTERM` by default, but the
   main entry point only handles `SIGINT` (Ctrl+C). Containers may be forcefully
   killed after the orchestrator's own timeout.
2. **Silent hangs**: When a job does not finish in time, only a generic warning
   is logged — operators cannot tell which job is blocking shutdown.
3. **Inconsistent behavior**: Different jobs use different shutdown mechanisms
   (`CancellationToken`, direct `ctrl_c` listener, oneshot channel). Some jobs
   ignore the central `JobManager` entirely.
4. **Timeout mismatch**: The `JobManager` waits 10 seconds per job sequentially,
   while Axum servers have a 90-second graceful shutdown. The main process may
   exit before servers finish draining connections.
5. **No graceful UDP shutdown**: The UDP server simply aborts its main loop.
   In-flight requests are dropped without notice.
6. **Hardcoded timeouts**: Grace periods are magic numbers scattered across the
   codebase with no configuration surface.

## Architecture Decision Criteria

Shutdown architecture choices must be evaluated against these criteria. The
amount of refactoring, number of packages touched, and public API changes are
important migration costs, but are **not** reasons by themselves to reject a
design that materially improves correctness, maintainability, readability, or
testability.

| Criterion                             | Why it matters                                                                                                                           |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| **Single shutdown authority**         | Prevents double-signal races and makes responsibility for shutdown decisions explicit.                                                   |
| **One normalized cancellation model** | Lets maintainers add jobs and servers without inventing another shutdown mechanism.                                                      |
| **Library usability**                 | Independently embedded HTTP and UDP servers must have a clear, predictable lifecycle contract.                                           |
| **Testability**                       | Tests must inject and observe shutdown deterministically, without relying on OS signals.                                                 |
| **Observability**                     | The supervisor must know which services are draining, stopped, failed, or timed out.                                                     |
| **Failure behavior**                  | The design must specify behavior when a controller is dropped, a task panics, or the process is force-terminated.                        |
| **API quality**                       | Public API changes are acceptable when they materially improve lifecycle semantics for component consumers.                              |
| **Migration cost**                    | Breaking changes and multi-package refactors must be planned, phased, documented, and tested; they are not automatic rejection criteria. |

The design selected for this feature must satisfy the first six criteria. It
must then explain the API and migration trade-offs against the final two.

## Current Task Topology

The [Preliminary Task Inventory](task-inventory.md) maps the production
tracker's task ownership, spawn hierarchy, retained handles, and current
shutdown triggers. It is a planning aid for the architecture decision; issue
number 1588 must revalidate and complete it against the implementation before that
issue can close.

The [Shutdown Architecture Examples](shutdown-architecture-examples.md) show
the leading Q2 alternatives through nested task levels so their lifecycle and
shutdown-ownership differences can be evaluated explicitly.

## Target Shutdown Architecture

The tracker uses a **supervised cancellation tree**:

- executable entry points translate OS signals into an in-process shutdown
  request;
- `JobManager` supervises named top-level tasks, initiates root-token
  cancellation, and aggregates their completion outcomes;
- each long-running component receives a child `CancellationToken` and owns
  the graceful shutdown and joining of every task it spawns;
- server libraries expose deterministic in-process lifecycle operations, but
  do not subscribe to OS signals.

Cancellation requests a component to stop; joining its task proves whether it
stopped, failed, or timed out. HTTP draining and UDP in-flight-work policy stay
component-specific. See [Q2](questions.md#q2) for the evaluated
alternatives, migration constraints, and decisions intentionally deferred to
Q3–Q5.

### Outcome and Deadline Policy

A fully graceful shutdown exits with code 0. A startup failure or any component
failure, timeout, or deliberate abort exits with code 1. A termination signal
that cannot be handled, such as SIGKILL, has an OS-defined process result.

All top-level components share one 25-second process-wide shutdown deadline.
HTTP, REST API, and health-check connection draining has a 20-second component
budget; UDP active request work has a five-second component budget. The
orchestrator grace period must be at least 30 seconds, leaving at least five
seconds after the tracker process deadline. Docker/Podman's default 10-second
deadline is insufficient and must be configured. See [Q3 and Q4](questions.md#q3)
for rationale and deployment constraints.

### Ownership and Propagation Rule

Shutdown requests flow **top-down** and completion outcomes flow **bottom-up**.
`JobManager` retains only the named, direct components it starts; it must not
collect every nested `JoinHandle`. Each component retains the handles of its
children, propagates cancellation to them, and awaits or deliberately aborts
them before reporting its own outcome. This prevents logically orphaned tasks
while preserving local ownership boundaries.

The `Started` oneshot remains separate because it reports a one-time startup
outcome. Only shutdown signaling migrates from `Halted` oneshot channels to
`CancellationToken` propagation.

### Readiness Before Drain

For a normal shutdown, the application marks itself not ready before it cancels
the root token. While components drain, `/health_check` returns HTTP 503 without
probing downstream services, allowing readiness-aware infrastructure to stop
routing new traffic. This does not stop direct TCP or UDP clients; server
components retain admission and graceful-stop responsibility. The readiness
transition has no separate timeout and remains inside the process deadline.

## The Contracts We Are Implementing

These are established, well-proven standards. We are not inventing anything new —
we are bringing the tracker into compliance with what every process manager,
container runtime, and operator already expects.

### Unix Process Signal Contract

> A well-behaved Unix process catches `SIGTERM` and shuts down gracefully.
> `SIGKILL` is the last resort when a process refuses to stop.

| Signal         | Meaning                       | Expected tracker behavior                                |
| -------------- | ----------------------------- | -------------------------------------------------------- |
| `SIGTERM` (15) | "Please stop gracefully"      | Start shutdown sequence, drain connections, exit cleanly |
| `SIGINT` (2)   | "User pressed Ctrl+C"         | Same as `SIGTERM` — start graceful shutdown              |
| `SIGKILL` (9)  | "Stop immediately, no choice" | Immediate termination by OS — cannot be caught           |

**Currently**: `SIGTERM` is ignored. `kill <pid>` has no effect.
**After fix**: `kill <pid>` triggers the same graceful shutdown as Ctrl+C.

### Windows Console Shutdown Support

On Windows, executable entry points use Tokio `ctrl_c()` to translate supported
console control events into the same in-process shutdown request. Unix-only
SIGTERM handling is conditionally compiled and has no Windows equivalent in this
feature. Server libraries remain platform-independent and do not subscribe to
OS signals. Windows service-control-manager integration and forceful task
termination are out of scope.

### Signal Targeting and Forced Termination

Send a signal to the tracker process that actually owns the Tokio runtime. In
development, `cargo run` can be a launcher process with a separate tracker
child process; signaling only Cargo does not signal that child. Use the tracker
binary PID for direct tests, or deliberately target the relevant process group.

SIGKILL cannot be handled or made graceful. It terminates the tracker process,
all its Tokio tasks, and its owned sockets. Containers and systemd are
responsible for sending SIGKILL only after their configured shutdown grace
period; server libraries must not retain OS-signal listeners as a fallback.

### Docker / Podman Container Stop Contract

> `docker stop <container>` sends `SIGTERM` and waits up to 10 seconds (the
> `--time` grace period). If the process has not exited, it sends `SIGKILL`.

```bash
# What docker stop does internally:
kill -TERM <pid>   # sends SIGTERM, waits up to 10s
kill -KILL <pid>   # sends SIGKILL if process is still running
```

**Currently**: `docker stop torrust-tracker` sends `SIGTERM`, which is ignored.
After the 10s timeout, Docker force-kills the container with `SIGKILL`.
**After fix**: `docker stop torrust-tracker` triggers graceful shutdown when
Docker is configured with at least the 30-second external grace period required
by the [Outcome and Deadline Policy](#outcome-and-deadline-policy). Docker's
default 10-second deadline is insufficient.

### Kubernetes Pod Termination Contract

> When a pod is deleted or evicted, Kubernetes sends `SIGTERM` and waits for
> `terminationGracePeriodSeconds` (default 30s) before sending `SIGKILL`.

**Currently**: Pod termination force-kills the tracker every time.
**After fix**: The tracker drains active connections and exits cleanly.

### Systemd / Init System Contract

> `systemctl stop <service>` sends `SIGTERM` and waits for `TimeoutStopSec`
> (default 90s on most distros) before sending `SIGKILL`.

**Currently**: `systemctl stop torrust-tracker` has no effect — the tracker
keeps running. After `TimeoutStopSec`, systemd force-kills it.
**After fix**: `systemctl stop torrust-tracker` gracefully shuts down the
tracker as expected.

### The Principle of Least Surprise

Any operator, developer, or automated agent interacting with the tracker will
try the most natural stop mechanisms first. They should not be surprised:

```bash
# These all SHOULD work — and currently DON'T (except Ctrl+C):
kill <pid>               # sends SIGTERM — currently ignored ❌
kill -TERM <pid>         # sends SIGTERM — currently ignored ❌
docker stop <container>  # sends SIGTERM — currently ignored ❌
podman stop <container>  # sends SIGTERM — currently ignored ❌
systemctl stop <service> # sends SIGTERM — currently ignored ❌

# This works but is less standard:
kill -INT <pid>          # sends SIGINT — works ✅

# This is the last resort and should never be needed:
kill -9 <pid>            # SIGKILL — force kill, no cleanup ❌
```

After implementing this feature, every graceful-stop mechanism marked ❌ above
will work when its required external grace period is configured. SIGKILL remains
an OS-enforced last resort and cannot become graceful.

## User Value

| Stakeholder             | Value                                                                                                                                                           |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Operators / DevOps**  | Every standard stop mechanism works as expected. No more needing to know the tracker's quirks. Container orchestrators can rely on `SIGTERM` working correctly. |
| **Developers**          | Single, consistent shutdown pattern for all job types. Easy to add new jobs with correct shutdown behavior.                                                     |
| **AI agents / scripts** | `kill <pid>` and `docker stop` work without special-casing the tracker. No need for `kill -9`.                                                                  |
| **End users**           | Fewer dropped connections during restarts/deployments. Active HTTP requests are drained before the process exits.                                               |

## Production Scenarios

The shutdown process must work correctly in all production scenarios where the
tracker runs:

### 1. Docker / Podman Containers

Container orchestrators send `SIGTERM` when stopping a container, followed by
`SIGKILL` after a grace period (default 10s). The tracker must:

- Catch `SIGTERM` and start the shutdown sequence.
- Drain active connections within the orchestrator's grace period.
- Exit cleanly before `SIGKILL` is sent.

### 2. Cloud Providers (Kubernetes, ECS, Nomad, etc.)

Cloud platforms add a pre-stop hook or a configurable termination grace period
(e.g., `terminationGracePeriodSeconds` in Kubernetes, typically 30s). The
tracker must:

- Respond to `SIGTERM` (sent by the platform before killing the pod).
- Respect the configured grace period and exit promptly.
- Allow the platform to collect logs before the pod is removed.

### 3. Systemd / Init System Managed Services

When a service manager stops the tracker, it sends `SIGTERM` and waits for the
process to exit. The tracker must:

- Handle `SIGTERM` correctly.
- Log shutdown progress so the service manager can capture it.
- Respect `TimeoutStopSec` (or equivalent) in the service unit.

### 4. Human User Running and Stopping the Service

A developer or operator starts the tracker from a terminal and presses Ctrl+C.
The tracker must:

- Catch `SIGINT` (Ctrl+C) and start the shutdown sequence.
- Show clear progress feedback in the terminal.
- Exit cleanly within a reasonable time.

### 5. AI Agents Running and Stopping the Service

Automated agents (CI/CD pipelines, deployment scripts, monitoring systems) start
and stop the tracker programmatically. They typically send signals via process
management (e.g., `kill`, `docker stop`, systemd). The tracker must:

- Behave predictably regardless of how the stop signal is sent.
- Not hang indefinitely.
- Exit with a consistent exit code so the agent can detect success/failure.

## Shutdown Triggers (Options Considered)

The tracker needs multiple ways to trigger shutdown. Below are the options
considered, with the recommended combination.

### Current Situation

`main.rs` only listens for `SIGINT` (Ctrl+C) via `tokio::signal::ctrl_c()`.
The `kill` command sends `SIGTERM` by default, which is not handled. AI agents
are forced to use `kill -INT <pid>` or `kill <pid>` (which does nothing) and
then fall back to `kill -9 <pid>`.

### Option 1: SIGTERM Handler (Minimum Recommendation)

Add a `SIGTERM` handler alongside the existing `SIGINT` handler. `kill <pid>`
would then trigger graceful shutdown automatically.

```rust
tokio::select! {
    _ = tokio::signal::ctrl_c() => { /* SIGINT */ }
    _ = tokio::signal::unix::signal(
        tokio::signal::unix::SignalKind::terminate()
    ).expect("...").recv() => { /* SIGTERM */ }
};
```

**Pros:**

- Minimal code change.
- Unix standard: any well-behaved process responds to `SIGTERM`.
- Compatible with `docker stop`, systemd, Kubernetes, and all process managers.
- AI agents can use `kill <pid>` (no `-9` needed).

**Cons:**

- AI agents still need to find the PID.

### Option 2: Unix Domain Socket Command Channel

Create a Unix socket (e.g. `/tmp/torrust-tracker.sock`) where commands can be
sent.

```rust
let listener = UnixListener::bind("/tmp/torrust-tracker.sock")?;
```

Usage: `echo "shutdown" | nc -U /tmp/torrust-tracker.sock`

**Pros:**

- Full control over commands (shutdown, status, metrics, etc.).
- No need to know the PID.
- Can be authenticated/authorized.

**Cons:**

- More code to maintain.
- Socket management (cleanup on exit, avoid collisions between instances).
- Only works on Unix.

### Option 3: HTTP Shutdown Endpoint

The tracker already exposes a REST API. Add a shutdown endpoint:

```text
POST /api/shutdown
```

Which internally triggers the `CancellationToken` from `JobManager`.

Usage: `curl -X POST http://localhost:1212/api/shutdown`

**Pros:**

- Very natural for AI agents.
- No need for PID or filesystem access.
- Integrates with existing API authentication.
- Works over network (remote shutdown).

**Cons:**

- Only works if the REST API is enabled and reachable.
- Security considerations (who can call this endpoint).

### Option 4: Custom Signals (SIGUSR1 and SIGUSR2)

Use Unix user-defined signals for custom actions:

```rust
tokio::signal::unix::signal(SignalKind::user_defined1())?
```

**Pros:**

- No new infrastructure needed.
- Can differentiate between shutdown (SIGUSR1) and other actions (SIGUSR2).

**Cons:**

- Not standard — operators must remember custom signal numbers.
- Only works on Unix.

### Conclusion and Current Priority

Adding `SIGTERM` is the only change needed to satisfy the Unix, Docker,
Kubernetes, and systemd contracts. It is a small code change with very high
value. The HTTP endpoint is a useful future enhancement for remote/API-driven
management, but it is not needed to meet the fundamental standards.

| Trigger                    | Priority       | Rationale                                                                                                                                            |
| -------------------------- | -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| **SIGTERM handler**        | ✅ Now         | Implements the Unix/Docker/K8s/systemd contract. Makes `kill <pid>`, `docker stop`, `systemctl stop`, and Kubernetes pod termination work correctly. |
| **HTTP shutdown endpoint** | 🔜 Future      | Useful for API-driven management and remote shutdown. Not needed for contract compliance.                                                            |
| Unix domain socket         | ❌ Not planned | SIGTERM covers the use case. Adds complexity without proportional value.                                                                             |
| Custom signals             | ❌ Not needed  | Non-standard. SIGTERM is sufficient.                                                                                                                 |

## Scope

### In Scope

- Centralize signal handling in `main.rs` (both `SIGINT` and `SIGTERM`).
- Consistent shutdown mechanism for all jobs (prefer `CancellationToken`).
- Configurable grace periods.
- Observable shutdown progress (which jobs are still running).
- Proper UDP server shutdown (drain or at least log in-flight work).
- Grace period alignment between `JobManager` and server-level shutdown.

### Out of Scope

- Hot-reload / restart without process exit.
- `SIGHUP` configuration reload. Configuration changes require a normal graceful
  restart; dynamic reload is deferred to a separate future feature.
- Dynamic job lifecycle (start/stop jobs at runtime via admin API).
- Windows-specific signal handling beyond what Tokio provides.
- The **profiling binary** (`src/console/profiling.rs`) — it is a developer-only
  tool for profiling (valgrind/callgrind), not a user-facing entry point. It can
  be updated independently as needed.

## Design Ideas

### Centralized Signal Handling

Only `main.rs` captures OS signals. On signal receipt:

1. Mark the application not ready.
2. Log the shutdown request and cancel the root `CancellationToken`.
3. Let each component propagate cancellation to, then join or deliberately
   abort, its owned child tasks.
4. Await named top-level component outcomes concurrently under the single
   process deadline.
5. Map aggregate outcomes to the defined process exit result.

### Consistent Job Interface

Every long-running job accepts a `CancellationToken` and observes it in its
main loop. Components that need protocol-specific shutdown, such as Axum
draining, implement that behavior behind their token-aware lifecycle API rather
than receiving a shutdown `Halted` channel.

### Observable Shutdown

The `JobManager` should periodically log which jobs are still running during
shutdown, e.g.:

```text
Waiting for jobs to finish (process deadline: 25s)...
  ✅ Health Check API — done
  ⏳ HTTP tracker (127.0.0.1:7070) — still running (5 active connections)
  ⏳ Torrent cleanup — still running
  ❌ Activity metrics updater — timed out
```

### Shutdown Deadline Policy

SI-20 will add validated configuration for the approved deadline hierarchy:

- a 25-second process-wide shutdown deadline;
- a 20-second connection-drain budget for HTTP, REST API, and health-check
  servers;
- a 5-second completion budget for accepted UDP requests; and
- an externally configured orchestrator grace period of at least 30 seconds,
  with 35 seconds or more recommended where practical.

The process deadline is shared by all top-level components, not applied
sequentially per job. Docker and Podman's default 10-second stop deadline is
therefore insufficient and must be explicitly increased.

## Related Documents

- [Analysis: Shutdown Process](../../analysis/20260716-shutdown-process/README.md) — detailed code-level analysis
- [Research: Console Shutdown Patterns](../../research/20260716-console-shutdown-patterns/README.md) — SIGINT vs SIGTERM and real-world patterns
- [EPIC: Overhaul Tracker Shutdown](../../issues/open/1488-overhaul-tracker-shutdown/ISSUE.md) — concrete task breakdown
