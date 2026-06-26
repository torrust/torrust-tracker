//! API routes for the [`stats`](crate::v1::context::stats) API context.
//!
//! - `GET /stats`
//!
//! Refer to the [API endpoint documentation](crate::v1::context::stats).
use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use torrust_tracker_rest_api_application::use_cases::stats::StatsApiService;

use super::handlers::{get_metrics_handler, get_stats_handler};

/// It adds the routes to the router for the [`stats`](crate::v1::context::stats) API context.
pub fn add(prefix: &str, router: Router, stats_service: &Arc<StatsApiService>) -> Router {
    router
        .route(
            &format!("{prefix}/stats"),
            get(get_stats_handler).with_state(stats_service.clone()),
        )
        .route(
            &format!("{prefix}/metrics"),
            get(get_metrics_handler).with_state(stats_service.clone()),
        )
}
