use std::net::SocketAddr;
use std::str::FromStr;

use crate::errors::settings::ServiceSettingsError;
use crate::settings::{Service, ServiceProtocol};
use crate::{check_field_is_not_empty, check_field_is_not_none};

pub mod connection_cookie;
pub mod error;
pub mod handlers;
pub mod request;
pub mod server;

pub type Bytes = u64;
pub type Port = u16;
pub type TransactionId = i64;

pub const MAX_PACKET_SIZE: usize = 1496;
pub const PROTOCOL_ID: i64 = 0x0417_2710_1980;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UdpServiceSettings {
    pub id: String,
    pub enabled: bool,
    pub display_name: String,
    pub socket: SocketAddr,
}

impl Default for UdpServiceSettings {
    fn default() -> Self {
        Self {
            id: "default_udp".to_string(),
            enabled: false,
            display_name: "UDP (default)".to_string(),
            socket: SocketAddr::from_str("0.0.0.0:6969").unwrap(),
        }
    }
}

impl TryFrom<(&String, &Service)> for UdpServiceSettings {
    type Error = ServiceSettingsError;

    fn try_from(value: (&String, &Service)) -> Result<Self, Self::Error> {
        check_field_is_not_none!(value.1 => ServiceSettingsError;
            enabled, service);

        if value.1.service.unwrap() != ServiceProtocol::Udp {
            return Err(ServiceSettingsError::WrongService {
                field: "service".to_string(),
                expected: ServiceProtocol::Udp,
                found: value.1.service.unwrap(),
                data: value.1.into(),
            });
        }

        check_field_is_not_empty!(value.1 => ServiceSettingsError;
                display_name: String);

        Ok(Self {
            id: value.0.to_owned(),
            enabled: value.1.enabled.unwrap(),
            display_name: value.1.display_name.to_owned().unwrap(),
            socket: value.1.get_socket()?,
        })
    }
}
