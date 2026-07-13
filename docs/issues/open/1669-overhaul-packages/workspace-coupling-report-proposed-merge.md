---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md
    - packages/
---

# Workspace Coupling Report — Proposed Protocol and Core Merge

**Status**: Hypothetical — this report shows what the coupling graph would look like
**if** the following two changes were applied to the workspace. It does **not** represent
an agreed decision.

**Source report**: [workspace-coupling-report.md](workspace-coupling-report.md)
(generated 2026-05-19 20:46 UTC; 29 packages)

---

## Changes being modelled

### Change 1 — Protocol merge

Merge the two protocol packages into a single crate with two features
(`udp` and `http`, both disabled by default):

| Before                             | After                         |
| ---------------------------------- | ----------------------------- |
| `packages/udp-protocol`            | _(removed)_                   |
| `packages/http-protocol`           | _(removed)_                   |
| _(new)_                            | `packages/protocol`           |
| `bittorrent-udp-tracker-protocol`  | _(crate deleted)_             |
| `bittorrent-http-tracker-protocol` | _(crate deleted)_             |
| _(new crate)_                      | `bittorrent-tracker-protocol` |

### Change 2 — Protocol-specific core merge

Merge the two protocol-specific core packages into the existing common core
(`packages/tracker-core` / `bittorrent-tracker-core`) with two features
(`udp` and `http`, both disabled by default):

| Before                         | After                                                               |
| ------------------------------ | ------------------------------------------------------------------- |
| `packages/udp-core`    | _(removed)_                                                         |
| `packages/http-core`   | _(removed)_                                                         |
| `packages/tracker-core`        | `packages/tracker-core` (expanded)                                  |
| `bittorrent-udp-core`  | _(crate deleted)_                                                   |
| `bittorrent-http-core` | _(crate deleted)_                                                   |
| `bittorrent-tracker-core`      | `bittorrent-tracker-core` (expanded with `udp` and `http` features) |

**Net effect**: workspace shrinks from **29** to **25** packages.

---

## ⚠️ Circular dependency blocker

Before reading the rest of this report, note that Change 1 as described **cannot be
implemented without first resolving a circular crate dependency**.

The current `bittorrent-http-tracker-protocol` depends on `bittorrent-tracker-core` for
four error types:

```text
bittorrent_tracker_core::authentication::Error
bittorrent_tracker_core::error::AnnounceError
bittorrent_tracker_core::error::ScrapeError
bittorrent_tracker_core::error::WhitelistError
```

After the merges, the dependency chain would be:

```text
bittorrent-tracker-core [http feature]
    → bittorrent-tracker-protocol [http feature]   (needs protocol types)
        → bittorrent-tracker-core                   (needs error types)
```

Cargo does not support circular dependencies between crates; features do not break the
crate boundary. The compilation would fail.

**Prerequisite to unblock Change 1**: the four error types imported by
`bittorrent-http-tracker-protocol` must be moved out of `bittorrent-tracker-core` into a
crate that neither the merged protocol nor the merged core depends on (e.g.,
`torrust-tracker-primitives` or a new `bittorrent-tracker-errors` crate).

The rest of this document models the coupling graph **assuming that prerequisite has been
resolved** (the error types live somewhere else; the circular edge is gone). The
`bittorrent-tracker-core` dependency of `bittorrent-tracker-protocol` is therefore
**absent** in the tables below.

---

## How to read this report

Same convention as the source report. For packages that changed, modifications are
annotated with _(was: `old-dep`)_ or _(new)_.

**Signal**: a dependency with only 1–3 distinct import paths may be a candidate
for elimination (move the item, break the edge).

---

## Packages with no workspace dependencies

These packages are leaves (no workspace dep) and are prime extraction candidates.
No change from the source report.

- `bittorrent-peer-id`
- `torrust-net-primitives`
- `torrust-tracker-rest-api-client`
- `torrust-tracker-clock`
- `torrust-tracker-contrib-bencode`
- `torrust-tracker-events`
- `torrust-tracker-located-error`
- `workspace-coupling`

---

## Package coupling details

### `bittorrent-tracker-protocol` _(new — merged from udp-protocol + http-protocol)_

Workspace deps: **3** (down from 6 combined across the two source packages)

The `udp` feature activates the UDP tracker protocol implementation; the `http` feature
activates the HTTP tracker protocol implementation. Both are disabled by default.

#### `bittorrent-peer-id` [normal, `udp` feature]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or
glob import)._

#### `torrust-tracker-contrib-bencode` [normal, `http` feature]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or
glob import)._

#### `torrust-tracker-located-error` [normal, `http` feature]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or
glob import)._

#### `torrust-tracker-clock` [normal, `http` feature]

- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::clock::Time`

#### `torrust-tracker-primitives` [normal, both features]

- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::ScrapeData`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::fixture`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

> **Note**: The four `bittorrent-tracker-core` error-type imports that previously appeared
> in `bittorrent-http-tracker-protocol` are absent here; they are assumed to have been
> relocated (see circular dependency blocker above).

---

### `bittorrent-tracker-core` _(expanded — absorbs udp-core and http-core as features)_

Workspace deps: **11** (up from 9 for the base package; `udp` and `http` features add
`bittorrent-tracker-protocol` and `torrust-net-primitives`)

The base code (always compiled) is unchanged. The `udp` and `http` features bring in the
logic that was previously in `bittorrent-udp-core` and
`bittorrent-http-core` respectively.

#### `bittorrent-tracker-protocol` [normal, `udp` and `http` features — _(new dep)_]

_`udp` feature_:

- `bittorrent_tracker_protocol::udp::AnnounceEvent::Completed`
- `bittorrent_tracker_protocol::udp::AnnounceEvent::None`
- `bittorrent_tracker_protocol::udp::AnnounceEvent::Started`
- `bittorrent_tracker_protocol::udp::AnnounceEvent::Stopped`
- `bittorrent_tracker_protocol::udp::AnnounceEvent::from`
- `bittorrent_tracker_protocol::udp::AnnounceRequest`
- `bittorrent_tracker_protocol::udp::ConnectionId`
- `bittorrent_tracker_protocol::udp::ScrapeRequest`
- `bittorrent_tracker_protocol::udp::common::InfoHash`

_`http` feature_:

- `bittorrent_tracker_protocol::http::v1::requests`
- `bittorrent_tracker_protocol::http::v1::services`

#### `torrust-net-primitives` [normal, `udp` and `http` features — _(new dep for base package)_]

- `torrust_net_primitives::service_binding`
- `torrust_net_primitives::service_binding::Protocol`
- `torrust_net_primitives::service_binding::ServiceBinding`

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::DurationSinceUnixEpoch`
- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::clock::Time`
- `torrust_tracker_clock::clock::stopped`
- `torrust_tracker_clock::conv::convert_from_timestamp_to_datetime_utc`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::Configuration`
- `torrust_tracker_configuration::Core`
- `torrust_tracker_configuration::Driver::MySQL`
- `torrust_tracker_configuration::Driver::PostgreSQL`
- `torrust_tracker_configuration::Driver::Sqlite3`
- `torrust_tracker_configuration::TORRENT_PEERS_LIMIT`
- `torrust_tracker_configuration::v2_0_0::core`

#### `torrust-tracker-events` [normal]

- `torrust_tracker_events::broadcaster::Broadcaster`
- `torrust_tracker_events::bus::EventBus`
- `torrust_tracker_events::bus::SenderStatus`
- `torrust_tracker_events::receiver::Receiver`
- `torrust_tracker_events::receiver::RecvError`
- `torrust_tracker_events::sender::SendError`
- `torrust_tracker_events::sender::Sender`

#### `torrust-tracker-located-error` [normal]

- `torrust_tracker_located_error::Located`
- `torrust_tracker_located_error::LocatedError`

#### `torrust-tracker-metrics` [normal]

- `torrust_tracker_metrics::label`
- `torrust_tracker_metrics::label::LabelSet`
- `torrust_tracker_metrics::label_name`
- `torrust_tracker_metrics::metric::MetricName`
- `torrust_tracker_metrics::metric::description`
- `torrust_tracker_metrics::metric_collection`
- `torrust_tracker_metrics::metric_collection::Error`
- `torrust_tracker_metrics::metric_collection::aggregate`
- `torrust_tracker_metrics::metric_name`
- `torrust_tracker_metrics::unit::Unit`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceData`
- `torrust_tracker_primitives::AnnounceEvent`
- `torrust_tracker_primitives::AnnouncePolicy`
- `torrust_tracker_primitives::NumberOfBytes`
- `torrust_tracker_primitives::NumberOfDownloads`
- `torrust_tracker_primitives::NumberOfDownloadsPerInfoHash`
- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::ScrapeData`
- `torrust_tracker_primitives::pagination::Pagination`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::Peer`
- `torrust_tracker_primitives::peer::PeerAnnouncement`
- `torrust_tracker_primitives::swarm_metadata`
- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::Registry`
- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`
- `torrust_tracker_swarm_coordination_registry::event::Event`
- `torrust_tracker_swarm_coordination_registry::event::receiver`
- `torrust_tracker_swarm_coordination_registry::statistics::event`

#### `torrust-tracker-rest-api-client` [dev]

_No `torrust_tracker_rest_api_client::` references found in source — may be used only in
`Cargo.toml` feature flags or `build.rs`._

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`
- `torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database`

---

### `bittorrent-tracker-client`

Workspace deps: **4** (unchanged count; `bittorrent-udp-tracker-protocol` → `bittorrent-tracker-protocol[udp]`)

#### `bittorrent-tracker-protocol` [normal — _(was: `bittorrent-udp-tracker-protocol`)_]

- `bittorrent_tracker_protocol::udp::PeerId`
- `bittorrent_tracker_protocol::udp::Request`

#### `torrust-net-primitives` [normal]

