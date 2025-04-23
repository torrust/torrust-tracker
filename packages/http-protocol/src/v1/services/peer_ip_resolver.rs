//! This service resolves the remote client address.
//!
//! The peer IP is used to identify the peer in the tracker. It's the peer IP
//! that is used in the `announce` responses (peer list). And it's also used to
//! send statistics events.
//!
//! Given this request chain:
//!
//! ```text
//! client          <-> http proxy 1                 <-> http proxy 2                          <-> server
//! ip: 126.0.0.1       ip: 126.0.0.2                    ip: 126.0.0.3                             ip: 126.0.0.4
//!                     X-Forwarded-For: 126.0.0.1       X-Forwarded-For: 126.0.0.1,126.0.0.2
//! ```
//!
//! This `ClientIpSources` contains two options for the peer IP:
//!
//! ```text
//! right_most_x_forwarded_for = 126.0.0.2
//! connection_info_ip         = 126.0.0.3
//! ```
//!
//! Which one to use depends on the `ReverseProxyMode`.
use std::net::{IpAddr, SocketAddr};
use std::panic::Location;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Resolves the client's real address considering proxy headers. Port is also
/// included when available.
///
/// # Errors
///
/// This function returns an error if the IP address cannot be resolved.
pub fn resolve_remote_client_addr(
    reverse_proxy_mode: &ReverseProxyMode,
    client_ip_sources: &ClientIpSources,
) -> Result<RemoteClientAddr, PeerIpResolutionError> {
    let ip = match reverse_proxy_mode {
        ReverseProxyMode::Enabled => ResolvedIp::FromXForwardedFor(client_ip_sources.try_client_ip_from_proxy_header()?),
        ReverseProxyMode::Disabled => ResolvedIp::FromSocketAddr(client_ip_sources.try_client_ip_from_connection_info()?),
    };

    let port = client_ip_sources.client_port_from_connection_info();

    Ok(RemoteClientAddr::new(ip, port))
}

/// This struct indicates whether the tracker is running on reverse proxy mode.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReverseProxyMode {
    Enabled,
    Disabled,
}

impl From<ReverseProxyMode> for bool {
    fn from(reverse_proxy_mode: ReverseProxyMode) -> Self {
        match reverse_proxy_mode {
            ReverseProxyMode::Enabled => true,
            ReverseProxyMode::Disabled => false,
        }
    }
}

impl From<bool> for ReverseProxyMode {
    fn from(reverse_proxy_mode: bool) -> Self {
        if reverse_proxy_mode {
            ReverseProxyMode::Enabled
        } else {
            ReverseProxyMode::Disabled
        }
    }
}
/// This struct contains the sources from which the peer IP can be obtained.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ClientIpSources {
    /// The right most IP from the `X-Forwarded-For` HTTP header.
    pub right_most_x_forwarded_for: Option<IpAddr>,

    /// The client's socket address from the connection info.
    pub connection_info_socket_address: Option<SocketAddr>,
}

impl ClientIpSources {
    fn try_client_ip_from_connection_info(&self) -> Result<IpAddr, PeerIpResolutionError> {
        if let Some(socket_addr) = self.connection_info_socket_address {
            Ok(socket_addr.ip())
        } else {
            Err(PeerIpResolutionError::MissingClientIp {
                location: Location::caller(),
            })
        }
    }

    fn try_client_ip_from_proxy_header(&self) -> Result<IpAddr, PeerIpResolutionError> {
        if let Some(ip) = self.right_most_x_forwarded_for {
            Ok(ip)
        } else {
            Err(PeerIpResolutionError::MissingRightMostXForwardedForIp {
                location: Location::caller(),
            })
        }
    }

    fn client_port_from_connection_info(&self) -> Option<u16> {
        if self.connection_info_socket_address.is_some() {
            self.connection_info_socket_address.map(|socket_addr| socket_addr.port())
        } else {
            None
        }
    }
}

