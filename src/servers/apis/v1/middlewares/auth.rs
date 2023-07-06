//! Authentication middleware for the API.
//!
//! It uses a "token" GET param to authenticate the user. URLs must be of the
//! form:
//!
//! `http://<host>:<port>/api/v1/<context>?token=<token>`.
//!
//! > **NOTICE**: the token can be at any position in the URL, not just at the
//! > beginning or at the end.
//!
//! The token must be one of the `access_tokens` in the tracker
//! [HTTP API configuration](torrust_tracker_configuration::HttpApi).
//!
//! The configuration file `config.toml` contains a list of tokens:
//!
//! ```toml
//! [http_api.access_tokens]
//! admin = "MyAccessToken"
//! ```
//!
//! All the tokes have the same permissions, so it is not possible to have
//! different permissions for different tokens. The label is only used to
//! identify the token.
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use torrust_tracker_configuration::{Configuration, HttpApi};

use crate::servers::apis::v1::responses::unhandled_rejection_response;

/// Container for the `token` extracted from the query params.
#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub token: Option<String>,
}

/// Middleware for authentication using a "token" GET param.
/// The token must be one of the tokens in the tracker [HTTP API configuration](torrust_tracker_configuration::HttpApi).
pub async fn auth<B>(
    State(config): State<Arc<Configuration>>,
    Query(params): Query<QueryParams>,
    request: Request<B>,
    next: Next<B>,
) -> Response
where
    B: Send,
{
    let Some(token) = params.token else {
        return AuthError::Unauthorized.into_response();
    };

    if !authenticate(&token, &config.http_api) {
        return AuthError::TokenNotValid.into_response();
    }

    next.run(request).await
}

enum AuthError {
    /// Missing token for authentication.
    Unauthorized,
    /// Token was provided but it is not valid.
    TokenNotValid,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::Unauthorized => unauthorized_response(),
            AuthError::TokenNotValid => token_not_valid_response(),
        }
    }
}

fn authenticate(token: &str, http_api_config: &HttpApi) -> bool {
    http_api_config.contains_token(token)
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
