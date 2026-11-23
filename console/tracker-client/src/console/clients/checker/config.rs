use std::error::Error;
use std::fmt;

use reqwest::Url as ServiceUrl;
use serde::Deserialize;

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
    pub udp_trackers: Vec<ServiceUrl>,
    pub http_trackers: Vec<ServiceUrl>,
    pub health_checks: Vec<ServiceUrl>,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ConfigurationError {
    JsonParseError(serde_json::Error),
    InvalidUdpAddress(std::net::AddrParseError),
    InvalidUrl(url::ParseError),
    /// The entry is a syntactically valid URL that the check cannot use.
    InvalidTrackerUrl {
        entry: String,
        reason: UrlRejection,
    },
}

/// Why a tracker or health-check URL cannot be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UrlRejection {
    MissingHost,
    MissingPort,
    UnsupportedScheme,
}

impl fmt::Display for UrlRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHost => write!(f, "the URL has no host"),
            Self::MissingPort => write!(f, "the URL has no port"),
            Self::UnsupportedScheme => write!(f, "the URL scheme is not supported here"),
        }
    }
}

impl Error for ConfigurationError {}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::JsonParseError(e) => write!(f, "JSON parse error: {e}"),
            Self::InvalidUdpAddress(e) => write!(f, "Invalid UDP address: {e}"),
            Self::InvalidUrl(e) => write!(f, "Invalid URL: {e}"),
            Self::InvalidTrackerUrl { entry, reason } => write!(f, "Invalid URL `{entry}`: {reason}"),
        }
    }
}

impl TryFrom<PlainConfiguration> for Configuration {
    type Error = ConfigurationError;

    fn try_from(plain_config: PlainConfiguration) -> Result<Self, Self::Error> {
        let udp_trackers = plain_config
            .udp_trackers
            .into_iter()
            .map(|entry| parse_udp_tracker_url(&entry))
            .collect::<Result<Vec<_>, _>>()?;

        let http_trackers = plain_config
            .http_trackers
            .into_iter()
            .map(|entry| parse_http_url(&entry))
            .collect::<Result<Vec<_>, _>>()?;

        let health_checks = plain_config
            .health_checks
            .into_iter()
            .map(|entry| parse_http_url(&entry))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            udp_trackers,
            http_trackers,
            health_checks,
        })
    }
}

/// Parses a UDP tracker entry. A missing scheme defaults to `udp`; the entry must have a host
/// and a port because UDP has no default port.
fn parse_udp_tracker_url(entry: &str) -> Result<ServiceUrl, ConfigurationError> {
    // The scheme is read from the raw entry: `127.0.0.1:6969` has none, and a URL parser alone
    // would read `localhost:6969` as the scheme `localhost`.
    let candidate = match leading_scheme(entry) {
        Some(scheme) if scheme.eq_ignore_ascii_case("udp") => entry.to_string(),
        Some(_) => return Err(reject(entry, UrlRejection::UnsupportedScheme)),
        None => format!("udp://{entry}"),
    };
    let url = candidate.parse::<ServiceUrl>().map_err(ConfigurationError::InvalidUrl)?;

    if url.host_str().is_none_or(str::is_empty) {
        return Err(reject(entry, UrlRejection::MissingHost));
    }

    if url.port().is_none() {
        return Err(reject(entry, UrlRejection::MissingPort));
    }

    Ok(url)
}

/// Returns the scheme when the entry starts with one (`<scheme>://`).
///
/// A scheme is a letter followed by letters, digits, `+`, `-` or `.` (RFC 3986), so `://` found
/// later in the entry, for example inside a query string, is not a leading scheme.
fn leading_scheme(entry: &str) -> Option<&str> {
    let (scheme, _) = entry.split_once("://")?;
    let mut chars = scheme.chars();

    chars.next().filter(char::is_ascii_alphabetic)?;
    chars
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
        .then_some(scheme)
}

