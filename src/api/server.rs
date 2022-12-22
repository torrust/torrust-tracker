use std::net::SocketAddr;
use std::sync::Arc;

use warp::serve;

use super::routes::routes;
use crate::tracker;

pub fn start(socket_addr: SocketAddr, tracker: &Arc<tracker::Tracker>) -> impl warp::Future<Output = ()> {
    let (_addr, api_server) = serve(routes(tracker)).bind_with_graceful_shutdown(socket_addr, async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
    });

    api_server
}

pub fn start_tls(
    socket_addr: SocketAddr,
    ssl_cert_path: String,
    ssl_key_path: String,
    tracker: &Arc<tracker::Tracker>,
) -> impl warp::Future<Output = ()> {
    let (_addr, api_server) = serve(routes(tracker))
        .tls()
        .cert_path(ssl_cert_path)
        .key_path(ssl_key_path)
        .bind_with_graceful_shutdown(socket_addr, async move {
            tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
        });

    api_server
}
