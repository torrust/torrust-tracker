---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - packages/
---

# Workspace Coupling Report

Generated: 2026-06-10 12:23 UTC

Workspace packages: 25

---

## How to read this report

Each section covers one workspace package that has at least one workspace-level
dependency. For every dependency the items actually imported from it are listed:

- **Normal dep** — required for compilation of the library/binary.
- **Dev dep** — required only in tests and benchmarks.
- **Build dep** — required only in `build.rs`.

Items are extracted by scanning the package's `src/`, `tests/`, and `benches/`
directories for `use MODULE::` statements and `MODULE::` fully-qualified path references.
The scan is text-based; it may miss items imported through re-exports or macros,
but it is accurate enough to identify thin-dependency patterns.

**Signal**: a dependency with only 1–3 distinct import paths may be a candidate
for elimination (move the item, break the edge).

---

## Packages with no workspace dependencies

These packages are leaves (no workspace dep) and are prime extraction candidates.

- `torrust-server-lib`
- `torrust-tracker-events`
- `torrust-tracker-http-protocol`
- `torrust-tracker-primitives`
- `torrust-tracker-rest-api-client`
- `torrust-tracker-udp-protocol`
- `workspace-coupling`

---

## Package coupling details

### `torrust-tracker`

Workspace deps: 15

#### `torrust-server-lib` [normal]

- `torrust_server_lib::logging::STARTED_ON`
- `torrust_server_lib::registar::Registar`
- `torrust_server_lib::registar::ServiceRegistrationForm`
- `torrust_server_lib::registar::ServiceRegistry`
- `torrust_server_lib::signals`

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

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::AccessTokens`
- `torrust_tracker_configuration::Configuration`
- `torrust_tracker_configuration::Core`
- `torrust_tracker_configuration::HealthCheckApi`
- `torrust_tracker_configuration::validator::Validator`

#### `torrust-tracker-core` [normal]

- `torrust_tracker_core::container::TrackerCoreContainer`
- `torrust_tracker_core::statistics::event`
- `torrust_tracker_core::statistics::persisted`
- `torrust_tracker_core::torrent::manager`

#### `torrust-tracker-http-core` [normal]

- `torrust_tracker_http_tracker_core::container`
- `torrust_tracker_http_tracker_core::container::HttpTrackerCoreContainer`
- `torrust_tracker_http_tracker_core::statistics::event`

#### `torrust-tracker-rest-api-client` [normal]

- `torrust_tracker_rest_api_client::connection_info`
- `torrust_tracker_rest_api_client::v1::Client`
- `torrust_tracker_rest_api_client::v1::client`

#### `torrust-tracker-rest-api-core` [normal]

- `torrust_tracker_rest_api_core::container::TrackerHttpApiCoreContainer`

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

#### `torrust-tracker-udp-core` [normal]

- `torrust_tracker_udp_tracker_core::UDP_TRACKER_LOG_TARGET`
- `torrust_tracker_udp_tracker_core::container`
- `torrust_tracker_udp_tracker_core::container::UdpTrackerCoreContainer`
- `torrust_tracker_udp_tracker_core::crypto::keys`
- `torrust_tracker_udp_tracker_core::initialize_static`
- `torrust_tracker_udp_tracker_core::statistics::event`

#### `torrust-tracker-client-lib` [dev]

_No `torrust_tracker_client_lib::` references found in source — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration::ephemeral_public`

### `torrust-tracker-axum-health-check-api-server`

Workspace deps: 8

#### `torrust-server-lib` [normal]

- `torrust_server_lib::logging::Latency`
- `torrust_server_lib::registar`
- `torrust_server_lib::registar::Registar`
- `torrust_server_lib::registar::ServiceRegistry`
- `torrust_server_lib::signals`

#### `torrust-tracker-axum-server` [normal]

