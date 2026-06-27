//! API resources for the stats context.
//!
//! These types define the serialization contract for the `/api/v1/stats`
//! and `/api/v1/metrics` endpoint responses.
use serde::{Deserialize, Serialize};
use torrust_metrics::metric_collection::MetricCollection;

/// Tracker statistics response for the `GET /api/v1/stats` endpoint.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Stats {
    // Torrent metrics
    /// Total number of torrents.
    pub torrents: u64,
    /// Total number of seeders for all torrents.
    pub seeders: u64,
    /// Total number of peers that have ever completed downloading for all torrents.
    pub completed: u64,
    /// Total number of leechers for all torrents.
    pub leechers: u64,

    // Protocol metrics
    /// Total number of TCP (HTTP tracker) connections from IPv4 peers.
    pub tcp4_connections_handled: u64,
    /// Total number of TCP (HTTP tracker) `announce` requests from IPv4 peers.
    pub tcp4_announces_handled: u64,
    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv4 peers.
    pub tcp4_scrapes_handled: u64,
    /// Total number of TCP (HTTP tracker) connections from IPv6 peers.
    pub tcp6_connections_handled: u64,
    /// Total number of TCP (HTTP tracker) `announce` requests from IPv6 peers.
    pub tcp6_announces_handled: u64,
    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv6 peers.
    pub tcp6_scrapes_handled: u64,

    // UDP
    /// Total number of UDP (UDP tracker) requests aborted.
    pub udp_requests_aborted: u64,
    /// Total number of UDP (UDP tracker) requests banned.
    pub udp_requests_banned: u64,
    /// Total number of IPs banned for UDP (UDP tracker) requests.
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
    /// Total number of UDP (UDP tracker) errors handled from IPv4 peers.
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
    /// Total number of UDP (UDP tracker) errors handled from IPv6 peers.
    pub udp6_errors_handled: u64,
}

/// Extendable metrics response for the `GET /api/v1/metrics` endpoint.
///
/// Contains structured labeled metrics that can be serialized to JSON
/// or Prometheus format.
#[derive(Serialize, Debug, PartialEq)]
pub struct LabeledStats {
    /// The labeled metrics collection from all tracker subsystems.
    pub metrics: MetricCollection,
}
