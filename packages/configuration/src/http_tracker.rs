//! Validated configuration for the HTTP Tracker service.
//!
//! [``crate::HttpTracker``] is a DTO containing the parsed data from the toml
//! file.
//!
//! This configuration is a first level of validation that can be perform
//! statically without running the service.
//!
//! For example, if SSL is enabled you must provide the certificate path. That
//! can be validated. However, this validation does not check if the
//! certificate is valid.
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::HttpTracker;

/// Errors that can occur when validating the plain configuration.
#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    /// Invalid bind address.
    #[error("Invalid bind address, got: {bind_address}")]
    InvalidBindAddress { bind_address: String },
    /// Missing SSL cert path.
    #[error("missing SSL cert path")]
    MissingSslCertPath,
    /// Missing SSL key path.
    #[error("missing SSL key path")]
    MissingSslKeyPath,
}

/// Configuration for each HTTP tracker.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Config {
    enabled: bool,
    bind_address: String, // todo: use SocketAddr
    ssl_enabled: bool,
    ssl_cert_path: Option<String>, // todo: use Path
    ssl_key_path: Option<String>,  // todo: use Path
}

impl Config {
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl TryFrom<HttpTracker> for Config {
    type Error = ValidationError;

    fn try_from(config: HttpTracker) -> Result<Self, Self::Error> {
        let socket_addr = match config.bind_address.parse::<SocketAddr>() {
            Ok(socket_addr) => socket_addr,
            Err(_err) => {
                return Err(ValidationError::InvalidBindAddress {
                    bind_address: config.bind_address,
                })
            }
        };

        if config.ssl_enabled {
            match config.ssl_cert_path.clone() {
                Some(ssl_cert_path) => {
                    if ssl_cert_path.is_empty() {
                        Err(ValidationError::MissingSslCertPath)
                    } else {
                        Ok(())
                    }
                }
                None => Err(ValidationError::MissingSslCertPath),
            }?;

            match config.ssl_key_path.clone() {
                Some(ssl_key_path) => {
                    if ssl_key_path.is_empty() {
                        Err(ValidationError::MissingSslKeyPath)
                    } else {
                        Ok(())
                    }
                }
                None => Err(ValidationError::MissingSslKeyPath),
            }?;
        }

        Ok(Self {
            enabled: config.enabled,
            bind_address: socket_addr.to_string(),
            ssl_enabled: config.ssl_enabled,
            ssl_cert_path: config.ssl_cert_path,
            ssl_key_path: config.ssl_key_path,
        })
    }
}

impl From<Config> for HttpTracker {
    fn from(config: Config) -> Self {
        Self {
            enabled: config.enabled,
            bind_address: config.bind_address,
            ssl_enabled: config.ssl_enabled,
            ssl_cert_path: config.ssl_cert_path,
            ssl_key_path: config.ssl_key_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_an_error_when_the_bind_address_is_not_a_valid_socket_address() {
        let plain_config = HttpTracker {
            enabled: true,
            bind_address: "300.300.300.300:7070".to_string(),
            ssl_enabled: true,
            ssl_cert_path: None,
            ssl_key_path: Some("./localhost.key".to_string()),
        };

        assert_eq!(
            Config::try_from(plain_config),
            Err(ValidationError::InvalidBindAddress {
                bind_address: "300.300.300.300:7070".to_string()
            })
        );
    }

    mod when_ssl_is_enabled {
        use crate::http_tracker::{Config, ValidationError};
        use crate::HttpTracker;

        #[test]
        fn it_should_return_an_error_when_ssl_is_enabled_but_the_cert_path_is_not_provided() {
            let plain_config = HttpTracker {
                enabled: true,
                bind_address: "127.0.0.1:7070".to_string(),
                ssl_enabled: true,
                ssl_cert_path: None,
                ssl_key_path: Some("./localhost.key".to_string()),
            };

            assert_eq!(Config::try_from(plain_config), Err(ValidationError::MissingSslCertPath));
        }

        #[test]
        fn it_should_return_an_error_when_ssl_is_enabled_but_the_cert_path_is_empty() {
            let plain_config = HttpTracker {
                enabled: true,
                bind_address: "127.0.0.1:7070".to_string(),
                ssl_enabled: true,
                ssl_cert_path: Some(String::new()),
                ssl_key_path: Some("./localhost.key".to_string()),
            };

            assert_eq!(Config::try_from(plain_config), Err(ValidationError::MissingSslCertPath));
        }

        #[test]
        fn it_should_return_an_error_when_ssl_is_enabled_but_the_key_path_is_not_provided() {
            let plain_config = HttpTracker {
                enabled: true,
                bind_address: "127.0.0.1:7070".to_string(),
                ssl_enabled: true,
                ssl_cert_path: Some("./localhost.crt".to_string()),
                ssl_key_path: None,
            };

            assert_eq!(Config::try_from(plain_config), Err(ValidationError::MissingSslKeyPath));
        }

        #[test]
        fn it_should_return_an_error_when_ssl_is_enabled_but_the_key_path_is_empty() {
            let plain_config = HttpTracker {
                enabled: true,
                bind_address: "127.0.0.1:7070".to_string(),
                ssl_enabled: true,
                ssl_cert_path: Some("./localhost.crt".to_string()),
                ssl_key_path: Some(String::new()),
            };

            assert_eq!(Config::try_from(plain_config), Err(ValidationError::MissingSslKeyPath));
        }
    }
}
