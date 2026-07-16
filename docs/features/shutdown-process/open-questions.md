---
doc-type: open-questions
status: open
last-updated-utc: 2026-07-16 (Q1 resolved)
semantic-links:
  related-artifacts:
    - docs/features/shutdown-process/README.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/research/20260716-console-shutdown-patterns/README.md
---

# Open Questions: Shutdown Process Feature

This document tracks open questions, risks, and gaps identified during the
specification of the shutdown process feature. All items must be addressed
(answered, resolved, or explicitly deferred with a rationale) before the
feature spec (`README.md`) can be considered complete and the EPIC sub-issues
can be created and scheduled for implementation.

## Progress

| #           | Severity     | Status      | Title                                                               |
| ----------- | ------------ | ----------- | ------------------------------------------------------------------- |
| [Q1](#q1)   | 🔴 Critical  | ✅ Resolved | `global_shutdown_signal()` removal not tracked                      |
| [Q2](#q2)   | 🔴 Critical  | Open        | Halt-sender wiring unspecified in design                            |
| [Q3](#q3)   | 🔴 Critical  | Open        | Exit codes on shutdown not defined                                  |
| [Q4](#q4)   | 🟡 Important | Open        | Docker 10s default vs. tracker grace period                         |
| [Q5](#q5)   | 🟡 Important | Open        | Orphan risk if `main.rs` crashes before sending halts               |
| [Q6](#q6)   | 🟡 Important | Open        | Two-phase shutdown not discussed for Kubernetes rolling deployments |
| [Q7](#q7)   | 🟡 Important | Open        | `#[cfg(unix)]` asymmetry on Windows not noted                       |
| [Q8](#q8)   | 🟢 Minor     | Open        | `SIGHUP` / config reload not explicitly deferred                    |
| [Q9](#q9)   | 🟢 Minor     | Open        | Docker experimental validation missing                              |
| [Q10](#q10) | 🟢 Minor     | Open        | Option 4 heading/body mismatch after signal rename                  |

## Question → Sub-issue Impact

This table shows which sub-issues each question affects and what the current
readiness is. Update it whenever a question is resolved.

| Question | Affected sub-issues         | Impact                                                   |
| -------- | --------------------------- | -------------------------------------------------------- |
| Q1 ✅    | SI-2 (partial), SI-3 (full) | SI-3 fully unblocked. SI-2 still needs Q5.               |
| Q2       | SI-2, SI-4, SI-5            | Wiring design decision needed before SI-2 implementation |
| Q3       | SI-1 (exit code AC), SI-8   | Exit code contract must be defined                       |
| Q4       | SI-6, SI-8                  | Grace period target values must be agreed                |
| Q5       | SI-2                        | Orphan risk strategy must be decided before SI-2 lands   |
| Q6       | SI-6                        | Advisory note — no hard block                            |
| Q7       | SI-1                        | Windows note to add — no hard block                      |
| Q8       | feature doc only            | Out-of-scope note to add — no hard block                 |
| Q9       | SI-1 verification           | Docker test for Phase 2 — no hard block                  |
| Q10      | feature doc only            | Cosmetic fix — no hard block                             |

## Sub-issue Readiness

| Sub-issue | Can start? | Waiting on                      |
| --------- | ---------- | ------------------------------- |
| SI-1      | ✅ Yes     | Nothing                         |
| SI-3      | ✅ Yes     | Nothing (Q1 resolved)           |
| SI-4      | ✅ Yes     | Nothing                         |
| SI-5      | ✅ Yes     | Nothing                         |
| SI-7      | ✅ Yes     | Nothing                         |
| SI-9      | ✅ Yes     | Nothing                         |
| SI-2      | ❌ No      | Q5 (orphan risk strategy)       |
| SI-6      | ❌ No      | Q4 (grace period target values) |
| SI-8      | ❌ No      | Q3 (exit codes), Q4             |

---

## Q1

**Severity**: 🔴 Critical  
**Status**: ✅ Resolved (2026-07-16)  
**Title**: `global_shutdown_signal()` removal not tracked; standalone binaries have a different contract

### Description

#### The double-signal problem in the main tracker binary

The analysis (§7.7) and research (§5.2) both identify a **double-signal problem**
in `src/main.rs`: when `SIGINT` or `SIGTERM` is received, both `main.rs` and each
server's internal `shutdown_signal()` (via `global_shutdown_signal()`) catch the
same signal independently. This creates a race condition where servers may begin
shutting themselves down before `main.rs` has called `jobs.cancel()` and
`jobs.wait_for_all()`.

Without removing `global_shutdown_signal()`, adding `SIGTERM` to `main.rs` creates
a **triple-signal** scenario for SIGTERM:

1. `main.rs` catches SIGTERM and starts the ordered shutdown.
2. Each server's `shutdown_signal()` also catches it via `global_shutdown_signal()`.
3. `main.rs` then also sends a `Halted` message via the oneshot halt channel.

This is **not tracked** as a sub-issue in the EPIC.

#### The standalone binary examples have a completely different shutdown contract

The tracker is intentionally designed as a set of composable packages. There are
two example standalone binaries that show how library users can build their own
trackers:

- `packages/axum-http-server/examples/http_only_public_tracker.rs`
- `packages/udp-server/examples/udp_only_public_tracker.rs`

Both examples use the same pattern — they **do not use `JobManager`** at all.
Instead they rely directly on `global_shutdown_signal()` (via `ctrl_c()`) and
the server's `Environment::stop()` method:

```rust
// Both examples look like this:
tokio::signal::ctrl_c().await.expect("failed to install Ctrl-C handler");
env.stop().await;
```

Looking at what `env.stop()` does in each case:

**HTTP example** (`Environment<Running>::stop()`):

```rust
pub async fn stop(self) -> Environment<Stopped> {
    // Stop the event listener — NOTE: uses abort(), not graceful cancellation
    if let Some(event_listener_job) = self.event_listener_job {
        // todo: send a message to the event listener to stop and wait for it to finish
        event_listener_job.abort();
    }
    // Stop the server — sends Halted::Normal via oneshot channel
    let server = self.server.stop().await.expect("...");
    ...
}
```

**UDP example** (`Environment<Running>::stop()`):

```rust
pub async fn stop(self) -> Environment<Stopped> {
    // Abort all three event listener jobs — NOTE: abort(), not graceful cancellation
    udp_core_event_listener_job.abort();
    udp_server_stats_event_listener_job.abort();
    udp_server_banning_event_listener_job.abort();
    // Stop the server — sends Halted::Normal via oneshot channel
    let server = self.server.stop().await.expect("...");
    ...
}
```

This reveals **two more issues specific to the standalone examples**:

1. **Both examples only handle `SIGINT`** — they call `tokio::signal::ctrl_c()`,
   which is SIGINT only. SIGTERM is **not** handled, just like the main binary.
   `docker stop` or `kill` will be ignored.

2. **Event listeners are `abort()`ed, not gracefully stopped** — the `TODO`
   comments in the code explicitly call this out. The `CancellationToken` in
   `Environment` is created but **never cancelled** — `cancel()` is never called
   on it. This means event listeners are abruptly killed rather than given time
   to drain their event queues. Any in-flight statistics events are lost.

#### The architecture implies a contract question

The tracker is designed as a library. The shutdown contract for library users
(standalone binaries) is:

- Currently: "call `env.stop()` after `ctrl_c()`"
- Problem: `env.stop()` aborts event listeners rather than cancelling them

If we fix `global_shutdown_signal()` in the main tracker binary, the standalone
examples remain broken in different ways. The fix strategy must consider both
consumers.

### Question to answer

1. Can we add `SIGTERM` to `main.rs` (as a standalone sub-issue) without
   removing `global_shutdown_signal()` from the servers, and without breaking
   the standalone binary contract?

2. Should `Environment::stop()` in both `axum-http-server` and `udp-server`
   use `cancellation_token.cancel()` instead of `abort()` for event listeners?
   The `CancellationToken` is already in the `Environment` struct but unused
   during shutdown.

3. Should the standalone example binaries also be updated to handle `SIGTERM`?
   They are documentation/examples but they model the intended usage pattern.

### Proposed approach

**For the main tracker binary (`src/main.rs`):**

Adding `SIGTERM` and removing `global_shutdown_signal()` are logically coupled but
can be landed as two sequential sub-issues if done carefully:

- Sub-issue A: Add `SIGTERM` to `main.rs` (the `global_shutdown_signal()` double-signal
  becomes a triple-signal for SIGTERM, but the behavior is still correct — just redundant).
- Sub-issue B: Remove `global_shutdown_signal()` from `shutdown_signal()` in
  `torrust_server_lib` (requires coordination with the standalone binary consumers).

Sub-issue B touches an external standalone package (`torrust-server-lib`) which
is no longer part of this workspace. That must be factored into planning.

**For the standalone binary examples:**

- Fix `Environment::stop()` to call `cancellation_token.cancel()` and await
  the event listener jobs instead of `abort()`ing them.
- Update both examples to handle `SIGTERM` alongside `SIGINT`.

### Decision

**1. SI-1 (SIGTERM in `main.rs`) can and should land before SI-2.**

The Phase 1 verification evidence confirms the sequence is safe: after SIGTERM,
the servers' `global_shutdown_signal()` reacts independently (they start draining
their connections), while `main.rs` does nothing. After SI-1 lands, `main.rs`
catches SIGTERM first at the top-level `tokio::select!`, calls `jobs.cancel()`,
and sends halt messages to the servers. The servers' own `global_shutdown_signal()`
fires afterward as a redundant no-op. The behavior is correct and the logs will
show duplicate "caught interrupt signal (terminate)" messages — noisy but
harmless. SI-2 will clean this up later.

**2. SI-1 and SI-2 are separate sub-issues landed sequentially.**

- SI-1: Add `SIGTERM` to `main.rs` — no prerequisites, safe to land now.
- SI-2: Remove `global_shutdown_signal()` from `shutdown_signal()` in
  `torrust_server_lib` — requires a coordinated release of `torrust-server-lib`.
  Must not land before the orphan risk in Q5 is also resolved.

**3. `Environment::stop()` should use `cancel()` instead of `abort()` for event
listeners.**

The `CancellationToken` is already wired into `Environment` but never cancelled.
The `TODO` comments in the code call this out explicitly. This is a pre-existing
bug in the standalone library API. The fix is tracked in SI-3.

**4. Both standalone example binaries should be updated to handle `SIGTERM`.**

They model the intended library usage pattern. If a user copies the example as
a starting point, their binary will have the same SIGTERM gap. SI-3 covers this.

### Actions Taken

- [x] Decision recorded: SI-1 and SI-2 are sequential, SI-1 is safe to land first.
- [x] SI-2 already exists in the EPIC sub-issue table with the `torrust-server-lib`
      external dependency noted.
- [x] SI-3 already exists covering `Environment::stop()` abort-vs-cancel and
      SIGTERM for standalone examples.
- [x] Q5 (orphan risk) remains open — it must be resolved before SI-2 lands,
      not before SI-1.

---

## Q2

**Severity**: 🔴 Critical  
**Status**: Open  
**Title**: Halt-sender wiring is unspecified in the design

### Description

The feature doc "Design Ideas" section says:

> "Send halt signal to all servers via oneshot channels."

This is architecturally vague. Currently the `halt_task` (the `Sender<Halted>`)
for each server is owned inside the job closure that was spawned in the bootstrap
job starters (`start_job` functions). The `JobManager` only holds `JoinHandle<()>`
values — it has no reference to the halt senders.

To implement centralized halt dispatch, one of these approaches is needed:

1. **Wrap halt senders in the `JobManager`** — add a `Vec<Sender<Halted>>` to
   `JobManager` alongside `Vec<JoinHandle>`. `cancel()` would then both cancel the
   `CancellationToken` and send `Halted::Normal` to all registered halt senders.
2. **Move server shutdown into `CancellationToken`** — remove the oneshot `Halted`
   channel entirely and make servers watch the `CancellationToken` directly, using
   the Axum `Handle` for connection draining triggered by the token.
3. **Leave halt senders inside the spawned jobs** — each job handles its own halt
   when the `CancellationToken` fires (but this requires the Axum servers to grow
   a `CancellationToken` watch loop, which they currently lack).

The design doc mentions none of these options. Without a concrete decision here,
developers implementing the sub-issues will face an unguided architectural choice
mid-implementation.

### Question to answer

Which wiring approach should be used? This decision affects the scope and
complexity of at least three sub-issues:

- "Centralize signal handling in `main.rs`"
- "Migrate torrent cleanup to `CancellationToken`"
- Any sub-issue touching Axum server shutdown

### Action

- [ ] Select one of the three wiring approaches (or propose a fourth).
- [ ] Document the chosen approach in the feature doc "Design Ideas" section.
- [ ] Check whether the chosen approach affects the `torrust_server_lib::signals`
      package (which is an external standalone crate).

---

## Q3

**Severity**: 🔴 Critical  
**Status**: Open  
**Title**: Exit codes on shutdown not defined

### Description

The feature doc states (AI agent scenario):

> "Exit with a consistent exit code so the agent can detect success/failure."

But neither the feature doc nor the EPIC spec defines what exit codes the tracker
should return. The current code exits 0 after `wait_for_all()` regardless of
whether jobs timed out. This is observed in `src/main.rs`:

```rust
jobs.wait_for_all(Duration::from_secs(10)).await;
tracing::info!("Torrust tracker successfully shutdown.");
// implicit exit 0
```

Open questions:

- Should a **graceful shutdown** (all jobs completed within the grace period) → exit 0?
- Should a **timeout shutdown** (some jobs did not finish in time) → exit 0 or a
  different code (e.g., 1 or `exit_code::UNAVAILABLE`)?
- Should **startup failure** (e.g., port already in use) → exit 1 or a specific code?
- Should systemd's `SuccessExitStatus` be documented for graceful timeouts?

This matters for:

- CI/CD pipelines checking the process exit code.
- Systemd deciding whether to restart the service (`Restart=on-failure`).
- Container orchestrators deciding whether the container exited cleanly.
- AI agents deciding whether to retry or escalate.

### Action

- [ ] Define the exit code contract for the tracker (at minimum: success=0,
      timeout-during-shutdown=?, startup-failure=?).
- [ ] Add an "Exit Codes" section to the feature doc.
- [ ] Check whether Vector's exit code crate pattern is appropriate here.

---

## Q4

**Severity**: 🟡 Important  
**Status**: Open  
**Title**: Docker's 10s default grace period may be shorter than the tracker needs

### Description

The feature doc describes the Docker contract correctly:

> `docker stop` sends SIGTERM and waits up to 10 seconds before SIGKILL.

However, it does not acknowledge the tension between Docker's **10s default** and
the tracker's **10s per-job sequential wait** in `JobManager`. In a worst case:

```text
t=0   Docker sends SIGTERM
t=0   main.rs catches SIGTERM, starts shutdown
t=0   JobManager starts waiting for job 1 (up to 10s)
t=5   job 1 finishes
t=5   JobManager starts waiting for job 2 (up to 10s)
t=10  Docker sends SIGKILL (tracker is killed mid-shutdown)
t=10  jobs 2..N are force-killed with no cleanup
```

The "after fix" description implies the tracker will exit cleanly within Docker's
grace period, but that is only guaranteed if either:

1. All jobs complete well within 10s total (likely in practice, but not guaranteed).
2. Operators configure Docker's `stop_grace_period` to a higher value.

The feature doc should explicitly state the **minimum recommended Docker/Compose
`stop_grace_period`** and warn operators to configure it appropriately.

### Question to answer

What is the minimum recommended `stop_grace_period` for the tracker, given its
current sequential 10s-per-job waiting? Should this be documented in the feature
spec or in a deployment guide?

### Action

- [ ] Add a note to the feature doc warning that Docker's 10s default may be
      insufficient and recommending a `stop_grace_period` ≥ 30s in `compose.yml`.
- [ ] Cross-reference `docs/containers.md` as the place to document deployment
      configuration.

---

## Q5

**Severity**: 🟡 Important  
**Status**: Open  
**Title**: Orphan risk if `main.rs` crashes before sending halt messages

### Description

If we remove `global_shutdown_signal()` from the servers (as recommended in Q1),
the servers will **only stop** when they receive a `Halted` message via their
oneshot channel — which is sent by `main.rs` during the shutdown sequence.

If `main.rs` crashes, panics, or is killed with `SIGKILL` before sending those
halt messages:

- The Tokio tasks running the servers continue running as orphans.
- They hold the TCP/UDP ports.
- The process appears to be gone (from the OS perspective the parent is dead) but
  the ports are not freed.
- A restart attempt will fail with "address already in use".

This was actually observed during experimental validation: when SIGTERM was sent
to the `cargo run` process (not the actual binary), the actual binary became
orphaned and held the ports.

### Question to answer

Should the servers retain a fallback `global_shutdown_signal()` as a safety net
for SIGKILL/crash scenarios, or should the deployment environment handle this
(e.g., container restart policy, systemd `KillMode=control-group`)?

### Action

- [ ] Decide on the safety net strategy.
- [ ] If retaining `global_shutdown_signal()` as a fallback: document this as
      intentional in the server signal design.
- [ ] If relying on the OS/container environment: document the deployment
      requirement (e.g., "always run in a container with a restart policy").
- [ ] Add a note to the feature doc about this risk.

---

## Q6

**Severity**: 🟡 Important  
**Status**: Open  
**Title**: Two-phase shutdown not discussed for Kubernetes rolling deployments

### Description

The research doc (§4.5) describes a pattern used by Vector and other production
services: before draining connections, **first mark the service as unhealthy**.
This causes load balancers and Kubernetes readiness probes to stop routing new
traffic to this instance during the drain window.

Without this, during a rolling deployment:

1. Kubernetes sends `SIGTERM` to the old pod.
2. The tracker starts shutting down but is still accepting new connections.
3. New BitTorrent clients connect and their announce/scrape requests are
   immediately dropped when the tracker exits.
4. Clients see unexplained errors during the deployment window.

For a BitTorrent tracker, the impact depends on client behavior:

- HTTP clients get a connection error or incomplete response.
- UDP clients get no response (UDP is fire-and-forget anyway).

The feature doc's K8s section says "The tracker drains active connections and
exits cleanly" but does not address whether the tracker stops accepting **new**
connections during the drain period.

### Question to answer

Should the tracker implement a pre-drain "unhealthy" phase? If so:

- Should the Health Check API start returning 503 immediately on shutdown signal?
- Is this needed for the initial SIGTERM sub-issue or is it a separate concern?

### Action

- [ ] Decide whether the Health Check API should return 503 during shutdown.
- [ ] If yes: add a sub-issue or note in the EPIC for "Mark Health Check as
      unhealthy during shutdown".
- [ ] If no: add a note to the feature doc explaining why this is acceptable
      (e.g., UDP clients retry anyway, HTTP clients are rare in BitTorrent usage).

---

## Q7

**Severity**: 🟡 Important  
**Status**: Open  
**Title**: `#[cfg(unix)]` asymmetry on Windows not noted

### Description

The recommended `SIGTERM` implementation (research doc §5.1 and feature doc design)
requires `#[cfg(unix)]` because `SIGTERM` does not exist on Windows:

```rust
#[cfg(unix)]
let mut sigterm = signal(SignalKind::terminate())
    .expect("failed to install SIGTERM handler");

tokio::select! {
    _ = ctrl_c => { ... }
    #[cfg(unix)]
    _ = sigterm.recv() => { ... }
}
```

On Windows, the `select!` only has `ctrl_c`. This is correct behavior — `kill`
(in the Windows sense, via Task Manager or `taskkill.exe`) sends `WM_CLOSE` or
terminates directly, and `ctrl_c()` already catches Ctrl+C, Ctrl+Break, and
console close events on Windows.

However, the feature doc and EPIC spec make no mention of Windows behavior. The
EPIC "Out of Scope" section says "Windows-specific signal handling beyond what
Tokio provides" which is fine, but neither document explains what that means
concretely for Windows users of the tracker.

### Question to answer

Is the current Windows behavior (only `ctrl_c()`) acceptable and documented
enough? Or should the feature doc at least note what happens on Windows?

### Action

- [ ] Add a brief Windows note to the feature doc explaining that `SIGTERM`
      handling is Unix-only and that `ctrl_c()` covers the relevant Windows
      termination events.

---

## Q8

**Severity**: 🟢 Minor  
**Status**: Open  
**Title**: `SIGHUP` / config reload not explicitly deferred

### Description

The research doc (§4.2) lists `SIGHUP` as commonly used for configuration reload
in daemons. The feature doc and EPIC make no decision about it — it is neither
in scope nor explicitly out of scope. Operators familiar with Unix daemons may
expect `SIGHUP` to trigger a config reload.

### Question to answer

Should `SIGHUP` handling (config reload without restart) be explicitly deferred
to a separate future feature, or explicitly excluded from this tracker's roadmap?

### Action

- [ ] Add `SIGHUP` to the feature doc "Out of Scope" section with a note:
      either "deferred to a future 'hot reload' feature" or "not planned — restart
      the tracker to apply configuration changes".

---

## Q9

**Severity**: 🟢 Minor  
**Status**: Open  
**Title**: Docker experimental validation missing

### Description

The analysis doc (§8) validates SIGTERM and SIGINT by sending signals directly to
the binary. However, the most common production scenario — `docker stop` — was
not tested. It is possible (as noted in Q4) that Docker's 10s default would
SIGKILL the tracker before shutdown completes, even after the SIGTERM fix.

### Action

- [ ] After implementing the SIGTERM handler, run `docker stop` against the
      containerized tracker and verify it exits cleanly within the default 10s.
- [ ] Document the result in the analysis doc (§8) as a new experiment.
- [ ] If the default 10s is insufficient, update Q4's recommendation accordingly.

---

## Q10

**Severity**: 🟢 Minor  
**Status**: Open  
**Title**: Option 4 heading/body mismatch after signal rename

### Description

In the feature doc "Shutdown Triggers" section, Option 4 was renamed from
"Custom Signals (SIGUSR1 / SIGUSR2)" to "Custom Signals" to avoid cspell issues.
However, the body still refers to `SIGUSR1` and `SIGUSR2` by name. The heading
no longer tells the reader which signals are being discussed.

### Action

- [ ] Restore the signal names to the heading: "Option 4: Custom Signals
      (`SIGUSR1` / `SIGUSR2`)" — the cspell issue is solved by the `SIGUSR` entry
      in `project-words.txt`, so the full name can now be used in the heading.
