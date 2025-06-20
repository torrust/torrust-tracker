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

    pub async fn recalculate_udp_avg_processing_time_ns(
        &self,
        req_processing_time: Duration,
        label_set: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> f64 {
        let mut stats_lock = self.stats.write().await;

        let new_avg = stats_lock.recalculate_udp_avg_processing_time_ns(req_processing_time, label_set, now);

        drop(stats_lock);

        new_avg
    }
}

#[cfg(test)]
mod tests {
    use core::f64;
    use std::time::Duration;

    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_metrics::metric_collection::aggregate::sum::Sum;
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
        assert_eq!(stats.udp_requests_aborted_total(), 0);
        assert_eq!(stats.udp_requests_banned_total(), 0);
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
        assert_eq!(stats.udp_requests_aborted_total(), 1);
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
        assert_eq!(stats.udp_requests_aborted_total(), 5);
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
        assert_eq!(stats.udp4_requests_received_total(), 1);
        assert_eq!(stats.udp6_requests_received_total(), 1);
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

        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let udp_avg_connect_processing_time_ns = stats
            .metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                &[("request_kind", "connect")].into(),
            )
            .unwrap_or_default() as u64;

        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let udp_avg_announce_processing_time_ns = stats
            .metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                &[("request_kind", "announce")].into(),
            )
            .unwrap_or_default() as u64;

        assert_eq!(udp_avg_connect_processing_time_ns, 1000);
        assert_eq!(udp_avg_announce_processing_time_ns, 2000);
    }

    #[tokio::test]
    async fn it_should_recalculate_the_udp_average_connect_processing_time_in_nanoseconds_using_moving_average() {
        let repo = Repository::new();
        let now = CurrentClock::now();

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
        // This will increment the processed requests counter from 0 to 1
        let processing_time = Duration::from_nanos(2000);
        let new_avg = repo
            .recalculate_udp_avg_processing_time_ns(processing_time, &connect_labels, now)
            .await;

        // Moving average: previous_avg + (new_value - previous_avg) / processed_requests_total
        // With processed_requests_total = 1 (incremented during the call):
        // 1000 + (2000 - 1000) / 1 = 1000 + 1000 = 2000
        let expected_avg = 1000.0 + (2000.0 - 1000.0) / 1.0;
        assert!(
            (new_avg - expected_avg).abs() < 0.01,
            "Expected {expected_avg}, got {new_avg}"
        );
    }

    #[tokio::test]
    async fn it_should_recalculate_the_udp_average_announce_processing_time_in_nanoseconds_using_moving_average() {
        let repo = Repository::new();
        let now = CurrentClock::now();

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
        // This will increment the processed requests counter from 0 to 1
        let processing_time = Duration::from_nanos(1500);
        let new_avg = repo
            .recalculate_udp_avg_processing_time_ns(processing_time, &announce_labels, now)
            .await;

        // Moving average: previous_avg + (new_value - previous_avg) / processed_requests_total
        // With processed_requests_total = 1 (incremented during the call):
        // 500 + (1500 - 500) / 1 = 500 + 1000 = 1500
        let expected_avg = 500.0 + (1500.0 - 500.0) / 1.0;
        assert!(
            (new_avg - expected_avg).abs() < 0.01,
            "Expected {expected_avg}, got {new_avg}"
        );
    }

    #[tokio::test]
    async fn it_should_recalculate_the_udp_average_scrape_processing_time_in_nanoseconds_using_moving_average() {
        let repo = Repository::new();
        let now = CurrentClock::now();

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
        // This will increment the processed requests counter from 0 to 1
        let processing_time = Duration::from_nanos(1200);
        let new_avg = repo
            .recalculate_udp_avg_processing_time_ns(processing_time, &scrape_labels, now)
            .await;

        // Moving average: previous_avg + (new_value - previous_avg) / processed_requests_total
        // With processed_requests_total = 1 (incremented during the call):
        // 800 + (1200 - 800) / 1 = 800 + 400 = 1200
        let expected_avg = 800.0 + (1200.0 - 800.0) / 1.0;
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
            .recalculate_udp_avg_processing_time_ns(processing_time, &connect_labels, now)
            .await;

        let announce_labels = LabelSet::from([("request_kind", "announce")]);
        let announce_avg = repo
            .recalculate_udp_avg_processing_time_ns(processing_time, &announce_labels, now)
            .await;

        let scrape_labels = LabelSet::from([("request_kind", "scrape")]);
        let scrape_avg = repo
            .recalculate_udp_avg_processing_time_ns(processing_time, &scrape_labels, now)
            .await;

        // With 0 total connections, the formula becomes 0 + (1000 - 0) / 0
        // This should handle the division by zero case gracefully
        assert!((connect_avg - 1000.0).abs() < f64::EPSILON);
        assert!((announce_avg - 1000.0).abs() < f64::EPSILON);
        assert!((scrape_avg - 1000.0).abs() < f64::EPSILON);
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
        assert_eq!(stats.udp_requests_aborted_total(), 50); // 10 tasks * 5 increments each
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
            .recalculate_udp_avg_processing_time_ns(large_duration, &connect_labels, now)
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
        assert_eq!(stats.udp_requests_aborted_total(), 1);
        assert_eq!(stats.udp_banned_ips_total(), 10);
        assert_eq!(stats.udp_requests_banned_total(), 1);
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
    #[allow(clippy::too_many_lines)]
    async fn it_should_handle_race_conditions_when_updating_udp_performance_metrics_in_parallel() {
        // Number of concurrent requests per server
        const REQUESTS_PER_SERVER: usize = 100;

        let repo = Repository::new();
        let now = CurrentClock::now();

        // Define labels for two different UDP servers
        let server1_labels = LabelSet::from([
            ("request_kind", "connect"),
            ("server_binding_address_ip_family", "inet"),
            ("server_port", "6868"),
        ]);
        let server2_labels = LabelSet::from([
            ("request_kind", "connect"),
            ("server_binding_address_ip_family", "inet"),
            ("server_port", "6969"),
        ]);

        let mut handles = vec![];

        // Spawn tasks for server 1
        for i in 0..REQUESTS_PER_SERVER {
            let repo_clone = repo.clone();
            let labels = server1_labels.clone();
            let handle = tokio::spawn(async move {
                // Simulate varying processing times (1000ns to 5000ns)
                let processing_time_ns = 1000 + (i % 5) * 1000;
                let processing_time = Duration::from_nanos(processing_time_ns as u64);

                repo_clone
                    .recalculate_udp_avg_processing_time_ns(processing_time, &labels, now)
                    .await
            });
            handles.push(handle);
        }

        // Spawn tasks for server 2
        for i in 0..REQUESTS_PER_SERVER {
            let repo_clone = repo.clone();
            let labels = server2_labels.clone();
            let handle = tokio::spawn(async move {
                // Simulate different processing times (2000ns to 6000ns)
                let processing_time_ns = 2000 + (i % 5) * 1000;
                let processing_time = Duration::from_nanos(processing_time_ns as u64);

                repo_clone
                    .recalculate_udp_avg_processing_time_ns(processing_time, &labels, now)
                    .await
            });
            handles.push(handle);
        }

        // Collect all the results
        let mut server1_results = Vec::new();
        let mut server2_results = Vec::new();

        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.await.unwrap();
            if i < REQUESTS_PER_SERVER {
                server1_results.push(result);
            } else {
                server2_results.push(result);
            }
        }

        // Verify that all tasks completed successfully
        assert_eq!(server1_results.len(), REQUESTS_PER_SERVER);
        assert_eq!(server2_results.len(), REQUESTS_PER_SERVER);

        // Verify that all results are finite and positive
        for result in &server1_results {
            assert!(result.is_finite(), "Server 1 result should be finite: {result}");
            assert!(*result > 0.0, "Server 1 result should be positive: {result}");
        }

        for result in &server2_results {
            assert!(result.is_finite(), "Server 2 result should be finite: {result}");
            assert!(*result > 0.0, "Server 2 result should be positive: {result}");
        }

        // Get final stats and verify metrics integrity
        let stats = repo.get_stats().await;

        // Verify that the processed requests counters are correct for each server
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let server1_processed = stats
            .metric_collection
            .get_counter_value(
                &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSED_REQUESTS_TOTAL),
                &server1_labels,
            )
            .unwrap()
            .value();

        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let server2_processed = stats
            .metric_collection
            .get_counter_value(
                &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSED_REQUESTS_TOTAL),
                &server2_labels,
            )
            .unwrap()
            .value();

        assert_eq!(
            server1_processed, REQUESTS_PER_SERVER as u64,
            "Server 1 should have processed {REQUESTS_PER_SERVER} requests",
        );
        assert_eq!(
            server2_processed, REQUESTS_PER_SERVER as u64,
            "Server 2 should have processed {REQUESTS_PER_SERVER} requests",
        );

        // Verify that the final average processing times are reasonable
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let server1_final_avg = stats
            .metric_collection
            .get_gauge_value(
                &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                &server1_labels,
            )
            .unwrap()
            .value();

        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let server2_final_avg = stats
            .metric_collection
            .get_gauge_value(
                &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                &server2_labels,
            )
            .unwrap()
            .value();

        // Server 1: 100 requests cycling through [1000, 2000, 3000, 4000, 5000] ns
        // Expected average: (20×1000 + 20×2000 + 20×3000 + 20×4000 + 20×5000) / 100 = 3000 ns
        // Note: Moving average with concurrent updates may have small deviations due to order dependency
        assert!(
            (server1_final_avg - 3000.0).abs() < 50.0,
            "Server 1 final average should be close to 3000ns (±50ns), got {server1_final_avg}ns"
        );

        // Server 2: 100 requests cycling through [2000, 3000, 4000, 5000, 6000] ns
        // Expected average: (20×2000 + 20×3000 + 20×4000 + 20×5000 + 20×6000) / 100 = 4000 ns
        // Note: Moving average with concurrent updates may have small deviations due to order dependency
        assert!(
            (server2_final_avg - 4000.0).abs() < 50.0,
            "Server 2 final average should be close to 4000ns (±50ns), got {server2_final_avg}ns"
        );

        // Verify that the two servers have different averages (they should since they have different processing time ranges)
        assert!(
            (server1_final_avg - server2_final_avg).abs() > 950.0,
            "Server 1 and Server 2 should have different average processing times"
        );

        // Server 2 should generally have higher averages since its processing times are higher
        assert!(
            server2_final_avg > server1_final_avg,
            "Server 2 average ({server2_final_avg}) should be higher than Server 1 average ({server1_final_avg})"
        );

        // Verify that the moving average calculation maintains consistency
        // The last result for each server should match the final stored average
        let server1_last_result = server1_results.last().copied().unwrap();
        let server2_last_result = server2_results.last().copied().unwrap();

        // Note: Due to race conditions, the last result might not exactly match the final stored average
        // but it should be in a reasonable range. We'll check that they're in the same ballpark.
        let server1_diff = (server1_last_result - server1_final_avg).abs();
        let server2_diff = (server2_last_result - server2_final_avg).abs();

        assert!(
            server1_diff <= 0.0,
            "Server 1 last result ({server1_last_result}) should be equal to final average ({server1_final_avg}), diff: {server1_diff}",
        );

        assert!(
            server2_diff <= 0.0,
            "Server 2 last result ({server2_last_result}) should be equal to final average ({server2_final_avg}), diff: {server2_diff}",
        );

        // Verify that the metric collection contains the expected metrics for both servers
        assert!(stats
            .metric_collection
            .contains_gauge(&metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS)));
        assert!(stats
            .metric_collection
            .contains_counter(&metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSED_REQUESTS_TOTAL)));

        println!(
            "Race condition test completed successfully:\n  Server 1: {server1_processed} requests, final avg: {server1_final_avg}ns\n  Server 2: {server2_processed} requests, final avg: {server2_final_avg}ns"
        );
    }
}
