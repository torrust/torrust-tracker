use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard};
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric::MetricName;
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

    pub async fn increase_udp4_connections(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp4_connections_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_udp4_announces(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp4_announces_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_udp4_scrapes(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp4_scrapes_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_udp6_connections(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp6_connections_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_udp6_announces(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp6_announces_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_udp6_scrapes(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp6_scrapes_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_counter(&self, metric_name: &MetricName, labels: &LabelSet, now: DurationSinceUnixEpoch) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.increase_counter(metric_name, labels, now);
        drop(stats_lock);
    }
}
