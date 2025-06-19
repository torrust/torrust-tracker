use std::time::Duration;

use serde::Serialize;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric::MetricName;
use torrust_tracker_metrics::metric_collection::aggregate::sum::Sum;
use torrust_tracker_metrics::metric_collection::{Error, MetricCollection};
use torrust_tracker_metrics::metric_name;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::statistics::{
    UDP_TRACKER_SERVER_ERRORS_TOTAL, UDP_TRACKER_SERVER_IPS_BANNED_TOTAL, UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS,
    UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL, UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL,
    UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL, UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL,
    UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL,
};

/// Metrics collected by the UDP tracker server.
#[derive(Debug, PartialEq, Default, Serialize)]
pub struct Metrics {
    /// A collection of metrics.
    pub metric_collection: MetricCollection,
}

impl Metrics {
    /// # Errors
    ///
    /// Returns an error if the metric does not exist and it cannot be created.
    pub fn increase_counter(
        &mut self,
        metric_name: &MetricName,
        labels: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        self.metric_collection.increment_counter(metric_name, labels, now)
    }

    /// # Errors
    ///
    /// Returns an error if the metric does not exist and it cannot be created.
    pub fn set_gauge(
        &mut self,
        metric_name: &MetricName,
        labels: &LabelSet,
        value: f64,
        now: DurationSinceUnixEpoch,
    ) -> Result<(), Error> {
        self.metric_collection.set_gauge(metric_name, labels, value, now)
    }
}

impl Metrics {
    #[allow(clippy::cast_precision_loss)]
    pub fn recalculate_udp_avg_connect_processing_time_ns(
        &mut self,
        req_processing_time: Duration,
        label_set: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> f64 {
        let req_processing_time = req_processing_time.as_nanos() as f64;

        let request_accepted_total = self.udp_request_accepted(label_set) as f64;

        let previous_avg = self.udp_avg_processing_time_ns(label_set);

        let new_avg = if request_accepted_total == 0.0 {
            req_processing_time
        } else {
            // Moving average: https://en.wikipedia.org/wiki/Moving_average
            previous_avg as f64 + (req_processing_time - previous_avg as f64) / request_accepted_total
        };

        tracing::debug!(
            "Recalculated UDP average connect processing time: {} ns (previous: {} ns, req_processing_time: {} ns, udp_connections_handled: {})",
            new_avg,
            previous_avg,
            req_processing_time,
            request_accepted_total
        );

        self.update_udp_avg_processing_time_ns(new_avg, label_set, now);

        new_avg
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn recalculate_udp_avg_announce_processing_time_ns(
        &mut self,
        req_processing_time: Duration,
        label_set: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> f64 {
        let req_processing_time = req_processing_time.as_nanos() as f64;

        let request_accepted_total = self.udp_request_accepted(label_set) as f64;

        let previous_avg = self.udp_avg_processing_time_ns(label_set);

        let new_avg = if request_accepted_total == 0.0 {
            req_processing_time
        } else {
            // Moving average: https://en.wikipedia.org/wiki/Moving_average
            previous_avg as f64 + (req_processing_time - previous_avg as f64) / request_accepted_total
        };

        tracing::debug!(
            "Recalculated UDP average announce processing time: {} ns (previous: {} ns, req_processing_time: {} ns, udp_announces_handled: {})",
            new_avg,
            previous_avg,
            req_processing_time,
            request_accepted_total
        );

        self.update_udp_avg_processing_time_ns(new_avg, label_set, now);

        new_avg
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn recalculate_udp_avg_scrape_processing_time_ns(
        &mut self,
        req_processing_time: Duration,
        label_set: &LabelSet,
        now: DurationSinceUnixEpoch,
    ) -> f64 {
        let req_processing_time = req_processing_time.as_nanos() as f64;

        let request_accepted_total = self.udp_request_accepted(label_set) as f64;

        let previous_avg = self.udp_avg_processing_time_ns(label_set);

        let new_avg = if request_accepted_total == 0.0 {
            req_processing_time
        } else {
            // Moving average: https://en.wikipedia.org/wiki/Moving_average
            previous_avg as f64 + (req_processing_time - previous_avg as f64) / request_accepted_total
        };

        tracing::debug!(
            "Recalculated UDP average scrape processing time: {} ns (previous: {} ns, req_processing_time: {} ns, udp_scrapes_handled: {})",
            new_avg,
            previous_avg,
            req_processing_time,
            request_accepted_total
        );

        self.update_udp_avg_processing_time_ns(new_avg, label_set, now);

        new_avg
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp_avg_processing_time_ns(&self, label_set: &LabelSet) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                label_set,
            )
            .unwrap_or_default() as u64
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp_request_accepted(&self, label_set: &LabelSet) -> u64 {
        self.metric_collection
            .sum(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), label_set)
            .unwrap_or_default() as u64
    }

    fn update_udp_avg_processing_time_ns(&mut self, new_avg: f64, label_set: &LabelSet, now: DurationSinceUnixEpoch) {
        tracing::debug!(
            "Updating average processing time metric to {} ns for label set {}",
            new_avg,
            label_set,
        );

        match self.set_gauge(
            &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
            label_set,
            new_avg,
            now,
        ) {
            Ok(()) => {}
            Err(err) => tracing::error!("Failed to set gauge: {}", err),
        }
    }

    // UDP
    /// Total number of UDP (UDP tracker) requests aborted.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp_requests_aborted(&self) -> u64 {
        self.metric_collection
            .sum(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL), &LabelSet::empty())
            .unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) requests banned.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp_requests_banned(&self) -> u64 {
        self.metric_collection
            .sum(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL), &LabelSet::empty())
            .unwrap_or_default() as u64
    }

