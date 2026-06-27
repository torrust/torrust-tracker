//! Tracker-specific implementation of [`StatsQueryPort`].
//!
//! Aggregates metrics from all tracker-internal repositories and services.
//! Previously this logic lived in `rest-api-core`; it was moved here as part
//! of the contract-first migration (SI-4), advancing toward the deprecation
//! of `rest-api-core` (SI-5).
use std::sync::Arc;

use async_trait::async_trait;
use torrust_metrics::metric_collection::MetricCollection;
use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use torrust_tracker_rest_api_application::ports::stats::StatsQueryPort;
use torrust_tracker_rest_api_protocol::v1::context::stats::resources::stats::{LabeledStats, Stats};
/// Adapter that queries all tracker-internal data sources and converts
/// domain types to protocol DTOs.
#[allow(clippy::struct_field_names)]
pub struct TrackerStatsAdapter {
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    swarms_stats_repository: Arc<torrust_tracker_swarm_coordination_registry::statistics::repository::Repository>,
    tracker_core_stats_repository: Arc<torrust_tracker_core::statistics::repository::Repository>,
    http_stats_repository: Arc<torrust_tracker_http_core::statistics::repository::Repository>,
    udp_core_stats_repository: Arc<torrust_tracker_udp_core::statistics::repository::Repository>,
    udp_server_stats_repository: Arc<torrust_tracker_udp_server::statistics::repository::Repository>,
}

impl TrackerStatsAdapter {
    /// Creates a new adapter wrapping all tracker repositories and services.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>,
        swarms_stats_repository: &Arc<torrust_tracker_swarm_coordination_registry::statistics::repository::Repository>,
        tracker_core_stats_repository: &Arc<torrust_tracker_core::statistics::repository::Repository>,
        http_stats_repository: &Arc<torrust_tracker_http_core::statistics::repository::Repository>,
        udp_core_stats_repository: &Arc<torrust_tracker_udp_core::statistics::repository::Repository>,
        udp_server_stats_repository: &Arc<torrust_tracker_udp_server::statistics::repository::Repository>,
    ) -> Self {
        Self {
            in_memory_torrent_repository: in_memory_torrent_repository.clone(),
            swarms_stats_repository: swarms_stats_repository.clone(),
            tracker_core_stats_repository: tracker_core_stats_repository.clone(),
            http_stats_repository: http_stats_repository.clone(),
            udp_core_stats_repository: udp_core_stats_repository.clone(),
            udp_server_stats_repository: udp_server_stats_repository.clone(),
        }
    }
}

#[async_trait]
impl StatsQueryPort for TrackerStatsAdapter {
    async fn get_stats(&self) -> Stats {
        let aggregate_swarm_metadata = self.in_memory_torrent_repository.get_aggregate_swarm_metadata().await;

        let total_downloaded = self.tracker_core_stats_repository.get_torrents_downloads_total().await;

        let http_stats = self.http_stats_repository.get_stats().await;
        let udp_server_stats = self.udp_server_stats_repository.get_stats().await;

        Stats {
            // Torrent metrics
            torrents: aggregate_swarm_metadata.total_torrents,
            seeders: aggregate_swarm_metadata.total_complete,
            completed: total_downloaded,
            leechers: aggregate_swarm_metadata.total_incomplete,

            // TCPv4
            tcp4_connections_handled: http_stats.tcp4_announces_handled() + http_stats.tcp4_scrapes_handled(),
            tcp4_announces_handled: http_stats.tcp4_announces_handled(),
            tcp4_scrapes_handled: http_stats.tcp4_scrapes_handled(),

            // TCPv6
            tcp6_connections_handled: http_stats.tcp6_announces_handled() + http_stats.tcp6_scrapes_handled(),
            tcp6_announces_handled: http_stats.tcp6_announces_handled(),
            tcp6_scrapes_handled: http_stats.tcp6_scrapes_handled(),

            // UDP
            udp_requests_aborted: udp_server_stats.udp_requests_aborted_total(),
            udp_requests_banned: udp_server_stats.udp_requests_banned_total(),
            udp_banned_ips_total: udp_server_stats.udp_banned_ips_total(),
            udp_avg_connect_processing_time_ns: udp_server_stats.udp_avg_connect_processing_time_ns_averaged(),
            udp_avg_announce_processing_time_ns: udp_server_stats.udp_avg_announce_processing_time_ns_averaged(),
            udp_avg_scrape_processing_time_ns: udp_server_stats.udp_avg_scrape_processing_time_ns_averaged(),

            // UDPv4
            udp4_requests: udp_server_stats.udp4_requests_received_total(),
            udp4_connections_handled: udp_server_stats.udp4_connect_requests_accepted_total(),
            udp4_announces_handled: udp_server_stats.udp4_announce_requests_accepted_total(),
            udp4_scrapes_handled: udp_server_stats.udp4_scrape_requests_accepted_total(),
            udp4_responses: udp_server_stats.udp4_responses_sent_total(),
            udp4_errors_handled: udp_server_stats.udp4_errors_total(),

            // UDPv6
            udp6_requests: udp_server_stats.udp6_requests_received_total(),
            udp6_connections_handled: udp_server_stats.udp6_connect_requests_accepted_total(),
            udp6_announces_handled: udp_server_stats.udp6_announce_requests_accepted_total(),
            udp6_scrapes_handled: udp_server_stats.udp6_scrape_requests_accepted_total(),
            udp6_responses: udp_server_stats.udp6_responses_sent_total(),
            udp6_errors_handled: udp_server_stats.udp6_errors_total(),
        }
    }

    async fn get_labeled_stats(&self) -> LabeledStats {
        let swarms_stats = self.swarms_stats_repository.get_metrics().await;
        let tracker_core_stats = self.tracker_core_stats_repository.get_metrics().await;
        let http_stats = self.http_stats_repository.get_stats().await;
        let udp_stats = self.udp_core_stats_repository.get_stats().await;
        let udp_server_stats = self.udp_server_stats_repository.get_stats().await;

        let mut metrics = MetricCollection::default();

        metrics
            .merge(&swarms_stats.metric_collection)
            .expect("failed to merge torrent repository metrics");
        metrics
            .merge(&tracker_core_stats.metric_collection)
            .expect("failed to merge tracker core metrics");
        metrics
            .merge(&http_stats.metric_collection)
            .expect("failed to merge HTTP core metrics");
        metrics
            .merge(&udp_stats.metric_collection)
            .expect("failed to merge UDP core metrics");
        metrics
            .merge(&udp_server_stats.metric_collection)
            .expect("failed to merge UDP server metrics");

        LabeledStats { metrics }
    }
}
