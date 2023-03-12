use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use super::handlers::get_stats_handler;
use crate::tracker::Tracker;

pub fn add(prefix: &str, router: Router, tracker: Arc<Tracker>) -> Router {
    router.route(&format!("{prefix}/stats"), get(get_stats_handler).with_state(tracker))
}
