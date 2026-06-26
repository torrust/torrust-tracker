//! API routes for the [`auth_key`](crate::v1::context::auth_key)
//! API context.
//!
//! - `POST /key/:seconds_valid`
//! - `DELETE /key/:key`
//! - `GET /keys/reload`
//!
//! Refer to the [API endpoint documentation](crate::v1::context::auth_key).
use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use torrust_tracker_rest_api_application::use_cases::auth_key::AuthKeyApiService;

use super::handlers::{add_auth_key_handler, delete_auth_key_handler, generate_auth_key_handler, reload_keys_handler};

/// It adds the routes to the router for the [`auth_key`](crate::v1::context::auth_key) API context.
pub fn add(prefix: &str, router: Router, auth_key_service: &Arc<AuthKeyApiService>) -> Router {
    // Keys
    router
        .route(
            &format!("{prefix}/key/{{seconds_valid_or_key}}"),
            post(generate_auth_key_handler)
                .with_state(auth_key_service.clone())
                .delete(delete_auth_key_handler)
                .with_state(auth_key_service.clone()),
        )
        // Keys command
        .route(
            &format!("{prefix}/keys/reload"),
            get(reload_keys_handler).with_state(auth_key_service.clone()),
        )
        .route(
            &format!("{prefix}/keys"),
            post(add_auth_key_handler).with_state(auth_key_service.clone()),
        )
}
