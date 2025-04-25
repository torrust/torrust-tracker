use std::sync::Arc;

use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use bittorrent_udp_tracker_core::services::banning::BanService;
use bittorrent_udp_tracker_core::{self};
use tokio::sync::RwLock;
use torrust_tracker_metrics::metric_collection::MetricCollection;
use torrust_tracker_primitives::swarm_metadata::AggregateSwarmMetadata;
use torrust_udp_tracker_server::statistics as udp_server_statistics;

use crate::statistics::metrics::Metrics;

/// All the metrics collected by the tracker.
#[derive(Debug, PartialEq)]
pub struct TrackerMetrics {
    /// Domain level metrics.
    ///
    /// General metrics for all torrents (number of seeders, leechers, etcetera)
    pub torrents_metrics: AggregateSwarmMetadata,

    /// Application level metrics. Usage statistics/metrics.
    ///
    /// Metrics about how the tracker is been used (number of udp announce requests, number of http scrape requests, etcetera)
    pub protocol_metrics: Metrics,
}

/// It returns all the [`TrackerMetrics`]
#[allow(deprecated)]
pub async fn get_metrics(
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    ban_service: Arc<RwLock<BanService>>,
    http_stats_repository: Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,
    udp_server_stats_repository: Arc<udp_server_statistics::repository::Repository>,
) -> TrackerMetrics {
    let torrents_metrics = in_memory_torrent_repository.get_torrents_metrics();
    let udp_banned_ips_total = ban_service.read().await.get_banned_ips_total();
    let http_stats = http_stats_repository.get_stats().await;
    let udp_server_stats = udp_server_stats_repository.get_stats().await;

    // For backward compatibility we keep the `tcp4_connections_handled` and
    // `tcp6_connections_handled` metrics. They don't make sense for the HTTP
    // tracker, but we keep them for now. In new major versions we should remove
    // them.

    TrackerMetrics {
        torrents_metrics,
        protocol_metrics: Metrics {
            // TCPv4
            tcp4_connections_handled: http_stats.tcp4_announces_handled + http_stats.tcp4_scrapes_handled,
            tcp4_announces_handled: http_stats.tcp4_announces_handled,
            tcp4_scrapes_handled: http_stats.tcp4_scrapes_handled,
            // TCPv6
            tcp6_connections_handled: http_stats.tcp6_announces_handled + http_stats.tcp6_scrapes_handled,
            tcp6_announces_handled: http_stats.tcp6_announces_handled,
            tcp6_scrapes_handled: http_stats.tcp6_scrapes_handled,
            // UDP
            udp_requests_aborted: udp_server_stats.udp_requests_aborted,
            udp_requests_banned: udp_server_stats.udp_requests_banned,
            udp_banned_ips_total: udp_banned_ips_total as u64,
            udp_avg_connect_processing_time_ns: udp_server_stats.udp_avg_connect_processing_time_ns,
            udp_avg_announce_processing_time_ns: udp_server_stats.udp_avg_announce_processing_time_ns,
            udp_avg_scrape_processing_time_ns: udp_server_stats.udp_avg_scrape_processing_time_ns,
            // UDPv4
            udp4_requests: udp_server_stats.udp4_requests,
            udp4_connections_handled: udp_server_stats.udp4_connections_handled,
            udp4_announces_handled: udp_server_stats.udp4_announces_handled,
            udp4_scrapes_handled: udp_server_stats.udp4_scrapes_handled,
            udp4_responses: udp_server_stats.udp4_responses,
            udp4_errors_handled: udp_server_stats.udp4_errors_handled,
            // UDPv6
            udp6_requests: udp_server_stats.udp6_requests,
            udp6_connections_handled: udp_server_stats.udp6_connections_handled,
            udp6_announces_handled: udp_server_stats.udp6_announces_handled,
            udp6_scrapes_handled: udp_server_stats.udp6_scrapes_handled,
            udp6_responses: udp_server_stats.udp6_responses,
            udp6_errors_handled: udp_server_stats.udp6_errors_handled,
        },
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
#[allow(deprecated)]
pub async fn get_labeled_metrics(
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    ban_service: Arc<RwLock<BanService>>,
    http_stats_repository: Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,
    udp_stats_repository: Arc<bittorrent_udp_tracker_core::statistics::repository::Repository>,
    udp_server_stats_repository: Arc<udp_server_statistics::repository::Repository>,
) -> TrackerLabeledMetrics {
    let _torrents_metrics = in_memory_torrent_repository.get_torrents_metrics();
    let _udp_banned_ips_total = ban_service.read().await.get_banned_ips_total();

    let http_stats = http_stats_repository.get_stats().await;
    let udp_stats_repository = udp_stats_repository.get_stats().await;
    let udp_server_stats = udp_server_stats_repository.get_stats().await;

    // Merge all the metrics into a single collection
    let mut metrics = MetricCollection::default();
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

    use bittorrent_http_tracker_core::event::sender::Broadcaster;
    use bittorrent_http_tracker_core::statistics::event::listener::run_event_listener;
    use bittorrent_http_tracker_core::statistics::keeper::Keeper;
    use bittorrent_http_tracker_core::statistics::repository::Repository;
    use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use bittorrent_tracker_core::{self};
    use bittorrent_udp_tracker_core::services::banning::BanService;
    use bittorrent_udp_tracker_core::MAX_CONNECTION_ID_ERRORS_PER_IP;
    use tokio::sync::RwLock;
    use torrust_tracker_configuration::Configuration;
    use torrust_tracker_primitives::swarm_metadata::AggregateSwarmMetadata;
    use torrust_tracker_test_helpers::configuration;

    use crate::statistics::metrics::Metrics;
    use crate::statistics::services::{get_metrics, TrackerMetrics};

    pub fn tracker_configuration() -> Configuration {
        configuration::ephemeral()
    }

    #[tokio::test]
    async fn the_statistics_service_should_return_the_tracker_metrics() {
        let config = tracker_configuration();

        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let ban_service = Arc::new(RwLock::new(BanService::new(MAX_CONNECTION_ID_ERRORS_PER_IP)));

        // HTTP core stats
        let http_core_broadcaster = Broadcaster::default();
        let http_stats_repository = Arc::new(Repository::new());
        let http_stats_keeper = Arc::new(Keeper::new(
            config.core.tracker_usage_statistics,
            http_core_broadcaster.clone(),
        ));

        if config.core.tracker_usage_statistics {
            let _unused = run_event_listener(http_stats_keeper.receiver(), &http_stats_repository);
        }

        // UDP core stats (not used in this test)

        // UDP server stats
        let (_udp_server_stats_keeper, udp_server_stats_repository) =
            torrust_udp_tracker_server::statistics::setup::factory(config.core.tracker_usage_statistics);

        let tracker_metrics = get_metrics(
            in_memory_torrent_repository.clone(),
            ban_service.clone(),
            http_stats_repository.clone(),
            udp_server_stats_repository.clone(),
        )
        .await;

        assert_eq!(
            tracker_metrics,
            TrackerMetrics {
                torrents_metrics: AggregateSwarmMetadata::default(),
                protocol_metrics: Metrics::default(),
            }
        );
    }
}
