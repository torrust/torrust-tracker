use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard};
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric::MetricName;
use torrust_metrics::metric_collection::Error;

use super::describe_metrics;
use super::metrics::Metrics;

/// A repository for the tracker metrics.
///
/// External callers read statistics through [`Repository::get_stats`].
///
/// The metrics lock is private:
///
/// ```rust,compile_fail,E0616
/// use torrust_tracker_http_core::statistics::repository::Repository;
///
/// fn read_stats(repository: &Repository) {
///     let _stats = &repository.stats;
/// }
/// ```
///
/// Read through the accessor instead:
///
/// ```rust
/// use torrust_tracker_http_core::statistics::repository::Repository;
///
/// async fn read_stats(repository: &Repository) {
///     let _stats = repository.get_stats().await;
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

    pub async fn get_stats(&self) -> RwLockReadGuard<'_, Metrics> {
        self.stats.read().await
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