- `torrust_tracker_axum_server::signals::graceful_shutdown`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::HealthCheckApi`

#### `torrust-tracker-axum-health-check-api-server` [dev]

- `torrust_tracker_axum_health_check_api_server::environment::Started`
- `torrust_tracker_axum_health_check_api_server::resources`

#### `torrust-tracker-axum-http-server` [dev]

- `torrust_tracker_axum_http_server::environment::Started`

#### `torrust-tracker-axum-rest-api-server` [dev]

- `torrust_tracker_axum_rest_api_server::environment::Started`

#### `torrust-tracker-test-helpers` [dev]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob import)._

#### `torrust-tracker-udp-server` [dev]

- `torrust_tracker_udp_server::environment::Started`

### `torrust-tracker-axum-http-server`

Workspace deps: 10

#### `torrust-server-lib` [normal]

- `torrust_server_lib::logging::Latency`
- `torrust_server_lib::logging::STARTED_ON`
- `torrust_server_lib::registar`
- `torrust_server_lib::registar::Registar`
- `torrust_server_lib::signals`

#### `torrust-tracker-axum-server` [normal]

- `torrust_tracker_axum_server::custom_axum_server`
- `torrust_tracker_axum_server::signals::graceful_shutdown`
- `torrust_tracker_axum_server::tsl::make_rust_tls`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::Configuration`
- `torrust_tracker_configuration::Configuration::core`

#### `torrust-tracker-core` [normal]

- `torrust_tracker_core::announce_handler::AnnounceHandler`
- `torrust_tracker_core::authentication`
- `torrust_tracker_core::authentication::Key`
- `torrust_tracker_core::authentication::key`
- `torrust_tracker_core::authentication::service`
- `torrust_tracker_core::container::TrackerCoreContainer`
- `torrust_tracker_core::databases::setup`
- `torrust_tracker_core::scrape_handler::ScrapeHandler`
- `torrust_tracker_core::statistics::persisted`
- `torrust_tracker_core::torrent::repository`
- `torrust_tracker_core::whitelist::authorization`
- `torrust_tracker_core::whitelist::repository`

#### `torrust-tracker-http-core` [normal]

- `torrust_tracker_http_tracker_core::container::HttpTrackerCoreContainer`
- `torrust_tracker_http_tracker_core::event::bus`
- `torrust_tracker_http_tracker_core::event::sender`
- `torrust_tracker_http_tracker_core::services::announce`
- `torrust_tracker_http_tracker_core::services::scrape`
- `torrust_tracker_http_tracker_core::statistics::event`
- `torrust_tracker_http_tracker_core::statistics::repository`

#### `torrust-tracker-http-protocol` [normal]

- `torrust_tracker_http_tracker_protocol::v1`
- `torrust_tracker_http_tracker_protocol::v1::query`
- `torrust_tracker_http_tracker_protocol::v1::requests`
- `torrust_tracker_http_tracker_protocol::v1::responses`
- `torrust_tracker_http_tracker_protocol::v1::services`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceData`
- `torrust_tracker_primitives::AnnouncePolicy::max_peers_per_announce`
- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::ScrapeData`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::fixture`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`

#### `torrust-tracker-udp-protocol` [normal]

- `torrust_tracker_udp_tracker_protocol::PeerId`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`
- `torrust_tracker_test_helpers::configuration::ephemeral_public`
- `torrust_tracker_test_helpers::logging::logs_contains_a_line_with`

### `torrust-tracker-axum-rest-api-server`

Workspace deps: 13

#### `torrust-server-lib` [normal]

- `torrust_server_lib::logging::Latency`
- `torrust_server_lib::logging::STARTED_ON`
- `torrust_server_lib::registar`
- `torrust_server_lib::registar::Registar`
- `torrust_server_lib::signals`

#### `torrust-tracker-axum-server` [normal]

- `torrust_tracker_axum_server::custom_axum_server`
- `torrust_tracker_axum_server::signals::graceful_shutdown`
- `torrust_tracker_axum_server::tsl::make_rust_tls`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::AccessTokens`
- `torrust_tracker_configuration::HttpApi`
- `torrust_tracker_configuration::HttpApi::tsl_config`

#### `torrust-tracker-core` [normal]

- `torrust_tracker_core::authentication`
- `torrust_tracker_core::authentication::Key`
- `torrust_tracker_core::authentication::handler`
- `torrust_tracker_core::container::TrackerCoreContainer`
- `torrust_tracker_core::databases::SchemaMigrator`
- `torrust_tracker_core::error::PeerKeyError`
- `torrust_tracker_core::statistics::repository`
- `torrust_tracker_core::torrent::repository`
- `torrust_tracker_core::torrent::services`
- `torrust_tracker_core::whitelist::manager`

