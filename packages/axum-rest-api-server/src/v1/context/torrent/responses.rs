//! API responses for the [`torrent`](crate::v1::context::torrent)
//! API context.
use axum::response::{IntoResponse, Json, Response};
use serde_json::json;
use torrust_tracker_rest_api_protocol::v1::resources::torrent::{ListItem, Torrent};

/// `200` response that contains an array of
/// [`ListItem`](torrust_tracker_rest_api_protocol::v1::resources::torrent::ListItem)
/// resources as json.
pub fn torrent_list_response(items: Vec<ListItem>) -> Json<Vec<ListItem>> {
    Json(items)
}

/// `200` response that contains a
/// [`Torrent`](torrust_tracker_rest_api_protocol::v1::resources::torrent::Torrent)
/// resources as json.
pub fn torrent_info_response(torrent: Torrent) -> Json<Torrent> {
    Json(torrent)
}

/// `500` error response in plain text returned when a torrent is not found.
#[must_use]
pub fn torrent_not_known_response() -> Response {
    Json(json!("torrent not known")).into_response()
}
