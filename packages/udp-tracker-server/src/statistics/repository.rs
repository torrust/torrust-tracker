use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{RwLock, RwLockReadGuard};
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric::MetricName;
use torrust_tracker_metrics::metric_collection::Error;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

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

    /// # Errors
    ///
    /// This function will return an error if the metric collection fails to
    /// increase the counter.
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

        result
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub async fn recalculate_udp_avg_connect_processing_time_ns(&self, req_processing_time: Duration) -> f64 {
        let stats_lock = self.stats.write().await;

        let req_processing_time = req_processing_time.as_nanos() as f64;
        let udp_connections_handled = (stats_lock.udp4_connections_handled() + stats_lock.udp6_connections_handled()) as f64;

        let previous_avg = stats_lock.udp_avg_connect_processing_time_ns();

        // Moving average: https://en.wikipedia.org/wiki/Moving_average
        let new_avg = previous_avg as f64 + (req_processing_time - previous_avg as f64) / udp_connections_handled;

        drop(stats_lock);

        tracing::debug!(
            "Recalculated UDP average connect processing time: {} ns (previous: {} ns, req_processing_time: {} ns, udp_connections_handled: {})",
            new_avg,
            previous_avg,
            req_processing_time,
            udp_connections_handled
        );

        new_avg
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub async fn recalculate_udp_avg_announce_processing_time_ns(&self, req_processing_time: Duration) -> f64 {
        let stats_lock = self.stats.write().await;

        let req_processing_time = req_processing_time.as_nanos() as f64;

        let udp_announces_handled = (stats_lock.udp4_announces_handled() + stats_lock.udp6_announces_handled()) as f64;

        let previous_avg = stats_lock.udp_avg_announce_processing_time_ns();

        // Moving average: https://en.wikipedia.org/wiki/Moving_average
        let new_avg = previous_avg as f64 + (req_processing_time - previous_avg as f64) / udp_announces_handled;

        drop(stats_lock);

        tracing::debug!(
            "Recalculated UDP average announce processing time: {} ns (previous: {} ns, req_processing_time: {} ns, udp_announces_handled: {})",
            new_avg,
            previous_avg,
            req_processing_time,
            udp_announces_handled
        );

        new_avg
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub async fn recalculate_udp_avg_scrape_processing_time_ns(&self, req_processing_time: Duration) -> f64 {
        let stats_lock = self.stats.write().await;

        let req_processing_time = req_processing_time.as_nanos() as f64;
        let udp_scrapes_handled = (stats_lock.udp4_scrapes_handled() + stats_lock.udp6_scrapes_handled()) as f64;

        let previous_avg = stats_lock.udp_avg_scrape_processing_time_ns();

        // Moving average: https://en.wikipedia.org/wiki/Moving_average
        let new_avg = previous_avg as f64 + (req_processing_time - previous_avg as f64) / udp_scrapes_handled;

        drop(stats_lock);

        tracing::debug!(
            "Recalculated UDP average scrape processing time: {} ns (previous: {} ns, req_processing_time: {} ns, udp_scrapes_handled: {})",
            new_avg,
            previous_avg,
            req_processing_time,
            udp_scrapes_handled
        );

        new_avg
    }
}
