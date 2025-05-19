pub mod event;
pub mod metrics;
pub mod repository;

use metrics::Metrics;
use torrust_tracker_metrics::metric::description::MetricDescription;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_metrics::unit::Unit;

// Torrent metrics

const TORRENT_REPOSITORY_TORRENTS_ADDED_TOTAL: &str = "torrent_repository_torrents_added_total";
const TORRENT_REPOSITORY_TORRENTS_REMOVED_TOTAL: &str = "torrent_repository_torrents_removed_total";

const TORRENT_REPOSITORY_TORRENTS_TOTAL: &str = "torrent_repository_torrents_total";
const TORRENT_REPOSITORY_TORRENTS_DOWNLOADS_TOTAL: &str = "torrent_repository_torrents_downloads_total";

// Peers metrics

const TORRENT_REPOSITORY_PEERS_ADDED_TOTAL: &str = "torrent_repository_peers_added_total";
const TORRENT_REPOSITORY_PEERS_REMOVED_TOTAL: &str = "torrent_repository_peers_removed_total";
const TORRENT_REPOSITORY_PEERS_UPDATED_TOTAL: &str = "torrent_repository_peers_updated_total";

const TORRENT_REPOSITORY_PEER_CONNECTIONS_TOTAL: &str = "torrent_repository_peer_connections_total";
const TORRENT_REPOSITORY_UNIQUE_PEERS_TOTAL: &str = "torrent_repository_unique_peers_total"; // todo: not implemented yet

#[must_use]
pub fn describe_metrics() -> Metrics {
    let mut metrics = Metrics::default();

    // Torrent metrics

    metrics.metric_collection.describe_counter(
        &metric_name!(TORRENT_REPOSITORY_TORRENTS_ADDED_TOTAL),
        Some(Unit::Count),
        Some(&MetricDescription::new("The total number of torrents added.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(TORRENT_REPOSITORY_TORRENTS_REMOVED_TOTAL),
        Some(Unit::Count),
        Some(&MetricDescription::new("The total number of torrents removed.")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(TORRENT_REPOSITORY_TORRENTS_TOTAL),
        Some(Unit::Count),
        Some(&MetricDescription::new("The total number of torrents.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(TORRENT_REPOSITORY_TORRENTS_DOWNLOADS_TOTAL),
        Some(Unit::Count),
        Some(&MetricDescription::new("The total number of torrent downloads.")),
    );

    // Peers metrics

    metrics.metric_collection.describe_counter(
        &metric_name!(TORRENT_REPOSITORY_PEERS_ADDED_TOTAL),
        Some(Unit::Count),
        Some(&MetricDescription::new("The total number of peers added.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(TORRENT_REPOSITORY_PEERS_REMOVED_TOTAL),
        Some(Unit::Count),
        Some(&MetricDescription::new("The total number of peers removed.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(TORRENT_REPOSITORY_PEERS_UPDATED_TOTAL),
        Some(Unit::Count),
        Some(&MetricDescription::new("The total number of peers updated.")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(TORRENT_REPOSITORY_PEER_CONNECTIONS_TOTAL),
        Some(Unit::Count),
        Some(&MetricDescription::new(
            "The total number of peer connections (one connection per torrent).",
        )),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(TORRENT_REPOSITORY_UNIQUE_PEERS_TOTAL),
        Some(Unit::Count),
        Some(&MetricDescription::new("The total number of unique peers.")),
    );

    metrics
}