/// Parses an HTTP tracker or health-check entry. It must have the scheme `http` or `https` and a
/// host; the port is optional because HTTP(S) has default ports.
fn parse_http_url(entry: &str) -> Result<ServiceUrl, ConfigurationError> {
    let url = entry.parse::<ServiceUrl>().map_err(ConfigurationError::InvalidUrl)?;

    if !matches!(url.scheme(), "http" | "https") {
        return Err(reject(entry, UrlRejection::UnsupportedScheme));
    }

    if url.host_str().is_none_or(str::is_empty) {
        return Err(reject(entry, UrlRejection::MissingHost));
    }

    Ok(url)
}

fn reject(entry: &str, reason: UrlRejection) -> ConfigurationError {
    ConfigurationError::InvalidTrackerUrl {
        entry: redacted_entry(entry),
        reason,
    }
}

/// Describes a tracker URL or a rejected entry without writing a secret.
///
/// A private tracker URL carries its key in the path, and a URL can carry credentials or a key in
/// the query, so only the scheme (as typed), the host and the port are kept. Anything else is
/// replaced by `/****`, the fixed mask of the CLI output contract.
pub(crate) fn redacted_entry(entry: &str) -> String {
    const MASK: &str = "****";

    let scheme = leading_scheme(entry).map(|scheme| format!("{scheme}://"));
    let parsable = if scheme.is_some() {
        entry.to_string()
    } else {
        format!("udp://{entry}")
    };

    let Ok(url) = parsable.parse::<ServiceUrl>() else {
        return format!("{}{MASK}", scheme.unwrap_or_default());
    };

    let authority = url.host_str().map_or_else(String::new, |host| {
        url.port().map_or_else(|| host.to_string(), |port| format!("{host}:{port}"))
    });
    let has_more = !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !matches!(url.path(), "" | "/");

    format!(
        "{}{authority}{}",
        scheme.unwrap_or_default(),
        if has_more { format!("/{MASK}") } else { String::new() }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_should_be_build_from_plain_serializable_configuration() {
        let dto = PlainConfiguration {
            udp_trackers: vec!["udp://127.0.0.1:8080".to_string()],
            http_trackers: vec!["http://127.0.0.1:8080".to_string()],
            health_checks: vec!["http://127.0.0.1:8080/health".to_string()],
        };

        let config = Configuration::try_from(dto).expect("A valid configuration");

        assert_eq!(config.udp_trackers, vec![ServiceUrl::parse("udp://127.0.0.1:8080").unwrap()]);

        assert_eq!(
            config.http_trackers,
            vec![ServiceUrl::parse("http://127.0.0.1:8080").unwrap()]
        );

        assert_eq!(
            config.health_checks,
            vec![ServiceUrl::parse("http://127.0.0.1:8080/health").unwrap()]
        );
    }

    mod building_configuration_from_plain_configuration_for {

        use crate::console::clients::checker::config::{Configuration, ConfigurationError, PlainConfiguration, UrlRejection};

        /// Returns the rejected entry and why it was rejected.
        fn rejection(plain_config: PlainConfiguration) -> (String, UrlRejection) {
            match Configuration::try_from(plain_config) {
                Err(ConfigurationError::InvalidTrackerUrl { entry, reason }) => (entry, reason),
                Err(other) => panic!("expected an invalid tracker URL error, got: {other}"),
                Ok(_) => panic!("the configuration should be rejected"),
            }
        }

        mod udp_trackers {
            use super::rejection;
            use crate::console::clients::checker::config::{Configuration, PlainConfiguration, ServiceUrl, UrlRejection};

            /* The plain configuration should allow UDP URLs with:

            - IP or domain.
            - With or without scheme.
            - With or without `announce` suffix.
            - With or without `/` at the end of the authority section (with empty path).

            For example:

            127.0.0.1:6969
            127.0.0.1:6969/
            127.0.0.1:6969/announce

            localhost:6969
            localhost:6969/
            localhost:6969/announce

            udp://127.0.0.1:6969
            udp://127.0.0.1:6969/
            udp://127.0.0.1:6969/announce

            udp://localhost:6969
            udp://localhost:6969/
            udp://localhost:6969/announce

            */

            #[test]
            fn it_should_fail_when_a_tracker_udp_url_is_invalid() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec!["invalid URL".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                };

                assert!(Configuration::try_from(plain_config).is_err());
            }

            #[test]
            fn it_should_fail_when_a_tracker_udp_url_has_no_port() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec!["udp://127.0.0.1".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                });

                assert_eq!(rejected, ("udp://127.0.0.1".to_string(), UrlRejection::MissingPort));
            }

            #[test]
            fn it_should_fail_when_a_tracker_udp_url_has_no_scheme_and_no_port() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec!["127.0.0.1".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                });

                assert_eq!(rejected, ("127.0.0.1".to_string(), UrlRejection::MissingPort));
            }

            #[test]
            fn it_should_fail_when_a_tracker_udp_url_has_a_path_but_no_port() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec!["udp://localhost/announce".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                });

                assert_eq!(rejected, ("udp://localhost/****".to_string(), UrlRejection::MissingPort));
            }

            #[test]
            fn it_should_fail_when_a_tracker_udp_url_has_no_host() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec!["udp:///".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                });

                assert_eq!(rejected, ("udp://".to_string(), UrlRejection::MissingHost));
            }

            #[test]
            fn it_should_fail_when_a_tracker_udp_url_has_the_http_scheme() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec!["http://127.0.0.1".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                });

                assert_eq!(rejected, ("http://127.0.0.1".to_string(), UrlRejection::UnsupportedScheme));
            }

            #[test]
            fn it_should_fail_when_a_tracker_udp_url_has_the_ftp_scheme() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec!["ftp://127.0.0.1:6969".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                });

                assert_eq!(
                    rejected,
                    ("ftp://127.0.0.1:6969".to_string(), UrlRejection::UnsupportedScheme)
                );
            }

            #[test]
            fn it_should_not_echo_the_path_of_a_rejected_tracker_url() {
                // A private HTTP tracker URL carries its key in the path.
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec!["http://127.0.0.1:7070/announce/PRIVATE_KEY".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                });

                assert_eq!(
                    rejected,
                    ("http://127.0.0.1:7070/****".to_string(), UrlRejection::UnsupportedScheme)
                );
            }

            #[test]
            fn it_should_not_echo_the_query_of_a_rejected_tracker_url() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec!["udp://127.0.0.1?key=PRIVATE_KEY".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                });

                assert_eq!(rejected, ("udp://127.0.0.1/****".to_string(), UrlRejection::MissingPort));
            }

            #[test]
            fn it_should_not_echo_the_credentials_of_a_rejected_tracker_url() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec!["ftp://user:PRIVATE_KEY@127.0.0.1:6969".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                });

                assert_eq!(
                    rejected,
                    ("ftp://127.0.0.1:6969/****".to_string(), UrlRejection::UnsupportedScheme)
                );
            }

            #[test]
            fn it_should_allow_the_url_to_have_no_scheme_when_it_has_a_domain_and_a_port() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec!["localhost:6969".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                };

                let config = Configuration::try_from(plain_config).expect("Invalid plain configuration");

                assert_eq!(config.udp_trackers[0], "udp://localhost:6969".parse::<ServiceUrl>().unwrap());
            }

            #[test]
            fn it_should_allow_the_udp_scheme_in_upper_case() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec!["UDP://127.0.0.1:6969".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                };

                let config = Configuration::try_from(plain_config).expect("Invalid plain configuration");

                assert_eq!(config.udp_trackers[0], "udp://127.0.0.1:6969".parse::<ServiceUrl>().unwrap());
            }

            #[test]
            fn it_should_not_read_a_scheme_from_a_query_string() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec!["127.0.0.1:6969/announce?next=http://example.com".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                };

                let config = Configuration::try_from(plain_config).expect("Invalid plain configuration");

                assert_eq!(
                    config.udp_trackers[0],
                    "udp://127.0.0.1:6969/announce?next=http://example.com"
                        .parse::<ServiceUrl>()
                        .unwrap()
                );
            }

            #[test]
            fn it_should_add_the_udp_scheme_to_the_udp_url_when_it_is_missing() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec!["127.0.0.1:6969".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                };

                let config = Configuration::try_from(plain_config).expect("Invalid plain configuration");

                assert_eq!(config.udp_trackers[0], "udp://127.0.0.1:6969".parse::<ServiceUrl>().unwrap());
            }

            #[test]
            fn it_should_allow_using_domains() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec!["udp://localhost:6969".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                };

                let config = Configuration::try_from(plain_config).expect("Invalid plain configuration");

                assert_eq!(config.udp_trackers[0], "udp://localhost:6969".parse::<ServiceUrl>().unwrap());
            }

            #[test]
            fn it_should_allow_the_url_to_have_an_empty_path() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec!["127.0.0.1:6969/".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                };

                let config = Configuration::try_from(plain_config).expect("Invalid plain configuration");

                assert_eq!(config.udp_trackers[0], "udp://127.0.0.1:6969/".parse::<ServiceUrl>().unwrap());
            }

            #[test]
            fn it_should_allow_the_url_to_contain_a_path() {
                // This is the common format for UDP tracker URLs:
                // udp://domain.com:6969/announce

                let plain_config = PlainConfiguration {
                    udp_trackers: vec!["127.0.0.1:6969/announce".to_string()],
                    http_trackers: vec![],
                    health_checks: vec![],
                };

                let config = Configuration::try_from(plain_config).expect("Invalid plain configuration");

                assert_eq!(
                    config.udp_trackers[0],
                    "udp://127.0.0.1:6969/announce".parse::<ServiceUrl>().unwrap()
                );
            }
        }

        mod http_trackers {
            use super::rejection;
            use crate::console::clients::checker::config::{Configuration, PlainConfiguration, ServiceUrl, UrlRejection};

            #[test]
            fn it_should_fail_when_a_tracker_http_url_is_invalid() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec![],
                    http_trackers: vec!["invalid URL".to_string()],
                    health_checks: vec![],
                };

                assert!(Configuration::try_from(plain_config).is_err());
            }

            #[test]
            fn it_should_fail_when_a_tracker_http_url_has_the_ftp_scheme() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec![],
                    http_trackers: vec!["ftp://127.0.0.1".to_string()],
                    health_checks: vec![],
                });

                assert_eq!(rejected, ("ftp://127.0.0.1".to_string(), UrlRejection::UnsupportedScheme));
            }

            #[test]
            fn it_should_fail_when_a_tracker_http_url_has_the_udp_scheme() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec![],
                    http_trackers: vec!["udp://127.0.0.1:6969".to_string()],
                    health_checks: vec![],
                });

                assert_eq!(
                    rejected,
                    ("udp://127.0.0.1:6969".to_string(), UrlRejection::UnsupportedScheme)
                );
            }

            #[test]
            fn it_should_not_echo_the_path_of_a_rejected_http_tracker_url() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec![],
                    http_trackers: vec!["ftp://127.0.0.1/announce/PRIVATE_KEY".to_string()],
                    health_checks: vec![],
                });

                assert_eq!(
                    rejected,
                    ("ftp://127.0.0.1/****".to_string(), UrlRejection::UnsupportedScheme)
                );
            }

            #[test]
            fn it_should_allow_https_urls_without_a_port() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec![],
                    http_trackers: vec!["https://tracker.example.com".to_string()],
                    health_checks: vec![],
                };

                assert!(Configuration::try_from(plain_config).is_ok());
            }

            #[test]
            fn it_should_allow_the_url_to_contain_a_path() {
                // This is the common format for HTTP tracker URLs:
                // http://domain.com:7070/announce

                let plain_config = PlainConfiguration {
                    udp_trackers: vec![],
                    http_trackers: vec!["http://127.0.0.1:7070/announce".to_string()],
                    health_checks: vec![],
                };

                let config = Configuration::try_from(plain_config).expect("Invalid plain configuration");

                assert_eq!(
                    config.http_trackers[0],
                    "http://127.0.0.1:7070/announce".parse::<ServiceUrl>().unwrap()
                );
            }

            #[test]
            fn it_should_allow_the_url_to_contain_an_empty_path() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec![],
                    http_trackers: vec!["http://127.0.0.1:7070/".to_string()],
                    health_checks: vec![],
                };

                let config = Configuration::try_from(plain_config).expect("Invalid plain configuration");

                assert_eq!(
                    config.http_trackers[0],
                    "http://127.0.0.1:7070/".parse::<ServiceUrl>().unwrap()
                );
            }
        }

        mod health_checks {
            use super::rejection;
            use crate::console::clients::checker::config::{Configuration, PlainConfiguration, UrlRejection};

            #[test]
            fn it_should_fail_when_a_health_check_http_url_is_invalid() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec![],
                    http_trackers: vec![],
                    health_checks: vec!["invalid URL".to_string()],
                };

                assert!(Configuration::try_from(plain_config).is_err());
            }

            #[test]
            fn it_should_fail_when_a_health_check_url_has_the_ftp_scheme() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec![],
                    http_trackers: vec![],
                    health_checks: vec!["ftp://127.0.0.1".to_string()],
                });

                assert_eq!(rejected, ("ftp://127.0.0.1".to_string(), UrlRejection::UnsupportedScheme));
            }

            #[test]
            fn it_should_fail_when_a_health_check_url_has_the_udp_scheme() {
                let rejected = rejection(PlainConfiguration {
                    udp_trackers: vec![],
                    http_trackers: vec![],
                    health_checks: vec!["udp://127.0.0.1:6969".to_string()],
                });

                assert_eq!(
                    rejected,
                    ("udp://127.0.0.1:6969".to_string(), UrlRejection::UnsupportedScheme)
                );
            }

            #[test]
            fn it_should_allow_https_urls_without_a_port() {
                let plain_config = PlainConfiguration {
                    udp_trackers: vec![],
                    http_trackers: vec![],
                    health_checks: vec!["https://tracker.example.com/health_check".to_string()],
                };

                assert!(Configuration::try_from(plain_config).is_ok());
            }
        }
    }

    mod parsing_from_json {
        use crate::console::clients::checker::config::parse_from_json;

        #[test]
        fn it_should_succeed_with_valid_json() {
            let json = r#"{"udp_trackers":[],"http_trackers":[],"health_checks":[]}"#;
            assert!(parse_from_json(json).is_ok());
        }

        #[test]
        fn it_should_fail_with_trailing_comma_and_include_serde_detail_in_error() {
            let json = r#"{
                "udp_trackers": [],
                "http_trackers": [
                    "http://127.0.0.1:7070",
                ],
                "health_checks": []
            }"#;

            let err = parse_from_json(json).err().expect("Expected a parse error");
            let message = err.to_string();

            // The specific serde_json detail must be present, not just "invalid config format"
            assert!(
                message.contains("trailing comma"),
                "Expected 'trailing comma' in error message, got: {message}"
            );
        }

        #[test]
        fn it_should_fail_with_missing_field_and_include_serde_detail_in_error() {
            // Missing required fields entirely
            let json = r#"{"udp_trackers":[]}"#;

            let err = parse_from_json(json)
                .err()
                .expect("Expected a parse error for missing fields");
            let message = err.to_string();

            assert!(!message.is_empty(), "Expected a non-empty error message, got empty string");
        }

        #[test]
        fn it_should_fail_with_malformed_json_and_include_serde_detail_in_error() {
            let json = r"not json at all";

            let err = parse_from_json(json)
                .err()
                .expect("Expected a parse error for malformed JSON");
            let message = err.to_string();

            assert!(
                message.contains("JSON parse error"),
                "Expected 'JSON parse error' prefix in error message, got: {message}"
            );
        }

        #[test]
        fn it_should_fail_with_invalid_url_and_include_detail_in_error() {
            let json = r#"{"udp_trackers":["not a url"],"http_trackers":[],"health_checks":[]}"#;

            let err = parse_from_json(json).err().expect("Expected an error for an invalid URL");
            let message = err.to_string();

            assert!(!message.is_empty(), "Expected a non-empty error message");
        }
    }
}
