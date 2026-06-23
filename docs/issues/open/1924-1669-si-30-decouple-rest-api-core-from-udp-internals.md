---
doc-type: spec
issue-type: task
status: completed
priority: p2
epic: 1669
github-issue: 1924
spec-path: docs/issues/open/1924-1669-si-30-decouple-rest-api-core-from-udp-internals.md
last-updated-utc: 2026-06-23
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-06-10.md
    - docs/issues/open/1903-1669-si-23-relocate-axum-rest-api-server-test-environment.md
    - docs/issues/open/1930-1669-si-33-rest-api-contract-first-architecture.md
---

# Issue #1924 - Decouple `rest-api-core` and `axum-rest-api-server` from Concrete UDP Server Internals

## Subissue of EPIC #1669 — Overhaul: Packages

**Note**: this is a **production code** decoupling focused on the UDP side.
It extracts trait abstractions from concrete UDP types so the REST layer can
depend on stable interfaces instead of internal details. The REST-side wiring
of these traits (container, services, handlers) is deferred to
[SI-33 (contract-first REST API architecture)](1930-1669-si-33-rest-api-contract-first-architecture.md).

This issue does **not** remove `udp-server` or `udp-core` from Cargo.toml
files — the REST API is an orchestrating service that legitimately depends on
other services. The goal is interface segregation, not Cargo.toml decoupling.

Implemented after the `environment.rs` relocations (subissues SI-23/SI-24/SI-25) — those were
test infrastructure moves, while this is a production dependency decoupling.

## Problem

Two packages import concrete UDP types, forcing runtime dependencies on `udp-server` and
`udp-core`:

### `rest-api-core`

**Production imports** in `src/container.rs` and `src/statistics/services.rs`:

| Import                                      | Location              |
| ------------------------------------------- | --------------------- |
| `BanService`                                | `container.rs`        |
| `UdpTrackerCoreContainer`                   | `container.rs`        |
| `UdpTrackerServerContainer`                 | `container.rs`        |
| `udp_stats_repository` types (`Repository`) | `container.rs`        |
| `BanService`                                | `statistics/services` |
| `udp_server::statistics`                    | `statistics/services` |

**Test-only imports** (follow from production deps):

| Import                            | Concern                     |
| --------------------------------- | --------------------------- |
| `MAX_CONNECTION_ID_ERRORS_PER_IP` | Test ban init constant      |
| `BanService` (concrete)           | Test BanService constructor |

### `axum-rest-api-server`

**Production imports** in `src/v1/context/stats/handlers.rs`:

| Import                                                           | Concern                          |
| ---------------------------------------------------------------- | -------------------------------- |
| `BanService`                                                     | Handler state type               |
| `torrust_tracker_udp_server::statistics::repository::Repository` | Handler state type (get_stats)   |
| `torrust_tracker_udp_core::statistics::repository::Repository`   | Handler state type (get_metrics) |

**Production references** in `src/v1/context/stats/routes.rs`:

| Reference                                        | Concern                   |
| ------------------------------------------------ | ------------------------- |
| `http_api_container.ban_service`                 | Passed into handler state |
| `http_api_container.udp_server_stats_repository` | Passed into handler state |
| `http_api_container.udp_core_stats_repository`   | Passed into handler state |

### Consequence

Both `rest-api-core/Cargo.toml` and `axum-rest-api-server/Cargo.toml` list
`udp-server` and `udp-core` as runtime dependencies. Since the REST API is an
orchestrating service that legitimately depends on other services, these
dependency arrows are architecturally sound. The concern is not
**which** package the API depends on, but **how** it depends on it — through
concrete types and internal accessor methods rather than stable trait
interfaces.

## Deep Coupling Analysis

This section documents exactly what the REST layer uses from each UDP concrete type,
and identifies the architectural violations.

### 1. `BanService` (from `udp-core`)

The REST layer calls **one single method** in `get_labeled_metrics`:

```rust
ban_service.read().await.get_banned_ips_total()  // → returns usize
```

It only needs the total count of banned IPs — a single `usize`. The banning
implementation details (Bloom filters, HashMaps, reset timestamps, connection
ID error thresholds) are entirely irrelevant to the API.

