use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::get;
use axum::{middleware, Router};
use futures::Future;
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
    })
}

pub fn start_tls(
    socket_addr: SocketAddr,
    _ssl_cert_path: &str,
    _ssl_key_path: &str,
    tracker: &Arc<tracker::Tracker>,
) -> impl Future<Output = hyper::Result<()>> {
    // todo: for the time being, it's just a copy & paste from start(...).

    let app = Router::new()
        .route("/stats", get(get_stats).with_state(tracker.clone()))
        .layer(middleware::from_fn_with_state(tracker.config.clone(), auth));

    let server = axum::Server::bind(&socket_addr).serve(app.into_make_service());

    server.with_graceful_shutdown(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
    })
}
