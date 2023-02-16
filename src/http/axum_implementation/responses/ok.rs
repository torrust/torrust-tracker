use axum::Json;

use crate::http::axum_implementation::extractors::remote_client_ip::RemoteClientIp;
use crate::http::axum_implementation::resources::ok::Ok;

#[must_use]
pub fn response(remote_client_ip: &RemoteClientIp) -> Json<Ok> {
    Json(Ok {
        remote_client_ip: remote_client_ip.clone(),
    })
}
