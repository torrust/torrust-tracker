use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::TslConfig;

pub type AccessTokens = HashMap<String, String>;

/// Configuration for the HTTP API.
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct HttpApi {
    /// Weather the HTTP API is enabled or not.
    pub enabled: bool,
    /// The address the tracker will bind to.
    /// The format is `ip:port`, for example `0.0.0.0:6969`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    pub bind_address: SocketAddr,
    /// Weather the HTTP API will use SSL or not.
    pub ssl_enabled: bool,
    /// TSL config. Only used if `ssl_enabled` is true.
    #[serde(flatten)]
    pub tsl_config: TslConfig,
    /// Access tokens for the HTTP API. The key is a label identifying the
    /// token and the value is the token itself. The token is used to
    /// authenticate the user. All tokens are valid for all endpoints and have
    /// all permissions.
    pub access_tokens: AccessTokens,
}

impl Default for HttpApi {
    fn default() -> Self {
        Self {
            enabled: true,
            bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1212),
            ssl_enabled: false,
            tsl_config: TslConfig::default(),
            access_tokens: [(String::from("admin"), String::from("MyAccessToken"))]
                .iter()
                .cloned()
                .collect(),
        }
    }
}

impl HttpApi {
    pub fn override_admin_token(&mut self, api_admin_token: &str) {
        self.access_tokens.insert("admin".to_string(), api_admin_token.to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::tracker_api::HttpApi;

    #[test]
    fn http_api_configuration_should_check_if_it_contains_a_token() {
        let configuration = HttpApi::default();

        assert!(configuration.access_tokens.values().any(|t| t == "MyAccessToken"));
        assert!(!configuration.access_tokens.values().any(|t| t == "NonExistingToken"));
    }
}
