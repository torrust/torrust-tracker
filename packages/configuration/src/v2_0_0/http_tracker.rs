use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::TslConfig;

/// Configuration for each HTTP tracker.
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct HttpTracker {
    /// The address the tracker will bind to.
    /// The format is `ip:port`, for example `0.0.0.0:6969`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    #[serde(default = "HttpTracker::default_bind_address")]
    pub bind_address: SocketAddr,

    /// TSL config.
    #[serde(default = "HttpTracker::default_tsl_config")]
    pub tsl_config: Option<TslConfig>,

    /// Weather the tracker should collect statistics about tracker usage.
    #[serde(default = "HttpTracker::default_tracker_usage_statistics")]
    pub tracker_usage_statistics: bool,

    /// Whether to set `IPV6_V6ONLY=1` on IPv6 sockets.
    ///
    /// When `true` (IPv6-only), the tracker must also bind an IPv4 socket
    /// (e.g. `0.0.0.0:<port>`) to accept IPv4 connections.
    /// When `false` (default, dual-stack), a single `[::]:<port>` socket
    /// accepts both IPv4 and IPv6 clients (IPv4 clients appear as
    /// IPv4-mapped IPv6 addresses `::ffff:<ipv4>`).
    ///
    /// > **Platform note**: On OpenBSD, `IPV6_V6ONLY` cannot be disabled;
    /// > setting this to `false` will cause a runtime error.
    #[serde(default = "HttpTracker::default_ipv6_v6only")]
    pub ipv6_v6only: bool,
}

impl Default for HttpTracker {
    fn default() -> Self {
        Self {
            bind_address: Self::default_bind_address(),
            tsl_config: Self::default_tsl_config(),
            tracker_usage_statistics: Self::default_tracker_usage_statistics(),
            ipv6_v6only: Self::default_ipv6_v6only(),
        }
    }
}

impl HttpTracker {
    fn default_bind_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 7070)
    }

    fn default_tsl_config() -> Option<TslConfig> {
        None
    }

    fn default_tracker_usage_statistics() -> bool {
        false
    }

    fn default_ipv6_v6only() -> bool {
        false
    }
}
