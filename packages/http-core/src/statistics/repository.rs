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

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_name;

    use super::Repository;
    use crate::statistics::HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;

    const NOW: DurationSinceUnixEpoch = DurationSinceUnixEpoch::ZERO;

    fn request_labels(request_kind: &str) -> LabelSet {
        [("server_binding_address_ip_family", "inet"), ("request_kind", request_kind)].into()
    }

    async fn increase_requests(repository: &Repository, labels: &LabelSet) {
        repository
            .increase_counter(&metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), labels, NOW)
            .await
            .expect("increasing the request counter should succeed");
    }

    /// The value of the request sample that has exactly `labels`, or `None` when there is no such sample.
    async fn requests_with_exactly(repository: &Repository, labels: &LabelSet) -> Option<u64> {
        repository
            .read_metrics(|metrics| {
                metrics
                    .metric_collection()
                    .get_counter_value(&metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), labels)
            })
            .await
            .map(u64::from)
    }

    #[rstest]
    #[case::created_with_new(Repository::new())]
    #[case::created_by_default(Repository::default())]
    #[tokio::test]
    async fn it_should_start_with_the_request_counter_described_and_at_zero(#[case] repository: Repository) {
        // Act
        let (is_described, totals) = repository
            .read_metrics(|metrics| {
                (
                    metrics
                        .metric_collection()
                        .contains_counter(&metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL)),
                    (
                        metrics.tcp4_announces_handled(),
                        metrics.tcp4_scrapes_handled(),
                        metrics.tcp6_announces_handled(),
                        metrics.tcp6_scrapes_handled(),
                    ),
                )
            })
            .await;

        // Assert
        assert!(is_described, "the request counter should be described from the start");
        assert_eq!(totals, (0, 0, 0, 0));
    }

    #[tokio::test]
    async fn it_should_add_one_to_the_sample_of_the_given_labels_and_to_no_other() {
        // Arrange
        let repository = Repository::new();
        let announce = request_labels("announce");
        let scrape = request_labels("scrape");

        // Act
        increase_requests(&repository, &announce).await;
        increase_requests(&repository, &announce).await;
        increase_requests(&repository, &scrape).await;

        // Assert
        assert_eq!(requests_with_exactly(&repository, &announce).await, Some(2));
        assert_eq!(requests_with_exactly(&repository, &scrape).await, Some(1));
    }

    #[tokio::test]
    async fn it_should_share_its_store_with_its_clones() {
        // Arrange
        let repository = Repository::new();
        let clone = repository.clone();
        let labels = request_labels("announce");

        // Act
        increase_requests(&clone, &labels).await;
        increase_requests(&repository, &labels).await;

        // Assert: each handle sees the increase made through the other.
        assert_eq!(requests_with_exactly(&repository, &labels).await, Some(2));
        assert_eq!(requests_with_exactly(&clone, &labels).await, Some(2));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn it_should_not_lose_increases_made_concurrently() {
        // Arrange
        let repository = Repository::new();
        let labels = request_labels("announce");

        // Act: eight tasks of twenty-five increases each.
        let tasks: Vec<_> = (0..8)
            .map(|_| {
                let repository = repository.clone();
                let labels = labels.clone();
                tokio::spawn(async move {
                    for _ in 0..25 {
                        increase_requests(&repository, &labels).await;
                    }
                })
            })
            .collect();
        for task in tasks {
            task.await.expect("an increasing task should not panic");
        }

        // Assert
        assert_eq!(requests_with_exactly(&repository, &labels).await, Some(200));
    }
}
