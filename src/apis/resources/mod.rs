use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::str::FromStr;

use crate::errors::settings::ServiceSettingsError;
use crate::settings::{Service, ServiceProtocol};
use crate::{check_field_is_not_empty, check_field_is_not_none};

pub mod auth_key;
pub mod peer;
pub mod stats;
pub mod torrent;

pub type ApiTokens = BTreeMap<String, String>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ApiServiceSettings {
    pub id: String,
    pub enabled: bool,
    pub display_name: String,
    pub socket: SocketAddr,
    pub access_tokens: ApiTokens,
}

impl Default for ApiServiceSettings {
    fn default() -> Self {
        let mut access_tokens = BTreeMap::new();
        access_tokens.insert("admin".to_string(), "password".to_string());

        Self {
            id: "default_api".to_string(),
            enabled: false,
            display_name: "HTTP API (default)".to_string(),
            socket: SocketAddr::from_str("127.0.0.1:1212").unwrap(),
            access_tokens,
        }
    }
}

impl TryFrom<(&String, &Service)> for ApiServiceSettings {
    type Error = ServiceSettingsError;

    fn try_from(value: (&String, &Service)) -> Result<Self, Self::Error> {
        check_field_is_not_none!(value.1 => ServiceSettingsError;
            enabled, service);

        if value.1.service.unwrap() != ServiceProtocol::Api {
            return Err(ServiceSettingsError::WrongService {
                field: "service".to_string(),
                expected: ServiceProtocol::Api,
                found: value.1.service.unwrap(),
                data: value.1.into(),
            });
        }

        check_field_is_not_empty!(value.1 => ServiceSettingsError;
                display_name: String);

        Ok(Self {
            id: value.0.clone(),
            enabled: value.1.enabled.unwrap(),
            display_name: value.1.display_name.clone().unwrap(),
            socket: value.1.get_socket()?,
            access_tokens: value.1.get_api_tokens()?,
        })
    }
}
