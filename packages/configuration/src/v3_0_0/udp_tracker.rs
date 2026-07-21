use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::v3_0_0::network::Network;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(deny_unknown_fields)]
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

    /// The maximum number of connection ID errors per IP before the client is
    /// banned. Default is `10`.
    #[serde(default = "UdpTracker::default_max_connection_id_errors_per_ip")]
    pub max_connection_id_errors_per_ip: u32,

    /// Per-instance network topology and socket behavior.
    #[serde(default = "UdpTracker::default_network")]
    pub network: Network,
}
impl Default for UdpTracker {
    fn default() -> Self {
        Self {
            bind_address: Self::default_bind_address(),
            cookie_lifetime: Self::default_cookie_lifetime(),
            tracker_usage_statistics: Self::default_tracker_usage_statistics(),
            max_connection_id_errors_per_ip: Self::default_max_connection_id_errors_per_ip(),
            network: Self::default_network(),
        }
    }
}

impl UdpTracker {
    fn default_bind_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 6969)
    }

    fn default_cookie_lifetime() -> Duration {
        Duration::from_secs(120)
    }

    fn default_tracker_usage_statistics() -> bool {
        false
    }

    fn default_max_connection_id_errors_per_ip() -> u32 {
        10
    }

    fn default_network() -> Network {
        Network::default()
    }
}
