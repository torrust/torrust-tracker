use axum::response::Json;

use super::resources::Stats;
use crate::tracker::services::statistics::TrackerMetrics;

pub fn stats_response(tracker_metrics: TrackerMetrics) -> Json<Stats> {
    Json(Stats::from(tracker_metrics))
}
