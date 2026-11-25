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
//! Each sample is the raw bencoded body. The final `e` closes the bencoded
//! dictionary; it is not part of the `failure reason` text.
//!
//! When the key param is **missing**:
//!
//! ```text
//! d14:failure reason102:Tracker authentication error: Missing authentication key: this tracker is private, use /announce/<key>e
//! ```
//!
//! When the key param has the **wrong length**, the reply states the number of characters
//! received, never the key (the key is a secret):
//!
//! ```text
//! d14:failure reason88:Tracker authentication error: Invalid authentication key: expected 32 characters, got 14e
//! ```
//!
//! When the key param has a **character outside `0-9`, `a-z` and `A-Z`**:
//!
//! ```text
//! d14:failure reason106:Tracker authentication error: Invalid authentication key: only the characters 0-9, a-z and A-Z are allowede
//! ```
//!
//! When the key is **not registered**:
//!
//! ```text
//! d14:failure reason63:Tracker authentication error: Authentication key not registerede
//! ```
//!
//! When the key is registered but it's **expired**:
//!
//! ```text
//! d14:failure reason56:Tracker authentication error: Authentication key expirede
//! ```
//!
//! > **NOTICE**: the returned HTTP status code is always `200` for authentication errors.
//! > Neither [The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
//! > nor [The Private Torrents](https://www.bittorrent.org/beps/bep_0027.html)
//! > specifications specify any HTTP status code for authentication errors.
// cspell:ignore keye expirede registerede allowede
use std::future::Future;

use axum::extract::rejection::PathRejection;
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::Deserialize;
use torrust_tracker_core::authentication::Key;
use torrust_tracker_core::authentication::key::ParseKeyError;
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

    #[allow(
        clippy::manual_async_fn,
        reason = "Axum FromRequestParts requires an explicit Send-capable future return type"
    )]
    fn from_request_parts(parts: &mut Parts, state: &S) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // Extract `key` from URL path with Axum `Path` extractor
            let maybe_path_with_key = Path::<KeyParam>::from_request_parts(parts, state).await;

            match extract_key(maybe_path_with_key) {
                Ok(key) => Ok(Self(key)),
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
        Err(ParseKeyError::InvalidKeyLength { length }) => {
            Err(responses::error::Error::from(auth::Error::InvalidKeyLength { length }))
        }
        Err(_) => Err(responses::error::Error::from(auth::Error::InvalidKeyChars)),
    }
}

fn custom_error(rejection: &PathRejection) -> responses::error::Error {
    match rejection {
        axum::extract::rejection::PathRejection::FailedToDeserializePathParams(_) => {
            responses::error::Error::from(auth::Error::InvalidKeyChars)
        }
        _ => responses::error::Error::from(auth::Error::CannotExtractKeyParam),
    }
}

#[cfg(test)]
mod tests {

    use axum::Router;
    use axum::body::{Body, to_bytes};
    use axum::extract::FromRequestParts;
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use axum::routing::get;
    use torrust_tracker_http_protocol::v1::responses::error::Error;
    use torrust_tracker_test_helpers::assertions::assert_names_no_source_path;
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
    fn it_should_name_the_received_length_and_not_the_key_when_the_path_key_has_the_wrong_length() {
        // Arrange
        let fourteen_character_key = "ShortTestKey01";

        // Act
        let actual_error_response = parse_key(fourteen_character_key).unwrap_err();

        // Assert
        assert_names_no_source_path(&actual_error_response.failure_reason);
        assert_eq!(
            actual_error_response.failure_reason,
            "Tracker authentication error: Invalid authentication key: expected 32 characters, got 14"
        );
    }

    #[test]
    fn it_should_name_the_allowed_characters_and_not_the_key_when_the_path_key_has_another_character() {
        // Arrange
        let key_with_a_dash = "TestPeerKey-00000000000000000001";

        // Act
        let actual_error_response = parse_key(key_with_a_dash).unwrap_err();

        // Assert
        assert_eq!(
            actual_error_response.failure_reason,
            "Tracker authentication error: Invalid authentication key: only the characters 0-9, a-z and A-Z are allowed"
        );
    }

    #[tokio::test]
    async fn it_should_say_where_the_key_is_expected_and_name_no_source_path_when_the_request_has_no_path_params() {
        // Arrange
        let (mut parts, ()) = axum::http::Request::builder()
            .uri("/announce")
            .body(())
            .expect("the test request should be valid")
            .into_parts();

        // Act
        let Err(response) = Extract::from_request_parts(&mut parts, &()).await else {
            panic!("a request without path params should be rejected");
        };
        let actual_error_response = decode_bencoded_failure_response(response).await;

        // Assert
        assert_names_no_source_path(&actual_error_response.failure_reason);
        assert_eq!(
            actual_error_response.failure_reason,
            "Tracker authentication error: Cannot extract authentication key param from URL path: \
             expected /announce/<key> or /scrape/<key>"
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
            "Tracker authentication error: Invalid authentication key: expected 32 characters, got 11",
        );
    }
}