- `torrust_net_primitives::service_binding::ServiceBinding`

#### `torrust-tracker-located-error` [normal]

- `torrust_tracker_located_error::DynError`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::peer`

---

### `torrust-tracker-axum-health-check-api-server`

Workspace deps: **10** — unchanged. No dependency on the merged packages.

> Same as source report.

---

### `torrust-tracker-axum-http-server`

Workspace deps: **12** (down from 14; `bittorrent-http-core` and
`bittorrent-http-tracker-protocol` each collapse to one dep on the merged crates;
`bittorrent-udp-tracker-protocol` also collapses into `bittorrent-tracker-protocol`)

#### `bittorrent-tracker-core` [normal — _(was: `bittorrent-http-core` + `bittorrent-tracker-core`)_]

Merged: items from both former packages, now under `bittorrent-tracker-core` with the
`http` feature active.

- `bittorrent_tracker_core::announce_handler::AnnounceHandler`
- `bittorrent_tracker_core::authentication`
- `bittorrent_tracker_core::authentication::Key`
- `bittorrent_tracker_core::authentication::key`
- `bittorrent_tracker_core::authentication::service`
- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::databases::setup`
- `bittorrent_tracker_core::http::container::HttpTrackerCoreContainer`
- `bittorrent_tracker_core::http::event::bus`
- `bittorrent_tracker_core::http::event::sender`
- `bittorrent_tracker_core::http::services::announce`
- `bittorrent_tracker_core::http::services::scrape`
- `bittorrent_tracker_core::http::statistics::event`
- `bittorrent_tracker_core::http::statistics::repository`
- `bittorrent_tracker_core::scrape_handler::ScrapeHandler`
- `bittorrent_tracker_core::statistics::persisted`
- `bittorrent_tracker_core::torrent::repository`
- `bittorrent_tracker_core::whitelist::authorization`
- `bittorrent_tracker_core::whitelist::repository`

#### `bittorrent-tracker-protocol` [normal — _(was: `bittorrent-http-tracker-protocol` + `bittorrent-udp-tracker-protocol`)_]

- `bittorrent_tracker_protocol::http::v1`
- `bittorrent_tracker_protocol::http::v1::query`
- `bittorrent_tracker_protocol::http::v1::requests`
- `bittorrent_tracker_protocol::http::v1::responses`
- `bittorrent_tracker_protocol::http::v1::services`
- `bittorrent_tracker_protocol::udp::PeerId`

#### `torrust-tracker-axum-server` [normal]

- `torrust_tracker_axum_server::custom_axum_server`
- `torrust_tracker_axum_server::signals::graceful_shutdown`
- `torrust_tracker_axum_server::tsl::make_rust_tls`

#### `torrust-net-primitives` [normal]

- `torrust_net_primitives::service_binding`
- `torrust_net_primitives::service_binding::ServiceBinding`

#### `torrust-server-lib` [normal]

- `torrust_server_lib::logging::Latency`
- `torrust_server_lib::logging::STARTED_ON`
- `torrust_server_lib::registar`
- `torrust_server_lib::registar::Registar`
- `torrust_server_lib::signals`

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::initialize_static`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::Configuration`
- `torrust_tracker_configuration::Configuration::core`
- `torrust_tracker_configuration::TORRENT_PEERS_LIMIT`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceData`
- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::ScrapeData`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::fixture`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`

#### `torrust-tracker-clock` [dev]

- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::initialize_static`

#### `torrust-tracker-events` [dev]

_No `torrust_tracker_events::` references found in source — may be used only in `Cargo.toml`
feature flags or `build.rs`._

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`
- `torrust_tracker_test_helpers::configuration::ephemeral_public`
- `torrust_tracker_test_helpers::logging::logs_contains_a_line_with`

---

### `torrust-tracker-axum-rest-api-server`

Workspace deps: **15** (down from 16; `bittorrent-http-core` and
`bittorrent-udp-core` collapse into a single `bittorrent-tracker-core[http,udp]` dep)

#### `bittorrent-tracker-core` [normal — _(was: `bittorrent-http-core` + `bittorrent-udp-core` + `bittorrent-tracker-core`)_]

