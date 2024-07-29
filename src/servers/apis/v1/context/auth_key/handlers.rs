//! API handlers for the [`auth_key`](crate::servers::apis::v1::context::auth_key) API context.
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{self, Path, State};
use axum::response::Response;
use serde::Deserialize;
use torrust_tracker_clock::clock::Time;

use super::forms::AddKeyForm;
use super::responses::{
    auth_key_response, failed_to_add_key_response, failed_to_delete_key_response, failed_to_generate_key_response,
    failed_to_reload_keys_response, invalid_auth_key_duration_response, invalid_auth_key_response,
};
use crate::core::auth::Key;
use crate::core::Tracker;
use crate::servers::apis::v1::context::auth_key::resources::AuthKey;
use crate::servers::apis::v1::responses::{invalid_auth_key_param_response, ok_response};
use crate::CurrentClock;

/// It handles the request to add a new authentication key.
///
/// It returns these types of responses:
///
/// - `200` with a json [`AuthKey`]
///    resource. If the key was generated successfully.
/// - `400` with an error if the key couldn't been added because of an invalid
///    request.
/// - `500` with serialized error in debug format. If the key couldn't be
///    generated.
///
/// Refer to the [API endpoint documentation](crate::servers::apis::v1::context::auth_key#generate-a-new-authentication-key)
/// for more information about this endpoint.
pub async fn add_auth_key_handler(
    State(tracker): State<Arc<Tracker>>,
    extract::Json(add_key_form): extract::Json<AddKeyForm>,
) -> Response {
    match add_key_form.opt_key {
        Some(pre_existing_key) => {
            let Some(valid_until) = CurrentClock::now_add(&Duration::from_secs(add_key_form.seconds_valid)) else {
                return invalid_auth_key_duration_response(add_key_form.seconds_valid);
            };

            let key = pre_existing_key.parse::<Key>();

            match key {
                Ok(key) => match tracker.add_auth_key(key, valid_until).await {
                    Ok(auth_key) => auth_key_response(&AuthKey::from(auth_key)),
                    Err(e) => failed_to_add_key_response(e),
                },
                Err(e) => invalid_auth_key_response(&pre_existing_key, &e),
            }
        }
        None => {
            match tracker
                .generate_auth_key(Duration::from_secs(add_key_form.seconds_valid))
                .await
            {
                Ok(auth_key) => auth_key_response(&AuthKey::from(auth_key)),
                Err(e) => failed_to_generate_key_response(e),
            }
        }
    }
}

/// It handles the request to generate a new authentication key.
///
/// It returns two types of responses:
///
/// - `200` with an json [`AuthKey`]
///    resource. If the key was generated successfully.
/// - `500` with serialized error in debug format. If the key couldn't be
///    generated.
///
/// Refer to the [API endpoint documentation](crate::servers::apis::v1::context::auth_key#generate-a-new-authentication-key)
/// for more information about this endpoint.
pub async fn generate_auth_key_handler(State(tracker): State<Arc<Tracker>>, Path(seconds_valid_or_key): Path<u64>) -> Response {
    let seconds_valid = seconds_valid_or_key;
    match tracker.generate_auth_key(Duration::from_secs(seconds_valid)).await {
        Ok(auth_key) => auth_key_response(&AuthKey::from(auth_key)),
        Err(e) => failed_to_generate_key_response(e),
    }
}

/// A container for the `key` parameter extracted from the URL PATH.
///
/// It does not perform any validation, it just stores the value.
///
/// In the current API version, the `key` parameter can be either a valid key
/// like `xqD6NWH9TcKrOCwDmqcdH5hF5RrbL0A6` or the number of seconds the
/// key will be valid, for example two minutes `120`.
///
/// For example, the `key` is used in the following requests:
///
/// - `POST /api/v1/key/120`. It will generate a new key valid for two minutes.
/// - `DELETE /api/v1/key/xqD6NWH9TcKrOCwDmqcdH5hF5RrbL0A6`. It will delete the
///   key `xqD6NWH9TcKrOCwDmqcdH5hF5RrbL0A6`.
///
/// > **NOTICE**: this may change in the future, in the [API v2](https://github.com/torrust/torrust-tracker/issues/144).
#[derive(Deserialize)]
pub struct KeyParam(String);

/// It handles the request to delete an authentication key.
///
/// It returns two types of responses:
///
/// - `200` with an json [`ActionStatus::Ok`](crate::servers::apis::v1::responses::ActionStatus::Ok)
///    response. If the key was deleted successfully.
/// - `500` with serialized error in debug format. If the key couldn't be
///    deleted.
///
/// Refer to the [API endpoint documentation](crate::servers::apis::v1::context::auth_key#delete-an-authentication-key)
/// for more information about this endpoint.
pub async fn delete_auth_key_handler(
    State(tracker): State<Arc<Tracker>>,
    Path(seconds_valid_or_key): Path<KeyParam>,
) -> Response {
    match Key::from_str(&seconds_valid_or_key.0) {
        Err(_) => invalid_auth_key_param_response(&seconds_valid_or_key.0),
        Ok(key) => match tracker.remove_auth_key(&key).await {
            Ok(()) => ok_response(),
            Err(e) => failed_to_delete_key_response(e),
        },
    }
}

/// It handles the request to reload the authentication keys from the database
/// into memory.
///
/// It returns two types of responses:
///
/// - `200` with an json [`ActionStatus::Ok`](crate::servers::apis::v1::responses::ActionStatus::Ok)
///    response. If the keys were successfully reloaded.
/// - `500` with serialized error in debug format. If the they couldn't be
///    reloaded.
///
/// Refer to the [API endpoint documentation](crate::servers::apis::v1::context::auth_key#reload-authentication-keys)
/// for more information about this endpoint.
pub async fn reload_keys_handler(State(tracker): State<Arc<Tracker>>) -> Response {
    match tracker.load_keys_from_database().await {
        Ok(()) => ok_response(),
        Err(e) => failed_to_reload_keys_response(e),
    }
}
