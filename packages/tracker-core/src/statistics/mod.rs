pub mod event;
pub mod metrics;
pub mod persisted;
pub mod repository;

use metrics::Metrics;
use torrust_tracker_metrics::metric::description::MetricDescription;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_metrics::unit::Unit;

// Torrent metrics

const TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL: &str = "tracker_core_persistent_torrents_downloads_total";

#[must_use]
pub fn describe_metrics() -> Metrics {
    let mut metrics = Metrics::default();

    // Torrent metrics

    metrics.metric_collection.describe_counter(
        &metric_name!(TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("The total number of torrent downloads (persisted).")),
    );

    metrics
}
