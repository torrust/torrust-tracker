---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/architecture/README.md
    - docs/architecture/application-jobs.md
    - docs/architecture/events.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - issue #2410
    - issue #2413
    - src/app.rs
    - src/main.rs
    - src/bootstrap/jobs/manager.rs
---

# Runtime Architecture Glossary

This glossary defines the terms the tracker's runtime-architecture and shutdown
documents use. Each term has one definition here. Other documents link to this
file instead of redefining a term.

The glossary describes the current code unless an entry is marked **Planned**.
A planned entry belongs to the shutdown-order design of sub-EPIC #2410 (SI-22),
which is not implemented yet. This file does not describe the event topology.
[`events.md`](events.md) owns it, and the
[task inventory](../features/shutdown-process/task-inventory.md#event-producer-consumer-inventory)
lists every producer and consumer.

## Events

- **Event**: an objective fact that a component publishes, such as "announce
  handled" or "peer download completed". See the
  [events-are-objective-facts ADR](../adrs/20260727000000_events_are_objective_facts.md).
- **Event bus**: an in-process broadcast channel for one family of events. The
  tracker has four: swarm coordination, http-core, udp-core, and udp-server.
  A bus delivers an event only to receivers that existed when it was sent. It
  buffers up to 65,536 events. A receiver that falls further behind lags and
  skips the oldest events.
- **Producer**: a component that publishes events on a bus, directly or
  through the services it uses. A producer must stop before every consumer of
  the buses it publishes on.
- **Consumer** (event listener): a component that receives events from a bus
  and applies them: statistics, persisted completed-download counts, or
  banning. A consumer must start before, and stop after, every producer of the
  buses it consumes from.
- **Relay**: a component that is both a consumer and a producer. No listener
  in the task inventory publishes events, so there is none today. Event flow
  must have no cycles.
- **Loss window (a)**: events already queued in a listener's channel when the
  listener stops. Today a cancelled listener returns at once and drops them.
- **Loss window (b)**: events published after every consumer of their bus has
  stopped, for example by a request that is still draining. Today they are
  dropped.

## Components and ownership

- **Service**: a Rust value created with its dependencies passed to its
  constructor, such as a repository or the announce service, and held in an
  application or instance container. A service may be shared between instances
  or owned by one. It has no lifecycle, never decides when to stop, and is not
  part of the shutdown tree.
- **Component**: something that runs until it is asked to stop, drains, and
  reports one outcome: a listener, a server, or a periodic job. It receives
  exactly one stop request. It joins or deliberately aborts every child task
  before it reports. It publishes no events after it reports. It never
  subscribes to OS signals.
- **Task**: a Tokio task, where code executes. A task can only finish or be
  aborted. It is never a node of the stop-order graph.
- **Top-level component** (also called a **direct component**): a component
  that `JobManager` supervises directly. Today `JobManager::spawn` runs each
  one on exactly one task in its `JoinSet`. It is the unit of independent
  shutdown, and the only kind of node in the stop-order graph (the boundary
  rule).
- **Sub-component**: a component owned by another component. Its parent stops
  and joins it. A sub-component that depends on anything outside its parent is
  promoted to a top-level component.
- **Job**: the name the code uses for a top-level component registered in
  `JobManager` (`JobManager::spawn`, `JobOutcome`). Prefer "component" in new
  documents.
- **Legacy job**: a pre-spawned periodic job registered with
  `JobManager::register_legacy`. `JobManager` keeps its `JoinHandle` outside
  the `JoinSet` and gives it no token of its own. Today only `udp_ban_cleanup`
  is one, until it gets its own periodic-job migration.
- **Owner**: the component or container that is responsible for stopping and
  joining a task. `JobManager` owns the top-level component tasks and the
  retained legacy handles. A top-level component owns any nested server task it
  starts.
- **Instance**: a configured UDP or HTTP listener, such as one element of
  `[[udp_trackers]]`.
- **Unmanaged task** (also **orphaned task**): a task that can outlive the
  component that logically owns it, so it retains resources and makes shutdown
  unreliable. Tokio tasks are not operating-system processes, so these are not
  zombie processes.
- **`JobManager`**: the application supervisor in `src/bootstrap/jobs/manager.rs`.
  It owns top-level components only, not their children. It applies the shared
  deadline, then aborts and joins what remains. **Planned**: it also issues each
  component's own stop token and enforces the stop order.
- **Composition root** (bootstrap): `src/app.rs` and `src/bootstrap/`, where
  services and components are created and wired. It is the only place that
  knows both producers and consumers, so it is the only place that declares the
  stop order.

## Shutdown

- **Stop request** (cancellation token): a `CancellationToken` that is
  cancelled to ask a component to stop. It is only a request and does not prove
  that the component stopped. Dropping a `CancellationToken` does not cancel
  it, so an owner that can be dropped without calling its `stop()` holds a
  `DropGuard` (issues #2459 and #2471).
- **Completion** (outcome): proof that a component stopped. Its top-level task
  returned or was aborted, and `JobManager` recorded a `JobStatus`:
  `Completed`, `Cancelled`, `Failed`, `Panicked`, or `Aborted`. **Planned**: a
  producer's completion is what allows its consumers to be cancelled.
- **Drain**: the work a component does after a stop request: an HTTP or UDP
  server finishes the requests it already accepted, and **Planned** a listener
  processes the events queued for it. What a drain timeout does when it expires
  depends on the component (see **Drain timeout**).
- **Drain timeout**: a component-specific drain policy. Two kinds exist, and
  they differ at expiry:
  - A **server drain deadline** is a hard limit. The HTTP server forces shutdown
    when its 90-second timeout passes, and the UDP server aborts the request
    processors still running when its 5-second deadline passes.
  - **Planned**: a **listener admission timeout** is checked only between
    events, so it never interrupts a handler that is running. Each listener gets
    its budget injected through its constructor, and a later issue (SI-20) aligns
    the aggregate budgets.

  Every drain timeout is still bounded by the shared deadline, which aborts
  whatever is left, including a running handler.
- **Shared deadline**: the single deadline `JobManager::wait_for_all` applies to
  all top-level components together. `main` passes 10 seconds today, and SI-20
  makes it configurable. When it expires, `JobManager` aborts the remaining
  components.
- **Abort**: a forced stop of a task (`JoinHandle::abort`). It takes effect
  when the runtime can next cancel the task, so it is not synchronous
  preemption. Join the task to observe that it ended. The outcome of an
  interrupted I/O operation can be unknown.
- **Graceful shutdown**: cooperative completion of every top-level component
  before the shared deadline, with no forced abort. Completion alone does not
  prove that every event was persisted, so read the drain and failure logs.
- **Stop-before edge** and **predecessor**: "P stops before C" is a stop-before
  edge, and P is a predecessor of C. **Planned**: an edge may point only to
  components that are already registered, so the graph has no cycles.
- **Event-flow registry** (`EventFlows`): **Planned**: a registry in
  `src/bootstrap/` that records which components publish on and consume from
  each bus. It derives the stop-before edges, and it rejects a consumer
  registered after a producer of its bus.
