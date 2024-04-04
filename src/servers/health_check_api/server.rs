//! Logic to run the Health Check HTTP API server.
//!
//! This API is intended to be used by the container infrastructure to check if
//! the whole application is healthy.
use std::net::SocketAddr;
use std::time::Duration;

use axum::http::HeaderName;
use axum::response::Response;
use axum::routing::get;
use axum::Json;
use derive_more::{Constructor, Display};
use futures::{FutureExt as _, TryFutureExt as _};
use hyper::Request;
use serde_json::json;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{error, info, instrument, trace, warn, Level, Span};

use crate::servers::health_check_api::handlers::health_check_handler;
use crate::servers::registar::{FnSpawnServiceHeathCheck, ServiceHealthCheckJob, ServiceRegistry};
use crate::servers::service::{AddrFuture, Error, Handle, Launcher, TaskFuture};
use crate::servers::signals::Halted;
use crate::servers::tcp::graceful_axum_shutdown;

/// Placeholder Check Function for the Health Check Itself
///
/// # Errors
///
/// This function will return an error if the check would fail.
///
#[must_use]
#[instrument(ret)]
fn check_fn(binding: &SocketAddr) -> ServiceHealthCheckJob {
    let url = format!("http://{binding}/health_check");

    let info = format!("todo self-check health check at: {url}");

    let job = tokio::spawn(async move { Ok("Todo: Not Implemented".to_string()) });
    ServiceHealthCheckJob::new(*binding, info, job)
}

pub struct HealthCheckHandle {
    pub axum_handle: axum_server::Handle,
    tx_shutdown: Option<tokio::sync::oneshot::Sender<Halted>>,
}

impl std::fmt::Debug for HealthCheckHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HealthCheckHandle")
            .field("axum_handle_conn:", &self.axum_handle.connection_count())
            .finish_non_exhaustive()
    }
}

#[derive(Constructor, Clone, Debug, Display)]
#[display(fmt = "intended_address: {addr}, registry: {registry}")]
pub struct HealthCheckLauncher {
    pub addr: SocketAddr,
    pub registry: ServiceRegistry,
}

impl HealthCheckHandle {
    #[instrument(err, ret)]
    fn shutdown(&mut self) -> Result<(), Error> {
        trace!("the internal shut down was called");
        if let Some(tx) = self.tx_shutdown.take() {
            trace!("sending a normal halt on the shutdown channel");
            tx.send(Halted::Normal)
                .map_err(|err| Error::UnableToSendHaltingMessage { err })?;
        } else {
            error!("shutdown was called, but the channel was missing!");
            panic!();
        };
        Ok(())
    }
}

impl Default for HealthCheckHandle {
    #[instrument(ret)]
    fn default() -> Self {
        trace!("setup the shutdown channel");
        let (tx_shutdown, rx_shutdown) = tokio::sync::oneshot::channel::<Halted>();

        trace!("setup the axum handle");
        let axum_handle = axum_server::Handle::default();

        trace!("setup the graceful axum meta-handler");
        let () = graceful_axum_shutdown(axum_handle.clone(), rx_shutdown, "Health Check Server".to_string());

        trace!("returning the new default handler");
        Self {
            axum_handle: axum_server::Handle::new(),
            tx_shutdown: Some(tx_shutdown),
        }
    }
}

impl Handle for HealthCheckHandle {
    #[instrument(ret)]
    fn stop(mut self) -> Result<(), Error> {
        info!("shutdown function was called");
        self.shutdown()
    }

    #[instrument]
    fn listening(&self) -> AddrFuture<'_> {
        info!("return the listening future form the axum handler");
        self.axum_handle.listening().boxed()
    }
}

impl Drop for HealthCheckHandle {
    #[instrument]
    fn drop(&mut self) {
        warn!("the health check handle was dropped, now shutting down");
        self.shutdown().expect("it should shutdown when dropped");
    }
}

impl Launcher<HealthCheckHandle> for HealthCheckLauncher {
    #[instrument(err, fields(self = %self))]
    fn start(self) -> Result<(TaskFuture<'static, (), Error>, HealthCheckHandle, FnSpawnServiceHeathCheck), Error> {
        trace!("setup the health check handler");
        let handle = HealthCheckHandle::default();

        trace!("make service task");
        let task: TaskFuture<'_, (), Error> = {
            trace!(address = ?self.addr, "try to bind on socket");
            let listener = std::net::TcpListener::bind(self.addr).map_err(|e| Error::UnableToBindToSocket {
                addr: self.addr,
                err: e.into(),
            })?;

            trace!("try to get local address");
            let addr = listener
                .local_addr()
                .map_err(|e| Error::UnableToGetLocalAddress { err: e.into() })?;
            info!(address = ?addr, "health tracker bound to tcp socket: {addr}");

            trace!("setup router");
            let router = router(self.registry, addr);

            trace!("make router into service");
            let make_service = router.into_make_service_with_connect_info::<SocketAddr>();

            info!("start and return axum service");
            axum_server::from_tcp(listener)
                .handle(handle.axum_handle.clone())
                .serve(make_service)
                .map_err(|e| Error::UnableToServe { err: e.into() })
                .boxed()
        };

        trace!("returning the axum task, handle, and check function closure");
        Ok((task, handle, check_fn))
    }
}

#[instrument(fields(registry = %registry))]
fn router(registry: ServiceRegistry, addr: SocketAddr) -> axum::Router {
    axum::Router::new()
        .route("/", get(|| async { Json(json!({})) }))
        .route("/health_check", get(health_check_handler))
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
                .on_response(move|response: &Response, latency: Duration, _span: &Span| {
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
