---
semantic-links:
  related-artifacts:
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - docs/adrs/20260727180000_shared_services_across_tracker_instances.md
    - packages/events/src/bus.rs
    - packages/http-core/src/container.rs
    - packages/udp-core/src/container.rs
    - packages/udp-server/src/container.rs
    - src/container.rs
    - src/bootstrap/jobs/
    - docs/issues/open/2039-normalize-per-instance-event-metrics-policy/ISSUE.md
    - docs/issues/drafts/optimize-event-publication-without-consumers/ISSUE.md
---

# Events Architecture

## Purpose

This guide describes the tracker event topology as it exists on `develop` and
the direction for normalizing per-listener metrics policy. It distinguishes an
event producer's responsibility to report an objective fact from a listener's
responsibility to apply a policy to that fact.

## Current Topology

`EventBus` wraps a Tokio broadcast channel and returns no sender when its
`SenderStatus` is disabled. This remains useful for explicit absent-sender
injection and for a future bootstrap-time consumer-demand decision. The HTTP
core, UDP core, and UDP server each create one event bus and one aggregate
statistics repository during application container initialization. They
explicitly enable their senders because their facts may be needed independently
of the originating listener's metrics policy.

The HTTP and UDP tracker instance containers share their respective core
services. The UDP server differs from both core layers: `AppContainer` creates
one application-wide `UdpTrackerServerContainer`, then passes a clone of that
container to every UDP listener. Consequently, the UDP server has one shared
event bus and one shared server statistics repository, not a bus or repository
per UDP listener.

| Layer      | Current event bus and repository ownership                                 | Event consumers                                         |
| ---------- | -------------------------------------------------------------------------- | ------------------------------------------------------- |
| HTTP core  | One application-wide bus and aggregate repository shared by HTTP listeners | HTTP core statistics listener                           |
| UDP core   | One application-wide bus and aggregate repository shared by UDP listeners  | UDP core statistics listener                            |
| UDP server | One application-wide bus and aggregate repository shared by UDP listeners  | UDP server statistics listener and UDP banning listener |

The REST metrics adapter exposes TCP announce counts from HTTP core statistics
and UDP announce counts from UDP server statistics. UDP request handling thus
has two event layers: a server lifecycle event stream and a core protocol event
stream.

## Producer and Listener Responsibilities

Events are objective facts. As established by
[ADR 20260727000000](adrs/20260727000000_events_are_objective_facts.md), an
event must not encode whether a particular consumer should act on it.

Applying a metrics setting at the producer is too broad when an event has more
than one consumer. In particular, UDP server cookie-error events are observed by
both the metrics listener and the banning listener. Suppressing production for
metrics also prevents the banning listener from observing the error.

The target division of responsibility is:

```text
listener instance
  -> always emit an objective event with stable runtime identity
  -> shared layer event bus
       -> metrics listener filters by that listener's metrics policy
          -> shared aggregate repository
       -> UDP banning listener receives every relevant security event
          -> shared ban service
```

Metrics filtering is therefore a consumer-side decision. Banning is independent
of metrics configuration and continues to enforce against the shared ban state.

## HTTP and UDP Asymmetry

The user-facing intent from [#1263][1263] and [#1401][1401] is aggregate
statistics with a per-public-listener `tracker_usage_statistics` policy. The
current implementation cannot express that intent consistently:

- HTTP and UDP core event production is gated globally, even though listeners
  are configured independently.
- UDP server metrics are also generated through one global event path and are
  the source of public UDP request counters.
- UDP server events additionally feed the banning listener, so a metrics gate
  cannot safely decide whether those facts exist.

This is an asymmetry of event ownership and consumers, not a reason to create a
repository per listener. A shared aggregate repository remains the desired
topology when the metrics listener filters events by stable listener identity.

## Proposed Normalization

The normalization work depends on [#2036][2036] defining canonical runtime
service and configuration-instance identity. That identity must travel with
metric-relevant events; configured socket addresses are not suitable because
multiple configuration blocks may use `0.0.0.0:0`.

The implementation makes producers emit objective facts regardless of
`tracker_usage_statistics`, then provides each metrics listener with an
immutable identity-to-policy lookup. The listener ignores disabled-listener
events before updating its shared aggregate repository. The UDP banning listener
does not use that lookup.

This approach preserves aggregate metrics, fixes the server-layer UDP gap, and
lets future non-metrics consumers receive complete facts without coupling them
to metrics configuration.

## Related Work

- Approved specification: [normalize per-instance event metrics policy](issues/open/2039-normalize-per-instance-event-metrics-policy/ISSUE.md)
- Deferred draft EPIC: [define and implement general error events](issues/drafts/generalize-error-events.md)
- Bootstrap bug: [#2035][2035]
- Runtime identity prerequisite: [#2036][2036]
- Shared-services decision: [ADR 20260727180000](adrs/20260727180000_shared_services_across_tracker_instances.md)
- Deferred investigation: [optimize event publication without consumers](issues/drafts/optimize-event-publication-without-consumers/ISSUE.md).
  This is not part of the correctness policy: it requires a complete consumer
  inventory and benchmark evidence before any publication gate is introduced.

[1263]: https://github.com/torrust/torrust-tracker/issues/1263
[1401]: https://github.com/torrust/torrust-tracker/issues/1401
[2035]: https://github.com/torrust/torrust-tracker/issues/2035
[2036]: https://github.com/torrust/torrust-tracker/issues/2036
