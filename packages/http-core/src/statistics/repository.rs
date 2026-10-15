use std::sync::Arc;

use tokio::sync::RwLock;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric::MetricName;
use torrust_metrics::metric_collection::Error;

use super::describe_metrics;
use super::metrics::Metrics;

/// A repository for the tracker metrics.
///
/// External callers read statistics through [`Repository::read_metrics`].
///
/// The metrics lock and the metric collection field are private:
///
/// ```rust,compile_fail,E0616
/// use torrust_tracker_http_core::statistics::repository::Repository;
///
/// fn read_stats(repository: &Repository) {
///     let _stats = &repository.stats;
/// }
/// ```
///
/// ```rust,compile_fail,E0616
/// use torrust_tracker_http_core::statistics::repository::Repository;
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
/// use torrust_tracker_http_core::statistics::repository::Repository;
///
/// async fn read_stats(repository: &Repository) {
///     let _tcp4_announces = repository.read_metrics(|metrics| metrics.tcp4_announces_handled()).await;
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
    /// increase the counter.
    pub async fn increase_counter(
        &self,
        metric_name: &MetricName,
        labels: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        let mut stats_lock = self.stats.write().await;

        let result = stats_lock.increase_counter(metric_name, labels, now);

        drop(stats_lock);

        result
    }
}
