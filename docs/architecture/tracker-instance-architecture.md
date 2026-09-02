---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/architecture/README.md
    - docs/architecture/events.md
    - docs/application-jobs.md
    - docs/packages.md
    - docs/adrs/20260721000000_make_network_configuration_per_tracker_instance.md
    - docs/adrs/20260727180000_shared_services_across_tracker_instances.md
    - issue #1980
    - src/container.rs
    - packages/tracker-core/src/container.rs
    - packages/http-core/src/container.rs
    - packages/udp-core/src/container.rs
    - packages/udp-server/src/container.rs
---

# Tracker Instance Architecture

## Purpose

This guide describes how one Torrust Tracker process composes configured HTTP
and UDP listener instances. It establishes the practical model for evaluating
configuration placement and deployment options:

> Multiple listener instances in one process expose one logical tracker. They
> are not independent tracker applications supervised by a launcher.

The accepted shared-services decision is recorded in
[ADR-20260727180000](../adrs/20260727180000_shared_services_across_tracker_instances.md).
This guide explains its runtime implications. It does not replace that ADR.

## Runtime Composition

`AppContainer` constructs application-wide containers once, then creates one
HTTP or UDP listener container for every configured listener entry. A listener
has its own transport configuration and protocol adapter, but uses shared
application state and services.

```text
one tracker process
└── AppContainer
    ├── shared TrackerCoreContainer
    │   ├── swarm and peer repository
    │   ├── whitelist and authentication state
    │   └── shared tracker policies and announce handling
    ├── shared HTTP core services
    ├── shared UDP core services
    │   └── shared UDP BanService
    ├── shared UDP server container
    ├── HTTP listener instance 0
    ├── HTTP listener instance 1
    ├── UDP listener instance 0
    └── UDP listener instance 1
```

The process can bind several listeners. HTTP and UDP listeners may use the
same socket address because they use different transports. A configured port of
`0` is also valid, so the final binding is known only after the listener starts.

## Shared Services and State

The following are application-wide. A request handled by one listener can
observe effects created through another listener because they use these same
objects.

| Shared concern                                                                   | Reason                                                                                                                                         |
| -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| Swarm, torrent, and peer data                                                    | All listeners serve the same swarm. A peer announcing over HTTP must be visible to an equivalent UDP announce and vice versa.                  |
| Whitelist and authentication data                                                | Authorization has one meaning for the logical tracker.                                                                                         |
| Core policies                                                                    | Private mode, listing/whitelist authorization, announce policy, and tracker policy govern shared tracker behavior.                             |
| UDP ban state                                                                    | An invalid-request budget remains process-wide so an attacker cannot multiply it by targeting several UDP listeners.                           |
| HTTP, UDP-core, and UDP-server event paths and aggregate statistics repositories | Events describe application facts and feed aggregate observability; metrics policy is applied by consumers. See [events.md](events.md).        |
| Runtime service registry and application jobs                                    | The application tracks the started services and owns jobs according to service lifetime. See [../application-jobs.md](../application-jobs.md). |

The current runtime consumes the configuration v3 model. Shutdown policy is not
yet part of that schema; when it is introduced, process-wide policy belongs in
an application-level v3 configuration section, while component-specific budgets
remain owned by their lifecycle contracts. The shared-process topology itself is
independent of that future shutdown-policy configuration.

## Listener-Owned Concerns

Each configured HTTP or UDP listener has an individual configuration entry and
container. These listeners own endpoint-specific concerns, including:

- binding and transport lifecycle;
- network topology, including external address and reverse-proxy behavior;
- public URL and TLS exposure where applicable;
- UDP cookie lifetime;
- HTTP request parsing behavior where it is endpoint-specific;
- stable configuration-instance identity; and
- participation in aggregate usage statistics.

HTTP and UDP listener containers instantiate their own protocol adapter
services, such as announce and scrape adapters. Those adapters are not shared;
they call into shared tracker-core state and shared protocol-layer services.

## Configuration Placement Rule

Place a setting on a listener only when each listener can apply it independently
without changing shared state, shared security behavior, or the logical
tracker's meaning for another listener.

Place a setting on `core` or another shared-service configuration when it
governs shared data, authorization, security state, aggregate application
behavior, or a service constructed once per process.

Examples:

| Configuration concern                                      | Boundary                  | Why                                                                                                                     |
| ---------------------------------------------------------- | ------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| Bind address, external IP, reverse-proxy trust, public URL | Listener                  | They describe how one endpoint is exposed or reached.                                                                   |
| Metrics participation                                      | Listener                  | A listener can choose whether its facts contribute to aggregate metrics without withholding facts from other consumers. |
| Private mode, listed mode, private-mode policy             | Core                      | Authentication and whitelist checks use shared tracker state and must have one meaning.                                 |
| Announce policy and tracker policy                         | Core                      | They define behavior for the shared swarm and its peer-management logic.                                                |
| UDP invalid-connection-ID limit and ban-reset policy       | Shared UDP server service | They govern one shared `BanService`, not one listener.                                                                  |

## Multiple Listeners Versus Multiple Processes

Run multiple listeners in one process when they are different ways to reach the
same logical tracker. Typical examples include HTTP and UDP endpoints for one
swarm or several network endpoints serving the same tracker policy.

Run separate tracker processes when endpoints require genuinely independent
tracker state or policy. Examples include:

- a private HTTP tracker and a public UDP tracker;
- separate torrent/peer populations or databases;
- independent whitelist or authentication-key state;
- different private, listed, announce, or tracker policies; or
- independent UDP banning budgets.

Separate processes have their own `AppContainer` and therefore their own
tracker-core, shared-service, and persistence boundaries. They do not receive
the resource-sharing or cross-protocol swarm visibility that listener instances
within one process provide.

## Related Documents

- [Runtime architecture index](README.md)
- [Event topology and metrics policy](events.md)
- [Package architecture](../packages.md)
- [Application jobs and task ownership](../application-jobs.md)
- [Shared services ADR](../adrs/20260727180000_shared_services_across_tracker_instances.md)
