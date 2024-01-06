//! Logic to run the Health Check HTTP API server.
//!
//! This API is intended to be used by the container infrastructure to check if
//! the whole application is healthy.
use std::net::SocketAddr;

use axum::routing::get;
use axum::{Json, Router};
use axum_server::Handle;
use futures::Future;
use serde_json::json;
use tokio::sync::oneshot::{Receiver, Sender};

use crate::bootstrap::jobs::Started;
use crate::servers::health_check_api::handlers::health_check_handler;
use crate::servers::registar::ServiceRegistry;
use crate::servers::signals::{graceful_shutdown, Halted};

/// Starts Health Check API server.
///
/// # Panics
///
/// Will panic if binding to the socket address fails.
pub fn start(
    bind_to: SocketAddr,
    tx: Sender<Started>,
    rx_halt: Receiver<Halted>,
    register: ServiceRegistry,
) -> impl Future<Output = Result<(), std::io::Error>> {
    let router = Router::new()
        .route("/", get(|| async { Json(json!({})) }))
        .route("/health_check", get(health_check_handler))
        .with_state(register);

    let socket = std::net::TcpListener::bind(bind_to).expect("Could not bind tcp_listener to address.");
    let address = socket.local_addr().expect("Could not get local_addr from tcp_listener.");

    let handle = Handle::new();

    tokio::task::spawn(graceful_shutdown(
        handle.clone(),
        rx_halt,
        format!("shutting down http server on socket address: {address}"),
    ));

    let running = axum_server::from_tcp(socket)
        .handle(handle)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>());

    tx.send(Started { address })
        .expect("the Health Check API server should not be dropped");

    running
}
