use std::sync::Arc;

use axum::Router;

use super::context::{auth_key, stats, torrent, whitelist};
use crate::tracker::Tracker;

pub fn add(router: Router, tracker: Arc<Tracker>) -> Router {
    let router = auth_key::routes::add(router, tracker.clone());
    let router = stats::routes::add(router, tracker.clone());
    let router = whitelist::routes::add(router, tracker.clone());
    torrent::routes::add(router, tracker)
}
