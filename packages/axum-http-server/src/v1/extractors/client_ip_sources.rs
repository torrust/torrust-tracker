//! Axum [`extractor`](axum::extract) to get the relevant information to resolve the remote
//! client IP.
//!
//! It's a wrapper for two third-party Axum extractors.
//!
//! The first one is `RightmostXForwardedFor` from the `axum-client-ip` crate.
//! This extractor is used to get the right-most IP address from the
//! `X-Forwarded-For` header.
//!
//! The second one is `ConnectInfo` from the `axum` crate. This extractor is
//! used to get the IP address of the client from the connection info.
//!
//! The `ClientIpSources` struct is a wrapper for the two extractors.
//!
//! The tracker can be configured to run behind a reverse proxy. In this case,
//! the tracker will use the `X-Forwarded-For` header to get the client IP
//! address.
//!
//! See [`torrust_tracker_configuration::v3_0_0::Configuration::core`].
//!
//! The tracker can also be configured to run without a reverse proxy. In this
//! case, the tracker will use the IP address from the connection info.
//!
//! Given the following scenario:
//!
//! ```text
//! client          <-> http proxy 1                 <-> http proxy 2                          <-> server
//! ip: 126.0.0.1       ip: 126.0.0.2                    ip: 126.0.0.3                             ip: 126.0.0.4
//!                     X-Forwarded-For: 126.0.0.1       X-Forwarded-For: 126.0.0.1,126.0.0.2
//! ```
//!
//! This extractor returns these values:
//!
//! ```text
//! `right_most_x_forwarded_for` = 126.0.0.2
//! `connection_info_ip`         = 126.0.0.3
//! ```
use std::future::Future;
use std::net::SocketAddr;

use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::request::Parts;
use axum::response::Response;
use axum_client_ip::RightmostXForwardedFor;
use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::ClientIpSources;

/// Extractor for the [`ClientIpSources`]
/// struct.
pub struct Extract(pub ClientIpSources);

impl<S> FromRequestParts<S> for Extract
where
    S: Send + Sync,
{
    type Rejection = Response;

    #[allow(clippy::manual_async_fn)]
    fn from_request_parts(parts: &mut Parts, state: &S) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let right_most_x_forwarded_for = RightmostXForwardedFor::from_request_parts(parts, state)
                .await
                .ok()
                .map(|right_most_x_forwarded_for| right_most_x_forwarded_for.0);

            let connection_info_ip = ConnectInfo::<SocketAddr>::from_request_parts(parts, state)
                .await
                .ok()
                .map(|connection_info_socket_addr| connection_info_socket_addr.0);

            Ok(Self(ClientIpSources {
                right_most_x_forwarded_for,
                connection_info_socket_address: connection_info_ip,
            }))
        }
    }
}
