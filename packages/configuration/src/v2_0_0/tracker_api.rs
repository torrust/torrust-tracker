use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub use crate::AccessTokens;
use crate::TslConfig;

/// Configuration for the HTTP API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[serde(
        default = "HttpApi::default_access_tokens",
        serialize_with = "serialize_access_tokens_for_output"
    )]
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
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1212)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn default_tsl_config() -> Option<TslConfig> {
        None
    }

    fn default_access_tokens() -> AccessTokens {
        [].iter().cloned().collect()
    }

    pub fn add_token(&mut self, key: &str, token: &str) {
        self.access_tokens.insert(key.to_string(), SecretString::from(token));
    }

    pub(crate) fn redact_access_tokens_for_output(&mut self) {
        for token in self.access_tokens.values_mut() {
            *token = SecretString::from("***");
        }
    }

    pub(crate) fn serialize_access_tokens_for_toml(&self) -> toml::Table {
        self.access_tokens
            .iter()
            .map(|(label, token)| (label.clone(), toml::Value::String(token.expose_secret().to_string())))
            .collect()
    }
}

fn serialize_access_tokens_for_output<S>(access_tokens: &AccessTokens, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    access_tokens
        .keys()
        .map(|label| (label, "***"))
        .collect::<std::collections::HashMap<_, _>>()
        .serialize(serializer)
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

        let formatted = format!("{configuration:?}");

        assert!(formatted.contains("SecretBox<str>([REDACTED])"));
        assert!(!formatted.contains("MyAccessToken"));
    }

    #[test]
    fn http_api_tokens_should_deserialize_and_serialize_with_toml_syntax() {
        let token = "v2-token-only-for-serialization-test";
        let configuration: HttpApi = toml::from_str(&format!("[access_tokens]\nadmin = \"{token}\"\n"))
            .expect("HTTP API tokens should deserialize from TOML");

        let serialized = serde_json::to_string(&configuration).expect("HTTP API tokens should serialize to JSON safely");

        assert!(!serialized.contains(token));
        assert!(serialized.contains("***"));
    }
}
