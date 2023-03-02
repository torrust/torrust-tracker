use std::future::Future;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use futures::future::BoxFuture;

use super::routes;
use crate::http::tracker_interface::HttpServerLauncher;
use crate::tracker;
use crate::tracker::Tracker;

#[derive(Debug)]
pub enum Error {
    Error(String),
}

pub struct Server;

impl Server {
    pub fn start_with_graceful_shutdown<F>(
        addr: SocketAddr,
        tracker: Arc<Tracker>,
        shutdown_signal: F,
    ) -> (SocketAddr, BoxFuture<'static, ()>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let (bind_addr, server) = warp::serve(routes::routes(tracker)).bind_with_graceful_shutdown(addr, shutdown_signal);

        (bind_addr, Box::pin(server))
    }

    pub fn start_tls_with_graceful_shutdown<F>(
        addr: SocketAddr,
        (ssl_cert_path, ssl_key_path): (&str, &str),
        tracker: Arc<Tracker>,
        shutdown_signal: F,
    ) -> (SocketAddr, BoxFuture<'static, ()>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let (bind_addr, server) = warp::serve(routes::routes(tracker))
            .tls()
            .cert_path(ssl_cert_path)
            .key_path(ssl_key_path)
            .bind_with_graceful_shutdown(addr, shutdown_signal);

        (bind_addr, Box::pin(server))
    }
}

impl HttpServerLauncher for Server {
    fn new() -> Self {
        Self {}
    }

    fn start_with_graceful_shutdown<F>(
        &self,
        cfg: torrust_tracker_configuration::HttpTracker,
        tracker: Arc<Tracker>,
        shutdown_signal: F,
    ) -> (SocketAddr, BoxFuture<'static, ()>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let addr = SocketAddr::from_str(&cfg.bind_address).expect("bind_address is not a valid SocketAddr.");

        if let (true, Some(ssl_cert_path), Some(ssl_key_path)) = (cfg.ssl_enabled, &cfg.ssl_cert_path, &cfg.ssl_key_path) {
            Self::start_tls_with_graceful_shutdown(addr, (ssl_cert_path, ssl_key_path), tracker, shutdown_signal)
        } else {
            Self::start_with_graceful_shutdown(addr, tracker, shutdown_signal)
        }
    }
}

/// Server that listens on HTTP, needs a `tracker::TorrentTracker`
#[derive(Clone)]
pub struct Http {
    tracker: Arc<tracker::Tracker>,
}

impl Http {
    #[must_use]
    pub fn new(tracker: Arc<tracker::Tracker>) -> Http {
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
