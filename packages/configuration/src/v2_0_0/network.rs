use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Network {
    /// The external IP address of the tracker. If the client is using a
    /// loopback IP address, this IP address will be used instead. If the peer
    /// is using a loopback IP address, the tracker assumes that the peer is
    /// in the same network as the tracker and will use the tracker's IP
    /// address instead.
    #[serde(default = "Network::default_external_ip")]
    pub external_ip: Option<IpAddr>,

    /// Weather the tracker is behind a reverse proxy or not.
    /// If the tracker is behind a reverse proxy, the `X-Forwarded-For` header
    /// sent from the proxy will be used to get the client's IP address.
    #[serde(default = "Network::default_on_reverse_proxy")]
    pub on_reverse_proxy: bool,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            external_ip: Self::default_external_ip(),
            on_reverse_proxy: Self::default_on_reverse_proxy(),
        }
    }
}

impl Network {
    #[allow(clippy::unnecessary_wraps)]
    fn default_external_ip() -> Option<IpAddr> {
        Some(IpAddr::V4(Ipv4Addr::UNSPECIFIED))
    }

    fn default_on_reverse_proxy() -> bool {
        false
    }
}