#### `torrust-tracker-http-core` [normal]

- `torrust_tracker_http_tracker_core::container::HttpTrackerCoreContainer`
- `torrust_tracker_http_tracker_core::statistics::repository`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceEvent`
- `torrust_tracker_primitives::pagination::Pagination`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::fixture`

#### `torrust-tracker-rest-api-client` [normal]

- `torrust_tracker_rest_api_client::common::http`
- `torrust_tracker_rest_api_client::connection_info`
- `torrust_tracker_rest_api_client::connection_info::ConnectionInfo`
- `torrust_tracker_rest_api_client::v1::client`

#### `torrust-tracker-rest-api-core` [normal]

- `torrust_tracker_rest_api_core::container::TrackerHttpApiCoreContainer`
- `torrust_tracker_rest_api_core::statistics::metrics`
- `torrust_tracker_rest_api_core::statistics::services`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`
- `torrust_tracker_swarm_coordination_registry::statistics::repository`

#### `torrust-tracker-udp-server` [normal]

- `torrust_tracker_udp_server::container::UdpTrackerServerContainer`
- `torrust_tracker_udp_server::statistics::repository`

#### `torrust-tracker-udp-core` [normal]

- `torrust_tracker_udp_tracker_core::container::UdpTrackerCoreContainer`
- `torrust_tracker_udp_tracker_core::initialize_static`
- `torrust_tracker_udp_tracker_core::services::banning`
- `torrust_tracker_udp_tracker_core::statistics::repository`

#### `torrust-tracker-rest-api-client` [dev]

- `torrust_tracker_rest_api_client::common::http`
- `torrust_tracker_rest_api_client::connection_info`
- `torrust_tracker_rest_api_client::connection_info::ConnectionInfo`
- `torrust_tracker_rest_api_client::v1::client`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration::ephemeral_public`
- `torrust_tracker_test_helpers::logging::logs_contains_a_line_with`

### `torrust-tracker-axum-server`

Workspace deps: 2

#### `torrust-server-lib` [normal]

- `torrust_server_lib::signals`

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::TslConfig`

### `torrust-tracker-client`

Workspace deps: 2

#### `torrust-tracker-client-lib` [normal]

_No `torrust_tracker_client_lib::` references found in source — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-tracker-udp-protocol` [normal]

- `torrust_tracker_udp_tracker_protocol::PeerId`
- `torrust_tracker_udp_tracker_protocol::Response`
- `torrust_tracker_udp_tracker_protocol::TransactionId`
- `torrust_tracker_udp_tracker_protocol::common::InfoHash`

### `torrust-tracker-client-lib`

Workspace deps: 2

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::peer`

#### `torrust-tracker-udp-protocol` [normal]

- `torrust_tracker_udp_tracker_protocol::PeerId`
- `torrust_tracker_udp_tracker_protocol::Request`

### `torrust-tracker-configuration`

Workspace deps: 1

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnouncePolicy`
- `torrust_tracker_primitives::announce::AnnouncePolicy`

### `torrust-tracker-core`

Workspace deps: 5

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::Configuration`
- `torrust_tracker_configuration::Core`
- `torrust_tracker_configuration::Driver::MySQL`
- `torrust_tracker_configuration::Driver::PostgreSQL`
- `torrust_tracker_configuration::Driver::Sqlite3`

#### `torrust-tracker-events` [normal]

- `torrust_tracker_events::receiver::RecvError`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceEvent`
- `torrust_tracker_primitives::AnnouncePolicy`
- `torrust_tracker_primitives::NumberOfBytes`
- `torrust_tracker_primitives::NumberOfDownloads`
- `torrust_tracker_primitives::NumberOfDownloadsPerInfoHash`
- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::PrivateMode`
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
- `torrust_tracker_swarm_coordination_registry::statistics::event`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`
- `torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database`

### `torrust-tracker-e2e-tools`

Workspace deps: 1

#### `torrust-tracker` [normal]

_No `torrust_tracker::` references found in source — may be used only in `Cargo.toml` feature flags or `build.rs`._

### `torrust-tracker-http-core`

