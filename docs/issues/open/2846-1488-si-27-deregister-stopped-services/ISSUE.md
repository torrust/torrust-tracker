---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 1488
github-issue: 2846
spec-path: docs/issues/open/2846-1488-si-27-deregister-stopped-services/ISSUE.md
branch: "2846-deregister-stopped-services"
related-pr: null
last-updated-utc: "2026-11-12 14:30"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/bootstrap/jobs/http_tracker.rs
    - src/bootstrap/jobs/tracker_apis.rs
    - src/bootstrap/jobs/udp_tracker.rs
    - src/bootstrap/jobs/health_check_api.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/udp-server/src/server/states.rs
    - packages/axum-health-check-api-server/src/environment.rs
    - packages/axum-health-check-api-server/src/handlers.rs
    - docs/issues/closed/2697-1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md
    - docs/issues/closed/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2846 - Deregister Services from the Registar as They Stop (SI-27)

> **EPIC position**: Roadmap row 24, sub-issue SI-27 of [EPIC #1488](../1488-overhaul-tracker-shutdown/ISSUE.md).
> A follow-up of [SI-21](../../closed/2697-1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md).
> Implementation order 3 of 4. Priority p3; must land before the first publish (#2583, see Dependencies).

## Goal

A service that stops in an orderly way removes its own registration from the shared `Registar`, so the
registry lists only what is running. Today the registry keeps every entry until the process exits.

## Background

Checked on 2026-11-12 against `torrust/develop` at `388faf59a`.

- Each server registers its `ServiceBinding` in the `Registar` on start (HTTP tracker, REST API, UDP
  tracker and health-check API). None removes it when it stops.
- `Registar::deregister` exists since `torrust-server-lib` 0.4.0 (the repository uses 0.6.0). It
  removes whichever registration holds a binding, so a caller must pass only a binding it registered.
- The only production-code caller is the health-check API test environment (`stop()`, added by SI-24
  so a fixed-port restart can register again). The application components do not call it.
- The maintainer suggested the change during the SI-21 review: the health list should stop reporting
  services that are no longer available. SI-21 kept it out of scope and recorded it as a later idea.

**What changes for an operator today: almost nothing.** The health handler answers 503 from the readiness
flag without touching the `Registar` once shutdown begins, so the stale entries are never listed during a
normal shutdown, and the process exits right after. The value is a registry that is correct by
construction (a listing, a future in-process restart, a test that stops one service and probes the
others). The acceptance criteria therefore include a go/no-go decision (T1), so the task is dropped if
the maintainer sees no use.

A failed service must **not** be deregistered: a service that stopped by failing should keep reporting
unhealthy in the health list. Only an orderly stop (cancellation requested, drained) removes the entry.

## Scope

### In Scope

- Decide where the deregistration lives (T1). Note that the SI-24 `Environment::stop` in
  `packages/axum-health-check-api-server/src/environment.rs` calls `deregister` before it maps the join
  error, so a failed stop also loses its entry; T1 must not copy that order, and records whether the
  environment changes to match. Recommended: in the token-aware stop path of each server
  package, as SI-24 did for the health-check environment, because the package knows the binding it
  registered and the entry is removed exactly when the stop completes. The alternative is the root
  component supervisors, which would need the binding handed back from `start_job`.
- Deregister the HTTP tracker, REST API, UDP tracker and health-check API binding after an orderly stop
  (drain completed), and leave the registration in place when the stop failed, timed out or was aborted.
- Tests: after an orderly stop the registry no longer lists the binding; after a drain timeout it still
  does; other services' registrations are untouched; two services on the same registry stay independent.
- Document the behavior in the shutdown-process docs and in `Registar` usage notes.

### Out of Scope

- Changing the 503-during-shutdown behavior or the readiness flag (SI-21).
- Changing the health-check report shape or the probe fan-out.
- Removing the registration on a failed stop, or registry changes in `torrust-server-lib` (an external repository;
  `deregister` already exists in 0.6.0).
- A bare drop of a test environment: a drop cannot await the registry lock (SI-24 recorded the same limit).

## Dependencies

- SI-21 (#2697) and SI-24 (#2450) are done and merged. `torrust-server-lib` 0.6.0 provides `deregister`.
- Independent of SI-25 and SI-26; SI-26 may change the supervisors this task could use (T1 decides the
  location, so ordering SI-26 first avoids touching the supervisors twice if the root location wins).
- Maintainer decision of 2026-11-12: SI-25, SI-26 and SI-27 land before the first publish of the
  workspace crates (#2583), which is blocked on EPIC #1488.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`,
  `docs/adrs/20261104120915_stop_the_health_check_api_last_to_report_not_ready_during_shutdown.md`.
- ADRs to create: none expected. If T1 chooses the root supervisors, record the rule "an orderly stop
  removes the registration, a failure keeps it" in the SI-21 ADR instead of a new one.
- Versioning: the change adds no public item but changes behavior of the server packages; record in the
  progress log whether it needs a version bump (independent-versioning ADR).

## Design and Ownership Review

The registration is owned by the component that registered it. Invariants: a registration exists only
while the service can serve; it is removed after the drain finished (so a probe during the drain still
sees the service), and never for a failed stop; removing it must not block the stop beyond the process
deadline `P`, because `deregister` awaits the registry lock (the lock is held only for map operations).
A design-review checkpoint follows the first passing vertical slice (the HTTP tracker).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Go/no-go and location decision; recorded in the spec after the maintainer answers. | Decision and reasons in the progress log. |
| T2 | TODO | HTTP tracker: deregister after an orderly drain; tests for drained, timed out, and other services intact. | First vertical slice; design-review checkpoint. |
| T3 | TODO | REST API, UDP tracker and health-check API, the same way. | Same tests per service. |
| T4 | TODO | Application-level test: stop one component and check the registry lists the rest. | Uses the existing component test harness. |
| T5 | TODO | Docs. | Shutdown-process docs and `Registar` notes. |
| T6 | TODO | Verify and record evidence. | `manual-verification-evidence.md`. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | None (decision in the spec) | Commit the spec edit after the decision. |
| T2 | HTTP tracker slice | Commit after focused validation and review. |
| T3 | The other three services | Commit after focused validation. |
| T4 | Application-level test | Commit with T3 or separately. |
| T5 | Documentation | Commit after the last production change. |
| T6 | Verification evidence | Commit after the last production change. |

Sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/`, moved to `docs/issues/open/`
- [x] Spec reviewed and approved by the maintainer (decisions of 2026-11-12, in chat)
- [x] GitHub issue created and issue number added to this spec (#2846)
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Go/no-go and location decision recorded (T1)
- [ ] Implementation completed
- [ ] Design review after the first passing vertical slice recorded
- [ ] Automatic verification completed (`linter all`, relevant tests, pre-push checks)
- [ ] Manual verification scenarios executed and recorded in `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-12 14:30 UTC - AI agent (Claude Sonnet 5.5) - Spec created from the SI-21 follow-up ("deregister services from the Registar as they shut down") on `torrust/develop` at `388faf59a`; recorded the maintainer's decisions of 2026-11-12 (opened as SI-27; lands before #2583) and the finding that the 503 short-circuit hides the stale entries during a normal shutdown, which is why T1 is a go/no-go decision.

## Acceptance Criteria

- [ ] The go/no-go and location decision is recorded.
- [ ] After an orderly stop the registry no longer lists the service's binding, for each of the four services.
- [ ] After a failed, timed-out or aborted stop the registration remains.
- [ ] Other services' registrations are unaffected, including two instances of one service type.
- [ ] Shutdown order, exit codes and the 503 behavior are unchanged.
- [ ] `linter all` passes.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo check --workspace --all-targets` warning-free; `cargo test --workspace` and the focused tests of the four server packages and `src/bootstrap/jobs`
- Pre-push checks before the implementation push

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

Executable-boundary verification follows the EPIC protocol (signal the actual binary PID, bounded wait,
exit status and shutdown output, then prove the bindings are released). The registry is not visible
after the process exits, so the observable part is the health list while the process is still running.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Orderly stop of the whole tracker | Start with all four services, `SIGTERM` the binary PID. | Exit code 0 within the deadline; the shutdown output is unchanged from before the change. | TODO | `manual-verification-evidence.md` |
| M2 | Health list while one service is down | Use the application-level harness of T4 (or a build with one listener stopped) and request `/health_check` while the process runs. | The stopped service is absent after an orderly stop; the running ones are listed healthy. | TODO | `manual-verification-evidence.md` |
| M3 | Failed stop keeps the entry | Hold a REST request open past its drain (the SI-21 V6 recipe) and read the registry in the harness. | The REST registration remains; exit code 1 within `P`. | TODO | `manual-verification-evidence.md` |

### Disposable Verification Scripts

None.

### Acceptance Verification

| AC | Status | Evidence |
| -- | ------ | -------- |
| Decision recorded | TODO | T1 |
| Orderly stop deregisters, four services | TODO | Automatic checks, M2 |
| Failure keeps the registration | TODO | Automatic checks, M3 |
| Other registrations unaffected | TODO | Automatic checks |
| Shutdown behavior unchanged | TODO | M1 |
| `linter all` passes | TODO | Automatic checks |

## Risks and Trade-offs

- Removing the registration of a failed service would hide a real fault from the health list; the rule above prevents it.
- `deregister` removes whichever registration holds the binding. Two services that register the same
  binding (a port clash that registration should already reject) would race; the services pass only
  the binding they registered.
- Low operator value today (see Background). If the maintainer decides it is not worth the churn before
  the first publish, the issue is closed as not planned and the EPIC row says so.

## Implementation Completion Review

Before closing, review what the implementation discovered. If it found anything material, create
`implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this
directory; otherwise add a progress-log entry saying why none was needed.

## References

- Parent EPIC: #1488 (roadmap row 24)
- Related: #2697 (SI-21), #2450 (SI-24, first use of `deregister`), #2583 (first publish)
- External: `torrust-server-lib` 0.4.0 added `Registar::deregister`

## Rollback

Revert the commits. Registrations are then left in place until the process exits, as before. No
persisted state or configuration is involved.
