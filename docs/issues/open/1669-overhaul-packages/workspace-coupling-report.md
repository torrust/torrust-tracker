# Workspace Coupling Report

Generated: 2026-05-19 11:17 UTC

Workspace packages: 28

---

## How to read this report

Each section covers one workspace package that has at least one workspace-level
dependency. For every dependency the items actually imported from it are listed:

- **Normal dep** — required for compilation of the library/binary.
- **Dev dep** — required only in tests and benchmarks.
- **Build dep** — required only in `build.rs`.

Items are extracted by scanning the package's `src/` directory for
`use MODULE::` statements and `MODULE::` fully-qualified path references.
The scan is text-based; it may miss items imported through re-exports or macros,
but it is accurate enough to identify thin-dependency patterns.

**Signal**: a dependency with only 1–3 distinct import paths may be a candidate
for elimination (move the item, break the edge).

---

## Packages with no workspace dependencies

These packages are leaves (no workspace dep) and are prime extraction candidates.

- `bittorrent-peer-id`
- `torrust-rest-tracker-api-client`
- `torrust-tracker-clock`
- `torrust-tracker-contrib-bencode`
- `torrust-tracker-events`
- `torrust-tracker-located-error`
- `workspace-coupling`

---

## Package coupling details

### `bittorrent-http-tracker-core`

Workspace deps: 9

#### `bittorrent-http-tracker-protocol` [normal]

- `bittorrent_http_tracker_protocol::v1::requests`
- `bittorrent_http_tracker_protocol::v1::services`

#### `bittorrent-tracker-core` [normal]

- `bittorrent_tracker_core::announce_handler`
- `bittorrent_tracker_core::announce_handler::AnnounceHandler`
- `bittorrent_tracker_core::announce_handler::PeersWanted`
- `bittorrent_tracker_core::authentication`
- `bittorrent_tracker_core::authentication::key`
- `bittorrent_tracker_core::authentication::service`
- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::databases::setup`
- `bittorrent_tracker_core::error`
- `bittorrent_tracker_core::scrape_handler::ScrapeHandler`
- `bittorrent_tracker_core::statistics::persisted`
- `bittorrent_tracker_core::torrent::repository`
- `bittorrent_tracker_core::whitelist`
- `bittorrent_tracker_core::whitelist::authorization`
- `bittorrent_tracker_core::whitelist::repository`

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::DurationSinceUnixEpoch`
- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::clock::Time`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::Configuration`
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
- `torrust_tracker_primitives::ScrapeData`
- `torrust_tracker_primitives::peer::Peer`
- `torrust_tracker_primitives::peer::PeerAnnouncement`
- `torrust_tracker_primitives::service_binding`
- `torrust_tracker_primitives::service_binding::Protocol`
- `torrust_tracker_primitives::service_binding::ServiceBinding`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`

### `bittorrent-http-tracker-protocol`

Workspace deps: 7

#### `bittorrent-tracker-core` [normal]

- `bittorrent_tracker_core::authentication::Error`
- `bittorrent_tracker_core::error::AnnounceError`
- `bittorrent_tracker_core::error::ScrapeError`
- `bittorrent_tracker_core::error::WhitelistError`

#### `bittorrent-udp-tracker-protocol` [normal]

- `bittorrent_udp_tracker_protocol::AnnounceEvent`
- `bittorrent_udp_tracker_protocol::AnnounceEvent::Completed`
- `bittorrent_udp_tracker_protocol::AnnounceEvent::None`
- `bittorrent_udp_tracker_protocol::AnnounceEvent::Started`
- `bittorrent_udp_tracker_protocol::AnnounceEvent::Stopped`

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::clock::Time`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::AnnouncePolicy`

#### `torrust-tracker-contrib-bencode` [normal]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob import)._

#### `torrust-tracker-located-error` [normal]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob import)._

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::ScrapeData`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::fixture`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

### `bittorrent-tracker-client`

Workspace deps: 3

#### `bittorrent-udp-tracker-protocol` [normal]

- `bittorrent_udp_tracker_protocol::PeerId`
- `bittorrent_udp_tracker_protocol::Request`

#### `torrust-tracker-located-error` [normal]

- `torrust_tracker_located_error::DynError`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::service_binding::ServiceBinding`

### `bittorrent-tracker-core`

