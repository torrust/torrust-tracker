use axum::response::{IntoResponse, Json, Response};
use serde_json::json;

use super::resources::torrent::{ListItem, Torrent};
use crate::tracker::services::torrent::{BasicInfo, Info};

pub fn torrent_list_response(basic_infos: &[BasicInfo]) -> Json<Vec<ListItem>> {
    Json(ListItem::new_vec(basic_infos))
}

pub fn torrent_info_response(info: Info) -> Json<Torrent> {
    Json(Torrent::from(info))
}

#[must_use]
pub fn torrent_not_known_response() -> Response {
    Json(json!("torrent not known")).into_response()
}
