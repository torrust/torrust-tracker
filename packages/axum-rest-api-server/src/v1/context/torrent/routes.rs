//! API routes for the [`torrent`](crate::v1::context::torrent) API context.
//!
//! - `GET /torrent/:info_hash`
//! - `GET /torrents`
//!
//! Refer to the [API endpoint documentation](crate::v1::context::torrent).
use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use torrust_tracker_rest_api_application::use_cases::torrent::TorrentApiService;

use super::handlers::{get_torrent_handler, get_torrents_handler};

/// It adds the routes to the router for the [`torrent`](crate::v1::context::torrent) API context.
pub fn add(prefix: &str, router: Router, service: &Arc<TorrentApiService>) -> Router {
    router
        .route(
            &format!("{prefix}/torrent/{{info_hash}}"),
            get(get_torrent_handler).with_state(service.clone()),
        )
        .route(
            &format!("{prefix}/torrents"),
            get(get_torrents_handler).with_state(service.clone()),
        )
}
