//! Route initialization for the v1 API.
use std::sync::Arc;

use axum::Router;

use super::context::{auth_key, stats, torrent, whitelist};
use crate::tracker::Tracker;

/// Add the routes for the v1 API.
///
/// > **NOTICE**: the old API endpoints without `v1` prefix are kept for
/// backward compatibility. For example, the `GET /api/stats` endpoint is
/// still available, but it is deprecated and will be removed in the future.
/// You should use the `GET /api/v1/stats` endpoint instead.
pub fn add(prefix: &str, router: Router, tracker: Arc<Tracker>) -> Router {
    // Without `v1` prefix.
    // We keep the old API endpoints without `v1` prefix for backward compatibility.
    // todo: remove when the torrust index backend is using the `v1` prefix.
    let router = auth_key::routes::add(prefix, router, tracker.clone());
    let router = stats::routes::add(prefix, router, tracker.clone());
    let router = whitelist::routes::add(prefix, router, tracker.clone());
    let router = torrent::routes::add(prefix, router, tracker.clone());

    // With `v1` prefix
    let v1_prefix = format!("{prefix}/v1");
    let router = auth_key::routes::add(&v1_prefix, router, tracker.clone());
    let router = stats::routes::add(&v1_prefix, router, tracker.clone());
    let router = whitelist::routes::add(&v1_prefix, router, tracker.clone());
    torrent::routes::add(&v1_prefix, router, tracker)
}
