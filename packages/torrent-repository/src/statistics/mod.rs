pub mod event;
pub mod metrics;
pub mod repository;

use metrics::Metrics;
use torrust_tracker_metrics::metric::description::MetricDescription;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_metrics::unit::Unit;

const TORRENT_REPOSITORY_TORRENTS_TOTAL: &str = "torrent_repository_torrents_total";
const TORRENT_REPOSITORY_TORRENTS_DOWNLOADS_TOTAL: &str = "torrent_repository_torrents_downloads_total";

#[must_use]
pub fn describe_metrics() -> Metrics {
    let mut metrics = Metrics::default();

    metrics.metric_collection.describe_gauge(
        &metric_name!(TORRENT_REPOSITORY_TORRENTS_TOTAL),
        Some(Unit::Count),
        Some(&MetricDescription::new("The total number of torrents.")),
    );

    metrics.metric_collection.describe_counter(
        &metric_name!(TORRENT_REPOSITORY_TORRENTS_DOWNLOADS_TOTAL),
        Some(Unit::Count),
        Some(&MetricDescription::new(
            "The total number of torrent downloads since the tracker process started.",
        )),
    );

    metrics
}
