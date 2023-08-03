//! Logic to run the HTTP API server.
//!
//! It contains two main structs: `ApiServer` and `Launcher`,
//! and two main functions: `start` and `start_tls`.
//!
//! The `ApiServer` struct is responsible for:
//! - Starting and stopping the server.
//! - Storing the configuration.
//!
//! `ApiServer` relies on a launcher to start the actual server.
///
/// 1. `ApiServer::start` -> spawns new asynchronous task.
/// 2. `Launcher::start` -> starts the server on the spawned task.
///
/// The `Launcher` struct is responsible for:
///
/// - Knowing how to start the server with graceful shutdown.
///
/// For the time being the `ApiServer` and `Launcher` are only used in tests
/// where we need to start and stop the server multiple times. In production
/// code and the main application uses the `start` and `start_tls` functions
/// to start the servers directly since we do not need to control the server
/// when it's running. In the future we might need to control the server,
/// for example, to restart it to apply new configuration changes, to remotely
/// shutdown the server, etc.
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use axum_server::Handle;
use futures::future::BoxFuture;
use futures::Future;
use log::info;

use super::routes::router;
use crate::servers::signals::shutdown_signal;
use crate::tracker::Tracker;

/// Errors that can occur when starting or stopping the API server.
#[derive(Debug)]
pub enum Error {
    Error(String),
}

/// An alias for the `ApiServer` struct with the `Stopped` state.
#[allow(clippy::module_name_repetitions)]
pub type StoppedApiServer = ApiServer<Stopped>;

/// An alias for the `ApiServer` struct with the `Running` state.
#[allow(clippy::module_name_repetitions)]
pub type RunningApiServer = ApiServer<Running>;

/// A struct responsible for starting and stopping an API server with a
/// specific configuration and keeping track of the started server.
///
/// It's a state machine that can be in one of two
/// states: `Stopped` or `Running`.
#[allow(clippy::module_name_repetitions)]
pub struct ApiServer<S> {
    pub cfg: torrust_tracker_configuration::HttpApi,
    pub state: S,
}

/// The `Stopped` state of the `ApiServer` struct.
pub struct Stopped;

/// The `Running` state of the `ApiServer` struct.
pub struct Running {
    pub bind_addr: SocketAddr,
    task_killer: tokio::sync::oneshot::Sender<u8>,
    task: tokio::task::JoinHandle<()>,
}

impl ApiServer<Stopped> {
    #[must_use]
    pub fn new(cfg: torrust_tracker_configuration::HttpApi) -> Self {
        Self { cfg, state: Stopped {} }
    }

    /// Starts the API server with the given configuration.
    ///
    /// # Errors
    ///
    /// It would return an error if no `SocketAddr` is returned after launching the server.
    ///
    /// # Panics
    ///
    /// It would panic if the bound socket address cannot be sent back to this starter.
    pub async fn start(self, tracker: Arc<Tracker>) -> Result<ApiServer<Running>, Error> {
        let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel::<u8>();
        let (addr_sender, addr_receiver) = tokio::sync::oneshot::channel::<SocketAddr>();

        let configuration = self.cfg.clone();

        let task = tokio::spawn(async move {
            let (bind_addr, server) = Launcher::start(&configuration, tracker, shutdown_signal(shutdown_receiver));

            addr_sender.send(bind_addr).expect("Could not return SocketAddr.");

            server.await;
        });

        let bind_address = addr_receiver
            .await
            .map_err(|_| Error::Error("Could not receive bind_address.".to_string()))?;

        Ok(ApiServer {
            cfg: self.cfg,
            state: Running {
                bind_addr: bind_address,
                task_killer: shutdown_sender,
                task,
            },
        })
    }
}

impl ApiServer<Running> {
    /// Stops the API server.
    ///
    /// # Errors
    ///
    /// It would return an error if the channel for the task killer signal was closed.
    pub async fn stop(self) -> Result<ApiServer<Stopped>, Error> {
        self.state
            .task_killer
            .send(0)
            .map_err(|_| Error::Error("Task killer channel was closed.".to_string()))?;

        drop(self.state.task.await);

        Ok(ApiServer {
            cfg: self.cfg,
            state: Stopped {},
        })
    }
}

/// A struct responsible for starting the API server.
struct Launcher;

impl Launcher {
    /// Starts the API server with graceful shutdown.
    ///
    /// If TLS is enabled in the configuration, it will start the server with
    /// TLS. See [`torrust-tracker-configuration`](torrust_tracker_configuration)
    /// for more  information about configuration.
    pub fn start<F>(
        cfg: &torrust_tracker_configuration::HttpApi,
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

        if let (true, Some(ssl_cert_path), Some(ssl_key_path)) = (&cfg.ssl_enabled, &cfg.ssl_cert_path, &cfg.ssl_key_path) {
            let server = Self::start_tls_with_graceful_shutdown(
                tcp_listener,
                (ssl_cert_path.to_string(), ssl_key_path.to_string()),
                tracker,
                shutdown_signal,
            );

            (bind_addr, server)
        } else {
            let server = Self::start_with_graceful_shutdown(tcp_listener, tracker, shutdown_signal);

            (bind_addr, server)
        }
    }

    /// Starts the API server with graceful shutdown.
    pub fn start_with_graceful_shutdown<F>(
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
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .with_graceful_shutdown(shutdown_signal)
                .await
                .expect("Axum server crashed.");
        })
    }

    /// Starts the API server with graceful shutdown and TLS.
    pub fn start_tls_with_graceful_shutdown<F>(
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
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .await
                .expect("Axum server crashed.");
        })
    }
}

/// Starts the API server with graceful shutdown on the current thread.
///
/// # Panics
///
/// It would panic if it fails to listen to shutdown signal.
pub fn start(socket_addr: SocketAddr, tracker: Arc<Tracker>) -> impl Future<Output = hyper::Result<()>> {
    let app = router(tracker);

    let server = axum::Server::bind(&socket_addr).serve(app.into_make_service());

    server.with_graceful_shutdown(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
        info!("Stopping Torrust APIs server on http://{} ...", socket_addr);
    })
}

/// Starts the API server with graceful shutdown and TLS on the current thread.
///
/// # Panics
///
/// It would panic if it fails to listen to shutdown signal.
pub fn start_tls(
    socket_addr: SocketAddr,
    ssl_config: RustlsConfig,
    tracker: Arc<Tracker>,
) -> impl Future<Output = Result<(), std::io::Error>> {
    let app = router(tracker);

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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_tracker_configuration::Configuration;
    use torrust_tracker_test_helpers::configuration;

    use crate::servers::apis::server::ApiServer;
    use crate::tracker;
    use crate::tracker::statistics;

    fn tracker_configuration() -> Arc<Configuration> {
        Arc::new(configuration::ephemeral())
    }

    #[tokio::test]
    async fn it_should_be_able_to_start_from_stopped_state_and_then_stop_again() {
        let cfg = tracker_configuration();

        let tracker = Arc::new(tracker::Tracker::new(cfg.clone(), None, statistics::Repo::new()).unwrap());

        let stopped_api_server = ApiServer::new(cfg.http_api.clone());

        let running_api_server_result = stopped_api_server.start(tracker).await;

        assert!(running_api_server_result.is_ok());

        let running_api_server = running_api_server_result.unwrap();

        assert!(running_api_server.stop().await.is_ok());
    }
}
