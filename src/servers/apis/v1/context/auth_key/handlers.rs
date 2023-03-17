use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Path, State};
use axum::response::Response;
use serde::Deserialize;

use super::responses::{
    auth_key_response, failed_to_delete_key_response, failed_to_generate_key_response, failed_to_reload_keys_response,
};
use crate::servers::apis::v1::context::auth_key::resources::AuthKey;
use crate::servers::apis::v1::responses::{invalid_auth_key_param_response, ok_response};
use crate::tracker::auth::Key;
use crate::tracker::Tracker;

pub async fn generate_auth_key_handler(State(tracker): State<Arc<Tracker>>, Path(seconds_valid_or_key): Path<u64>) -> Response {
    let seconds_valid = seconds_valid_or_key;
    match tracker.generate_auth_key(Duration::from_secs(seconds_valid)).await {
        Ok(auth_key) => auth_key_response(&AuthKey::from(auth_key)),
        Err(e) => failed_to_generate_key_response(e),
    }
}

#[derive(Deserialize)]
pub struct KeyParam(String);

pub async fn delete_auth_key_handler(
    State(tracker): State<Arc<Tracker>>,
    Path(seconds_valid_or_key): Path<KeyParam>,
) -> Response {
    match Key::from_str(&seconds_valid_or_key.0) {
        Err(_) => invalid_auth_key_param_response(&seconds_valid_or_key.0),
        Ok(key) => match tracker.remove_auth_key(&key).await {
            Ok(_) => ok_response(),
            Err(e) => failed_to_delete_key_response(e),
        },
    }
}

pub async fn reload_keys_handler(State(tracker): State<Arc<Tracker>>) -> Response {
    match tracker.load_keys_from_database().await {
        Ok(_) => ok_response(),
        Err(e) => failed_to_reload_keys_response(e),
    }
}
