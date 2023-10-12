//! API handlers for the [`torrent`](crate::servers::apis::v1::context::torrent)
//! API context.
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Json, Response};
use log::debug;
use serde::{de, Deserialize, Deserializer};

use super::resources::torrent::ListItem;
use super::responses::{torrent_info_response, torrent_list_response, torrent_not_known_response};
use crate::servers::apis::v1::responses::invalid_info_hash_param_response;
use crate::servers::apis::InfoHashParam;
use crate::shared::bit_torrent::info_hash::InfoHash;
use crate::tracker::services::torrent::{get_torrent_info, get_torrents, Pagination};
use crate::tracker::Tracker;

/// It handles the request to get the torrent data.
///
/// It returns:
///
/// - `200` response with a json [`Torrent`](crate::servers::apis::v1::context::torrent::resources::torrent::Torrent).
/// - `500` with serialized error in debug format if the torrent is not known.
///
/// Refer to the [API endpoint documentation](crate::servers::apis::v1::context::torrent#get-a-torrent)
/// for more information about this endpoint.
pub async fn get_torrent_handler(State(tracker): State<Arc<Tracker>>, Path(info_hash): Path<InfoHashParam>) -> Response {
    match InfoHash::from_str(&info_hash.0) {
        Err(_) => invalid_info_hash_param_response(&info_hash.0),
        Ok(info_hash) => match get_torrent_info(tracker.clone(), &info_hash).await {
            Some(info) => torrent_info_response(info).into_response(),
            None => torrent_not_known_response(),
        },
    }
}

/// A container for the optional URL query pagination parameters:
/// `offset` and `limit`.
#[derive(Deserialize, Debug)]
pub struct PaginationParams {
    /// The offset of the first page to return. Starts at 0.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub offset: Option<u32>,
    /// The maximum number of items to return per page
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub limit: Option<u32>,
}

/// It handles the request to get a list of torrents.
///
/// It returns a `200` response with a json array with
/// [`ListItem`]
/// resources.
///
/// Refer to the [API endpoint documentation](crate::servers::apis::v1::context::torrent#list-torrents)
/// for more information about this endpoint.
pub async fn get_torrents_handler(
    State(tracker): State<Arc<Tracker>>,
    pagination: Query<PaginationParams>,
) -> Json<Vec<ListItem>> {
    debug!("pagination: {:?}", pagination);

    torrent_list_response(
        &get_torrents(
            tracker.clone(),
            &Pagination::new_with_options(pagination.0.offset, pagination.0.limit),
        )
        .await,
    )
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
