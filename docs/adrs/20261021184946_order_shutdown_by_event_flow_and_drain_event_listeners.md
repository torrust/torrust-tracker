---
semantic-links:
  skill-links:
    - create-adr
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - docs/issues/closed/2413-2410-si-22-1-document-event-flows-and-draft-adr/ISSUE.md
    - docs/architecture/glossary.md
    - docs/architecture/events.md
    - docs/architecture/application-jobs.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - packages/events/src/receiver.rs
    - src/app.rs
    - src/bootstrap/jobs/manager.rs
---

<!-- skill-link: create-adr -->

# Order Shutdown by Event Flow and Drain Event Listeners

> Statements marked **read from source, not run** come from reading the code
> and the dependency source on 2026-10-16 and 2026-10-21; no program was
> executed for them. The stop order and the listener drain were implemented
> and verified by sub-EPIC #2410 (issues #2414 to #2416); its design record
> holds the decisions D1 to D20, the discussion record, and the evidence.

## Scope

Repository-level decision, so it belongs in `docs/adrs/`. It spans the
application bootstrap (`src/app.rs`, `src/bootstrap/`), the supervisor
(`src/bootstrap/jobs/manager.rs`), the shared `events` package, and the four
event buses with their seven listeners in the `swarm-coordination-registry`,
`tracker-core`, `http-core`, `udp-core`, and `udp-server` packages. The
decision lives in no single package. Terms are defined in the
[glossary](../architecture/glossary.md); the bus topology is in
[`events.md`](../architecture/events.md) and the
[task inventory](../features/shutdown-process/task-inventory.md#event-producer-consumer-inventory).

This ADR fully supersedes
[Adopt a supervised cancellation tree for shutdown](20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md)
(D11). A partial supersession of its points 2 and 3 would force readers to merge
two documents, so this one restates everything that remains valid.

## Description

The tracker publishes domain events (announce handled, peer download
completed, and so on) on four in-process broadcast channels. Seven listener
components consume them to update statistics, to persist completed-download
counts, and to ban clients. Before this decision, on shutdown
`JobManager::cancel` cancelled one root token and every component saw it at once
(read from source, not run: `src/bootstrap/jobs/manager.rs`).

Before this decision, each listener ran a `tokio::select!` marked `biased` with
the cancellation branch first, and the event handler ran in the body of the
`recv` branch (read from source, not run: all seven listener loops). Once the
token was cancelled the cancellation branch always won, so the listener returned
even when events were queued. Two loss windows followed:

- **Window (a)**: events already queued when a listener stops are dropped.
- **Window (b)**: events published after every consumer of their bus has
  stopped are dropped. HTTP and UDP servers keep serving accepted requests
  while they drain, and those requests still publish.

Both are silent. The client receives a successful answer, and nothing records
that the event's effect was lost. Metrics accuracy is not critical for this
tracker, so bounded loss can be acceptable, but only as an explicit and
observable decision. The reproduction on the real binary (V1 in the sub-EPIC)
measured about 92% of answered `completed` announces missing from the database
when SIGTERM arrived during a burst.

Facts that constrained the design:

- A Tokio broadcast channel delivers an event only to receivers that existed
  when it was sent. Each bus buffers up to 65,536 events (`CHANNEL_CAPACITY`);
  a receiver that falls further behind lags and skips the oldest events.
- Stopping a producer does not close its channel and does not stop its
  consumers. The sender clones live in services and in the `EventBus`, and
  `main` holds the application container until after
  `jobs.wait_for_all(...)` (read from source, not run: `src/main.rs`). A
  consumer needs its own stop request.
- `start_jobs_with_manager` already started every listener before any producer
  (read from source, not run: `src/app.rs`). The order existed only as the
  order of lines, and shutdown ignored it.
- Producers are the HTTP and UDP tracker instances and the torrent cleanup job,
  which publishes `PeerRemoved` and `TorrentRemoved` on the swarm bus through
  the swarm registry. Every publish site awaits `send` inline; none sends from a
  detached task (read from source, not run).
- The `Receiver` trait in `packages/events/src/receiver.rs` exposed only an
  async `recv`.
- `main` gives `JobManager::wait_for_all` a 10-second shared deadline, while
  the HTTP drain timeout is 90 seconds and the UDP request drain is 5 seconds
  (read from source, not run). A slow HTTP drain can therefore consume the
  whole deadline.

## Agreement

Shutdown stops event producers first, and each event listener then drains the
events queued for it within a bounded time. The order is declared once, at the
composition root, from facts about event buses.

### 1. Components, services, and tasks, and the boundary rule

The terms are those of the [glossary](../architecture/glossary.md). A
**service** is a Rust value with constructor dependencies and no lifecycle. A
**component** runs until asked to stop, drains, and reports one outcome. A
**task** is where code executes. A token targets components, never tasks: it
requests a stop, and awaiting task handles is how a component proves it
stopped.

**Boundary rule.** A top-level component is the unit of independent shutdown.
The stop-order graph has top-level components as nodes, never tasks or
services. A sub-component with a dependency outside its parent is promoted to
top level. A component that hosts several consumers stops after the union of
their predecessors. Hosting each component on one top-level task is how things
work today, not part of what a component is.

### 2. The order is known only at the composition root

Producers and consumers never reference each other. Decoupling them is the
reason the event bus exists, and making listeners children of their producers
cannot compile for swarm events (a dependency cycle between packages). Only the
composition root (`src/app.rs`, `src/bootstrap/`) knows both sides, so only it
declares which component stops before which.

### 3. Stop-before edges in `JobManager`

`JobManager` stays a generic supervisor that does not know about events. It
gains per-component tokens and stop-before edges. This changes points 2 and 3 of
the superseded ADR: the root token stops being the only source of tokens.

- `reserve(name)` returns a reservation: a component id and the component's own
  `CancellationToken`, not derived from a shared root token. A consumer
  registered before its producers must not hold a child of a token that its
  producers' cancellation would also cancel.
- `spawn_with(reservation, StopBefore(&ids), runner)` registers the runner and its
  edges; `spawn(reservation, runner)` is the same without edges. The component
  being spawned stops before every id in `StopBefore`. Ids come only from
  earlier reservations of the same manager (an id names its manager and is
  ordered by reservation), so every edge points backwards and the graph has no
  cycle. A reservation dropped without being spawned (for example after a bind
  failure) is released, and no component waits for it.
- `cancel()` cancels every component with no unfinished predecessor, plus the
  legacy root token. It is idempotent.
- `wait_for_all(deadline)` keeps one shared deadline, then aborts and joins what
  remains, with named outcomes. A component is finished when its task has been
  joined there. After shutdown is requested, each completion cancels the
  components whose predecessors have all finished; a predecessor that finished
  before `cancel()` is accounted for at the first join. A failed or panicked
  predecessor releases its dependents once its owned work has stopped, and its
  failure outcome stays visible. Completion before shutdown never cancels a
  consumer by itself. At the deadline every component left, including consumers
  that were never asked to stop, is aborted and joined, and the log names what
  each was still waiting for. `cancel()` logs, per component, that it was asked
  to stop or what it waits for; each release logs which completion released it.
  `waiting_components()` returns the same waiting relation for inspection.
- `register_legacy` keeps today's behavior (the root token, no edges) until
  `udp_ban_cleanup` migrates. The root token is reached only through
  `legacy_cancellation_token()`, so a component cannot pick it up by accident.
- A reservation is spawned at most once, because `spawn` consumes it, and only
  by the manager that issued it: `spawn` returns `SpawnError::ForeignReservation`
  otherwise, and bootstrap propagates that error as a startup failure.
- Registration must not lose a stop request that was already issued: cancellation
  before `reserve`, between `reserve` and `spawn`, and repeated cancellation are
  covered. `spawn_with` rejects, with a `SpawnError` that leaves no edge and no
  running runner and releases the reservation: an id that is not a component of
  this manager (another manager's, a released reservation, never issued)
  (`UnknownStopBeforeTarget`), an id that is the component itself or was reserved
  after it (`StopBeforeTargetNotEarlier`), and any edge once `cancel()` was
  called (`ShutdownRequested`), so no live producer appears behind an
  already-cancelled consumer. Bootstrap propagates these errors as startup
  failures.

### 4. Per-bus declarations in a bootstrap registry (`EventFlows`)

Bootstrap declares, for each component, which buses it publishes on or consumes
from. A small `EventFlows` registry in `src/bootstrap/` turns those
declarations into stop-before edges (option F of the alternatives below):

```rust
pub enum Bus { Swarm, HttpCore, UdpCore, UdpServer }

impl EventFlows {
    /// Fails if `bus` already has a producer: consumers must start first.
    pub fn add_consumer(&mut self, bus: Bus, consumer: ComponentId) -> Result<(), Error>;
    /// Marks `buses` as having a producer and returns their consumers.
    pub fn consumers_of(&mut self, buses: &[Bus]) -> Vec<ComponentId>;
}
```

| Component                                                          | Declaration                                                |
| ------------------------------------------------------------------ | ---------------------------------------------------------- |
| Swarm listener, tracker-core in-memory and persistent listeners    | `add_consumer(Bus::Swarm, ...)`, only when started         |
| http-core listener                                                 | `add_consumer(Bus::HttpCore, ...)`                         |
| udp-core listener                                                  | `add_consumer(Bus::UdpCore, ...)`, starts unconditionally  |
| udp-server statistics and banning listeners                        | `add_consumer(Bus::UdpServer, ...)`, when UDP starts       |
| HTTP tracker instance                                              | `spawn_with(.., StopBefore(&consumers_of(&[HttpCore, Swarm])), ..)` |
| UDP tracker instance                                               | `spawn_with(.., StopBefore(&consumers_of(&[UdpCore, UdpServer, Swarm])), ..)` |
| Torrent cleanup                                                    | `spawn_with(.., StopBefore(&consumers_of(&[Swarm])), ..)`  |
| REST API, health-check API, activity metrics, `udp_ban_cleanup`    | No edges                                                   |

`consumers_of` takes the buses a component publishes on, directly or through
the services it uses, which is why torrent cleanup lists the swarm bus. The
registry states real facts ("publishes on bus B", "consumes from bus B") rather
than their cross product, handles listeners that start only under configuration
by registering only the ones that started, and lets a listener be added with one
line without touching any producer. Plain stop-before edges stay available for
dependencies that are not about events.

### 5. One declaration orders start and stop, and flow has no cycles

Start order is consumers first, because a channel delivers an event only to
receivers that existed when it was sent. Stop order is the reverse. Both come
from the same direction of dependency, not from parent-child ownership. A
component may reference only components already registered, so the stop graph
has no cycle. Event flow must also have no cycle: a cycle (A feeds B, B feeds
A) cannot shut down without loss under any option. A **relay** (a component that
consumes and publishes) is allowed if it is registered as a consumer of its
input bus before it declares its output bus. No relay exists today.

### 6. Subscription and bus lifetime

- Each enabled listener subscribes before any producer of its bus starts. The
  registry rejects a consumer registered after a producer of its bus. This says
  nothing about disabled consumers or buffer overflow.
- The application retains the event buses through listener shutdown. Individual
  sender clones may drop safely; retaining every clone is not required. Even
  dropping the last sender does not discard events still available to a live
  receiver: it reports `Closed` after they are consumed. Dropping the receiver
  abandons its unread events.

### 7. Listener drain with a bounded admission time

On cancellation, each listener keeps processing the events already buffered
until its queue is empty or its drain timeout expires. It logs how many events
it processed. When the timeout expires with events left, it logs a warning
naming the listener and the number left, when known. `Lagged(n)` during the
drain counts as lost events.

- **Per-listener timeout.** Each listener's drain timeout is its own value,
  injected through its constructor. It is not derived at run time from the time
  left in the shared deadline. The default is `DEFAULT_DRAIN_TIMEOUT`, 5 seconds,
  a named constant in `packages/events`. A `release`-build measurement (issue
  #2414) showed that the slowest listener, the persistent one, needs 12 to 14
  seconds to persist a full buffer of 65,536 completed-download events with SQLite
  on tmpfs and 158 to 188 seconds on an NVMe disk, so no timeout that fits the
  10-second shared deadline drains a full buffer. The timeout therefore bounds how
  long a cancelled listener keeps admitting events, and the warning reports what
  is left; batching the writes (issue #2418) is the separate remedy. The default
  stays below the shared deadline and equals the UDP request drain's budget. A
  later issue (SI-20) owns where the value becomes configuration and how it fits
  the process deadline.
- **Admission only.** The drain checks its timeout between events. An event
  taken from the buffer is awaited without racing it against the timeout, so the
  timeout is an admission budget, not a hard bound on a handler. The shared
  deadline stays the final abort. Supervisor abort, a panic, and a processing
  failure remain exceptions to the drain: the abort log names the component but
  cannot quantify an in-progress event's outcome, and queue length cannot count
  an event already received.
  After each handled event the drain yields to the runtime, so a handler that never waits cannot hold
  the worker thread until the timeout expires. On a release build with a handler that does nothing, the
  yield costs about 0.3 to 0.4 microseconds per event, 19 to 26 ms for a full 65,536-event buffer,
  and yielding every 64 events costs about 0.6 ms; the choice of yielding after every event is kept
  because that cost is small next to a handler that locks or does I/O (V12 of the #2414
  manual-verification evidence). The deadline is computed with `checked_add`, so a
  timeout too large for the clock to represent never expires, and does not panic.
- **`Receiver` trait.** The trait in `packages/events/src/receiver.rs` gains a
  non-blocking `try_recv` that returns an event or a new `TryRecvError` (`Empty`,
  `Lagged(n)`, or `Closed`), and `len`, the number of events this receiver has
  not read yet, with a default `is_empty` next to it. A shared drain helper in `packages/events` runs the loop for all
  seven listeners: after cancellation it takes events with `try_recv` until
  `Empty` or the timeout, hands each to the listener's handler, and returns the
  counts of events processed, lost to lag, and left over. One policy and one log
  format follow. The change touches the trait, its seven implementations (one
  production, six scripted test receivers), and the `mockall` mock; the coming
  major release allows it.
- **Log format and wording.** The helper only returns the counts; each listener
  logs them through the `log_drain_report!` macro in `packages/events`, which
  expands in the listener's own package so it uses that package's constant
  `tracing` target (a function cannot take the target: `tracing` needs a constant
  one). Every listener logs the same explicit fields (`listener`, `outcome`,
  `processed`, `lost_to_lag`, `unread_when_timed_out`), never a `Debug`
  rendering, with a warning when the drain lost events and an info line otherwise.
  The counts are **events of every type** on the bus, not completions: the
  persistent listener ignores all but `PeerDownloadCompleted`, so a count of
  unread events does not say how many completed downloads were lost.
- **Tokio semantics the trait relies on** (tokio 1.53.3, the version in
  `Cargo.lock`; **read from source, not run**: `tokio-1.53.3/src/sync/broadcast.rs`):
  - `Receiver::try_recv` returns the next buffered value, or
    `TryRecvError::Lagged(n)` when sends overwrote messages this receiver had not
    read. After `Lagged(n)` the cursor sits at the oldest retained message, and
    the next call returns it. It returns `Empty` when this receiver has read
    everything, and `Closed` only when it has read everything and every sender is
    gone, so buffered values are delivered before `Closed`.
  - `Receiver::len` is the number of messages sent and not yet received by this
    receiver (the send position minus the receiver's position). It includes
    messages already overwritten and no longer readable, until a `Lagged` result
    advances the receiver. It is greater than the buffer length exactly when the
    next receive reports `Lagged`.
  - So `len()` is the unread-position count, not the number still recoverable.
    The drain helper reports it as the leftover count, avoids counting lag twice,
    and its tests cover a timeout that fires before the first lag observation.
    In-progress events and failed side effects are not in the count.

### 8. Test environments

The standalone HTTP and UDP test environments give their servers and their
listeners independent tokens and one `DropGuard` each. `stop()` sets the order
explicitly: cancel and join the server, then cancel and join the listeners.
`stop()` disarms the server guard and cancels that token itself; the listeners
guard moves into the join and is disarmed only once the server stage has ended,
so abandoning `stop()` while the server still drains (for example when its
caller's deadline expires) cannot leave a listener that was never asked to stop.
The listener tasks are then cancelled but not joined: they finish their bounded
drain unjoined, as when the environment is dropped. Dropping an
environment without `stop()` cancels both tokens. Dropping a `CancellationToken`
does not cancel it, so each owner that can be dropped holds a `DropGuard`. Each
token is independent and not derived from the other: a parent-child token tree
would hide the order in the direction of the tree, which the supervisor design
rejects. A server failure must not skip the listener stage, so `stop()` joins
the listeners before it reports any failure. The HTTP environment has one
listener and the UDP environment three, which share one listeners token
(`packages/axum-http-server/src/testing/environment.rs`,
`packages/udp-server/src/testing/environment.rs`).

### 9. What this ADR keeps from the cancellation-tree ADR

These points remain valid and are restated here, so the old ADR can stand as
history:

1. Executable entry points are the only OS-signal boundary. On Unix they map
   `SIGINT` and `SIGTERM`; on Windows they map supported Tokio `ctrl_c()` console
   events. They translate these events into one in-process shutdown request.
2. Every component owns its nested tasks and joins them, or deliberately aborts
   them under a documented bounded policy, before it reports its own outcome.
   This is now load-bearing for event correctness: a component must not publish
   events after it reports.
3. Server libraries expose deterministic in-process lifecycle operations. They do
   not subscribe to OS signals. The existing token-to-`Halted` forwarding remains
   only a temporary compatibility bridge.
4. `Started` remains a one-time startup notification. Shutdown `Halted` channels
   are migration compatibility only.
5. Cancellation flows top-down from owner to child, and completion, failure,
   timeout, and abort outcomes flow bottom-up through awaited handles. Inside a
   component, cancellation still flows from the component to its children. What
   changes is that top-level components get their tokens from `JobManager`
   itself, not as children of one root token (section 3).
6. The deployment and exit-result policy: fully graceful completion exits with
   code `0`; startup or component failure, timeout, panic, or deliberate abort
   exits with code `1`; orchestrators must provide at least 30 seconds, with a
   larger margin recommended where practical. The old ADR also set an initial
   25-second shared process deadline with a 20-second HTTP-family budget and a
   5-second UDP budget; the code today uses 10, 90, and 5 seconds (read from
   source, not run), and SI-20 owns aligning them. This ADR does not change any
   budget. The code does not apply the exit-result policy yet either: `main`
   discards the `Vec<JobOutcome>` that `JobManager::wait_for_all` returns and
   logs a successful shutdown, so a shutdown timeout or abort exits with code 0
   today (`src/main.rs`, `src/bootstrap/jobs/manager.rs`; read from source, not
   run).
7. A supervisor deadline applies concurrently across top-level components, not
   sequentially as a per-job budget.
8. Shared lifecycle APIs follow add, migrate consumers, deprecate, remove. The
   `JobManager` changes of section 3 apply this: `reserve` and `spawn_with` arrive
   beside the previous `spawn`, components migrate one by one, and
   `register_legacy` stays until `udp_ban_cleanup` migrates.
9. Each migration includes deterministic token and lifecycle tests. OS-signal
   and container behavior remain executable-level integration evidence.
10. The task inventory is revisited whenever lifecycle topology changes. The old
    ADR also required revalidating it before issue #1588 closed; that condition
    lapsed because the issue is closed, and the revisit-on-change rule is the
    part that stays.
11. The architecture is implemented progressively, by EPIC #1488. Sub-EPIC #2410
    implements the stop order and the listener drain of this ADR. This ADR does
    not claim that the rest of the cancellation-tree architecture (the aligned
    budgets of SI-20 and the exit-result policy) is implemented yet.
12. The old ADR's rejected alternatives stay rejected:

    - supervisor-owned raw `Halted` senders, because transport-specific channels
      leak into application supervision and do not define component-owned child
      completion;
    - a cancellation token without lifecycle ownership, because cancellation only
      requests stop and proves nothing about completion, failure, or timeout;
    - token-to-oneshot forwarding as the final architecture, because it leaves two
      normal cancellation paths and library OS-signal subscriptions.

## Alternatives Considered

### How to stop producers before consumers

The option letters match the sub-EPIC's design record.

| Option                            | Outcome  | Why                                                                                                                                              |
| --------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| A. Ordered stages                 | Rejected | Sorts components into categories (consumer stage, producer stage) instead of stating relationships. A relay fits in neither group without a new stage. Coarse and global. |
| B. Listeners stop on channel close | Rejected | Unreachable without moving bus ownership out of the package containers, and one leaked sender clone hangs shutdown until the deadline. A cycle never closes.        |
| C. Listeners as children of producers | Rejected | Couples producers to consumers, which defeats the event bus, and cannot compile for swarm events (dependency cycle). Shared listeners break a tree.             |
| D. Stop-before edges only         | Rejected as the declaration layer, kept as the mechanism | Needs the same `JobManager` change as F. With conditional listeners, bootstrap rebuilds a registry by hand, every producer is edited when a consumer is added, and a forgotten edge silently reopens window (b). |
| E. Event-flow owner per channel   | Deferred | The likely long-term direction, but the largest refactor: bus creation leaves four package containers that are being prepared for extraction (#1669). It still needs D or A as a mechanism. |
| **F. Per-bus declarations on D**  | Chosen   | Small to medium change, explicit order per bus, one declaration for start and stop, and the registry can enforce consumers-first, which edges alone cannot. |
| G. Strict reverse registration    | Rejected | The order stays implicit in line order and can change silently. Sequential shutdown is slow. It states no relationships.                          |

A per-topic index inside `JobManager` (opaque topic keys) was also rejected: it
would avoid threading `&mut EventFlows` through about ten `start_*` functions,
but it would teach the supervisor publish and subscribe. If the plumbing proves
painful, prefer a small bootstrap context struct holding the manager and the
registry over moving topics into `JobManager`.

### When to move to an event-flow owner (option E)

Keep option F while the four buses are created inside their package containers.
Move to an event-flow owner when bus creation is restructured, for example when
the `bittorrent-*` packages are extracted (#1669). The registry moves into the
owner without changing the `JobManager` mechanism, because the owner still
produces the same stop-before edges.

### How a stopped listener reads what is left

| Option                                    | Outcome  | Why                                                                                                                              |
| ----------------------------------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------- |
| **A. `try_recv` and `len` in the trait**  | Chosen   | Explicit and deterministic, testable with the existing scripted receivers, and gives an unread-position count with documented lag semantics. Cost: trait change in seven implementations and the mock. |
| B. `recv().now_or_never()` loop           | Rejected | Needs no trait change but relies on the unstated assumption that `recv` is ready whenever an event is buffered, and gives no leftover count. |
| C. Short idle timeout                     | Rejected | Time-based and nondeterministic; may stop while events still arrive.                                                              |

### What a listener does on cancellation

| Option                           | Outcome  | Why                                                                                                       |
| -------------------------------- | -------- | --------------------------------------------------------------------------------------------------------- |
| Stop immediately (today)         | Rejected | Fastest and simplest, but silent and unbounded loss that nobody decided.                                   |
| Drain fully, no timeout          | Rejected | No intentional backlog discard, but unbounded shutdown can hit the orchestrator's `SIGKILL`.                |
| **Drain with timeout, warn on loss** | Chosen | No loss in the normal case, bounded, and visible. Shutdown is slightly longer and loss is still possible, but reported. |

## Consequences

- Window (a) is closed within the drain timeout, and window (b) is closed by the
  stop order. Neither guarantees lossless persistence under buffer overflow,
  handler or database failure, panic, or forced abort. Disabled consumers have no
  delivery guarantee. Drain logs distinguish processed events, lag, and queued
  leftovers.
- A graceful shutdown takes slightly longer: each level of an event chain drains
  within the one shared deadline, so the last levels of a long chain get the least
  time. The drain timeout must fit inside the shared deadline, and SI-20 aligns
  the budgets.
- A large backlog can still be cut short while the persistent listener writes
  one event at a time. Batching those writes is a separate issue, implemented
  after the shutdown overhaul.
- The `Receiver` trait, its seven implementations, and the `mockall` mock change.
  The standalone test environments change to two independent tokens each.
- A forgotten bus in a component's declaration silently reopens window (b) for
  that component. The registry makes the facts explicit and checks that a consumer
  is not registered after a producer of its bus, but it cannot infer which buses a
  producer reaches through its services. Review of new producers must check the
  declaration.
- Bootstrap threads `&mut EventFlows` through the `start_*` functions that
  register components.
- The cancellation-tree ADR is marked as superseded by this ADR and stays as history.

## Affected Code

- [`src/bootstrap/jobs/manager.rs`](../../src/bootstrap/jobs/manager.rs): reservations, `StopBefore` edges, and the ordered shutdown
- [`src/bootstrap/event_flows.rs`](../../src/bootstrap/event_flows.rs): `Bus` and `EventFlows`
- [`src/app.rs`](../../src/app.rs): the declarations of consumers and producers
- [`packages/events/src/drain.rs`](../../packages/events/src/drain.rs): the shared listener drain
- [`packages/axum-http-server/src/testing/environment.rs`](../../packages/axum-http-server/src/testing/environment.rs) and
  [`packages/udp-server/src/testing/environment.rs`](../../packages/udp-server/src/testing/environment.rs): the test environments

## Date

2026-10-21

## References

- EPIC #1488: [Overhaul Tracker Shutdown](../issues/open/1488-overhaul-tracker-shutdown/ISSUE.md)
- Sub-EPIC #2410: [EPIC.md](../issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md), the design record with decisions D1 to D20 and the discussion record (the diagrams are in the [events](../architecture/events.md) and [application jobs](../architecture/application-jobs.md) architecture documents)
- Issue #2413: [SI-22.1 spec](../issues/closed/2413-2410-si-22-1-document-event-flows-and-draft-adr/ISSUE.md)
- [Runtime architecture glossary](../architecture/glossary.md)
- [Events architecture](../architecture/events.md)
- [Application jobs and task ownership](../architecture/application-jobs.md)
- [Shutdown task inventory](../features/shutdown-process/task-inventory.md)
- ADR: [Adopt a supervised cancellation tree for shutdown](20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md) (superseded by this ADR)
- ADR: [Events are objective facts](20260727000000_events_are_objective_facts.md)
- Issues #2414, #2415, and #2416: the implementation, in
  [#2414](../issues/closed/2414-2410-si-22-2-drain-listener-queues-on-shutdown/ISSUE.md),
  [#2415](../issues/closed/2415-2410-si-22-3-per-component-cancellation-tokens/ISSUE.md), and
  [#2416](../issues/open/2416-2410-si-22-4-stop-producers-before-listeners/ISSUE.md)
- Issue #2418: batched persisted download writes
- Issue #1669: extraction of the `bittorrent-*` packages
