use serde::Serialize;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric::MetricName;
use torrust_tracker_metrics::metric_collection::{Error, MetricCollection};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

/// Metrics collected by the tracker.
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Metrics {
    /// Total number of TCP (HTTP tracker) `announce` requests from IPv4 peers.
    pub tcp4_announces_handled: u64,

    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv4 peers.
    pub tcp4_scrapes_handled: u64,

    /// Total number of TCP (HTTP tracker) `announce` requests from IPv6 peers.
    pub tcp6_announces_handled: u64,

    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv6 peers.
    pub tcp6_scrapes_handled: u64,

    /// A collection of metrics.
    pub metric_collection: MetricCollection,
}

impl Metrics {
    /// # Errors
    ///
    /// Returns an error if the metric does not exist and it cannot be created.
    pub fn increase_counter(
        &mut self,
        metric_name: &MetricName,
        labels: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        self.metric_collection.increment_counter(metric_name, labels, now)
    }

    /// # Errors
    ///
    /// Returns an error if the metric does not exist and it cannot be created.
    pub fn set_gauge(
        &mut self,
        metric_name: &MetricName,
        labels: &LabelSet,
        value: f64,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        self.metric_collection.set_gauge(metric_name, labels, value, now)
    }
}
