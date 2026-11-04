---
semantic-links:
  skill-links:
    - create-adr
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2697-1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md
    - docs/features/shutdown-process/README.md
    - docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md
    - docs/adrs/20261027145446_budget_shutdown_in_two_chained_stages_inside_one_process_deadline.md
    - src/app.rs
    - src/bootstrap/jobs/manager.rs
    - packages/primitives/src/readiness.rs
    - packages/axum-health-check-api-server/src/handlers.rs
---

<!-- skill-link: create-adr -->

# Stop the Health Check API Last to Report Not Ready During Shutdown

## Scope

Repository-level decision, so it belongs in `docs/adrs/`. It spans the application bootstrap
(`src/app.rs`, `src/main.rs`), the supervisor (`src/bootstrap/jobs/manager.rs`), the
`torrust-tracker-primitives` package (the readiness state) and the health check API package.

## Description

Q6 of the shutdown feature approved a two-phase shutdown: mark the tracker not ready, then drain. A
readiness probe can only see "not ready" if the health check API still answers. Before issue #2697 the
health check API had no stop-order edge, so the supervisor cancelled it together with every other
component and its listener closed at once: a new probe got a reset or "connection refused", never a
503 (measured in the issue's `manual-verification-evidence.md`, V1 and V2). A refused connection also
fails a Kubernetes readiness probe, but it is indistinguishable from a crash and gives a load balancer
no signal that the instance is draining.

## Agreement

- The application owns a one-way `Readiness` state (`torrust_tracker_primitives::Readiness`, default
  ready). `begin_shutdown` marks it not ready and only then cancels the components. While it is not
  ready, `/health_check` answers `503` with `{"status":"Error","message":"shutting down","details":[]}`
  without probing any registered service.
- **The health check API stops after every other component.** `JobManager::reserve_stop_last` reserves
  it first (an edge needs a target reserved earlier) and the manager adds "stops before the health check
  API" to every component spawned afterwards, so a component added later cannot forget the edge. The
  server is still bound and spawned last, so startup order and port conflicts do not change.
- The health check API keeps answering 503 for the whole drain, and is cancelled only when every other
  component has completed, in any state. A failed or panicked component still releases it; a hung one
  keeps it serving 503 until the process deadline P aborts everything, which exits with code 1.
- Its own drain starts only after the others finish and is not budgeted inside `max(H, U) + L < P`. It
  is short when no probe connection is open, which the evidence of issue #2697 measures.
- The legacy `udp_ban_cleanup` job has no component id, so the ordering does not cover it. It holds no
  listener and serves no traffic; it is still stopped through the legacy root token.

### Alternatives Considered

- **Flip the flag, then cancel everything (document only).** Rejected: no new connection can observe
  the 503, because the health listener closes with the rest.
- **A short grace period between the flip and the cancellation.** Rejected: it amends Q6, which forbids
  a separate budget, and adds latency to every shutdown.
- **Reuse a `CancellationToken` as the readiness state, or derive readiness from `JobManager`.**
  Rejected: a token says "stop", not "do not route to me", and the health check crate would need the
  supervisor.

## Consequences

- During a normal shutdown the health check API outlives the other servers, so its list of services
  can still show ones that already stopped. `Registar::deregister` can remove them in a follow-up.
- Every 503 answer is logged by the health check API at error level, as any server error response is.
- `torrust-tracker-primitives` gains a public type (3.1.0) and the health check API package a changed
  start signature (0.2.0), under the independent-versioning decision.

## Date

2026-11-04

## References

- Issue [#2697](https://github.com/torrust/torrust-tracker/issues/2697), EPIC
  [#1488](https://github.com/torrust/torrust-tracker/issues/1488)
- [Shutdown-order ADR](20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md)
- [Shutdown budget ADR](20261027145446_budget_shutdown_in_two_chained_stages_inside_one_process_deadline.md)
