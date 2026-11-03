---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 1488
github-issue: null
spec-path: docs/issues/drafts/1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md
branch: "1488-si-21-mark-health-unhealthy-during-shutdown-draft"
related-pr: null
last-updated-utc: "2026-11-03 16:44"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/main.rs
    - src/app.rs
    - src/bootstrap/jobs/manager.rs
    - src/bootstrap/jobs/health_check_api.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/axum-health-check-api-server/src/handlers.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/issues/closed/2324-1488-si-13-migrate-health-check-api-token-lifecycle/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/drafts/1488-si-21-mark-health-unhealthy-during-shutdown/manual-verification-evidence.md
    - docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md
---

<!-- skill-link: create-issue -->

# Draft SI-21 — Mark Health Check Unhealthy During Shutdown

> **EPIC position**: Readiness-before-drain vertical slice (roadmap row 21).
> Every prerequisite is done on `develop`: SI-13 (#2324), SI-22 (#2410) and
> SI-20 (#2668). It uses the configured process deadline `P`
> (`[application.shutdown]`, default 25 s) and adds no timer.
>
> Refreshed on 2026-10-28 against `develop` (merge of #2686). Facts marked
> "read from source, not run" were read, not executed; the measured behaviour is
> in `manual-verification-evidence.md`.

## Goal

On a normal shutdown request, mark the tracker as not ready before server
components begin draining. While draining, `/health_check` returns HTTP 503 so
Kubernetes readiness probes and readiness-aware load balancers stop routing new
traffic to this instance. Existing accepted connections continue through their
component-specific graceful shutdown policy.

**Measured on the real binary (baseline, before any change):** after SIGINT or
SIGTERM the health-check server stops accepting new connections at once, so a
new probe sees a reset or `Connection refused`, never a 503, and the 503 would
be visible only on already-open connections, which also close. Setting a
readiness flag before the root-token cancellation therefore gives no observable
503 on its own: see "Design Decision" below.

This task changes readiness state only. It neither changes token propagation nor
introduces an independent shutdown timer.

## Decision Context

Q6 approved a two-phase service shutdown:

```text
shutdown request → mark not ready → drain existing work → process exits
```

The readiness state is observable only by infrastructure that uses the health
endpoint. It cannot prevent direct clients from sending UDP packets or opening
new direct TCP connections; protocol components still own admission and drain
behavior.

## Design Decision

Cause (read from source, not run): `src/app.rs` `start_health_check_api`
spawns the health-check component with no stop-before edge, so
`JobManager::cancel()` (called from `src/main.rs` right after the "shutting
down" log) cancels its token together with every component that waits for no
other job. Its `graceful_shutdown_on_cancellation` then stops axum accepting.

Maintainer decisions (recorded 2026-11-03 from the review chat; the maintainer
has not yet approved creating the issue from this text):

- Option (a) is accepted: "I think option A is fine".
- The health server's own near-instant idle drain is not budgeted inside `P`.
- Reserving `health_check_api` first is accepted; it is still bound and spawned
  last, so startup order and port-conflict behaviour do not change. Every other
  supervised component declares `StopBefore` on it. Read from source, not run:
  a target only has to be reserved, not spawned, and a reservation dropped by a
  failed startup is released (`release_if_reserved`). A test must assert that
  every supervised component has the health server as a successor, so a
  component added later cannot silently stop concurrently with it.
- Readiness state shape is NOT decided. Proposal: a small one-way readiness
  handle (atomic flag) owned by the application, defined in a crate the health
  crate may depend on, created by the app and flipped by `main` before
  `jobs.cancel()`. Alternatives: reuse a `CancellationToken` as "not ready", or
  derive it from `JobManager`. The final shape is fixed at the Phase 2 design
  review, before anything is wired through `main`.

Options considered:

- **(a) Health-check server stops last (accepted).** Spawn it with
  `StopBefore` the other supervised components, as SI-22 does for event
  listeners and as `src/app.rs` does for the runner. The readiness flip happens
  before `jobs.cancel()`; the health server keeps serving 503 for the whole
  drain and is cancelled only after every component it waits for completed.
  Uses the existing `JobManager` API, no timer, no Q6 amendment.
  Risk check (all read from source in `src/bootstrap/jobs/manager.rs`, not run):
  - The exit stays bounded by P: `wait_for_all` shares one deadline across all
    jobs, and a last-stopping health server is aborted at P at the latest.
  - A failure is not hidden: failed, panicked and aborted jobs all appear in
    the outcomes, and `report_shutdown` returns exit 1 for each of them. A
    health server aborted at P also counts as aborted.
  - A failed or panicked predecessor still releases its dependents, because
    `mark_finished` runs for every outcome that has a component id.
  - Worst case, a hung predecessor keeps the health server serving 503 until P.
  - Budget: the health server's own graceful drain starts only after the others
    finish, with `P - elapsed` left, and `max(H, U) + L < P` does not budget for
    it. Idle, its drain is near-instant; the maintainer accepted this.
  - Reservation order: `add_stop_before` needs target ids strictly earlier than
    the added id, and `health_check_api` is reserved last (`src/app.rs`), so it
    must be reserved first (its server may still start last).
  - The 503 handler must be a constant-time atomic read of application-owned
    readiness state, with no `Registar` probe fan-out.
  Also, the health list still shows services that stopped (see the Registar
  note).
- **(b) Flip then cancel, document only.** Simple, but a refused connection also
  fails a Kubernetes readiness probe, so the 503 adds almost nothing and
  acceptance criteria 3 and 7 could not be met on new connections.
- **(c) Short documented grace between flip and cancel.** Amends Q6 (which
  forbids a separate budget), needs an ADR and a spec change, and adds latency
  to every shutdown.

NOTE (not implemented here): the maintainer suggested removing services
from the `Registar` as they shut down so the health list stops reporting
services that are no longer available. `Registar::deregister` exists since
`torrust-server-lib` 0.4.0 (read from source, not run).

## Scope

### In scope

- Introduce application-owned readiness state with healthy as its startup
  default and not-ready as its irreversible shutdown state.
- Set readiness to not ready immediately when `main()` / `JobManager` initiates
  normal shutdown, before the health-check server is cancelled (before `jobs.cancel()` in
  `src/main.rs`; under option (a) the server stays up afterwards).
- Make the health-check endpoint return HTTP 503 while not ready, without
  running downstream service probes.
- Preserve the current healthy response behavior and probe fan-out before a
  shutdown request.
- Add deterministic tests that set readiness without OS signals and verify 503
  plus the absence of downstream probe execution.
- Add focused manual verification that records readiness transition before
  component drain and process exit.

### Out of scope

- Changing tracker HTTP/UDP listener admission or direct-client behavior.
- New health endpoint routes, retry timers, or an independent readiness timeout.
- Changing cancellation-tree ownership, component drain budgets, or exit codes.
- Health-check API token lifecycle migration; SI-13 owns that prerequisite.
- Kubernetes manifests or deployment automation beyond documenting the required
  readiness-probe behavior.

## Implementation Constraints

1. Readiness is owned by the application lifecycle, not by an individual server
   or by a request handler-local value.
2. The transition from ready to not ready is one-way for a process lifetime.
   Restarting the tracker creates a new ready lifecycle.
3. The readiness state must be safe to read concurrently from health requests
   and safe to set during shutdown initiation.
4. A not-ready response has HTTP status 503 and must not execute service probes;
   it reports lifecycle state, not a transient probe failure.
5. The readiness transition happens before the health-check server stops and
   consumes no separate time budget within the process deadline `P`
   (`[application.shutdown] deadline_in_secs`, default 25 s, SI-20).
6. If startup fails before readiness becomes ready, the process follows Q3's
   startup-failure exit result and must not advertise readiness.
7. The health-check crate must not depend on the root crate: the readiness
   state is a type owned outside it and injected (read `docs/packages.md`; the
   exact home is fixed at the Phase 2 design review).

## Acceptance Criteria

- [ ] Application readiness defaults to ready only after successful startup.
- [ ] Normal shutdown sets readiness to not ready before `jobs.cancel()` and,
      under the chosen design, a NEW connection to the health-check API during
      the drain receives HTTP 503 (not a refused connection).
- [ ] `/health_check` returns HTTP 503 while not ready and does not execute
      downstream registered-service probes.
- [ ] Healthy `/health_check` behavior and response body remain unchanged before
      shutdown initiation.
- [ ] Readiness transition is one-way and safe under concurrent health requests.
- [ ] Deterministic tests cover ready response, not-ready 503, no probe fan-out,
      and ordering before cancellation without OS signals.
- [ ] The health-check component stops after every other supervised component
      (a test asserts each has it as a successor), and a failed, panicked or
      aborted component still yields exit code 1 within `P`.
- [ ] Focused manual verification records readiness becoming 503 before the
      relevant component drain logs and before process exit, with timestamped
      probes on new connections, for SIGTERM and SIGINT.
- [ ] `linter all` passes.

## Dependencies

All done on `develop`:

- SI-13 health-check API token lifecycle migration (#2324).
- Supervisor shutdown initiation and SIGTERM handling (#1586, SI-1).
- SI-22 event-flow ordering (#2410) and its ADR.
- SI-20 configured process deadline and drain budgets (#2668).

This task adds no timer or separate deadline.

## Rollback

Before release and operational adoption, revert only the readiness state and
503 response path. The health-check API continues to use its token lifecycle and
all component shutdown behavior remains unchanged. After release, preserve the
published 503 readiness contract and repair defects through a compatible patch
rather than silently restoring always-probe behavior.

## Manual Verification

Record commands, output, and logs in issue-local `manual-verification-evidence.md` before closing this issue. Follow the EPIC executable-boundary protocol: signal the direct tracker binary PID, capture bounded completion and exit status, and prove readiness changes before affected bindings are released.

1. Run deterministic handler/application tests that toggle readiness without
   delivering an OS signal. Prove not-ready returns 503 without probe fan-out.
2. Run the tracker with health-check API enabled. Send SIGTERM and SIGINT to
   the tracker binary PID while a request holds the HTTP tracker drain open, and
   poll `/health_check` with NEW connections; record the 503 transition before
   drain-completion and process-exit logs, the exit status and listener release.
3. Verify a healthy running tracker still returns the existing health response.
4. Review deployment documentation to confirm readiness probes use
   `/health_check` and infrastructure removes not-ready instances from traffic.
