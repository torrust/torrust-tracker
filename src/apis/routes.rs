use std::sync::Arc;

use axum::extract::State;
use axum::response::Json;
use serde_json::{json, Value};

use crate::api::resource::stats::Stats;
use crate::tracker::services::statistics::get_metrics;
use crate::tracker::Tracker;

#[allow(clippy::unused_async)]
pub async fn root() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

#[allow(clippy::unused_async)]
pub async fn get_stats(State(tracker): State<Arc<Tracker>>) -> Json<Value> {
    Json(json!(Stats::from(get_metrics(tracker.clone()).await)))
}
