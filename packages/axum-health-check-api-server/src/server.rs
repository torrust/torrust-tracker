//! Logic to run the Health Check HTTP API server.
//!
//! This API is intended to be used by the container infrastructure to check if
//! the whole application is healthy.
use std::net::SocketAddr;
use std::time::Duration;

use axum::http::HeaderName;
use axum::response::Response;
use axum::routing::get;
use axum::{Json, Router};
use axum_server::Handle;
use futures::Future;
use hyper::Request;
use serde_json::json;
use tokio::sync::oneshot::{Receiver, Sender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_server_lib::logging::{Latency, STARTED_ON};
use torrust_server_lib::registar::{Registar, RegistrationError, ServiceRegistration};
use torrust_server_lib::signals::{Halted, Started};
use torrust_tracker_axum_server::signals::{GracefulShutdownOutcome, graceful_shutdown, graceful_shutdown_on_cancellation};
use torrust_tracker_primitives::RuntimeServiceMetadata;
use tower_http::LatencyUnit;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{Level, Span, instrument};

use crate::HEALTH_CHECK_API_LOG_TARGET;
use crate::handlers::health_check_handler;

/// Errors returned by [`start_with_cancellation`].
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Could not bind the health check API listener. Check that its bind address is available: {source}")]
    Bind { source: std::io::Error },

    #[error("Could not configure the health check API listener: {source}")]
    Listener { source: std::io::Error },

    #[error("Could not register the health check API service: {source}")]
    Registration { source: RegistrationError },
}

/// A token-aware health-check runtime and its owned drain controller.
///
/// The caller must retain and join both handles. [`RunningServer`] remains the
/// compatibility path for consumers using [`Halted`].
pub struct CancellationRunning {
    pub local_addr: SocketAddr,
    /// The binding this server registered in the `registar`. A caller that
    /// stops the server and wants the registry to forget it passes this
    /// binding to `Registar::deregister`.
    pub service_binding: ServiceBinding,
    pub task: JoinHandle<Result<(), std::io::Error>>,
    pub shutdown_controller: JoinHandle<GracefulShutdownOutcome>,
}

/// A health-check server's runtime future and its graceful-shutdown controller.
///
/// The caller owns both tasks. It must join them during normal shutdown or
/// abort them when its enclosing component is aborted.
pub struct RunningServer<F> {
    pub running: F,
    pub shutdown_controller: JoinHandle<()>,
}

/// Starts Health Check API server.
///
/// # Errors
///
/// Returns an error if the listener cannot bind or be configured, or if the
/// startup receiver is dropped before the listener is reported.
#[instrument(skip(bind_to, tx, rx_halt, registar))]
pub fn start(
    bind_to: SocketAddr,
    tx: Sender<Started>,
    rx_halt: Receiver<Halted>,
    registar: Registar<RuntimeServiceMetadata>,
) -> Result<RunningServer<impl Future<Output = Result<(), std::io::Error>> + Send>, std::io::Error> {
    let router = router(registar);

    let socket = std::net::TcpListener::bind(bind_to)?;
    socket.set_nonblocking(true)?;
    let address = socket.local_addr()?;
    let protocol = Protocol::HTTP; // The health check API only supports HTTP directly now. Use a reverse proxy for HTTPS.
    let service_binding = ServiceBinding::new(protocol, address).map_err(std::io::Error::other)?;

    let handle = Handle::new();

    tracing::debug!(target: HEALTH_CHECK_API_LOG_TARGET, "Starting service with graceful shutdown in a spawned task ...");

    let shutdown_controller = tokio::task::spawn(graceful_shutdown(
        handle.clone(),
        rx_halt,
        format!("Shutting down http server on socket address: {address}"),
        address,
    ));

    let running = axum_server::from_tcp(socket)?
        .handle(handle)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>());

    if tx
        .send(Started {
            service_binding,
            address,
        })
        .is_err()
    {
        shutdown_controller.abort();
        return Err(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "health check startup receiver was dropped",
        ));
    }

    Ok(RunningServer {
        running,
        shutdown_controller,
    })
}

