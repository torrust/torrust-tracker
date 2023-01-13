use std::fmt;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Path, Query, State};
use axum::response::{Json, Response};
use serde::{de, Deserialize, Deserializer};

use super::responses::{
    response_auth_key, response_failed_to_delete_key, response_failed_to_generate_key, response_failed_to_reload_keys,
    response_failed_to_reload_whitelist, response_failed_to_remove_torrent_from_whitelist, response_failed_to_whitelist_torrent,
    response_invalid_auth_key_param, response_invalid_info_hash_param, response_ok, response_stats, response_torrent_info,
    response_torrent_list, response_torrent_not_known,
};
use crate::apis::resources::auth_key::AuthKey;
use crate::apis::resources::stats::Stats;
use crate::apis::resources::torrent::ListItem;
use crate::protocol::info_hash::InfoHash;
use crate::tracker::auth::KeyId;
use crate::tracker::services::statistics::get_metrics;
use crate::tracker::services::torrent::{get_torrent_info, get_torrents, Pagination};
use crate::tracker::Tracker;

pub async fn get_stats_handler(State(tracker): State<Arc<Tracker>>) -> Json<Stats> {
    response_stats(get_metrics(tracker.clone()).await)
}

#[derive(Deserialize)]
pub struct InfoHashParam(String);

pub async fn get_torrent_handler(State(tracker): State<Arc<Tracker>>, Path(info_hash): Path<InfoHashParam>) -> Response {
    match InfoHash::from_str(&info_hash.0) {
        Err(_) => response_invalid_info_hash_param(&info_hash.0),
        Ok(info_hash) => match get_torrent_info(tracker.clone(), &info_hash).await {
            Some(info) => response_torrent_info(info),
            None => response_torrent_not_known(),
        },
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
    response_torrent_list(
        &get_torrents(
            tracker.clone(),
            &Pagination::new_with_options(pagination.0.offset, pagination.0.limit),
        )
        .await,
    )
}

pub async fn add_torrent_to_whitelist_handler(
    State(tracker): State<Arc<Tracker>>,
    Path(info_hash): Path<InfoHashParam>,
) -> Response {
    match InfoHash::from_str(&info_hash.0) {
        Err(_) => response_invalid_info_hash_param(&info_hash.0),
        Ok(info_hash) => match tracker.add_torrent_to_whitelist(&info_hash).await {
            Ok(..) => response_ok(),
            Err(..) => response_failed_to_whitelist_torrent(),
        },
    }
}

pub async fn remove_torrent_from_whitelist_handler(
    State(tracker): State<Arc<Tracker>>,
    Path(info_hash): Path<InfoHashParam>,
) -> Response {
    match InfoHash::from_str(&info_hash.0) {
        Err(_) => response_invalid_info_hash_param(&info_hash.0),
        Ok(info_hash) => match tracker.remove_torrent_from_whitelist(&info_hash).await {
            Ok(..) => response_ok(),
            Err(..) => response_failed_to_remove_torrent_from_whitelist(),
        },
    }
}

pub async fn reload_whitelist_handler(State(tracker): State<Arc<Tracker>>) -> Response {
    match tracker.load_whitelist().await {
        Ok(..) => response_ok(),
        Err(..) => response_failed_to_reload_whitelist(),
    }
}

pub async fn generate_auth_key_handler(State(tracker): State<Arc<Tracker>>, Path(seconds_valid_or_key): Path<u64>) -> Response {
    let seconds_valid = seconds_valid_or_key;
    match tracker.generate_auth_key(Duration::from_secs(seconds_valid)).await {
        Ok(auth_key) => response_auth_key(&AuthKey::from(auth_key)),
        Err(_) => response_failed_to_generate_key(),
    }
}

#[derive(Deserialize)]
pub struct KeyIdParam(String);

pub async fn delete_auth_key_handler(
    State(tracker): State<Arc<Tracker>>,
    Path(seconds_valid_or_key): Path<KeyIdParam>,
) -> Response {
    match KeyId::from_str(&seconds_valid_or_key.0) {
        Err(_) => response_invalid_auth_key_param(&seconds_valid_or_key.0),
        Ok(key_id) => match tracker.remove_auth_key(&key_id.to_string()).await {
            Ok(_) => response_ok(),
            Err(_) => response_failed_to_delete_key(),
        },
    }
}

pub async fn reload_keys_handler(State(tracker): State<Arc<Tracker>>) -> Response {
    match tracker.load_keys().await {
        Ok(..) => response_ok(),
        Err(..) => response_failed_to_reload_keys(),
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
