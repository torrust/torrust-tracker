//! API handlers for the [`stats`](crate::v1::context::stats)
//! API context.
use std::sync::Arc;

use axum::extract::State;
use axum::response::Response;
use axum_extra::extract::Query;
use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use bittorrent_udp_tracker_core::services::banning::BanService;
use serde::Deserialize;
use tokio::sync::RwLock;
use torrust_rest_tracker_api_core::statistics::services::{get_labeled_metrics, get_metrics};

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
///
/// By default it returns a `200` response with the stats in JSON format.
///
/// You can add the GET parameter `format=prometheus` to get the stats in
/// Prometheus Text Exposition Format.
///
/// Refer to the [API endpoint documentation](crate::v1::context::stats#get-tracker-statistics)
/// for more information about this endpoint.
#[allow(clippy::type_complexity)]
pub async fn get_stats_handler(
    State(state): State<(
        Arc<InMemoryTorrentRepository>,
        Arc<bittorrent_tracker_core::statistics::repository::Repository>,
        Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,
        Arc<torrust_udp_tracker_server::statistics::repository::Repository>,
    )>,
    params: Query<QueryParams>,
) -> Response {
    let metrics = get_metrics(state.0.clone(), state.1.clone(), state.2.clone(), state.3.clone()).await;

    match params.0.format {
        Some(format) => match format {
            Format::Json => stats_response(metrics),
            Format::Prometheus => metrics_response(&metrics),
        },
        None => stats_response(metrics),
    }
}

/// It handles the request to get the tracker extendable metrics.
///
/// By default it returns a `200` response with the stats in JSON format.
///
/// You can add the GET parameter `format=prometheus` to get the stats in
/// Prometheus Text Exposition Format.
#[allow(clippy::type_complexity)]
pub async fn get_metrics_handler(
    State(state): State<(
        Arc<InMemoryTorrentRepository>,
        Arc<RwLock<BanService>>,
        Arc<torrust_tracker_swarm_coordination_registry::statistics::repository::Repository>,
        Arc<bittorrent_tracker_core::statistics::repository::Repository>,
        Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,
        Arc<bittorrent_udp_tracker_core::statistics::repository::Repository>,
        Arc<torrust_udp_tracker_server::statistics::repository::Repository>,
    )>,
    params: Query<QueryParams>,
) -> Response {
    let metrics = get_labeled_metrics(
        state.0.clone(),
        state.1.clone(),
        state.2.clone(),
        state.3.clone(),
        state.4.clone(),
        state.5.clone(),
        state.6.clone(),
    )
    .await;

    match params.0.format {
        Some(format) => match format {
            Format::Json => labeled_stats_response(metrics),
            Format::Prometheus => labeled_metrics_response(&metrics),
        },
        None => labeled_stats_response(metrics),
    }
}
