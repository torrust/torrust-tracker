use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use axum_client_ip::SecureClientIpSource;

use super::handlers::{announce, scrape, status};
use crate::tracker::Tracker;

pub fn router(tracker: &Arc<Tracker>) -> Router {
    Router::new()
        // Status
        .route("/status", get(status::handle))
        // Announce request
        .route("/announce", get(announce::handle).with_state(tracker.clone()))
        // Scrape request
        .route("/scrape", get(scrape::handle).with_state(tracker.clone()))
        // Add extension to get the client IP from the connection info
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
}
