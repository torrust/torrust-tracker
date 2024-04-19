//! Validated configuration for the UDP Tracker service.
//!
//! [``crate::UdpTracker``] is a DTO containing the parsed data from the toml
//! file.
//!
//! This configuration is a first level of validation that can be perform
//! statically without running the service.
//!
//! For example, the `bind_address` must be a valid socket address.
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::UdpTracker;

/// Errors that can occur when validating the plain configuration.
#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    /// Invalid bind address.
    #[error("invalid bind address, got: {bind_address}")]
    InvalidBindAddress { bind_address: String },
}

/// Configuration for each HTTP tracker.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Config {
    enabled: bool,
    bind_address: String, // todo: use SocketAddr
}

impl Config {
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub fn bind_address(&self) -> &str {
        &self.bind_address
    }
}

impl TryFrom<UdpTracker> for Config {
    type Error = ValidationError;

    fn try_from(config: UdpTracker) -> Result<Self, Self::Error> {
        let socket_addr = match config.bind_address.parse::<SocketAddr>() {
            Ok(socket_addr) => socket_addr,
            Err(_err) => {
                return Err(ValidationError::InvalidBindAddress {
                    bind_address: config.bind_address,
                })
            }
        };

        Ok(Self {
            enabled: config.enabled,
            bind_address: socket_addr.to_string(),
        })
    }
}

impl From<Config> for UdpTracker {
    fn from(config: Config) -> Self {
        Self {
            enabled: config.enabled,
            bind_address: config.bind_address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_an_error_when_the_bind_address_is_not_a_valid_socket_address() {
        let plain_config = UdpTracker {
            enabled: true,
            bind_address: "300.300.300.300:7070".to_string(),
        };

        assert_eq!(
            Config::try_from(plain_config),
            Err(ValidationError::InvalidBindAddress {
                bind_address: "300.300.300.300:7070".to_string()
            })
        );
    }
}
