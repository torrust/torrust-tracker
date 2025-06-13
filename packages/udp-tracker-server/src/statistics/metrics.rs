use serde::Serialize;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric::MetricName;
use torrust_tracker_metrics::metric_collection::{Error, MetricCollection};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

/// Metrics collected by the UDP tracker server.
#[derive(Debug, PartialEq, Default, Serialize)]
pub struct Metrics {
    // UDP
    /// Total number of UDP (UDP tracker) requests aborted.
    pub udp_requests_aborted: u64,

    /// Total number of UDP (UDP tracker) requests banned.
    pub udp_requests_banned: u64,

    /// Total number of banned IPs.
    pub udp_banned_ips_total: u64,

    /// Average rounded time spent processing UDP connect requests.
    pub udp_avg_connect_processing_time_ns: u64,

    /// Average rounded time spent processing UDP announce requests.
    pub udp_avg_announce_processing_time_ns: u64,

    /// Average rounded time spent processing UDP scrape requests.
    pub udp_avg_scrape_processing_time_ns: u64,

    // UDPv4
    /// Total number of UDP (UDP tracker) requests from IPv4 peers.
    pub udp4_requests: u64,

    /// Total number of UDP (UDP tracker) connections from IPv4 peers.
    pub udp4_connections_handled: u64,

    /// Total number of UDP (UDP tracker) `announce` requests from IPv4 peers.
    pub udp4_announces_handled: u64,

    /// Total number of UDP (UDP tracker) `scrape` requests from IPv4 peers.
    pub udp4_scrapes_handled: u64,

    /// Total number of UDP (UDP tracker) responses from IPv4 peers.
    pub udp4_responses: u64,

    /// Total number of UDP (UDP tracker) `error` requests from IPv4 peers.
    pub udp4_errors_handled: u64,

    // UDPv6
    /// Total number of UDP (UDP tracker) requests from IPv6 peers.
    pub udp6_requests: u64,

    /// Total number of UDP (UDP tracker) `connection` requests from IPv6 peers.
    pub udp6_connections_handled: u64,

    /// Total number of UDP (UDP tracker) `announce` requests from IPv6 peers.
    pub udp6_announces_handled: u64,

    /// Total number of UDP (UDP tracker) `scrape` requests from IPv6 peers.
    pub udp6_scrapes_handled: u64,

    /// Total number of UDP (UDP tracker) responses from IPv6 peers.
    pub udp6_responses: u64,

    /// Total number of UDP (UDP tracker) `error` requests from IPv6 peers.
    pub udp6_errors_handled: u64,

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
