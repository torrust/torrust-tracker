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

    pub async fn recalculate_udp_avg_connect_processing_time_ns(
        &self,
        req_processing_time: Duration,
        label_set: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> f64 {
        let mut stats_lock = self.stats.write().await;

        let new_avg = stats_lock.recalculate_udp_avg_connect_processing_time_ns(req_processing_time, label_set, now);

        drop(stats_lock);

        new_avg
    }

    pub async fn recalculate_udp_avg_announce_processing_time_ns(
        &self,
        req_processing_time: Duration,
        label_set: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> f64 {
        let mut stats_lock = self.stats.write().await;

        let new_avg = stats_lock.recalculate_udp_avg_announce_processing_time_ns(req_processing_time, label_set, now);

        drop(stats_lock);

        new_avg
    }

    pub async fn recalculate_udp_avg_scrape_processing_time_ns(&self, req_processing_time: Duration) -> f64 {
        let stats_lock = self.stats.write().await;

        let new_avg = stats_lock.recalculate_udp_avg_scrape_processing_time_ns(req_processing_time);

        drop(stats_lock);

        new_avg
    }
}

#[cfg(test)]
mod tests {
    use core::f64;
    use std::time::Duration;

    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_metrics::metric_name;

    use super::*;
    use crate::statistics::*;
    use crate::CurrentClock;

    #[test]
    fn it_should_implement_default() {
        let repo = Repository::default();
        assert!(!std::ptr::eq(&repo.stats, &Repository::new().stats));
    }

    #[test]
    fn it_should_be_cloneable() {
        let repo = Repository::new();
        let cloned_repo = repo.clone();
        assert!(!std::ptr::eq(&repo.stats, &cloned_repo.stats));
    }

    #[tokio::test]
    async fn it_should_be_initialized_with_described_metrics() {
        let repo = Repository::new();
        let stats = repo.get_stats().await;

        // Check that the described metrics are present
        assert!(stats
            .metric_collection
            .contains_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL)));
        assert!(stats
            .metric_collection
            .contains_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL)));
        assert!(stats
            .metric_collection
            .contains_gauge(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL)));
        assert!(stats
            .metric_collection
            .contains_counter(&metric_name!(UDP_TRACKER_SERVER_CONNECTION_ID_ERRORS_TOTAL)));
        assert!(stats
            .metric_collection
            .contains_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL)));
        assert!(stats
            .metric_collection
            .contains_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL)));
        assert!(stats
            .metric_collection
            .contains_counter(&metric_name!(UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL)));
        assert!(stats
            .metric_collection
            .contains_counter(&metric_name!(UDP_TRACKER_SERVER_ERRORS_TOTAL)));
        assert!(stats
            .metric_collection
            .contains_gauge(&metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS)));
    }

    #[tokio::test]
    async fn it_should_return_a_read_guard_to_metrics() {
        let repo = Repository::new();
        let stats = repo.get_stats().await;

        // Should be able to read metrics through the guard
        assert_eq!(stats.udp_requests_aborted(), 0);
        assert_eq!(stats.udp_requests_banned(), 0);
    }

    #[tokio::test]
    async fn it_should_allow_increasing_a_counter_metric_successfully() {
        let repo = Repository::new();
        let now = CurrentClock::now();
        let labels = LabelSet::empty();

        // Increase a counter metric
        let result = repo
            .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL), &labels, now)
            .await;

        assert!(result.is_ok());

        // Verify the counter was incremented
        let stats = repo.get_stats().await;
        assert_eq!(stats.udp_requests_aborted(), 1);
    }

    #[tokio::test]
    async fn it_should_allow_increasing_a_counter_multiple_times() {
        let repo = Repository::new();
        let now = CurrentClock::now();
        let labels = LabelSet::empty();

        // Increase counter multiple times
        for _ in 0..5 {
            repo.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL), &labels, now)
                .await
                .unwrap();
        }

        // Verify the counter was incremented correctly
        let stats = repo.get_stats().await;
        assert_eq!(stats.udp_requests_aborted(), 5);
    }

    #[tokio::test]
    async fn it_should_allow_increasing_a_counter_with_different_labels() {
        let repo = Repository::new();
        let now = CurrentClock::now();

        let labels_ipv4 = LabelSet::from([("server_binding_address_ip_family", "inet")]);
        let labels_ipv6 = LabelSet::from([("server_binding_address_ip_family", "inet6")]);

        // Increase counters with different labels
        repo.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL), &labels_ipv4, now)
            .await
            .unwrap();

        repo.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL), &labels_ipv6, now)
            .await
            .unwrap();

        // Verify both labeled metrics
        let stats = repo.get_stats().await;
        assert_eq!(stats.udp4_requests(), 1);
        assert_eq!(stats.udp6_requests(), 1);
    }

    #[tokio::test]
    async fn it_should_set_a_gauge_metric_successfully() {
        let repo = Repository::new();
        let now = CurrentClock::now();
        let labels = LabelSet::empty();

        // Set a gauge metric
        let result = repo
            .set_gauge(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL), &labels, 42.0, now)
            .await;

        assert!(result.is_ok());

        // Verify the gauge was set
        let stats = repo.get_stats().await;
        assert_eq!(stats.udp_banned_ips_total(), 42);
    }

    #[tokio::test]
    async fn it_should_overwrite_previous_value_when_setting_a_gauge_with_a_previous_value() {
        let repo = Repository::new();
        let now = CurrentClock::now();
        let labels = LabelSet::empty();

        // Set gauge to initial value
        repo.set_gauge(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL), &labels, 10.0, now)
            .await
            .unwrap();

        // Overwrite with new value
        repo.set_gauge(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL), &labels, 25.0, now)
            .await
            .unwrap();

        // Verify the gauge has the new value
        let stats = repo.get_stats().await;
        assert_eq!(stats.udp_banned_ips_total(), 25);
    }

    #[tokio::test]
    async fn it_should_allow_setting_a_gauge_with_different_labels() {
        let repo = Repository::new();
        let now = CurrentClock::now();

        let labels_connect = LabelSet::from([("request_kind", "connect")]);
        let labels_announce = LabelSet::from([("request_kind", "announce")]);

        // Set gauges with different labels
        repo.set_gauge(
            &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
            &labels_connect,
            1000.0,
            now,
        )
        .await
        .unwrap();

        repo.set_gauge(
            &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
            &labels_announce,
            2000.0,
            now,
        )
        .await
        .unwrap();

        // Verify both labeled metrics
        let stats = repo.get_stats().await;
        assert_eq!(stats.udp_avg_connect_processing_time_ns(), 1000);
        assert_eq!(stats.udp_avg_announce_processing_time_ns(), 2000);
    }

    #[tokio::test]
    async fn it_should_recalculate_the_udp_average_connect_processing_time_in_nanoseconds_using_moving_average() {
        let repo = Repository::new();
        let now = CurrentClock::now();

        // Set up initial connections handled
        let ipv4_labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "connect")]);
        let ipv6_labels = LabelSet::from([("server_binding_address_ip_family", "inet6"), ("request_kind", "connect")]);

        // Simulate 2 IPv4 and 1 IPv6 connections
        repo.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &ipv4_labels, now)
            .await
            .unwrap();
        repo.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &ipv4_labels, now)
            .await
            .unwrap();
        repo.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &ipv6_labels, now)
            .await
            .unwrap();

        // Set initial average to 1000ns
        let connect_labels = LabelSet::from([("request_kind", "connect")]);
        repo.set_gauge(
            &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
            &connect_labels,
            1000.0,
            now,
        )
        .await
        .unwrap();

        // Calculate new average with processing time of 2000ns
        let processing_time = Duration::from_nanos(2000);
        let new_avg = repo
            .recalculate_udp_avg_connect_processing_time_ns(processing_time, &connect_labels, now)
            .await;

        // Moving average: previous_avg + (new_value - previous_avg) / total_connections
        // 1000 + (2000 - 1000) / 3 = 1000 + 333.33 = 1333.33
        let expected_avg = 1000.0 + (2000.0 - 1000.0) / 3.0;
        assert!(
            (new_avg - expected_avg).abs() < 0.01,
            "Expected {expected_avg}, got {new_avg}"
        );
    }

    #[tokio::test]
    async fn it_should_recalculate_the_udp_average_announce_processing_time_in_nanoseconds_using_moving_average() {
        let repo = Repository::new();
        let now = CurrentClock::now();

        // Set up initial announces handled
        let ipv4_labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "announce")]);
        let ipv6_labels = LabelSet::from([("server_binding_address_ip_family", "inet6"), ("request_kind", "announce")]);

        // Simulate 3 IPv4 and 2 IPv6 announces
        for _ in 0..3 {
            repo.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &ipv4_labels, now)
                .await
                .unwrap();
        }
        for _ in 0..2 {
            repo.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &ipv6_labels, now)
                .await
                .unwrap();
        }

        // Set initial average to 500ns
        let announce_labels = LabelSet::from([("request_kind", "announce")]);
        repo.set_gauge(
            &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
            &announce_labels,
            500.0,
            now,
        )
        .await
        .unwrap();

        // Calculate new average with processing time of 1500ns
        let processing_time = Duration::from_nanos(1500);
        let new_avg = repo
            .recalculate_udp_avg_announce_processing_time_ns(processing_time, &announce_labels, now)
            .await;

        // Moving average: previous_avg + (new_value - previous_avg) / total_announces
        // 500 + (1500 - 500) / 5 = 500 + 200 = 700
        let expected_avg = 500.0 + (1500.0 - 500.0) / 5.0;
        assert!(
            (new_avg - expected_avg).abs() < 0.01,
            "Expected {expected_avg}, got {new_avg}"
        );
    }

    #[tokio::test]
    async fn it_should_recalculate_the_udp_average_scrape_processing_time_in_nanoseconds_using_moving_average() {
        let repo = Repository::new();
        let now = CurrentClock::now();

        // Set up initial scrapes handled
        let ipv4_labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "scrape")]);

        // Simulate 4 IPv4 scrapes
        for _ in 0..4 {
            repo.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &ipv4_labels, now)
                .await
                .unwrap();
        }

        // Set initial average to 800ns
        let scrape_labels = LabelSet::from([("request_kind", "scrape")]);
        repo.set_gauge(
            &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
            &scrape_labels,
            800.0,
            now,
        )
        .await
        .unwrap();

        // Calculate new average with processing time of 1200ns
        let processing_time = Duration::from_nanos(1200);
        let new_avg = repo.recalculate_udp_avg_scrape_processing_time_ns(processing_time).await;

        // Moving average: previous_avg + (new_value - previous_avg) / total_scrapes
        // 800 + (1200 - 800) / 4 = 800 + 100 = 900
        let expected_avg = 800.0 + (1200.0 - 800.0) / 4.0;
        assert!(
            (new_avg - expected_avg).abs() < 0.01,
            "Expected {expected_avg}, got {new_avg}"
        );
    }

    #[tokio::test]
    async fn recalculate_average_methods_should_handle_zero_connections_gracefully() {
        let repo = Repository::new();
        let now = CurrentClock::now();

        // Test with zero connections (should not panic, should handle division by zero)
        let processing_time = Duration::from_nanos(1000);

        let connect_labels = LabelSet::from([("request_kind", "connect")]);
        let connect_avg = repo
            .recalculate_udp_avg_connect_processing_time_ns(processing_time, &connect_labels, now)
            .await;

        let announce_labels = LabelSet::from([("request_kind", "announce")]);
        let announce_avg = repo
            .recalculate_udp_avg_announce_processing_time_ns(processing_time, &announce_labels, now)
            .await;

        let _scrape_labels = LabelSet::from([("request_kind", "scrape")]);
        let scrape_avg = repo.recalculate_udp_avg_scrape_processing_time_ns(processing_time).await;

        // With 0 total connections, the formula becomes 0 + (1000 - 0) / 0
        // This should handle the division by zero case gracefully
        assert!((connect_avg - 1000.0).abs() < f64::EPSILON);
        assert!((announce_avg - 1000.0).abs() < f64::EPSILON);
        assert!(scrape_avg.is_infinite() || scrape_avg.is_nan());
    }

    #[tokio::test]
    async fn it_should_handle_concurrent_access() {
        let repo = Repository::new();
        let now = CurrentClock::now();

        // Spawn multiple concurrent tasks
        let mut handles = vec![];

        for i in 0..10 {
            let repo_clone = repo.clone();
            let handle = tokio::spawn(async move {
                for _ in 0..5 {
                    repo_clone
                        .increase_counter(
                            &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL),
                            &LabelSet::empty(),
                            now,
                        )
                        .await
                        .unwrap();
                }
                i
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }

        // Verify all increments were properly recorded
        let stats = repo.get_stats().await;
        assert_eq!(stats.udp_requests_aborted(), 50); // 10 tasks * 5 increments each
    }

    #[tokio::test]
    async fn it_should_handle_large_processing_times() {
        let repo = Repository::new();
        let now = CurrentClock::now();

        // Set up a connection
        let ipv4_labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "connect")]);
        repo.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &ipv4_labels, now)
            .await
            .unwrap();

        // Test with very large processing time
        let large_duration = Duration::from_secs(1); // 1 second = 1,000,000,000 ns
        let connect_labels = LabelSet::from([("request_kind", "connect")]);
        let new_avg = repo
            .recalculate_udp_avg_connect_processing_time_ns(large_duration, &connect_labels, now)
            .await;

        // Should handle large numbers without overflow
        assert!(new_avg > 0.0);
        assert!(new_avg.is_finite());
    }

    #[tokio::test]
    async fn it_should_maintain_consistency_across_operations() {
        let repo = Repository::new();
        let now = CurrentClock::now();

        // Perform a series of operations
        repo.increase_counter(
            &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL),
            &LabelSet::empty(),
            now,
        )
        .await
        .unwrap();

        repo.set_gauge(
            &metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL),
            &LabelSet::empty(),
            10.0,
            now,
        )
        .await
        .unwrap();

        repo.increase_counter(
            &metric_name!(UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL),
            &LabelSet::empty(),
            now,
        )
        .await
        .unwrap();

        // Check final state
        let stats = repo.get_stats().await;
        assert_eq!(stats.udp_requests_aborted(), 1);
        assert_eq!(stats.udp_banned_ips_total(), 10);
        assert_eq!(stats.udp_requests_banned(), 1);
    }

    #[tokio::test]
    async fn it_should_handle_error_cases_gracefully() {
        let repo = Repository::new();
        let now = CurrentClock::now();

        // Test with invalid metric name (this should still work as metrics are created dynamically)
        let result = repo
            .increase_counter(&metric_name!("non_existent_metric"), &LabelSet::empty(), now)
            .await;

        // Should succeed as metrics are created on demand
        assert!(result.is_ok());

        // Test with NaN value for gauge
        let result = repo
            .set_gauge(
                &metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL),
                &LabelSet::empty(),
                f64::NAN,
                now,
            )
            .await;

        // Should handle NaN values
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn it_should_handle_moving_average_calculation_before_any_connections_are_recorded() {
        let repo = Repository::new();
        let connect_labels = LabelSet::from([("request_kind", "connect")]);
        let now = CurrentClock::now();

        // This test checks the behavior of `recalculate_udp_avg_connect_processing_time_ns``
        // when no connections have been recorded yet. The first call should
        // handle division by zero gracefully and return an infinite average,
        // which is the current behavior.

        // todo: the first average should be 2000ns, not infinity.
        // This is because the first connection is not counted in the average
        // calculation if the counter is increased after calculating the average.
        // The problem is that we count requests when they are accepted, not
        // when they are processed. And we calculate the average when the
        // response is sent.

        // First calculation: no connections recorded yet, should result in infinity
        let processing_time_1 = Duration::from_nanos(2000);
        let avg_1 = repo
            .recalculate_udp_avg_connect_processing_time_ns(processing_time_1, &connect_labels, now)
            .await;

        assert!(
            (avg_1 - 2000.0).abs() < f64::EPSILON,
            "First calculation should be 2000, but got {avg_1}"
        );

        // Now add one connection and try again
        let ipv4_labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "connect")]);
        repo.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &ipv4_labels, now)
            .await
            .unwrap();

        // Second calculation: 1 connection
        let processing_time_2 = Duration::from_nanos(3000);
        let connect_labels = LabelSet::from([("request_kind", "connect")]);
        let avg_2 = repo
            .recalculate_udp_avg_connect_processing_time_ns(processing_time_2, &connect_labels, now)
            .await;

        // There is one connection, so the average should be:
        // 2000 + (3000 - 2000) / 1 = 2000 + 1000 = 3000
        // This is because one connection is not counted yet in the average calculation,
        // so the average is simply the processing time of the second connection.
        assert!(
            (avg_2 - 3000.0).abs() < f64::EPSILON,
            "Second calculation should be 3000ns, but got {avg_2}"
        );
    }
}
