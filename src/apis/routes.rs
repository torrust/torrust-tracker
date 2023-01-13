use std::sync::Arc;

use axum::routing::{delete, get, post};
use axum::{middleware, Router};

use super::handlers::{
    add_torrent_to_whitelist_handler, delete_auth_key_handler, generate_auth_key_handler, get_stats_handler, get_torrent_handler,
    get_torrents_handler, reload_keys_handler, reload_whitelist_handler, remove_torrent_from_whitelist_handler,
};
use super::middlewares::auth::auth;
use crate::tracker::Tracker;

/* code-review:
    When Axum cannot parse a path or query param it shows a message like this:

    For the "seconds_valid_or_key" path param:

    "Invalid URL: Cannot parse "-1" to a `u64`"

    That message is not an informative message, specially if you have more than one param.
    We should show a message similar to the one we use when we parse the value in the handler.
    For example:

    "Invalid URL: invalid infohash param: string \"INVALID VALUE\", expected a 40 character long string"

    We can customize the error message by using a custom type with custom serde deserialization.
    The same we are using for the "InfoHashVisitor".

    Input data from HTTP requests should use struts with primitive types (first level of validation).
    We can put the second level of validation in the application and domain services.
*/

pub fn router(tracker: &Arc<Tracker>) -> Router {
    Router::new()
        // Stats
        .route("/api/stats", get(get_stats_handler).with_state(tracker.clone()))
        // Torrents
        .route(
            "/api/torrent/:info_hash",
            get(get_torrent_handler).with_state(tracker.clone()),
        )
        .route("/api/torrents", get(get_torrents_handler).with_state(tracker.clone()))
        // Whitelisted torrents
        .route(
            "/api/whitelist/:info_hash",
            post(add_torrent_to_whitelist_handler).with_state(tracker.clone()),
        )
        .route(
            "/api/whitelist/:info_hash",
            delete(remove_torrent_from_whitelist_handler).with_state(tracker.clone()),
        )
        // Whitelist command
        .route(
            "/api/whitelist/reload",
            get(reload_whitelist_handler).with_state(tracker.clone()),
        )
        // Keys
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
        .route("/api/keys/reload", get(reload_keys_handler).with_state(tracker.clone()))
        .layer(middleware::from_fn_with_state(tracker.config.clone(), auth))
}
