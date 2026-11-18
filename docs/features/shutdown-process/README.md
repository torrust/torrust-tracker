---
doc-type: feature
status: done
last-updated-utc: "2026-11-18 17:25"
semantic-links:
  related-artifacts:
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - docs/features/shutdown-process/legacy-api-migration.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/closed/1488-overhaul-tracker-shutdown/EPIC.md
    - docs/research/20260716-console-shutdown-patterns/README.md
---

# Feature: Shutdown Process

## Status

Implemented. [EPIC #1488](../../issues/closed/1488-overhaul-tracker-shutdown/EPIC.md) delivered this
feature and was closed on 2026-11-18; its roadmap records each sub-issue.

How to read this document: it was written before the implementation, as the definition of the feature,
and it keeps that text as history. The sections that describe a problem ("The Problem in One Sentence",
"Motivation", the "Before the EPIC" notes, "Situation Before the EPIC") describe the tracker **before**
the EPIC, in the present tense of that time. The sections "Target Shutdown Architecture", "Outcome and
Deadline Policy", "Readiness Before Drain" and "Shutdown Deadline Policy" describe the **current**
behaviour. The current task ownership, startup and shutdown sequences are in
[Application Jobs](../../architecture/application-jobs.md); the [task inventory](task-inventory.md) lists
every task; the [legacy API migration guide](legacy-api-migration.md) maps each removed legacy shutdown
symbol to its token-aware replacement.

One design idea was not built: the periodic "still running" progress report sketched in
[Observable Shutdown](#observable-shutdown). The manager logs each stop request, what each waiting
component waits for, and each component's outcome instead.

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

This section and the next describe the tracker before the EPIC. Both problems
are fixed.

`kill <pid>` — the most basic Unix way to ask a process to stop — begins an
uncoordinated partial shutdown today. Server libraries observe `SIGTERM`, but
`main.rs` does not cancel and await all application jobs.

> **A note on `kill`**: Despite its name, `kill` does not force-terminate a
> process by default. It sends `SIGTERM` (signal 15), which is simply a polite
> request to stop gracefully. The word "kill" sounds brutal, but the mechanism
> is not — it is the standard Unix way of asking a process to exit. The truly
> forceful command is `kill -9` (SIGKILL), which cannot be caught or ignored.
> The tracker currently handles `SIGTERM` at the wrong boundary, leaving the
> process without coordinated application-wide shutdown — surprising and wrong.

## Motivation

The current shutdown process has several pain points:

1. **Container orchestration**: Docker/Podman send `SIGTERM` by default, but the
   main entry point only handles `SIGINT` (Ctrl+C). Containers may be forcefully
   killed after the orchestrator's own timeout.
2. **Incomplete shutdown observability**: The current manager logs the name of
   a job that exceeds its timeout, but it has no concurrent aggregate outcome
   model or complete component-level drain progress.
3. **Inconsistent behavior**: Different jobs use different shutdown mechanisms
   (`CancellationToken`, direct `ctrl_c` listener, oneshot channel). Some jobs
   ignore the central `JobManager` entirely.
4. **Timeout mismatch**: The `JobManager` waits 10 seconds per job sequentially
   and force-aborts timed-out wrappers, while Axum servers have a 90-second
   graceful shutdown. The wrapper cannot prove its detached drain completed.
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

This is the implemented architecture. The tracker uses a **supervised shutdown
model** (first designed as a cancellation tree with one root token, see the ADRs
below):

- the executable entry point translates OS signals into an in-process shutdown
  request (`app::run_until_shutdown_signal` in `src/app.rs`, called by `main.rs`);
- `JobManager` supervises named top-level tasks, issues each its own
  `CancellationToken`, requests their cancellation in a declared order (an event
  listener is cancelled only after the components that publish to its buses have
  completed), and aggregates their completion outcomes;
- each long-running component owns the graceful shutdown and joining of every
  task it spawns;
- server libraries expose deterministic in-process lifecycle operations, but
  do not subscribe to OS signals.

Cancellation requests a component to stop; joining its task proves whether it
stopped, failed, or timed out. HTTP draining and UDP in-flight-work policy stay
component-specific. See [Q2](questions.md#q2) for the evaluated
alternatives, migration constraints, and decisions intentionally deferred to
Q3–Q5.

The durable repository-wide decision and its alternatives are recorded in the
[supervised cancellation-tree ADR](../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md),
which the [shutdown-order ADR](../../adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md)
supersedes: it keeps every still-valid point and adds per-component tokens, the
stop order of event producers and listeners, and the listener drain. The current
supervision tree and the startup and shutdown sequences are in
[Application Jobs](../../architecture/application-jobs.md).

### Outcome and Deadline Policy

A fully graceful shutdown exits with code 0. A startup failure or any component
failure, timeout, or deliberate abort exits with code 1. A termination signal
that cannot be handled, such as SIGKILL, has an OS-defined process result.

All top-level components share one process-wide shutdown deadline, 25 seconds
by default. HTTP, REST API, and health-check connection draining has a
15-second component budget by default; UDP active request work has a
five-second component budget; the event listeners that process the events
queued by those servers have a five-second budget of their own. The stages run
one after the other (servers first, listeners second), so the tracker refuses
to start unless `max(HTTP drain, UDP drain) + listener drain` is below the
deadline. All four values are set in `[application.shutdown]`; see
[Shutdown Deadline Policy](#shutdown-deadline-policy). The orchestrator grace
period must be at least 30 seconds for the defaults, leaving at least five
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

For a normal shutdown, the application marks itself not ready before it calls
`JobManager::cancel()`. While components drain, `/health_check` returns HTTP 503 without
probing downstream services, allowing readiness-aware infrastructure to stop
routing new traffic. This does not stop direct TCP or UDP clients; server
components retain admission and graceful-stop responsibility. The readiness
transition has no separate timeout and remains inside the process deadline.

#### Readiness Contract for Operators

Use `GET /health_check` on the health-check API (default `127.0.0.1:1313`) as the
readiness probe.

| Tracker state                        | `/health_check` answer                                                                 |
| ------------------------------------ | ---------------------------------------------------------------------------------------- |
| Running                              | `200` with the probe results of every registered service (unchanged). |
| Shutdown requested (SIGTERM, SIGINT) | `503` with `{"status":"Error","message":"shutting down","details":[]}`, no service probed. |
| Process exited or still starting     | The connection is refused: not ready either.                                              |

- The change is one-way: a tracker that began shutting down never reports ready again.
- The health-check API stops after every other component, so a new probe gets `503` (not a refused
  connection) for the whole drain. Only when every other component has completed does it stop, which
  is within the process deadline `P` at the latest (see [the ADR](../../adrs/20261104120915_stop_the_health_check_api_last_to_report_not_ready_during_shutdown.md)).
- A `503` removes the instance from Kubernetes Service endpoints and from readiness-aware load
  balancers. It does not stop clients that connect directly to a tracker port: each server drains its
  accepted work under its own budget.
- Startup failure: the process exits before the health-check API is bound, so a failed start never
  advertises readiness.
- Do not use `/health_check` as a liveness probe: a draining tracker is alive, and restarting it
  would cut the drain.

```yaml
# Kubernetes: readiness, not liveness, uses the health check API.
readinessProbe:
  httpGet:
    path: /health_check
    port: 1313
  periodSeconds: 5
  failureThreshold: 1
terminationGracePeriodSeconds: 35 # at least P + 5 s, see the deadline policy
```

Every `503` is logged at error level by the health-check API, so a drain with a probe every few
seconds produces error lines; they are the expected answer, not a failure.

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

**Before the EPIC**: server libraries reacted to `SIGTERM`, but `main.rs` did not
cancel and await every application job. `kill <pid>` therefore did not provide
a coordinated process shutdown.
**Now**: `kill <pid>` triggers the same coordinated graceful shutdown as
Ctrl+C.

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

**Before the EPIC**: `docker stop torrust-tracker` sent `SIGTERM`, which server
libraries observed independently while `main.rs` did not coordinate cancellation
and completion of all application jobs. After the 10s timeout, Docker could
force-kill the container with `SIGKILL`.
**Now**: `docker stop torrust-tracker` triggers graceful shutdown when
Docker is configured with at least the 30-second external grace period required
by the [Outcome and Deadline Policy](#outcome-and-deadline-policy). Docker's
default 10-second deadline is insufficient.

### Kubernetes Pod Termination Contract

> When a pod is deleted or evicted, Kubernetes sends `SIGTERM` and waits for
> `terminationGracePeriodSeconds` (default 30s) before sending `SIGKILL`.

**Before the EPIC**: Pod termination force-killed the tracker every time.
**Now**: The tracker drains active connections and exits cleanly within its
process deadline; `terminationGracePeriodSeconds` must be at least 30 seconds
for the default budgets.

### Systemd / Init System Contract

> `systemctl stop <service>` sends `SIGTERM` and waits for `TimeoutStopSec`
> (default 90s on most distros) before sending `SIGKILL`.

**Before the EPIC**: `systemctl stop torrust-tracker` did not shut the tracker
down — it kept running until systemd force-killed it after `TimeoutStopSec`.
**Now**: `systemctl stop torrust-tracker` gracefully shuts down the
tracker as expected.

### The Principle of Least Surprise

Any operator, developer, or automated agent interacting with the tracker will
try the most natural stop mechanisms first. They should not be surprised:

Before the EPIC:

```bash
# These all SHOULD trigger coordinated shutdown, but only stopped the servers:
kill <pid>               # SIGTERM reaches server libraries but bypasses main ❌
kill -TERM <pid>         # SIGTERM reaches server libraries but bypasses main ❌
docker stop <container>  # SIGTERM reaches server libraries but bypasses main ❌
podman stop <container>  # SIGTERM reaches server libraries but bypasses main ❌
systemctl stop <service> # SIGTERM reaches server libraries but bypasses main ❌

# This works but is less standard:
kill -INT <pid>          # sends SIGINT — works ✅

# This is the last resort and should never be needed:
kill -9 <pid>            # SIGKILL — force kill, no cleanup ❌
```

Now every graceful-stop mechanism marked ❌ above works when its required
external grace period is configured. SIGKILL remains an OS-enforced last resort
and cannot become graceful.

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

### Situation Before the EPIC

`main.rs` only listened for `SIGINT` (Ctrl+C) via `tokio::signal::ctrl_c()`.
The `kill` command sends `SIGTERM` by default, which was not handled. AI agents
were forced to use `kill -INT <pid>` or `kill <pid>` (which did not shut the
tracker down) and then fall back to `kill -9 <pid>`. Option 1 below was
implemented by SI-1 (#2132).

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

- Centralize signal handling at the executable boundary (both `SIGINT` and
  `SIGTERM`): implemented in `src/app.rs`, which `main.rs` calls.
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

These ideas guided the implementation. Each subsection says what was built.

### Centralized Signal Handling

Only the tracker executable captures OS signals, in
`app::run_until_shutdown_signal` (`src/app.rs`), which `main.rs` calls. On signal
receipt:

1. Log the shutdown request.
2. Mark the application not ready and call `JobManager::cancel()`
   (`app::begin_shutdown`). There is no root `CancellationToken`: the manager
   cancels the token of every component that waits for no other component, and
   cancels each event listener when the components that publish on its buses
   have completed.
3. Let each component propagate cancellation to, then join or deliberately
   abort, its owned child tasks.
4. Await named top-level component outcomes concurrently under the single
   process deadline (`JobManager::wait_for_all`), aborting what remains at the
   deadline.
5. Map aggregate outcomes to the defined process exit result.

### Consistent Job Interface

Every long-running job accepts a `CancellationToken` and observes it in its
main loop. Components that need protocol-specific shutdown, such as Axum
draining, implement that behavior behind their token-aware lifecycle API rather
than receiving a shutdown `Halted` channel.

### Observable Shutdown

Not built in this form. The idea was that the `JobManager` should periodically
log which jobs are still running during shutdown, e.g.:

```text
Waiting for jobs to finish (process deadline: 25s)...
  ✅ Health Check API — done
  ⏳ HTTP tracker (127.0.0.1:7070) — still running (5 active connections)
  ⏳ Torrent cleanup — still running
  ❌ Activity metrics updater — timed out
```

What was built: the manager logs each stop request and what each waiting
component waits for, every component logs its own drain, each outcome is logged
by name when the component finishes, and the final log line names the
components that did not stop gracefully.

### Shutdown Deadline Policy

The `[application.shutdown]` configuration table holds the validated deadline
hierarchy (see the
[budget ADR](../../adrs/20261027145446_budget_shutdown_in_two_chained_stages_inside_one_process_deadline.md)
and the [placement ADR](../../adrs/20261027145429_place_process_wide_settings_in_the_application_configuration_section.md)).
The defaults are:

- a 25-second process-wide shutdown deadline (`deadline_in_secs`);
- a 15-second connection-drain budget for HTTP, REST API, and health-check
  servers (`http_drain_in_secs`);
- a 5-second completion budget for accepted UDP requests
  (`udp_request_drain_in_secs`);
- a 5-second admission budget for each event listener, which starts after the
  servers that publish on its bus have stopped (`event_listener_drain_in_secs`);
  and
- an externally configured orchestrator grace period of at least 30 seconds,
  with 35 seconds or more recommended where practical.

Every value is a whole number of seconds from 1 to 3600, and the tracker
refuses to start unless `max(http_drain_in_secs, udp_request_drain_in_secs) +
event_listener_drain_in_secs` is below `deadline_in_secs`. The process deadline
is shared by all top-level components, not applied sequentially per job. Docker
and Podman's default 10-second stop deadline is therefore insufficient and must
be explicitly increased; [Stopping the Tracker](../../containers.md#stopping-the-tracker)
gives the settings for Docker, Compose, Kubernetes, and systemd.

## Related Documents

- [Analysis: Shutdown Process](../../analysis/20260716-shutdown-process/README.md) — detailed code-level analysis
- [Research: Console Shutdown Patterns](../../research/20260716-console-shutdown-patterns/README.md) — SIGINT vs SIGTERM and real-world patterns
- [EPIC: Overhaul Tracker Shutdown](../../issues/closed/1488-overhaul-tracker-shutdown/EPIC.md) — concrete task breakdown
