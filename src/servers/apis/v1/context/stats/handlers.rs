//! API handlers for the [`stats`](crate::servers::apis::v1::context::stats)
//! API context.
use std::sync::Arc;

use axum::extract::State;
use axum::response::Json;

use super::resources::Stats;
use super::responses::stats_response;
use crate::core::services::statistics::get_metrics;
use crate::core::Tracker;

/// It handles the request to get the tracker statistics.
///
/// It returns a `200` response with a json [`Stats`]
///
/// Refer to the [API endpoint documentation](crate::servers::apis::v1::context::stats#get-tracker-statistics)
/// for more information about this endpoint.
pub async fn get_stats_handler(State(tracker): State<Arc<Tracker>>) -> Json<Stats> {
    stats_response(get_metrics(tracker.clone()).await)
}
