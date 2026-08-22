//! HTTP (REST) API configuration for schema v3.
//!
//! **Field type convention**: use typed newtypes for fields with domain constraints —
//! not `String` or other unvalidated primitives. See [`crate::v3_0_0::public_url`].
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub use crate::AccessTokens;
use crate::v3_0_0::public_url::HttpUrl;
use crate::v3_0_0::tls::TlsConfig;

/// Configuration for the HTTP API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HttpApi {
    /// The address the tracker will bind to.
    /// The format is `ip:port`, for example `0.0.0.0:6969`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    #[serde(default = "HttpApi::default_bind_address")]
    pub bind_address: SocketAddr,

    /// TLS config. Provide this section to enable TLS for the HTTP API.
    #[serde(default = "HttpApi::default_tls_config")]
    pub tls_config: Option<TlsConfig>,

    /// Access tokens for the HTTP API. The key is a label identifying the
    /// token and the value is the token itself. The token is used to
    /// authenticate the user. All tokens are valid for all endpoints and have
    /// all permissions.
    #[serde(default = "HttpApi::default_access_tokens", serialize_with = "serialize_access_tokens")]
    pub access_tokens: AccessTokens,

    /// The public-facing URL of the REST API, e.g.
    /// `"https://api.tracker.example.com"`. Used for service discovery and
    /// logging. Must use the `http://` or `https://` scheme. Optional; defaults
    /// to `None`.
    #[serde(default)]
    pub public_url: Option<HttpUrl>,
}

impl Default for HttpApi {
    fn default() -> Self {
        Self {
            bind_address: Self::default_bind_address(),
            tls_config: Self::default_tls_config(),
            access_tokens: Self::default_access_tokens(),
            public_url: Self::default_public_url(),
        }
    }
}

impl HttpApi {
    fn default_bind_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1212)
    }

    fn default_tls_config() -> Option<TlsConfig> {
        None
    }

    fn default_access_tokens() -> AccessTokens {
        AccessTokens::new()
    }

    fn default_public_url() -> Option<HttpUrl> {
        None
    }

    pub fn add_token(&mut self, key: &str, token: &str) {
        self.access_tokens.insert(key.to_string(), SecretString::from(token));
    }

    pub(crate) fn redact_access_tokens_for_output(&mut self) {
        for token in self.access_tokens.values_mut() {
            *token = SecretString::from("***");
        }
    }
}

fn serialize_access_tokens<S>(access_tokens: &AccessTokens, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    access_tokens
        .iter()
        .map(|(label, token)| (label, token.expose_secret()))
        .collect::<std::collections::HashMap<_, _>>()
        .serialize(serializer)
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;

    use crate::v3_0_0::public_url::HttpUrl;
    use crate::v3_0_0::tracker_api::HttpApi;

    #[test]
    fn default_http_api_configuration_should_not_contains_any_token() {
        let configuration = HttpApi::default();

        assert_eq!(configuration.access_tokens.values().len(), 0);
    }

    #[test]
    fn http_api_configuration_should_allow_adding_tokens() {
        let mut configuration = HttpApi::default();

        configuration.add_token("admin", "MyAccessToken");

        let formatted = format!("{configuration:?}");

        assert!(formatted.contains("SecretBox<str>([REDACTED])"));
        assert!(!formatted.contains("MyAccessToken"));
    }

    #[test]
    fn http_api_tokens_should_deserialize_and_serialize_with_toml_syntax() {
        let token = "v3-token-only-for-serialization-test";
        let configuration: HttpApi = toml::from_str(&format!("[access_tokens]\nadmin = \"{token}\"\n"))
            .expect("HTTP API tokens should deserialize from TOML");

        let serialized = toml::to_string(&configuration).expect("HTTP API tokens should serialize to TOML");

        assert!(serialized.contains("[access_tokens]"));
        assert!(serialized.contains(token));
    }

    #[test]
    fn tls_config_should_deserialize_from_corrected_key() {
        let configuration: HttpApi = toml::from_str(
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
        let configuration = HttpApi::default();

        // Assert
        assert!(configuration.public_url.is_none());
    }

    #[test]
    fn it_should_accept_public_url_when_scheme_is_https() {
        // Arrange
        let toml = r#"public_url = "https://api.tracker.example.com/""#;

        // Act
        let configuration: HttpApi = toml::from_str(toml).expect("https:// public_url should deserialize for HttpApi");

        // Assert
        assert_eq!(
            configuration.public_url.as_ref().map(HttpUrl::as_str),
            Some("https://api.tracker.example.com/")
        );
    }

    #[test]
    fn it_should_reject_public_url_when_scheme_is_udp() {
        // Arrange
        let toml = r#"public_url = "udp://tracker.example.com:6969""#;

        // Act
        let result = toml::from_str::<HttpApi>(toml);

        // Assert
        assert!(result.is_err(), "udp:// scheme should be rejected for HttpApi public_url");
    }
}