Note: `BanService` is **not a generic banning service**. It is UDP-specific —
it bans IPs that send invalid connection IDs, which is how the UDP tracker
authenticates clients. The name may have been chosen anticipating more
banning reasons in the future, but currently it only bans for that reason.

### 2. `udp-core::statistics::repository::Repository`

The REST layer calls `.get_stats().await` then accesses `.metric_collection`
(a `MetricCollection`) and merges it into the global metrics collection.
That's the full usage.

### 3. `udp-server::statistics::repository::Repository`

**Two usage patterns**, depending on the function:

**`get_metrics`**: Calls `.get_stats().await` then calls **~15 typed accessor
methods** on the returned `Metrics` struct:

```rust
udp_server_stats.udp_requests_aborted_total()
udp_server_stats.udp_requests_banned_total()
udp_server_stats.udp_banned_ips_total()
udp_server_stats.udp_avg_connect_processing_time_ns_averaged()
udp_server_stats.udp_avg_announce_processing_time_ns_averaged()
udp_server_stats.udp_avg_scrape_processing_time_ns_averaged()
udp4_requests, udp4_connections_handled, udp4_announces_handled, ...
udp6_requests, udp6_connections_handled, udp6_announces_handled, ...
```

**`get_labeled_metrics`**: Calls `.get_stats().await` then accesses
`.metric_collection` and merges it — same pattern as udp-core stats.

### Corrected Architectural Understanding

The workspace hosts **three main service stacks** (plus minor ones like health check):

```output
REST API service:     server (axum-rest-api-server) → core (rest-api-core) → protocol (future)
UDP tracker service:  server (udp-server)           → core (udp-core)       → protocol (udp-protocol)
HTTP tracker service: server (axum-http-server)     → core (http-core)      → protocol (http-protocol)
```

Each service has its own internal **server → core → protocol** layering. But the
REST API is an **orchestrating service** that sits conceptually on top of the
UDP and HTTP trackers — it collects metrics and manages configuration from both.

Therefore, `rest-api-core` depending on `udp-server` is **not a layer inversion**.
It is a cross-service dependency from a higher-level orchestrating service's core
to a lower-level service's server. This is architecturally sound.

The real problem is not **which** package the API depends on, but **how** it
depends on it — through concrete types and internal accessor methods rather than
stable trait interfaces.

### Identified Problems

**Problem 1 (banning stats leak)**: The API needs `get_banned_ips_total()`
(a single `usize` from the ban service), but must import the entire `BanService`
concrete type — including its Bloom filter internals, constructors, and reset
logic. The API has no business knowing any of that.

**Problem 2 (UDP server metrics interface leak)**: The API stats layer makes 15+
typed method calls on the UDP server's `Metrics` struct (`udp4_announces_handled()`,
`udp6_connections_handled()`, etc.). If the UDP server renames any of these
methods or changes the metric structure, the API breaks. The API should only
need to consume metrics data through a stable interface, not know the exact
accessor API of each subsystem.

**Problem 3 (UDP core stats repository leak)**: The API directly imports
`udp_core::statistics::repository::Repository` just to call `.get_stats().await`
and access `.metric_collection`. This is a stable enough pattern (the method
returns a `MetricCollection`), but still creates a direct dependency on the
concrete repository type.

**Problem 4 (hardcoded constant propagation)**: `MAX_CONNECTION_ID_ERRORS_PER_IP`
is a hardcoded constant in `udp-core` that propagates into `rest-api-core`'s
test code, creating a fragile cross-package constant dependency.

## Relationship to Other Issues

This issue is a **prerequisite** for
[SI-33 (contract-first REST API architecture)](1930-1669-si-33-rest-api-contract-first-architecture.md).
That issue will restructure `rest-api-core` and `axum-rest-api-server` into new
contract/application/adapter packages, and will wire the UDP-side traits
defined here through the new architecture.

**Boundary**:

