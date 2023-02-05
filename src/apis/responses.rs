use std::error::Error;

use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Json, Response};
use serde::Serialize;
use serde_json::json;

use crate::apis::resources::auth_key::AuthKey;
use crate::apis::resources::stats::Stats;
use crate::apis::resources::torrent::{ListItem, Torrent};
use crate::tracker::services::statistics::TrackerMetrics;
use crate::tracker::services::torrent::{BasicInfo, Info};

/* code-review:
    When Axum cannot parse a path or query param it shows a message like this:

    For the "seconds_valid_or_key" path param:

    "Invalid URL: Cannot parse "-1" to a `u64`"

    That message is not an informative message, specially if you have more than one param.
    We should show a message similar to the one we use when we parse the value in the handler.
    For example:

    "Invalid URL: invalid infohash param: string \"INVALID VALUE\", expected a 40 character long string"

    We can customize the error message by using a custom type with custom serde deserialization.
    The same we are using for the "InfoHashVisitor".

    Input data from HTTP requests should use struts with primitive types (first level of validation).
    We can put the second level of validation in the application and domain services.
*/

#[derive(Serialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ActionStatus<'a> {
    Ok,
    Err { reason: std::borrow::Cow<'a, str> },
}

// Resource responses

#[must_use]
pub fn stats_response(tracker_metrics: TrackerMetrics) -> Json<Stats> {
    Json(Stats::from(tracker_metrics))
}

#[must_use]
pub fn torrent_list_response(basic_infos: &[BasicInfo]) -> Json<Vec<ListItem>> {
    Json(ListItem::new_vec(basic_infos))
}

#[must_use]
pub fn torrent_info_response(info: Info) -> Json<Torrent> {
    Json(Torrent::from(info))
}

/// # Panics
///
/// Will panic if it can't convert the `AuthKey` resource to json
#[must_use]
pub fn auth_key_response(auth_key: &AuthKey) -> Response {
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
pub fn ok_response() -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        serde_json::to_string(&ActionStatus::Ok).unwrap(),
    )
        .into_response()
}

// Error responses

#[must_use]
pub fn invalid_info_hash_param_response(info_hash: &str) -> Response {
    bad_request_response(&format!(
        "Invalid URL: invalid infohash param: string \"{info_hash}\", expected a 40 character long string"
    ))
}

#[must_use]
pub fn invalid_auth_key_param_response(invalid_key: &str) -> Response {
    bad_request_response(&format!("Invalid auth key id param \"{invalid_key}\""))
}

fn bad_request_response(body: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        body.to_owned(),
    )
        .into_response()
}

#[must_use]
pub fn torrent_not_known_response() -> Response {
    Json(json!("torrent not known")).into_response()
}

#[must_use]
pub fn failed_to_remove_torrent_from_whitelist_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to remove torrent from whitelist: {e}").to_string())
}

#[must_use]
pub fn failed_to_whitelist_torrent_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to whitelist torrent: {e}").to_string())
}

#[must_use]
pub fn failed_to_reload_whitelist_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to reload whitelist: {e}").to_string())
}

#[must_use]
pub fn failed_to_generate_key_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to generate key: {e}").to_string())
}

#[must_use]
pub fn failed_to_delete_key_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to delete key: {e}").to_string())
}

#[must_use]
pub fn failed_to_reload_keys_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to reload keys: {e}").to_string())
}

/// This error response is to keep backward compatibility with the old Warp API.
/// It should be a plain text or json.
#[must_use]
pub fn unhandled_rejection_response(reason: String) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        format!("Unhandled rejection: {:?}", ActionStatus::Err { reason: reason.into() }),
    )
        .into_response()
}
