//! HTTP server routes for version `v1`.
use std::sync::Arc;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::http::HeaderName;
use axum::response::Response;
use axum::routing::get;
use axum::{BoxError, Router};
use axum_client_ip::SecureClientIpSource;
use hyper::{Request, StatusCode};
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_server_lib::logging::Latency;
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use tower::ServiceBuilder;
use tower::timeout::TimeoutLayer;
use tower_http::LatencyUnit;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{Level, Span, instrument};

use super::handlers::{announce, health_check, scrape};
use crate::HTTP_TRACKER_LOG_TARGET;

const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

/// It adds the routes to the router.
///
/// > **NOTICE**: it's added a layer to get the client IP from the connection
/// > info. The tracker could use the connection info to get the client IP.
#[instrument(skip(http_tracker_container, server_service_binding))]
pub fn router(http_tracker_container: &Arc<HttpTrackerCoreContainer>, server_service_binding: &ServiceBinding) -> Router {
    let router = Router::new()
        // Health check
        .route("/health_check", get(health_check::handler))
        // Announce request
        .route(
            "/announce",
            get(announce::handle_without_key).with_state((
                http_tracker_container.announce_service.clone(),
                server_service_binding.clone(),
            )),
        )
        .route(
            "/announce/{key}",
            get(announce::handle_with_key).with_state((
                http_tracker_container.announce_service.clone(),
                server_service_binding.clone(),
            )),
        )
        // Scrape request
        .route(
            "/scrape",
            get(scrape::handle_without_key)
                .with_state((http_tracker_container.scrape_service.clone(), server_service_binding.clone())),
        )
        .route(
            "/scrape/{key}",
            get(scrape::handle_with_key)
                .with_state((http_tracker_container.scrape_service.clone(), server_service_binding.clone())),
        );

    with_request_layers(router, server_service_binding)
}

fn with_request_layers(router: Router, server_service_binding: &ServiceBinding) -> Router {
    let server_socket_addr = server_service_binding.bind_address();
    let request_service_binding = server_service_binding.clone();
    let response_service_binding = server_service_binding.clone();
    let failure_service_binding = server_service_binding.clone();

    router
        // Add extension to get the client IP from the connection info
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .layer(CompressionLayer::new())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(move |request: &Request<axum::body::Body>, span: &Span| {
                    let method = request.method().to_string();
                    let uri = request.uri().to_string();
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .map(|v| v.to_str().unwrap_or_default())
                        .unwrap_or_default();

                    span.record("request_id", request_id);

                    tracing::event!(
                        target: HTTP_TRACKER_LOG_TARGET,
                        tracing::Level::INFO,
                        %server_socket_addr,
                        service_binding = %request_service_binding,
                        %method,
                        %uri,
                        %request_id,
                        "request"
                    );
                })
                .on_response(move |response: &Response, latency: Duration, span: &Span| {
                    let latency_ms = latency.as_millis();
                    let status_code = response.status();
                    let request_id = response
                        .headers()
                        .get("x-request-id")
                        .map(|v| v.to_str().unwrap_or_default())
                        .unwrap_or_default();

                    span.record("request_id", request_id);

                    if status_code.is_server_error() {
                        tracing::event!(
                            target: HTTP_TRACKER_LOG_TARGET,
                            tracing::Level::ERROR,
                            %server_socket_addr,
                            service_binding = %response_service_binding,
                            %latency_ms,
                            %status_code,
                            %request_id,
                            "response"
                        );
                    } else {
                        tracing::event!(
                            target: HTTP_TRACKER_LOG_TARGET,
                            tracing::Level::INFO,
                            %server_socket_addr,
                            service_binding = %response_service_binding,
                            %latency_ms,
                            %status_code,
                            %request_id,
                            "response"
                        );
                    }
                })
                .on_failure(
                    move |failure_classification: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                        let latency = Latency::new(LatencyUnit::Millis, latency);

                        tracing::event!(
                            target: HTTP_TRACKER_LOG_TARGET, tracing::Level::ERROR,
                            %failure_classification,
                            %latency,
                            service_binding = %failure_service_binding,
                            "response failed"
                        );
                    },
                ),
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(
            ServiceBuilder::new()
                // this middleware goes above `TimeoutLayer` because it will receive
                // errors returned by `TimeoutLayer`
                .layer(HandleErrorLayer::new(|_: BoxError| async { StatusCode::REQUEST_TIMEOUT }))
                .layer(TimeoutLayer::new(DEFAULT_REQUEST_TIMEOUT)),
        )
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::header::HeaderName;
    use axum::routing::get;
    use axum::{Router, http};
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use tower::ServiceExt;
    use uuid::Uuid;

    use super::with_request_layers;

    fn service_binding() -> ServiceBinding {
        ServiceBinding::new(Protocol::HTTP, "127.0.0.1:7070".parse().expect("valid socket address"))
            .expect("valid HTTP service binding")
    }

    fn test_router() -> Router {
        with_request_layers(Router::new().route("/", get(|| async {})), &service_binding())
    }

    #[tokio::test]
    async fn it_should_propagate_a_client_supplied_request_id() {
        // Arrange
        let client_request_id = "test-request-id";
        let request = http::Request::builder()
            .uri("/")
            .header("x-request-id", client_request_id)
            .body(Body::empty())
            .expect("valid request");

        // Act
        let response = test_router().oneshot(request).await.expect("router should handle request");

        // Assert
        assert_eq!(response.status(), http::StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get("x-request-id")
                .expect("request ID header")
                .to_str()
                .expect("request ID header should be valid text"),
            client_request_id
        );
    }

    #[tokio::test]
    async fn it_should_add_a_uuid_request_id_when_the_client_does_not_supply_one() {
        // Arrange
        let request = http::Request::builder().uri("/").body(Body::empty()).expect("valid request");

        // Act
        let response = test_router().oneshot(request).await.expect("router should handle request");

        // Assert
        assert_eq!(response.status(), http::StatusCode::OK);
        let actual_request_id = response
            .headers()
            .get(HeaderName::from_static("x-request-id"))
            .expect("request ID header")
            .to_str()
            .expect("request ID header should be valid text");
        assert!(
            Uuid::parse_str(actual_request_id).is_ok(),
            "request ID should be a UUID: {actual_request_id}"
        );
    }
}
