use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::Response;

use super::responses::{
    failed_to_reload_whitelist_response, failed_to_remove_torrent_from_whitelist_response, failed_to_whitelist_torrent_response,
};
use crate::apis::responses::{invalid_info_hash_param_response, ok_response};
use crate::apis::InfoHashParam;
use crate::protocol::info_hash::InfoHash;
use crate::tracker::Tracker;

pub async fn add_torrent_to_whitelist_handler(
    State(tracker): State<Arc<Tracker>>,
    Path(info_hash): Path<InfoHashParam>,
) -> Response {
    match InfoHash::from_str(&info_hash.0) {
        Err(_) => invalid_info_hash_param_response(&info_hash.0),
        Ok(info_hash) => match tracker.add_torrent_to_whitelist(&info_hash).await {
            Ok(_) => ok_response(),
            Err(e) => failed_to_whitelist_torrent_response(e),
        },
    }
}

pub async fn remove_torrent_from_whitelist_handler(
    State(tracker): State<Arc<Tracker>>,
    Path(info_hash): Path<InfoHashParam>,
) -> Response {
    match InfoHash::from_str(&info_hash.0) {
        Err(_) => invalid_info_hash_param_response(&info_hash.0),
        Ok(info_hash) => match tracker.remove_torrent_from_whitelist(&info_hash).await {
            Ok(_) => ok_response(),
            Err(e) => failed_to_remove_torrent_from_whitelist_response(e),
        },
    }
}

pub async fn reload_whitelist_handler(State(tracker): State<Arc<Tracker>>) -> Response {
    match tracker.load_whitelist_from_database().await {
        Ok(_) => ok_response(),
        Err(e) => failed_to_reload_whitelist_response(e),
    }
}