- `bittorrent_tracker_core::authentication`
- `bittorrent_tracker_core::authentication::Key`
- `bittorrent_tracker_core::authentication::handler`
- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::databases::SchemaMigrator`
- `bittorrent_tracker_core::error::PeerKeyError`
- `bittorrent_tracker_core::http::container::HttpTrackerCoreContainer`
- `bittorrent_tracker_core::http::statistics::repository`
- `bittorrent_tracker_core::statistics::repository`
- `bittorrent_tracker_core::torrent::repository`
- `bittorrent_tracker_core::torrent::services`
- `bittorrent_tracker_core::udp::MAX_CONNECTION_ID_ERRORS_PER_IP`
- `bittorrent_tracker_core::udp::container::UdpTrackerCoreContainer`
- `bittorrent_tracker_core::udp::initialize_static`
- `bittorrent_tracker_core::udp::services::banning`
- `bittorrent_tracker_core::udp::statistics::repository`
- `bittorrent_tracker_core::whitelist::manager`

#### `torrust-tracker-axum-server` [normal]

- `torrust_tracker_axum_server::custom_axum_server`
- `torrust_tracker_axum_server::signals::graceful_shutdown`
- `torrust_tracker_axum_server::tsl::make_rust_tls`

#### `torrust-net-primitives` [normal]

- `torrust_net_primitives::service_binding`

#### `torrust-tracker-rest-api-client` [normal]

- `torrust_tracker_rest_api_client::common::http`
- `torrust_tracker_rest_api_client::connection_info`
- `torrust_tracker_rest_api_client::connection_info::ConnectionInfo`
- `torrust_tracker_rest_api_client::v1::client`

#### `torrust-tracker-rest-api-core` [normal]

- `torrust_tracker_rest_api_core::container::TrackerHttpApiCoreContainer`
- `torrust_tracker_rest_api_core::statistics::metrics`
- `torrust_tracker_rest_api_core::statistics::services`

#### `torrust-server-lib` [normal]

- `torrust_server_lib::logging::Latency`
- `torrust_server_lib::logging::STARTED_ON`
- `torrust_server_lib::registar`
- `torrust_server_lib::registar::Registar`
- `torrust_server_lib::signals`

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::DurationSinceUnixEpoch`
- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::clock::stopped`
- `torrust_tracker_clock::conv::convert_from_iso_8601_to_timestamp`
- `torrust_tracker_clock::initialize_static`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::AccessTokens`
- `torrust_tracker_configuration::HttpApi`
- `torrust_tracker_configuration::HttpApi::tsl_config`

#### `torrust-tracker-metrics` [normal]

- `torrust_tracker_metrics::metric_collection::MetricCollection`
- `torrust_tracker_metrics::prometheus::PrometheusSerializable`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceEvent`
- `torrust_tracker_primitives::pagination::Pagination`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::fixture`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`
- `torrust_tracker_swarm_coordination_registry::statistics::repository`

#### `torrust-tracker-udp-server` [normal]

- `torrust_tracker_udp_server::container::UdpTrackerServerContainer`
- `torrust_tracker_udp_server::statistics::repository`

#### `torrust-tracker-rest-api-client` [dev]

- `torrust_tracker_rest_api_client::common::http`
- `torrust_tracker_rest_api_client::connection_info`
- `torrust_tracker_rest_api_client::connection_info::ConnectionInfo`
- `torrust_tracker_rest_api_client::v1::client`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration::ephemeral_public`
- `torrust_tracker_test_helpers::logging::logs_contains_a_line_with`

---

### `torrust-tracker-axum-server`

Workspace deps: **3** — unchanged. No dependency on the merged packages.

> Same as source report.

---

### `torrust-tracker-rest-api-core`

Workspace deps: **9** (down from 10; `bittorrent-http-core` and
`bittorrent-udp-core` collapse into `bittorrent-tracker-core[http,udp]`)

#### `bittorrent-tracker-core` [normal — _(was: `bittorrent-http-core` + `bittorrent-udp-core` + `bittorrent-tracker-core`)_]

- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::http::container::HttpTrackerCoreContainer`
- `bittorrent_tracker_core::http::event::bus`
- `bittorrent_tracker_core::http::event::sender`
- `bittorrent_tracker_core::http::statistics::event`
- `bittorrent_tracker_core::http::statistics::repository`
- `bittorrent_tracker_core::statistics::repository`
- `bittorrent_tracker_core::torrent::repository`
- `bittorrent_tracker_core::udp::MAX_CONNECTION_ID_ERRORS_PER_IP`
- `bittorrent_tracker_core::udp::container::UdpTrackerCoreContainer`
- `bittorrent_tracker_core::udp::services::banning`
- `bittorrent_tracker_core::udp::statistics::repository`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::Configuration`

#### `torrust-tracker-metrics` [normal]

- `torrust_tracker_metrics::metric_collection::MetricCollection`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`
- `torrust_tracker_swarm_coordination_registry::statistics::repository`

#### `torrust-tracker-udp-server` [normal]

- `torrust_tracker_udp_server::container::UdpTrackerServerContainer`
- `torrust_tracker_udp_server::statistics`
- `torrust_tracker_udp_server::statistics::repository`

#### `torrust-tracker-events` [dev]

- `torrust_tracker_events::bus::SenderStatus`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`

---

### `torrust-server-lib`

Workspace deps: **1** — unchanged.

> Same as source report.

---

### `torrust-tracker`

Workspace deps: **14** (down from 16; `bittorrent-http-core` and
`bittorrent-udp-core` collapse into `bittorrent-tracker-core[http,udp]`)

#### `bittorrent-tracker-core` [normal — _(was: `bittorrent-http-core` + `bittorrent-udp-core` + `bittorrent-tracker-core`)_]

- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::http::container`
- `bittorrent_tracker_core::http::container::HttpTrackerCoreContainer`
- `bittorrent_tracker_core::http::statistics::event`
- `bittorrent_tracker_core::statistics::event`
- `bittorrent_tracker_core::statistics::persisted`
- `bittorrent_tracker_core::torrent::manager`
- `bittorrent_tracker_core::udp::UDP_TRACKER_LOG_TARGET`
- `bittorrent_tracker_core::udp::container`
- `bittorrent_tracker_core::udp::container::UdpTrackerCoreContainer`
- `bittorrent_tracker_core::udp::crypto::keys`
- `bittorrent_tracker_core::udp::initialize_static`
- `bittorrent_tracker_core::udp::statistics::event`

#### `torrust-tracker-axum-health-check-api-server` [normal]

- `torrust_tracker_axum_health_check_api_server::HEALTH_CHECK_API_LOG_TARGET`

#### `torrust-tracker-axum-http-server` [normal]

- `torrust_tracker_axum_http_server::HTTP_TRACKER_LOG_TARGET`
- `torrust_tracker_axum_http_server::Version`
- `torrust_tracker_axum_http_server::Version::V1`
- `torrust_tracker_axum_http_server::server`

#### `torrust-tracker-axum-rest-api-server` [normal]

- `torrust_tracker_axum_rest_api_server::Version`
- `torrust_tracker_axum_rest_api_server::Version::V1`
- `torrust_tracker_axum_rest_api_server::server`
- `torrust_tracker_axum_rest_api_server::v1::context`

#### `torrust-tracker-axum-server` [normal]

- `torrust_tracker_axum_server::tsl::make_rust_tls`

#### `torrust-tracker-rest-api-client` [normal]

- `torrust_tracker_rest_api_client::connection_info`
- `torrust_tracker_rest_api_client::v1::Client`
- `torrust_tracker_rest_api_client::v1::client`

#### `torrust-tracker-rest-api-core` [normal]

- `torrust_tracker_rest_api_core::container::TrackerHttpApiCoreContainer`

#### `torrust-server-lib` [normal]

- `torrust_server_lib::logging::STARTED_ON`
- `torrust_server_lib::registar::Registar`
- `torrust_server_lib::registar::ServiceRegistrationForm`
- `torrust_server_lib::registar::ServiceRegistry`
- `torrust_server_lib::signals`

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::clock::Time`
- `torrust_tracker_clock::initialize_static`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::AccessTokens`
- `torrust_tracker_configuration::Configuration`
- `torrust_tracker_configuration::Core`
- `torrust_tracker_configuration::HealthCheckApi`
- `torrust_tracker_configuration::validator::Validator`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`
- `torrust_tracker_swarm_coordination_registry::statistics::activity_metrics_updater`
- `torrust_tracker_swarm_coordination_registry::statistics::event`

#### `torrust-tracker-udp-server` [normal]

- `torrust_tracker_udp_server::banning::event`
- `torrust_tracker_udp_server::container::UdpTrackerServerContainer`
- `torrust_tracker_udp_server::server::Server`
- `torrust_tracker_udp_server::server::spawner`
- `torrust_tracker_udp_server::statistics::event`

#### `bittorrent-tracker-client` [dev]

- `bittorrent_tracker_client::http::client`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration::ephemeral_public`

---

### `torrust-tracker-client`

Workspace deps: **2** (unchanged count; `bittorrent-udp-tracker-protocol` → `bittorrent-tracker-protocol[udp]`)

#### `bittorrent-tracker-client` [normal]

- `bittorrent_tracker_client::http::client`
- `bittorrent_tracker_client::peer_id::default_production_peer_id`
- `bittorrent_tracker_client::udp`
- `bittorrent_tracker_client::udp::client`

#### `bittorrent-tracker-protocol` [normal — _(was: `bittorrent-udp-tracker-protocol`)_]

- `bittorrent_tracker_protocol::udp::PeerId`
- `bittorrent_tracker_protocol::udp::Response`
- `bittorrent_tracker_protocol::udp::TransactionId`
- `bittorrent_tracker_protocol::udp::common::InfoHash`

---

### `torrust-tracker-configuration`

Workspace deps: **2** — unchanged.

> Same as source report.

---

### `torrust-tracker-metrics`

Workspace deps: **1** — unchanged.

> Same as source report.

---

### `torrust-tracker-primitives`

Workspace deps: **3** — unchanged.

> Same as source report.

---

### `torrust-tracker-swarm-coordination-registry`

Workspace deps: **6** — unchanged.

> Same as source report.

---

### `torrust-tracker-test-helpers`

Workspace deps: **1** — unchanged.

> Same as source report.

---

### `torrust-tracker-torrent-repository-benchmarking`

Workspace deps: **3** — unchanged.

> Same as source report.

---

### `torrust-tracker-udp-server`

Workspace deps: **11** (down from 13; `bittorrent-udp-core` and
`bittorrent-udp-tracker-protocol` collapse into the merged crates)

#### `bittorrent-tracker-core` [normal — _(was: `bittorrent-udp-core` + `bittorrent-tracker-core`)_]

- `bittorrent_tracker_core::MAX_SCRAPE_TORRENTS`
- `bittorrent_tracker_core::announce_handler::AnnounceHandler`
- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::databases::setup`
- `bittorrent_tracker_core::error`
- `bittorrent_tracker_core::scrape_handler::ScrapeHandler`
- `bittorrent_tracker_core::statistics::persisted`
- `bittorrent_tracker_core::torrent::repository`
- `bittorrent_tracker_core::udp::UDP_TRACKER_LOG_TARGET`
- `bittorrent_tracker_core::udp::connection_cookie`
- `bittorrent_tracker_core::udp::connection_cookie::gen_remote_fingerprint`
- `bittorrent_tracker_core::udp::connection_cookie::make`
- `bittorrent_tracker_core::udp::container::UdpTrackerCoreContainer`
- `bittorrent_tracker_core::udp::event`
- `bittorrent_tracker_core::udp::event::Event`
- `bittorrent_tracker_core::udp::event::bus`
- `bittorrent_tracker_core::udp::event::sender`
- `bittorrent_tracker_core::udp::initialize_static`
- `bittorrent_tracker_core::udp::services::announce`
- `bittorrent_tracker_core::udp::services::banning`
- `bittorrent_tracker_core::udp::services::connect`
- `bittorrent_tracker_core::udp::services::scrape`
- `bittorrent_tracker_core::udp::statistics::event`
- `bittorrent_tracker_core::whitelist`
- `bittorrent_tracker_core::whitelist::authorization`
- `bittorrent_tracker_core::whitelist::repository`

