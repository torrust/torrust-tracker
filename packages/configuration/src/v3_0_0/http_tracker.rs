//! HTTP tracker instance configuration for schema v3.
//!
//! **Field type convention**: use typed newtypes for fields with domain constraints —
//! not `String` or other unvalidated primitives. See [`crate::v3_0_0::public_url`].
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::v3_0_0::network::Network;
use crate::v3_0_0::public_url::HttpUrl;
use crate::v3_0_0::tls::TlsConfig;

/// Configuration for each HTTP tracker.
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(deny_unknown_fields)]
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

    /// The public-facing URL of this HTTP tracker instance, e.g.
    /// `"https://tracker.example.com/announce"`. Used for metrics labels,
    /// logging, and API discovery. Must use the `http://` or `https://` scheme.
    /// Optional; defaults to `None`.
    #[serde(default)]
    pub public_url: Option<HttpUrl>,

    /// Per-instance network topology and socket behavior.
    #[serde(default = "HttpTracker::default_network")]
    pub network: Network,
}

impl Default for HttpTracker {
    fn default() -> Self {
        Self {
            bind_address: Self::default_bind_address(),
            tls_config: Self::default_tls_config(),
            tracker_usage_statistics: Self::default_tracker_usage_statistics(),
            public_url: Self::default_public_url(),
            network: Self::default_network(),
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

    fn default_public_url() -> Option<HttpUrl> {
        None
    }

    fn default_network() -> Network {
        Network::default()
    }
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;

    use crate::v3_0_0::http_tracker::HttpTracker;
    use crate::v3_0_0::public_url::HttpUrl;

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

    #[test]
    fn it_should_default_public_url_to_none() {
        // Act
        let configuration = HttpTracker::default();

        // Assert
        assert!(configuration.public_url.is_none());
    }

    #[test]
    fn it_should_accept_public_url_when_scheme_is_https() {
        // Arrange
        let toml = r#"public_url = "https://tracker.example.com/announce""#;

        // Act
        let configuration: HttpTracker = toml::from_str(toml).expect("https:// public_url should deserialize");

        // Assert
        assert_eq!(
            configuration.public_url.as_ref().map(HttpUrl::as_str),
            Some("https://tracker.example.com/announce")
        );
    }

    #[test]
    fn it_should_accept_public_url_when_scheme_is_http() {
        // Arrange
        let toml = r#"public_url = "http://tracker.example.com:7070/announce""#; // DevSkim: ignore DS137138

        // Act
        let configuration: HttpTracker = toml::from_str(toml).expect("http:// public_url should deserialize");

        // Assert
        assert_eq!(
            configuration.public_url.as_ref().map(HttpUrl::as_str),
            Some("http://tracker.example.com:7070/announce") // DevSkim: ignore DS137138
        );
    }

    #[test]
    fn it_should_reject_public_url_when_scheme_is_udp() {
        // Arrange
        let toml = r#"public_url = "udp://tracker.example.com:6969""#;

        // Act
        let result = toml::from_str::<HttpTracker>(toml);

        // Assert
        assert!(
            result.is_err(),
            "udp:// scheme should be rejected for HTTP tracker public_url"
        );
    }

    #[test]
    fn it_should_reject_public_url_when_url_is_malformed() {
        // Arrange
        let toml = r#"public_url = "not-a-url""#;

        // Act
        let result = toml::from_str::<HttpTracker>(toml);

        // Assert
        assert!(
            result.is_err(),
            "malformed URL should be rejected for HTTP tracker public_url"
        );
    }
}