Workspace deps: 7

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::Configuration`
- `torrust_tracker_configuration::Core`

#### `torrust-tracker-core` [normal]

- `torrust_tracker_core::announce_handler`
- `torrust_tracker_core::announce_handler::AnnounceHandler`
- `torrust_tracker_core::announce_handler::PeersWanted`
- `torrust_tracker_core::authentication`
- `torrust_tracker_core::authentication::key`
- `torrust_tracker_core::authentication::service`
- `torrust_tracker_core::container::TrackerCoreContainer`
- `torrust_tracker_core::databases::setup`
- `torrust_tracker_core::error`
- `torrust_tracker_core::error::TrackerCoreError`
- `torrust_tracker_core::scrape_handler::ScrapeHandler`
- `torrust_tracker_core::statistics::persisted`
- `torrust_tracker_core::torrent::repository`
- `torrust_tracker_core::whitelist`
- `torrust_tracker_core::whitelist::authorization`
- `torrust_tracker_core::whitelist::repository`

#### `torrust-tracker-events` [normal]

- `torrust_tracker_events::broadcaster::Broadcaster`
- `torrust_tracker_events::bus::EventBus`
- `torrust_tracker_events::bus::SenderStatus`
- `torrust_tracker_events::receiver::Receiver`
- `torrust_tracker_events::receiver::RecvError`
- `torrust_tracker_events::sender::SendError`
- `torrust_tracker_events::sender::Sender`

#### `torrust-tracker-http-protocol` [normal]

- `torrust_tracker_http_tracker_protocol::v1::requests`
- `torrust_tracker_http_tracker_protocol::v1::responses`
- `torrust_tracker_http_tracker_protocol::v1::services`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceEvent::Completed`
- `torrust_tracker_primitives::AnnounceEvent::None`
- `torrust_tracker_primitives::AnnounceEvent::Started`
- `torrust_tracker_primitives::AnnounceEvent::Stopped`
- `torrust_tracker_primitives::ScrapeData`
- `torrust_tracker_primitives::peer::Peer`
- `torrust_tracker_primitives::peer::PeerAnnouncement`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`

### `torrust-tracker-persistence-benchmark`

Workspace deps: 2

#### `torrust-tracker-configuration` [normal]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob import)._

#### `torrust-tracker-core` [normal]

- `torrust_tracker_core::authentication`
- `torrust_tracker_core::databases`
- `torrust_tracker_core::databases::AuthKeyStore`
- `torrust_tracker_core::databases::Database`
- `torrust_tracker_core::databases::SchemaMigrator`
- `torrust_tracker_core::databases::TorrentMetricsStore`
- `torrust_tracker_core::databases::WhitelistStore`
- `torrust_tracker_core::databases::driver`
- `torrust_tracker_core::databases::setup`

### `torrust-tracker-rest-api-core`

Workspace deps: 9

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::Configuration`

#### `torrust-tracker-core` [normal]

- `torrust_tracker_core::container::TrackerCoreContainer`
- `torrust_tracker_core::statistics::repository`
- `torrust_tracker_core::torrent::repository`

#### `torrust-tracker-http-core` [normal]

- `torrust_tracker_http_tracker_core::container::HttpTrackerCoreContainer`
- `torrust_tracker_http_tracker_core::event::bus`
- `torrust_tracker_http_tracker_core::event::sender`
- `torrust_tracker_http_tracker_core::statistics::event`
- `torrust_tracker_http_tracker_core::statistics::repository`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`
- `torrust_tracker_swarm_coordination_registry::statistics::repository`

#### `torrust-tracker-udp-server` [normal]

- `torrust_tracker_udp_server::container::UdpTrackerServerContainer`
- `torrust_tracker_udp_server::statistics`
- `torrust_tracker_udp_server::statistics::repository`

#### `torrust-tracker-udp-core` [normal]

- `torrust_tracker_udp_tracker_core::MAX_CONNECTION_ID_ERRORS_PER_IP`
- `torrust_tracker_udp_tracker_core::container::UdpTrackerCoreContainer`
- `torrust_tracker_udp_tracker_core::services::banning`
- `torrust_tracker_udp_tracker_core::statistics::repository`

#### `torrust-tracker-events` [dev]

- `torrust_tracker_events::bus::SenderStatus`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`

### `torrust-tracker-swarm-coordination-registry`

