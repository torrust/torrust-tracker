use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::http::HeaderName;
use axum::Json;
use hyper::Request;
use serde_json::json;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{instrument, Level, Span};

use super::handlers::handle_health_check;
use crate::servers::registar::Registry;

#[instrument(fields(registry = %registry))]
pub fn router(registry: Arc<Registry>, addr: SocketAddr) -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(|| async { Json(json!({})) }))
        .route("/health_check", axum::routing::get(handle_health_check))
        .with_state(registry)
        .layer(CompressionLayer::new())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request( move|request: &Request<axum::body::Body>, _span: &Span| {
                    let method = request.method().to_string();
                    let uri = request.uri().to_string();
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .map(|v| v.to_str().unwrap_or_default())
                        .unwrap_or_default();

                    tracing::span!(
                        target: "HEALTH CHECK API",
                        tracing::Level::INFO, "request", socket_addr= %addr, method = %method, uri = %uri, request_id = %request_id);
                })
                .on_response(move|response: &axum::response::Response, latency: Duration, _span: &Span| {
                    let status_code = response.status();
                    let request_id = response
                        .headers()
                        .get("x-request-id")
                        .map(|v| v.to_str().unwrap_or_default())
                        .unwrap_or_default();
                    let latency_ms = latency.as_millis();

                    tracing::span!(
                        target: "HEALTH CHECK API",
                        tracing::Level::INFO, "response", socket_addr= %addr, latency = %latency_ms, status = %status_code, request_id = %request_id);
                }),
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
}
