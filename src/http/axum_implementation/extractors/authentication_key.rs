//! Wrapper for Axum `Path` extractor to return custom errors.
use std::panic::Location;

use axum::async_trait;
use axum::extract::rejection::PathRejection;
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::http::axum_implementation::handlers::common::auth;
use crate::http::axum_implementation::responses;
use crate::tracker::auth::Key;

pub struct Extract(pub Key);

#[derive(Deserialize)]
pub struct KeyParam(String);

impl KeyParam {
    #[must_use]
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Extract
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract `key` from URL path with Axum `Path` extractor
        let maybe_path_with_key = Path::<KeyParam>::from_request_parts(parts, state).await;

        match extract_key(maybe_path_with_key) {
            Ok(key) => Ok(Extract(key)),
            Err(error) => Err(error.into_response()),
        }
    }
}

fn extract_key(path_extractor_result: Result<Path<KeyParam>, PathRejection>) -> Result<Key, responses::error::Error> {
    match path_extractor_result {
        Ok(key_param) => match parse_key(&key_param.0.value()) {
            Ok(key) => Ok(key),
            Err(error) => Err(error),
        },
        Err(path_rejection) => Err(custom_error(&path_rejection)),
    }
}

fn parse_key(key: &str) -> Result<Key, responses::error::Error> {
    let key = key.parse::<Key>();

    match key {
        Ok(key) => Ok(key),
        Err(_parse_key_error) => Err(responses::error::Error::from(auth::Error::InvalidKeyFormat {
            location: Location::caller(),
        })),
    }
}

fn custom_error(rejection: &PathRejection) -> responses::error::Error {
    match rejection {
        axum::extract::rejection::PathRejection::FailedToDeserializePathParams(_) => {
            responses::error::Error::from(auth::Error::InvalidKeyFormat {
                location: Location::caller(),
            })
        }
        axum::extract::rejection::PathRejection::MissingPathParams(_) => {
            responses::error::Error::from(auth::Error::MissingAuthKey {
                location: Location::caller(),
            })
        }
        _ => responses::error::Error::from(auth::Error::CannotExtractKeyParam {
            location: Location::caller(),
        }),
    }
}

#[cfg(test)]
mod tests {

    use super::parse_key;
    use crate::http::axum_implementation::responses::error::Error;

    fn assert_error_response(error: &Error, error_message: &str) {
        assert!(
            error.failure_reason.contains(error_message),
            "Error response does not contain message: '{error_message}'. Error: {error:?}"
        );
    }

    #[test]
    fn it_should_return_an_authentication_error_if_the_key_cannot_be_parsed() {
        let invalid_key = "invalid_key";

        let response = parse_key(invalid_key).unwrap_err();

        assert_error_response(&response, "Authentication error: Invalid format for authentication key param");
    }
}
