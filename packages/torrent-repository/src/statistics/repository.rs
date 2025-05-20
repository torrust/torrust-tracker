use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard};
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric::MetricName;
use torrust_tracker_metrics::metric_collection::Error;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::describe_metrics;
use super::metrics::Metrics;

/// A repository for the torrent repository metrics.
#[derive(Clone)]
pub struct Repository {
    pub stats: Arc<RwLock<Metrics>>,
}

impl Default for Repository {
    fn default() -> Self {
        Self::new()
    }
}

impl Repository {
    #[must_use]
    pub fn new() -> Self {
        let stats = Arc::new(RwLock::new(describe_metrics()));

        Self { stats }
    }

    pub async fn get_metrics(&self) -> RwLockReadGuard<'_, Metrics> {
        self.stats.read().await
    }

    /// # Errors
    ///
    /// This function will return an error if the metric collection fails to
    /// increment the counter.
    pub async fn increment_counter(
        &self,
        metric_name: &MetricName,
        labels: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        let mut stats_lock = self.stats.write().await;

        let result = stats_lock.increment_counter(metric_name, labels, now);

        drop(stats_lock);

        match result {
            Ok(()) => {}
            Err(ref err) => tracing::error!("Failed to increment the counter: {}", err),
        }

        result
    }

    /// # Errors
    ///
    /// This function will return an error if the metric collection fails to
    /// set the gauge.
    pub async fn set_gauge(
        &self,
        metric_name: &MetricName,
        labels: &LabelSet,
        value: f64,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        let mut stats_lock = self.stats.write().await;

        let result = stats_lock.set_gauge(metric_name, labels, value, now);

        drop(stats_lock);

        match result {
            Ok(()) => {}
            Err(ref err) => tracing::error!("Failed to set the gauge: {}", err),
        }

        result
    }

    /// # Errors
    ///
    /// This function will return an error if the metric collection fails to
    /// increment the gauge.
    pub async fn increment_gauge(
        &self,
        metric_name: &MetricName,
        labels: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        let mut stats_lock = self.stats.write().await;

        let result = stats_lock.increment_gauge(metric_name, labels, now);

        drop(stats_lock);

        match result {
            Ok(()) => {}
            Err(ref err) => tracing::error!("Failed to increment the gauge: {}", err),
        }

        result
    }

    /// # Errors
    ///
    /// This function will return an error if the metric collection fails to
    /// decrement the gauge.
    pub async fn decrement_gauge(
        &self,
        metric_name: &MetricName,
        labels: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        let mut stats_lock = self.stats.write().await;

        let result = stats_lock.decrement_gauge(metric_name, labels, now);

        drop(stats_lock);

        match result {
            Ok(()) => {}
            Err(ref err) => tracing::error!("Failed to decrement the gauge: {}", err),
        }

        result
    }
}
