//! Axum [`extractor`](axum::extract) to extract the authentication [`Key`]
//! from the URL path.
//!
//! It's only used when the tracker is running in private mode.
//!
//! Given the following URL route with a path param: `/announce/:key`,
//! it extracts the `key` param from the URL path.
//!
//! It's a wrapper for Axum `Path` extractor in order to return custom
//! authentication errors.
//!
//! It returns a bencoded [`Error`](torrust_tracker_http_protocol::v1::responses::error)
//! response with HTTP status `200 OK` if the `key` parameter is missing or invalid.
//!
//! **Sample authentication error responses**
//!
//! When the key param is **missing**:
//!
//! ```text
//! d14:failure reason131:Authentication error: Missing authentication key param for private tracker. Error in src/servers/http/v1/handlers/announce.rs:79:31e
//! ```
//!
//! When the key param has an **invalid format**:
//!
//! ```text
//! d14:failure reason134:Authentication error: Invalid format for authentication key param. Error in src/servers/http/v1/extractors/authentication_key.rs:73:23e
//! ```
//!
//! When the key is **not found** in the database:
//!
//! ```text
//! d14:failure reason101:Authentication error: Failed to read key: YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ, src/tracker/mod.rs:848:27e
//! ```
//!
//! When the key is found in the database but it's **expired**:
//!
//! ```text
//! d14:failure reason64:Authentication error: Key has expired, src/tracker/auth.rs:88:23e
//! ```
//!
//! > **NOTICE**: the returned HTTP status code is always `200` for authentication errors.
//! > Neither [The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
//! > nor [The Private Torrents](https://www.bittorrent.org/beps/bep_0027.html)
//! > specifications specify any HTTP status code for authentication errors.
use std::future::Future;
use std::panic::Location;

use axum::extract::rejection::PathRejection;
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::Deserialize;
use torrust_tracker_core::authentication::Key;
use torrust_tracker_http_protocol::v1::{auth, responses};

/// Extractor for the [`Key`] struct.
pub struct Extract(pub Key);

#[derive(Deserialize)]
pub struct KeyParam(String);

impl KeyParam {
    #[must_use]
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl<S> FromRequestParts<S> for Extract
where
    S: Send + Sync + 'static,
{
    type Rejection = Response;

    #[allow(clippy::manual_async_fn)]
    fn from_request_parts(parts: &mut Parts, state: &S) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // Extract `key` from URL path with Axum `Path` extractor
            let maybe_path_with_key = Path::<KeyParam>::from_request_parts(parts, state).await;

            match extract_key(maybe_path_with_key) {
                Ok(key) => Ok(Extract(key)),
                Err(error) => Err((StatusCode::OK, error.write()).into_response()),
            }
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
        _ => responses::error::Error::from(auth::Error::CannotExtractKeyParam {
            location: Location::caller(),
        }),
    }
}

#[cfg(test)]
mod tests {

    use axum::body::Body;
    use axum::body::to_bytes;
    use axum::http::StatusCode;
    use axum::response::Response;
    use axum::routing::get;
    use axum::{Router, response::IntoResponse};
    use torrust_tracker_http_protocol::v1::responses::error::Error;
    use tower::ServiceExt;

    use super::{Extract, Key, parse_key};

    const MAX_RESPONSE_BODY_BYTES: usize = 64 * 1024;

    async fn protected_handler(Extract(_key): Extract) -> impl IntoResponse {
        StatusCode::NO_CONTENT
    }

    async fn decode_bencoded_failure_response(response: Response) -> Error {
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "a BitTorrent authentication failure response should use HTTP 200"
        );

        let body = to_bytes(response.into_body(), MAX_RESPONSE_BODY_BYTES)
            .await
            .expect("the failure response body should be readable");

        serde_bencode::from_bytes(&body).expect("the failure response should be valid bencode")
    }

    fn assert_failure_reason_contains(error: &Error, error_message: &str) {
        assert!(
            error.failure_reason.contains(error_message),
            "Error response does not contain message: '{error_message}'. Error: {error:?}"
        );
    }

    #[test]
    fn it_should_map_an_invalid_path_key_to_an_invalid_key_format_authentication_failure() {
        // Arrange
        let invalid_key = "invalid_key";

        // Act
        let actual_error_response = parse_key(invalid_key).unwrap_err();

        // Assert
        assert_failure_reason_contains(
            &actual_error_response,
            "Tracker authentication error: Invalid format for authentication key param",
        );
    }

    #[test]
    fn it_should_parse_a_valid_path_key() {
        // Arrange
        let valid_key = "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ";
        let expected_key = valid_key
            .parse::<Key>()
            .expect("the fixture should be a valid authentication key");

        // Act
        let actual_key = parse_key(valid_key).expect("a valid path key should be accepted");

        // Assert
        assert_eq!(actual_key, expected_key);
    }

    #[tokio::test]
    async fn it_should_encode_an_invalid_key_format_failure_response_for_an_invalid_path_key() {
        // Arrange
        let router = Router::new().route("/{key}", get(protected_handler));
        let request = axum::http::Request::builder()
            .uri("/invalid_key")
            .body(Body::empty())
            .expect("the test request should be valid");

        // Act
        let response = router.oneshot(request).await.expect("the router should handle the request");
        let actual_error_response = decode_bencoded_failure_response(response).await;

        // Assert
        assert_failure_reason_contains(
            &actual_error_response,
            "Tracker authentication error: Invalid format for authentication key param",
        );
    }
}
