use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use axum_client_ip::SecureClientIpSource;

use super::handlers::announce::handle;
use super::handlers::status::get_status_handler;
use crate::tracker::Tracker;

pub fn router(tracker: &Arc<Tracker>) -> Router {
    let secure_client_ip_source = if tracker.config.on_reverse_proxy {
        SecureClientIpSource::RightmostXForwardedFor
    } else {
        SecureClientIpSource::ConnectInfo
    };

    Router::new()
        // Status
        .route("/status", get(get_status_handler))
        // Announce request
        .route("/announce", get(handle).with_state(tracker.clone()))
        .layer(secure_client_ip_source.into_extension())
}
