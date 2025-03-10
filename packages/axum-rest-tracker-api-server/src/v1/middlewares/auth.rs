//! Authentication middleware for the API.
//!
//! It uses a "token" to authenticate the user. The token must be one of the
//! `access_tokens` in the tracker [HTTP API configuration](torrust_tracker_configuration::HttpApi).
//!
//! There are two ways to provide the token:
//!
//! 1. As a `Bearer` token in the `Authorization` header.
//! 2. As a `token` GET param in the URL.
//!
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
//! [HTTP API configuration](torrust_tracker_configuration::HttpApi).
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
//! provided, the `Authorization` header will be used.
use std::sync::Arc;

use axum::extract::{self};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use torrust_tracker_configuration::AccessTokens;

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
/// The token must be one of the tokens in the tracker [HTTP API configuration](torrust_tracker_configuration::HttpApi).
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

    let header_value = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header_value| header_value.to_str().ok());

    match header_value {
        None => Ok(None),
        Some(header_value) => {
            if header_value == AUTH_BEARER_TOKEN_HEADER_PREFIX {
                // Empty token
                return Ok(Some(String::new()));
            }

            if !header_value.starts_with(&format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} ").to_string()) {
                // Invalid token type. Missing "Bearer" prefix.
                return Err(AuthError::UnknownTokenProvided);
            }

            Ok(header_value
                .strip_prefix(&format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} ").to_string())
                .map(std::string::ToString::to_string))
        }
    }
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
            AuthError::Unauthorized => unauthorized_response(),
            AuthError::TokenNotValid => token_not_valid_response(),
            AuthError::UnknownTokenProvided => unknown_auth_data_provided_response(),
        }
    }
}

fn authenticate(token: &str, tokens: &AccessTokens) -> bool {
    tokens.values().any(|t| t == token)
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
