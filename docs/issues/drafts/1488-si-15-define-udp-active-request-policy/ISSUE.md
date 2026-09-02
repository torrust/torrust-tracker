---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/request_buffer.rs
    - packages/udp-server/src/server/states.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/drafts/1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
    - docs/issues/drafts/1488-si-9-improve-udp-shutdown/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-15 — Define UDP Active-Request Shutdown Policy

> **EPIC position**: Roadmap step 11. A focused policy and implementation change
> after UDP receive-loop ownership is token-driven and joinable.

## Goal

Define and implement the shutdown policy for UDP request processor tasks already
accepted when cancellation begins. The UDP component first stops admission and
completes its receive-loop lifecycle. This task then defines how it waits for active
request processors, when it deliberately aborts any remaining processors, and
how it reports completed versus aborted work.

The policy must preserve BitTorrent UDP's best-effort semantics while making
shutdown bounded, observable, deterministic, and safe for the component owner
to report upward.

## Current State

`run_udp_server_main` spawns one processor task per accepted UDP request and
stores only its `AbortHandle` in the fixed-capacity `ActiveRequests` ring buffer.
When the buffer is full, `force_push` can abort an older unfinished task.
`ActiveRequests::drop` aborts every retained unfinished task. There is no
shutdown-specific timeout, completed-versus-aborted outcome count, or explicit
owner-visible request-set completion mechanism.

The preceding receive-loop migration retains this behavior as a compatibility
fallback. The application-level UDP IP-ban cleanup job remains separately
manager-owned. This task may change only the active-request lifecycle after the
component has stopped accepting new packets.

## Scope

### In scope

- Define the active-request shutdown contract: wait until a component request
  deadline, then deliberately abort remaining processor tasks.
- Replace abort-handle-only tracking where necessary with tracking that lets the
  UDP component await processors and count completed, failed, and aborted work.
- Preserve bounded active-request capacity and document any necessary change to
  its implementation.
- Emit structured logs or metrics for shutdown-time request outcomes.
- Add deterministic tests for completion before deadline and deliberate abort
  after deadline, without OS signals.
- Add manual verification under UDP traffic after SI-1, recording the final
  request outcome summary.

### Out of scope

- Token propagation and receive-loop ownership; the preceding UDP receive-loop
  migration owns those changes. UDP IP-ban cleanup remains a separate
  manager-owned application job.
- HTTP, REST API, health-check API, or standalone UDP environment migration.
- Changing normal-operation overload behavior except where a tracking change is
  required to preserve the documented bounded-capacity contract.
- Final operator-configurable deadline values; Q4 and the later policy
  configuration work own those values.
- Legacy lifecycle API removal/deprecation.

## Proposed Shutdown Contract

1. After root cancellation reaches the UDP component, it stops admitting new
  packets and completes the receive-loop shutdown defined by the prior
  migration.
2. The component awaits active processors until its request deadline, which is
   supplied by the component lifecycle policy.
3. It deliberately aborts processors still incomplete at that deadline.
4. It awaits aborted task termination where Tokio permits, records the count of
   completed, failed, and deliberately aborted processors, and returns one UDP
   component outcome to its parent.
5. Normal-operation capacity pressure remains bounded and separately observable;
   shutdown-induced aborts must be distinguishable from overload-induced aborts.

## Acceptance Criteria

- [ ] The UDP component can await every active request processor it owns during
      shutdown, rather than only holding abort handles.
- [ ] Cancellation stops packet admission before the active-request deadline
      begins.
- [ ] Processors completing before the deadline are counted and included in the
      shutdown outcome summary.
- [ ] Processors remaining after the deadline are deliberately aborted, awaited,
      and counted separately from failed processors.
- [ ] The shutdown summary distinguishes completed, failed, and aborted request
      processors; shutdown-induced aborts are distinguishable from overload
      aborts.
- [ ] Existing bounded-capacity normal-operation behavior is preserved or any
      change is explicitly documented and covered by tests.
- [ ] Deterministic tests control request completion and deadline expiry without
      OS signals, sleeps, or external network dependencies.
- [ ] Manual UDP traffic verification records the request outcome summary after
      a SIGTERM-triggered shutdown path is available through SI-1.
- [ ] `linter all` passes.

## Dependencies

- UDP receive-loop lifecycle migration is complete; the UDP IP-ban cleanup job
  remains independently manager-owned.
- Q4 defines the approved component/request deadline hierarchy. SI-20 later
  makes its production values configurable; tests may use controlled deadlines.
- SI-1 is required only for manual SIGTERM verification.

## Rollback

Revert only the active-request tracking and shutdown policy. The preceding UDP
component token lifecycle remains intact and falls back to the previously
supported bounded `ActiveRequests` abort behavior. No other protocol component
or standalone consumer is changed.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Run deterministic tests with controlled request completion and deadline
   expiry, recording completed, failed, and aborted counts.
2. Run the tracker with UDP enabled, send a bounded request burst, then send
   SIGTERM after SI-1. Record the `main()` signal event and UDP request-outcome
   summary.
3. Repeat a normal-operation saturation scenario and confirm overload-induced
   aborts remain distinguishable from shutdown-induced aborts.
4. Confirm the UDP component does not complete before it has joined or
   deliberately aborted every active request task it owns.
