use std::sync::Arc;

use axum::extract::State;
use axum::response::Json;

use super::resources::Stats;
use super::responses::stats_response;
use crate::tracker::services::statistics::get_metrics;
use crate::tracker::Tracker;

pub async fn get_stats_handler(State(tracker): State<Arc<Tracker>>) -> Json<Stats> {
    stats_response(get_metrics(tracker.clone()).await)
}
