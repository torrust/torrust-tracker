# Investigation: Runtime Service Registration and Health Check API

## Goal

Determine how the application can expose runtime-discovered service identity and final bindings to its internal consumers. This is needed to identify services reliably in integration tests when configured with port zero, without parsing logs or inferring identity from configured IP addresses.

## Running tracker

Started with config: all services on `0.0.0.0:0` (port zero).

```text
HTTP TRACKER: Started on: http://0.0.0.0:33303
HTTP TRACKER: Started on: http://0.0.0.0:52633
API:          Started on: http://0.0.0.0:46715
HEALTH CHECK: Started on: http://0.0.0.0:46199
```

## Health check API response

The health check API endpoint returns service type information:

```json
{
  "status": "Ok",
  "message": "",
  "details": [
    {
      "service_binding": "http://0.0.0.0:33303/",
      "binding": "0.0.0.0:33303",
      "service_type": "http_tracker",
      "info": "checking http tracker health check at: http://0.0.0.0:33303/health_check",
      "result": { "Ok": "200 OK" }
    },
    {
      "service_binding": "http://0.0.0.0:52633/",
      "binding": "0.0.0.0:52633",
      "service_type": "http_tracker",
      "info": "checking http tracker health check at: http://0.0.0.0:52633/health_check",
      "result": { "Ok": "200 OK" }
    },
    {
      "service_binding": "http://0.0.0.0:46715/",
      "binding": "0.0.0.0:46715",
      "service_type": "tracker_rest_api",
      "info": "checking api health check at: http://0.0.0.0:46715/api/health_check",
      "result": { "Ok": "200 OK" }
    }
  ]
}
```

### Key observations

- `service_type` values: `"http_tracker"`, `"tracker_rest_api"`, `"udp_tracker"`
- `info` strings contain the health check URL path, which differs by service type
- The health check API itself is NOT in the registar (it is the thing that queries the registar)

## Independent runtime metadata

The three identity-related fields in a health check report are not redundant. They describe separate facts about a service known to the tracker:

| Field             | Meaning                                               | Example                 |
| ----------------- | ----------------------------------------------------- | ----------------------- |
| `binding`         | The socket address on which the process is listening. | `0.0.0.0:33303`         |
| `service_binding` | The local listener protocol plus its socket address.  | `http://0.0.0.0:33303/` |
| `service_type`    | The tracker role implemented by that listener.        | `http_tracker`          |

`binding` alone does not identify a service role or protocol. `service_type` cannot reconstruct `service_binding`: an HTTP tracker may use either HTTP or HTTPS, and the role is intentionally independent of the transport protocol. The combination of `service_binding` and `service_type` is therefore required to identify both how the process listens and what it serves.

These values describe only local runtime state managed by the tracker. They do not claim to be public client endpoints. A deployment may place a reverse proxy, load balancer, domain name, different public IP address, or path routing in front of the process. Public endpoint configuration is outside this registry's scope, including any optional public URL configuration introduced elsewhere. The registry records only the mandatory data needed to run and inspect the local services.

## Current Registar structure

`ServiceRegistration` stores only:

- `service_binding: ServiceBinding` — the protocol + address
- `check_fn: FnSpawnServiceHeathCheck` — a function pointer to spawn health checks

The `service_type` and `info` fields are only in `ServiceHealthCheckJob`, which is created by calling `spawn_check()` on the registration. This makes an HTTP request as a side effect.

The registar uses `HashMap<ServiceBinding, ServiceRegistration>`. There is no service type information on the key or value.

## Service type constants

Each server package defines its own type string constant:

| Package                | Constant      | Value                |
| ---------------------- | ------------- | -------------------- |
| `axum-http-server`     | `TYPE_STRING` | `"http_tracker"`     |
| `axum-rest-api-server` | `TYPE_STRING` | `"tracker_rest_api"` |
| `udp-server`           | `TYPE_STRING` | `"udp_tracker"`      |

These are passed to `ServiceHealthCheckJob::new()` but **not** stored in `ServiceRegistration`.

### Candidate tracker-owned service role enum

The duplicated constants should become one tracker-owned enum, tentatively named `ServiceRole` to distinguish it from the network `Protocol` stored in `ServiceBinding`:

```rust
pub enum ServiceRole {
  HttpTracker,
  TrackerRestApi,
  UdpTracker,
}

impl ServiceRole {
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::HttpTracker => "http_tracker",
      Self::TrackerRestApi => "tracker_rest_api",
      Self::UdpTracker => "udp_tracker",
    }
  }
}
```

