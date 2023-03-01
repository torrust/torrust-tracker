use std::panic::Location;

use serde::Deserialize;
use thiserror::Error;

use crate::http::axum_implementation::responses;
use crate::tracker::auth;

#[derive(Deserialize)]
pub struct KeyIdParam(String);

impl KeyIdParam {
    #[must_use]
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing authentication key for private tracker. Error in {location}")]
    MissingAuthKey { location: &'static Location<'static> },
    #[error("Invalid format authentication key. Error in {location}")]
    InvalidKeyFormat { location: &'static Location<'static> },
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
