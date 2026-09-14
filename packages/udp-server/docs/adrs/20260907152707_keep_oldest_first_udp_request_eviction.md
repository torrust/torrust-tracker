---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - packages/udp-server/src/server/request_buffer.rs
    - packages/udp-server/src/server/launcher.rs
    - issue #2149
    - docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md
---

<!-- skill-link: create-adr -->

# Keep Oldest-First UDP Request Eviction

## Scope

This is a package-local decision in `packages/udp-server/docs/adrs/`. It governs only the
extractable UDP server's bounded normal-operation request buffer. It does not define cross-package
protocol behavior, tracker-domain rules, or shutdown policy.

## Description

`ActiveRequests` stores up to 50 `AbortHandle` values for UDP request processor tasks. When a new
request arrives while the buffer is full, the server must make room quickly on the request hot path.

A literal reading of the historic comments suggested scanning every retained handle and reclaiming
all completed tasks before aborting a live task. A deterministic test explored an ordering where the
oldest task was pending and later tasks were already completed. The implementation instead yields
once to the oldest pending task and aborts it when no older completed task has made space.

The original active-request-buffer review established that a design which decoupled removal from
cleaning all completed tasks caused a performance regression. It also recorded that the oldest task
cannot be assumed to be the next task to complete. The current bounded traversal is therefore an
intentional normal-operation overload policy, not evidence of a defect.

## Agreement

When `ActiveRequests` is full, preserve the following oldest-first, bounded decision:

1. Traverse handles from oldest to newest.
2. Discard completed handles encountered before the first handle that remains active after one
   scheduler yield.
3. If such an active handle is encountered before any completed handle has created capacity, abort
   that oldest active handle and stop scanning.
4. Otherwise continue its bounded traversal after capacity has been created. The current
  implementation retains at most one subsequently encountered active handle for re-entry; any
  broader change to that tracking behavior needs separate analysis and performance evidence.

This policy favors prompt, bounded overload handling over a full-buffer scan that would preserve a
live oldest task when newer completed handles exist. Its work is bounded by the fixed capacity of
50, and it must not add dynamic dispatch, per-request heap allocation, or additional asynchronous
coordination.

The `yield_now` call is a fairness opportunity for the oldest task to complete; it is not a
shutdown deadline, task-joining mechanism, or guarantee that every completed handle is reclaimed on
each insertion.

## Alternatives Considered

### Scan every retained handle before selecting an eviction

This would preserve the oldest active task whenever any newer handle has completed. It was rejected:
the historical #922 experiment that separated removal from cleaning completed tasks regressed
performance, and the request path must remain bounded and inexpensive under load.

### Replace the ring buffer or change its capacity

Rejected. The issue is policy clarification, not a demonstrated data-structure or capacity defect.
Any future capacity or algorithm change requires separate evidence, review, and performance
measurement.

### Treat this as shutdown behavior

Rejected. This ADR governs normal-operation overload. Shutdown-time processor ownership, deadlines,
joining, and outcome reporting are separately owned by the planned SI-15 work.

## Consequences

- A later completed task may remain in the ring buffer when an older task is aborted under pressure.
- This ADR does not broaden the existing policy for tracking multiple active handles after an
  earlier completed handle has created capacity; that behavior requires separate analysis before
  it is changed or treated as a contract.
- Request-buffer tests must assert the documented oldest-first policy rather than a full-scan
  reclamation policy.
- Any production change to this path requires the equivalent before/after performance evidence
  described in Issue #2149.
- Future contributors have an explicit rationale for retaining this non-obvious trade-off.

## Affected Code

- `packages/udp-server/src/server/request_buffer.rs`: buffer traversal, completed-handle removal,
  and oldest-active-task eviction.
- `packages/udp-server/src/server/launcher.rs`: calls `force_push` and publishes an aborted-request
  fact only when the buffer reports an eviction.

## Date

2026-09-07

## References

- Issue #2149: https://github.com/torrust/torrust-tracker/issues/2149
- Original implementation: commit `89bb73576`
- Original review clarification: PR #921
  (<https://github.com/torrust/torrust-tracker/pull/921>)
- Rejected performance-regression experiment: PR #922
  (<https://github.com/torrust/torrust-tracker/pull/922>)
- Planned shutdown policy: `docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md`