Workspace deps: 2

#### `torrust-tracker-events` [normal]

- `torrust_tracker_events::broadcaster::Broadcaster`
- `torrust_tracker_events::bus::EventBus`
- `torrust_tracker_events::bus::SenderStatus`
- `torrust_tracker_events::receiver::Receiver`
- `torrust_tracker_events::receiver::RecvError`
- `torrust_tracker_events::sender`
- `torrust_tracker_events::sender::Sender`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceEvent::Completed`
- `torrust_tracker_primitives::AnnounceEvent::Started`
- `torrust_tracker_primitives::NumberOfBytes`
- `torrust_tracker_primitives::NumberOfDownloadsPerInfoHash`
- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::TrackerPolicy`
- `torrust_tracker_primitives::pagination::Pagination`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::Peer`
- `torrust_tracker_primitives::peer::PeerRole`
- `torrust_tracker_primitives::peer::fixture`
- `torrust_tracker_primitives::swarm_metadata`
- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

### `torrust-tracker-test-helpers`

Workspace deps: 1

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::logging::TraceStyle`

### `torrust-tracker-torrent-repository-benchmarking`

Workspace deps: 1

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::pagination::Pagination`
- `torrust_tracker_primitives::peer`
- `torrust_tracker_primitives::peer::Peer`
- `torrust_tracker_primitives::peer::ReadInfo`
- `torrust_tracker_primitives::peer::fixture`
- `torrust_tracker_primitives::swarm_metadata`
- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

### `torrust-tracker-udp-server`

Workspace deps: 10

#### `torrust-server-lib` [normal]

- `torrust_server_lib::logging::STARTED_ON`
- `torrust_server_lib::registar`
- `torrust_server_lib::registar::Registar`
- `torrust_server_lib::registar::ServiceHealthCheckJob`
- `torrust_server_lib::signals`

#### `torrust-tracker-client-lib` [normal]

_No `torrust_tracker_client_lib::` references found in source — may be used only in `Cargo.toml` feature flags or `build.rs`._

#### `torrust-tracker-configuration` [normal]

- `torrust_tracker_configuration::Core`

#### `torrust-tracker-core` [normal]

- `torrust_tracker_core::MAX_SCRAPE_TORRENTS`
- `torrust_tracker_core::announce_handler::AnnounceHandler`
- `torrust_tracker_core::container::TrackerCoreContainer`
- `torrust_tracker_core::databases::setup`
- `torrust_tracker_core::error`
- `torrust_tracker_core::scrape_handler::ScrapeHandler`
- `torrust_tracker_core::statistics::persisted`
- `torrust_tracker_core::torrent::repository`
- `torrust_tracker_core::whitelist`
- `torrust_tracker_core::whitelist::authorization`
- `torrust_tracker_core::whitelist::repository`

#### `torrust-tracker-events` [normal]

- `torrust_tracker_events::broadcaster::Broadcaster`
- `torrust_tracker_events::bus::EventBus`
- `torrust_tracker_events::bus::SenderStatus`
- `torrust_tracker_events::receiver::Receiver`
- `torrust_tracker_events::receiver::RecvError`
- `torrust_tracker_events::sender::SendError`
- `torrust_tracker_events::sender::Sender`

#### `torrust-tracker-primitives` [normal]

- `torrust_tracker_primitives::AnnounceData`
- `torrust_tracker_primitives::PeerId`
- `torrust_tracker_primitives::ScrapeData`
- `torrust_tracker_primitives::peer::fixture`
- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`
- `torrust_tracker_primitives::swarm_metadata::SwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`

#### `torrust-tracker-udp-core` [normal]

- `torrust_tracker_udp_tracker_core::UDP_TRACKER_LOG_TARGET`
- `torrust_tracker_udp_tracker_core::connection_cookie`
- `torrust_tracker_udp_tracker_core::connection_cookie::gen_remote_fingerprint`
- `torrust_tracker_udp_tracker_core::connection_cookie::make`
- `torrust_tracker_udp_tracker_core::container::UdpTrackerCoreContainer`
- `torrust_tracker_udp_tracker_core::event`
- `torrust_tracker_udp_tracker_core::event::Event`
- `torrust_tracker_udp_tracker_core::event::bus`
- `torrust_tracker_udp_tracker_core::event::sender`
- `torrust_tracker_udp_tracker_core::initialize_static`
- `torrust_tracker_udp_tracker_core::services::announce`
- `torrust_tracker_udp_tracker_core::services::banning`
- `torrust_tracker_udp_tracker_core::services::connect`
- `torrust_tracker_udp_tracker_core::services::scrape`
- `torrust_tracker_udp_tracker_core::statistics::event`

