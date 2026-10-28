//! Authentication middleware for the API.
//!
//! It uses a "token" to authenticate the user. The token must be one of the
//! `access_tokens` in the tracker [HTTP API configuration](torrust_tracker_configuration::v3_0_0::tracker_api::HttpApi).
//!
//! There are two ways to provide the token:
//!
//! 1. As a `Bearer` token in the `Authorization` header.
//! 2. As a `token` GET param in the URL.
//!
//! skill-link: use-rest-api
//! Using the `Authorization` header:
//!
//! ```console
//! curl -H "Authorization: Bearer MyAccessToken" http://<host>:<port>/api/v1/<context>
//! ```
//!
//! Using the `token` GET param:
//!
//! `http://<host>:<port>/api/v1/<context>?token=<token>`.
//!
//! > **NOTICE**: the token can be at any position in the URL, not just at the
//! > beginning or at the end.
//!
//! The token must be one of the `access_tokens` in the tracker
//! [HTTP API configuration](torrust_tracker_configuration::v3_0_0::tracker_api::HttpApi).
//!
//! The configuration file `tracker.toml` contains a list of tokens:
//!
//! ```toml
//! [http_api.access_tokens]
//! admin = "MyAccessToken"
//! ```
//!
//! All the tokes have the same permissions, so it is not possible to have
//! different permissions for different tokens. The label is only used to
//! identify the token.
//!
//! NOTICE: The token is not encrypted, so it is recommended to use HTTPS to
//! protect the token from being intercepted.
//!
//! NOTICE: If both the `Authorization` header and the `token` GET param are
//! provided, the `Authorization` header will be used. An `Authorization` header
//! that is not readable text (not visible ASCII) is rejected like any other
//! invalid header; it never falls back to the `token` GET param.
use std::sync::Arc;

use axum::extract::{self};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use secrecy::ExposeSecret;
use serde::Deserialize;
use subtle::{Choice, ConstantTimeEq};
use torrust_tracker_configuration::v3_0_0::tracker_api::AccessTokens;

use crate::v1::responses::unhandled_rejection_response;

pub const AUTH_BEARER_TOKEN_HEADER_PREFIX: &str = "Bearer";

/// Container for the `token` extracted from the query params.
#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub token: Option<String>,
}

#[derive(Clone, Debug)]
pub struct State {
    pub access_tokens: Arc<AccessTokens>,
}

/// Middleware for authentication.
///
/// The token must be one of the tokens in the tracker [HTTP API configuration](torrust_tracker_configuration::v3_0_0::tracker_api::HttpApi).
pub async fn auth(
    extract::State(state): extract::State<State>,
    extract::Query(params): extract::Query<QueryParams>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let token_from_header = match extract_bearer_token_from_header(&request) {
        Ok(token) => token,
        Err(err) => return err.into_response(),
    };

    let token_from_get_param = params.token.clone();

    let provided_tokens = (token_from_header, token_from_get_param);

    let token = match provided_tokens {
        (Some(token_from_header), Some(_token_from_get_param)) => token_from_header,
        (Some(token_from_header), None) => token_from_header,
        (None, Some(token_from_get_param)) => token_from_get_param,
        (None, None) => return AuthError::Unauthorized.into_response(),
    };

    if !authenticate(&token, &state.access_tokens) {
        return AuthError::TokenNotValid.into_response();
    }

    next.run(request).await
}

fn extract_bearer_token_from_header(request: &Request<axum::body::Body>) -> Result<Option<String>, AuthError> {
    let headers = request.headers();

    let Some(header_value) = headers.get(axum::http::header::AUTHORIZATION) else {
        return Ok(None);
    };

    // A value that is not visible ASCII is a malformed header, not an absent one: treating it as absent would
    // fall back to the `token` query parameter.
    let header_value = header_value.to_str().map_err(|_| AuthError::UnknownTokenProvided)?;

    if header_value == AUTH_BEARER_TOKEN_HEADER_PREFIX {
        // Empty token
        return Ok(Some(String::new()));
    }

    if !header_value.starts_with(&format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} ")) {
        // Invalid token type. Missing "Bearer" prefix.
        return Err(AuthError::UnknownTokenProvided);
    }

    Ok(header_value
        .strip_prefix(&format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} "))
        .map(std::string::ToString::to_string))
}

enum AuthError {
    /// Missing token for authentication.
    Unauthorized,

    /// Token was provided but it is not valid.
    TokenNotValid,

    /// Token was provided but it is not in a format that the server can't understands.
    UnknownTokenProvided,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthorized => unauthorized_response(),
            Self::TokenNotValid => token_not_valid_response(),
            Self::UnknownTokenProvided => unknown_auth_data_provided_response(),
        }
    }
}

