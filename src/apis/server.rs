use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use futures::Future;
use warp::hyper;

use super::routes::root;
use crate::tracker;

pub fn start(socket_addr: SocketAddr, _tracker: &Arc<tracker::Tracker>) -> impl Future<Output = hyper::Result<()>> {
    let app = Router::new().route("/", get(root));

    let server = axum::Server::bind(&socket_addr).serve(app.into_make_service());

    server.with_graceful_shutdown(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
    })
}

pub fn start_tls(
    socket_addr: SocketAddr,
    _ssl_cert_path: &str,
    _ssl_key_path: &str,
    _tracker: &Arc<tracker::Tracker>,
) -> impl Future<Output = hyper::Result<()>> {
    let app = Router::new().route("/", get(root));

    let server = axum::Server::bind(&socket_addr).serve(app.into_make_service());

    server.with_graceful_shutdown(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
    })
}
