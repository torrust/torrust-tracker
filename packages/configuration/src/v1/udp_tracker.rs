use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct UdpTracker {
    /// Weather the UDP tracker is enabled or not.
    pub enabled: bool,
    /// The address the tracker will bind to.
    /// The format is `ip:port`, for example `0.0.0.0:6969`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    pub bind_address: SocketAddr,
}
impl Default for UdpTracker {
    fn default() -> Self {
        Self {
            enabled: false,
            bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 6969),
        }
    }
}
