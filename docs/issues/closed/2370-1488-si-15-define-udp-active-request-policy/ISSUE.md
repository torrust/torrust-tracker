---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 1488
github-issue: 2370
spec-path: docs/issues/closed/2370-1488-si-15-define-udp-active-request-policy/ISSUE.md
branch: "2370-1488-si-15-define-udp-active-request-policy"
related-pr: 2382
last-updated-utc: "2026-09-30 12:24"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/request_buffer.rs
    - packages/udp-server/src/server/processor.rs
    - packages/udp-server/src/server/states.rs
    - packages/udp-server/docs/adrs/20260929181216_bound_udp_request_concurrency_with_task_per_request_ring.md
    - packages/udp-server/docs/adrs/20260907152707_keep_oldest_first_udp_request_eviction.md
    - packages/udp-server/docs/adrs/index.md
    - packages/udp-server/README.md
    - src/bootstrap/jobs/udp_tracker.rs
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
    - docs/benchmarking.md
    - docs/skills/semantic-skill-link-convention.md
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/closed/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
    - docs/issues/drafts/1488-si-9-improve-udp-shutdown/ISSUE.md
    - docs/issues/drafts/1488-si-20-configure-shutdown-policy/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2370 - Define UDP Active-Request Shutdown Policy

Parent: [EPIC #1488 - Overhaul: Tracker Shutdown](../../open/1488-overhaul-tracker-shutdown/ISSUE.md)

> **EPIC position**: Roadmap sequence 11 (SI-15). A focused policy and
> implementation change after SI-14 (#2342) made UDP receive-loop ownership
> token-driven and joinable.

## Goal

When a UDP tracker instance is cancelled, it must stop admitting datagrams,
wait up to five seconds for request processors it has already accepted, then
deliberately abort and join any that remain. It logs one summary with
completed, failed, aborted, and overload-evicted counts before the receive
loop returns.

The policy keeps BitTorrent UDP's best-effort semantics while making shutdown
bounded, observable, deterministic, and safe to report upward.

## Background

`run_udp_server_main` spawns one processor task per accepted UDP request and
stores only its `AbortHandle` in the fixed-capacity `ActiveRequests` ring
buffer (50 handles). When the buffer is full, `force_push` applies the
oldest-first eviction policy recorded in the
[UDP request-eviction ADR](../../../../packages/udp-server/docs/adrs/20260907152707_keep_oldest_first_udp_request_eviction.md)
and may abort an older unfinished task. `ActiveRequests::drop` aborts every
retained unfinished task. There is no shutdown deadline, no terminal result
collection, and no completed-versus-aborted outcome count.

After SI-14 (#2342), the receive loop observes its component token between
datagrams, returns `Ok(())` on cancellation, and drops `ActiveRequests`. That
return point is the seam for this task: a drain policy can replace the drop
without restructuring the loop. The same receive loop serves both the
token-aware tracker component and the legacy `Server::start`/`Server::stop`
adapter.

Two facts found while completing this draft (2026-09-29) shape the design:

1. **The ring buffer orphans running processors.** When the buffer is full
   and its oldest handle has completed, `force_push` does not stop after
   freeing that slot. It pops the whole buffer, yields once per still-active
   handle, and keeps only the newest active one: each later active handle
   overwrites the retained one. Every other active handle is dropped, so up to
   48 running processors can lose their only handle in one insertion.
   Dropping an `AbortHandle` neither aborts nor tracks its task. These are not
   OS zombies: Tokio still runs them to completion and frees them. They are
   orphans: no owner can await, count, or abort them; they do not count
   against the 50-request bound; they can outlive the receive loop and hold
   the socket open. The ADR records the one-handle re-entry limit as
   intentional, but not the orphaning or its capacity effect.
2. **Processor tasks cannot return an error.** `Processor::process_request`
   returns `()`. Request-level errors become UDP error responses inside the
   task, and encode or socket-send failures are only logged. A processor task
   can therefore end only as completed, panicked, or aborted. This is not a
   hard constraint: the method is only called by the receive loop and its own
   tests (see D7).

Each processor also holds a clone of the socket `Arc`. The socket closes only
after the runtime drops the last processor, so today its release is prompt but
not awaited.

## Scope

### In Scope

- Component-owned tracking of every spawned UDP request processor, so the UDP
  receive loop can await, abort, and join each one.
- A shutdown drain after admission stops: wait up to the request deadline,
  deliberately abort remaining processors, join them, and count outcomes.
- A private five-second request-deadline constant, as approved in
  [Q4](../../../features/shutdown-process/questions.md#q4).
- Structured `tracing` logs at the key drain points and one outcome summary.
- A `Result` return type for request processor tasks (D7).
- Deterministic tests for completion before the deadline, abort at the
  deadline, panic and error classification, and overload-evicted tasks,
  without OS signals or real-time sleeps.
- UDP throughput evidence at three checkpoints (D6), because the change
  touches the request hot path and the eviction ADR requires performance
  evidence for it.
- A package-local ADR recording the UDP request-handling design, its history,
  evidence, and alternatives, plus bidirectional semantic links between that
  ADR, the implementation, and the documentation (T2).
- Updates to the shutdown task inventory and feature documentation.

### Out of Scope

- Token propagation and receive-loop ownership (SI-14, done).
- The separate manager-owned UDP IP-ban cleanup job.
- HTTP, REST API, health-check API, or standalone UDP environment migration
  (SI-16, SI-17).
- Changing the oldest-first overload eviction decision, buffer capacity, or
  the ring-buffer data structure. Any redesign of UDP overload protection is
  outside SI-15 and EPIC #1488 (D8).
- Operator-configurable deadlines, the process-wide 25-second deadline, and
  exit-code mapping (SI-20).
- New metrics or domain events (see D4).
- Legacy lifecycle API deprecation or removal (SI-18, SI-19).

## Design Decisions

D1, D4, and D5 were decided by the maintainer on 2026-09-29. D2, D3, and D6
were agent recommendations; the maintainer approved them on 2026-09-29 and
stated that before/after benchmarking is critical. The maintainer decided D7,
D8, and the D9 example question on 2026-09-29.

Guiding priority from the maintainer: request handling must stay fast.
Slower start or stop is acceptable, so shutdown work may cost time at stop but
must add as little as possible to each request.

### D1 - Private five-second request deadline (maintainer decision)

The drain uses a private `const` of five seconds, the Q4-approved UDP
active-request budget. SI-20 later replaces it with configuration and
validates its relationship to the process deadline. The drain function takes
its deadline as a parameter so tests control it. Production passes only the
constant.

`main()` currently passes a 10-second grace period to `JobManager`. Five
seconds fits inside that bound. Q4's 25-second process deadline is SI-20 work.

### D2 - Track processors in a component-owned `JoinSet` (recommended)

Spawn each processor into a
`tokio::task::JoinSet<Result<(), ProcessorError>>` owned by the receive loop.
`JoinSet::spawn` returns the `AbortHandle` that is still passed to
`ActiveRequests::force_push`, so the overload eviction decision is unchanged.
The loop reaps finished tasks without blocking (`try_join_next`) so the set's
memory stays bounded by live work. On cancellation the drain awaits the set.
Dropping the set aborts every remaining task, which covers the drop paths.

| Option | Pros | Cons |
| ------ | ---- | ---- |
| **`JoinSet` alongside the ring (recommended)** | Owns every processor, including the ones the ring drops today. Collects panics and cancellations as `JoinError`. Aborts everything on drop. Keeps the ADR eviction path byte-for-byte. Already a Tokio dependency. | Adds per-spawn bookkeeping and a non-blocking reap per datagram on the hot path; needs the D6 before/after measurement. Two structures hold handles to the same tasks. |
| Replace the ring with owned `JoinHandle`s | One structure. | Changes the ADR-governed eviction algorithm and data structure; needs separate analysis and performance evidence. Out of scope. |
| `tokio_util::task::TaskTracker` | Cheap wait-for-all. | Cannot abort tasks or report panics, so it cannot count failed or aborted work. |
| Keep abort handles and poll `is_finished` | No new structure. | Cannot wait efficiently, cannot see panics, and still loses the handles the ring drops. |

### D3 - Outcome classification and component result (recommended)

Classify each processor joined by the drain:

| Processor result during the drain | Counter | Log |
| --------------------------------- | ------- | --- |
| Finished normally | `completed` | none per task |
| Panicked (`JoinError::is_panic`) | `failed` | one new drain-level `error` per task |
| Returned `Err` (D7) | `failed` | existing processor-level `warn`; no duplicate drain-level log |
| Aborted by the drain at the deadline | `aborted` | included in the deadline `warn` |
| Aborted earlier by overload eviction and joined during the drain | `evicted` | none per task; already reported by the `UdpRequestAborted` event |

`completed + failed + aborted + evicted` equals the number of processors the
drain found. Only the drain's own deadline aborts count as `aborted`, so
shutdown-induced and overload-induced aborts stay distinct. The drain
continues after a panic. It always joins every processor before returning,
matching the maintainer's direction to keep control and collect every result.

The UDP component still reports `Cancelled` to `JobManager` when the drain
finishes within its deadline, even when some requests were aborted or panicked.
The summary is logged at `warn` when `failed` or `aborted` is non-zero, and at
`info` otherwise.
Rationale: UDP is best effort and the drain met its own deadline. Q3's
"deliberately aborted means failure" rule applies to top-level components the
supervisor aborts, not to request work a component bounds by design. A
receive-loop error keeps failing the component, as SI-14 defined. SI-20 may
revisit this when it maps outcomes to exit codes.

### D4 - Observability: `tracing` only (maintainer decision)

Log with `tracing` under `UDP_TRACKER_LOG_TARGET` at the key points only:

1. Drain start: active processor count and deadline (`debug`, or `info` when
   the count is non-zero).
2. Each panicked processor (`error`).
3. Deadline reached: remaining processor count before abort (`warn`).
4. One summary: `completed`, `failed`, `aborted`, `evicted`, and elapsed time.

Add no metrics and no domain events. If a later feature or user request
needs metrics, the drain must first emit a domain event and derive metrics
from it, following the existing UDP server event and statistics pattern.
Record that as a future option, not SI-15 work.

### D5 - Failures are counted, not escalated (maintainer decision)

A panicked processor does not stop the drain or fail the component. The drain
waits for the remaining processors, collects every result, and reports the
final summary (D3).

### D7 - Processor task returns `Result` (maintainer decision)

Change `Processor::process_request` to return `Result<(), ProcessorError>`,
where the error covers only failures to finish the work: response encoding
failure and socket send failure. A UDP error response is a correctly handled
request and stays `Ok(())`. Existing log messages stay unchanged.

The maintainer's rule: prefer `Result` over `()` unless `()` is measurably
faster. The change therefore lands as its own commit (T4) and gets its own
benchmark checkpoint (D6, B1) before the `JoinSet` wiring, so its cost is
measured separately from D2's. If B1 shows a regression attributable to D7,
stop and return to the maintainer before continuing.

| Option | Pros | Cons |
| ------ | ---- | ---- |
| **Return `Result<(), ProcessorError>` (chosen)** | `failed` reflects real work failures, not only panics. The value is a small enum stored in the task's output slot: no allocation and no extra await. Only one production caller. | Changes the public signature of `server::processor::Processor`, a breaking change for any external caller of the `udp-server` crate. Normal-operation reaping discards the result, so it adds no new normal-operation log. |
| Keep `()` | No API change. | `failed` means panic only; send failures during shutdown are visible only in per-request logs. |

### D8 - Overload bound stays as designed; redesign is out of scope (maintainer decision)

The fixed 50-request bound is a deliberate overload-protection design, not a
bug. SI-15 changes only ownership: D2 keeps every processor, including those
whose handles the ring drops, in the `JoinSet` so shutdown can await, count,
and abort it. SI-15 does not change the bound, the eviction algorithm, or the
ring. Any redesign is out of scope for SI-15 and for EPIC #1488.

The [request-concurrency ADR](../../../../packages/udp-server/docs/adrs/20260929181216_bound_udp_request_concurrency_with_task_per_request_ring.md)
is the canonical record of this design's history, evidence, alternatives, and
re-evaluation triggers. Future overload redesign work starts from that ADR in a
separate issue or EPIC outside #1488.

### D9 - Test strategy without production test hooks

No test may add a runtime flag, feature-gated branch, or extra check to the
request hot path. Use the seams that already exist, at three levels:

1. **Drain primitive (T3).** Spawn controlled futures into a `JoinSet`: gated
   by a `oneshot`, pending forever, panicking, or already aborted. Use paused
   Tokio time (`#[tokio::test(start_paused = true)]`, which needs the Tokio
   `test-util` feature in `dev-dependencies` only) so the five-second deadline
   elapses instantly and deterministically.
2. **Receive loop with a real processor (T5).** Build the server container
   with a test `Sender` whose `send` future waits on a gate. The handlers
   `await` the `UdpRequestAccepted` event before replying, so a real processor
   stays active until the test opens the gate or the deadline aborts it. The
   stats sender is already an injected `dyn Sender`, so production pays
   nothing extra. Send one real datagram over loopback, cancel the token, and
   assert the summary counts and socket release.
3. **Executable boundary (M1-M3).** The release binary has no injection seam,
   and SI-15 adds none. It proves `completed` counts under load and clean
   release. Deadline aborts are proven by level 2, which uses a real socket and
   the real processor.

**Decision: no example.** The maintainer asked for a test that proves the
deadline abort works, and an example only if a test is impossible. The
level-2 test is possible and deterministic: the gate decides when the
processor may finish, and paused Tokio time decides when the deadline expires,
so no outcome depends on real timing. An example would need the same gated
sender to show an abort, making it the level-2 test in a less checked form, or
real timing, making it non-deterministic. Neither is worth maintaining.

The level-2 gate relies on handlers awaiting the event before replying. If
implementation finds that order is not stable, use another injected
collaborator already in the container and record the choice.

### D6 - Measure UDP throughput at three checkpoints (maintainer decision)

D2 adds work to every accepted datagram, and D7 changes the processor's
return type. The eviction ADR requires equivalent before/after performance
evidence for changes to this path, and the maintainer considers benchmarking
critical. Reuse the SI-14 method (five 30-second `aquatic_udp_load_test` runs
per checkpoint, same machine, build profile, configuration, and load-test
settings) at three checkpoints, all recorded in issue-local
`performance-evidence.md`:

| Checkpoint | Code measured | Compared with | Isolates |
| ---------- | ------------- | ------------- | -------- |
| B0 (T1) | `develop` before any code change | - | Baseline |
| B1 (T4) | D7 processor `Result` only | B0 | Cost of D7 |
| B2 (T5) | D7 plus `JoinSet` wiring and drain | B1 and B0 | Cost of D2 and the total |

The T2 ADR and T3 drain primitive are not on the request path, so they need
no separate checkpoint. SI-14 found this shared desktop detects only large
regressions, so each comparison uses its one-sided bound (AC11). A checkpoint
that misses the bound stops the work until the cause is explained to the
maintainer.

## Implementation Constraints

1. Admission stops before the drain starts. Once the token is cancelled, no
   datagram is received and no processor is spawned.
2. The oldest-first eviction decision in `ActiveRequests::force_push` is
   unchanged; its existing tests pass without modification.
3. Every spawned processor is owned by the receive loop until it is joined or
   aborted. No processor is detached, including those the ring buffer drops.
4. The receive loop returns only after every processor is joined, so the
   socket `Arc` is released before the loop completes.
5. The drain never waits longer than its deadline plus the time Tokio needs to
   deliver the aborts. Aborts are joined, not merely requested.
6. Tests control completion and deadline expiry with channels, an injected
   deadline, or paused Tokio time (D9). No OS signals, real sleeps, external
   network dependencies, or production test hooks.
7. The token-aware and legacy paths share the drain, because they share the
   receive loop. Legacy stops may therefore take up to the deadline instead of
   aborting immediately. The maintainer accepted this; record it as a behavior
   change.
8. Per-request cost is the priority. The hot path gains at most the `JoinSet`
   insertion and one non-blocking reap; any other cost belongs to stop time.
9. Startup and existing receive-loop log messages are unchanged.

## Architectural Decisions

- Related ADRs:
  [supervised cancellation tree](../../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md)
  and the package-local
  [oldest-first UDP request eviction](../../../../packages/udp-server/docs/adrs/20260907152707_keep_oldest_first_udp_request_eviction.md).
- ADRs to create (T2): a package-local ADR in
  `packages/udp-server/docs/adrs/`, provisionally titled "Bound UDP request
  concurrency with a task-per-request abort-handle ring". The maintainer
  called this algorithm the essence of the tracker, and no record explains the
  overall design: the existing eviction ADR covers only the full-buffer
  eviction step. The new ADR records:
  - the design as built: one Tokio task per datagram, the fixed 50-request
    bound, oldest-first abort under overload, and the one-yield fairness
    opportunity;
  - the history and evidence from D8, with links to every PR, issue, and
    commit;
  - its known trade-offs, including orphaned handles and inexact accounting;
  - SI-15's split: the ring admits and evicts, and the `JoinSet` owns
    processors for shutdown;
  - every alternative, classified as rejected with evidence, discussed but not
    evaluated, or never considered, with what evaluating each would require;
  - re-evaluation triggers, such as sustained overload aborts in production
    metrics or a benchmark showing a simpler design is at least as fast.

  The eviction ADR stays valid as a narrower refinement of the new ADR. Neither
  supersedes the other.
- If implementation must change the eviction decision or the ring's role
  beyond D2, stop and record it in a package-local ADR before continuing.

### Semantic Link Map (T2)

The maintainer asked for links that make this algorithm easy to find from any
starting point. Follow the
[semantic link convention](../../../skills/semantic-skill-link-convention.md):
ADR paths are stable and used directly; issue specs are referenced as
`"issue #<number>"` because their paths change as they move to `closed/`.

| From | To | Mechanism |
| ---- | -- | --------- |
| New request-handling ADR | `request_buffer.rs`, `launcher.rs`, `processor.rs`, the drain module, the eviction ADR, the supervised cancellation-tree ADR, `docs/features/shutdown-process/task-inventory.md`, `docs/benchmarking.md`, this issue, and issues #566, #611, #918, and #2149 | `semantic-links.related-artifacts` frontmatter and an `Affected Code` section |
| Eviction ADR | New ADR and this issue | Frontmatter and one body sentence naming the new ADR as its parent decision |
| ADR index | New ADR | Index row |
| `ActiveRequests` in `request_buffer.rs` | New ADR, beside the existing eviction ADR marker | `// ADR:` comment |
| `run_udp_server_main` in `launcher.rs` | New ADR and this issue | `// ADR:` and `// issue: #<number>` comments at the processor spawn and drain call |
| `Processor::process_request` in `processor.rs` | New ADR | `// ADR:` comment |
| Drain module (T3) | New ADR and this issue | `//! ADR:` and `//! issue: #<number>` module comments |
| `packages/udp-server/README.md` | ADR index and new ADR | A short "Design Decisions" section |
| `task-inventory.md` UDP request-processor row | New ADR | Body link and frontmatter |
| This spec | New ADR | Frontmatter and this section |

The existing `issue-spec` markers for `simplify-udp-server-main-loop` stay
unchanged; they point to another draft.

## Design and Ownership Review

- **Interfaces**: the receive loop's signature and `Result` contract are
  unchanged. A package-private drain function takes the
  `JoinSet<Result<(), ProcessorError>>` and a deadline and returns the outcome
  counts. `JobManager` and the bootstrap UDP component see no new type.
- **Ownership**: `JobManager` owns the UDP component; the component owns the
  receive loop through SI-14 `OwnedTask`; `OwnedTask::join(self)` consumes that
  owner, and dropping the component before joining aborts the loop. The receive
  loop owns the processor `JoinSet` and the eviction ring; each processor holds
  a socket `Arc` clone.
- **Normal path**: root token -> component child token -> admission stops ->
  drain waits up to five seconds -> deadline aborts -> join all -> summary ->
  loop returns `Ok(())` -> component reports `Cancelled`.
- **Failure paths**: processor panic or `Err` -> counted and logged, drain
  continues. Receive error -> immediately abort every remaining processor,
  join each termination, then return the original receive error. It does not
  use the five-second graceful deadline, because admission has failed rather
  than received a normal cancellation request. Dropping a `JoinSet` is only
  outer-task-abort escalation; it is not a substitute for joining on a normal
  receive-error return.
- **Startup failure**: retain SI-14's registration rollback: cancel and abort
  the receive loop, join it, release the socket, and return the registration
  error before returning a component future.
- **Drop paths**: the component, receive-loop task, or legacy launcher is
  dropped or aborted during the drain -> the `JoinSet` drops and aborts every
  processor. No task is detached.
- **Deadlines**: the drain's only awaited operation is bounded by D1. It must
  finish before the current 10-second `JobManager` bound, which remains the
  outer escalation.
- **Checkpoint**: design review after the first passing vertical slice (T6),
  before documentation and final verification.

## Bug-Fix Process

This issue is substantively a bug fix as well as a lifecycle policy change:
the current full-buffer path can drop the only `AbortHandle` for live processor
tasks, so they can outlive their receive-loop owner and keep its socket alive.
Use the [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md)
workflow.

1. T1 records the current code-path analysis, real-artifact reproduction
  attempt, environment, and outcome in `manual-verification-evidence.md`.
  The bug is internal: a real UDP client cannot observe a lost task handle.
  If the direct tracker run reaches the trigger but cannot observe the orphan,
  record `Trigger only`; if that is infeasible, record the attempted command,
  constraint, and strongest substitute evidence.
2. T3 adds the smallest maintained regression test at the receive-loop and
  processor-owner collaboration boundary. It must fail against the current
  ring-only owner before the production fix. Record the red command and
  output in the evidence file.
3. T5 implements the owner and drain, records green focused-test output, and
  repeats the T1 artifact scenario or its documented strongest substitute.

## Regression Test Strategy

The causal defect is not a `request_buffer` unit decision: it is observable
only when `force_push` drops a live handle and the receive loop later exits.
The smallest maintained boundary is therefore the D9 level-2 receive-loop and
processor-owner collaboration test. It fills the ring into the orphaning
ordering, keeps the affected real processor gated, cancels the loop, and
asserts that the loop does not return until that processor is joined or
deliberately aborted. T3 first proves the test red against the current code;
T5 proves it green after ownership wiring. Mutate the fix in the working tree
without staging it, observe the failure, then restore it.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Analyze and reproduce the orphaned-processor bug; record benchmark B0 | Confirmed the receive-loop, ring, and processor facts on the branch base. The direct-artifact run reached the eviction trigger 94 times but cannot expose lost handles, so it is `Trigger only`; evidence is in `manual-verification-evidence.md`. Recorded isolated, nonpersistent B0: mean 159413.39 responses/s, median 158734.29, range 152965.92-165129.04. |
| T2 | DONE | Request-handling ADR and semantic links | Created `20260929181216_bound_udp_request_concurrency_with_task_per_request_ring.md`, registered it in the package index, linked it bidirectionally with the eviction ADR, and added high-signal source, README, inventory, and issue links. Targeted frontmatter validation plus Markdown, spelling, and local-link checks pass. Later tasks add links for newly created code. |
| T3 | DONE | Drain primitive and red orphaned-processor regression test | Added package-private `drain_request_processors` and `RequestDrainOutcome` in `launcher.rs`, generic over the processor error so T4 can supply `ProcessorError`. Five paused-time unit tests cover completion, deadline abort, `Err` and panic as `failed` with the drain continuing, pre-drain cancellation as `evicted`, and an empty set. The receive-loop collaboration test is red 10 of 10 runs with 48 orphans, committed with `#[ignore]` until T5. Named the ring capacity `ACTIVE_REQUESTS_CAPACITY` without changing behavior. Tokio `time` is now a regular feature because the drain uses `tokio::time::timeout`; `test-util` is a dev-dependency only. |
| T4 | DONE | Processor returns `Result` (D7) and benchmark B1 | `process_request` returns `Result<(), ProcessorError>`, with `EncodeResponse` and `SendResponse` variants; the receive loop still discards it. Existing log messages are unchanged. Processor tests assert `Ok` for a discarded request and for UDP error responses, and a new test proves a failed send returns `SendResponse` (IPv4 socket answering an IPv6 client). B1 mean 164634.37 responses/s, above the lowest B0 run (152965.92). |
| T5 | DONE | Wire the drain into the receive loop, green regression, and benchmark B2 | The receive loop spawns every processor into a loop-owned `JoinSet<Result<(), ProcessorError>>`, passes the `AbortHandle` to the unchanged `force_push`, and reaps finished tasks with `try_join_next` before each spawn. On cancellation it drains with the private 5-second `REQUEST_DRAIN_DEADLINE` and logs the D4 summary: `warn` if any processor failed or was aborted, otherwise `info`. On a receive error it calls `JoinSet::shutdown` before returning the original error. The deadline is injected through the package-private `start_receive_loop_with_request_drain_deadline`, and production passes the constant. Regression is green 10 of 10 runs and mutation-proven; a companion test proves a released processor answers before the loop returns. B2 mean 167716.20 responses/s passes AC11. The T1 artifact recheck drained `completed=6` after 408 evictions. |
| T6 | DONE | Review the first passing vertical slice | No ownership, drop-path, receive-error, log-level, or component-outcome correction needed. Two semantic-link corrections were committed: ADR/issue markers on the drain primitive, and the ADR's Affected Code entry, which no longer calls the shutdown owner "future". |
| T7 | DONE | Update shutdown documentation | `task-inventory.md`: the ownership tree, flowchart, overview row, UDP-instance and request-processor notes, and finding 7 now describe the loop-owned `JoinSet` and the five-second drain. EPIC row 11 now names the delivered scope; it stays `Open` until close-out. The feature README only restates the Q4 five-second UDP budget, which the implementation now matches, so it is unchanged. |
| T8 | DONE | Executable-boundary verification | M1-M4 recorded in `manual-verification-evidence.md`: every run exits 0 with a reconciled drain summary and immediate rebind; M4 records the known SI-17 panic. |
| T9 | DONE | Acceptance and completion review | Independent Task Reviewer reports in `agent-review-reports.md`; findings 1-8 and 11-13 resolved, 9-10 accepted; retrospective recorded. Final re-review (07:58 UTC): `REVIEW PASSED`. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Bug reproduction evidence and benchmark B0 | Commit before any code change. |
| T2 | Request-handling ADR, index row, and semantic links | One `docs(adrs)` commit; code comments only, no behavior change. |
| T3 | Drain primitive, its unit tests, and red regression evidence | Commit after focused validation and test-design review. |
| T4 | D7 processor `Result`, its tests, and B1 | Separate commit, so D7 can be measured and reverted on its own. |
| T5 | Receive-loop wiring, its tests, and B2 | Separate commit, so the wiring can be reverted without the primitive or D7. |
| T6 | Material ownership correction only | Commit substantive corrections separately; otherwise record a no-change decision. |
| T7 | Documentation updates | One `docs(...)` commit. |
| T8-T9 | Evidence and completion review | Commit with the final evidence. |

For each test-producing increment, use the `write-unit-test` skill: write
temporary Arrange/Act/Assert prose, refactor until the test body expresses it,
remove redundant prose, and record the review in the progress log. Each test
exposes its one causal initial-state difference, keeps the production Act
visible, and states its expected result independently. Complete this review
for each test-producing task before starting the next. Stop for maintainer
review after the final test increment, before final verification and the PR.
Sign every commit with GPG and use the `udp-server` scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2370 created, linked as a sub-issue of EPIC #1488, and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-28 UTC - GitHub Copilot - SI-14 (#2342) merged; added the receive-loop seam note to this draft.
- 2026-09-29 07:44 UTC - GitHub Copilot - Completed the draft on provisional branch `chore/udp-active-request-policy-spec`. Maintainer decided D1 (private five-second constant), D4 (`tracing` only), and D5 (count failures, continue the drain). Found that the ring buffer can detach live processors and that processor tasks return `()`; recorded both in Background and proposed D2, D3, and D6 for approval.
- 2026-09-29 08:04 UTC - GitHub Copilot - Maintainer approved D2, D3, and D6, called before/after benchmarking critical, and accepted slower stops in favor of fast request handling. Clarified that a full-buffer insertion can orphan up to 48 running processors; added pending D7 (processor result type) and D8 (separate capacity-bookkeeping bug), and D9 (test strategy with no production hooks, using the injected event sender to hold a real processor open).
- 2026-09-29 10:50 UTC - GitHub Copilot - Maintainer decided D7 (`Result`, benchmarked on its own), D8 (the 50-request bound is a design decision, not a bug; any redesign is outside SI-15 and EPIC #1488), and no example (the deterministic level-2 test is the proof). Researched the ring buffer's history (PRs #644, #873, #921, #922; issues #566, #918; commits `72c83485`, `9e01f7fa`, `84cc1a1d`) and recorded it with future alternatives in D8. Split the plan so B0, B1 (D7), and B2 (`JoinSet`) are separate benchmark checkpoints.
- 2026-09-29 11:10 UTC - GitHub Copilot - Maintainer approved adding a package-local ADR for the UDP request-handling design as task T2, because this algorithm is critical to the tracker, and asked for semantic links between ADRs, implementation, and docs. Added the ADR scope, the Semantic Link Map, AC15-AC16, and renumbered later tasks. Maintainer approved creating the GitHub issue and spec-only PR.
- 2026-09-29 11:15 UTC - GitHub Copilot - Created GitHub issue #2370 and linked it as a sub-issue of EPIC #1488. Renamed the local branch to `2370-1488-si-15-define-udp-active-request-policy-spec`, promoted this spec to `docs/issues/open/`, removed the superseded `verification.md` placeholder, and updated live references. Next step: spec-only PR.
- 2026-09-29 11:17 UTC - GitHub Copilot - Compared the prior SI-15 hand-off with this specification. Preserved its explicit SI-14 `OwnedTask` and registration-rollback guarantees, and corrected D2 to own `Result<(), ProcessorError>` in the `JoinSet`, which is required for D7 error classification.
- 2026-09-29 13:15 UTC - GitHub Copilot - Addressed PR #2372 Copilot findings F1-F7: adopted the semantic bug-fix workflow and evidence, require immediate abort-and-join before a receive-error return, reconcile rather than assume an idle summary is zero, distinguish panic and returned-error logs, include the `evicted` goal counter, and replace movable issue paths in the long-lived eviction ADR.
- 2026-09-29 17:07 UTC - GitHub Copilot - Started implementation on branch `2370-1488-si-15-define-udp-active-request-policy`. T1 is in progress: release tracker build passed; B0 will use the saved 30-second `b0-load-test.toml` configuration and the clean Aquatic `master` checkout recorded in `performance-evidence.md`.
- 2026-09-29 18:03 UTC - GitHub Copilot - Completed T1. The direct release-binary traffic attempt reached the `force_push` overload eviction log 94 times, but no public artifact exposes whether a later live handle was orphaned; recorded `Trigger only` and deferred proof to the T3/T5 collaboration regression test. Replaced the initial shared-SQLite benchmark attempt, which accumulated state and invalidated later runs, with isolated nonpersistent tracker processes. B0 five-run baseline: mean 159413.39 responses/s, median 158734.29, range 152965.92-165129.04.
- 2026-09-29 18:14 UTC - GitHub Copilot - Completed T2. Created the package-local request-concurrency ADR, moved D8 history and alternatives into it, registered it in the package index, and added the planned high-signal semantic links. Targeted frontmatter validation, Markdown, spelling, local-link, whitespace, and Rust-format checks pass.
- 2026-09-29 18:48 UTC - GitHub Copilot - Completed T3.
  - Drain primitive: waits up to its deadline, reaps already-finished tasks before aborting so pre-drain cancellations stay `evicted`, then aborts and joins the rest as `aborted`. Panics are logged at `error`; returned errors are only counted, since the processor already logs them (D3). It is not wired yet: non-test builds carry a `dead_code` expectation that T5 removes.
  - Regression test: `receive_loop_shutdown` drives a real receive loop on loopback. The injected `Sender` holds processors at `UdpRequestAccepted` and counts them until their futures drop. Red 10 of 10 runs: the loop returned `Ok(())` while 48 spawned processors still ran. This observes the wrong outcome at its nearest seam, so V1 is upgraded from `Trigger only` to `Reproduced`.
  - T5 notes: the test needs a short drain deadline, because the 5-second production constant exceeds the test's 5-second lifecycle bound. Pass the deadline into the receive loop, with production supplying the D1 constant, then remove `#[ignore]`.
  - Prose-first review: each drain test's Arrange spawns exactly the processor kinds that select its counter (finished, pending, `Err`, panic, pre-aborted, none), the Act is the visible `drain_request_processors` call, and the Assert is an independently written `RequestDrainOutcome`. The regression scenario fixture `ReceiveLoopWithOrphanedProcessors` owns the ordering mechanics (released oldest request, 49 held, one more held insert); the test keeps the cancellation Act and the running-processor assertion visible, and its message states the count and loop result.
  - Validation (nightly Rust toolchain): `cargo test -p torrust-tracker-udp-server` 187 passed, 1 ignored; `cargo +nightly fmt --all -- --check`; `linter clippy`.
- 2026-09-29 19:05 UTC - GitHub Copilot - Completed T4. Added `ProcessorError` (`EncodeResponse`, `SendResponse`) and made `process_request` return `Result<(), ProcessorError>`; this changes the public `Processor` signature, whose only production caller is the receive loop. Prose-first review: the send-failure test's causal state is an IPv6 client address the IPv4-only processor socket cannot reach, the Act is the visible `process_request`, and the Assert names the expected `SendResponse` variant. B1 (five isolated runs, same machine and settings as B0): mean 164634.37, median 164598.75, range 163440.01-165488.27 responses/s. B1 is above the lowest B0 run, so D7 shows no measurable cost. Validation (nightly Rust toolchain): `cargo test -p torrust-tracker-udp-server` 188 passed, 1 ignored; `cargo +nightly fmt --all -- --check`; `linter clippy`.
- 2026-09-29 21:12 UTC - GitHub Copilot - Completed T5.
  - Wiring: `run_udp_server_main` owns every processor in a `JoinSet<Result<(), ProcessorError>>`, and the ring keeps only overload-eviction decisions. Finished tasks are reaped with `try_join_next` before each spawn so the set stays bounded without blocking the request path. Cancellation drains with the private `REQUEST_DRAIN_DEADLINE` (5 s). A receive error calls `JoinSet::shutdown` and then returns the original error. The T3 `dead_code` expectations are gone. The drain logging is split into start and outcome helpers because the `tracing` macros exceeded the cognitive-complexity limit.
  - Tests: the deadline is injected through the package-private `start_receive_loop_with_request_drain_deadline`, and `start_receive_loop` passes the production constant. The held-processor sender now waits on a `watch` release gate instead of `pending()`, and `ReceiveLoopHoldingRequests` owns the shared mechanics. The regression test no longer carries `#[ignore]`: with a 100 ms deadline it also asserts `Ok(())` and a bindable socket. The new `it_should_let_an_accepted_request_finish_and_answer_before_returning_on_cancellation` releases one held processor after cancellation, with a deadline longer than the test bound, and checks that it answers. Green 10 of 10 runs. Mutation proof: restoring the detached `tokio::task::spawn` by hand (unstaged) brought back `48 request processors ... still running`, and the fix was then restored by hand.
  - Prose-first review: the regression test's causal state is the named fixture `ReceiveLoopWithOrphanedProcessors` (full ring, short drain deadline). The completion test's causal state is `ReceiveLoopWithOneHeldRequest` plus the visible `release_held_processors()` Act step. Both keep the cancellation and join visible, and their assertions state the loop result, the running count, and the socket or response outcome. Drain counters stay asserted in the T3 unit tests rather than being duplicated here.
  - B2 (five isolated runs, same settings): mean 167716.20, median 167329.09, range 167005.40-169580.45 responses/s. This passes AC11 against B0 and B1.
  - Artifact recheck of T1 at info level: after 408 evictions, the `SIGTERM` drain reported `active=6 completed=6 failed=0 aborted=0 evicted=0`, and the tracker logged a successful shutdown.
  - Validation (nightly Rust toolchain): `cargo test -p torrust-tracker-udp-server` 190 passed, 0 ignored; `cargo test -p torrust-tracker --lib udp` 11 passed; `cargo +nightly fmt --all`; `linter clippy`.
- 2026-09-29 21:19 UTC - GitHub Copilot - Completed T6 (design review of the T5 slice).
  - Ownership: every processor is spawned into the loop-owned `JoinSet`; `ActiveRequests` keeps only `AbortHandle`s for overload eviction, and dropping those handles does not abort anything. No spawn path bypasses the set.
  - Drop paths: if the receive-loop task is aborted, for example because the `OwnedReceiveLoop`/`OwnedTask` owner is dropped, the `JoinSet` is dropped and aborts every processor. No task is detached.
  - Receive-error path: `JoinSet::shutdown` aborts and joins every processor before the original error is returned, without the graceful deadline, as the design review requires.
  - Log levels match D3/D4: drain start at `debug`/`info`, a panic `error` per task, a deadline `warn` with the remaining count, and one summary at `warn` when `failed` or `aborted` is non-zero, otherwise `info`. Processors reaped by `try_join_next` during normal operation are discarded silently, as the detached tasks were before; Tokio's panic hook still prints panics. No change.
  - Component outcome: `udp_tracker::start_job` still maps `Ok(())` to `Cancelled` and a loop error or panic to a component error; its existing tests pass unchanged (AC9).
  - Semantic links: added `// ADR:` and `// issue: #2370` markers to `drain_request_processors`. The drain lives in `launcher.rs`, not a separate module, so these are item comments rather than `//!` module comments. Updated the ADR's Affected Code entry for `launcher.rs`, which still called the shutdown owner "future" wiring.
- 2026-09-29 21:25 UTC - GitHub Copilot - Completed T7. Updated the shutdown task inventory, which now shows UDP request processors as loop-owned, joined, and drained within five seconds, with outcome counts. Also updated EPIC row 11's scope text; its status stays `Open` until close-out. The feature README describes the planned Q4 policy (a five-second UDP budget), which the implementation now meets, so it needed no change. Its overall `Status` section is stale across the whole EPIC and is left to the EPIC close-out.
- 2026-09-30 06:10 UTC - GitHub Copilot - Completed T8 against release builds at the T7 commit.
  - M1 (idle): exit 0; the drain reported `active=1 completed=1`; the UDP instance completed after cooperative cancellation; rebind ok.
  - M2 (`SIGTERM` 10 s into an Aquatic load run, 85 evictions): exit 0; the drain reported `active=5 completed=5 failed=0 aborted=0 evicted=0`; in-process shutdown took 1.8 ms.
  - M3: an immediate restart rebinds and serves an announce.
  - M4: the standalone example serves, then exits 101 on `SIGINT` with the known SI-17 panic, unchanged.
  - Harness error: M3's first attempt failed because the load generator still ran after the restart, and its stale cookies got 127.0.0.1 IP-banned. Diagnosed, recorded, and rerun with the generator stopped first.
- 2026-09-30 06:20 UTC - GitHub Copilot (Task Reviewer) - Independent T9 review recorded in `agent-review-reports.md`: `REVIEW FAILED`. Verified and ticked AC1-AC11, AC13, AC14, and AC16. AC12 (receive-error join untested and undocumented) and AC15 (ADR omits the orphaned-handle trade-off) stay `PENDING`; the completion review is not yet recorded. No production or test code changed.
- 2026-09-30 07:27 UTC - GitHub Copilot - Addressed the T9 review findings; re-review pending.
  - F1: added `implementation-retrospective.md`.
  - F2: extracted the receive-error branch into `join_request_processors_after_receive_error` and added `it_should_join_every_processor_before_returning_the_receive_error`. A real receive error cannot be produced reliably on loopback, and D9 forbids production hooks. Replacing `shutdown` with `abort_all` makes the test fail (`running` 1).
  - F4: the panic test now uses a `oneshot`, so the surviving processor finishes only after the panic. A drain that stops after a panic fails it with `completed: 0`. The mutation was proven on the helper form; the panicking processor was then inlined to satisfy the conflicting `unused_async` and `manual_async_fn` lints, with unchanged semantics.
  - F3: the ADR now describes `force_push` precisely, adds Known Trade-offs (inexact bound, up to 48 dropped live handles, summary snapshot semantics), and hyperlinks issues #566, #611, #918, and #2149.
  - F5: the D9 deviation (real-time loopback collaboration tests with a 100 ms injected deadline; counts asserted only in paused-time unit tests) is recorded in the retrospective.
  - F6: noted for SI-17: the test `Environment` stop bound (5 s) equals `REQUEST_DRAIN_DEADLINE`, so a stuck processor would hit the stop panic before the drain aborts it.
  - F7: the task inventory explains the `active` and `evicted` snapshot semantics.
  - F8: removed the duplicated B0 paragraph.
  - F9 and F10 need no change; history is not rewritten.
  - Validation: `cargo test -p torrust-tracker-udp-server --lib` 191 passed; `linter clippy`.
- 2026-09-30 07:52 UTC - GitHub Copilot (Task Reviewer) - Re-review recorded in `agent-review-reports.md`: `REVIEW FAILED`. Findings 1-8 resolved; 9-10 accepted. Reviewer mutations confirmed the receive-error join and the drain-after-panic tests; the receive-error wiring is guarded by the non-test build (`dead_code`). Ticked AC12, AC15, the re-review criterion, and the completion-review checkpoint. New finding 11 blocks T9: no prose-first Arrange-Act-Assert evidence for the two changed tests. Nits 12-13 are optional. No production or test code changed.
- 2026-09-30 07:56 UTC - GitHub Copilot - Addressed re-review findings 11-13; final re-review pending.
  - F11, prose-first review of `it_should_count_failed_processors_and_keep_draining_the_rest`. Prose: "Given a failing processor, a panicking processor, and a processor that finishes only after the panic, when the drain runs, then it counts one completion and two failures." The Arrange shows those three processors directly. Its one comment stays because the causal state is the ordering: the `oneshot` is what forces the drain past the panic, and the spawn lines alone do not show that. The Act is the visible `drain_request_processors` call. The Assert is an independently written `RequestDrainOutcome`.
  - F11, prose-first review of `it_should_join_every_processor_before_returning_the_receive_error`. Prose: "Given a processor that never finishes, when the receive-error path runs, then the error is handed back unchanged and no processor is still running." The Arrange's causal state is the pending processor counted by `RunningProcessor::start`. The Act is the visible `join_request_processors_after_receive_error` call with a named error kind. The asserts state the error kind, a zero running count with a message, and an empty set. No comment is needed.
  - F13: moved one `RunningProcessor` guard into the parent test module, with a `start` constructor that increments the counter. Both `request_drain` and `receive_loop_shutdown` use it, which removes `RunningHeldProcessor`.
  - F12: refreshed `last-updated-utc` in the task inventory and the SI-17 draft.
  - Validation: `cargo test -p torrust-tracker-udp-server --lib` 191 passed; `linter clippy`.
- 2026-09-30 07:58 UTC - GitHub Copilot (Task Reviewer) - Final re-review recorded in `agent-review-reports.md`: `REVIEW PASSED`. Findings 11-13 resolved: the recorded prose matches both test bodies, and the shared `RunningProcessor` guard keeps the increment-before-`held.send` order and the receive-error counter at 1 while pending. A reviewer mutation (`abort_all` instead of `shutdown`) still fails the receive-error test; `launcher.rs` was restored and its diff is byte-identical. Set T9 to `DONE`. Issue closure stays open. No production or test code changed.
- 2026-09-30 09:54 UTC - GitHub Copilot - Addressed PR #2382 Copilot review findings F1-F4. The audit is in `docs/pr-reviews/pr-2382-review/PR-REVIEW.md`.
  - F1 and F2: the ring-capacity doc comments and the request-concurrency ADR no longer say the ring limits running processors or protects memory. It bounds only the abort handles it keeps (D8 unchanged).
  - F4: the new drain start and summary logs use the canonical `service_binding` field. Existing receive-loop logs keep `local_addr` (Implementation Constraint 9).
  - F3: the deadline warning includes `service_binding`. The new paused-time test `it_should_name_the_listener_in_the_deadline_warning` reads the warning from the test-helpers log buffer and needs no production hook (D9). Removing the field, or logging it as `local_addr`, made the test fail.
  - Rebased onto `develop`: the only conflict was upstream's processor test changes, resolved by keeping upstream's assertions and adding the D7 `Ok` checks.
  - Validation (nightly Rust toolchain for formatting and docs): `cargo test -p torrust-tracker-udp-server` 209 passed; `linter clippy`; `linter all`; pre-push checks passed.
- 2026-09-30 10:18 UTC - GitHub Copilot - Maintainer decision: `related-pr` records implementation PR #2382, following SI-14 (`related-pr: 2351`). Spec-only PR #2372 stays in this log because the v1 field holds one value. The maintainer asked for a schema follow-up to allow several related PRs; it is drafted as a #2264 subissue in `docs/issues/drafts/2264-support-multiple-related-prs-in-issue-frontmatter/ISSUE.md`.
- 2026-09-30 11:03 UTC - GitHub Copilot - Maintainer decision on the mixed UDP listener field names left by Copilot finding F4: this PR keeps them, because constraint 9 forbids changing existing receive-loop logs. A standalone follow-up, issue #2387, reviews all startup and shutdown logs for coherence and missing operator information, with a verbatim baseline capture. It is postponed until EPIC #1488 is complete, and its spec merges with this PR.
- 2026-09-30 12:24 UTC - GitHub Copilot - PR #2382 merged into `develop` as merge commit `f911083d`, and GitHub closed the issue as completed. The PR was not linked as the closing PR, because `Closes #2370` was added by editing the body after creation; the issue was closed by the merge commit message, and an issue comment now records the link. Archived this spec to `docs/issues/closed/`, set EPIC #1488 row 11 to Done, and repaired live references to the old path.

## Acceptance Criteria

- [x] AC1: Every spawned UDP request processor is owned by the receive loop until it is joined or aborted, including processors whose abort handles the eviction ring drops.
- [x] AC2: On cancellation the receive loop stops admitting datagrams before the drain starts.
- [x] AC3: The drain waits up to the five-second request deadline; processors that finish in time are counted as `completed`.
- [x] AC4: Processors still running at the deadline are deliberately aborted, joined, and counted as `aborted`.
- [x] AC5: Panicked processors and processors returning `Err` are counted as `failed` and logged; the drain continues and joins every remaining processor.
- [x] AC6: Processors aborted earlier by overload eviction are counted as `evicted`, never as `aborted`.
- [x] AC7: On cancellation or a receive error, the receive loop returns only after every processor is joined, and the UDP socket is released when it returns.
- [x] AC8: One `tracing` summary reports `completed`, `failed`, `aborted`, `evicted`, and elapsed time; no metrics or domain events are added.
- [x] AC9: The UDP component reports `Cancelled` after a drain that finishes within its deadline; a receive-loop error still fails it.
- [x] AC10: `ActiveRequests::force_push` eviction behavior is unchanged and its existing tests pass without modification.
- [x] AC11: At each D6 checkpoint, the mean UDP throughput is not below the lowest run of the checkpoint it is compared with (B1 against B0; B2 against B1 and B0) on the same machine and settings; any drop below it is investigated and explained to the maintainer before continuing.
- [x] AC12: Deterministic tests cover AC3-AC7 without OS signals, real sleeps, or network dependencies.
- [x] AC13: Manual SIGTERM verification under UDP traffic records the `main()` signal event, the drain summary, a clean exit, and immediate UDP rebind.
- [x] AC14: `Processor::process_request` returns `Result<(), ProcessorError>`; encode and send failures are `Err`, and handled requests, including UDP error responses, are `Ok(())`.
- [x] AC15: A package-local ADR records the UDP request-handling design, its history and evidence, the SI-15 ownership split, every alternative with its evaluation status, and re-evaluation triggers; it is registered in the package ADR index and linked from the eviction ADR.
- [x] AC16: Every link in the Semantic Link Map exists in both directions where the map specifies it, and the frontmatter validator and `linter lychee` pass.
- [x] `linter all` exits with code `0`.
- [x] Relevant tests pass.
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation is updated when behavior or workflow changes.

## Verification Plan

### Automatic Checks

- Focused `torrust-tracker-udp-server` tests: the new drain tests,
  `request_buffer` tests, receive-loop and token-aware start tests, and the
  legacy environment and launcher tests.
- Focused `torrust-tracker` `udp_tracker` component and `app` bootstrap tests.
- `linter all`.
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`.
- Pre-push checks.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

Follow the EPIC executable-boundary protocol: signal the direct tracker-binary
PID, bound the wait, capture the exit status and logs, and prove the binding
is released. Record everything in issue-local `manual-verification-evidence.md`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Idle UDP shutdown | Start `target/release/torrust-tracker` with one UDP binding, confirm readiness with `tracker_client udp announce`, wait for the request to finish, then send `SIGTERM` to the binary PID. | The drain summary reconciles every processor observed at shutdown; it may count the completed readiness request if no later datagram reaped it. The component reports `Cancelled`; exit `0`. | DONE | `manual-verification-evidence.md` M1 |
| M2 | Shutdown under UDP load | Run `aquatic_udp_load_test` against the tracker, send `SIGTERM` to the binary PID during the run, and capture bounded exit and logs. | `main()` logs the signal; the summary reports non-zero processors and reconciles its counts; the process exits within the drain deadline plus shutdown overhead. | DONE | `manual-verification-evidence.md` M2 |
| M3 | Listener release | Restart the same configuration immediately after M2 and announce again. | The UDP socket rebinds immediately and serves the announce. | DONE | `manual-verification-evidence.md` M3 |
| M4 | Legacy UDP lifecycle | Run the standalone UDP example or environment start/stop path. | It starts, serves, and stops; any stop delay is bounded by the drain deadline. The known SI-17 Ctrl-C panic is recorded, not fixed. | DONE | `manual-verification-evidence.md` M4 |
| M5 | UDP throughput checkpoints | Follow the UDP load test in `docs/benchmarking.md` with the release build and one saved `aquatic_udp_load_test` config. Run it five times at each D6 checkpoint: B0 (T1), B1 (T4), B2 (T5), on the same machine. | Each checkpoint satisfies AC11. | DONE | `performance-evidence.md` |

Deadline aborts cannot be produced reliably from outside the release binary,
because real processors finish in microseconds and SI-15 adds no production
injection seam (D9). AC4 is proven by the D9 level-2 test, which uses a real
loopback socket and the real processor held open by an injected sender. M2
records whether any `aborted` count appeared under load.

### Disposable Verification Scripts

None planned. If a repeatable direct-PID harness becomes necessary, place it
in this issue directory and record why a maintained Rust test cannot cover it.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | Every spawn goes through the loop-owned `JoinSet` (`launcher.rs` `processors.spawn`); regression test `it_should_not_return_while_processors_orphaned_by_the_request_ring_are_still_running` red 48 orphans before T5, green 10/10 on reviewer re-run, mutation-proven (V2) |
| AC2 | DONE | `biased` `select!` drains inside the cancelled branch, so the receiver is never polled again; SI-14 token-stop tests pass |
| AC3 | DONE | `it_should_count_processors_that_finish_before_the_deadline_as_completed` (paused time); `it_should_let_an_accepted_request_finish_and_answer_before_returning_on_cancellation` |
| AC4 | DONE | `it_should_abort_and_join_processors_still_running_at_the_deadline`; orphan regression test with a never-released gate and 100 ms deadline ends with 0 running processors |
| AC5 | DONE | `it_should_count_failed_processors_and_keep_draining_the_rest` orders the `Ok` processor after the panic with a `oneshot`; reviewer mutation (break after a panic) fails it with `completed: 0`; panic `error` and processor-level `warn`/`error` reviewed |
| AC6 | DONE | `it_should_count_a_processor_aborted_before_the_drain_as_evicted`; deadline path reaps finished tasks before `abort_all` |
| AC7 | DONE | Cancellation: regression test asserts 0 running, `Ok(())`, and bindable socket. Receive error: `it_should_join_every_processor_before_returning_the_receive_error` on `join_request_processors_after_receive_error`; reviewer mutation `abort_all` fails it (`running` 1); bypassing the helper fails the non-test build (`dead_code`) |
| AC8 | DONE | One summary with all four counters and `elapsed`; diff adds no `Event` variant or metric; M1-M2 logs |
| AC9 | DONE | `src/bootstrap/jobs/udp_tracker.rs` unchanged; `cargo test -p torrust-tracker --lib udp` 12 passed; M1-M2 log cooperative cancellation |
| AC10 | DONE | `request_buffer.rs` diff is only `ACTIVE_REQUESTS_CAPACITY` (still 50) and link comments; its tests are unmodified and pass |
| AC11 | DONE | B0 mean 159413.39, lowest 152965.92; B1 mean 164634.37, lowest 163440.01; B2 mean 167716.20; means recomputed by reviewer |
| AC12 | DONE | AC3-AC6 and both AC7 paths covered by paused-time unit tests or gate-decided loopback tests; no OS signals or external network; D9 real-time deviation recorded in `implementation-retrospective.md` |
| AC13 | DONE | M2 records `SIGTERM` log, drain summary `active=5 completed=5`, exit 0; M3 immediate rebind and announce |
| AC14 | DONE | `ProcessorError::{EncodeResponse, SendResponse}`; processor tests assert `Ok` for discarded and error-response requests and `SendResponse` for an unreachable IPv6 client |
| AC15 | DONE | ADR, index row, and eviction-ADR link exist; Agreement 2 matches `force_push`; Known Trade-offs records the inexact bound, up to 48 dropped live handles, the `JoinSet` ownership split, and snapshot counts; history issues hyperlinked |
| AC16 | DONE | Every map entry present (drain uses item-level `// ADR:`/`// issue:` markers, accepted); frontmatter validator and `linter lychee` exit 0 on reviewer re-run |

## Dependencies

- SI-14 (#2342) is merged and provides the cooperative return point.
- Q4 approved the five-second UDP active-request budget. SI-20 later makes it
  configurable and wires the concurrent 25-second process deadline.
- SI-1 (#2132) provides the `SIGTERM` path used by manual verification.

## Rollback

Revert the T5 commit to return the receive loop to ring-only tracking and the
drop-abort fallback; the T3 primitive then has no production consumer. Revert
T4 to restore the `()` processor signature, and T3 to remove the primitive;
each reverts independently. The T2 ADR and links document existing behavior
and stay valid after those reverts, apart from the SI-15 ownership split,
which would need a one-line correction. SI-14 token lifecycle and the
manager-owned IP-ban cleanup job are unaffected.

## Risks and Trade-offs

- **Hot-path cost (D2, D7)**: `JoinSet` bookkeeping, a non-blocking reap per
  datagram, and a `Result` output. Mitigation: separate D6 checkpoints B1 and
  B2 and AC11. If B2 regresses, reconsider the reap frequency before touching
  the eviction ADR.
- **Slower legacy stop**: legacy consumers now wait up to five seconds for
  active requests. Accepted by the maintainer: stop speed is secondary to
  request speed. Idle stops remain immediate.
- **Inexact overload accounting (D8)**: processors whose handles the ring
  drops still run outside the 50-request count. Accepted as part of the
  existing overload design; SI-15 only ensures shutdown owns them.
- **Best-effort outcome (D3)**: reporting `Cancelled` despite aborted requests
  could hide slow processing. Mitigation: the `warn` summary makes it visible;
  SI-20 reviews the exit-code mapping.
- **Overlap with `simplify-udp-server-main-loop`**: both touch
  `run_udp_server_main`. Mitigation: keep SI-15 changes to processor spawning
  and the cancellation return path; note the dependency in that draft.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: recorded in [implementation-retrospective.md](implementation-retrospective.md).
- Create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` for reusable lessons or
  material deviations; otherwise record why none is needed in the progress log.
- The independent Task Reviewer verifies the acceptance criteria, processor
  ownership on every drop path, test design, manual evidence, and validation,
  and records its report in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.
- Evidence lives in `manual-verification-evidence.md` and
  `performance-evidence.md`; the earlier `verification.md` placeholder was
  removed at promotion.

## References

- Parent EPIC: [#1488](https://github.com/torrust/torrust-tracker/issues/1488)
- Preceding migration: [#2342](https://github.com/torrust/torrust-tracker/issues/2342), PR [#2351](https://github.com/torrust/torrust-tracker/pull/2351)
- Eviction policy background: [#2149](https://github.com/torrust/torrust-tracker/issues/2149)
- Decisions: [Q3 and Q4](../../../features/shutdown-process/questions.md)
- Superseded combined draft: [SI-9](../../drafts/1488-si-9-improve-udp-shutdown/ISSUE.md)
