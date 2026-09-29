---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - packages/udp-server/src/server/request_buffer.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/processor.rs
    - packages/udp-server/docs/adrs/20260907152707_keep_oldest_first_udp_request_eviction.md
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/benchmarking.md
    - "issue #2370"
    - "issue #566"
    - "issue #611"
    - "issue #918"
    - "issue #2149"
---

<!-- skill-link: create-adr -->

# Bound UDP Request Concurrency With a Task-Per-Request Ring

## Scope

This is a package-local decision in `packages/udp-server/docs/adrs/`. It owns
how the extractable UDP server admits, bounds, and sheds request-processor work.
It does not define tracker-wide shutdown deadlines, process exit codes, or
configuration policy.

## Description

The UDP server processes each accepted datagram in a Tokio task. Under overload,
it must prevent request work from growing without bound while keeping the normal
request path inexpensive. The design therefore retains abort handles for a
fixed-size active-request ring (currently 50) and uses an oldest-first policy
when the ring is full.

The narrower [oldest-first eviction ADR](20260907152707_keep_oldest_first_udp_request_eviction.md)
defines the full-buffer traversal. This ADR records the broader request-handling
design, its evidence, and when it should be reconsidered.

## Agreement

1. Spawn one lightweight Tokio processor task per accepted UDP request.
2. Retain a fixed-capacity ring of processor `AbortHandle` values. A full ring
   reclaims finished work when possible; otherwise it gives the oldest active
   task one scheduler yield and aborts it if no earlier completion made space.
3. Keep the normal path cheap: do not add locks, dynamic dispatch, per-request
   heap allocation beyond the spawned task, or latency measurement solely for
   overload control.
4. Treat the ring as normal-operation admission and eviction policy, not as the
   shutdown owner. Issue #2370 adds a `JoinSet` so the receive loop owns every
   processor for cancellation, joining, outcomes, and socket release while the
   ring keeps its existing overload decision.

The fixed bound protects memory. The original implementation judged Tokio tasks
lightweight and UDP requests short-lived; it expected the tracker data locks and
kernel packet sending to dominate beyond a small concurrency level. The yield is
an intentional fairness opportunity, not a completion guarantee.

## History and Evidence

- [PR #644](https://github.com/torrust/torrust-tracker/pull/644), implementing
  issue #611, introduced concurrent UDP request tasks and the fixed 50-request
  bound in commit `72c83485`.
- Commit `9e01f7fa` corrected an earlier ring implementation that effectively
  handled one request at a time; its author noted that a vector could provide
  the same effect.
- [PR #873](https://github.com/torrust/torrust-tracker/pull/873), commit
  `84cc1a1d`, reimplemented request handling using a stream and the current
  `force_push` shape; its measurements improved average throughput.
- Issue #566 discussed adapting concurrency from measured core latency.
- Issue #918 observed real overload aborts and proposed a pending-request
  queue. [PR #921](https://github.com/torrust/torrust-tracker/pull/921)
  recorded the chosen eager-spawn and yield rationale.
- [PR #922](https://github.com/torrust/torrust-tracker/pull/922) tried simpler
  eviction and cleanup separation; it regressed performance and was rejected.
- Issue #2149 supplied focused request-buffer tests and the narrower eviction
  ADR.

## Alternatives Considered

### Rejected With Evidence

- **Evict one task, then clean finished tasks.** PR #922 measured a regression.
  The oldest task is not reliably the next one to complete.

### Discussed but Not Evaluated

- **Pending-request queue.** Avoids spawning work that cannot run immediately,
  but changes UDP response timing and adds queue ownership.
- **Adaptive concurrency limit.** Issue #566's latency-driven limit could adapt
  to core contention, but requires per-request measurement, tuning, and a
  benchmark-backed policy.
- **Per-client pools.** The original author proposed a two-level pool to keep a
  single client from exhausting all work. It adds per-client state on the hot
  path.
- **Vector-backed handles.** Mentioned in `9e01f7fa`; it needs equivalent
  bounded traversal and performance evidence.

### Not Yet Considered in the Original Design

- **Semaphore admission.** Reject or defer a new datagram before spawning when
  capacity is exhausted. It provides an exact bound but changes load shedding
  from evicting old work to refusing new work.
- **Kernel backpressure.** Stop reading when full and rely on the socket receive
  buffer. It minimizes application work but makes shedding less observable.
- **`JoinSet` as the sole bound.** Provides one owner and exact count, but needs
  a replacement ordering policy for oldest-first eviction.
- **Fixed worker pool.** Avoids per-request spawn cost but risks head-of-line
  blocking and is a larger redesign.

## Consequences

- The current request path remains fast under normal load and has bounded
  overload handling.
- The ring alone cannot observe, join, or account for every task; issue #2370
  adds the owner boundary without changing overload policy.
- Replacing the ring, changing capacity, or choosing a different shedding policy
  requires a separate issue or EPIC, updated ADR decision, and before/after
  benchmark evidence.

## Re-evaluation Triggers

Reassess this decision when any of these occur:

- production evidence shows sustained overload evictions or unfair client
  impact;
- a simpler alternative matches or exceeds the current B0/B2 benchmark results;
- memory, latency, or socket-pressure evidence shows the fixed bound no longer
  protects the deployment profile; or
- a supported protocol consumer needs explicit admission or fairness semantics.

## Affected Code

- `packages/udp-server/src/server/request_buffer.rs`: bounded handle ring and
  oldest-first eviction.
- `packages/udp-server/src/server/launcher.rs`: processor spawn, ring admission,
  and future shutdown owner wiring.
- `packages/udp-server/src/server/processor.rs`: per-request task boundary.

## Date

2026-09-29

## References

- Issue #2370: UDP active-request shutdown policy.
- [Supervised cancellation tree ADR](../../../../docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md)
- [Oldest-first eviction ADR](20260907152707_keep_oldest_first_udp_request_eviction.md)
- [UDP shutdown task inventory](../../../../docs/features/shutdown-process/task-inventory.md)
- [UDP benchmark guide](../../../../docs/benchmarking.md)
