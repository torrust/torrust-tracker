//! Logic to start new HTTP server instances.
use std::future::Future;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use axum_server::tls_rustls::RustlsConfig;
use axum_server::Handle;
use futures::future::BoxFuture;
use log::info;

use super::routes::router;
use crate::core::Tracker;
use crate::servers::http::server::HttpServerLauncher;

#[derive(Debug)]
pub enum Error {
    Error(String),
}

pub struct Launcher;

impl Launcher {
    /// It starts a new HTTP server instance from a TCP listener with graceful shutdown.
    ///
    /// # Panics
    ///
    /// Will panic if:
    ///
    /// - The TCP listener could not be bound.
    /// - The Axum server crashes.
    pub fn start_from_tcp_listener_with_graceful_shutdown<F>(
        tcp_listener: std::net::TcpListener,
        tracker: Arc<Tracker>,
        shutdown_signal: F,
    ) -> BoxFuture<'static, ()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let app = router(tracker);

        Box::pin(async {
            axum::Server::from_tcp(tcp_listener)
                .expect("Could not bind to tcp listener.")
                .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
                .with_graceful_shutdown(shutdown_signal)
                .await
                .expect("Axum server crashed.");
        })
    }

    /// It starts a new HTTPS server instance from a TCP listener with graceful shutdown.
    ///
    /// # Panics
    ///
    /// Will panic if:
    ///
    /// - The SSL certificate could not be read from the provided path or is invalid.
    /// - The Axum server crashes.
    pub fn start_tls_from_tcp_listener_with_graceful_shutdown<F>(
        tcp_listener: std::net::TcpListener,
        (ssl_cert_path, ssl_key_path): (String, String),
        tracker: Arc<Tracker>,
        shutdown_signal: F,
    ) -> BoxFuture<'static, ()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let app = router(tracker);

        let handle = Handle::new();

        let cloned_handle = handle.clone();

        tokio::task::spawn_local(async move {
            shutdown_signal.await;
            cloned_handle.shutdown();
        });

        Box::pin(async {
            let tls_config = RustlsConfig::from_pem_file(ssl_cert_path, ssl_key_path)
                .await
                .expect("Could not read tls cert.");

            axum_server::from_tcp_rustls(tcp_listener, tls_config)
                .handle(handle)
                .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
                .await
                .expect("Axum server crashed.");
        })
    }
}

#[async_trait]
impl HttpServerLauncher for Launcher {
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
        let tcp_listener = std::net::TcpListener::bind(addr).expect("Could not bind tcp_listener to address.");
        let bind_addr = tcp_listener
            .local_addr()
            .expect("Could not get local_addr from tcp_listener.");

        if let (true, Some(ssl_cert_path), Some(ssl_key_path)) = (cfg.ssl_enabled, &cfg.ssl_cert_path, &cfg.ssl_key_path) {
            let server = Self::start_tls_from_tcp_listener_with_graceful_shutdown(
                tcp_listener,
                (ssl_cert_path.to_string(), ssl_key_path.to_string()),
                tracker,
                shutdown_signal,
            );

            (bind_addr, server)
        } else {
            let server = Self::start_from_tcp_listener_with_graceful_shutdown(tcp_listener, tracker, shutdown_signal);

            (bind_addr, server)
        }
    }
}

/// Starts a new HTTP server instance.
///
/// # Panics
///
/// Panics if the server could not listen to shutdown (ctrl+c) signal.
pub fn start(socket_addr: std::net::SocketAddr, tracker: Arc<Tracker>) -> impl Future<Output = hyper::Result<()>> {
    let app = router(tracker);

    let server = axum::Server::bind(&socket_addr).serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>());

    server.with_graceful_shutdown(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
        info!("Stopping Torrust HTTP tracker server on http://{} ...", socket_addr);
    })
}

/// Starts a new HTTPS server instance.
///
/// # Panics
///
/// Panics if the server could not listen to shutdown (ctrl+c) signal.
pub fn start_tls(
    socket_addr: std::net::SocketAddr,
    ssl_config: RustlsConfig,
    tracker: Arc<Tracker>,
) -> impl Future<Output = Result<(), std::io::Error>> {
    let app = router(tracker);

    let handle = Handle::new();
    let shutdown_handle = handle.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
        info!("Stopping Torrust HTTP tracker server on https://{} ...", socket_addr);
        shutdown_handle.shutdown();
    });

    axum_server::bind_rustls(socket_addr, ssl_config)
        .handle(handle)
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
}
