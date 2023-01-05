use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::Json;

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
/// Will panic if the torrent does not exist.
pub async fn get_torrent(State(tracker): State<Arc<Tracker>>, Path(info_hash): Path<String>) -> Json<Torrent> {
    let info = get_torrent_info(tracker.clone(), &InfoHash::from_str(&info_hash).unwrap())
        .await
        .unwrap();
    // todo: return "not found" if the torrent does not exist
    Json(Torrent::from(info))
}
