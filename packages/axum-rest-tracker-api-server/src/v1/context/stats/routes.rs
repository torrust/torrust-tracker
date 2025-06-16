//! API routes for the [`stats`](crate::v1::context::stats) API context.
//!
//! - `GET /stats`
//!
//! Refer to the [API endpoint documentation](crate::v1::context::stats).
use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use torrust_rest_tracker_api_core::container::TrackerHttpApiCoreContainer;

use super::handlers::{get_metrics_handler, get_stats_handler};

/// It adds the routes to the router for the [`stats`](crate::v1::context::stats) API context.
pub fn add(prefix: &str, router: Router, http_api_container: &Arc<TrackerHttpApiCoreContainer>) -> Router {
    router
        .route(
            &format!("{prefix}/stats"),
            get(get_stats_handler).with_state((
                http_api_container.tracker_core_container.in_memory_torrent_repository.clone(),
                http_api_container.tracker_core_container.stats_repository.clone(),
                http_api_container.http_stats_repository.clone(),
                http_api_container.udp_server_stats_repository.clone(),
            )),
        )
        .route(
            &format!("{prefix}/metrics"),
            get(get_metrics_handler).with_state((
                http_api_container.tracker_core_container.in_memory_torrent_repository.clone(),
                http_api_container.ban_service.clone(),
                // Stats
                http_api_container
                    .swarm_coordination_registry_container
                    .stats_repository
                    .clone(),
                http_api_container.tracker_core_container.stats_repository.clone(),
                http_api_container.http_stats_repository.clone(),
                http_api_container.udp_core_stats_repository.clone(),
                http_api_container.udp_server_stats_repository.clone(),
            )),
        )
}
