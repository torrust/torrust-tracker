//! Tracker completed-download counters use explicit retention terminology.
//!
//! The terminology is defined by ADR
//! [`20260901113500_define_completed_download_metric_retention_names`](../../../../docs/adrs/20260901113500_define_completed_download_metric_retention_names.md).
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

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_name;

    use super::metrics::Metrics;
    use super::{
        TRACKER_CORE_IN_SESSION_TORRENTS_DOWNLOADS_TOTAL, TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL,
        TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL, describe_metrics,
    };

    /// The value of an unlabeled counter, or `None` when it is not registered.
    fn counter(metrics: &Metrics, name: &str) -> Option<u64> {
        metrics
            .metric_collection()
            .get_counter_value(&metric_name!(name), &LabelSet::default())
            .map(|counter| counter.value())
    }

    #[rstest]
    #[case::both_disabled(false, false)]
    #[case::only_persisted_statistics(false, true)]
    #[case::only_usage_statistics(true, false)]
    #[case::both_enabled(true, true)]
    fn it_should_register_each_downloads_total_only_when_its_statistics_are_enabled(
        #[case] usage: bool,
        #[case] persisted: bool,
    ) {
        // Act
        let metrics = describe_metrics(usage, persisted);

        // Assert
        assert_eq!(
            (
                counter(&metrics, TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL),
                counter(&metrics, TRACKER_CORE_IN_SESSION_TORRENTS_DOWNLOADS_TOTAL),
                counter(&metrics, TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL)
            ),
            (Some(0), usage.then_some(0), persisted.then_some(0)),
            "(persistent, in-session, persisted) totals with usage statistics {usage} and persisted statistics {persisted}"
        );
    }
}