Workspace deps: 9

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::DurationSinceUnixEpoch`
- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::clock::Time`
- `torrust_tracker_clock::clock::stopped`
- `torrust_tracker_clock::conv::convert_from_timestamp_to_datetime_utc`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::AnnouncePolicy`
- `torrust_tracker_configuration::Configuration`
- `torrust_tracker_configuration::Core`
- `torrust_tracker_configuration::Driver::MySQL`
- `torrust_tracker_configuration::Driver::PostgreSQL`
- `torrust_tracker_configuration::Driver::Sqlite3`
- `torrust_tracker_configuration::TORRENT_PEERS_LIMIT`
- `torrust_tracker_configuration::v2_0_0::core`

#### `torrust-tracker-events` [normal]

- `torrust_tracker_events::receiver::RecvError`

#### `torrust-tracker-located-error` [normal]

- `torrust_tracker_located_error::Located`
- `torrust_tracker_located_error::LocatedError`

#### `torrust-tracker-metrics` [normal]

- `torrust_tracker_metrics::label::LabelSet`
- `torrust_tracker_metrics::metric::MetricName`
- `torrust_tracker_metrics::metric::description`
- `torrust_tracker_metrics::metric_collection`
- `torrust_tracker_metrics::metric_collection::Error`
- `torrust_tracker_metrics::metric_name`
- `torrust_tracker_metrics::unit::Unit`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceEvent`
- `torrust_tracker_primitives::NumberOfBytes`
- `torrust_tracker_primitives::NumberOfDownloads`
- `torrust_tracker_primitives::NumberOfDownloadsBTreeMap`
- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::ScrapeData`
- `torrust_tracker_primitives::pagination::Pagination`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::Peer`
- `torrust_tracker_primitives::swarm_metadata`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::Registry`
- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`
- `torrust_tracker_swarm_coordination_registry::event::Event`
- `torrust_tracker_swarm_coordination_registry::event::receiver`

#### `torrust-rest-tracker-api-client` [dev]

_No `torrust_rest_tracker_api_client::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`
- `torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database`

### `bittorrent-udp-tracker-core`

Workspace deps: 9

#### `bittorrent-tracker-core` [normal]

- `bittorrent_tracker_core::announce_handler`
- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::error`
- `bittorrent_tracker_core::scrape_handler::ScrapeHandler`
- `bittorrent_tracker_core::torrent::repository`
- `bittorrent_tracker_core::whitelist`

#### `bittorrent-udp-tracker-protocol` [normal]

- `bittorrent_udp_tracker_protocol::AnnounceEvent::Completed`
- `bittorrent_udp_tracker_protocol::AnnounceEvent::None`
- `bittorrent_udp_tracker_protocol::AnnounceEvent::Started`
- `bittorrent_udp_tracker_protocol::AnnounceEvent::Stopped`
- `bittorrent_udp_tracker_protocol::AnnounceEvent::from`
- `bittorrent_udp_tracker_protocol::AnnounceRequest`
- `bittorrent_udp_tracker_protocol::ConnectionId`
- `bittorrent_udp_tracker_protocol::ScrapeRequest`
- `bittorrent_udp_tracker_protocol::common::InfoHash`

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::DurationSinceUnixEpoch`
- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::clock::Time`

#### `torrust-tracker-configuration` [normal]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob import)._

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
- `torrust_tracker_primitives::AnnounceEvent::Completed`
- `torrust_tracker_primitives::AnnounceEvent::None`
- `torrust_tracker_primitives::AnnounceEvent::Started`
- `torrust_tracker_primitives::AnnounceEvent::Stopped`
- `torrust_tracker_primitives::NumberOfBytes::new`
- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::ScrapeData`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::PeerAnnouncement`
- `torrust_tracker_primitives::service_binding`
- `torrust_tracker_primitives::service_binding::ServiceBinding`
- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`

#### `torrust-tracker-test-helpers` [dev]

