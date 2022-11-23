use std::net::SocketAddr;
use std::sync::Arc;

use super::routes;
use crate::tracker::TorrentTracker;

/// Server that listens on HTTP, needs a `TorrentTracker`
#[derive(Clone)]
pub struct Http {
    tracker: Arc<TorrentTracker>,
}

impl Http {
    #[must_use]
    pub fn new(tracker: Arc<TorrentTracker>) -> Http {
        Http { tracker }
    }

    /// Start the `HttpServer`
    pub fn start(&self, socket_addr: SocketAddr) -> impl warp::Future<Output = ()> {
        let (_addr, server) =
            warp::serve(routes::routes(self.tracker.clone())).bind_with_graceful_shutdown(socket_addr, async move {
                tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
            });

        server
    }

    /// Start the `HttpServer` in TLS mode
    pub fn start_tls(
        &self,
        socket_addr: SocketAddr,
        ssl_cert_path: String,
        ssl_key_path: String,
    ) -> impl warp::Future<Output = ()> {
        let (_addr, server) = warp::serve(routes::routes(self.tracker.clone()))
            .tls()
            .cert_path(ssl_cert_path)
            .key_path(ssl_key_path)
            .bind_with_graceful_shutdown(socket_addr, async move {
                tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
            });

        server
    }
}