| Responsibility                                          | SI-30 (#1924) | Contract-first (#1930)              |
| ------------------------------------------------------- | ------------- | ----------------------------------- |
| `BanningStats` trait in `udp-core`                      | ✅ Define     | ❌ Inherits                         |
| `UdpCoreStatsRepository` trait in `udp-core`            | ✅ Define     | ❌ Inherits                         |
| `UdpServerStatsRepository` trait in `udp-server`        | ✅ Define     | ❌ Inherits                         |
| BanService trait impls in `udp-core`                    | ✅ Implement  | ❌ Inherits                         |
| Stats repo trait impls in `udp-core`/`udp-server`       | ✅ Implement  | ❌ Inherits                         |
| `MAX_CONNECTION_ID_ERRORS_PER_IP` → config              | ✅ Move       | ❌ Inherits                         |
| Refactor `get_protocol_metrics()` to `MetricCollection` | ✅ Do         | ❌ Inherits                         |
| `rest-api-core` container stores trait objects          | ❌ Defer      | ✅ Wire through new adapter         |
| `rest-api-core` services use trait refs                 | ❌ Defer      | ✅ Wire through new use-case layer  |
| `axum-rest-api-server` handlers use trait objects       | ❌ Defer      | ✅ Wire through new transport layer |

## Design Discussion

Two approaches were considered:

**Approach A — Trait-based interface segregation**: Define minimal traits in a
shared location (e.g. `tracker-core` or the source packages) and have the REST
layer depend on trait types (`Arc<dyn BanningService>`, `Arc<dyn UdpServerStatsRepository>`)
instead of concrete imports. This hides internal details while keeping Cargo.toml
dependency arrows intact.

**Approach B — Dependency arrow removal**: Move all traits to a shared neutral
package so that `rest-api-core` and `axum-rest-api-server` no longer have
`udp-server`/`udp-core` as runtime deps. This would eliminate the cross-service
dependency edge entirely.

**Decision**: Approach A (interface segregation only, no Cargo.toml decoupling).

The cross-service dependency from the REST API (an orchestrating service) into
the UDP tracker is architecturally sound. The goal is interface segregation.

**Trait location**: Traits are defined in their source packages (`udp-core` or
`udp-server`) alongside the concrete implementations. Since we are keeping the
Cargo.toml runtime dependencies, there is no benefit to extracting traits into
a neutral shared package.

**Trait naming**: `BanningStats` — naming reflects that the API only needs
aggregate statistics about banning (currently `get_banned_ips_total()`), not
control over the banning service itself.

## Scope

### 1. Add decisions to DECISIONS.md

Record the decision (Approach A — interface segregation only, no Cargo.toml
decoupling) as the next available DEC number.

### 2. Trait extraction for `BanService`

Define a minimal `BanningStats` trait in `udp-core` exposing only what the REST
layer needs:

```rust
pub trait BanningStats {
    /// Returns the total number of banned IPs.
    fn get_banned_ips_total(&self) -> usize;
}
```

`impl BanningStats for BanService` in the same crate (`udp-core`).

The API calls:

```rust
ban_service.read().await.get_banned_ips_total()  // returns usize
```

Since the `RwLock` wrapping is a container concern (not part of the business
logic), the trait keeps a sync `fn` signature — the calling code remains
responsible for acquiring the lock.

### 3. Trait extraction for UDP core stats

The API calls:

```rust
let stats = udp_stats_repository.get_stats().await;
metrics.merge(&stats.metric_collection)
```

Define a trait in `udp-core`'s statistics module:

```rust
#[async_trait]
pub trait UdpCoreStatsRepository: Send + Sync {
    async fn get_stats(&self) -> MetricCollection;
}
```

`impl UdpCoreStatsRepository for udp_core::statistics::repository::Repository` in `udp-core`.

The return type `MetricCollection` comes from the standalone
[`torrust-metrics`](https://github.com/torrust/torrust-metrics) crate, which is
already a dependency. This is a stable, generic metrics abstraction that all
service layers already use internally.

### 4. Trait extraction for UDP server stats

The API accesses two usage patterns:

- `get_metrics()`: 15+ typed accessor methods (`udp_requests_aborted_total()`,
  `udp4_announces_handled()`, etc.)
- `get_labeled_metrics()`: `.get_stats().await.metric_collection` merge

Since `MetricCollection` is already a stable generic abstraction from an
extracted standalone crate, the trait exposes `get_stats() -> MetricCollection`.
The `get_metrics()` function will be refactored to extract values directly from
the `MetricCollection` instead of calling typed accessor methods on the
concrete `Metrics` struct.

Define a trait in `udp-server`'s statistics module:

```rust
#[async_trait]
pub trait UdpServerStatsRepository: Send + Sync {
    async fn get_stats(&self) -> MetricCollection;
}
```

`impl UdpServerStatsRepository for udp_server::statistics::repository::Repository` in `udp-server`.

### 5. Turn `MAX_CONNECTION_ID_ERRORS_PER_IP` into a configuration option

Move `MAX_CONNECTION_ID_ERRORS_PER_IP` from a hardcoded `pub const` in `udp_core` to
a new config field in the `UdpTracker` configuration struct
(`packages/configuration/src/v2_0_0/udp_tracker.rs`):

- Add field `pub max_connection_id_errors_per_ip: u32` with default `10` via
  `#[serde(default)]` (or `#[serde(default = "default_max_connection_id_errors")]`).
- `UdpTrackerCoreContainer` already holds `Arc<UdpTracker>`, so `container.rs`
  reads `udp_tracker_config.max_connection_id_errors_per_ip` instead of the constant.
- Tests in `rest-api-core` use a literal `10` (or a local test constant) instead of
  importing `MAX_CONNECTION_ID_ERRORS_PER_IP` from `udp_core`.

This eliminates a fragile cross-package constant dependency and is more aligned
with the project's observability and testability principles (configuration-driven,
not hardcoded).

### 6. Update `udp-server` and `udp-core`

- Implement the new traits on their existing concrete types.
- `impl BanningStats for BanService` (or the chosen trait name) in `udp-core`.
- `impl UdpCoreStatsRepository for udp_core::statistics::repository::Repository`.
- `impl UdpServerStatsRepository for udp_server::statistics::repository::Repository`.

### 7. Clean up

- Run `cargo test --workspace`.
- Run `linter all`.

## Acceptance Criteria

1. `BanningStats` trait defined in `udp-core` and implemented on `BanService`.
2. `UdpCoreStatsRepository` trait defined in `udp-core` and implemented on stats `Repository`.
3. `UdpServerStatsRepository` trait defined in `udp-server` and implemented on stats `Repository`.
4. `MAX_CONNECTION_ID_ERRORS_PER_IP` removed from `rest-api-core` test code
   (replaced by a local literal or config-driven value).
5. `cargo test --workspace` passes.
6. `linter all` passes.

## Out of Scope

- Extracting any UDP package to a standalone repository.
- Changing the HTTP tracker side of the REST layer.
- Relocating test environments (already done in SI-23/SI-24/SI-25).
- Removing `udp-server` or `udp-core` from Cargo.toml files (the REST API is
  an orchestrating service that legitimately depends on other services).
- Restructuring the REST API's own server/core/protocol layers — this is
  tracked by [SI-33 (contract-first REST API architecture)](1930-1669-si-33-rest-api-contract-first-architecture.md).
- Changing the HTTP tracker side of the REST layer.
- Relocating test environments (already done in SI-23/SI-24/SI-25).
- Removing `udp-server` or `udp-core` from Cargo.toml files (the REST API is
  an orchestrating service that legitimately depends on other services).
- Restructuring the REST API's own server/core/protocol layers (tracked in a
  separate spec).

## Verification

- [ ] DEC recorded in `docs/issues/open/1669-overhaul-packages/DECISIONS.md`
- [ ] `BanningStats` trait defined in `udp-core` and implemented on `BanService`
- [ ] `UdpCoreStatsRepository` trait defined in `udp-core` and implemented on stats `Repository`
- [ ] `UdpServerStatsRepository` trait defined in `udp-server` and implemented on stats `Repository`
- [ ] `MAX_CONNECTION_ID_ERRORS_PER_IP` added as a configuration option in `UdpTracker` config struct (default `10`)
- [ ] `get_protocol_metrics()` refactored to extract from `MetricCollection` instead of typed accessors
- [ ] `cargo test --workspace` — pass
- [ ] `linter all` — pass
