use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::TslConfig;

/// Configuration for each HTTP tracker.
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct HttpTracker {
    /// Weather the HTTP tracker is enabled or not.
    pub enabled: bool,
    /// The address the tracker will bind to.
    /// The format is `ip:port`, for example `0.0.0.0:6969`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    pub bind_address: SocketAddr,
    /// Weather the HTTP tracker will use SSL or not.
    pub ssl_enabled: bool,
    /// TSL config. Only used if `ssl_enabled` is true.
    #[serde(flatten)]
    pub tsl_config: TslConfig,
}

impl Default for HttpTracker {
    fn default() -> Self {
        Self {
            enabled: false,
            bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 7070),
            ssl_enabled: false,
            tsl_config: TslConfig::default(),
        }
    }
}
