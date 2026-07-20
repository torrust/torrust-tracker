use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::v3_0_0::tls::TlsConfig;

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

    /// TLS config.
    #[serde(default = "HttpTracker::default_tls_config")]
    pub tls_config: Option<TlsConfig>,

    /// Whether the tracker should collect statistics about tracker usage.
    #[serde(default = "HttpTracker::default_tracker_usage_statistics")]
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
    #[serde(default = "HttpTracker::default_ipv6_v6only")]
    pub ipv6_v6only: bool,
}

impl Default for HttpTracker {
    fn default() -> Self {
        Self {
            bind_address: Self::default_bind_address(),
            tls_config: Self::default_tls_config(),
            tracker_usage_statistics: Self::default_tracker_usage_statistics(),
            ipv6_v6only: Self::default_ipv6_v6only(),
        }
    }
}

impl HttpTracker {
    fn default_bind_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 7070)
    }

    fn default_tls_config() -> Option<TlsConfig> {
        None
    }

    fn default_tracker_usage_statistics() -> bool {
        false
    }

    fn default_ipv6_v6only() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;

    use crate::v3_0_0::http_tracker::HttpTracker;

    #[test]
    fn tls_config_should_deserialize_from_corrected_key() {
        let configuration: HttpTracker = toml::from_str(
            r#"
                [tls_config]
                ssl_cert_path = "certificate.pem"
                ssl_key_path = "private-key.pem"
            "#,
        )
        .expect("the corrected v3 TLS configuration should deserialize");

        let tls_config = configuration.tls_config.expect("TLS configuration should be present");

        assert_eq!(tls_config.ssl_cert_path, Utf8PathBuf::from("certificate.pem"));
        assert_eq!(tls_config.ssl_key_path, Utf8PathBuf::from("private-key.pem"));
    }
}
