---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/issues/closed/2846-1488-si-27-deregister-stopped-services/ISSUE.md
    - docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md
    - docs/adrs/20261104120915_stop_the_health_check_api_last_to_report_not_ready_during_shutdown.md
    - src/app.rs
    - src/container.rs
    - src/bootstrap/jobs/manager.rs
    - packages/axum-health-check-api-server/src/handlers.rs
    - packages/axum-health-check-api-server/src/environment.rs
---

<!-- skill-link: create-adr -->

# Keep Registar Registrations for the Process Lifetime

## Scope

Repository-level decision, so it belongs in `docs/adrs/`. It concerns the contract between the
application (`src/app.rs`, `src/container.rs`), the four server packages that register in the
`Registar` (HTTP tracker, REST API, UDP tracker and health check API) and the external
`torrust-server-lib` crate that owns the registry type.

## Description

Each server registers its `ServiceBinding` in the shared `Registar` when it starts, and no server
removes it when it stops: the registry keeps every entry until the process exits. The health check
API reads the registry to decide which services to probe.

During the review of SI-21 (#2697) the maintainer asked whether a stopped service should remove its
registration, so the health check would stop listing services that are no longer available. The
concern was a shutdown in which one service has already stopped while another one still drains for a
long time: could a request in that window see the stopped service as available? Issue #2846 (SI-27)
was opened to decide it, with a go/no-go decision as its first task.

The investigation (on `torrust/develop` at `b009b6425`) found:

- **A service is cancelled only after shutdown begins.** `JobManager` cancels component tokens only
  in `request_shutdown` and, for the components that wait for others, in `mark_finished` once shutdown
  was requested. Nothing stops one service while the others keep running.
- **The health check does not read the registry during shutdown.** `begin_shutdown` marks the tracker
  not ready before it cancels any component, and `/health_check` then answers `503` with the message
  "shutting down" and an empty `details` list, without probing anything. The health check API stops
  after every other component, so this answer holds for the whole drain, however long one service
  takes (SI-21 ADR).
- **A stale entry cannot be reported as healthy.** Outside shutdown, the health check opens a real
  connection to each registered service. A service that stopped has closed its listener, so its probe
  fails and the service is reported as failing.
- **The case where the process keeps running must keep the entry.** A server that returns on its own,
  without being cancelled, leaves the process running without it. Its entry must stay, so the health
  check reports the fault.
- **Servers cannot remove their own entry.** A server receives a single-use `ServiceRegistrationForm`,
  which `register()` consumes. After that it holds no handle to the `Registar`.

## Agreement

- **A registration lives until the process exits.** No server removes its registration when it stops,
  whether the stop is orderly or a failure. The application code does not call `Registar::deregister`.
- **Readiness, not the registry, reports a shutdown.** During a shutdown the health check reports
  "not ready" (`503`) from the readiness state, as the SI-21 ADR decided; the registry contents are not
  part of that answer.
- **Liveness comes from the probe, not from the registry.** Outside shutdown, a registered service
  that no longer runs is reported as failing by its health probe. The registry says what to probe,
  not what is healthy.
- **Test environments are the one exception.** The health check API test environment removes its own
  registration in `stop()` (SI-24), because a test restarts it on a fixed port in the same process,
  which a stale entry would reject with `RegistrationError::DuplicateBinding`. That is the reopening
  trigger below, in a test.

Reasons:

- During a normal shutdown the change would have no effect any caller can observe: the readiness
  answer hides the registry, and the process exits within the deadline `P`, which takes the
  in-memory registry with it.
- A tracker is shut down rarely: operators usually run it on a dedicated server or VM, and the tracker
  does not support autoscaling. BitTorrent clients tolerate a tracker that disappears (they retry, use
  other trackers of the torrent, or use DHT and peer exchange), so a few failed requests during a drain
  are harmless.
- Keeping every entry never hides a fault. Removing entries risks removing the one of a failed service,
  which is the case the health check exists to report.
- It keeps the registration capability narrow: a server can add only itself and can never remove
  anything, so one service cannot remove another's entry (`deregister` removes whatever holds the
  binding).
- It avoids changing the public start functions of four server packages just before their first
  publish (#2583).

### Reopening Trigger

Revisit this decision when the application restarts a single service while the process keeps running
(for example a hot reload, or restarting one listener after a configuration change). The restarted
service would register the same binding again and get `RegistrationError::DuplicateBinding`. The
preferred design then is a change in `torrust-server-lib`: `register()` returns a registration handle
that can remove only its own entry, so the capability stays narrow.

### Alternatives Considered

- **Give each server package the `Registar` instead of a form**, so it deregisters in its own stop
  path after an orderly drain. Rejected: it changes the public start functions of four packages, and a
  server could remove any other service's entry.
- **Deregister in the root supervisors** (`src/bootstrap/jobs`), after a cancelled and drained stop.
  Rejected: each `start_job` would return the binding and the supervisors would need the registry,
  for no observable benefit today.
- **Add a registration handle to `torrust-server-lib`** (`register()` returns a handle with
  `deregister`). Not done now: it is the right design if the reopening trigger happens, but it needs a
  release of an external crate for a change with no current use.

## Consequences

- During a normal shutdown the registry still holds the entries of services that already stopped. No
  caller sees them: the health check answers `503` without reading the registry.
- Outside shutdown, a service that stopped on its own stays in the registry and its probe reports it
  as failing.
- Issue #2846 (SI-27) is closed as not planned; the server packages and the supervisors do not change.

## Date

2026-11-16

## References

- Issue [#2846](https://github.com/torrust/torrust-tracker/issues/2846) (SI-27), EPIC
  [#1488](https://github.com/torrust/torrust-tracker/issues/1488)
- [SI-27 spec](../issues/closed/2846-1488-si-27-deregister-stopped-services/ISSUE.md)
- [Registar ADR](20260728115400_define_registar_as_runtime_service_registry.md)
- [Health check API stops last ADR](20261104120915_stop_the_health_check_api_last_to_report_not_ready_during_shutdown.md)
- [Health check handler](../../packages/axum-health-check-api-server/src/handlers.rs)
- [Shutdown entry point (`begin_shutdown`)](../../src/app.rs)
