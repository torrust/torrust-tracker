use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::v3_0_0::tls::TlsConfig;

pub type AccessTokens = HashMap<String, String>;

/// Configuration for the HTTP API.
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
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
    #[serde(default = "HttpApi::default_access_tokens")]
    pub access_tokens: AccessTokens,
}

impl Default for HttpApi {
    fn default() -> Self {
        Self {
            bind_address: Self::default_bind_address(),
            tls_config: Self::default_tls_config(),
            access_tokens: Self::default_access_tokens(),
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
        HashMap::new()
    }

    pub fn add_token(&mut self, key: &str, token: &str) {
        self.access_tokens.insert(key.to_string(), token.to_string());
    }

    pub fn mask_secrets(&mut self) {
        for token in self.access_tokens.values_mut() {
            *token = "***".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;

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

        assert!(configuration.access_tokens.values().any(|t| t == "MyAccessToken"));
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
}
