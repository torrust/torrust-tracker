use serde::Serialize;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric::MetricName;
use torrust_tracker_metrics::metric_collection::{Error, MetricCollection};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

/// Metrics collected by the torrent repository.
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Metrics {
    /// A collection of metrics.
    pub metric_collection: MetricCollection,
}

impl Metrics {
    /// # Errors
    ///
    /// Returns an error if the metric does not exist and it cannot be created.
    pub fn increment_counter(
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
    pub fn set_counter(
        &mut self,
        metric_name: &MetricName,
        labels: &LabelSet,
        value: u64,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        self.metric_collection.set_counter(metric_name, labels, value, now)
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

    /// # Errors
    ///
    /// Returns an error if the metric does not exist and it cannot be created.
    pub fn increment_gauge(
        &mut self,
        metric_name: &MetricName,
        labels: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        self.metric_collection.increment_gauge(metric_name, labels, now)
    }

    /// # Errors
    ///
    /// Returns an error if the metric does not exist and it cannot be created.
    pub fn decrement_gauge(
        &mut self,
        metric_name: &MetricName,
        labels: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        self.metric_collection.decrement_gauge(metric_name, labels, now)
    }
}