    /// Total number of banned IPs.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp_banned_ips_total(&self) -> u64 {
        self.metric_collection
            .sum(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL), &LabelSet::empty())
            .unwrap_or_default() as u64
    }

    /// Average rounded time spent processing UDP connect requests.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp_avg_connect_processing_time_ns(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                &[("request_kind", "connect")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Average rounded time spent processing UDP announce requests.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp_avg_announce_processing_time_ns(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                &[("request_kind", "announce")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Average rounded time spent processing UDP scrape requests.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp_avg_scrape_processing_time_ns(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                &[("request_kind", "scrape")].into(),
            )
            .unwrap_or_default() as u64
    }

    // UDPv4
    /// Total number of UDP (UDP tracker) requests from IPv4 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp4_requests(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL),
                &[("server_binding_address_ip_family", "inet")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) connections from IPv4 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp4_connections_handled(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                &[("server_binding_address_ip_family", "inet"), ("request_kind", "connect")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) `announce` requests from IPv4 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp4_announces_handled(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                &[("server_binding_address_ip_family", "inet"), ("request_kind", "announce")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) `scrape` requests from IPv4 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp4_scrapes_handled(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                &[("server_binding_address_ip_family", "inet"), ("request_kind", "scrape")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) responses from IPv4 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp4_responses(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL),
                &[("server_binding_address_ip_family", "inet")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) `error` requests from IPv4 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp4_errors_handled(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_ERRORS_TOTAL),
                &[("server_binding_address_ip_family", "inet")].into(),
            )
            .unwrap_or_default() as u64
    }

    // UDPv6
    /// Total number of UDP (UDP tracker) requests from IPv6 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp6_requests(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL),
                &[("server_binding_address_ip_family", "inet6")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) `connection` requests from IPv6 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp6_connections_handled(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                &[("server_binding_address_ip_family", "inet6"), ("request_kind", "connect")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) `announce` requests from IPv6 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp6_announces_handled(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                &[("server_binding_address_ip_family", "inet6"), ("request_kind", "announce")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) `scrape` requests from IPv6 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp6_scrapes_handled(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                &[("server_binding_address_ip_family", "inet6"), ("request_kind", "scrape")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) responses from IPv6 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp6_responses(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL),
                &[("server_binding_address_ip_family", "inet6")].into(),
            )
            .unwrap_or_default() as u64
    }

    /// Total number of UDP (UDP tracker) `error` requests from IPv6 peers.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn udp6_errors_handled(&self) -> u64 {
        self.metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_SERVER_ERRORS_TOTAL),
                &[("server_binding_address_ip_family", "inet6")].into(),
            )
            .unwrap_or_default() as u64
    }
}

#[cfg(test)]
mod tests {
    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_metrics::metric_name;

    use super::*;
    use crate::statistics::{
        UDP_TRACKER_SERVER_ERRORS_TOTAL, UDP_TRACKER_SERVER_IPS_BANNED_TOTAL,
        UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS, UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL,
        UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL, UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL,
        UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL, UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL,
    };
    use crate::CurrentClock;

    #[test]
    fn it_should_implement_default() {
        let metrics = Metrics::default();
        // MetricCollection starts with empty collections
        assert_eq!(metrics, Metrics::default());
    }

