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
/// use torrust_tracker_udp_core::statistics::repository::Repository;
///
/// fn read_stats(repository: &Repository) {
///     let _stats = &repository.stats;
/// }
/// ```
///
/// ```rust,compile_fail,E0616
/// use torrust_tracker_udp_core::statistics::repository::Repository;
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
/// use torrust_tracker_udp_core::statistics::repository::Repository;
///
/// async fn read_stats(repository: &Repository) {
///     let _udp4_connections = repository.read_metrics(|metrics| metrics.udp4_connections_handled()).await;
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
        Self {
            stats: Arc::new(RwLock::new(describe_metrics())),
        }
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

    #[rstest]
    #[case::new(Repository::new)]
    #[case::default(Repository::default)]
    #[tokio::test]
    async fn it_should_describe_the_requests_received_counter_when_created(#[case] construct: fn() -> Repository) {
        // Act
        let repository = construct();

        // Assert
        let described = repository
            .read_metrics(|metrics| {
                metrics
                    .metric_collection()
                    .contains_counter(&metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL))
            })
            .await;
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
        repository
            .increase_counter(
                &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
                &announce_labels(),
                DurationSinceUnixEpoch::ZERO,
            )
            .await
            .unwrap();

        // Assert
        let count = repository
            .read_metrics(|metrics| {
                metrics
                    .metric_collection()
                    .get_counter_value(&metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), &announce_labels())
            })
            .await;
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