/// The error that can occur when resolving the peer IP.
#[derive(Error, Debug, Clone)]
pub enum PeerIpResolutionError {
    /// The peer IP cannot be obtained because the tracker is configured as a
    /// reverse proxy but the `X-Forwarded-For` HTTP header is missing or
    /// invalid.
    #[error(
        "missing or invalid the right most X-Forwarded-For IP (mandatory on reverse proxy tracker configuration) in {location}"
    )]
    MissingRightMostXForwardedForIp { location: &'static Location<'static> },

    /// The peer IP cannot be obtained because the tracker is not configured as
    /// a reverse proxy but the connection info was not provided to the Axum
    /// framework via a route extension.
    #[error("cannot get the client IP from the connection info in {location}")]
    MissingClientIp { location: &'static Location<'static> },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct RemoteClientAddr {
    ip: ResolvedIp,
    port: Option<u16>,
}

impl RemoteClientAddr {
    #[must_use]
    pub fn new(ip: ResolvedIp, port: Option<u16>) -> Self {
        Self { ip, port }
    }

    #[must_use]
    pub fn ip(&self) -> IpAddr {
        match self.ip {
            ResolvedIp::FromSocketAddr(ip) | ResolvedIp::FromXForwardedFor(ip) => ip,
        }
    }

    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }
}

/// This enum indicates the source of the resolved IP address.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ResolvedIp {
    FromXForwardedFor(IpAddr),
    FromSocketAddr(IpAddr),
}

#[cfg(test)]
mod tests {
    use super::resolve_remote_client_addr;

    mod working_without_reverse_proxy {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::str::FromStr;

        use super::resolve_remote_client_addr;
        use crate::v1::services::peer_ip_resolver::{
            ClientIpSources, PeerIpResolutionError, RemoteClientAddr, ResolvedIp, ReverseProxyMode,
        };

        #[test]
        fn it_should_get_the_remote_client_address_from_the_connection_info() {
            let reverse_proxy_mode = ReverseProxyMode::Disabled;

            let ip = resolve_remote_client_addr(
                &reverse_proxy_mode,
                &ClientIpSources {
                    right_most_x_forwarded_for: None,
                    connection_info_socket_address: Some(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080)),
                },
            )
            .unwrap();

            assert_eq!(
                ip,
                RemoteClientAddr::new(
                    ResolvedIp::FromSocketAddr(IpAddr::from_str("203.0.113.195").unwrap()),
                    Some(8080)
                )
            );
        }

        #[test]
        fn it_should_return_an_error_if_it_cannot_get_the_remote_client_ip_from_the_connection_info() {
            let reverse_proxy_mode = ReverseProxyMode::Disabled;

            let error = resolve_remote_client_addr(
                &reverse_proxy_mode,
                &ClientIpSources {
                    right_most_x_forwarded_for: None,
                    connection_info_socket_address: None,
                },
            )
            .unwrap_err();

            assert!(matches!(error, PeerIpResolutionError::MissingClientIp { .. }));
        }
    }

    mod working_on_reverse_proxy_mode {
        use std::net::IpAddr;
        use std::str::FromStr;

        use crate::v1::services::peer_ip_resolver::{
            resolve_remote_client_addr, ClientIpSources, PeerIpResolutionError, RemoteClientAddr, ResolvedIp, ReverseProxyMode,
        };

        #[test]
        fn it_should_get_the_remote_client_ip_from_the_right_most_ip_in_the_x_forwarded_for_header() {
            let reverse_proxy_mode = ReverseProxyMode::Enabled;

            let ip = resolve_remote_client_addr(
                &reverse_proxy_mode,
                &ClientIpSources {
                    right_most_x_forwarded_for: Some(IpAddr::from_str("203.0.113.195").unwrap()),
                    connection_info_socket_address: None,
                },
            )
            .unwrap();

            assert_eq!(
                ip,
                RemoteClientAddr::new(
                    ResolvedIp::FromXForwardedFor(IpAddr::from_str("203.0.113.195").unwrap()),
                    None
                )
            );
        }

        #[test]
        fn it_should_return_an_error_if_it_cannot_get_the_right_most_ip_from_the_x_forwarded_for_header() {
            let reverse_proxy_mode = ReverseProxyMode::Enabled;

            let error = resolve_remote_client_addr(
                &reverse_proxy_mode,
                &ClientIpSources {
                    right_most_x_forwarded_for: None,
                    connection_info_socket_address: None,
                },
            )
            .unwrap_err();

            assert!(matches!(error, PeerIpResolutionError::MissingRightMostXForwardedForIp { .. }));
        }
    }
}
