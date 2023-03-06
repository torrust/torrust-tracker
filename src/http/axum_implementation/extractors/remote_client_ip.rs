use std::net::{IpAddr, SocketAddr};

use axum::async_trait;
use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::request::Parts;
use axum::response::Response;
use axum_client_ip::RightmostXForwardedFor;
use serde::{Deserialize, Serialize};

/// Given this request chain:
///
/// client          <-> http proxy 1                 <-> http proxy 2                          <-> server
/// ip: 126.0.0.1       ip: 126.0.0.2                    ip: 126.0.0.3                             ip: 126.0.0.4
///                     X-Forwarded-For: 126.0.0.1       X-Forwarded-For: 126.0.0.1,126.0.0.2
///
/// This extractor extracts these values from the HTTP headers and connection info.
///
/// `right_most_x_forwarded_for` = 126.0.0.2
/// `connection_info_ip`         = 126.0.0.3
///
/// More info about inner extractors: <https://github.com/imbolc/axum-client-ip>
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct RemoteClientIp {
    pub right_most_x_forwarded_for: Option<IpAddr>,
    pub connection_info_ip: Option<IpAddr>,
}

#[async_trait]
impl<S> FromRequestParts<S> for RemoteClientIp
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

        Ok(RemoteClientIp {
            right_most_x_forwarded_for,
            connection_info_ip,
        })
    }
}
