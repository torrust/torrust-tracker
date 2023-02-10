//! Tracker HTTP/HTTPS Protocol:
//!
//! Original specification in BEP 3 (section "Trackers"):
//!
//! <https://www.bittorrent.org/beps/bep_0003.html>
//!
//! Other resources:
//!
//! - <https://wiki.theory.org/BitTorrentSpecification#Tracker_HTTP.2FHTTPS_Protocol>
//! - <https://wiki.theory.org/BitTorrent_Tracker_Protocol>
//!

use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::settings::ServiceSettingsError;
use crate::settings::{Service, ServiceProtocol};
use crate::{check_field_is_not_empty, check_field_is_not_none};

pub mod axum_implementation;
pub mod percent_encoding;
pub mod warp_implementation;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Version {
    Warp,
    Axum,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HttpServiceSettings {
    pub id: String,
    pub enabled: bool,
    pub display_name: String,
    pub socket: SocketAddr,
}

impl Default for HttpServiceSettings {
    fn default() -> Self {
        Self {
            id: "default_http".to_string(),
            enabled: false,
            display_name: "HTTP (default)".to_string(),
            socket: SocketAddr::from_str("0.0.0.0:6969").unwrap(),
        }
    }
}

impl TryFrom<(&String, &Service)> for HttpServiceSettings {
    type Error = ServiceSettingsError;

    fn try_from(value: (&String, &Service)) -> Result<Self, Self::Error> {
        check_field_is_not_none!(value.1 => ServiceSettingsError;
            enabled, service);

        if value.1.service.unwrap() != ServiceProtocol::Http {
            return Err(ServiceSettingsError::WrongService {
                field: "service".to_string(),
                expected: ServiceProtocol::Http,
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
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TlsServiceSettings {
    pub id: String,
    pub enabled: bool,
    pub display_name: String,
    pub socket: SocketAddr,
    pub certificate_file_path: Box<Path>,
    pub key_file_path: Box<Path>,
}

impl Default for TlsServiceSettings {
    fn default() -> Self {
        Self {
            id: "default_http".to_string(),
            enabled: false,
            display_name: "HTTP (default)".to_string(),
            socket: SocketAddr::from_str("0.0.0.0:6969").unwrap(),
            certificate_file_path: Path::new("").into(),
            key_file_path: Path::new("").into(),
        }
    }
}

impl TryFrom<(&String, &Service)> for TlsServiceSettings {
    type Error = ServiceSettingsError;

    fn try_from(value: (&String, &Service)) -> Result<Self, Self::Error> {
        check_field_is_not_none!(value.1 => ServiceSettingsError;
            enabled, service, tls);

        if value.1.service.unwrap() != ServiceProtocol::Tls {
            return Err(ServiceSettingsError::WrongService {
                field: "service".to_string(),
                expected: ServiceProtocol::Tls,
                found: value.1.service.unwrap(),
                data: value.1.into(),
            });
        }

        check_field_is_not_empty!(value.1 => ServiceSettingsError;
                display_name: String);

        let tls = value.1.tls.clone().unwrap();

        Ok(Self {
            id: value.0.clone(),
            enabled: value.1.enabled.unwrap(),
            display_name: value.1.display_name.clone().unwrap(),
            socket: value.1.get_socket()?,

            certificate_file_path: tls
                .get_certificate_file_path()
                .map_err(|err| ServiceSettingsError::TlsSettingsError {
                    field: value.0.clone(),
                    source: err,
                    data: value.1.into(),
                })?,

            key_file_path: tls
                .get_key_file_path()
                .map_err(|err| ServiceSettingsError::TlsSettingsError {
                    field: value.0.clone(),
                    source: err,
                    data: value.1.into(),
                })?,
        })
    }
}
