//! API routes.
//!
//! It loads all the API routes for all API versions and adds the authentication
//! middleware to them.
//!
//! All the API routes have the `/api` prefix and the version number as the
//! first path segment. For example: `/api/v1/torrents`.
use std::sync::Arc;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::http::HeaderName;
use axum::response::Response;
use axum::routing::get;
use axum::{BoxError, Router, middleware};
use hyper::{Request, StatusCode};
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_server_lib::logging::Latency;
use torrust_tracker_configuration::v3_0_0::tracker_api::AccessTokens;
use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;
use tower::ServiceBuilder;
use tower::timeout::TimeoutLayer;
use tower_http::LatencyUnit;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{Level, Span, instrument};

const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

const API_URL_PREFIX: &str = "/api";

use super::v1;
use super::v1::context::health_check::handlers::health_check_handler;
use super::v1::middlewares::auth::State;
use crate::API_LOG_TARGET;

/// Add all API routes to the router.
#[instrument(skip(http_api_container, access_tokens))]
pub fn router(
    http_api_container: &Arc<TrackerHttpApiCoreContainer>,
    access_tokens: Arc<AccessTokens>,
    server_service_binding: &ServiceBinding,
) -> Router {
    let routes = v1::routes::add(API_URL_PREFIX, Router::new(), http_api_container);

    with_layers(routes, access_tokens, server_service_binding, DEFAULT_REQUEST_TIMEOUT)
}