#### `torrust-tracker-udp-protocol` [normal]

- `torrust_tracker_udp_tracker_protocol::AnnounceEvent`
- `torrust_tracker_udp_tracker_protocol::AnnounceInterval`
- `torrust_tracker_udp_tracker_protocol::AnnounceRequest`
- `torrust_tracker_udp_tracker_protocol::InfoHash`
- `torrust_tracker_udp_tracker_protocol::PeerClient`
- `torrust_tracker_udp_tracker_protocol::Response`
- `torrust_tracker_udp_tracker_protocol::TransactionId`
- `torrust_tracker_udp_tracker_protocol::common::ConnectionId`
- `torrust_tracker_udp_tracker_protocol::common::InfoHash`
- `torrust_tracker_udp_tracker_protocol::common::NumberOfBytes`
- `torrust_tracker_udp_tracker_protocol::common::NumberOfPeers`
- `torrust_tracker_udp_tracker_protocol::common::PeerId`
- `torrust_tracker_udp_tracker_protocol::common::Port`
- `torrust_tracker_udp_tracker_protocol::common::ResponsePeer`
- `torrust_tracker_udp_tracker_protocol::common::TransactionId`
- `torrust_tracker_udp_tracker_protocol::request::ConnectRequest`
- `torrust_tracker_udp_tracker_protocol::request::ScrapeRequest`
- `torrust_tracker_udp_tracker_protocol::response::AnnounceResponse`
- `torrust_tracker_udp_tracker_protocol::response::ConnectResponse`
- `torrust_tracker_udp_tracker_protocol::response::ScrapeResponse`
- `torrust_tracker_udp_tracker_protocol::response::TorrentScrapeStatistics`

#### `torrust-tracker-test-helpers` [dev]

- `torrust_tracker_test_helpers::configuration`
- `torrust_tracker_test_helpers::configuration::ephemeral_public`
- `torrust_tracker_test_helpers::logging::logs_contains_a_line_with`

### `torrust-tracker-udp-core`

Workspace deps: 6

#### `torrust-tracker-configuration` [normal]

_Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob import)._

#### `torrust-tracker-core` [normal]

- `torrust_tracker_core::announce_handler`
- `torrust_tracker_core::container::TrackerCoreContainer`
- `torrust_tracker_core::error`
- `torrust_tracker_core::scrape_handler::ScrapeHandler`
- `torrust_tracker_core::torrent::repository`
- `torrust_tracker_core::whitelist`

#### `torrust-tracker-events` [normal]

- `torrust_tracker_events::broadcaster::Broadcaster`
- `torrust_tracker_events::bus::EventBus`
- `torrust_tracker_events::bus::SenderStatus`
- `torrust_tracker_events::receiver::Receiver`
- `torrust_tracker_events::receiver::RecvError`
- `torrust_tracker_events::sender::SendError`
- `torrust_tracker_events::sender::Sender`

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
- `torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata`

#### `torrust-tracker-swarm-coordination-registry` [normal]

- `torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer`

#### `torrust-tracker-udp-protocol` [normal]

- `torrust_tracker_udp_tracker_protocol::AnnounceEvent::Completed`
- `torrust_tracker_udp_tracker_protocol::AnnounceEvent::None`
- `torrust_tracker_udp_tracker_protocol::AnnounceEvent::Started`
- `torrust_tracker_udp_tracker_protocol::AnnounceEvent::Stopped`
- `torrust_tracker_udp_tracker_protocol::AnnounceEvent::from`
- `torrust_tracker_udp_tracker_protocol::AnnounceRequest`
- `torrust_tracker_udp_tracker_protocol::ConnectionId`
- `torrust_tracker_udp_tracker_protocol::ScrapeRequest`
- `torrust_tracker_udp_tracker_protocol::common::InfoHash`

---

