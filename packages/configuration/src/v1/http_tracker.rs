use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::TslConfig;

/// Configuration for each HTTP tracker.
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct HttpTracker {
    /// Weather the HTTP tracker is enabled or not.
    #[serde(default = "HttpTracker::default_enabled")]
    pub enabled: bool,
    /// The address the tracker will bind to.
    /// The format is `ip:port`, for example `0.0.0.0:6969`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    #[serde(default = "HttpTracker::default_bind_address")]
    pub bind_address: SocketAddr,
    /// Weather the HTTP tracker will use SSL or not.
    #[serde(default = "HttpTracker::default_ssl_enabled")]
    pub ssl_enabled: bool,
    /// TSL config. Only used if `ssl_enabled` is true.
    #[serde(flatten)]
    #[serde(default = "TslConfig::default")]
    pub tsl_config: TslConfig,
}

impl Default for HttpTracker {
    fn default() -> Self {
        Self {
            enabled: Self::default_enabled(),
            bind_address: Self::default_bind_address(),
            ssl_enabled: Self::default_ssl_enabled(),
            tsl_config: TslConfig::default(),
        }
    }
}

impl HttpTracker {
    fn default_enabled() -> bool {
        false
    }

    fn default_bind_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 7070)
    }

    fn default_ssl_enabled() -> bool {
        false
    }
}