_No `torrust_tracker_test_helpers::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

### `bittorrent-udp-tracker-protocol`

Workspace deps: 1

#### `bittorrent-peer-id` [normal]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob import)._

### `torrust-axum-health-check-api-server`

Workspace deps: 10

#### `torrust-axum-server` [normal]

- `torrust_axum_server::signals::graceful_shutdown`

#### `torrust-server-lib` [normal]

- `torrust_server_lib::logging::Latency`
- `torrust_server_lib::registar`
- `torrust_server_lib::registar::Registar`
- `torrust_server_lib::registar::ServiceRegistry`
- `torrust_server_lib::signals`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::HealthCheckApi`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::service_binding`

#### `torrust-axum-health-check-api-server` [dev]

_No `torrust_axum_health_check_api_server::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-axum-http-tracker-server` [dev]

_No `torrust_axum_http_tracker_server::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-axum-rest-tracker-api-server` [dev]

_No `torrust_axum_rest_tracker_api_server::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-tracker-clock` [dev]

_No `torrust_tracker_clock::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-tracker-test-helpers` [dev]

_No `torrust_tracker_test_helpers::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-udp-tracker-server` [dev]

_No `torrust_udp_tracker_server::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

### `torrust-axum-http-tracker-server`

Workspace deps: 13

#### `bittorrent-http-tracker-core` [normal]

- `bittorrent_http_tracker_core::container::HttpTrackerCoreContainer`
- `bittorrent_http_tracker_core::event::bus`
- `bittorrent_http_tracker_core::event::sender`
- `bittorrent_http_tracker_core::services::announce`
- `bittorrent_http_tracker_core::services::scrape`
- `bittorrent_http_tracker_core::statistics::event`
- `bittorrent_http_tracker_core::statistics::repository`

#### `bittorrent-http-tracker-protocol` [normal]

- `bittorrent_http_tracker_protocol::v1`
- `bittorrent_http_tracker_protocol::v1::query`
- `bittorrent_http_tracker_protocol::v1::requests`
- `bittorrent_http_tracker_protocol::v1::responses`
- `bittorrent_http_tracker_protocol::v1::services`

#### `bittorrent-tracker-core` [normal]

- `bittorrent_tracker_core::announce_handler::AnnounceHandler`
- `bittorrent_tracker_core::authentication`
- `bittorrent_tracker_core::authentication::Key`
- `bittorrent_tracker_core::authentication::key`
- `bittorrent_tracker_core::authentication::service`
- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::databases::setup`
- `bittorrent_tracker_core::scrape_handler::ScrapeHandler`
- `bittorrent_tracker_core::statistics::persisted`
- `bittorrent_tracker_core::torrent::repository`
- `bittorrent_tracker_core::whitelist::authorization`
- `bittorrent_tracker_core::whitelist::repository`

#### `bittorrent-udp-tracker-protocol` [normal]

_No `bittorrent_udp_tracker_protocol::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-axum-server` [normal]

- `torrust_axum_server::custom_axum_server`
- `torrust_axum_server::signals::graceful_shutdown`
- `torrust_axum_server::tsl::make_rust_tls`

#### `torrust-server-lib` [normal]

- `torrust_server_lib::logging::Latency`
- `torrust_server_lib::logging::STARTED_ON`
- `torrust_server_lib::registar`
- `torrust_server_lib::registar::Registar`
- `torrust_server_lib::signals`

#### `torrust-tracker-clock` [normal]

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
- `torrust_tracker_primitives::service_binding`
- `torrust_tracker_primitives::service_binding::ServiceBinding`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`

#### `torrust-tracker-clock` [dev]

- `torrust_tracker_clock::initialize_static`

#### `torrust-tracker-events` [dev]

_No `torrust_tracker_events::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`
- `torrust_tracker_test_helpers::configuration::ephemeral_public`

### `torrust-axum-rest-tracker-api-server`

Workspace deps: 15

#### `bittorrent-http-tracker-core` [normal]

- `bittorrent_http_tracker_core::container::HttpTrackerCoreContainer`
- `bittorrent_http_tracker_core::statistics::repository`

#### `bittorrent-tracker-core` [normal]

- `bittorrent_tracker_core::authentication`
- `bittorrent_tracker_core::authentication::Key`
- `bittorrent_tracker_core::authentication::handler`
- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::error::PeerKeyError`
- `bittorrent_tracker_core::statistics::repository`
- `bittorrent_tracker_core::torrent::repository`
- `bittorrent_tracker_core::torrent::services`
- `bittorrent_tracker_core::whitelist::manager`

#### `bittorrent-udp-tracker-core` [normal]

- `bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer`
- `bittorrent_udp_tracker_core::initialize_static`
- `bittorrent_udp_tracker_core::services::banning`
- `bittorrent_udp_tracker_core::statistics::repository`

#### `torrust-axum-server` [normal]

- `torrust_axum_server::custom_axum_server`
- `torrust_axum_server::signals::graceful_shutdown`
- `torrust_axum_server::tsl::make_rust_tls`

#### `torrust-rest-tracker-api-client` [normal]

- `torrust_rest_tracker_api_client::connection_info`

#### `torrust-rest-tracker-api-core` [normal]

- `torrust_rest_tracker_api_core::container::TrackerHttpApiCoreContainer`
- `torrust_rest_tracker_api_core::statistics::metrics`
- `torrust_rest_tracker_api_core::statistics::services`

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
- `torrust_tracker_primitives::service_binding`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`
- `torrust_tracker_swarm_coordination_registry::statistics::repository`

