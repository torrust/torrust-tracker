//! HTTP server routes for version `v1`.
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::http::HeaderName;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use axum_client_ip::SecureClientIpSource;
use hyper::Request;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{Level, Span};

use super::handlers::{announce, health_check, scrape};
use crate::core::Tracker;

/// It adds the routes to the router.
///
/// > **NOTICE**: it's added a layer to get the client IP from the connection
/// info. The tracker could use the connection info to get the client IP.
#[allow(clippy::needless_pass_by_value)]
pub fn router(tracker: Arc<Tracker>, &addr: &SocketAddr) -> Router {
    Router::new()
        // Health check
        .route("/health_check", get(health_check::handler))
        // Announce request
        .route("/announce", get(announce::handle_without_key).with_state(tracker.clone()))
        .route("/announce/:key", get(announce::handle_with_key).with_state(tracker.clone()))
        // Scrape request
        .route("/scrape", get(scrape::handle_without_key).with_state(tracker.clone()))
        .route("/scrape/:key", get(scrape::handle_with_key).with_state(tracker))
        // Add extension to get the client IP from the connection info
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .layer(CompressionLayer::new())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(move |request: &Request<axum::body::Body>, _span: &Span| {
                    let method = request.method().to_string();
                    let uri = request.uri().to_string();
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .map(|v| v.to_str().unwrap_or_default())
                        .unwrap_or_default();

                    tracing::span!(
                        target:"HTTP TRACKER",
                        tracing::Level::INFO, "request", server_socket_addr= %addr, method = %method, uri = %uri, request_id = %request_id);
                })
                .on_response(move |response: &Response, latency: Duration, _span: &Span| {
                    let status_code = response.status();
                    let request_id = response
                        .headers()
                        .get("x-request-id")
                        .map(|v| v.to_str().unwrap_or_default())
                        .unwrap_or_default();
                    let latency_ms = latency.as_millis();

                    tracing::span!(
                        target: "HTTP TRACKER",
                        tracing::Level::INFO, "response", server_socket_addr= %addr, latency = %latency_ms, status = %status_code, request_id = %request_id);
                }),
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
}
