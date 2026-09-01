---
doc-type: questions
status: resolved
last-updated-utc: 2026-09-01
semantic-links:
  related-artifacts:
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/research/20260716-console-shutdown-patterns/README.md
---

# Questions: Shutdown Process Feature

This document records the questions, decisions, risks, and gaps identified
during specification of the shutdown process feature. All questions are now
resolved or explicitly deferred with a rationale.

## Progress

| #           | Severity     | Status      | Title                                                    |
| ----------- | ------------ | ----------- | -------------------------------------------------------- |
| [Q1](#q1)   | 🔴 Critical  | ✅ Resolved | `global_shutdown_signal()` removal not tracked           |
| [Q2](#q2)   | 🔴 Critical  | ✅ Resolved | Select the shutdown ownership and signaling architecture |
| [Q3](#q3)   | 🔴 Critical  | ✅ Resolved | Exit-code contract for shutdown outcomes                 |
| [Q4](#q4)   | 🟡 Important | ✅ Resolved | Shutdown deadline hierarchy and deployment minimums      |
| [Q5](#q5)   | 🟡 Important | ✅ Resolved | Process-wrapper signal targeting and forced termination  |
| [Q6](#q6)   | 🟡 Important | ✅ Resolved | Mark health check not ready before draining              |
| [Q7](#q7)   | 🟡 Important | ✅ Resolved | Document supported Windows shutdown boundary             |
| [Q8](#q8)   | 🟢 Minor     | ✅ Resolved | Explicitly defer `SIGHUP` configuration reload           |
| [Q9](#q9)   | 🟢 Minor     | ✅ Resolved | Docker verification uses configured shutdown grace       |
| [Q10](#q10) | 🟢 Minor     | ✅ Resolved | Custom-signal heading identifies discussed signals       |

## Question → Sub-issue Impact

This table shows which sub-issues each decision affects and their current
readiness.

| Question | Affected sub-issues                 | Impact                                                       |
| -------- | ----------------------------------- | ------------------------------------------------------------ |
| Q1 ✅    | SI-1, SI-2, SI-19                   | Defines signal-boundary migration and final legacy removal.  |
| Q2 ✅    | #1586, SI-2, SI-4–SI-5, SI-10–SI-21 | Defines cancellation propagation and child-task ownership.   |
| Q3 ✅    | #1586, SI-19, SI-20                 | Defines exit results from supervisor outcomes                |
| Q4 ✅    | SI-15, SI-19, SI-20                 | Defines component/process/orchestrator deadline hierarchy    |
| Q5 ✅    | SI-18, SI-19                        | Defines process-wrapper scope for deprecation/removal        |
| Q6 ✅    | SI-13, SI-21                        | Defines readiness-before-drain behavior                      |
| Q7 ✅    | SI-1, SI-16, SI-17                  | Defines conditional Unix SIGTERM and Windows console support |
| Q8 ✅    | feature and EPIC scope              | Reload explicitly deferred; graceful restart is required     |
| Q9 ✅    | SI-20                               | Requires configured Docker/Podman validation                 |
| Q10 ✅   | feature doc only                    | Documentation heading corrected; no implementation impact    |

## Sub-issue Readiness

| Sub-issue        | Can start? | Waiting on                                           |
| ---------------- | ---------- | ---------------------------------------------------- |
| SI-1, SI-4, SI-5 | ✅ Yes     | Nothing                                              |
| #1586            | ✅ Yes     | Nothing; #1588 inventory remains supporting evidence |
| SI-2             | ✅ Yes     | Nothing; additive shared API only                    |
| SI-10            | ❌ No      | SI-2                                                 |
| SI-11–SI-13      | ❌ No      | SI-2, SI-10                                          |
| SI-14            | ❌ No      | SI-2                                                 |
| SI-15            | ❌ No      | SI-14                                                |
| SI-16            | ❌ No      | SI-11                                                |
| SI-17            | ❌ No      | SI-14, SI-15                                         |
| SI-18            | ❌ No      | All supported consumers migrated and #1588 evidence  |
| SI-19            | ❌ No      | SI-18 support window and breaking-release approval   |
| SI-20            | ❌ No      | #1586, SI-10–SI-15                                   |
| SI-21            | ❌ No      | SI-13, SI-20                                         |
| SI-3, SI-6–SI-9  | Superseded | See the EPIC roadmap replacements                    |

---

## Q1

**Severity**: 🔴 Critical\\
**Status**: ✅ Resolved (2026-07-16)\\
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
bug in the standalone library API. The HTTP and UDP migrations are tracked in
SI-16 and SI-17, respectively.

**4. Both standalone example binaries should be updated to handle `SIGTERM`.**

They model the intended library usage pattern. If a user copies the example as
a starting point, their binary will have the same SIGTERM gap. SI-16 and SI-17
cover this independently for HTTP and UDP.

### Actions Taken

- [x] Decision recorded: SI-1 and SI-2 are sequential, SI-1 is safe to land first.
- [x] SI-2 already exists in the EPIC sub-issue table with the `torrust-server-lib`
      external dependency noted.
- [x] SI-16 and SI-17 cover `Environment::stop()` abort-vs-cancel and SIGTERM
      for standalone HTTP and UDP examples, respectively.
- [x] Q5 is resolved and is no longer a blocker for SI-2; its process-targeting
      rule remains required for SI-18/SI-19 verification.

---

## Q2

**Severity**: 🔴 Critical\\
**Status**: ✅ Resolved (2026-09-01)\\
**Title**: Select the shutdown ownership and signaling architecture

### Description

The feature doc "Design Ideas" section says:

> "Send halt signal to all servers via oneshot channels."

This is architecturally vague. Currently the `halt_task` (the `Sender<Halted>`)
for each server is owned inside the job closure that was spawned in the bootstrap
job starters (`start_job` functions). The `JobManager` only holds `JoinHandle<()>`
values — it has no reference to the halt senders.

### Alternatives under consideration

The [Shutdown Architecture Examples](shutdown-architecture-examples.md) make
the leading alternatives concrete from `main()` through nested child tasks.

1. **Explicit component controllers owned by the supervisor**: evolve
   `JobManager` into an application supervisor that retains each component's
   managed task and a typed graceful-stop controller. The narrow variant stores
   `Sender<Halted>` values, but the preferred form exposes lifecycle semantics
   rather than a transport-specific channel.
2. **Shared cancellation tree with component-owned graceful stop**: create an
   application root `CancellationToken`, derive child tokens per component, and
   make every long-running component observe its token. Servers perform their
   own graceful drain and join their child controller before completing.
3. **Wrapper token-to-halt forwarding (transition option)**: retain the current
   oneshot protocol and have each managed wrapper forward its cancellation token
   to its private `Halted` sender. This can migrate signal ownership incrementally,
   but leaves two shutdown mechanisms in the design.

These are not merely different ways to route a signal. They assign lifecycle
ownership, API responsibility, test seams, and failure handling differently.

The design doc mentions none of these options. Without a concrete decision here,
developers implementing the sub-issues will face an unguided architectural choice
mid-implementation.

### Question to answer

Which wiring approach should be used? This decision affects the scope and
complexity of at least three sub-issues:

- "Centralize signal handling in `main.rs`"
- "Migrate torrent cleanup to `CancellationToken`"
- Any sub-issue touching Axum server shutdown

### Reopened: prior decision was based on the wrong criterion

Q2 was previously marked resolved by selecting Option 3 because it avoided API
changes and touched fewer packages. That is not a sufficient basis for an
architecture decision. The feature and EPIC do not prohibit breaking changes or
complex refactors. Such changes are costs to plan and mitigate, not reasons to
reject a design that better satisfies correctness, maintainability, readability,
and testability.

Option 3 remains a valid **transitional** candidate, but it is no longer the
approved final architecture.

### Evaluation

The [Preliminary Task Inventory](task-inventory.md) establishes that the
current manager reaches only the event listeners, while server shutdown has
unjoined controller tasks and library-level OS-signal listeners. The
[Shutdown Architecture Examples](shutdown-architecture-examples.md) apply the
following comparison to the tracker binary and standalone HTTP/UDP consumers.

#### Single shutdown authority

- **Option 1 — supervisor controllers**: Strong for tracker-owned components,
  but controllers do not themselves remove unrelated OS-signal listeners.
- **Option 2 — cancellation tree only**: Strong when libraries no longer
  observe OS signals.
- **Option 3 — token-to-halt forwarding**: Partial; retains the legacy
  per-server halt protocol.
- **Recommended target — supervised cancellation tree**: Strong; binaries
  translate OS signals and `JobManager` coordinates application shutdown.

#### One normalized cancellation model

- **Option 1 — supervisor controllers**: Weak; controllers and tokens remain
  separate normal cancellation models.
- **Option 2 — cancellation tree only**: Strong for shutdown requests.
- **Option 3 — token-to-halt forwarding**: Weak; token and oneshot
  cancellation coexist as normal behavior.
- **Recommended target — supervised cancellation tree**: Strong; the token is
  the normal stop request, while component actions are lifecycle details.

#### Library usability

- **Option 1 — supervisor controllers**: Strong if controllers are public and
  typed.
- **Option 2 — cancellation tree only**: Strong for injected-token consumers,
  but a stop-and-wait API is still needed.
- **Option 3 — token-to-halt forwarding**: Weak; callers must understand a
  hidden legacy channel.
- **Recommended target — supervised cancellation tree**: Strong; components
  expose deterministic `stop()` behavior while tokens remain injectable.

#### Testability

- **Option 1 — supervisor controllers**: Strong; tests invoke a controller.
- **Option 2 — cancellation tree only**: Strong; tests cancel an injected
  token.
- **Option 3 — token-to-halt forwarding**: Moderate; tests must assert
  forwarding to an internal channel.
- **Recommended target — supervised cancellation tree**: Strong; tests request
  cancellation and await observable component outcomes.

#### Observability

- **Option 1 — supervisor controllers**: Strong at the top level if controllers
  and task results are retained.
- **Option 2 — cancellation tree only**: Moderate; a token carries no
  completion or state information.
- **Option 3 — token-to-halt forwarding**: Weak; forwarding obscures component
  state and child completion.
- **Recommended target — supervised cancellation tree**: Strong; `JobManager`
  collects named top-level outcomes while components join and report children.

#### Failure behavior

- **Option 1 — supervisor controllers**: Moderate; a separate child-task
  ownership policy is still required.
- **Option 2 — cancellation tree only**: Moderate; completion, timeout, and
  child-task policies remain unspecified.
- **Option 3 — token-to-halt forwarding**: Weak; preserves detached controllers
  and ambiguous channel-loss behavior.
- **Recommended target — supervised cancellation tree**: Strong target; every
  component owns each child and defines join, timeout, or deliberate abort.

#### API quality

- **Option 1 — supervisor controllers**: Better than exposing raw senders, but
  risks a controller API unrelated to cancellation used elsewhere.
- **Option 2 — cancellation tree only**: Minimal but incomplete unless a
  lifecycle API is also added.
- **Option 3 — token-to-halt forwarding**: Poor target API; a transport-specific
  channel remains part of the design.
- **Recommended target — supervised cancellation tree**: Strong; separates a
  generic cancellation request from component-specific lifecycle behavior.

#### Migration cost

- **Option 1 — supervisor controllers**: High; requires controller APIs and
  manager registration.
- **Option 2 — cancellation tree only**: High; server and environment APIs must
  change.
- **Option 3 — token-to-halt forwarding**: Low, but suitable only as a temporary
  compatibility bridge.
- **Recommended target — supervised cancellation tree**: High and accepted;
  the cost is justified by a coherent, correct, and testable lifecycle contract.

### Decision

Adopt the **supervised cancellation tree** as the target architecture.

1. `main()` and other executable entry points are the only OS-signal boundaries.
   They translate `SIGINT` and `SIGTERM` into an in-process shutdown request.
2. `JobManager` remains the tracker application's supervisor. It owns named,
   top-level task handles, requests shutdown through its root
   `CancellationToken`, waits for top-level task outcomes concurrently under an
   overall deadline, and reports each component's completion, failure, or
   timeout.
3. Every long-running component receives a child `CancellationToken`. Token
   cancellation is the normal cooperative stop request across event listeners,
   periodic jobs, and servers.
4. Each component owns its nested tasks. It must join them before reporting
   completion, or deliberately abort them according to a documented policy.
   Detached, indefinite background tasks are not an accepted steady-state
   lifecycle design.
5. Components retain protocol-specific shutdown behavior. For example, HTTP
   servers drain connections and UDP servers apply a documented policy for the
   receive loop and active request processors. A token requests this work; it
   does not replace it.
6. Standalone package consumers receive an in-process lifecycle API such as
   `Environment::stop()` that requests cancellation and awaits component-owned
   work. Libraries do not subscribe to OS signals.

**Propagation and completion rule**: cancellation flows from each owner to its
direct children through `CancellationToken` clones or child tokens. Completion,
failure, and timeout outcomes flow back from children to their owner through
awaited task handles. `JobManager` owns only named top-level component handles;
it does not collect every nested handle. A component cannot report completion
until every child it owns has completed or been deliberately aborted under its
documented policy.

The existing `Started` oneshot reports startup and remains separate. The legacy
`Halted` oneshot is a temporary migration bridge only and will be replaced by
token-based shutdown propagation.

The concrete HTTP and UDP flows are documented in the
[recommended target example](shutdown-architecture-examples.md#recommended-target-jobmanager-supervision-with-a-cancellation-tree).

### Migration Constraints and Deferred Decisions

- Existing `Halted` oneshot channels may be used temporarily to bridge legacy
  implementations, but token-to-oneshot forwarding is not part of the target
  public contract.
- Removing library-level `global_shutdown_signal()` is mandatory before the
  architecture is complete; mixed signal authority must be explicitly bounded
  during migration to avoid shutdown races.
- Q3 defines exit-code semantics. Q4 defines the overall and component
  deadlines. Q5 defines process-crash and orphan-risk behavior. This decision
  defines their ownership boundaries, but does not resolve their policies.
- The exact public types, compatibility policy, and package release sequence
  are implementation-backlog work. Breaking changes are acceptable where they
  materially improve lifecycle semantics.

### Actions Taken

- [x] Compared all Q2 alternatives against every Architecture Decision
      Criterion.
- [x] Selected the supervised cancellation-tree target architecture.
- [x] Defined OS-signal, application-supervision, component, and standalone
      consumer responsibilities.
- [x] Recorded migration constraints and the Q3–Q5 decisions that remain
      intentionally separate.
- [x] Updated SI-2, SI-4, SI-5, and the SI-10–SI-21 replacement drafts with
      the decision and migration plan; SI-3 is superseded.
- [x] Updated the EPIC readiness, dependency, and sequencing notes.

---

## Q3

**Severity**: 🔴 Critical\\
**Status**: ✅ Resolved (2026-09-01)\\
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

### Analysis

The supervised cancellation tree gives `JobManager` named aggregate outcomes.
The process result must distinguish a fully graceful stop from a component that
failed, missed the overall deadline, or needed a deliberate abort. Returning 0
for an incomplete shutdown would falsely tell systemd, containers, CI/CD, and
automation that the tracker stopped cleanly.

The contract should remain intentionally small. It does not need distinct exit
codes for every component or failure subtype because structured logs contain
that diagnostic detail. Standard convention is enough: 0 for success and a
non-zero code for an operational failure. Signal termination is owned by the OS
only when SIGKILL or another signal that cannot be handled prevents the process from
running its shutdown path.

### Decision

Use these process results:

| Situation                                                          | Exit code  | Rationale                                                                                           |
| ------------------------------------------------------------------ | ---------- | --------------------------------------------------------------------------------------------------- |
| All top-level components complete within the overall deadline      | `0`        | The requested graceful shutdown completed.                                                          |
| Any component fails, panics, times out, or is deliberately aborted | `1`        | Shutdown was incomplete or abnormal; supervisors and automation must detect it.                     |
| Startup cannot complete                                            | `1`        | The service was never ready; retain normal Rust error/log diagnostics.                              |
| OS termination that cannot be handled, such as SIGKILL             | OS-defined | The process cannot select an exit result; on Unix the shell commonly reports $128 + signal number$. |

`JobManager` must return structured named outcomes to `main()`. `main()` maps
the aggregate result to this contract after logging the per-component evidence.
The tracker must not call `std::process::exit` from a component task. This keeps
tests deterministic and lets component owners complete their cleanup first.

No third-party exit-code crate is needed. Two stable result classes avoid an
unnecessary dependency and preserve a familiar process-manager contract.

### Consequences

- A timeout is not a successful graceful shutdown, even if the process exits
  voluntarily afterward.
- Systemd `Restart=on-failure` and similar policies may restart after a
  non-zero shutdown result. Operators who intentionally want no restart must
  configure their service policy accordingly; do not list an incomplete
  shutdown in `SuccessExitStatus`.
- Issue #1586 supplies the aggregate outcomes; the final policy/configuration task
  maps them to the documented process result.

### Actions Taken

- [x] Approved code `1` for startup and shutdown failures.
- [x] Approved a deliberate component abort after its deadline as failure.
- [x] Assigned SI-20 to implement the process result and policy configuration.

---

## Q4

**Severity**: 🟡 Important\\
**Status**: ✅ Resolved (2026-09-01)\\
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

### Analysis

The current sequential per-job timeout is not a usable budget model and will be
replaced. The target model has one process-wide monotonic deadline, while each
component receives a smaller budget within it. The outer runtime must reserve
time to deliver SIGTERM, flush logs, and observe the process result; otherwise
it can issue SIGKILL while the tracker is still completing normal cleanup.

The existing 90-second Axum value is incompatible with Docker's default 10
seconds and Kubernetes' default 30 seconds. Conversely, silently requiring all
deployments to use 90 seconds is an unjustified operational cost. The defaults
must fit the common 30-second Kubernetes grace period while leaving a material
outer margin.

### Decision

Use the following deadline hierarchy and defaults:

$$
T_{\text{orchestrator}} \ge T_{\text{process}} + 5\ \text{s}
$$

$$
T_{\text{process}} = 25\ \text{s}
$$

$$
T_{\text{component}} < T_{\text{process}}
$$

The initial component budgets are:

| Budget                            | Default            | Purpose                                                                 |
| --------------------------------- | ------------------ | ----------------------------------------------------------------------- |
| Process shutdown deadline         | 25 seconds         | Concurrent overall time budget for all top-level components.            |
| HTTP/REST/health connection drain | 20 seconds         | Maximum time for active HTTP connections to finish.                     |
| UDP active-request completion     | 5 seconds          | Maximum time for already accepted UDP requests before deliberate abort. |
| Orchestrator grace period         | 30 seconds minimum | External deadline; leaves at least five seconds after tracker shutdown. |

The process deadline is **not** a per-job timeout. All top-level components run
their shutdown concurrently within the same 25-second monotonic deadline.
Component budgets must be less than that deadline; a component may finish early
and return its named outcome. The final policy/configuration task validates the
relationships rather than allowing an impossible configuration.

Deployment guidance belongs in `docs/containers.md` and the final policy task,
with a concise warning in the feature README. Docker/Podman users must set
`stop_grace_period` or `docker stop --time` to at least 30 seconds. Kubernetes
must set `terminationGracePeriodSeconds` to at least 30 seconds. Systemd must
set `TimeoutStopSec` to at least 30 seconds. Operators may choose larger values
when their expected request duration requires them.

### Consequences

- Docker/Podman's 10-second default is insufficient for the default tracker
  policy and must not be represented as a supported graceful-draining setup.
- The 5-second outer margin is the minimum contract. Production guidance should
  recommend a larger margin, such as 35 seconds, where the platform allows it.
- Q6 readiness behavior, if adopted, must use part of the same process budget;
  it cannot add another independent timeout.
- Issue #1586 implements concurrent aggregate outcome collection first. The final
  policy/configuration task wires the approved numeric defaults and validation.

### Actions Taken

- [x] Approved the 25-second process deadline and 20-second HTTP drain default.
- [x] Approved the five-second outer safety margin and 30-second minimum
      orchestrator deadline.
- [x] Approved the five-second UDP active-request completion budget.
- [x] Identified Docker/Podman's default 10-second stop behavior as insufficient.
- [x] Assigned SI-20 to configure and document the policy.

---

## Q5

**Severity**: 🟡 Important\\
**Status**: ✅ Resolved (2026-09-01)\\
**Title**: Process-wrapper signal targeting and forced termination

### Description

The original premise was incorrect. Tokio tasks do not become OS processes and
cannot survive a `SIGKILL` of the tracker process. The kernel terminates the
process, its Tokio runtime, and all of its tasks; it releases the process's
sockets. Retaining `global_shutdown_signal()` in server libraries cannot make
SIGKILL graceful and is not a valid crash-safety mechanism.

The observed port retention had a different cause: a signal was sent to a
`cargo run` launcher process rather than its separate `torrust-tracker` child
process. The child process correctly remained alive because it did not receive
the signal. That is process-tree targeting behavior, not a same-process task
ownership failure.

### Decision

1. Server libraries do **not** retain `global_shutdown_signal()` as a fallback
   for process crashes or SIGKILL. Libraries use only their in-process token
   lifecycle contract.
2. Supported production launch models are the direct tracker binary, a
   container whose tracker is PID 1, and a systemd-managed tracker process.
   Their supervisor owns SIGTERM delivery, deadline enforcement, and SIGKILL if
   required.
3. `cargo run` is a development launcher, not a supported production supervisor.
   Manual verification must signal the actual `torrust-tracker` binary PID, or
   deliberately signal the relevant process group when testing launcher
   behavior.
4. Container and systemd documentation must configure an external grace period
   of at least 30 seconds (Q4) and explain their responsibility for forceful
   process termination after that deadline.

### Verification Rule

- Identify the target before signaling it. For direct local testing, discover
  the tracker binary PID rather than using the parent `cargo` PID.
- For `cargo run` experiments, record the complete process tree and specify
  whether the target is the child binary or its process group.
- For containers and systemd, verify the service/container's declared main
  process receives SIGTERM and let the runtime/service manager enforce the
  configured deadline.

### Actions Taken

- [x] Rejected library-level OS-signal fallback as a SIGKILL/crash strategy.
- [x] Distinguished same-process Tokio ownership from external launcher
      process-tree behavior.
- [x] Defined supported production launch models and manual verification
      targeting requirements.
- [x] Removed Q5 as a blocker for token lifecycle migration and legacy API
      removal; SI-18/SI-19 retain process-wrapper documentation evidence.

---

## Q6

**Severity**: 🟡 Important\\
**Status**: ✅ Resolved (2026-09-01)\\
**Title**: Mark health check not ready before draining

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

### Decision

Adopt a two-phase normal shutdown:

```text
shutdown request → mark not ready → drain existing work → process exits
```

The application sets readiness to not ready before it propagates root-token
cancellation. While it drains, `/health_check` returns HTTP 503 without running
registered-service probes. This lets Kubernetes readiness probes and
readiness-aware load balancers remove the instance from new traffic before the
HTTP, REST API, and health-check server components finish their own drain.

This does not prevent direct clients from opening TCP connections or sending UDP
packets. Protocol components remain responsible for admission and their own
graceful-stop policy. The readiness transition must use no independent timer and
fit inside Q4's 25-second process deadline.

### Actions Taken

- [x] Approved readiness-before-drain and HTTP 503 during shutdown.
- [x] Kept readiness separate from token lifecycle ownership and deadline policy.
- [x] Created SI-21 to implement application-owned readiness state and endpoint
      behavior after SI-13.

---

## Q7

**Severity**: 🟡 Important\\
**Status**: ✅ Resolved (2026-09-01)\\
**Title**: Document supported Windows shutdown boundary

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

### Decision

Use one platform-conditional executable signal boundary:

- **Unix**: executable entry points translate both `SIGINT` and `SIGTERM` into
  the same in-process shutdown request.
- **Windows**: executable entry points translate console control events exposed
  through Tokio `ctrl_c()` into that same shutdown request. There is no Unix
  `SIGTERM` equivalent to add.

Server libraries remain platform-independent: they receive in-process lifecycle
requests and do not subscribe to OS signals. The feature does not add Windows
service-control-manager integration or support a forceful `taskkill` termination
as a graceful shutdown path.

### Actions Taken

- [x] Accepted Tokio `ctrl_c()` console-event support as the Windows graceful
      shutdown boundary.
- [x] Defined Unix `SIGTERM` handling as conditionally compiled at executable
      boundaries only.
- [x] Added the Windows support note to the feature definition.
- [x] Kept Windows service-manager integration and forceful termination out of
      scope; no separate implementation sub-issue is required.

---

## Q8

**Severity**: 🟢 Minor\\
**Status**: ✅ Resolved (2026-09-01)\\
**Title**: Explicitly defer `SIGHUP` configuration reload

### Description

The research doc (§4.2) lists `SIGHUP` as commonly used for configuration reload
in daemons. The feature doc and EPIC make no decision about it — it is neither
in scope nor explicitly out of scope. Operators familiar with Unix daemons may
expect `SIGHUP` to trigger a config reload.

### Decision

`SIGHUP` does not reload configuration and does not trigger shutdown.
Configuration changes require a normal graceful restart. Dynamic configuration
reload is deferred to a future feature with its own atomic validation, rollback,
active-component, observability, and platform-compatibility design.

This shutdown feature remains limited to predictable termination: SIGINT and
Unix SIGTERM at executable boundaries, followed by the cancellation-tree
shutdown process. Adding SIGHUP behavior here would expand operational risk and
blur that termination contract.

### Actions Taken

- [x] Deferred SIGHUP configuration reload to a future feature.
- [x] Defined graceful restart as the required configuration-change procedure.
- [x] Added the deferral to the feature and EPIC out-of-scope lists.
- [x] Confirmed no shutdown sub-issue is needed.

---

## Q9

**Severity**: 🟢 Minor\\
**Status**: ✅ Resolved (2026-09-01)\\
**Title**: Docker verification uses configured shutdown grace

### Description

The analysis doc (§8) validates SIGTERM and SIGINT by sending signals directly to
the binary. However, the most common production scenario — `docker stop` — was
not tested. It is possible (as noted in Q4) that Docker's 10s default would
SIGKILL the tracker before shutdown completes, even after the SIGTERM fix.

### Decision

Docker/Podman verification is required only after the token lifecycle, outcome,
and deadline policy are implemented. It belongs to SI-20's end-to-end evidence,
not SI-1's incremental SIGTERM-boundary verification.

The test must configure an external grace period of at least 30 seconds, such
as `docker run --stop-timeout 30` or Compose `stop_grace_period: 30s`. It must
record the configured value, SIGTERM reception at the tracker boundary, named
component outcomes, and the final process result. Docker/Podman's default
10-second timeout is explicitly insufficient for the approved default policy;
it is not a passing graceful-drain target.

After implementation, record the raw command output and logs in SI-20's
`verification.md`, then add the observed behavior to the shutdown analysis.

### Actions Taken

- [x] Assigned configured Docker/Podman validation to SI-20.
- [x] Rejected Docker/Podman's default 10-second timeout as a validation target.
- [x] Required raw evidence for configured grace, signal receipt, component
      outcomes, and process result.

---

## Q10

**Severity**: 🟢 Minor\\
**Status**: ✅ Resolved (2026-09-01)\\
**Title**: Custom-signal heading identifies discussed signals

### Description

The feature doc's "Shutdown Triggers" Option 4 explains custom Unix signals
and refers to `SIGUSR1` and `SIGUSR2`. The heading must identify those signals
so the option can be understood without reading its full body.

### Decision

Keep the heading as **"Option 4: Custom Signals (SIGUSR1 and SIGUSR2)"**. It
matches the body and passes spelling checks. This is documentation-only and
does not add custom-signal behavior to the feature.

### Actions Taken

- [x] Confirmed the feature heading identifies SIGUSR1 and SIGUSR2.
- [x] Confirmed the heading matches the option body.
- [x] Recorded no implementation or additional sub-issue is required.
