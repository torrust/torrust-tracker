use std::net::IpAddr;

use axum::Json;

use crate::http::axum_implementation::resources::ok::Ok;

#[must_use]
pub fn response(remote_client_insecure_ip: &IpAddr, remote_client_secure_ip: &IpAddr) -> Json<Ok> {
    Json(Ok {
        remote_client_insecure_ip: *remote_client_insecure_ip,
        remote_client_secure_ip: *remote_client_secure_ip,
    })
}