/// Adds the authentication, the public health check and the transport layers to the API routes.
///
/// The authentication layer is added before the health check route, so it protects the API routes and not the health
/// check. The timeout is a parameter so that a test can use a short one.
fn with_layers(
    routes: Router,
    access_tokens: Arc<AccessTokens>,
    server_service_binding: &ServiceBinding,
    timeout: Duration,
) -> Router {
    let server_socket_addr = server_service_binding.bind_address();
    let request_service_binding = server_service_binding.clone();
    let response_service_binding = server_service_binding.clone();
    let failure_service_binding = server_service_binding.clone();

    let state = State { access_tokens };

    routes
        .layer(middleware::from_fn_with_state(state, v1::middlewares::auth::auth))
        .route(&format!("{API_URL_PREFIX}/health_check"), get(health_check_handler))
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
                        target: API_LOG_TARGET,
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
                            target: API_LOG_TARGET,
                            tracing::Level::ERROR,
                            %latency_ms,
                            %status_code,
                            %server_socket_addr,
                            service_binding = %response_service_binding,
                            %request_id,
                            "response"
                        );
                    } else {
                        tracing::event!(
                            target: API_LOG_TARGET,
                            tracing::Level::INFO,
                            %latency_ms,
                            %status_code,
                            %server_socket_addr,
                            service_binding = %response_service_binding,
                            %request_id,
                            "response"
                        );
                    }
                })
                .on_failure(
                    move |failure_classification: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                        let latency = Latency::new(LatencyUnit::Millis, latency);

                        tracing::event!(
                            target: API_LOG_TARGET,
                            tracing::Level::ERROR,
                            %failure_classification,
                            %latency,
                            %server_socket_addr,
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
                .layer(TimeoutLayer::new(timeout)),
        )
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use axum::Router;
    use axum::body::{Body, to_bytes};
    use axum::http::header::{ACCEPT_ENCODING, CONTENT_ENCODING};
    use axum::http::{HeaderMap, Request, StatusCode};
    use axum::routing::get;
    use rstest::rstest;
    use secrecy::SecretString;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_configuration::v3_0_0::tracker_api::AccessTokens;
    use torrust_tracker_test_helpers::configuration::ephemeral;
    use tower::ServiceExt;
    use uuid::Uuid;

    use super::{router, with_layers};
    use crate::test_support::{MAX_BODY_BYTES, api_container};

    const TOKEN: &str = "test-token";
    const DEADLINE: Duration = Duration::from_secs(5);

    /// What a client receives: the status, the headers and the body.
    struct Received {
        status: StatusCode,
        headers: HeaderMap,
        body: String,
    }

    fn access_tokens() -> Arc<AccessTokens> {
        Arc::new(AccessTokens::from([("test".to_string(), SecretString::from(TOKEN))]))
    }

    fn service_binding() -> ServiceBinding {
        ServiceBinding::new(Protocol::HTTP, "127.0.0.1:1212".parse().expect("the address is valid"))
            .expect("the service binding is valid")
    }

    fn get_request(uri: &str) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .body(Body::empty())
            .expect("the request is well formed")
    }

    async fn into_received(response: axum::response::Response) -> Received {
        let status = response.status();
        let headers = response.headers().clone();
        let body = to_bytes(response.into_body(), MAX_BODY_BYTES)
            .await
            .expect("the body is readable and within the limit");

        Received {
            status,
            headers,
            body: String::from_utf8_lossy(&body).into_owned(),
        }
    }

    /// Sends the request through the API router built for a default tracker.
    async fn through_the_router(request: Request<Body>) -> Received {
        let container = api_container(&ephemeral()).await;
        let router = router(&container, access_tokens(), &service_binding());

        into_received(router.oneshot(request).await.expect("the router never fails")).await
    }

    #[tokio::test]
    async fn it_should_serve_the_health_check_without_a_token() {
        let request = get_request("/api/health_check");

        let received = through_the_router(request).await;

        assert_eq!(received.status, StatusCode::OK);
    }

    #[rstest]
    #[case::no_token("/api/v1/stats", r#"Unhandled rejection: Err { reason: "unauthorized" }"#)]
    #[case::a_wrong_token(
        "/api/v1/stats?token=wrong-token",
        r#"Unhandled rejection: Err { reason: "token not valid" }"#
    )]
    #[tokio::test]
    async fn it_should_reject_a_request_to_an_api_route_without_a_valid_token(#[case] uri: &str, #[case] body: &str) {
        let request = get_request(uri);

        let received = through_the_router(request).await;

        assert_eq!(
            (received.status, received.body),
            (StatusCode::INTERNAL_SERVER_ERROR, body.to_string())
        );
    }

    #[tokio::test]
    async fn it_should_serve_an_api_route_when_the_request_carries_a_token_given_to_the_router() {
        let request = get_request("/api/v1/stats?token=test-token");

        let received = through_the_router(request).await;

        assert_eq!(received.status, StatusCode::OK);
    }

    #[tokio::test]
    async fn it_should_generate_a_request_id_when_the_request_has_none() {
        let request = get_request("/api/health_check");

        let received = through_the_router(request).await;

        let request_id = received
            .headers
            .get("x-request-id")
            .expect("the response has a request id")
            .to_str()
            .expect("the request id is text");
        assert!(
            Uuid::parse_str(request_id).is_ok(),
            "the request id is not a UUID: {request_id}"
        );
    }

    #[tokio::test]
    async fn it_should_return_the_request_id_provided_by_the_client() {
        let request = Request::builder()
            .uri("/api/health_check")
            .header("x-request-id", "my-request-1")
            .body(Body::empty())
            .expect("the request is well formed");

        let received = through_the_router(request).await;

        assert_eq!(
            received.headers.get("x-request-id").map(axum::http::HeaderValue::as_bytes),
            Some(&b"my-request-1"[..])
        );
    }

    #[rstest]
    #[case::when_the_client_accepts_gzip(Some("gzip"), Some("gzip"))]
    #[case::when_the_client_does_not_ask_for_compression(None, None)]
    #[tokio::test]
    async fn it_should_compress_the_response_only_as_the_client_asks(
        #[case] accept_encoding: Option<&str>,
        #[case] content_encoding: Option<&str>,
    ) {
        let mut request = get_request("/api/v1/stats?token=test-token");
        if let Some(accept_encoding) = accept_encoding {
            request
                .headers_mut()
                .insert(ACCEPT_ENCODING, accept_encoding.parse().expect("the header value is valid"));
        }

        let received = through_the_router(request).await;

        assert_eq!(
            received.headers.get(CONTENT_ENCODING).map(axum::http::HeaderValue::as_bytes),
            content_encoding.map(str::as_bytes)
        );
    }

    #[tokio::test]
    async fn it_should_answer_request_timeout_when_an_api_route_does_not_answer_within_the_timeout() {
        let never_answering = Router::new().route("/api/v1/slow", get(std::future::pending::<&'static str>));
        let router = with_layers(
            never_answering,
            access_tokens(),
            &service_binding(),
            Duration::from_millis(50),
        );
        let request = get_request("/api/v1/slow?token=test-token");

        let response = tokio::time::timeout(DEADLINE, router.oneshot(request))
            .await
            .expect("the router answers within the deadline")
            .expect("the router never fails");

        let received = into_received(response).await;
        assert_eq!((received.status, received.body), (StatusCode::REQUEST_TIMEOUT, String::new()));
    }
}
