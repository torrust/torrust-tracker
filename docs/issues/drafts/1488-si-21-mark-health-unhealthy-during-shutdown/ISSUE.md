---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
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
    - docs/issues/drafts/1488-si-13-migrate-health-check-api-token-lifecycle/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-21 — Mark Health Check Unhealthy During Shutdown

> **EPIC position**: Readiness-before-drain vertical slice. It follows the
> health-check token lifecycle migration and uses the existing process deadline.

## Goal

On a normal shutdown request, mark the tracker as not ready before server
components begin draining. While draining, `/health_check` returns HTTP 503 so
Kubernetes readiness probes and readiness-aware load balancers stop routing new
traffic to this instance. Existing accepted connections continue through their
component-specific graceful shutdown policy.

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

## Scope

### In scope

- Introduce application-owned readiness state with healthy as its startup
  default and not-ready as its irreversible shutdown state.
- Set readiness to not ready immediately when `main()` / `JobManager` initiates
  normal shutdown, before root-token cancellation is propagated.
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
5. The readiness transition happens before root token cancellation and consumes
   no separate time budget within the Q4 25-second process deadline.
6. If startup fails before readiness becomes ready, the process follows Q3's
   startup-failure exit result and must not advertise readiness.

## Acceptance Criteria

- [ ] Application readiness defaults to ready only after successful startup.
- [ ] Normal shutdown sets readiness to not ready before `JobManager` cancels
      the root token.
- [ ] `/health_check` returns HTTP 503 while not ready and does not execute
      downstream registered-service probes.
- [ ] Healthy `/health_check` behavior and response body remain unchanged before
      shutdown initiation.
- [ ] Readiness transition is one-way and safe under concurrent health requests.
- [ ] Deterministic tests cover ready response, not-ready 503, no probe fan-out,
      and ordering before cancellation without OS signals.
- [ ] Focused manual verification records readiness becoming 503 before the
      relevant component drain logs and before process exit.
- [ ] `linter all` passes.

## Dependencies

- SI-13 health-check API token lifecycle migration is complete.
- Issue #1586 provides supervisor shutdown initiation and named outcomes.
- SI-20 supplies the configured process deadline and deployment contract.
- SI-1 is required only for manual SIGTERM verification.
  This task adds no timer or separate deadline.

## Rollback

Before release and operational adoption, revert only the readiness state and
503 response path. The health-check API continues to use its token lifecycle and
all component shutdown behavior remains unchanged. After release, preserve the
published 503 readiness contract and repair defects through a compatible patch
rather than silently restoring always-probe behavior.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Run deterministic handler/application tests that toggle readiness without
   delivering an OS signal. Prove not-ready returns 503 without probe fan-out.
2. Run the tracker with health-check API enabled. After SI-1, send SIGTERM to
   the tracker binary and poll `/health_check`; record the 503 transition before
   drain-completion and process-exit logs.
3. Verify a healthy running tracker still returns the existing health response.
4. Review deployment documentation to confirm readiness probes use
   `/health_check` and infrastructure removes not-ready instances from traffic.
