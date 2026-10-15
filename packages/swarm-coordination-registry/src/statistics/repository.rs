use std::sync::Arc;

use tokio::sync::RwLock;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric::MetricName;
use torrust_metrics::metric_collection::Error;

use super::describe_metrics;
use super::metrics::Metrics;

/// A repository for the torrent repository metrics.
///
/// External callers read statistics through [`Repository::read_metrics`].
///
/// The metrics lock and the metric collection field are private:
///
/// ```rust,compile_fail,E0616
/// use torrust_tracker_swarm_coordination_registry::statistics::repository::Repository;
///
/// fn read_metrics(repository: &Repository) {
///     let _metrics = &repository.stats;
/// }
/// ```
///
/// ```rust,compile_fail,E0616
/// use torrust_tracker_swarm_coordination_registry::statistics::repository::Repository;
///
/// async fn read_collection(repository: &Repository) {
///     repository
///         .read_metrics(|metrics| {
///             let _collection = &metrics.metric_collection;
///         })
///         .await;
/// }
/// ```
///
/// Read through the closure and the collection accessor instead:
///
/// ```rust
/// use torrust_tracker_swarm_coordination_registry::statistics::repository::Repository;
///
/// async fn read_metrics(repository: &Repository) {
///     let _collection_is_readable = repository
///         .read_metrics(|metrics| {
///             let _collection = metrics.metric_collection();
///         })
///         .await;
/// }
/// ```
#[derive(Clone)]
pub struct Repository {
    stats: Arc<RwLock<Metrics>>,
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

    /// Runs `read` on the current metrics while holding the read lock.
    ///
    /// Writers wait while `read` runs, so keep it short: it must not block the thread
    /// (blocking I/O, sleeping, or long computation) or call `read_metrics` on this repository.
    pub async fn read_metrics<R>(&self, read: impl FnOnce(&Metrics) -> R) -> R {
        let metrics = self.stats.read().await;
        read(&metrics)
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
