use std::sync::Arc;

use axum::{middleware, Router};

use super::context::{auth_key, stats, torrent, whitelist};
use super::middlewares::auth::auth;
use crate::tracker::Tracker;

#[allow(clippy::needless_pass_by_value)]
pub fn router(tracker: Arc<Tracker>) -> Router {
    let router = Router::new();

    let router = auth_key::routes::add(router, tracker.clone());
    let router = stats::routes::add(router, tracker.clone());
    let router = whitelist::routes::add(router, tracker.clone());
    let router = torrent::routes::add(router, tracker.clone());

    router.layer(middleware::from_fn_with_state(tracker.config.clone(), auth))
}
