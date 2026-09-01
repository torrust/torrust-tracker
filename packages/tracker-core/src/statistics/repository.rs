use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard};
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric::MetricName;
use torrust_metrics::metric_collection::Error;
use torrust_metrics::metric_name;

use super::metrics::Metrics;
use super::{
    TRACKER_CORE_IN_SESSION_TORRENTS_DOWNLOADS_TOTAL, TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL,
    TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL, describe_metrics,
};

/// A repository for the torrent repository metrics.
#[derive(Clone)]
pub struct Repository {
    pub stats: Arc<RwLock<Metrics>>,
}

impl Default for Repository {
    fn default() -> Self {
        Self::new(true, false)
    }
}

impl Repository {
    #[must_use]
    pub fn new(tracker_usage_statistics_enabled: bool, persisted_completed_statistics_enabled: bool) -> Self {
        let stats = Arc::new(RwLock::new(describe_metrics(
            tracker_usage_statistics_enabled,
            persisted_completed_statistics_enabled,
        )));

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
    /// increment the counter.
    pub async fn set_counter(
        &self,
        metric_name: &MetricName,
        labels: &LabelSet,
        value: u64,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        let mut stats_lock = self.stats.write().await;

        let result = stats_lock.set_counter(metric_name, labels, value, now);

        drop(stats_lock);

        match result {
            Ok(()) => {}
            Err(ref err) => tracing::error!("Failed to set the counter: {}", err),
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

    /// Gets the deprecated, conditionally retained total number of torrent downloads.
    pub async fn get_torrents_downloads_total(&self) -> u64 {
        self.get_counter_value(TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL).await
    }

    /// Gets completed downloads observed by this tracker process.
    pub async fn get_torrents_downloads_in_session_total(&self) -> u64 {
        self.get_counter_value(TRACKER_CORE_IN_SESSION_TORRENTS_DOWNLOADS_TOTAL).await
    }

    /// Gets completed downloads restored from and maintained in persistent storage.
    pub async fn get_torrents_downloads_persisted_total(&self) -> u64 {
        self.get_counter_value(TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL).await
    }

    async fn get_counter_value(&self, metric_name: &str) -> u64 {
        let metrics = self.get_metrics().await;

        let downloads = metrics
            .metric_collection
            .get_counter_value(&metric_name!(metric_name), &LabelSet::default());

        if let Some(downloads) = downloads {
            downloads.value()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use torrust_metrics::metric_name;

    use super::Repository;
    use crate::statistics::{
        TRACKER_CORE_IN_SESSION_TORRENTS_DOWNLOADS_TOTAL, TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL,
        TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL,
    };

    #[tokio::test]
    async fn it_should_omit_persisted_metric_when_persisted_completed_statistics_are_disabled() {
        // Arrange
        let repository = Repository::new(true, false);

        // Act
        let metrics = repository.get_metrics().await;

        // Assert
        assert!(
            metrics
                .metric_collection
                .contains_counter(&metric_name!(TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL))
        );
        assert!(
            metrics
                .metric_collection
                .contains_counter(&metric_name!(TRACKER_CORE_IN_SESSION_TORRENTS_DOWNLOADS_TOTAL))
        );
        assert!(
            !metrics
                .metric_collection
                .contains_counter(&metric_name!(TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL))
        );
    }

    #[tokio::test]
    async fn it_should_export_persisted_metric_with_zero_value_when_persisted_completed_statistics_are_enabled() {
        // Arrange
        let repository = Repository::new(true, true);

        // Act
        let metrics = repository.get_metrics().await;

        // Assert
        assert!(
            metrics
                .metric_collection
                .contains_counter(&metric_name!(TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL))
        );
        assert_eq!(repository.get_torrents_downloads_persisted_total().await, 0);
    }
}
