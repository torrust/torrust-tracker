//! HTTP server routes for version `v1`.
use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use axum_client_ip::SecureClientIpSource;
use tower_http::compression::CompressionLayer;

use super::handlers::{announce, health_check, scrape};
use crate::core::Tracker;

/// It adds the routes to the router.
///
/// > **NOTICE**: it's added a layer to get the client IP from the connection
/// info. The tracker could use the connection info to get the client IP.
#[allow(clippy::needless_pass_by_value)]
pub fn router(tracker: Arc<Tracker>) -> Router {
    Router::new()
        // Health check
        .route("/health_check", get(health_check::handler))
        // Announce request
        .route("/announce", get(announce::handle_without_key).with_state(tracker.clone()))
        .route("/announce/:key", get(announce::handle_with_key).with_state(tracker.clone()))
        // Scrape request
        .route("/scrape", get(scrape::handle_without_key).with_state(tracker.clone()))
        .route("/scrape/:key", get(scrape::handle_with_key).with_state(tracker))
        // Add extension to get the client IP from the connection info
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .layer(CompressionLayer::new())
}
