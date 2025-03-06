/// Metrics collected by the tracker.
///
/// - Number of connections handled
/// - Number of `announce` requests handled
/// - Number of `scrape` request handled
///
/// These metrics are collected for each connection type: UDP and HTTP
/// and also for each IP version used by the peers: IPv4 and IPv6.
#[derive(Debug, PartialEq, Default)]
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
}