#### `torrust-udp-tracker-server` [normal]

- `torrust_udp_tracker_server::container::UdpTrackerServerContainer`
- `torrust_udp_tracker_server::statistics::repository`

#### `torrust-rest-tracker-api-client` [dev]

- `torrust_rest_tracker_api_client::connection_info`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration::ephemeral_public`

### `torrust-axum-server`

Workspace deps: 3

#### `torrust-server-lib` [normal]

- `torrust_server_lib::signals`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::TslConfig`

#### `torrust-tracker-located-error` [normal]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob import)._

### `torrust-rest-tracker-api-core`

Workspace deps: 10

#### `bittorrent-http-tracker-core` [normal]

- `bittorrent_http_tracker_core::container::HttpTrackerCoreContainer`
- `bittorrent_http_tracker_core::event::bus`
- `bittorrent_http_tracker_core::event::sender`
- `bittorrent_http_tracker_core::statistics::event`
- `bittorrent_http_tracker_core::statistics::repository`

#### `bittorrent-tracker-core` [normal]

- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::statistics::repository`
- `bittorrent_tracker_core::torrent::repository`

#### `bittorrent-udp-tracker-core` [normal]

- `bittorrent_udp_tracker_core::MAX_CONNECTION_ID_ERRORS_PER_IP`
- `bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer`
- `bittorrent_udp_tracker_core::services::banning`
- `bittorrent_udp_tracker_core::statistics::repository`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::Configuration`

#### `torrust-tracker-metrics` [normal]

- `torrust_tracker_metrics::metric_collection::MetricCollection`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`
- `torrust_tracker_swarm_coordination_registry::statistics::repository`

#### `torrust-udp-tracker-server` [normal]

- `torrust_udp_tracker_server::container::UdpTrackerServerContainer`
- `torrust_udp_tracker_server::statistics`
- `torrust_udp_tracker_server::statistics::repository`

#### `torrust-tracker-events` [dev]

- `torrust_tracker_events::bus::SenderStatus`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`

### `torrust-server-lib`

Workspace deps: 1

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::service_binding::ServiceBinding`

### `torrust-tracker`

Workspace deps: 16

#### `bittorrent-http-tracker-core` [normal]

- `bittorrent_http_tracker_core::container`
- `bittorrent_http_tracker_core::container::HttpTrackerCoreContainer`
- `bittorrent_http_tracker_core::statistics::event`

#### `bittorrent-tracker-core` [normal]

- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::statistics::event`
- `bittorrent_tracker_core::statistics::persisted`
- `bittorrent_tracker_core::torrent::manager`

#### `bittorrent-udp-tracker-core` [normal]

- `bittorrent_udp_tracker_core::UDP_TRACKER_LOG_TARGET`
- `bittorrent_udp_tracker_core::container`
- `bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer`
- `bittorrent_udp_tracker_core::crypto::keys`
- `bittorrent_udp_tracker_core::initialize_static`
- `bittorrent_udp_tracker_core::statistics::event`

#### `torrust-axum-health-check-api-server` [normal]

- `torrust_axum_health_check_api_server::HEALTH_CHECK_API_LOG_TARGET`

#### `torrust-axum-http-tracker-server` [normal]

- `torrust_axum_http_tracker_server::HTTP_TRACKER_LOG_TARGET`
- `torrust_axum_http_tracker_server::Version`
- `torrust_axum_http_tracker_server::Version::V1`
- `torrust_axum_http_tracker_server::server`

#### `torrust-axum-rest-tracker-api-server` [normal]

- `torrust_axum_rest_tracker_api_server::Version`
- `torrust_axum_rest_tracker_api_server::Version::V1`
- `torrust_axum_rest_tracker_api_server::server`
- `torrust_axum_rest_tracker_api_server::v1::context`

