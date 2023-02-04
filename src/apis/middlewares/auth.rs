use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use torrust_tracker_configuration::{Configuration, HttpApi};

use crate::apis::responses::unhandled_rejection_response;

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub token: Option<String>,
}

/// Middleware for authentication using a "token" GET param.
/// The token must be one of the tokens in the tracker HTTP API configuration.
pub async fn auth<B>(
    State(config): State<Arc<Configuration>>,
    Query(params): Query<QueryParams>,
    request: Request<B>,
    next: Next<B>,
) -> Response
where
    B: Send,
{
    let token = match params.token {
        None => return AuthError::Unauthorized.into_response(),
        Some(token) => token,
    };

    if !authenticate(&token, &config.http_api) {
        return AuthError::TokenNotValid.into_response();
    }

    next.run(request).await
}

enum AuthError {
    Unauthorized,
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

#[must_use]
pub fn unauthorized_response() -> Response {
    unhandled_rejection_response("unauthorized".to_string())
}

#[must_use]
pub fn token_not_valid_response() -> Response {
    unhandled_rejection_response("token not valid".to_string())
}
