use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Json, Response};
use serde::Serialize;
use serde_json::json;

use crate::apis::resources::auth_key::AuthKey;
use crate::apis::resources::stats::Stats;
use crate::apis::resources::torrent::{ListItem, Torrent};
use crate::tracker::services::statistics::TrackerMetrics;
use crate::tracker::services::torrent::{BasicInfo, Info};

#[derive(Serialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ActionStatus<'a> {
    Ok,
    Err { reason: std::borrow::Cow<'a, str> },
}

// Resource responses

#[must_use]
pub fn response_stats(tracker_metrics: TrackerMetrics) -> Json<Stats> {
    Json(Stats::from(tracker_metrics))
}

#[must_use]
pub fn response_torrent_list(basic_infos: &[BasicInfo]) -> Json<Vec<ListItem>> {
    Json(ListItem::new_vec(basic_infos))
}

#[must_use]
pub fn response_torrent_info(info: Info) -> Response {
    Json(Torrent::from(info)).into_response()
}

/// # Panics
///
/// Will panic if it can't convert the `AuthKey` resource to json
#[must_use]
pub fn response_auth_key(auth_key: &AuthKey) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        serde_json::to_string(auth_key).unwrap(),
    )
        .into_response()
}

// OK response

/// # Panics
///
/// Will panic if it can't convert the `ActionStatus` to json
#[must_use]
pub fn response_ok() -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        serde_json::to_string(&ActionStatus::Ok).unwrap(),
    )
        .into_response()
}

// Error responses

#[must_use]
pub fn response_invalid_info_hash_param(info_hash: &str) -> Response {
    response_bad_request(&format!(
        "Invalid URL: invalid infohash param: string \"{}\", expected a 40 character long string",
        info_hash
    ))
}

#[must_use]
pub fn response_invalid_auth_key_param(invalid_key: &str) -> Response {
    response_bad_request(&format!("Invalid auth key id param \"{invalid_key}\""))
}

fn response_bad_request(body: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        body.to_owned(),
    )
        .into_response()
}

#[must_use]
pub fn response_torrent_not_known() -> Response {
    Json(json!("torrent not known")).into_response()
}

#[must_use]
pub fn response_failed_to_remove_torrent_from_whitelist() -> Response {
    response_unhandled_rejection("failed to remove torrent from whitelist".to_string())
}

#[must_use]
pub fn response_failed_to_whitelist_torrent() -> Response {
    response_unhandled_rejection("failed to whitelist torrent".to_string())
}

#[must_use]
pub fn response_failed_to_reload_whitelist() -> Response {
    response_unhandled_rejection("failed to reload whitelist".to_string())
}

#[must_use]
pub fn response_failed_to_generate_key() -> Response {
    response_unhandled_rejection("failed to generate key".to_string())
}

#[must_use]
pub fn response_failed_to_delete_key() -> Response {
    response_unhandled_rejection("failed to delete key".to_string())
}

#[must_use]
pub fn response_failed_to_reload_keys() -> Response {
    response_unhandled_rejection("failed to reload keys".to_string())
}

fn response_unhandled_rejection(reason: String) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        format!("Unhandled rejection: {:?}", ActionStatus::Err { reason: reason.into() }),
    )
        .into_response()
}
