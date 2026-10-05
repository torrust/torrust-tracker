//! API handlers for the [`stats`](crate::v1::context::stats)
//! API context.
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::Response;
use serde::Deserialize;
use torrust_tracker_rest_api_application::v1::use_cases::stats::StatsApiService;

use super::responses::{
    failed_to_get_stats_response, labeled_metrics_response, labeled_stats_response, metrics_response, stats_response,
};

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
/// It returns `500` if the statistics cannot be collected.
pub async fn get_stats_handler(State(stats_service): State<Arc<StatsApiService>>, params: Query<QueryParams>) -> Response {
    let stats = match stats_service.get_stats().await {
        Ok(stats) => stats,
        Err(e) => return failed_to_get_stats_response(e),
    };

    params.0.format.map_or_else(
        || stats_response(&stats),
        |format| match format {
            Format::Json => stats_response(&stats),
            Format::Prometheus => metrics_response(&stats),
        },
    )
}

/// It handles the request to get the tracker extendable metrics.
pub async fn get_metrics_handler(State(stats_service): State<Arc<StatsApiService>>, params: Query<QueryParams>) -> Response {
    let labeled_stats = stats_service.get_labeled_stats().await;

    params.0.format.map_or_else(
        || labeled_stats_response(&labeled_stats),
        |format| match format {
            Format::Json => labeled_stats_response(&labeled_stats),
            Format::Prometheus => labeled_metrics_response(&labeled_stats),
        },
    )
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use axum::body::to_bytes;
    use axum::extract::{Query, State};
    use axum::http::StatusCode;
    use torrust_metrics::metric_collection::MetricCollection;
    use torrust_tracker_rest_api_application::v1::ports::stats::StatsQueryPort;
    use torrust_tracker_rest_api_application::v1::use_cases::stats::StatsApiService;
    use torrust_tracker_rest_api_protocol::v1::context::stats::resources::stats::{LabeledStats, Stats, StatsError};

    use super::{QueryParams, get_stats_handler};

    struct FailingStatsQueryPort;

    #[async_trait]
    impl StatsQueryPort for FailingStatsQueryPort {
        async fn get_stats(&self) -> Result<Stats, StatsError> {
            Err(StatsError::TorrentRepository("swarm registry unavailable".to_string()))
        }

        async fn get_labeled_stats(&self) -> LabeledStats {
            LabeledStats {
                metrics: MetricCollection::default(),
            }
        }
    }

    #[tokio::test]
    async fn it_should_respond_with_an_internal_server_error_when_the_stats_cannot_be_collected() {
        let stats_service = Arc::new(StatsApiService::new(Box::new(FailingStatsQueryPort)));

        let response = get_stats_handler(State(stats_service), Query(QueryParams { format: None })).await;

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(String::from_utf8_lossy(&body).contains("swarm registry unavailable"));
    }
}
