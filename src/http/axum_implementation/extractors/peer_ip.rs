use std::net::IpAddr;
use std::panic::Location;

use axum::response::{IntoResponse, Response};
use thiserror::Error;

use super::remote_client_ip::RemoteClientIp;
use crate::http::axum_implementation::responses;

#[derive(Error, Debug)]
pub enum ResolutionError {
    #[error("missing the right most X-Forwarded-For IP (mandatory on reverse proxy tracker configuration) in {location}")]
    MissingRightMostXForwardedForIp { location: &'static Location<'static> },
    #[error("cannot get the client IP from the connection info in {location}")]
    MissingClientIp { location: &'static Location<'static> },
}

impl From<ResolutionError> for responses::error::Error {
    fn from(err: ResolutionError) -> Self {
        responses::error::Error {
            failure_reason: format!("{err}"),
        }
    }
}

/// It resolves the peer IP.
///
/// # Errors
///
/// Will return an error if the peer IP cannot be obtained according to the configuration.
/// For example, if the IP is extracted from an HTTP header which is missing in the request.
pub fn peer_ip(on_reverse_proxy: bool, remote_client_ip: &RemoteClientIp) -> Result<IpAddr, Response> {
    if on_reverse_proxy {
        if let Some(ip) = remote_client_ip.right_most_x_forwarded_for {
            Ok(ip)
        } else {
            Err(
                responses::error::Error::from(ResolutionError::MissingRightMostXForwardedForIp {
                    location: Location::caller(),
                })
                .into_response(),
            )
        }
    } else if let Some(ip) = remote_client_ip.connection_info_ip {
        Ok(ip)
    } else {
        Err(responses::error::Error::from(ResolutionError::MissingClientIp {
            location: Location::caller(),
        })
        .into_response())
    }
}
