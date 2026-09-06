use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct UdpTracker {
    /// The address the tracker will bind to.
    /// The format is `ip:port`, for example `0.0.0.0:6969`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    #[serde(default = "UdpTracker::default_bind_address")]
    pub bind_address: SocketAddr,

    /// The lifetime of the server-generated connection cookie, that is passed
    /// the client as the `ConnectionId`.
    #[serde(default = "UdpTracker::default_cookie_lifetime")]
    pub cookie_lifetime: Duration,

    /// Whether the tracker should collect statistics about tracker usage.
    #[serde(default = "UdpTracker::default_tracker_usage_statistics")]
    pub tracker_usage_statistics: bool,

    /// Whether to set `IPV6_V6ONLY=1` on IPv6 sockets.
    ///
    /// When `true` (IPv6-only), the tracker must also bind an IPv4 socket
    /// (e.g. `0.0.0.0:<port>`) to accept IPv4 connections.
    /// When `false` (default), the socket option is not overridden and the
    /// OS default applies (dual-stack on Linux, IPv6-only on other platforms).
    ///
    /// > **Platform note**: On OpenBSD, `IPV6_V6ONLY` is always `1` and cannot
    /// > be disabled; setting this to `false` is a no-op.
    #[serde(default = "UdpTracker::default_ipv6_v6only")]
    pub ipv6_v6only: bool,

    /// The maximum number of connection ID errors per IP before the client is
    /// banned. Default is `10`.
    #[serde(default = "UdpTracker::default_max_connection_id_errors_per_ip")]
    pub max_connection_id_errors_per_ip: u32,
}
impl Default for UdpTracker {
    fn default() -> Self {
        Self {
            bind_address: Self::default_bind_address(),
            cookie_lifetime: Self::default_cookie_lifetime(),
            tracker_usage_statistics: Self::default_tracker_usage_statistics(),
            ipv6_v6only: Self::default_ipv6_v6only(),
            max_connection_id_errors_per_ip: Self::default_max_connection_id_errors_per_ip(),
        }
    }
}

impl UdpTracker {
    const fn default_bind_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 6969)
    }

    const fn default_cookie_lifetime() -> Duration {
        Duration::from_secs(120)
    }

    const fn default_tracker_usage_statistics() -> bool {
        false
    }

    const fn default_ipv6_v6only() -> bool {
        false
    }

    const fn default_max_connection_id_errors_per_ip() -> u32 {
        10
    }
}
