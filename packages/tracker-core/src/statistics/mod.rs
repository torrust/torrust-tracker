//! Tracker completed-download counters use the retention terminology defined
//! by ADR [`20260901113500_define_completed_download_metric_retention_names`](../../../../docs/adrs/20260901113500_define_completed_download_metric_retention_names.md).
pub mod event;
pub mod metrics;
pub mod persisted;
pub mod repository;

use metrics::Metrics;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric::description::MetricDescription;
use torrust_metrics::metric_name;
use torrust_metrics::unit::Unit;

// Torrent metrics

/// Deprecated legacy counter. Its value is process-local without persisted
/// completed statistics and historical when that capability is enabled.
pub const TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL: &str = "tracker_core_persistent_torrents_downloads_total";
pub const TRACKER_CORE_IN_SESSION_TORRENTS_DOWNLOADS_TOTAL: &str = "tracker_core_in_session_torrents_downloads_total";
pub const TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL: &str = "tracker_core_persisted_torrents_downloads_total";

#[must_use]
pub fn describe_metrics(tracker_usage_statistics_enabled: bool, persisted_completed_statistics_enabled: bool) -> Metrics {
    let mut metrics = Metrics::default();

    // Torrent metrics

    metrics.metric_collection.describe_counter(
        &metric_name!(TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new(
            "Deprecated: use tracker_core_in_session_torrents_downloads_total or tracker_core_persisted_torrents_downloads_total. This counter is process-local unless persisted completed statistics are enabled.",
        )),
    );
    set_counter_to_zero(&mut metrics, TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL);

    if tracker_usage_statistics_enabled {
        metrics.metric_collection.describe_counter(
            &metric_name!(TRACKER_CORE_IN_SESSION_TORRENTS_DOWNLOADS_TOTAL),
            Some(Unit::Count),
            Some(MetricDescription::new(
                "The number of torrent downloads completed since this tracker process started.",
            )),
        );
        set_counter_to_zero(&mut metrics, TRACKER_CORE_IN_SESSION_TORRENTS_DOWNLOADS_TOTAL);
    }

    if persisted_completed_statistics_enabled {
        metrics.metric_collection.describe_counter(
            &metric_name!(TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL),
            Some(Unit::Count),
            Some(MetricDescription::new(
                "The number of torrent downloads restored from and maintained in persistent storage.",
            )),
        );
        set_counter_to_zero(&mut metrics, TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL);
    }

    metrics
}

fn set_counter_to_zero(metrics: &mut Metrics, metric_name: &str) {
    metrics
        .metric_collection
        .set_counter(
            &metric_name!(metric_name),
            &LabelSet::default(),
            0,
            DurationSinceUnixEpoch::from_secs(0),
        )
        .expect("described counters accept an initial zero value");
}
