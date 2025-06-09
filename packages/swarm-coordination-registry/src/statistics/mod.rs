pub mod activity_metrics_updater;
pub mod event;
pub mod metrics;
pub mod repository;

use metrics::Metrics;
use torrust_tracker_metrics::metric::description::MetricDescription;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_metrics::unit::Unit;

// Torrent metrics

const SWARM_COORDINATION_REGISTRY_TORRENTS_ADDED_TOTAL: &str = "swarm_coordination_registry_torrents_added_total";
const SWARM_COORDINATION_REGISTRY_TORRENTS_REMOVED_TOTAL: &str = "swarm_coordination_registry_torrents_removed_total";

const SWARM_COORDINATION_REGISTRY_TORRENTS_TOTAL: &str = "swarm_coordination_registry_torrents_total";
const SWARM_COORDINATION_REGISTRY_TORRENTS_DOWNLOADS_TOTAL: &str = "swarm_coordination_registry_torrents_downloads_total";
const SWARM_COORDINATION_REGISTRY_TORRENTS_INACTIVE_TOTAL: &str = "swarm_coordination_registry_torrents_inactive_total";

// Peers metrics

const SWARM_COORDINATION_REGISTRY_PEERS_ADDED_TOTAL: &str = "swarm_coordination_registry_peers_added_total";
const SWARM_COORDINATION_REGISTRY_PEERS_REMOVED_TOTAL: &str = "swarm_coordination_registry_peers_removed_total";
const SWARM_COORDINATION_REGISTRY_PEERS_UPDATED_TOTAL: &str = "swarm_coordination_registry_peers_updated_total";

const SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL: &str = "swarm_coordination_registry_peer_connections_total";
const SWARM_COORDINATION_REGISTRY_UNIQUE_PEERS_TOTAL: &str = "swarm_coordination_registry_unique_peers_total"; // todo: not implemented yet
const SWARM_COORDINATION_REGISTRY_PEERS_INACTIVE_TOTAL: &str = "swarm_coordination_registry_peers_inactive_total";
const SWARM_COORDINATION_REGISTRY_PEERS_COMPLETED_STATE_REVERTED_TOTAL: &str =
    "swarm_coordination_registry_peers_completed_state_reverted_total";

#[must_use]
pub fn describe_metrics() -> Metrics {
    let mut metrics = Metrics::default();

    // Torrent metrics

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_ADDED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of torrents added.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_REMOVED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of torrents removed.")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of torrents.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_DOWNLOADS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of torrent downloads.")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_INACTIVE_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of inactive torrents.")),
    );

    // Peers metrics

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_ADDED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of peers added.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_REMOVED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of peers removed.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_UPDATED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of peers updated.")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new(
            "The total number of peer connections (one connection per torrent).",
        )),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(SWARM_COORDINATION_REGISTRY_UNIQUE_PEERS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of unique peers.")),
    );

    metrics.metric_collection.describe_gauge(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_INACTIVE_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of inactive peers.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_COMPLETED_STATE_REVERTED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new(
            "The total number of peers whose completed state was reverted.",
        )),
    );

    metrics
}
