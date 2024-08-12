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
    /// The address the tracker will bind to.
    /// The format is `ip:port`, for example `0.0.0.0:6969`. If you want to
    /// listen to all interfaces, use `0.0.0.0`. If you want the operating
    /// system to choose a random port, use port `0`.
    #[serde(default = "HttpApi::default_bind_address")]
    pub bind_address: SocketAddr,

    /// TSL config. Only used if `ssl_enabled` is true.
    #[serde(default = "HttpApi::default_tsl_config")]
    pub tsl_config: Option<TslConfig>,

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
            tsl_config: Self::default_tsl_config(),
            access_tokens: Self::default_access_tokens(),
        }
    }
}

impl HttpApi {
    fn default_bind_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1212)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn default_tsl_config() -> Option<TslConfig> {
        None
    }

    fn default_access_tokens() -> AccessTokens {
        [].iter().cloned().collect()
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
    use crate::v2_0_0::tracker_api::HttpApi;

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
}