#### `torrust-axum-server` [normal]

- `torrust_axum_server::tsl::make_rust_tls`

#### `torrust-rest-tracker-api-client` [normal]

- `torrust_rest_tracker_api_client::connection_info`
- `torrust_rest_tracker_api_client::v1::Client`
- `torrust_rest_tracker_api_client::v1::client`

#### `torrust-rest-tracker-api-core` [normal]

- `torrust_rest_tracker_api_core::container::TrackerHttpApiCoreContainer`

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

#### `torrust-udp-tracker-server` [normal]

- `torrust_udp_tracker_server::banning::event`
- `torrust_udp_tracker_server::container::UdpTrackerServerContainer`
- `torrust_udp_tracker_server::server::Server`
- `torrust_udp_tracker_server::server::spawner`
- `torrust_udp_tracker_server::statistics::event`

#### `bittorrent-tracker-client` [dev]

_No `bittorrent_tracker_client::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration::ephemeral_public`

### `torrust-tracker-client`

Workspace deps: 2

#### `bittorrent-tracker-client` [normal]

- `bittorrent_tracker_client::http::client`
- `bittorrent_tracker_client::peer_id::default_production_peer_id`
- `bittorrent_tracker_client::udp`
- `bittorrent_tracker_client::udp::client`

#### `bittorrent-udp-tracker-protocol` [normal]

- `bittorrent_udp_tracker_protocol::PeerId`
- `bittorrent_udp_tracker_protocol::Response`
- `bittorrent_udp_tracker_protocol::TransactionId`
- `bittorrent_udp_tracker_protocol::common::InfoHash`

### `torrust-tracker-configuration`

Workspace deps: 1

#### `torrust-tracker-located-error` [normal]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob import)._

### `torrust-tracker-metrics`

Workspace deps: 1

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::DurationSinceUnixEpoch`

### `torrust-tracker-primitives`

Workspace deps: 3

#### `bittorrent-peer-id` [normal]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob import)._

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::DurationSinceUnixEpoch`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::AnnouncePolicy`

### `torrust-tracker-swarm-coordination-registry`

Workspace deps: 6

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::DurationSinceUnixEpoch`
- `torrust_tracker_clock::clock`
- `torrust_tracker_clock::clock::Time`
- `torrust_tracker_clock::clock::stopped`
- `torrust_tracker_clock::conv::convert_from_timestamp_to_datetime_utc`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::TORRENT_PEERS_LIMIT`
- `torrust_tracker_configuration::TrackerPolicy`

#### `torrust-tracker-events` [normal]

- `torrust_tracker_events::broadcaster::Broadcaster`
- `torrust_tracker_events::bus::EventBus`
- `torrust_tracker_events::bus::SenderStatus`
- `torrust_tracker_events::receiver::Receiver`
- `torrust_tracker_events::receiver::RecvError`
- `torrust_tracker_events::sender`
- `torrust_tracker_events::sender::Sender`

#### `torrust-tracker-metrics` [normal]

- `torrust_tracker_metrics::label`
- `torrust_tracker_metrics::label::LabelSet`
- `torrust_tracker_metrics::label::LabelValue`
- `torrust_tracker_metrics::metric::MetricName`
- `torrust_tracker_metrics::metric::description`
- `torrust_tracker_metrics::metric_collection`
- `torrust_tracker_metrics::metric_collection::Error`
- `torrust_tracker_metrics::metric_name`
- `torrust_tracker_metrics::unit::Unit`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceEvent`
- `torrust_tracker_primitives::AnnounceEvent::Completed`
- `torrust_tracker_primitives::AnnounceEvent::Started`
- `torrust_tracker_primitives::NumberOfBytes`
- `torrust_tracker_primitives::NumberOfDownloadsBTreeMap`
- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::pagination::Pagination`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::Peer`
- `torrust_tracker_primitives::peer::PeerRole`
- `torrust_tracker_primitives::peer::fixture`
- `torrust_tracker_primitives::swarm_metadata`
- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

#### `torrust-tracker-test-helpers` [dev]

_No `torrust_tracker_test_helpers::` references found in `src/` — may be used only in `Cargo.toml` feature flags or `build.rs`._

### `torrust-tracker-test-helpers`

