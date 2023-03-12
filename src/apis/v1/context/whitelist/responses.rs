use std::error::Error;

use axum::response::Response;

use crate::apis::v1::responses::unhandled_rejection_response;

#[must_use]
pub fn failed_to_remove_torrent_from_whitelist_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to remove torrent from whitelist: {e}"))
}

#[must_use]
pub fn failed_to_whitelist_torrent_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to whitelist torrent: {e}"))
}

#[must_use]
pub fn failed_to_reload_whitelist_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to reload whitelist: {e}"))
}
