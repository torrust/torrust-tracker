//! Route initialization for the v1 API.
use std::sync::Arc;

use axum::Router;

use super::context::{auth_key, stats, torrent, whitelist};
use crate::core::Tracker;

/// Add the routes for the v1 API.
pub fn add(prefix: &str, router: Router, tracker: Arc<Tracker>) -> Router {
    let v1_prefix = format!("{prefix}/v1");
    let router = auth_key::routes::add(&v1_prefix, router, tracker.clone());
    let router = stats::routes::add(&v1_prefix, router, tracker.clone());
    let router = whitelist::routes::add(&v1_prefix, router, tracker.clone());
    torrent::routes::add(&v1_prefix, router, tracker)
}
