---
semantic-links:
  skill-links:
    - create-adr
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
    - docs/features/shutdown-process/questions.md
    - docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md
    - packages/configuration/src/v3_0_0/shutdown.rs
    - src/main.rs
    - src/app.rs
    - src/bootstrap/jobs/manager.rs
    - docs/containers.md
---

<!-- skill-link: create-adr -->

# Budget Shutdown in Two Chained Stages Inside One Process Deadline

## Scope

Repository-level decision, so it belongs in `docs/adrs/`. The budgets are consumed by the application
bootstrap and the supervisor (`src/main.rs`, `src/app.rs`, `src/bootstrap/jobs/manager.rs`), by four
server packages (HTTP tracker, REST API, health check API, UDP tracker), and by the event listeners of
five packages. They are validated in `packages/configuration`. The operator contract they create
reaches the container, Compose, Kubernetes, and systemd settings of every deployment.

## Description

Question Q4 of the shutdown feature
([questions](../features/shutdown-process/questions.md#q4)) fixed one process deadline of 25 seconds,
a 20-second drain for the HTTP-family servers, a five-second drain for accepted UDP requests, and a
requirement that `T_component < T_process`. That rule holds for each component taken alone. It does not
hold for the whole shutdown, because the
[shutdown-order ADR](20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md) made
shutdown two sequential stages: an event listener is asked to stop only after every producer that
publishes on its bus has completed, and its drain budget limits how long it keeps taking events, not
how long a handler runs.

```text
t = 0                       t = max(H, U)                   t = max(H, U) + L (+ one in-flight event)
|-- stage 1: producers -----|-- stage 2: listeners drain ---|
   HTTP trackers   (<= H)      swarm, tracker-core, http-core,
   UDP trackers    (<= U)      udp-core, udp-server listeners  (<= L)
   REST API        (<= H)
   health check    (<= H)
```

With the Q4 numbers and the listener budget of five seconds that the shutdown-order work chose, the
worst case is `20 + 5 = 25` seconds: a drain that used its whole budget would leave the listeners no
time, and the supervisor would abort them at the deadline, which is the loss the shutdown-order
decision set out to prevent. Until issue #2668 every one of these values was a constant in its own
crate, and the supervisor deadline was 10 seconds.

## Agreement

Four budgets, all in `[application.shutdown]` (see
[the placement ADR](20261027145429_place_process_wide_settings_in_the_application_configuration_section.md)),
all whole seconds from 1 to 3600:

| Key                            | Symbol | Default | Applies to                                                                      |
| ------------------------------ | ------ | ------- | ------------------------------------------------------------------------------- |
| `deadline_in_secs`             | P      | 25 s    | The supervisor: components still running at P are aborted.                      |
| `http_drain_in_secs`           | H      | 15 s    | The HTTP tracker, the REST API, and the health check API.                       |
| `udp_request_drain_in_secs`    | U      | 5 s     | The UDP tracker's already accepted requests.                                    |
| `event_listener_drain_in_secs` | L      | 5 s     | Each event listener, after its producers have stopped.                          |

The startup validation rejects a configuration unless `max(H, U) + L < P`. The strict inequality
leaves at least one second for the in-flight event of a listener and for task joins. The error names
the longer server drain, the listener drain, and the deadline, and what to change. A value outside 1 to
3600 is rejected when the configuration is parsed.

The values are injected from the composition root to their consumers as plain durations; the server
packages keep no constant of their own, and `main` and the startup-failure cleanup read the deadline
from the configuration, so no `10 s`, `90 s`, or other budget constant remains on the token-aware
path. The legacy launcher paths keep their own values until their removal (issue SI-19 in EPIC #1488).

The orchestrator contract is `T_orchestrator >= P + 5 s`: 30 seconds at least and 35 seconds
recommended for the defaults. Docker and Podman wait 10 seconds by default, which is less than the
default P, so a tracker that is still draining is killed (exit status 137). Raising P requires raising
the orchestrator value by the same amount. The documentation is `docs/containers.md`.

The process exits `0` only when every component outcome is completed or cancelled, and `1` when any
component failed, panicked, or was aborted, or when startup failed (the Q3 contract). Only the
HTTP-family drains (the HTTP tracker, the REST API, and the health check API) end as a failed
component when their budget is reached. The two other drains log a warning and still end as
completed or cancelled: the UDP request drain aborts the processors it could not finish and logs how
many, and an event listener whose admission budget expires logs how many events it left unread.
An exit code `0` and the success message therefore do not prove that no UDP request or queued event
was discarded; read the shutdown warnings for that. A requested stop that ends with `1` leaves a
systemd unit `failed`; `1` is not added to `SuccessExitStatus`.

### Alternatives Considered

- **Keep 20 s for H and validate only `H < P`.** Rejected: a full HTTP drain would starve the
  listeners.
- **Raise P to 30 s.** Rejected: the orchestrator minimum would become 35 s, which no longer fits
  Kubernetes' default grace period of 30 s.
- **Shrink L.** Rejected: it contradicts the five-second choice of the shutdown-order work.
- **A `JobManager` that owns the deadline and per-job budgets.** Rejected: the budgets belong to the
  services that apply them, and the deadline stays a single value passed to `wait_for_all`.
- **Per-instance or per-listener drain budgets.** Rejected: no demonstrated need, and the chain rule
  would need to know which instances run.

## Consequences

- The default HTTP-family drain falls from 20 s (Q4) to 15 s, and the health check drain rises from 5 s
  to 15 s because it shares H. The drain only waits while a connection stays open, so a stop without
  open connections takes the same time as before. Operators with very slow clients raise H and P
  together; the validation tells them when they do not.
- The chain rule is checked on the configuration alone, so it does not know which services run, and
  it can be stricter than a deployment without HTTP needs. A fixed rule is simpler than case analysis.
- A listener that cannot persist a full buffer within L can still be cut at P. Batching persisted
  writes (issue #2418) is the remedy, not a larger L.
- Readiness during shutdown (issue SI-21 in EPIC #1488) must use part of P and add no timer of its own.
- The Q4 defaults are amended: H is 15 s, and L is a fourth budget. The Q4 decision text stays as the
  record of what was decided on 2026-09-01.

## Date

2026-10-27

## References

- Issue #2668 (SI-20): the specification holds the derivation of the defaults and the evidence.
- [Q3 and Q4](../features/shutdown-process/questions.md#q3) of the shutdown feature.
- [Order shutdown by event flow and drain event listeners](20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md).
- [Place process-wide settings in the application configuration section](20261027145429_place_process_wide_settings_in_the_application_configuration_section.md).
- [Containers](../containers.md#stopping-the-tracker), the operator documentation.
- EPIC #1488: overhaul tracker shutdown.
