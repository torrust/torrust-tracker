use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard};
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric::MetricName;
use torrust_metrics::metric_collection::Error;

use super::describe_metrics;
use super::metrics::Metrics;

/// A repository for the tracker metrics.
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
        Self {
            stats: Arc::new(RwLock::new(describe_metrics())),
        }
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

#[cfg(test)]
mod tests {
    use torrust_metrics::metric_name;

    use super::*;
    use crate::statistics::UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;

    #[tokio::test]
    async fn it_should_contain_the_described_metrics_when_created() {
        let repo = Repository::new();

        let metric_collection = repo.get_stats().await.metric_collection.clone();

        assert!(
            metric_collection.contains_counter(&metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL)),
            "a new repository should contain the described `{UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL}` counter"
        );
    }
}