`torrust-tracker-primitives` is a viable home because all three registering server packages already depend on it. `torrust-net-primitives` must not own this enum because the role values are tracker-specific, not generic network concepts.

`torrust-server-lib` must remain decoupled from this closed tracker role set. The tracker packages should convert `ServiceRole` to its canonical string when constructing a `ServiceRegistration`; the generic registry stores that opaque role name and exposes it to its consumers. This preserves a single source of truth for tracker role names without making the standalone server library depend on tracker packages.

## Architectural reassessment

`Registar` was introduced for the health check API, but its data flow represents a broader responsibility: it receives information from services only after those services have started and therefore knows their final runtime bindings. In particular, it is the existing parent-side destination for bindings selected by the operating system when a service is configured with port zero.

The health check API is one consumer of that runtime service information. Integration tests are another legitimate internal consumer: after `app::run()` has started services, a test needs to discover the concrete tracker and REST API endpoints in order to exercise them. Requiring either consumer to parse application logs, perform a health check merely to obtain metadata, or assume a particular bind IP is an API design gap.

The responsibilities should remain distinct:

- `AppContainer` owns application composition and boot-time configuration. It can describe which services are intended to run, but cannot by itself provide runtime facts that only exist after binding, such as an OS-assigned port.
- `Registar` owns the registry of runtime-discovered services and their stable descriptive metadata.
- `JobManager` owns task lifecycle management, including cancellation and shutdown. It must not become a service-information registry.
- Service-specific parent-child command channels remain appropriate where their behavior is specific to that service. They do not need to be generalized into the registry.

The existing `ServiceRegistrationForm` is the appropriate child-to-parent channel for this information. Introducing a second registry or a new, parallel reporting type would duplicate that established startup flow without adding a useful separation of responsibilities.

## Rejected approaches

### Infer identity from the bind IP address

This is the current test-only workaround: HTTP trackers use an unspecified address while the REST API and health check API use different loopback addresses. It is invalid as a production contract because users may legitimately configure any of these services with the same valid bind address, including `0.0.0.0`.

### Derive service identity from `AppContainer` counts

For example, selecting the first HTTP registrations based on `AppContainer.http_tracker_instance_containers.len()` is fragile. The registry contains multiple HTTP services, `HashMap` ordering is unspecified, and the relationship between configured service containers and runtime registrations is not an identity contract.

### Reuse health checks as metadata queries

Calling `spawn_check()` merely to obtain `service_type` makes a network request and couples a metadata query to service health. A registry lookup must be side-effect free and usable even when a service is unhealthy.

### Add a separate runtime registry

This would duplicate the registration form and service-to-parent reporting path that already delivers the final runtime binding. The existing `Registar` should be evolved instead.

## Design direction

`ServiceRegistration` should represent a running service's registration record, not only the data required to spawn a health check. It needs enough immutable metadata for consumers to identify the service and use its final binding without executing the health-check function.

The tracker-owned `ServiceRole` enum should be the source of truth for currently supported tracker roles. `ServiceRegistration` in `torrust-server-lib` should store its canonical string form, keeping the generic crate independent from tracker packages. The health API can serialize that stored value using its existing `service_type: String` response field.

The desired consumer outcome is conceptually:

```rust
let tracker_services = container.registar.services_of_type(ServiceType::HttpTracker).await;
```

Each returned entry must provide the final `ServiceBinding`. Tests can then translate an unspecified listener address to a loopback client URL while retaining its runtime-assigned port. This removes assumptions about IP addresses, registry iteration order, protocol-only matching, and health-check side effects.

## Questions for implementation design

1. Should the generic role name in `torrust-server-lib` remain a `String` or become a generic validated newtype around `String`?
2. Should `ServiceRegistration` expose its own immutable metadata, or should `Registar` provide read-only query methods and hide the storage representation?
3. Which metadata is registration-time data versus health-check-execution data? Role and `ServiceBinding` are registration-time data; `info` and the result may remain health-check data.
4. Should `ServiceHealthCheckJob` stop carrying `service_binding` and `service_type`, so the health API obtains immutable identity metadata directly from the registration record?
5. How will the tracker update its dependency on the standalone `torrust-server-lib` crate once the registry API is finalized?

## Decision and Implementation Handoff

This investigation remains the record of observed current behavior, the discovered limitation, and
the reasoning that led to the change. The approved architectural boundary is defined by
[ADR 20260728115400](../../../adrs/20260728115400_define_registar_as_runtime_service_registry.md).
The ordered implementation and validation work is defined by the
[runtime service registry refactor plan](../../../refactor-plans/open/1419-runtime-service-registry-refactor.md).
