use std::sync::Arc;

use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use bittorrent_udp_tracker_core::services::banning::BanService;
use bittorrent_udp_tracker_core::{self};
use tokio::sync::RwLock;
use torrust_tracker_metrics::metric_collection::MetricCollection;
use torrust_udp_tracker_server::statistics::{self as udp_server_statistics};

use super::metrics::TorrentsMetrics;
use crate::statistics::metrics::ProtocolMetrics;

/// All the metrics collected by the tracker.
#[derive(Debug, PartialEq)]
pub struct TrackerMetrics {
    /// Domain level metrics.
    ///
    /// General metrics for all torrents (number of seeders, leechers, etcetera)
    pub torrents_metrics: TorrentsMetrics,

    /// Application level metrics. Usage statistics/metrics.
    ///
    /// Metrics about how the tracker is been used (number of udp announce requests, number of http scrape requests, etcetera)
    pub protocol_metrics: ProtocolMetrics,
}

/// It returns all the [`TrackerMetrics`]
pub async fn get_metrics(
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    tracker_core_stats_repository: Arc<bittorrent_tracker_core::statistics::repository::Repository>,
    http_stats_repository: Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,
    udp_server_stats_repository: Arc<udp_server_statistics::repository::Repository>,
) -> TrackerMetrics {
    TrackerMetrics {
        torrents_metrics: get_torrents_metrics(in_memory_torrent_repository, tracker_core_stats_repository).await,
        protocol_metrics: get_protocol_metrics(http_stats_repository.clone(), udp_server_stats_repository.clone()).await,
    }
}

async fn get_torrents_metrics(
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,

    tracker_core_stats_repository: Arc<bittorrent_tracker_core::statistics::repository::Repository>,
) -> TorrentsMetrics {
    let aggregate_active_swarm_metadata = in_memory_torrent_repository.get_aggregate_swarm_metadata().await;

    let mut torrents_metrics: TorrentsMetrics = aggregate_active_swarm_metadata.into();
    torrents_metrics.total_downloaded = tracker_core_stats_repository.get_torrents_downloads_total().await;

    torrents_metrics
}

#[allow(deprecated)]
#[allow(clippy::too_many_lines)]
async fn get_protocol_metrics(
    http_stats_repository: Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,
    udp_server_stats_repository: Arc<udp_server_statistics::repository::Repository>,
) -> ProtocolMetrics {
    let http_stats = http_stats_repository.get_stats().await;
    let udp_server_stats = udp_server_stats_repository.get_stats().await;

    // TCPv4

    let tcp4_announces_handled = http_stats.tcp4_announces_handled();
    let tcp4_scrapes_handled = http_stats.tcp4_scrapes_handled();

    // TCPv6

    let tcp6_announces_handled = http_stats.tcp6_announces_handled();
    let tcp6_scrapes_handled = http_stats.tcp6_scrapes_handled();

    // UDP

    let udp_requests_aborted = udp_server_stats.udp_requests_aborted_total();
    let udp_requests_banned = udp_server_stats.udp_requests_banned_total();
    let udp_banned_ips_total = udp_server_stats.udp_banned_ips_total();
    let udp_avg_connect_processing_time_ns = udp_server_stats.udp_avg_connect_processing_time_ns_averaged();
    let udp_avg_announce_processing_time_ns = udp_server_stats.udp_avg_announce_processing_time_ns_averaged();
    let udp_avg_scrape_processing_time_ns = udp_server_stats.udp_avg_scrape_processing_time_ns_averaged();

    // UDPv4

    let udp4_requests = udp_server_stats.udp4_requests_received_total();
    let udp4_connections_handled = udp_server_stats.udp4_connect_requests_accepted_total();
    let udp4_announces_handled = udp_server_stats.udp4_announce_requests_accepted_total();
    let udp4_scrapes_handled = udp_server_stats.udp4_scrape_requests_accepted_total();
    let udp4_responses = udp_server_stats.udp4_responses_sent_total();
    let udp4_errors_handled = udp_server_stats.udp4_errors_total();

    // UDPv6

    let udp6_requests = udp_server_stats.udp6_requests_received_total();
    let udp6_connections_handled = udp_server_stats.udp6_connect_requests_accepted_total();
    let udp6_announces_handled = udp_server_stats.udp6_announce_requests_accepted_total();
    let udp6_scrapes_handled = udp_server_stats.udp6_scrape_requests_accepted_total();
    let udp6_responses = udp_server_stats.udp6_responses_sent_total();
    let udp6_errors_handled = udp_server_stats.udp6_errors_total();

    // For backward compatibility we keep the `tcp4_connections_handled` and
    // `tcp6_connections_handled` metrics. They don't make sense for the HTTP
    // tracker, but we keep them for now. In new major versions we should remove
    // them.

    ProtocolMetrics {
        // TCPv4
        tcp4_connections_handled: tcp4_announces_handled + tcp4_scrapes_handled,
        tcp4_announces_handled,
        tcp4_scrapes_handled,
        // TCPv6
        tcp6_connections_handled: tcp6_announces_handled + tcp6_scrapes_handled,
        tcp6_announces_handled,
        tcp6_scrapes_handled,
        // UDP
        udp_requests_aborted,
        udp_requests_banned,
        udp_banned_ips_total,
        udp_avg_connect_processing_time_ns,
        udp_avg_announce_processing_time_ns,
        udp_avg_scrape_processing_time_ns,
        // UDPv4
        udp4_requests,
        udp4_connections_handled,
        udp4_announces_handled,
        udp4_scrapes_handled,
        udp4_responses,
        udp4_errors_handled,
        // UDPv6
        udp6_requests,
        udp6_connections_handled,
        udp6_announces_handled,
        udp6_scrapes_handled,
        udp6_responses,
        udp6_errors_handled,
    }
}

