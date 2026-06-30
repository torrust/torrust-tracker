//! API handlers for the [`auth_key`](crate::v1::context::auth_key) API context.
use std::sync::Arc;

use axum::extract::{self, Path, State};
use axum::response::Response;
use serde::Deserialize;
use torrust_tracker_rest_api_application::v1::use_cases::auth_key::AuthKeyApiService;
use torrust_tracker_rest_api_protocol::v1::context::auth_key::forms::add_key_form::AddKeyForm;

use super::responses::{
    auth_key_response, failed_to_add_key_response, failed_to_delete_key_response, failed_to_generate_key_response,
    failed_to_reload_keys_response, invalid_auth_key_duration_response, invalid_auth_key_response,
};
use crate::v1::responses::{invalid_auth_key_param_response, ok_response};

/// It handles the request to add a new authentication key.
///
/// It returns these types of responses:
///
/// - `200` with a json [`AuthKey`]
///   resource. If the key was generated successfully.
/// - `400` with an error if the key couldn't been added because of an invalid
///   request.
/// - `500` with serialized error in debug format. If the key couldn't be
///   generated.
///
/// Refer to the [API endpoint documentation](crate::v1::context::auth_key#generate-a-new-authentication-key)
/// for more information about this endpoint.
pub async fn add_auth_key_handler(
    State(auth_key_service): State<Arc<AuthKeyApiService>>,
    extract::Json(add_key_form): extract::Json<AddKeyForm>,
) -> Response {
    match auth_key_service.add_key(&add_key_form).await {
        Ok(auth_key) => auth_key_response(&auth_key),
        Err(err) => match &err {
            torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKeyError::DurationOverflow {
                seconds_valid,
            } => invalid_auth_key_duration_response(*seconds_valid),
            torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKeyError::InvalidKey {
                key,
                reason,
            } => invalid_auth_key_response(key, reason),
            torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKeyError::Database(_) => {
                failed_to_add_key_response(AuthKeyErrorDisplay(&err))
            }
        },
    }
}

/// It handles the request to generate a new authentication key.
///
/// It returns two types of responses:
///
/// - `200` with an json [`AuthKey`]
///   resource. If the key was generated successfully.
/// - `500` with serialized error in debug format. If the key couldn't be
///   generated.
///
/// Refer to the [API endpoint documentation](crate::v1::context::auth_key#generate-a-new-authentication-key)
/// for more information about this endpoint.
///
/// This endpoint has been deprecated. Use [`add_auth_key_handler`].
pub async fn generate_auth_key_handler(
    State(auth_key_service): State<Arc<AuthKeyApiService>>,
    Path(seconds_valid_or_key): Path<u64>,
) -> Response {
    let seconds_valid = seconds_valid_or_key;
    match auth_key_service.generate_key(seconds_valid).await {
        Ok(auth_key) => auth_key_response(&auth_key),
        Err(e) => failed_to_generate_key_response(AuthKeyErrorDisplay(&e)),
    }
}

/// A container for the `key` parameter extracted from the URL PATH.
///
/// It does not perform any validation, it just stores the value.
#[derive(Deserialize)]
pub struct KeyParam(String);

/// It handles the request to delete an authentication key.
///
/// It returns two types of responses:
///
/// - `200` with an json [`ActionStatus::Ok`](crate::v1::responses::ActionStatus::Ok)
///   response. If the key was deleted successfully.
/// - `500` with serialized error in debug format. If the key couldn't be
///   deleted.
///
/// Refer to the [API endpoint documentation](crate::v1::context::auth_key#delete-an-authentication-key)
/// for more information about this endpoint.
pub async fn delete_auth_key_handler(
    State(auth_key_service): State<Arc<AuthKeyApiService>>,
    Path(seconds_valid_or_key): Path<KeyParam>,
) -> Response {
    match auth_key_service.delete_key(&seconds_valid_or_key.0).await {
        Ok(()) => ok_response(),
        Err(torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKeyError::InvalidKey {
            key: _,
            reason: _,
        }) => invalid_auth_key_param_response(&seconds_valid_or_key.0),
        Err(e) => failed_to_delete_key_response(AuthKeyErrorDisplay(&e)),
    }
}

/// It handles the request to reload the authentication keys from the database
/// into memory.
///
/// It returns two types of responses:
///
/// - `200` with an json [`ActionStatus::Ok`](crate::v1::responses::ActionStatus::Ok)
///   response. If the keys were successfully reloaded.
/// - `500` with serialized error in debug format. If the they couldn't be
///   reloaded.
///
/// Refer to the [API endpoint documentation](crate::v1::context::auth_key#reload-authentication-keys)
/// for more information about this endpoint.
pub async fn reload_keys_handler(State(auth_key_service): State<Arc<AuthKeyApiService>>) -> Response {
    match auth_key_service.reload_keys().await {
        Ok(()) => ok_response(),
        Err(e) => failed_to_reload_keys_response(AuthKeyErrorDisplay(&e)),
    }
}

/// Wrapper to allow passing an [`AuthKeyError`] reference to response
/// functions that expect `E: std::error::Error`.
struct AuthKeyErrorDisplay<'a>(&'a torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKeyError);

impl std::fmt::Display for AuthKeyErrorDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.0, f)
    }
}

impl std::error::Error for AuthKeyErrorDisplay<'_> {}

impl std::fmt::Debug for AuthKeyErrorDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.0, f)
    }
}