/// Starts the Health Check API using an injected cancellation token.
///
/// This additive path does not subscribe to operating-system signals. It
/// registers the service and, if registration fails, cancels
/// `cancellation_token`, stops both children, and releases the listener before
/// returning. Otherwise the caller owns the returned runtime task and drain
/// controller. `drain_timeout` is how long the server waits for open connections to finish after
/// `cancellation_token` is cancelled, before it closes them.
///
/// # Errors
///
/// Returns listener or service-registration errors.
pub async fn start_with_cancellation(
    bind_to: SocketAddr,
    registar: Registar<RuntimeServiceMetadata>,
    metadata: RuntimeServiceMetadata,
    cancellation_token: CancellationToken,
    drain_timeout: Duration,
) -> Result<CancellationRunning, Error> {
    let protocol = Protocol::HTTP; // The health check API only supports HTTP directly now. Use a reverse proxy for HTTPS.

    tracing::info!(target: HEALTH_CHECK_API_LOG_TARGET, "Starting on: {protocol}://{bind_to}");

    let socket = std::net::TcpListener::bind(bind_to).map_err(|source| Error::Bind { source })?;
    socket.set_nonblocking(true).map_err(|source| Error::Listener { source })?;
    let local_addr = socket.local_addr().map_err(|source| Error::Listener { source })?;
    let service_binding = ServiceBinding::new(protocol.clone(), local_addr).map_err(|error| Error::Listener {
        source: std::io::Error::other(error),
    })?;
    let server = axum_server::from_tcp(socket).map_err(|source| Error::Listener { source })?;

    let handle = Handle::new();
    let serving = server
        .handle(handle.clone())
        .serve(router(registar.clone()).into_make_service_with_connect_info::<SocketAddr>());
    let task = tokio::spawn(serving);
    let shutdown_controller = tokio::spawn(graceful_shutdown_on_cancellation(
        handle,
        cancellation_token.clone(),
        format!("Shutting down health check API server on socket address: {local_addr}"),
        local_addr,
        drain_timeout,
    ));

    tracing::info!(
        target: HEALTH_CHECK_API_LOG_TARGET,
        service_role = metadata.service_role().as_str(),
        instance_index = metadata.configuration_instance_id().instance_index(),
        service_binding = %service_binding,
        "Started health check API"
    );

    if let Err(source) = registar
        .give_form()
        .register(ServiceRegistration::new(service_binding.clone(), metadata, None))
        .await
    {
        cancellation_token.cancel();
        task.abort();
        shutdown_controller.abort();
        drop(task.await);
        drop(shutdown_controller.await);
        return Err(Error::Registration { source });
    }

    tracing::info!(target: HEALTH_CHECK_API_LOG_TARGET, "{STARTED_ON}: {protocol}://{local_addr}");

    Ok(CancellationRunning {
        local_addr,
        service_binding,
        task,
        shutdown_controller,
    })
}