#### `bittorrent-tracker-protocol` [normal — _(was: `bittorrent-udp-tracker-protocol`)_]

- `bittorrent_tracker_protocol::udp::AnnounceEvent`
- `bittorrent_tracker_protocol::udp::AnnounceInterval`
- `bittorrent_tracker_protocol::udp::AnnounceRequest`
- `bittorrent_tracker_protocol::udp::InfoHash`
- `bittorrent_tracker_protocol::udp::PeerClient`
- `bittorrent_tracker_protocol::udp::Response`
- `bittorrent_tracker_protocol::udp::TransactionId`
- `bittorrent_tracker_protocol::udp::common::ConnectionId`
- `bittorrent_tracker_protocol::udp::common::InfoHash`
- `bittorrent_tracker_protocol::udp::common::NumberOfBytes`
- `bittorrent_tracker_protocol::udp::common::NumberOfPeers`
- `bittorrent_tracker_protocol::udp::common::PeerId`
- `bittorrent_tracker_protocol::udp::common::Port`
- `bittorrent_tracker_protocol::udp::common::ResponsePeer`
- `bittorrent_tracker_protocol::udp::common::TransactionId`
- `bittorrent_tracker_protocol::udp::request::ConnectRequest`
- `bittorrent_tracker_protocol::udp::request::ScrapeRequest`
- `bittorrent_tracker_protocol::udp::response::AnnounceResponse`
- `bittorrent_tracker_protocol::udp::response::ConnectResponse`
- `bittorrent_tracker_protocol::udp::response::ScrapeResponse`
- `bittorrent_tracker_protocol::udp::response::TorrentScrapeStatistics`

#### `bittorrent-tracker-client` [normal]

- `bittorrent_tracker_client::udp::client`

#### `torrust-net-primitives` [normal]

- `torrust_net_primitives::service_binding`
- `torrust_net_primitives::service_binding::ServiceBinding`

#### `torrust-server-lib` [normal]

- `torrust_server_lib::logging::STARTED_ON`
- `torrust_server_lib::registar`
- `torrust_server_lib::registar::Registar`
- `torrust_server_lib::registar::ServiceHealthCheckJob`
- `torrust_server_lib::signals`

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::DurationSinceUnixEpoch`
- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::clock::Time`
- `torrust_tracker_clock::initialize_static`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::Core`

#### `torrust-tracker-events` [normal]

- `torrust_tracker_events::broadcaster::Broadcaster`
- `torrust_tracker_events::bus::EventBus`
- `torrust_tracker_events::bus::SenderStatus`
- `torrust_tracker_events::receiver::Receiver`
- `torrust_tracker_events::receiver::RecvError`
- `torrust_tracker_events::sender::SendError`
- `torrust_tracker_events::sender::Sender`

#### `torrust-tracker-metrics` [normal]

- `torrust_tracker_metrics::label`
- `torrust_tracker_metrics::label::LabelSet`
- `torrust_tracker_metrics::label_name`
- `torrust_tracker_metrics::metric::MetricName`
- `torrust_tracker_metrics::metric::description`
- `torrust_tracker_metrics::metric_collection`
- `torrust_tracker_metrics::metric_collection::Error`
- `torrust_tracker_metrics::metric_collection::aggregate`
- `torrust_tracker_metrics::metric_name`
- `torrust_tracker_metrics::unit::Unit`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceData`
- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::ScrapeData`
- `torrust_tracker_primitives::peer::fixture`
- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`
- `torrust_tracker_test_helpers::configuration::ephemeral_public`
- `torrust_tracker_test_helpers::logging::logs_contains_a_line_with`

