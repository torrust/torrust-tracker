use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use super::handlers::announce::handle;
use super::handlers::status::get_status_handler;
use crate::tracker::Tracker;

pub fn router(tracker: &Arc<Tracker>) -> Router {
    Router::new()
        // Status
        .route("/status", get(get_status_handler))
        // Announce request
        .route("/announce", get(handle).with_state(tracker.clone()))
}
