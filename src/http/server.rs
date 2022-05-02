use std::net::SocketAddr;
use std::sync::Arc;

use crate::TorrentTracker;
use crate::http::routes;

/// Server that listens on HTTP, needs a TorrentTracker
#[derive(Clone)]
pub struct HttpServer {
    tracker: Arc<TorrentTracker>,
}

impl HttpServer {
    pub fn new(tracker: Arc<TorrentTracker>) -> HttpServer {
        HttpServer {
            tracker
        }
    }

    /// Start the HttpServer
    pub async fn start(&self, socket_addr: SocketAddr) {
        let (_addr, server) = warp::serve(routes(self.tracker.clone()))
            .bind_with_graceful_shutdown(socket_addr, async move {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to listen to shutdown signal");
            });
        tokio::task::spawn(server);
    }

    /// Start the HttpServer in TLS mode
    pub async fn start_tls(&self, socket_addr: SocketAddr, ssl_cert_path: &str, ssl_key_path: &str) {
        let (_addr, server) = warp::serve(routes(self.tracker.clone()))
            .tls()
            .cert_path(ssl_cert_path)
            .key_path(ssl_key_path)
            .bind_with_graceful_shutdown(socket_addr, async move {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to listen to shutdown signal");
            });
        tokio::task::spawn(server);
    }
}
