use std::fmt;
use std::net::SocketAddr;

use reqwest::Url as ServiceUrl;
use serde::Deserialize;
use url;

/// It parses the configuration from a JSON format.
///
/// # Errors
///
/// Will return an error if the configuration is not valid.
///
/// # Panics
///
/// Will panic if unable to read the configuration file.
pub fn parse_from_json(json: &str) -> Result<Configuration, ConfigurationError> {
    let plain_config: PlainConfiguration = serde_json::from_str(json).map_err(ConfigurationError::JsonParseError)?;
    Configuration::try_from(plain_config)
}

/// DTO for the configuration to serialize/deserialize configuration.
///
/// Configuration does not need to be valid.
#[derive(Deserialize)]
struct PlainConfiguration {
    pub udp_trackers: Vec<String>,
    pub http_trackers: Vec<String>,
    pub health_checks: Vec<String>,
}

/// Validated configuration
pub struct Configuration {
    pub udp_trackers: Vec<SocketAddr>,
    pub http_trackers: Vec<ServiceUrl>,
    pub health_checks: Vec<ServiceUrl>,
}

#[derive(Debug)]
pub enum ConfigurationError {
    JsonParseError(serde_json::Error),
    InvalidUdpAddress(std::net::AddrParseError),
    InvalidUrl(url::ParseError),
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigurationError::JsonParseError(e) => write!(f, "JSON parse error: {e}"),
            ConfigurationError::InvalidUdpAddress(e) => write!(f, "Invalid UDP address: {e}"),
            ConfigurationError::InvalidUrl(e) => write!(f, "Invalid URL: {e}"),
        }
    }
}

impl TryFrom<PlainConfiguration> for Configuration {
    type Error = ConfigurationError;

    fn try_from(plain_config: PlainConfiguration) -> Result<Self, Self::Error> {
        let udp_trackers = plain_config
            .udp_trackers
            .into_iter()
            .map(|s| s.parse::<SocketAddr>().map_err(ConfigurationError::InvalidUdpAddress))
            .collect::<Result<Vec<_>, _>>()?;

        let http_trackers = plain_config
            .http_trackers
            .into_iter()
            .map(|s| s.parse::<ServiceUrl>().map_err(ConfigurationError::InvalidUrl))
            .collect::<Result<Vec<_>, _>>()?;

        let health_checks = plain_config
            .health_checks
            .into_iter()
            .map(|s| s.parse::<ServiceUrl>().map_err(ConfigurationError::InvalidUrl))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Configuration {
            udp_trackers,
            http_trackers,
            health_checks,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use super::*;

    #[test]
    fn configuration_should_be_build_from_plain_serializable_configuration() {
        let dto = PlainConfiguration {
            udp_trackers: vec!["127.0.0.1:8080".to_string()],
            http_trackers: vec!["http://127.0.0.1:8080".to_string()],
            health_checks: vec!["http://127.0.0.1:8080/health".to_string()],
        };

        let config = Configuration::try_from(dto).expect("A valid configuration");

        assert_eq!(
            config.udp_trackers,
            vec![SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)]
        );
        assert_eq!(
            config.http_trackers,
            vec![ServiceUrl::parse("http://127.0.0.1:8080").unwrap()]
        );
        assert_eq!(
            config.health_checks,
            vec![ServiceUrl::parse("http://127.0.0.1:8080/health").unwrap()]
        );
    }

    mod building_configuration_from_plan_configuration {
        use crate::checker::config::{Configuration, PlainConfiguration};

        #[test]
        fn it_should_fail_when_a_tracker_udp_address_is_invalid() {
            let plain_config = PlainConfiguration {
                udp_trackers: vec!["invalid_address".to_string()],
                http_trackers: vec![],
                health_checks: vec![],
            };

            assert!(Configuration::try_from(plain_config).is_err());
        }

        #[test]
        fn it_should_fail_when_a_tracker_http_address_is_invalid() {
            let plain_config = PlainConfiguration {
                udp_trackers: vec![],
                http_trackers: vec!["not_a_url".to_string()],
                health_checks: vec![],
            };

            assert!(Configuration::try_from(plain_config).is_err());
        }

        #[test]
        fn it_should_fail_when_a_health_check_http_address_is_invalid() {
            let plain_config = PlainConfiguration {
                udp_trackers: vec![],
                http_trackers: vec![],
                health_checks: vec!["not_a_url".to_string()],
            };

            assert!(Configuration::try_from(plain_config).is_err());
        }
    }
}
