//! Validated configuration for the Health Check Api service.
//!
//! [``crate::HealthCheckApi``] is a DTO containing the parsed data from the toml
//! file.
//!
//! This configuration is a first level of validation that can be perform
//! statically without running the service.
//!
//! For example, the `bind_address` must be a valid socket address.
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::HealthCheckApi;

/// Errors that can occur when validating the plain configuration.
#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    /// Invalid bind address.
    #[error("Invalid bind address, got: {bind_address}")]
    InvalidBindAddress { bind_address: String },
}

/// Configuration for each HTTP tracker.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Config {
    bind_address: String, // todo: use SocketAddr
}

impl Config {
    #[must_use]
    pub fn bind_address(&self) -> &str {
        &self.bind_address
    }
}

impl TryFrom<HealthCheckApi> for Config {
    type Error = ValidationError;

    fn try_from(config: HealthCheckApi) -> Result<Self, Self::Error> {
        let socket_addr = match config.bind_address.parse::<SocketAddr>() {
            Ok(socket_addr) => socket_addr,
            Err(_err) => {
                return Err(ValidationError::InvalidBindAddress {
                    bind_address: config.bind_address,
                })
            }
        };

        Ok(Self {
            bind_address: socket_addr.to_string(),
        })
    }
}

impl From<Config> for HealthCheckApi {
    fn from(config: Config) -> Self {
        Self {
            bind_address: config.bind_address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_an_error_when_the_bind_address_is_not_a_valid_socket_address() {
        let plain_config = HealthCheckApi {
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
