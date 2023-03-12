use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use super::handlers::{delete_auth_key_handler, generate_auth_key_handler, reload_keys_handler};
use crate::tracker::Tracker;

pub fn add(router: Router, tracker: Arc<Tracker>) -> Router {
    // Keys
    router
        .route(
            // code-review: Axum does not allow two routes with the same path but different path variable name.
            // In the new major API version, `seconds_valid` should be a POST form field so that we will have two paths:
            // POST /api/key
            // DELETE /api/key/:key
            "/api/key/:seconds_valid_or_key",
            post(generate_auth_key_handler)
                .with_state(tracker.clone())
                .delete(delete_auth_key_handler)
                .with_state(tracker.clone()),
        )
        // Keys command
        .route("/api/keys/reload", get(reload_keys_handler).with_state(tracker))
}
