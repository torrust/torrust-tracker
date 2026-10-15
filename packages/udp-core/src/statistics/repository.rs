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
/// use torrust_tracker_udp_core::statistics::repository::Repository;
///
/// fn read_stats(repository: &Repository) {
///     let _stats = &repository.stats;
/// }
/// ```
///
/// Read through the accessor instead:
///
/// ```rust
/// use torrust_tracker_udp_core::statistics::repository::Repository;
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
    use rstest::rstest;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_metrics::counter::Counter;
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_collection::Error;
    use torrust_metrics::metric_name;

    use super::Repository;
    use crate::statistics::UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;

    fn announce_labels() -> LabelSet {
        [("server_binding_address_ip_family", "inet"), ("request_kind", "announce")].into()
    }

    async fn record_one_announce(repository: &Repository) {
        repository
            .increase_counter(
                &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
                &announce_labels(),
                DurationSinceUnixEpoch::ZERO,
            )
            .await
            .unwrap();
    }

    #[rstest]
    #[case::new(Repository::new)]
    #[case::default(Repository::default)]
    #[tokio::test]
    async fn it_should_describe_the_requests_received_counter_when_created(#[case] construct: fn() -> Repository) {
        // Act
        let repository = construct();

        // Assert
        let described = repository
            .get_stats()
            .await
            .metric_collection
            .contains_counter(&metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL));
        assert!(
            described,
            "a new repository should contain the described `{UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL}` counter"
        );
    }

    #[tokio::test]
    async fn it_should_make_an_increased_counter_visible_through_the_repository_stats() {
        // Arrange
        let repository = Repository::new();

        // Act
        record_one_announce(&repository).await;

        // Assert
        let count = repository
            .get_stats()
            .await
            .metric_collection
            .get_counter_value(&metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), &announce_labels());
        assert_eq!(count, Some(Counter::from(1)));
    }

    #[tokio::test]
    async fn it_should_return_the_metric_collection_error_when_a_counter_cannot_be_increased() {
        // Arrange
        let repository = Repository::new();
        repository
            .stats
            .write()
            .await
            .metric_collection
            .set_gauge(
                &metric_name!("already_a_gauge"),
                &LabelSet::empty(),
                1.0,
                DurationSinceUnixEpoch::ZERO,
            )
            .unwrap();

        // Act
        let result = repository
            .increase_counter(
                &metric_name!("already_a_gauge"),
                &LabelSet::empty(),
                DurationSinceUnixEpoch::ZERO,
            )
            .await;

        // Assert
        assert!(matches!(result, Err(Error::MetricNameCollisionAdding { .. })));
    }
}
