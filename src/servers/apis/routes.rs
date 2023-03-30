//! API routes.
//!
//! It loads all the API routes for all API versions and adds the authentication
//! middleware to them.
//!
//! All the API routes have the `/api` prefix and the version number as the
//! first path segment. For example: `/api/v1/torrents`.
use std::sync::Arc;

use axum::{middleware, Router};

use super::v1;
use crate::tracker::Tracker;

/// Add all API routes to the router.
#[allow(clippy::needless_pass_by_value)]
pub fn router(tracker: Arc<Tracker>) -> Router {
    let router = Router::new();

    let prefix = "/api";

    let router = v1::routes::add(prefix, router, tracker.clone());

    router.layer(middleware::from_fn_with_state(
        tracker.config.clone(),
        v1::middlewares::auth::auth,
    ))
}
