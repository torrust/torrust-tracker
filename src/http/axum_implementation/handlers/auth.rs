use std::panic::Location;
use std::sync::Arc;

use thiserror::Error;

use crate::http::axum_implementation::responses;
use crate::tracker::auth::{self, KeyId};
use crate::tracker::Tracker;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing authentication key for private tracker. Error in {location}")]
    MissingAuthKey { location: &'static Location<'static> },
}

/// # Errors
///
/// Will return an error if the the authentication key cannot be verified.
pub async fn authenticate(key_id: &KeyId, tracker: &Arc<Tracker>) -> Result<(), auth::Error> {
    if tracker.is_private() {
        tracker.verify_auth_key(key_id).await
    } else {
        Ok(())
    }
}

impl From<Error> for responses::error::Error {
    fn from(err: Error) -> Self {
        responses::error::Error {
            failure_reason: format!("Authentication error: {err}"),
        }
    }
}

impl From<auth::Error> for responses::error::Error {
    fn from(err: auth::Error) -> Self {
        responses::error::Error {
            failure_reason: format!("Authentication error: {err}"),
        }
    }
}
