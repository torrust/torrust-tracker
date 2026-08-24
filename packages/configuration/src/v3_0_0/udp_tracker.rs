//! UDP tracker instance configuration for schema v3.
//!
//! **Field type convention**: use typed newtypes for fields with domain constraints —
//! not `String` or other unvalidated primitives. See [`crate::v3_0_0::public_url`].
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::v3_0_0::network::Network;
use crate::v3_0_0::public_url::UdpUrl;

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

    /// The public-facing URL of this UDP tracker instance, e.g.
    /// `"udp://tracker.example.com:6969"`. Used for metrics labels, logging,
    /// and API discovery. Must use the `udp://` scheme. Optional; defaults to `None`.
    #[serde(default)]
    pub public_url: Option<UdpUrl>,

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
            public_url: Self::default_public_url(),
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

    fn default_public_url() -> Option<UdpUrl> {
        None
    }

    fn default_network() -> Network {
        Network::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::v3_0_0::public_url::UdpUrl;
    use crate::v3_0_0::udp_tracker::UdpTracker;

    #[test]
    fn it_should_default_public_url_to_none() {
        // Act
        let configuration = UdpTracker::default();

        // Assert
        assert!(configuration.public_url.is_none());
    }

    #[test]
    fn it_should_accept_public_url_when_scheme_is_udp() {
        // Arrange
        let toml = r#"public_url = "udp://tracker.example.com:6969""#;

        // Act
        let configuration: UdpTracker = toml::from_str(toml).expect("udp:// public_url should deserialize");

        // Assert
        assert_eq!(
            configuration.public_url.as_ref().map(UdpUrl::as_str),
            Some("udp://tracker.example.com:6969")
        );
    }

    #[test]
    fn it_should_reject_public_url_when_scheme_is_https() {
        // Arrange
        let toml = r#"public_url = "https://tracker.example.com/announce""#;

        // Act
        let result = toml::from_str::<UdpTracker>(toml);

        // Assert
        assert!(
            result.is_err(),
            "https:// scheme should be rejected for UDP tracker public_url"
        );
    }

    #[test]
    fn it_should_reject_public_url_when_url_is_malformed() {
        // Arrange
        let toml = r#"public_url = "not-a-url""#;

        // Act
        let result = toml::from_str::<UdpTracker>(toml);

        // Assert
        assert!(result.is_err(), "malformed URL should be rejected for UDP tracker public_url");
    }
}