## Observations

To be filled in after reviewing the report above.

### Known thin dependencies (pre-existing)

None — previously known thin dependencies have been resolved:

- `torrust-clock` → `torrust-tracker-primitives` (resolved by SI-02, #1790)
- `torrust-tracker-configuration` → `torrust-clock` (resolved by SI-03, #1793)
- `torrust-tracker-configuration` → `torrust-tracker-primitives`: only
  `TrackerPolicy`/`PrivateMode`/`TORRENT_PEERS_LIMIT` imported. Addressed by FU-1
  (#1859) — items moved to `primitives`. Remaining dependency is `AnnouncePolicy`
  from `primitives` (architecturally expected — config types reference domain
  types).

### Improvements since the previous report (2026-05-19)

Comparing the baseline report against this regenerated version shows measurable
reduction in workspace coupling thanks to completed EPIC subissues:

| Metric                     | Before (May 19) | After (Jun 10)               |
| -------------------------- | --------------- | ---------------------------- |
| Workspace packages         | 29              | 25                           |
| Leaf packages (no ws deps) | 8               | 7 (completely different set) |
| Highest dep count          | 16 (2 packages) | 15                           |

**Packages extracted to standalone repositories** (removed from workspace):

- `bittorrent-peer-id` → `torrust-peer-id` (SI-19, #1884)
- `torrust-clock` (SI-17, #1879)
- `torrust-located-error` (SI-22, #1894)
- `torrust-metrics` (SI-18, #1882)
- `torrust-net-primitives` (SI-20, #1885)
- `torrust-tracker-contrib-bencode` → `torrust-bencode` in `torrust/torrust-bittorrent` (SI-16, #1881)

**Protocol packages decoupled from domain** (SI-12, SI-13, SI-14):

- `torrust-tracker-http-protocol`: **6 → 0** workspace deps (now a leaf)
- `torrust-tracker-udp-protocol`: **1 → 0** workspace deps (now a leaf)

**Core dependency reductions**:

- `torrust-tracker-core` (was `bittorrent-tracker-core`): **9 → 5** deps
- `torrust-tracker-http-core` (was `bittorrent-http-core`): **10 → 7** deps
- `torrust-tracker-udp-core` (was `bittorrent-udp-core`): **10 → 6** deps

**Server dependency reductions** (from renamed/moved dependencies):

- `torrust-tracker-axum-http-server`: **14 → 10** deps
- `torrust-tracker-axum-rest-api-server`: **16 → 13** deps
- `torrust-tracker-axum-health-check-api-server`: **10 → 8** deps
- `torrust-tracker-udp-server`: **13 → 10** deps

**Domain/shared leafification**:

- `torrust-tracker-primitives`: **3 → 0** deps (now a leaf — ServiceBinding → net-primitives, InfoHash → extracted)
- `torrust-server-lib`: **1 → 0** deps (now a leaf — net-primitives extracted)

**Other reductions**:

- `torrust-tracker-swarm-coordination-registry`: **6 → 2** deps (FU-1 moved TrackerPolicy/TORRENT_PEERS_LIMIT/PrivateMode)
- `torrust-tracker-torrent-repository-benchmarking`: **3 → 1** dep (FU-1 removed config dependency)
- `torrust-tracker-configuration`: **2 → 1** dep

### New findings

Record any new thin-dependency or cluster-dependency findings here, with a
reference to the subissue opened for each.

#### Thin dependencies worth investigating

1. **`axum-http-server` → `udp-tracker-protocol`** (1 import: `PeerId`)
   The HTTP server depends on the UDP protocol crate solely for `PeerId`.
   Should use `torrust-peer-id` directly (already an external dep).
   Draft spec: [docs/issues/drafts/1669-remove-udp-protocol-peer-id-re-export.md](../../drafts/1669-remove-udp-protocol-peer-id-re-export.md)

#### Domain concept misplacement

1. **`tracker-core` → `Driver` enum in `configuration`**
   `Driver` is a cross-cutting domain concept (database backend selection), not
   a configuration DTO. It is used by `configuration`, `tracker-core`, and
   `persistence-benchmark`. The current duplication in `tracker-core` (its own
   copy of the enum with a pointless mapping in `setup.rs`) is a symptom of
   misplaced ownership. `Driver` should live in `primitives` — a shared home
   for stable, cross-cutting domain types.
   Draft spec: [docs/issues/drafts/1669-move-driver-enum-to-primitives.md](../../drafts/1669-move-driver-enum-to-primitives.md)

#### Acceptable thin dependencies (not worth addressing)

- **`axum-server` → `configuration`** (1 import: `TslConfig`)
  Deliberately kept per [DEC-08](../DECISIONS.md#dec-08--keep-tslconfig-in-tracker-configuration-and-keep-torrust-tracker-axum-server-tracker-scoped):
  `TslConfig` is the public DTO in the tracker configuration contract
  (see [issue #1860](../../closed/1860-1669-evaluate-tslconfig-move-to-axum-server/ISSUE.md)).
  Moving it would invert the dependency direction or require a separate
  package — overkill for a two-field stable struct.

- **`tracker-core` → `events`** (1 import: `RecvError`)
  Kept as a direct dependency — `tracker-core` uses the events system directly
  and re-exporting `RecvError` through an intermediate package would create a
  hidden transitive dependency that makes the graph harder to reason about.

- **`configuration` → `primitives`** (1 import path: `AnnouncePolicy`)
  Architecturally expected — config types reference domain types.

- **`test-helpers` → `configuration`** (1 import: `TraceStyle`)
  Test utilities referencing production types — natural and acceptable.

- **`udp-server` → `client-lib`** (uses `torrust_tracker_client::udp::client::check`
  — the old crate name before `torrust-tracker-client-lib`)
  The UDP server imports a `check` function from the client library for its
  own health check. This is a standard pattern: the server uses its client
  to self-test its availability. Acceptable per [DEC-11](../DECISIONS.md#dec-11--accept-server--client-library-dependency-for-health-checks).

- **`e2e-tools` → `tracker` (root)** (uses `torrust_tracker_lib::`)
  The scan looks for `torrust_tracker::` (the crate module name), but the
  root crate lib is named `torrust_tracker_lib`, so binaries import it as
  `use torrust_tracker_lib::console::ci::e2e` etc. This is a real dependency
  — e2e-tools binaries call into the tracker's console entry points.

#### Cluster dependencies (architectural concerns)

1. **`axum-rest-api-server` -> `udp-server` + `udp-core`**
   The REST server container depends on concrete UDP containers for wiring and
   initialization. See draft:
   [1669-decouple-axum-rest-api-server-from-udp-containers.md](../../drafts/1669-decouple-axum-rest-api-server-from-udp-containers.md)

2. **`rest-api-core` -> `udp-server` + `udp-core`**
   The REST core depends on concrete UDP types for statistics and banning.
   See draft:
   [1669-decouple-rest-api-core-from-udp-internals.md](../../drafts/1669-decouple-rest-api-core-from-udp-internals.md)

3. **`http-core` -> `tracker-core`** (16 import paths)
   This is an **architecturally expected** coupling, not a problem to fix.
   `http-core` is a thin protocol-specific layer that delegates
   to `tracker-core`. The imports break down as:
   - **Runtime** (12 paths): container wrapping (`TrackerCoreContainer`),
     handler delegation (`AnnounceHandler`, `ScrapeHandler`), auth
     (`AuthenticationService`, `Key`), whitelist, error types, and
     metrics persistence. These are the API boundary — expected.
   - **Test-only** (4 paths): `initialize_database`, `InMemoryKeyRepository`,
     `InMemoryTorrentRepository`, `InMemoryWhitelist`. Used only in `#[cfg(test)]`.
     Moving test helpers to `test-helpers` is possible but minor.
     Per [DEC-12](../DECISIONS.md#dec-12--accept-http-core-to-tracker-core-coupling-as-by-design).

#### Recommended prioritization

| Priority | Edge                                         | Change                                    | Est. effort |
| -------- | -------------------------------------------- | ----------------------------------------- | ----------- |
| 1        | `axum-http-server` → `udp-tracker-protocol`  | Replace with `torrust-peer-id`            | Very low    |
| 2        | `tracker-core` → `Driver` in `configuration` | Move `Driver` enum to `primitives`        | Low         |
| 3        | REST layer → UDP internals                   | Trait-based abstraction for stats/banning | Medium      |
