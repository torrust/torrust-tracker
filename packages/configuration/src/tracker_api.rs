//! Validated configuration for the Tracker API service.
//!
//! [``crate::HttpApi``] is a DTO containing the parsed data from the toml
//! file.
//!
//! This configuration is a first level of validation that can be perform
//! statically without running the service.
//!
//! For example, if SSL is enabled you must provide the certificate path. That
//! can be validated. However, this validation does not check if the
//! certificate is valid.
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{AccessTokens, HttpApi};

/// Errors that can occur when validating the plain configuration.
#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
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
    access_tokens: AccessTokens,
}

impl Config {
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl TryFrom<HttpApi> for Config {
    type Error = ValidationError;

    fn try_from(config: HttpApi) -> Result<Self, Self::Error> {
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
            bind_address: config.bind_address,
            ssl_enabled: config.ssl_enabled,
            ssl_cert_path: config.ssl_cert_path,
            ssl_key_path: config.ssl_key_path,
            access_tokens: config.access_tokens,
        })
    }
}

impl From<Config> for HttpApi {
    fn from(config: Config) -> Self {
        Self {
            enabled: config.enabled,
            bind_address: config.bind_address,
            ssl_enabled: config.ssl_enabled,
            ssl_cert_path: config.ssl_cert_path,
            ssl_key_path: config.ssl_key_path,
            access_tokens: config.access_tokens,
        }
    }
}

#[cfg(test)]
mod tests {

    mod when_ssl_is_enabled {
        use std::collections::HashMap;

        use crate::tracker_api::{Config, ValidationError};
        use crate::HttpApi;

        #[test]
        fn it_should_return_an_error_when_ssl_is_enabled_but_the_cert_path_is_not_provided() {
            let plain_config = HttpApi {
                enabled: true,
                bind_address: "127.0.0.1:1212".to_string(),
                ssl_enabled: true,
                ssl_cert_path: None,
                ssl_key_path: Some("./localhost.key".to_string()),
                access_tokens: HashMap::new(),
            };

            assert_eq!(Config::try_from(plain_config), Err(ValidationError::MissingSslCertPath));
        }

        #[test]
        fn it_should_return_an_error_when_ssl_is_enabled_but_the_cert_path_is_empty() {
            let plain_config = HttpApi {
                enabled: true,
                bind_address: "127.0.0.1:1212".to_string(),
                ssl_enabled: true,
                ssl_cert_path: Some(String::new()),
                ssl_key_path: Some("./localhost.key".to_string()),
                access_tokens: HashMap::new(),
            };

            assert_eq!(Config::try_from(plain_config), Err(ValidationError::MissingSslCertPath));
        }

        #[test]
        fn it_should_return_an_error_when_ssl_is_enabled_but_the_key_path_is_not_provided() {
            let plain_config = HttpApi {
                enabled: true,
                bind_address: "127.0.0.1:1212".to_string(),
                ssl_enabled: true,
                ssl_cert_path: Some("./localhost.crt".to_string()),
                ssl_key_path: None,
                access_tokens: HashMap::new(),
            };

            assert_eq!(Config::try_from(plain_config), Err(ValidationError::MissingSslKeyPath));
        }

        #[test]
        fn it_should_return_an_error_when_ssl_is_enabled_but_the_key_path_is_empty() {
            let plain_config = HttpApi {
                enabled: true,
                bind_address: "127.0.0.1:1212".to_string(),
                ssl_enabled: true,
                ssl_cert_path: Some("./localhost.crt".to_string()),
                ssl_key_path: Some(String::new()),
                access_tokens: HashMap::new(),
            };

            assert_eq!(Config::try_from(plain_config), Err(ValidationError::MissingSslKeyPath));
        }
    }
}
