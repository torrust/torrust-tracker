use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Json, Response};
use serde::{de, Deserialize, Deserializer};
use serde_json::json;

use crate::api::resource::stats::Stats;
use crate::api::resource::torrent::{ListItem, Torrent};
use crate::protocol::info_hash::InfoHash;
use crate::tracker::services::statistics::get_metrics;
use crate::tracker::services::torrent::{get_torrent_info, get_torrents, Pagination};
use crate::tracker::Tracker;

pub async fn get_stats_handler(State(tracker): State<Arc<Tracker>>) -> Json<Stats> {
    Json(Stats::from(get_metrics(tracker.clone()).await))
}

/// # Panics
///
/// Will panic if it can't parse the infohash in the request
pub async fn get_torrent_handler(State(tracker): State<Arc<Tracker>>, Path(info_hash): Path<String>) -> Response {
    let optional_torrent_info = get_torrent_info(tracker.clone(), &InfoHash::from_str(&info_hash).unwrap()).await;

    match optional_torrent_info {
        Some(info) => Json(Torrent::from(info)).into_response(),
        None => Json(json!("torrent not known")).into_response(),
    }
}

#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

pub async fn get_torrents_handler(
    State(tracker): State<Arc<Tracker>>,
    pagination: Query<PaginationParams>,
) -> Json<Vec<ListItem>> {
    Json(ListItem::new_vec(
        &get_torrents(
            tracker.clone(),
            &Pagination::new_with_options(pagination.0.offset, pagination.0.limit),
        )
        .await,
    ))
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
