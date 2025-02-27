//! API responses for the [`whitelist`](crate::v1::context::whitelist)
//! API context.
use std::error::Error;

use axum::response::Response;

use crate::v1::responses::unhandled_rejection_response;

/// `500` error response when a torrent cannot be removed from the whitelist.
#[must_use]
pub fn failed_to_remove_torrent_from_whitelist_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to remove torrent from whitelist: {e}"))
}

/// `500` error response when a torrent cannot be added to the whitelist.
#[must_use]
pub fn failed_to_whitelist_torrent_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to whitelist torrent: {e}"))
}

/// `500` error response when the whitelist cannot be reloaded from the database.
#[must_use]
pub fn failed_to_reload_whitelist_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to reload whitelist: {e}"))
}
