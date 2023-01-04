use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::{header, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::config::{Configuration, HttpApi};

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
        let body = match self {
            AuthError::Unauthorized => "Unhandled rejection: Err { reason: \"unauthorized\" }",
            AuthError::TokenNotValid => "Unhandled rejection: Err { reason: \"token not valid\" }",
        };

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "text/plain")],
            body,
        )
            .into_response()
    }
}

fn authenticate(token: &str, http_api_config: &HttpApi) -> bool {
    http_api_config.contains_token(token)
}
