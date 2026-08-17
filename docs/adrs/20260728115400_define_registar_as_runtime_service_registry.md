---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - docs/issues/closed/2036-add-runtime-service-registry-metadata/ISSUE.md
    - src/container.rs
    - tests/common/mod.rs
    - packages/axum-health-check-api-server/src/handlers.rs
---

# Define Registar as the runtime service registry

## Description

Services configured with port zero receive their final listener address only after the operating
system binds their socket. The tracker needs a single, side-effect-free way for internal consumers
to discover those running services. The health check API already receives such information through
`Registar`, but the current registration record only contains a `ServiceBinding` and a function to
run a health check. Service role metadata is instead constructed while a health check runs.

This forces consumers that need endpoint discovery, including main-application integration tests,
to infer service identity from bind IP conventions, registry iteration order, logs, or health-check
side effects. None is a valid application contract.

## Agreement

`Registar` is the authoritative internal registry of services that have successfully started. A
registration contains immutable local runtime metadata:

- `ServiceBinding`: the listener protocol and final socket address;
- service role: the application role implemented by the listener; and
- optional health-check behavior.

`ServiceBinding`, its socket address, and service role describe different facts. The binding is
derivable from `ServiceBinding` and must not be stored separately in the registration. Service role
cannot reconstruct `ServiceBinding`, because, for example, an HTTP tracker role may listen using
either HTTP or HTTPS.

The role set is owned by the tracker, not by generic network primitives or `torrust-server-lib`.
Tracker packages use a shared `ServiceRole` enum to define canonical role names. The standalone
server library stores the resulting opaque role name and remains usable by applications with other
role sets.

`AppContainer` retains ownership of application composition and boot-time configuration.
`JobManager` retains ownership of task lifecycle, cancellation, and shutdown. Neither replaces the
runtime registry. `ServiceRegistrationForm` remains the sole service-to-parent reporting channel;
no parallel registry or reporting type is introduced.

The registry exposes local process listener data only. It does not represent public URLs, reverse
proxy routes, DNS names, load balancers, or other deployment topology.

The health check API is a registry consumer. It obtains immutable identity metadata from the
registration and executes only optional health-check behavior. A metadata query must not itself
perform a health check.

## Alternatives Considered

### Infer service identity from listener IP address or protocol

Rejected because multiple valid services can share HTTP, HTTPS, or the same bind IP. Deployment
configuration must not become a service-identity contract.

### Derive identity from `AppContainer` service counts

Rejected because configuration containers and runtime registrations have no stable identity mapping,
and `HashMap` iteration order is unspecified.

### Use a health check to retrieve service metadata

Rejected because it performs network I/O, reports health rather than identity, and fails exactly
when metadata is most useful for diagnosing an unhealthy service.

### Add a separate runtime registry

Rejected because `Registar` and `ServiceRegistrationForm` already provide the required runtime
service-to-parent reporting flow. A second registry would duplicate state and create consistency
risk.

### Put tracker service roles in `torrust-net-primitives` or `torrust-server-lib`

Rejected because `http_tracker`, `tracker_rest_api`, and `udp_tracker` are tracker application
roles, not generic network or server-library concepts.

## Consequences

- Internal consumers can discover final runtime bindings by role without log parsing or IP-based
  conventions.
- Main-application integration tests can use port zero for all listeners and reliably target the
  intended service.
- Health-check reporting has one source of truth for stable metadata.
- The change requires coordinated versioning: `torrust-server-lib` must release the registry API
  before the tracker updates its dependency and migrates its server packages.

## Date

2026-07-28

## References

- Issue #1419: [main-application integration tests](../issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md)
- Feature #2036: [add runtime service registry metadata](../issues/open/2036-add-runtime-service-registry-metadata/ISSUE.md)
- [Investigation: runtime service registration and health check API](../issues/open/1419-allow-multiple-integration-tests-at-main-app-level/investigation-registar-and-health-check.md)
- [App container](../../src/container.rs)
- [Integration-test helpers](../../tests/common/mod.rs)
- [Health-check handler](../../packages/axum-health-check-api-server/src/handlers.rs)