fn router(registar: Registar<RuntimeServiceMetadata>) -> Router {
    Router::new()
        .route("/", get(|| async { Json(json!({})) }))
        .route("/health_check", get(health_check_handler))
        .with_state(registar)
        .layer(CompressionLayer::new())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(|request: &Request<axum::body::Body>, span: &Span| {
                    let method = request.method().to_string();
                    let uri = request.uri().to_string();
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .map(|v| v.to_str().unwrap_or_default())
                        .unwrap_or_default();

                    span.record("request_id", request_id);

                    tracing::event!(
                        target: HEALTH_CHECK_API_LOG_TARGET,
                        tracing::Level::INFO, %method, %uri, %request_id, "request");
                })
                .on_response(|response: &Response, latency: Duration, span: &Span| {
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
                            target: HEALTH_CHECK_API_LOG_TARGET,
                            tracing::Level::ERROR, %latency_ms, %status_code, %request_id, "response");
                    } else {
                        tracing::event!(
                            target: HEALTH_CHECK_API_LOG_TARGET,
                            tracing::Level::INFO, %latency_ms, %status_code, %request_id, "response");
                    }
                })
                .on_failure(
                    |failure_classification: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                        let latency = Latency::new(LatencyUnit::Millis, latency);

                        tracing::event!(
                            target: HEALTH_CHECK_API_LOG_TARGET,
                            tracing::Level::ERROR, %failure_classification, %latency, "response failed");
                    },
                ),
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, TcpListener};
    use std::time::Duration;

    use tokio_util::sync::CancellationToken;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::{Registar, RegistrationError, ServiceRegistration};
    use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
    use torrust_tracker_configuration::v3_0_0::shutdown::Shutdown;
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};

    use crate::server::{Error, start_with_cancellation};

    const TEST_COMPLETION_TIMEOUT: Duration = Duration::from_secs(5);

    fn default_drain_timeout() -> Duration {
        Shutdown::default().http_drain_in_secs.as_duration()
    }

    fn any_available_port() -> SocketAddr {
        SocketAddr::from((Ipv4Addr::LOCALHOST, 0))
    }

    fn health_check_api_metadata() -> RuntimeServiceMetadata {
        RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::HealthCheckApi, 0))
    }

    /// A free health-check address that another registration already claims.
    struct ServerStartWithDuplicateRegistration {
        bind_to: SocketAddr,
        registar: Registar<RuntimeServiceMetadata>,
    }

    impl ServerStartWithDuplicateRegistration {
        async fn new() -> Self {
            let available_listener = TcpListener::bind(any_available_port()).expect("select available health-check address");
            let bind_to = available_listener.local_addr().expect("read available health-check address");
            drop(available_listener);
            let registar = Registar::default();
            registar
                .give_form()
                .register(ServiceRegistration::new(
                    ServiceBinding::new(Protocol::HTTP, bind_to).expect("health-check service binding should be valid"),
                    health_check_api_metadata(),
                    None,
                ))
                .await
                .expect("reserve the health-check service registration");

            Self { bind_to, registar }
        }
    }

    #[tokio::test]
    async fn it_should_register_the_token_aware_health_check_api_service() {
        // Arrange
        let registar = Registar::default();
        let cancellation_token = CancellationToken::new();

        // Act
        let running = start_with_cancellation(
            any_available_port(),
            registar.clone(),
            health_check_api_metadata(),
            cancellation_token.clone(),
            default_drain_timeout(),
        )
        .await
        .expect("the token-aware health check API should start");

        // Assert
        let services = registar.services().await;
        assert_eq!(services.len(), 1, "exactly the started health-check API should be registered");
        assert_eq!(services[0].service_binding().bind_address(), running.local_addr);
        assert_eq!(services[0].metadata().service_role(), ServiceRole::HealthCheckApi);

        cancellation_token.cancel();
        drop(running.task.await);
        drop(running.shutdown_controller.await);
    }

    #[tokio::test]
    async fn it_should_drain_the_token_aware_health_check_api_when_its_cancellation_token_is_cancelled() {
        // Arrange
        let cancellation_token = CancellationToken::new();
        let running = start_with_cancellation(
            any_available_port(),
            Registar::default(),
            health_check_api_metadata(),
            cancellation_token.clone(),
            default_drain_timeout(),
        )
        .await
        .expect("the token-aware health check API should start");

        // Act
        cancellation_token.cancel();
        let server_result = tokio::time::timeout(TEST_COMPLETION_TIMEOUT, running.task)
            .await
            .expect("the health check API task should stop after token cancellation")
            .expect("the health check API task should not panic");
        let drain_outcome = tokio::time::timeout(TEST_COMPLETION_TIMEOUT, running.shutdown_controller)
            .await
            .expect("the health check API drain controller should complete after token cancellation")
            .expect("the health check API drain controller should not panic");

        // Assert
        assert!(
            server_result.is_ok(),
            "the health check API task should stop without an error: {server_result:?}"
        );
        assert_eq!(drain_outcome, GracefulShutdownOutcome::Drained);
    }

    #[tokio::test]
    async fn it_should_time_out_the_drain_after_the_configured_drain_timeout_when_a_connection_stays_open() {
        // Arrange
        // The server is configured with a 300 ms drain timeout, far below the 15 s default. A client
        // opens a connection and sends only part of a request header, so the server keeps waiting for
        // it. A second request that completes proves the server has accepted the first connection.
        let configured_drain_timeout = Duration::from_millis(300);
        let cancellation_token = CancellationToken::new();
        let running = start_with_cancellation(
            any_available_port(),
            Registar::default(),
            health_check_api_metadata(),
            cancellation_token.clone(),
            configured_drain_timeout,
        )
        .await
        .expect("the token-aware health check API should start");
        let mut held_connection = std::net::TcpStream::connect(running.local_addr).expect("open the connection to hold");
        std::io::Write::write_all(&mut held_connection, b"GET /health_check HTTP/1.1\r\nHost: tracker\r\n")
            .expect("send a partial request header");
        reqwest::get(format!("http://{}/health_check", running.local_addr))
            .await
            .expect("the server should answer once it has accepted the held connection");

        // Act
        cancellation_token.cancel();
        let drain_outcome = tokio::time::timeout(TEST_COMPLETION_TIMEOUT, running.shutdown_controller)
            .await
            .expect("the drain should end by the configured timeout, not the default one")
            .expect("the health check API drain controller should not panic");

        // Assert
        assert_eq!(
            drain_outcome,
            GracefulShutdownOutcome::TimedOut,
            "a connection held open past the {configured_drain_timeout:?} drain timeout should end the drain as timed out"
        );
        drop(held_connection);
    }

    #[tokio::test]
    async fn it_should_release_the_listener_when_token_aware_startup_registration_fails() {
        // Arrange
        let scenario = ServerStartWithDuplicateRegistration::new().await;

        // Act
        let result = start_with_cancellation(
            scenario.bind_to,
            scenario.registar,
            health_check_api_metadata(),
            CancellationToken::new(),
            default_drain_timeout(),
        )
        .await;

        // Assert
        let binding = match result {
            Err(Error::Registration {
                source: RegistrationError::DuplicateBinding(binding),
            }) => binding,
            Err(error) => panic!("token-aware starter should retain the registration failure source: {error}"),
            Ok(_) => panic!("duplicate registration should fail"),
        };
        assert_eq!(binding.bind_address(), scenario.bind_to);
        TcpListener::bind(scenario.bind_to).expect("health check API listener should be released after registration failure");
    }
}
