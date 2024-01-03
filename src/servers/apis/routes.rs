//! API routes.
//!
//! It loads all the API routes for all API versions and adds the authentication
//! middleware to them.
//!
//! All the API routes have the `/api` prefix and the version number as the
//! first path segment. For example: `/api/v1/torrents`.
use std::sync::Arc;

use axum::routing::get;
use axum::{middleware, Router};
use torrust_tracker_configuration::AccessTokens;
use tower_http::compression::CompressionLayer;

use super::v1;
use super::v1::context::health_check::handlers::health_check_handler;
use super::v1::middlewares::auth::State;
use crate::core::Tracker;

/// Add all API routes to the router.
#[allow(clippy::needless_pass_by_value)]
pub fn router(tracker: Arc<Tracker>, access_tokens: Arc<AccessTokens>) -> Router {
    let router = Router::new();

    let api_url_prefix = "/api";

    let router = v1::routes::add(api_url_prefix, router, tracker.clone());

    let state = State { access_tokens };

    router
        .layer(middleware::from_fn_with_state(state, v1::middlewares::auth::auth))
        .route(&format!("{api_url_prefix}/health_check"), get(health_check_handler))
        .layer(CompressionLayer::new())
}