---

## Summary of coupling changes

| Package                                | Deps before | Deps after | Delta |
| -------------------------------------- | ----------- | ---------- | ----- |
| `bittorrent-tracker-protocol`          | N/A (new)   | 5          | +5    |
| `bittorrent-tracker-core`              | 9           | 11         | +2    |
| `bittorrent-tracker-client`            | 4           | 4          | 0     |
| `torrust-tracker-axum-http-server`     | 14          | 12         | −2    |
| `torrust-tracker-axum-rest-api-server` | 16          | 15         | −1    |
| `torrust-tracker-rest-api-core`        | 10          | 9          | −1    |
| `torrust-tracker`                      | 16          | 14         | −2    |
| `torrust-tracker-client`               | 2           | 2          | 0     |
| `torrust-tracker-udp-server`           | 13          | 11         | −2    |
| _All other packages_                   | —           | —          | 0     |

**Workspace package count**: 29 → 25 (−4)

---

## Analysis: Pros and Cons

### Dimension 1 — Inter-package coupling

#### Effect on the dependency graph

The number of distinct workspace-dependency edges decreases at every consumer. In the
`torrust-tracker` root crate alone, two separate entries (`bittorrent-http-core` and
`bittorrent-udp-core`) collapse into a single `bittorrent-tracker-core` entry with
feature flags. The same compression happens in `torrust-tracker-axum-http-server`,
`torrust-tracker-rest-api-core`, and `torrust-tracker-udp-server`.

**Apparent pro — fewer edges**: The `Cargo.toml` dependency lists in consumers are shorter,
and the number of workspace packages shrinks by four.

**Real con — edges hidden, not removed**: The logical coupling does not decrease. What was
expressed as inter-crate edges (visible, checkable with `cargo tree`, enforceable with
`cargo deny`) becomes intra-crate feature coupling (invisible by default, no tooling
equivalent to deny or dependency lint). Cycles, accidental cross-feature leakage, and
improper feature-flag gating are much harder to detect.

**Hard con — circular dependency as a prerequisite cost**: As documented above, the protocol
merge requires relocating error types out of `bittorrent-tracker-core` before Cargo will
even compile. That is a substantial refactor in its own right; it is a hidden cost attached
to this proposal that is not present in the source report.

**Con — `bittorrent-tracker-core` grows into a large multi-concern crate**: After the core
merge it contains base peer-management logic, UDP-specific connection cookie handling and
banning, and HTTP-specific announce/scrape service adapters — three distinct concerns that
today have clean crate boundaries. Reviewers reading `bittorrent-tracker-core` must now
understand all three layers simultaneously, and `#[cfg(feature = ...)]` guards
interspersed throughout the source replace clear module boundaries at the crate level.

#### Verdict — coupling dimension

The proposal reduces the _count_ of workspace edges while increasing the _density_ and
_opacity_ of coupling inside the merged crates. The net effect on maintainability is
negative for coupling clarity.

---

### Dimension 2 — Working on protocol-specification-driven features

This scenario covers changes like a BEP update (e.g., a new field in the UDP
connect/announce exchange, or a new HTTP scrape extension).

#### Status quo (separate crates)

A BEP 15 (UDP) revision touches exactly `packages/udp-protocol` and possibly
`packages/udp-core`. A BEP 23 (HTTP compact peer lists) change touches
`packages/http-protocol` and `packages/http-core`. The two streams are completely
independent: different folders, different `Cargo.toml` files, different CI build units.
A developer can branch, implement, and review without touching any HTTP code, and the
compiler enforces the boundary.

#### After the merge

A BEP 15 change now lives in `packages/protocol` behind `#[cfg(feature = "udp")]`. The
developer must be careful not to accidentally break HTTP protocol parsing code sitting in
the same file or module. CI compiles and tests the crate in at least three configurations
(`--no-default-features`, `--features udp`, `--features http`, `--all-features`); if this
matrix is absent, a change to the `udp` feature can silently break the `http` feature.
Adding this CI matrix is extra maintenance work.

**Con — increased review surface**: A PR for a pure UDP BEP update shows diffs inside a
file that also contains HTTP protocol code. Reviewers must mentally filter out irrelevant
context.

**Con — feature-flag discipline required permanently**: Every future protocol contributor
must learn the feature-gating convention. An incorrect `use` statement without a `cfg`
guard would silently pull one protocol's types into the other's compilation path.

**Con — harder to extract later**: One of the stated goals of EPIC #1669 is eventual
extraction of `bittorrent-*` crates to their own repositories. A merged
`bittorrent-tracker-protocol` is harder to extract than two separate standalone crates;
extraction would require splitting it back apart or publishing a single crate with optional
features to crates.io — which complicates SemVer and changelog management.

