use std::net::IpAddr;
use std::panic::Location;

use axum::response::{IntoResponse, Response};
use thiserror::Error;

use super::remote_client_ip::RemoteClientIp;
use crate::http::axum_implementation::responses;

#[derive(Error, Debug)]
pub enum ResolutionError {
    #[error(
        "missing or invalid the right most X-Forwarded-For IP (mandatory on reverse proxy tracker configuration) in {location}"
    )]
    MissingRightMostXForwardedForIp { location: &'static Location<'static> },
    #[error("cannot get the client IP from the connection info in {location}")]
    MissingClientIp { location: &'static Location<'static> },
}

impl From<ResolutionError> for responses::error::Error {
    fn from(err: ResolutionError) -> Self {
        responses::error::Error {
            failure_reason: format!("Error resolving peer IP: {err}"),
        }
    }
}

/// It resolves the peer IP.
///
/// # Errors
///
/// Will return an error if the peer IP cannot be obtained according to the configuration.
/// For example, if the IP is extracted from an HTTP header which is missing in the request.
pub fn resolve(on_reverse_proxy: bool, remote_client_ip: &RemoteClientIp) -> Result<IpAddr, Response> {
    match resolve_peer_ip(on_reverse_proxy, remote_client_ip) {
        Ok(ip) => Ok(ip),
        Err(error) => Err(error.into_response()),
    }
}

fn resolve_peer_ip(on_reverse_proxy: bool, remote_client_ip: &RemoteClientIp) -> Result<IpAddr, responses::error::Error> {
    if on_reverse_proxy {
        resolve_peer_ip_on_reverse_proxy(remote_client_ip)
    } else {
        resolve_peer_ip_without_reverse_proxy(remote_client_ip)
    }
}

fn resolve_peer_ip_without_reverse_proxy(remote_client_ip: &RemoteClientIp) -> Result<IpAddr, responses::error::Error> {
    if let Some(ip) = remote_client_ip.connection_info_ip {
        Ok(ip)
    } else {
        Err(responses::error::Error::from(ResolutionError::MissingClientIp {
            location: Location::caller(),
        }))
    }
}

fn resolve_peer_ip_on_reverse_proxy(remote_client_ip: &RemoteClientIp) -> Result<IpAddr, responses::error::Error> {
    if let Some(ip) = remote_client_ip.right_most_x_forwarded_for {
        Ok(ip)
    } else {
        Err(responses::error::Error::from(
            ResolutionError::MissingRightMostXForwardedForIp {
                location: Location::caller(),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_peer_ip;
    use crate::http::axum_implementation::responses::error::Error;

    fn assert_error_response(error: &Error, error_message: &str) {
        assert!(
            error.failure_reason.contains(error_message),
            "Error response does not contain message: '{error_message}'. Error: {error:?}"
        );
    }

    mod working_without_reverse_proxy {
        use std::net::IpAddr;
        use std::str::FromStr;

        use super::{assert_error_response, resolve_peer_ip};
        use crate::http::axum_implementation::extractors::remote_client_ip::RemoteClientIp;

        #[test]
        fn it_should_get_the_peer_ip_from_the_connection_info() {
            let on_reverse_proxy = false;

            let ip = resolve_peer_ip(
                on_reverse_proxy,
                &RemoteClientIp {
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

            let response = resolve_peer_ip(
                on_reverse_proxy,
                &RemoteClientIp {
                    right_most_x_forwarded_for: None,
                    connection_info_ip: None,
                },
            )
            .unwrap_err();

            assert_error_response(
                &response,
                "Error resolving peer IP: cannot get the client IP from the connection info",
            );
        }
    }

    mod working_on_reverse_proxy {
        use std::net::IpAddr;
        use std::str::FromStr;

        use super::assert_error_response;
        use crate::http::axum_implementation::extractors::peer_ip::resolve_peer_ip;
        use crate::http::axum_implementation::extractors::remote_client_ip::RemoteClientIp;

        #[test]
        fn it_should_get_the_peer_ip_from_the_right_most_ip_in_the_x_forwarded_for_header() {
            let on_reverse_proxy = true;

            let ip = resolve_peer_ip(
                on_reverse_proxy,
                &RemoteClientIp {
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

            let response = resolve_peer_ip(
                on_reverse_proxy,
                &RemoteClientIp {
                    right_most_x_forwarded_for: None,
                    connection_info_ip: None,
                },
            )
            .unwrap_err();

            assert_error_response(
                &response,
                "Error resolving peer IP: missing or invalid the right most X-Forwarded-For IP",
            );
        }
    }
}
