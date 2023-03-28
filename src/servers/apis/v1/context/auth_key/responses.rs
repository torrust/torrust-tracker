//! API responses for the [`auth_key`](crate::servers::apis::v1::context::auth_key) API context.
use std::error::Error;

use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};

use crate::servers::apis::v1::context::auth_key::resources::AuthKey;
use crate::servers::apis::v1::responses::unhandled_rejection_response;

/// `200` response that contains the `AuthKey` resource as json.
///
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

/// `500` error response when a new authentication key cannot be generated.
#[must_use]
pub fn failed_to_generate_key_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to generate key: {e}"))
}

/// `500` error response when an authentication key cannot be deleted.
#[must_use]
pub fn failed_to_delete_key_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to delete key: {e}"))
}

/// `500` error response when the authentication keys cannot be reloaded from
/// the database into memory.
#[must_use]
pub fn failed_to_reload_keys_response<E: Error>(e: E) -> Response {
    unhandled_rejection_response(format!("failed to reload keys: {e}"))
}