**Marginal pro — shared test infrastructure**: If a test helper or fixture is common to
both protocol implementations (e.g., a mock peer ID generator), it can live once in the
crate rather than being duplicated. This benefit is small and can equally be achieved with
a shared test-helper module in `torrust-tracker-test-helpers`.

#### Verdict — protocol-specification dimension

For changes driven by protocol specification updates, the separate-crate structure provides
stronger isolation and clearer reviewability. The merged structure provides no meaningful
advantage for this scenario and introduces non-trivial discipline overhead.

---

### Dimension 3 — Cross-protocol same-layer changes

This scenario covers work that is logically required in both the UDP layer and the HTTP
layer at the same abstraction level — for example, a new statistics counter, a change to
whitelist checking, or a refactor of the scrape-handler signature.

#### The key observation: shared logic is already centralized

The **truly shared** announce/scrape/whitelist/statistics logic already lives in
`bittorrent-tracker-core` (`packages/tracker-core`). When a change is needed across
protocols at the shared layer, a developer modifies that one package and both
`udp-core` and `http-core` benefit automatically by virtue of their
dependency on it. This is the current design working as intended.

What lives in `udp-core` and `http-core` is, by definition,
**protocol-specific**: UDP connection-cookie handling, HTTP query-parameter parsing, UDP
event bus, HTTP event bus. These are not the same code. They require different changes for
different reasons.

#### What the merge actually changes for this scenario

After the core merge, a developer changing both the UDP and HTTP event-bus implementations
simultaneously would touch one crate instead of two. The diff appears in one PR, and
`cargo test` for the merged crate runs both test suites in one invocation.

**Marginal pro — one crate to update in `Cargo.toml`**: Downstream consumers (`rest-api-core`,
`torrust-tracker`) add one feature list instead of two separate `[dependencies]` entries.

**Con — false sense of unity**: The code behind `#[cfg(feature = "udp")]` and
`#[cfg(feature = "http")]` is still two separate implementations. They happen to share a
crate boundary, not logic. Treating them as "one thing" obscures their independence.

**Con — larger change scope per PR**: A PR that only needs to fix the UDP banning service
now lives in a crate that also contains HTTP core logic. The reviewer must confirm the HTTP
code was not touched (or understand why it was). With separate crates, scope is enforced
structurally.

**Con — test isolation degraded**: The current `bittorrent-udp-core` tests only
ever exercise UDP paths; `bittorrent-http-core` tests only HTTP paths. After the
merge, a misconfigured test that enables both features could inadvertently test cross-feature
interactions that the developer did not intend and that do not represent a real deployment.

**Con — incremental compilation cost**: Touching any file in `bittorrent-tracker-core`
(base, UDP, or HTTP feature) invalidates the compiled artifact for the entire crate. With
separate crates, a UDP-only change does not force recompilation of the HTTP core, and vice
versa.

#### Verdict — cross-protocol same-layer dimension

For changes that genuinely span both protocols at the same layer, the case for the merged
crate is weakest: the shared part already has a dedicated home (`bittorrent-tracker-core`
base), and the protocol-specific parts are not actually the same code. The merge provides
cosmetic co-location but at a real cost to compilation speed, test isolation, and review
clarity.

---

## Overall assessment

| Criterion                           | Separate crates (status quo) |     Merged with features (proposal)     |
| ----------------------------------- | :--------------------------: | :-------------------------------------: |
| Workspace size                      |      More packages (29)      |           Fewer packages (25)           |
| Coupling visibility                 |  Explicit, tooling-enforced  |       Hidden behind feature flags       |
| Circular dependency blocker         |             None             |  Requires prior error-type relocation   |
| Protocol-spec changes (isolation)   |            Strong            |                Weakened                 |
| Protocol-spec changes (review)      |        Clean, focused        |     Noisy, requires cfg discipline      |
| Cross-protocol shared-layer changes | Already centralized in base  |      No improvement; cosmetic only      |
| Extraction to standalone repos      |  Straightforward per-crate   | Requires split or feature-aware publish |
| Incremental build                   |  Per-protocol invalidation   |        Whole-crate invalidation         |
| Test isolation                      |   Per-protocol test suite    |        Feature-combination risk         |

The proposal reduces the visible package count and shortens some `Cargo.toml` files,
but it does not improve — and in several dimensions actively degrades — the separation of
concerns that the current structure provides. The circular dependency that must be resolved
as a prerequisite is a concrete, non-trivial cost not present in the current design.

The one scenario where the merged structure offers a real (not cosmetic) benefit is if the
codebase reaches a point where UDP and HTTP protocol implementations share so much internal
logic that a single module tree is genuinely more natural than two separate crates. The
current coupling report shows no evidence of that: the two protocol packages and the two
core packages have almost entirely disjoint import lists, sharing only their common
downstream dependencies (`torrust-tracker-primitives`, `torrust-tracker-clock`, etc.).
