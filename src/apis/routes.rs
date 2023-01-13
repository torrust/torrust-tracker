use std::fmt;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Path, Query, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{delete, get, post};
use axum::{middleware, Router};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::json;

use super::middlewares::auth::auth;
use crate::api::resource::auth_key::AuthKey;
use crate::api::resource::stats::Stats;
use crate::api::resource::torrent::{ListItem, Torrent};
use crate::protocol::info_hash::InfoHash;
use crate::tracker::auth::KeyId;
use crate::tracker::services::statistics::{get_metrics, TrackerMetrics};
use crate::tracker::services::torrent::{get_torrent_info, get_torrents, BasicInfo, Info, Pagination};
use crate::tracker::Tracker;

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

pub fn router(tracker: &Arc<Tracker>) -> Router {
    Router::new()
        // Stats
        .route("/api/stats", get(get_stats_handler).with_state(tracker.clone()))
        // Torrents
        .route(
            "/api/torrent/:info_hash",
            get(get_torrent_handler).with_state(tracker.clone()),
        )
        .route("/api/torrents", get(get_torrents_handler).with_state(tracker.clone()))
        // Whitelisted torrents
        .route(
            "/api/whitelist/:info_hash",
            post(add_torrent_to_whitelist_handler).with_state(tracker.clone()),
        )
        .route(
            "/api/whitelist/:info_hash",
            delete(remove_torrent_from_whitelist_handler).with_state(tracker.clone()),
        )
        // Whitelist command
        .route(
            "/api/whitelist/reload",
            get(reload_whitelist_handler).with_state(tracker.clone()),
        )
        // Keys
        .route(
            // code-review: Axum does not allow two routes with the same path but different path variable name.
            // In the new major API version, `seconds_valid` should be a POST form field so that we will have two paths:
            // POST /api/key
            // DELETE /api/key/:key
            "/api/key/:seconds_valid_or_key",
            post(generate_auth_key_handler)
                .with_state(tracker.clone())
                .delete(delete_auth_key_handler)
                .with_state(tracker.clone()),
        )
        // Keys command
        .route("/api/keys/reload", get(reload_keys_handler).with_state(tracker.clone()))
        .layer(middleware::from_fn_with_state(tracker.config.clone(), auth))
}

#[derive(Serialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ActionStatus<'a> {
    Ok,
    Err { reason: std::borrow::Cow<'a, str> },
}

// Resource responses

fn response_stats(tracker_metrics: TrackerMetrics) -> Json<Stats> {
    Json(Stats::from(tracker_metrics))
}

fn response_torrent_list(basic_infos: &[BasicInfo]) -> Json<Vec<ListItem>> {
    Json(ListItem::new_vec(basic_infos))
}

fn response_torrent_info(info: Info) -> Response {
    Json(Torrent::from(info)).into_response()
}

fn response_auth_key(auth_key: &AuthKey) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        serde_json::to_string(auth_key).unwrap(),
    )
        .into_response()
}

// OK response

fn response_ok() -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        serde_json::to_string(&ActionStatus::Ok).unwrap(),
    )
        .into_response()
}

// Error responses

fn response_invalid_info_hash_param(info_hash: &str) -> Response {
    response_bad_request(&format!(
        "Invalid URL: invalid infohash param: string \"{}\", expected a 40 character long string",
        info_hash
    ))
}

fn response_invalid_auth_key_param(invalid_key: &str) -> Response {
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

fn response_torrent_not_known() -> Response {
    Json(json!("torrent not known")).into_response()
}

fn response_failed_to_remove_torrent_from_whitelist() -> Response {
    response_unhandled_rejection("failed to remove torrent from whitelist".to_string())
}

fn response_failed_to_whitelist_torrent() -> Response {
    response_unhandled_rejection("failed to whitelist torrent".to_string())
}

fn response_failed_to_reload_whitelist() -> Response {
    response_unhandled_rejection("failed to reload whitelist".to_string())
}

fn response_failed_to_generate_key() -> Response {
    response_unhandled_rejection("failed to generate key".to_string())
}

fn response_failed_to_delete_key() -> Response {
    response_unhandled_rejection("failed to delete key".to_string())
}

fn response_failed_to_reload_keys() -> Response {
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
