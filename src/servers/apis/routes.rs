//! API routes.
//!
//! It loads all the API routes for all API versions and adds the authentication
//! middleware to them.
//!
//! All the API routes have the `/api` prefix and the version number as the
//! first path segment. For example: `/api/v1/torrents`.
use std::sync::Arc;
use std::time::Duration;

use axum::http::{HeaderName, HeaderValue};
use axum::response::Response;
use axum::routing::get;
use axum::{middleware, Router};
use hyper::Request;
use torrust_tracker_configuration::AccessTokens;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestId, RequestId, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{Level, Span};
use uuid::Uuid;

use super::v1;
use super::v1::context::health_check::handlers::health_check_handler;
use super::v1::middlewares::auth::State;
use crate::core::Tracker;

/// Add all API routes to the router.
#[allow(clippy::needless_pass_by_value)]
pub fn router(tracker: Arc<Tracker>, access_tokens: Arc<AccessTokens>) -> Router {
    let router = Router::new();

    let api_url_prefix = "/api";

    let router = v1::routes::add(api_url_prefix, router, tracker.clone());

    let state = State { access_tokens };

    router
        .layer(middleware::from_fn_with_state(state, v1::middlewares::auth::auth))
        .route(&format!("{api_url_prefix}/health_check"), get(health_check_handler))
        .layer(CompressionLayer::new())
        .layer(SetRequestIdLayer::x_request_id(RequestIdGenerator))
        .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(|request: &Request<axum::body::Body>, _span: &Span| {
                    let method = request.method().to_string();
                    let uri = request.uri().to_string();
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .map(|v| v.to_str().unwrap_or_default())
                        .unwrap_or_default();

                    tracing::span!(
                        target: "API",
                        tracing::Level::INFO, "request", method = %method, uri = %uri, request_id = %request_id);
                })
                .on_response(|response: &Response, latency: Duration, _span: &Span| {
                    let status_code = response.status();
                    let request_id = response
                        .headers()
                        .get("x-request-id")
                        .map(|v| v.to_str().unwrap_or_default())
                        .unwrap_or_default();
                    let latency_ms = latency.as_millis();

                    tracing::span!(
                        target: "API",
                        tracing::Level::INFO, "response", latency = %latency_ms, status = %status_code, request_id = %request_id);
                }),
        )
        .layer(SetRequestIdLayer::x_request_id(RequestIdGenerator))
}

#[derive(Clone, Default)]
struct RequestIdGenerator;

impl MakeRequestId for RequestIdGenerator {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let id = HeaderValue::from_str(&Uuid::new_v4().to_string()).expect("UUID is a valid HTTP header value");
        Some(RequestId::new(id))
    }
}
