use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use super::handlers::{get_torrent_handler, get_torrents_handler};
use crate::tracker::Tracker;

pub fn add(prefix: &str, router: Router, tracker: Arc<Tracker>) -> Router {
    // Torrents
    router
        .route(
            &format!("{prefix}/torrent/:info_hash"),
            get(get_torrent_handler).with_state(tracker.clone()),
        )
        .route(&format!("{prefix}/torrents"), get(get_torrents_handler).with_state(tracker))
}
