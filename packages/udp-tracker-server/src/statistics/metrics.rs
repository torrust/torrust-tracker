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
