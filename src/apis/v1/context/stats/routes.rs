use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use super::handlers::get_stats_handler;
use crate::tracker::Tracker;

pub fn add(router: Router, tracker: Arc<Tracker>) -> Router {
    router.route("/api/stats", get(get_stats_handler).with_state(tracker))
}
