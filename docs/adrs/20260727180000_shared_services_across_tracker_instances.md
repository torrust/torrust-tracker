---
semantic-links:
  related-artifacts:
    - docs/adrs/index.md
    - docs/events-architecture.md
    - packages/http-core/src/container.rs
    - packages/udp-core/src/container.rs
    - packages/udp-server/src/container.rs
    - packages/udp-core/src/services/banning.rs
    - packages/tracker-core/src/container.rs
    - packages/configuration/src/v3_0_0/udp_tracker_server.rs
    - src/container.rs
---

# Shared Services Across Tracker Instances

## Description

The tracker can run multiple UDP and HTTP tracker listeners in a single process.
Each listener binds to a different address/port but shares core infrastructure:

- **Peer repository** (`TrackerCoreContainer`) — all instances share the same
  swarm data (torrents, peers, statistics). This is the primary reason to run
  multiple listeners: they serve the same swarm.
- **Ban service** (`BanService` in `UdpTrackerCoreServices`) — all UDP instances
  share the same IP-ban state. An IP banned on one UDP listener is banned on all.
- **Event buses and repositories** — HTTP core, UDP core, and UDP server events
  are aggregate application services. The UDP server container is shared by all
  UDP listeners.

This ADR documents the shared-services design and the rationale for keeping
certain services global rather than per-instance.

## Agreement

### Shared services

The following services are created once and shared across all instances of the
same type:

| Service                                       | Location                                      | Shared? | Rationale                                                                                                          |
| --------------------------------------------- | --------------------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------ |
| Peer repository                               | `TrackerCoreContainer`                        | Yes     | All listeners serve the same swarm                                                                                 |
| Swarm coordination registry                   | `SwarmCoordinationRegistryContainer`          | Yes     | Single source of truth for swarm state                                                                             |
| UDP ban service                               | `UdpTrackerCoreServices::ban_service`         | Yes     | Resource protection: an attacker should not be able to consume N× resources by attacking N listeners independently |
| UDP core event bus                            | `UdpTrackerCoreServices::event_bus`           | Yes     | Core events (connect, announce, scrape) are objective facts about the swarm, not about a specific listener         |
| UDP core services (connect, announce, scrape) | `UdpTrackerCoreServices`                      | Yes     | Stateless service objects; they read from the shared peer repository                                               |
| HTTP core event bus and repository            | `HttpTrackerCoreServices`                     | Yes     | Aggregate HTTP metrics are collected in one application-wide event path                                            |
| UDP server event bus                          | `UdpTrackerServerContainer::event_bus`        | Yes     | One application-wide bus is passed to every UDP listener                                                           |
| UDP server stats repository                   | `UdpTrackerServerContainer::stats_repository` | Yes     | One aggregate server repository receives events from every UDP listener                                            |

The UDP server's shared bus and repository do not conflict with per-listener
metrics policy. Events are objective facts and must be emitted independently of
metrics configuration. The target design filters events in the metrics listener
by stable runtime listener identity before mutating the shared aggregate
repository. A configured `SocketAddr` is not sufficient identity because two
listeners may validly use `0.0.0.0:0`.

UDP banning remains independent of metrics. Its listener receives every
security-relevant event from the shared UDP server bus and updates the shared
ban service regardless of whether the originating listener contributes to
metrics. See [events-architecture.md](../events-architecture.md).

### Why the ban service is shared

The ban service protects server resources by rate-limiting misbehaving IPs.
If each UDP listener had its own independent ban service, an attacker could
send `max_connection_id_errors_per_ip` invalid requests to each listener
independently, consuming N× the allowed error budget. A shared ban service
ensures that the total error rate across all listeners is bounded.

This is consistent with the principle that the tracker is a single logical
service, even when it exposes multiple network endpoints.

### Consequences for per-listener configuration

Settings that affect shared services must themselves be global. For example:

- `connection_id_validation` (issue #1136) controls whether the shared ban
  service's enforcement is active. It must be a global setting because the
  ban service is global — a per-instance policy would create an inconsistency
  where one listener's traffic pollutes the shared ban counter that another
  listener enforces against.

Settings that are inherently per-listener (bind address, cookie lifetime,
public URL, network topology) remain on the per-instance config struct.

### Alternatives Considered

**Per-instance ban service.**

Rejected because it allows an attacker to multiply resource consumption by
the number of listeners. It also complicates the operator's mental model:
"why did I ban this IP on port 6969 but not on port 6970?"

**Per-instance peer repository.**

Rejected because the primary reason to run multiple listeners is to serve
the same swarm through different protocols or addresses. Isolated peer
repositories would defeat this purpose.

### Consequences

#### Positive

- Resource protection scales with the number of listeners.
- Operators have a single ban list to reason about.
- Configuration for shared services is naturally global, avoiding
  per-instance inconsistencies.

#### Negative

- Per-listener policies that interact with shared services (like
  `connection_id_validation`) must be global, reducing flexibility.
- A misconfigured listener on one port can affect the ban state for all
  listeners.