/// Checks a supplied token against every configured token without content-dependent early exits.
///
/// Do not simplify this to `==` or `Iterator::any`: both can stop early and make comparison
/// work depend on matching token content. `Choice` keeps all comparison results until every
/// configured token has been checked.
///
/// Security record: `docs/security/analysis/reports/2026-09-04_rest-api-token-timing.md`.
fn authenticate(token: &str, tokens: &AccessTokens) -> bool {
    let token = token.as_bytes();

    let authentication_result = tokens.values().fold(Choice::from(0), |result, configured_token| {
        result | configured_token.expose_secret().as_bytes().ct_eq(token)
    });

    bool::from(authentication_result)
}

/// `500` error response returned when the token is missing.
#[must_use]
pub fn unauthorized_response() -> Response {
    unhandled_rejection_response("unauthorized".to_string())
}

/// `500` error response when the provided token is not valid.
#[must_use]
pub fn token_not_valid_response() -> Response {
    unhandled_rejection_response("token not valid".to_string())
}

/// `500` error response when the provided token type is not valid.
///
/// The client has provided authentication information that the server does not
/// understand.
#[must_use]
pub fn unknown_auth_data_provided_response() -> Response {
    unhandled_rejection_response("unknown token provided".to_string())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    use axum::body::{Body, to_bytes};
    use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
    use axum::http::{HeaderValue, Request, StatusCode};
    use axum::routing::get;
    use axum::{Router, middleware};
    use rstest::rstest;
    use secrecy::SecretString;
    use torrust_tracker_configuration::v3_0_0::tracker_api::AccessTokens;
    use tower::ServiceExt;

    use super::{State, auth, authenticate};
    use crate::test_support::MAX_BODY_BYTES;

    const FIRST_TOKEN: &str = "api-token-12345";
    const SECOND_TOKEN: &str = "other-token-678";

    /// What a client sees after the request crossed the authentication middleware, and whether the
    /// handler behind it ran.
    #[derive(Debug, PartialEq, Eq)]
    struct Sent {
        status: StatusCode,
        content_type: Option<String>,
        body: String,
        handler_ran: bool,
    }

    fn access_tokens() -> AccessTokens {
        AccessTokens::from([
            ("first".to_string(), SecretString::from(FIRST_TOKEN)),
            ("second".to_string(), SecretString::from(SECOND_TOKEN)),
        ])
    }

    fn request(authorization: Option<&str>, uri: &str) -> Request<Body> {
        let authorization = authorization.map(|value| HeaderValue::from_str(value).expect("the test header value is valid"));

        request_with_header(authorization, uri)
    }

    fn request_with_header(authorization: Option<HeaderValue>, uri: &str) -> Request<Body> {
        let mut request = Request::builder().uri(uri);
        if let Some(authorization) = authorization {
            request = request.header(AUTHORIZATION, authorization);
        }
        request.body(Body::empty()).expect("the request is well formed")
    }

    async fn send(request: Request<Body>) -> Sent {
        let handler_ran = Arc::new(AtomicBool::new(false));
        let handler_ran_by_handler = handler_ran.clone();
        let state = State {
            access_tokens: Arc::new(access_tokens()),
        };
        let app = Router::new()
            .route(
                "/",
                get(move || async move {
                    handler_ran_by_handler.store(true, Ordering::SeqCst);
                    "handler ran"
                }),
            )
            .layer(middleware::from_fn_with_state(state, auth));

        let response = app.oneshot(request).await.expect("the router never fails");

        let status = response.status();
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .map(|value| value.to_str().expect("the content type is text").to_string());
        let body = to_bytes(response.into_body(), MAX_BODY_BYTES)
            .await
            .expect("the body is readable and within the limit");

        Sent {
            status,
            content_type,
            body: String::from_utf8(body.to_vec()).expect("the body is text"),
            handler_ran: handler_ran.load(Ordering::SeqCst),
        }
    }

    #[rstest]
    #[case::the_token_in_the_header(Some("Bearer api-token-12345"), "/")]
    #[case::the_other_configured_token_in_the_header(Some("Bearer other-token-678"), "/")]
    #[case::the_token_in_the_query(None, "/?token=api-token-12345")]
    #[case::the_token_in_the_query_at_any_position(None, "/?format=json&token=api-token-12345&limit=1")]
    #[case::a_valid_header_and_an_invalid_query_token(Some("Bearer api-token-12345"), "/?token=wrong-token")]
    #[tokio::test]
    async fn it_should_let_the_request_reach_the_handler_when_a_configured_token_is_provided(
        #[case] authorization: Option<&str>,
        #[case] uri: &str,
    ) {
        let request = request(authorization, uri);

        let sent = send(request).await;

        assert_eq!(
            sent,
            Sent {
                status: StatusCode::OK,
                content_type: Some("text/plain; charset=utf-8".to_string()),
                body: "handler ran".to_string(),
                handler_ran: true,
            }
        );
    }

    #[rstest]
    #[case::an_invalid_token_in_the_header(Some("Bearer wrong-token"), "/")]
    #[case::an_invalid_token_in_the_query(None, "/?token=wrong-token")]
    #[case::an_empty_token_in_the_query(None, "/?token=")]
    #[case::an_invalid_header_even_with_a_valid_query_token(Some("Bearer wrong-token"), "/?token=api-token-12345")]
    #[case::the_bearer_scheme_without_a_token(Some("Bearer"), "/")]
    #[case::the_bearer_scheme_followed_by_a_space_and_no_token(Some("Bearer "), "/")]
    #[tokio::test]
    async fn it_should_reject_the_request_as_token_not_valid_when_the_provided_token_is_not_a_configured_one(
        #[case] authorization: Option<&str>,
        #[case] uri: &str,
    ) {
        let request = request(authorization, uri);

        let sent = send(request).await;

        assert_eq!(
            sent,
            Sent {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                content_type: Some("text/plain; charset=utf-8".to_string()),
                body: r#"Unhandled rejection: Err { reason: "token not valid" }"#.to_string(),
                handler_ran: false,
            }
        );
    }

    #[rstest]
    #[case::another_scheme(Some("Basic api-token-12345"), "/")]
    #[case::a_lower_case_bearer_scheme(Some("bearer api-token-12345"), "/")]
    #[case::the_scheme_glued_to_the_token(Some("Bearer-api-token-12345"), "/")]
    #[case::an_empty_header(Some(""), "/")]
    #[case::another_scheme_even_with_a_valid_query_token(Some("Basic api-token-12345"), "/?token=api-token-12345")]
    #[tokio::test]
    async fn it_should_reject_the_request_as_unknown_token_provided_when_the_header_is_not_a_bearer_credential(
        #[case] authorization: Option<&str>,
        #[case] uri: &str,
    ) {
        let request = request(authorization, uri);

        let sent = send(request).await;

        assert_eq!(
            sent,
            Sent {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                content_type: Some("text/plain; charset=utf-8".to_string()),
                body: r#"Unhandled rejection: Err { reason: "unknown token provided" }"#.to_string(),
                handler_ran: false,
            }
        );
    }

    #[tokio::test]
    async fn it_should_reject_an_unreadable_authorization_header_even_with_a_valid_token_query() {
        let unreadable_header = HeaderValue::from_bytes(b"Bearer \xff").expect("the bytes are a valid header value");
        let request = request_with_header(Some(unreadable_header), "/?token=api-token-12345");

        let sent = send(request).await;

        assert_eq!(
            sent,
            Sent {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                content_type: Some("text/plain; charset=utf-8".to_string()),
                body: r#"Unhandled rejection: Err { reason: "unknown token provided" }"#.to_string(),
                handler_ran: false,
            }
        );
    }

    #[rstest]
    #[case::no_query("/")]
    #[case::a_query_without_a_token("/?format=json")]
    #[tokio::test]
    async fn it_should_reject_the_request_as_unauthorized_when_no_token_is_provided(#[case] uri: &str) {
        let request = request(None, uri);

        let sent = send(request).await;

        assert_eq!(
            sent,
            Sent {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                content_type: Some("text/plain; charset=utf-8".to_string()),
                body: r#"Unhandled rejection: Err { reason: "unauthorized" }"#.to_string(),
                handler_ran: false,
            }
        );
    }

    #[tokio::test]
    async fn it_should_reject_a_duplicated_token_query_parameter_before_authenticating() {
        let request = request(None, "/?token=api-token-12345&token=api-token-12345");

        let sent = send(request).await;

        // The text of a query rejection belongs to Axum; the package owns that the handler does not run.
        assert_eq!(sent.status, StatusCode::BAD_REQUEST);
        assert!(!sent.handler_ran);
    }

    #[rstest]
    #[case::the_first_configured_token(FIRST_TOKEN)]
    #[case::the_second_configured_token(SECOND_TOKEN)]
    fn it_should_authenticate_a_configured_token(#[case] token: &str) {
        let access_tokens = access_tokens();

        assert!(authenticate(token, &access_tokens));
    }

    #[rstest]
    #[case::a_different_first_character("xpi-token-12345")]
    #[case::a_different_last_character("api-token-1234x")]
    #[case::a_shorter_token("api-token-1234")]
    #[case::a_longer_token("api-token-123456")]
    #[case::an_empty_token("")]
    fn it_should_not_authenticate_a_token_that_is_not_a_configured_one(#[case] token: &str) {
        let access_tokens = access_tokens();

        assert!(!authenticate(token, &access_tokens));
    }
}
