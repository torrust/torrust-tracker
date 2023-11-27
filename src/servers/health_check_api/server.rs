//! Logic to run the Health Check HTTP API server.
//!
//! This API is intended to be used by the container infrastructure to check if
//! the whole application is healthy.
use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::get;
use axum::{Json, Router};
use futures::Future;
use log::info;
use serde_json::json;
use tokio::sync::oneshot::Sender;
use torrust_tracker_configuration::Configuration;

use crate::bootstrap::jobs::health_check_api::ApiServerJobStarted;
use crate::servers::health_check_api::handlers::health_check_handler;

/// Starts Health Check API server.
///
/// # Panics
///
/// Will panic if binding to the socket address fails.
pub fn start(
    socket_addr: SocketAddr,
    tx: Sender<ApiServerJobStarted>,
    config: Arc<Configuration>,
) -> impl Future<Output = hyper::Result<()>> {
    let app = Router::new()
        .route("/", get(|| async { Json(json!({})) }))
        .route("/health_check", get(health_check_handler))
        .with_state(config);

    let server = axum::Server::bind(&socket_addr).serve(app.into_make_service());

    let bound_addr = server.local_addr();

    info!("Health Check API server listening on http://{}", bound_addr);

    let running = server.with_graceful_shutdown(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
        info!("Stopping Torrust Health Check API server o http://{} ...", socket_addr);
    });

    tx.send(ApiServerJobStarted { bound_addr })
        .expect("the Health Check API server should not be dropped");

    running
}
