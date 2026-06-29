//! API handlers for the [`stats`](crate::v1::context::stats)
//! API context.
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::Response;
use serde::Deserialize;
use torrust_tracker_rest_api_application::v1::use_cases::stats::StatsApiService;

use super::responses::{labeled_metrics_response, labeled_stats_response, metrics_response, stats_response};

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    #[default]
    Json,
    Prometheus,
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    /// The [`Format`] of the stats.
    #[serde(default)]
    pub format: Option<Format>,
}

/// It handles the request to get the tracker global metrics.
pub async fn get_stats_handler(State(stats_service): State<Arc<StatsApiService>>, params: Query<QueryParams>) -> Response {
    let stats = stats_service.get_stats().await;

    match params.0.format {
        Some(format) => match format {
            Format::Json => stats_response(&stats),
            Format::Prometheus => metrics_response(&stats),
        },
        None => stats_response(&stats),
    }
}

/// It handles the request to get the tracker extendable metrics.
pub async fn get_metrics_handler(State(stats_service): State<Arc<StatsApiService>>, params: Query<QueryParams>) -> Response {
    let labeled_stats = stats_service.get_labeled_stats().await;

    match params.0.format {
        Some(format) => match format {
            Format::Json => labeled_stats_response(&labeled_stats),
            Format::Prometheus => labeled_metrics_response(&labeled_stats),
        },
        None => labeled_stats_response(&labeled_stats),
    }
}
