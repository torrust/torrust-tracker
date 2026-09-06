//! Health-check API configuration for schema v3.
//!
//! **Field type convention**: use typed newtypes for fields with domain constraints —
//! not `String` or other unvalidated primitives. See [`crate::v3_0_0::public_url`].
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Configuration for the Health Check API.
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HealthCheckApi {
    /// The address the API will bind to.
    /// The format is `ip:port`, for example `127.0.0.1:1313`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    #[serde(default = "HealthCheckApi::default_bind_address")]
    pub bind_address: SocketAddr,
}

impl Default for HealthCheckApi {
    fn default() -> Self {
        Self {
            bind_address: Self::default_bind_address(),
        }
    }
}

impl HealthCheckApi {
    const fn default_bind_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1313)
    }
}