    #[test]
    fn it_should_implement_debug() {
        let metrics = Metrics::default();
        let debug_string = format!("{metrics:?}");
        assert!(debug_string.contains("Metrics"));
        assert!(debug_string.contains("metric_collection"));
    }

    #[test]
    fn it_should_implement_partial_eq() {
        let metrics1 = Metrics::default();
        let metrics2 = Metrics::default();
        assert_eq!(metrics1, metrics2);
    }

    #[test]
    fn it_should_increase_counter_metric() {
        let mut metrics = Metrics::default();
        let now = CurrentClock::now();
        let labels = LabelSet::empty();

        let result = metrics.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL), &labels, now);

        assert!(result.is_ok());
    }

    #[test]
    fn it_should_increase_counter_metric_with_labels() {
        let mut metrics = Metrics::default();
        let now = CurrentClock::now();
        let labels = LabelSet::from([("server_binding_address_ip_family", "inet")]);

        let result = metrics.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL), &labels, now);

        assert!(result.is_ok());
    }

    #[test]
    fn it_should_set_gauge_metric() {
        let mut metrics = Metrics::default();
        let now = CurrentClock::now();
        let labels = LabelSet::empty();

        let result = metrics.set_gauge(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL), &labels, 42.0, now);

        assert!(result.is_ok());
    }

    #[test]
    fn it_should_set_gauge_metric_with_labels() {
        let mut metrics = Metrics::default();
        let now = CurrentClock::now();
        let labels = LabelSet::from([("request_kind", "connect")]);

        let result = metrics.set_gauge(
            &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
            &labels,
            1000.0,
            now,
        );

        assert!(result.is_ok());
    }

    mod udp_general_metrics {
        use super::*;

        #[test]
        fn it_should_return_zero_for_udp_requests_aborted_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp_requests_aborted(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp_requests_aborted() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::empty();

            metrics
                .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL), &labels, now)
                .unwrap();
            metrics
                .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL), &labels, now)
                .unwrap();

            assert_eq!(metrics.udp_requests_aborted(), 2);
        }

        #[test]
        fn it_should_return_zero_for_udp_requests_banned_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp_requests_banned(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp_requests_banned() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::empty();

            for _ in 0..3 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp_requests_banned(), 3);
        }

        #[test]
        fn it_should_return_zero_for_udp_banned_ips_total_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp_banned_ips_total(), 0);
        }

        #[test]
        fn it_should_return_gauge_value_for_udp_banned_ips_total() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::empty();

            metrics
                .set_gauge(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL), &labels, 10.0, now)
                .unwrap();

            assert_eq!(metrics.udp_banned_ips_total(), 10);
        }
    }

    mod udp_performance_metrics {
        use super::*;

        #[test]
        fn it_should_return_zero_for_udp_avg_connect_processing_time_ns_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp_avg_connect_processing_time_ns(), 0);
        }

        #[test]
        fn it_should_return_gauge_value_for_udp_avg_connect_processing_time_ns() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("request_kind", "connect")]);

            metrics
                .set_gauge(
                    &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                    &labels,
                    1500.0,
                    now,
                )
                .unwrap();

            assert_eq!(metrics.udp_avg_connect_processing_time_ns(), 1500);
        }

        #[test]
        fn it_should_return_zero_for_udp_avg_announce_processing_time_ns_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp_avg_announce_processing_time_ns(), 0);
        }

        #[test]
        fn it_should_return_gauge_value_for_udp_avg_announce_processing_time_ns() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("request_kind", "announce")]);

            metrics
                .set_gauge(
                    &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                    &labels,
                    2500.0,
                    now,
                )
                .unwrap();

            assert_eq!(metrics.udp_avg_announce_processing_time_ns(), 2500);
        }

        #[test]
        fn it_should_return_zero_for_udp_avg_scrape_processing_time_ns_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp_avg_scrape_processing_time_ns(), 0);
        }

        #[test]
        fn it_should_return_gauge_value_for_udp_avg_scrape_processing_time_ns() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("request_kind", "scrape")]);

            metrics
                .set_gauge(
                    &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                    &labels,
                    3500.0,
                    now,
                )
                .unwrap();

            assert_eq!(metrics.udp_avg_scrape_processing_time_ns(), 3500);
        }
    }

    mod udpv4_metrics {
        use super::*;

        #[test]
        fn it_should_return_zero_for_udp4_requests_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp4_requests(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp4_requests() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet")]);

            for _ in 0..5 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp4_requests(), 5);
        }

        #[test]
        fn it_should_return_zero_for_udp4_connections_handled_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp4_connections_handled(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp4_connections_handled() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "connect")]);

            for _ in 0..3 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp4_connections_handled(), 3);
        }

        #[test]
        fn it_should_return_zero_for_udp4_announces_handled_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp4_announces_handled(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp4_announces_handled() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "announce")]);

            for _ in 0..7 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp4_announces_handled(), 7);
        }

        #[test]
        fn it_should_return_zero_for_udp4_scrapes_handled_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp4_scrapes_handled(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp4_scrapes_handled() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "scrape")]);

            for _ in 0..4 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp4_scrapes_handled(), 4);
        }

        #[test]
        fn it_should_return_zero_for_udp4_responses_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp4_responses(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp4_responses() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet")]);

            for _ in 0..6 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp4_responses(), 6);
        }

        #[test]
        fn it_should_return_zero_for_udp4_errors_handled_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp4_errors_handled(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp4_errors_handled() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet")]);

            for _ in 0..2 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_ERRORS_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp4_errors_handled(), 2);
        }
    }

    mod udpv6_metrics {
        use super::*;

        #[test]
        fn it_should_return_zero_for_udp6_requests_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp6_requests(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp6_requests() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet6")]);

            for _ in 0..8 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp6_requests(), 8);
        }

        #[test]
        fn it_should_return_zero_for_udp6_connections_handled_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp6_connections_handled(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp6_connections_handled() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet6"), ("request_kind", "connect")]);

            for _ in 0..4 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp6_connections_handled(), 4);
        }

        #[test]
        fn it_should_return_zero_for_udp6_announces_handled_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp6_announces_handled(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp6_announces_handled() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet6"), ("request_kind", "announce")]);

            for _ in 0..9 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp6_announces_handled(), 9);
        }

        #[test]
        fn it_should_return_zero_for_udp6_scrapes_handled_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp6_scrapes_handled(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp6_scrapes_handled() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet6"), ("request_kind", "scrape")]);

            for _ in 0..6 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp6_scrapes_handled(), 6);
        }

        #[test]
        fn it_should_return_zero_for_udp6_responses_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp6_responses(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp6_responses() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet6")]);

            for _ in 0..11 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp6_responses(), 11);
        }

        #[test]
        fn it_should_return_zero_for_udp6_errors_handled_when_no_data() {
            let metrics = Metrics::default();
            assert_eq!(metrics.udp6_errors_handled(), 0);
        }

        #[test]
        fn it_should_return_sum_of_udp6_errors_handled() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("server_binding_address_ip_family", "inet6")]);

            for _ in 0..3 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_ERRORS_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp6_errors_handled(), 3);
        }
    }

    mod combined_metrics {
        use super::*;

        #[test]
        fn it_should_distinguish_between_ipv4_and_ipv6_metrics() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();

            let ipv4_labels = LabelSet::from([("server_binding_address_ip_family", "inet")]);
            let ipv6_labels = LabelSet::from([("server_binding_address_ip_family", "inet6")]);

            // Add different counts for IPv4 and IPv6
            for _ in 0..3 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL), &ipv4_labels, now)
                    .unwrap();
            }

            for _ in 0..7 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL), &ipv6_labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp4_requests(), 3);
            assert_eq!(metrics.udp6_requests(), 7);
        }

        #[test]
        fn it_should_distinguish_between_different_request_kinds() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();

            let connect_labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "connect")]);
            let announce_labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "announce")]);
            let scrape_labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "scrape")]);

            // Add different counts for different request kinds
            for _ in 0..2 {
                metrics
                    .increase_counter(
                        &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                        &connect_labels,
                        now,
                    )
                    .unwrap();
            }

            for _ in 0..5 {
                metrics
                    .increase_counter(
                        &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                        &announce_labels,
                        now,
                    )
                    .unwrap();
            }

            for _ in 0..1 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &scrape_labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp4_connections_handled(), 2);
            assert_eq!(metrics.udp4_announces_handled(), 5);
            assert_eq!(metrics.udp4_scrapes_handled(), 1);
        }

        #[test]
        fn it_should_handle_mixed_ipv4_and_ipv6_for_different_request_kinds() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();

            let ipv4_connect_labels = LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "connect")]);
            let ipv6_connect_labels =
                LabelSet::from([("server_binding_address_ip_family", "inet6"), ("request_kind", "connect")]);
            let ipv4_announce_labels =
                LabelSet::from([("server_binding_address_ip_family", "inet"), ("request_kind", "announce")]);
            let ipv6_announce_labels =
                LabelSet::from([("server_binding_address_ip_family", "inet6"), ("request_kind", "announce")]);

            // Add mixed IPv4/IPv6 counts
            for _ in 0..3 {
                metrics
                    .increase_counter(
                        &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                        &ipv4_connect_labels,
                        now,
                    )
                    .unwrap();
            }

            for _ in 0..2 {
                metrics
                    .increase_counter(
                        &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                        &ipv6_connect_labels,
                        now,
                    )
                    .unwrap();
            }

            for _ in 0..4 {
                metrics
                    .increase_counter(
                        &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                        &ipv4_announce_labels,
                        now,
                    )
                    .unwrap();
            }

            for _ in 0..6 {
                metrics
                    .increase_counter(
                        &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL),
                        &ipv6_announce_labels,
                        now,
                    )
                    .unwrap();
            }

            assert_eq!(metrics.udp4_connections_handled(), 3);
            assert_eq!(metrics.udp6_connections_handled(), 2);
            assert_eq!(metrics.udp4_announces_handled(), 4);
            assert_eq!(metrics.udp6_announces_handled(), 6);
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn it_should_handle_large_counter_values() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::empty();

            // Add a large number of increments
            for _ in 0..1000 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL), &labels, now)
                    .unwrap();
            }

            assert_eq!(metrics.udp_requests_aborted(), 1000);
        }

        #[test]
        fn it_should_handle_large_gauge_values() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::empty();

            // Set a large gauge value
            metrics
                .set_gauge(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL), &labels, 999_999.0, now)
                .unwrap();

            assert_eq!(metrics.udp_banned_ips_total(), 999_999);
        }

        #[test]
        fn it_should_handle_zero_gauge_values() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::empty();

            metrics
                .set_gauge(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL), &labels, 0.0, now)
                .unwrap();

            assert_eq!(metrics.udp_banned_ips_total(), 0);
        }

        #[test]
        fn it_should_handle_fractional_gauge_values_with_truncation() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::from([("request_kind", "connect")]);

            metrics
                .set_gauge(
                    &metric_name!(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                    &labels,
                    1234.567,
                    now,
                )
                .unwrap();

            // Should truncate to 1234
            assert_eq!(metrics.udp_avg_connect_processing_time_ns(), 1234);
        }

        #[test]
        fn it_should_overwrite_gauge_values_when_set_multiple_times() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::empty();

            // Set initial value
            metrics
                .set_gauge(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL), &labels, 50.0, now)
                .unwrap();

            assert_eq!(metrics.udp_banned_ips_total(), 50);

            // Overwrite with new value
            metrics
                .set_gauge(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL), &labels, 75.0, now)
                .unwrap();

            assert_eq!(metrics.udp_banned_ips_total(), 75);
        }

        #[test]
        fn it_should_handle_empty_label_sets() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let empty_labels = LabelSet::empty();

            let result = metrics.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL), &empty_labels, now);

            assert!(result.is_ok());
            assert_eq!(metrics.udp_requests_aborted(), 1);
        }

        #[test]
        fn it_should_handle_multiple_labels_on_same_metric() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();

            let labels1 = LabelSet::from([("server_binding_address_ip_family", "inet")]);
            let labels2 = LabelSet::from([("server_binding_address_ip_family", "inet6")]);

            // Add to same metric with different labels
            for _ in 0..3 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL), &labels1, now)
                    .unwrap();
            }

            for _ in 0..5 {
                metrics
                    .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL), &labels2, now)
                    .unwrap();
            }

            // Should return labeled sums correctly
            assert_eq!(metrics.udp4_requests(), 3);
            assert_eq!(metrics.udp6_requests(), 5);
        }
    }

    mod error_handling {
        use super::*;

        #[test]
        fn it_should_return_ok_result_for_valid_counter_operations() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::empty();

            let result = metrics.increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL), &labels, now);

            assert!(result.is_ok());
        }

        #[test]
        fn it_should_return_ok_result_for_valid_gauge_operations() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::empty();

            let result = metrics.set_gauge(&metric_name!(UDP_TRACKER_SERVER_IPS_BANNED_TOTAL), &labels, 42.0, now);

            assert!(result.is_ok());
        }

        #[test]
        fn it_should_handle_unknown_metric_names_gracefully() {
            let mut metrics = Metrics::default();
            let now = CurrentClock::now();
            let labels = LabelSet::empty();

            // This should still work as metrics are created on demand
            let result = metrics.increase_counter(&metric_name!("unknown_metric"), &labels, now);

            assert!(result.is_ok());
        }
    }
}
