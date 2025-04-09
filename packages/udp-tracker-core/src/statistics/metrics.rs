use serde::Serialize;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric::MetricName;
use torrust_tracker_metrics::metric_collection::MetricCollection;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

/// Metrics collected by the tracker.
///
/// - Number of connections handled
/// - Number of `announce` requests handled
/// - Number of `scrape` request handled
///
/// These metrics are collected for each connection type: UDP and HTTP
/// and also for each IP version used by the peers: IPv4 and IPv6.
#[derive(Debug, PartialEq, Default, Serialize)]
pub struct Metrics {
    /// Total number of UDP (UDP tracker) connections from IPv4 peers.
    pub udp4_connections_handled: u64,

    /// Total number of UDP (UDP tracker) `announce` requests from IPv4 peers.
    pub udp4_announces_handled: u64,

    /// Total number of UDP (UDP tracker) `scrape` requests from IPv4 peers.
    pub udp4_scrapes_handled: u64,

    /// Total number of UDP (UDP tracker) `connection` requests from IPv6 peers.
    pub udp6_connections_handled: u64,

    /// Total number of UDP (UDP tracker) `announce` requests from IPv6 peers.
    pub udp6_announces_handled: u64,

    /// Total number of UDP (UDP tracker) `scrape` requests from IPv6 peers.
    pub udp6_scrapes_handled: u64,

    /// A collection of metrics.
    pub metric_collection: MetricCollection,
}

impl Metrics {
    pub fn increase_counter(&mut self, metric_name: &MetricName, labels: &LabelSet, now: DurationSinceUnixEpoch) {
        self.metric_collection.increase_counter(metric_name, labels, now);
    }

    pub fn set_gauge(&mut self, metric_name: &MetricName, labels: &LabelSet, value: f64, now: DurationSinceUnixEpoch) {
        self.metric_collection.set_gauge(metric_name, labels, value, now);
    }
}
