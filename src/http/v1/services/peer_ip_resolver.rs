//! Given this request chain:
//!
//! client          <-> http proxy 1                 <-> http proxy 2                          <-> server
//! ip: 126.0.0.1       ip: 126.0.0.2                    ip: 126.0.0.3                             ip: 126.0.0.4
//!                     X-Forwarded-For: 126.0.0.1       X-Forwarded-For: 126.0.0.1,126.0.0.2
//!
//! This service resolves the peer IP from these values:
//!
//! `right_most_x_forwarded_for` = 126.0.0.2
//! `connection_info_ip`         = 126.0.0.3
//!
//! Depending on the tracker configuration.
use std::net::IpAddr;
use std::panic::Location;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ClientIpSources {
    pub right_most_x_forwarded_for: Option<IpAddr>,
    pub connection_info_ip: Option<IpAddr>,
}

#[derive(Error, Debug)]
pub enum PeerIpResolutionError {
    #[error(
        "missing or invalid the right most X-Forwarded-For IP (mandatory on reverse proxy tracker configuration) in {location}"
    )]
    MissingRightMostXForwardedForIp { location: &'static Location<'static> },
    #[error("cannot get the client IP from the connection info in {location}")]
    MissingClientIp { location: &'static Location<'static> },
}

/// # Errors
///
/// Will return an error if the peer IP cannot be obtained according to the configuration.
/// For example, if the IP is extracted from an HTTP header which is missing in the request.
pub fn invoke(on_reverse_proxy: bool, client_ip_sources: &ClientIpSources) -> Result<IpAddr, PeerIpResolutionError> {
    if on_reverse_proxy {
        resolve_peer_ip_on_reverse_proxy(client_ip_sources)
    } else {
        resolve_peer_ip_without_reverse_proxy(client_ip_sources)
    }
}

fn resolve_peer_ip_without_reverse_proxy(remote_client_ip: &ClientIpSources) -> Result<IpAddr, PeerIpResolutionError> {
    if let Some(ip) = remote_client_ip.connection_info_ip {
        Ok(ip)
    } else {
        Err(PeerIpResolutionError::MissingClientIp {
            location: Location::caller(),
        })
    }
}

fn resolve_peer_ip_on_reverse_proxy(remote_client_ip: &ClientIpSources) -> Result<IpAddr, PeerIpResolutionError> {
    if let Some(ip) = remote_client_ip.right_most_x_forwarded_for {
        Ok(ip)
    } else {
        Err(PeerIpResolutionError::MissingRightMostXForwardedForIp {
            location: Location::caller(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::invoke;

    mod working_without_reverse_proxy {
        use std::net::IpAddr;
        use std::str::FromStr;

        use super::invoke;
        use crate::http::v1::services::peer_ip_resolver::{ClientIpSources, PeerIpResolutionError};

        #[test]
        fn it_should_get_the_peer_ip_from_the_connection_info() {
            let on_reverse_proxy = false;

            let ip = invoke(
                on_reverse_proxy,
                &ClientIpSources {
                    right_most_x_forwarded_for: None,
                    connection_info_ip: Some(IpAddr::from_str("203.0.113.195").unwrap()),
                },
            )
            .unwrap();

            assert_eq!(ip, IpAddr::from_str("203.0.113.195").unwrap());
        }

        #[test]
        fn it_should_return_an_error_if_it_cannot_get_the_peer_ip_from_the_connection_info() {
            let on_reverse_proxy = false;

            let error = invoke(
                on_reverse_proxy,
                &ClientIpSources {
                    right_most_x_forwarded_for: None,
                    connection_info_ip: None,
                },
            )
            .unwrap_err();

            assert!(matches!(error, PeerIpResolutionError::MissingClientIp { .. }));
        }
    }

    mod working_on_reverse_proxy {
        use std::net::IpAddr;
        use std::str::FromStr;

        use crate::http::v1::services::peer_ip_resolver::{invoke, ClientIpSources, PeerIpResolutionError};

        #[test]
        fn it_should_get_the_peer_ip_from_the_right_most_ip_in_the_x_forwarded_for_header() {
            let on_reverse_proxy = true;

            let ip = invoke(
                on_reverse_proxy,
                &ClientIpSources {
                    right_most_x_forwarded_for: Some(IpAddr::from_str("203.0.113.195").unwrap()),
                    connection_info_ip: None,
                },
            )
            .unwrap();

            assert_eq!(ip, IpAddr::from_str("203.0.113.195").unwrap());
        }

        #[test]
        fn it_should_return_an_error_if_it_cannot_get_the_right_most_ip_from_the_x_forwarded_for_header() {
            let on_reverse_proxy = true;

            let error = invoke(
                on_reverse_proxy,
                &ClientIpSources {
                    right_most_x_forwarded_for: None,
                    connection_info_ip: None,
                },
            )
            .unwrap_err();

            assert!(matches!(error, PeerIpResolutionError::MissingRightMostXForwardedForIp { .. }));
        }
    }
}
