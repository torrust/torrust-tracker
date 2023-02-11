use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::panic::Location;
use std::path::Path;
use std::str::FromStr;

use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use super::error::Error;
use crate::config::HttpApi;
use crate::errors::settings::ServiceSettingsError;
use crate::settings::{Service, ServiceProtocol};
use crate::tracker::services::common::{Tls, TlsBuilder};
use crate::{check_field_is_not_empty, check_field_is_not_none};

pub type ApiTokens = BTreeMap<String, String>;

#[derive(Builder, Getters, Default, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
#[builder(default, pattern = "immutable")]
pub struct Settings {
    #[builder(setter(into), default = "\"default_api\".to_string()")]
    #[getter(rename = "get_id")]
    id: String,
    #[builder(default = "false")]
    #[getter(rename = "is_enabled")]
    enabled: bool,
    #[builder(setter(into), default = "\"HTTP API (default)\".to_string()")]
    #[getter(rename = "get_display_name")]
    display_name: String,
    #[builder(default = "Some(SocketAddr::from_str(\"127.0.0.1:1212\").unwrap())")]
    #[getter(rename = "get_socket")]
    socket: Option<SocketAddr>,
    #[builder(default = "self.api_token_default()")]
    #[getter(rename = "get_access_tokens")]
    access_tokens: ApiTokens,
    #[getter(rename = "get_tls_settings")]
    tls: Option<Tls>,
}

impl SettingsBuilder {
    // Private helper method that will set the default database path if the database is Sqlite.
    #[allow(clippy::unused_self)]
    fn api_token_default(&self) -> ApiTokens {
        let mut access_tokens = BTreeMap::new();
        access_tokens.insert("admin".to_string(), "password".to_string());
        access_tokens
    }
}

impl TryFrom<&HttpApi> for Settings {
    type Error = Error;

    fn try_from(api: &HttpApi) -> Result<Self, Self::Error> {
        let tls = if api.ssl_enabled {
            let cert = Path::new(match &api.ssl_cert_path {
                Some(p) => p,
                None => {
                    return Err(Error::ParseConfig {
                        location: Location::caller(),
                        message: "ssl_cert_path is none and tls is enabled!".to_string(),
                    })
                }
            });

            let key = Path::new(match &api.ssl_key_path {
                Some(p) => p,
                None => {
                    return Err(Error::ParseConfig {
                        location: Location::caller(),
                        message: "ssl_key_path is none and tls is enabled!".to_string(),
                    })
                }
            });

            Some(
                TlsBuilder::default()
                    .certificate_file_path(cert.into())
                    .key_file_path(key.into())
                    .build()
                    .expect("failed to build tls settings"),
            )
        } else {
            None
        };

        Ok(SettingsBuilder::default()
            .id("imported_api")
            .enabled(api.enabled)
            .display_name("Imported API")
            .socket(SocketAddr::from_str(api.bind_address.as_str()).ok())
            .access_tokens(api.access_tokens.clone())
            .tls(tls)
            .build()
            .expect("failed to import settings"))
    }
}

impl TryFrom<(&String, &Service)> for Settings {
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
            socket: Some(value.1.get_socket()?),
            access_tokens: value.1.get_api_tokens()?,
            tls: value.1.get_tls()?,
        })
    }
}