Workspace deps: 1

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::logging::TraceStyle`

### `torrust-tracker-torrent-repository-benchmarking`

Workspace deps: 3

#### `torrust-tracker-clock` [normal]

- `torrust_tracker_clock::DurationSinceUnixEpoch`
- `torrust_tracker_clock::clock`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::TrackerPolicy`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceEvent`
- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::pagination::Pagination`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::fixture`
- `torrust_tracker_primitives::swarm_metadata`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

### `torrust-udp-tracker-server`

Workspace deps: 12

#### `bittorrent-tracker-client` [normal]

- `bittorrent_tracker_client::udp::client`

#### `bittorrent-tracker-core` [normal]

- `bittorrent_tracker_core::MAX_SCRAPE_TORRENTS`
- `bittorrent_tracker_core::announce_handler::AnnounceHandler`
- `bittorrent_tracker_core::container::TrackerCoreContainer`
- `bittorrent_tracker_core::databases::setup`
- `bittorrent_tracker_core::error`
- `bittorrent_tracker_core::scrape_handler::ScrapeHandler`
- `bittorrent_tracker_core::statistics::persisted`
- `bittorrent_tracker_core::torrent::repository`
- `bittorrent_tracker_core::whitelist`
- `bittorrent_tracker_core::whitelist::authorization`
- `bittorrent_tracker_core::whitelist::repository`

#### `bittorrent-udp-tracker-core` [normal]

- `bittorrent_udp_tracker_core::UDP_TRACKER_LOG_TARGET`
- `bittorrent_udp_tracker_core::connection_cookie`
- `bittorrent_udp_tracker_core::connection_cookie::gen_remote_fingerprint`
- `bittorrent_udp_tracker_core::connection_cookie::make`
- `bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer`
- `bittorrent_udp_tracker_core::event`
- `bittorrent_udp_tracker_core::event::Event`
- `bittorrent_udp_tracker_core::event::bus`
- `bittorrent_udp_tracker_core::event::sender`
- `bittorrent_udp_tracker_core::initialize_static`
- `bittorrent_udp_tracker_core::services::announce`
- `bittorrent_udp_tracker_core::services::banning`
- `bittorrent_udp_tracker_core::services::connect`
- `bittorrent_udp_tracker_core::services::scrape`
- `bittorrent_udp_tracker_core::statistics::event`

#### `bittorrent-udp-tracker-protocol` [normal]

- `bittorrent_udp_tracker_protocol::AnnounceEvent`
- `bittorrent_udp_tracker_protocol::AnnounceInterval`
- `bittorrent_udp_tracker_protocol::AnnounceRequest`
- `bittorrent_udp_tracker_protocol::InfoHash`
- `bittorrent_udp_tracker_protocol::PeerClient`
- `bittorrent_udp_tracker_protocol::Response`
- `bittorrent_udp_tracker_protocol::common::ConnectionId`
- `bittorrent_udp_tracker_protocol::common::InfoHash`
- `bittorrent_udp_tracker_protocol::common::NumberOfBytes`
- `bittorrent_udp_tracker_protocol::common::NumberOfPeers`
- `bittorrent_udp_tracker_protocol::common::PeerId`
- `bittorrent_udp_tracker_protocol::common::Port`
- `bittorrent_udp_tracker_protocol::common::ResponsePeer`
- `bittorrent_udp_tracker_protocol::common::TransactionId`
- `bittorrent_udp_tracker_protocol::request::ConnectRequest`
- `bittorrent_udp_tracker_protocol::request::ScrapeRequest`
- `bittorrent_udp_tracker_protocol::response::AnnounceResponse`
- `bittorrent_udp_tracker_protocol::response::ConnectResponse`
- `bittorrent_udp_tracker_protocol::response::ScrapeResponse`
- `bittorrent_udp_tracker_protocol::response::TorrentScrapeStatistics`

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
- `torrust_tracker_primitives::service_binding`
- `torrust_tracker_primitives::service_binding::ServiceBinding`
- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`
- `torrust_tracker_test_helpers::configuration::ephemeral_public`

---

## Observations

(To be filled in after reviewing the report above.)

### Known thin dependencies (pre-existing)

- `torrust-tracker-clock` → `torrust-tracker-primitives`: only
  `DurationSinceUnixEpoch` imported. Addressed by SI-02.

### New findings

(Record any new thin-dependency or cluster-dependency findings here, with a
reference to the subissue opened for each.)