#[derive(Debug, PartialEq)]
pub struct TrackerLabeledMetrics {
    pub metrics: MetricCollection,
}

/// It returns all the [`TrackerLabeledMetrics`]
///
/// # Panics
///
/// Will panic if the metrics cannot be merged. This could happen if the
/// packages are producing duplicate metric names, for example.
pub async fn get_labeled_metrics(
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    ban_service: Arc<RwLock<BanService>>,
    swarms_stats_repository: Arc<torrust_tracker_swarm_coordination_registry::statistics::repository::Repository>,
    tracker_core_stats_repository: Arc<bittorrent_tracker_core::statistics::repository::Repository>,
    http_stats_repository: Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,
    udp_stats_repository: Arc<bittorrent_udp_tracker_core::statistics::repository::Repository>,
    udp_server_stats_repository: Arc<udp_server_statistics::repository::Repository>,
) -> TrackerLabeledMetrics {
    let _torrents_metrics = in_memory_torrent_repository.get_aggregate_swarm_metadata();
    let _udp_banned_ips_total = ban_service.read().await.get_banned_ips_total();

    let swarms_stats = swarms_stats_repository.get_metrics().await;
    let tracker_core_stats = tracker_core_stats_repository.get_metrics().await;
    let http_stats = http_stats_repository.get_stats().await;
    let udp_stats_repository = udp_stats_repository.get_stats().await;
    let udp_server_stats = udp_server_stats_repository.get_stats().await;

    // Merge all the metrics into a single collection
    let mut metrics = MetricCollection::default();

    metrics
        .merge(&swarms_stats.metric_collection)
        .expect("msg: failed to merge torrent repository metrics");
    metrics
        .merge(&tracker_core_stats.metric_collection)
        .expect("msg: failed to merge tracker core metrics");
    metrics
        .merge(&http_stats.metric_collection)
        .expect("msg: failed to merge HTTP core metrics");
    metrics
        .merge(&udp_stats_repository.metric_collection)
        .expect("failed to merge UDP core metrics");
    metrics
        .merge(&udp_server_stats.metric_collection)
        .expect("failed to merge UDP server metrics");

    TrackerLabeledMetrics { metrics }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bittorrent_http_tracker_core::event::bus::EventBus;
    use bittorrent_http_tracker_core::event::sender::Broadcaster;
    use bittorrent_http_tracker_core::statistics::event::listener::run_event_listener;
    use bittorrent_http_tracker_core::statistics::repository::Repository;
    use bittorrent_tracker_core::container::TrackerCoreContainer;
    use bittorrent_tracker_core::{self};
    use bittorrent_udp_tracker_core::services::banning::BanService;
    use bittorrent_udp_tracker_core::MAX_CONNECTION_ID_ERRORS_PER_IP;
    use tokio::sync::RwLock;
    use tokio_util::sync::CancellationToken;
    use torrust_tracker_configuration::Configuration;
    use torrust_tracker_events::bus::SenderStatus;
    use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
    use torrust_tracker_test_helpers::configuration;

    use crate::statistics::metrics::{ProtocolMetrics, TorrentsMetrics};
    use crate::statistics::services::{get_metrics, TrackerMetrics};

    pub fn tracker_configuration() -> Configuration {
        configuration::ephemeral()
    }

    #[tokio::test]
    async fn the_statistics_service_should_return_the_tracker_metrics() {
        let cancellation_token = CancellationToken::new();

        let config = tracker_configuration();
        let core_config = Arc::new(config.core.clone());

        let swarm_coordination_registry_container =
            Arc::new(SwarmCoordinationRegistryContainer::initialize(SenderStatus::Enabled));

        let tracker_core_container =
            TrackerCoreContainer::initialize_from(&core_config, &swarm_coordination_registry_container.clone());

        let _ban_service = Arc::new(RwLock::new(BanService::new(MAX_CONNECTION_ID_ERRORS_PER_IP)));

        // HTTP core stats
        let http_core_broadcaster = Broadcaster::default();
        let http_stats_repository = Arc::new(Repository::new());
        let http_stats_event_bus = Arc::new(EventBus::new(
            config.core.tracker_usage_statistics.into(),
            http_core_broadcaster.clone(),
        ));

        if config.core.tracker_usage_statistics {
            let _unused = run_event_listener(http_stats_event_bus.receiver(), cancellation_token, &http_stats_repository);
        }

        // UDP server stats
        let udp_server_stats_repository = Arc::new(torrust_udp_tracker_server::statistics::repository::Repository::new());

        let tracker_metrics = get_metrics(
            tracker_core_container.in_memory_torrent_repository.clone(),
            tracker_core_container.stats_repository.clone(),
            http_stats_repository.clone(),
            udp_server_stats_repository.clone(),
        )
        .await;

        assert_eq!(
            tracker_metrics,
            TrackerMetrics {
                torrents_metrics: TorrentsMetrics::default(),
                protocol_metrics: ProtocolMetrics::default(),
            }
        );
    }
}
