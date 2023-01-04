use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::get;
use axum::{middleware, Router};
use axum_server::tls_rustls::RustlsConfig;
use axum_server::Handle;
use futures::Future;
use log::info;
use warp::hyper;

use super::middlewares::auth::auth;
use super::routes::get_stats;
use crate::tracker;

pub fn start(socket_addr: SocketAddr, tracker: &Arc<tracker::Tracker>) -> impl Future<Output = hyper::Result<()>> {
    let app = Router::new()
        .route("/stats", get(get_stats).with_state(tracker.clone()))
        .layer(middleware::from_fn_with_state(tracker.config.clone(), auth));

    let server = axum::Server::bind(&socket_addr).serve(app.into_make_service());

    server.with_graceful_shutdown(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
        info!("Stopping Torrust APIs server on http://{} ...", socket_addr);
    })
}

pub fn start_tls(
    socket_addr: SocketAddr,
    ssl_config: RustlsConfig,
    tracker: &Arc<tracker::Tracker>,
) -> impl Future<Output = Result<(), std::io::Error>> {
    let app = Router::new()
        .route("/stats", get(get_stats).with_state(tracker.clone()))
        .layer(middleware::from_fn_with_state(tracker.config.clone(), auth));

    let handle = Handle::new();
    let shutdown_handle = handle.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
        info!("Stopping Torrust APIs server on https://{} ...", socket_addr);
        shutdown_handle.shutdown();
    });

    axum_server::bind_rustls(socket_addr, ssl_config)
        .handle(handle)
        .serve(app.into_make_service())
}
