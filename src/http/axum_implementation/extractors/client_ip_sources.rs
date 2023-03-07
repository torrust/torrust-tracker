//! Wrapper for two Axum extractors to get the relevant information
//! to resolve the remote client IP.
use std::net::SocketAddr;

use axum::async_trait;
use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::request::Parts;
use axum::response::Response;
use axum_client_ip::RightmostXForwardedFor;

use crate::http::axum_implementation::services::peer_ip_resolver::ClientIpSources;

pub struct Extract(pub ClientIpSources);

#[async_trait]
impl<S> FromRequestParts<S> for Extract
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let right_most_x_forwarded_for = match RightmostXForwardedFor::from_request_parts(parts, state).await {
            Ok(right_most_x_forwarded_for) => Some(right_most_x_forwarded_for.0),
            Err(_) => None,
        };

        let connection_info_ip = match ConnectInfo::<SocketAddr>::from_request_parts(parts, state).await {
            Ok(connection_info_socket_addr) => Some(connection_info_socket_addr.0.ip()),
            Err(_) => None,
        };

        Ok(Extract(ClientIpSources {
            right_most_x_forwarded_for,
            connection_info_ip,
        }))
    }
}
