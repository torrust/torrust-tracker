//! API responses for the [`torrent`](crate::v1::context::torrent)
//! API context.
use axum::response::{IntoResponse, Json, Response};
use serde_json::json;
use torrust_tracker_core::torrent::services::{BasicInfo, Info};

use super::resources::torrent;

/// `200` response that contains an array of
/// [`ListItem`](torrust_tracker_rest_api_protocol::v1::resources::torrent::ListItem)
/// resources as json.
pub fn torrent_list_response(
    basic_infos: &[BasicInfo],
) -> Json<Vec<torrust_tracker_rest_api_protocol::v1::resources::torrent::ListItem>> {
    Json(torrent::list_items_from_domain(basic_infos))
}

/// `200` response that contains a
/// [`Torrent`](torrust_tracker_rest_api_protocol::v1::resources::torrent::Torrent)
/// resources as json.
pub fn torrent_info_response(info: Info) -> Json<torrust_tracker_rest_api_protocol::v1::resources::torrent::Torrent> {
    Json(torrent::from_domain_info(info))
}

/// `500` error response in plain text returned when a torrent is not found.
#[must_use]
pub fn torrent_not_known_response() -> Response {
    Json(json!("torrent not known")).into_response()
}
