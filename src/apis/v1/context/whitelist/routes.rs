use std::sync::Arc;

use axum::routing::{delete, get, post};
use axum::Router;

use super::handlers::{add_torrent_to_whitelist_handler, reload_whitelist_handler, remove_torrent_from_whitelist_handler};
use crate::tracker::Tracker;

pub fn add(prefix: &str, router: Router, tracker: Arc<Tracker>) -> Router {
    let prefix = format!("{prefix}/whitelist");

    router
        // Whitelisted torrents
        .route(
            &format!("{prefix}/:info_hash"),
            post(add_torrent_to_whitelist_handler).with_state(tracker.clone()),
        )
        .route(
            &format!("{prefix}/:info_hash"),
            delete(remove_torrent_from_whitelist_handler).with_state(tracker.clone()),
        )
        // Whitelist commands
        .route(&format!("{prefix}/reload"), get(reload_whitelist_handler).with_state(tracker))
}
