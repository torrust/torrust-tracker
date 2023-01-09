use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::{IntoResponse, Json, Response};
use serde_json::json;

use crate::api::resource::stats::Stats;
use crate::api::resource::torrent::Torrent;
use crate::protocol::info_hash::InfoHash;
use crate::tracker::services::statistics::get_metrics;
use crate::tracker::services::torrent::get_torrent_info;
use crate::tracker::Tracker;

pub async fn get_stats(State(tracker): State<Arc<Tracker>>) -> Json<Stats> {
    Json(Stats::from(get_metrics(tracker.clone()).await))
}

/// # Panics
///
/// Will panic if it can't parse the infohash in the request
pub async fn get_torrent(State(tracker): State<Arc<Tracker>>, Path(info_hash): Path<String>) -> Response {
    let optional_torrent_info = get_torrent_info(tracker.clone(), &InfoHash::from_str(&info_hash).unwrap()).await;

    match optional_torrent_info {
        Some(info) => Json(Torrent::from(info)).into_response(),
        None => Json(json!("torrent not known")).into_response(),
    }
}
