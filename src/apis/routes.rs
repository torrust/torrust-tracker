use std::fmt;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Path, Query, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Json, Response};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::json;

use crate::api::resource::auth_key::AuthKey;
use crate::api::resource::stats::Stats;
use crate::api::resource::torrent::{ListItem, Torrent};
use crate::protocol::info_hash::InfoHash;
use crate::tracker::services::statistics::get_metrics;
use crate::tracker::services::torrent::{get_torrent_info, get_torrents, Pagination};
use crate::tracker::Tracker;

#[derive(Serialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ActionStatus<'a> {
    Ok,
    Err { reason: std::borrow::Cow<'a, str> },
}

fn response_ok() -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        format!("{:?}", ActionStatus::Ok),
    )
        .into_response()
}

fn response_err(reason: String) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        format!("Unhandled rejection: {:?}", ActionStatus::Err { reason: reason.into() }),
    )
        .into_response()
}

fn response_auth_key(auth_key: &AuthKey) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        serde_json::to_string(auth_key).unwrap(),
    )
        .into_response()
}

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

/// # Panics
///
/// Will panic if it can't parse the infohash in the request
pub async fn add_torrent_to_whitelist_handler(State(tracker): State<Arc<Tracker>>, Path(info_hash): Path<String>) -> Response {
    match tracker
        .add_torrent_to_whitelist(&InfoHash::from_str(&info_hash).unwrap())
        .await
    {
        Ok(..) => response_ok(),
        Err(..) => response_err("failed to whitelist torrent".to_string()),
    }
}

/// # Panics
///
/// Will panic if it can't parse the infohash in the request
pub async fn delete_torrent_from_whitelist_handler(
    State(tracker): State<Arc<Tracker>>,
    Path(info_hash): Path<String>,
) -> Response {
    match tracker
        .remove_torrent_from_whitelist(&InfoHash::from_str(&info_hash).unwrap())
        .await
    {
        Ok(..) => response_ok(),
        Err(..) => response_err("failed to remove torrent from whitelist".to_string()),
    }
}

pub async fn reload_whitelist_handler(State(tracker): State<Arc<Tracker>>) -> Response {
    match tracker.load_whitelist().await {
        Ok(..) => response_ok(),
        Err(..) => response_err("failed to reload whitelist".to_string()),
    }
}

pub async fn generate_auth_key_handler(State(tracker): State<Arc<Tracker>>, Path(seconds_valid): Path<u64>) -> Response {
    match tracker.generate_auth_key(Duration::from_secs(seconds_valid)).await {
        Ok(auth_key) => response_auth_key(&AuthKey::from(auth_key)),
        Err(_) => response_err("failed to generate key".to_string()),
    }
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
