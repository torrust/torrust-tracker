use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use super::handlers::get_status_handler;
use crate::tracker::Tracker;

pub fn router(_tracker: &Arc<Tracker>) -> Router {
    Router::new()
        // Status
        .route("/status", get(get_status_handler))
}
