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
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use axum_server::Handle;
use derive_more::Constructor;
use futures::future::BoxFuture;
use log::error;
use tokio::sync::oneshot::{Receiver, Sender};

use super::routes::router;
use crate::bootstrap::jobs::Started;
use crate::core::Tracker;
use crate::servers::signals::{graceful_shutdown, Halted};

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
    pub state: S,
}

/// The `Stopped` state of the `ApiServer` struct.
pub struct Stopped {
    launcher: Launcher,
}

/// The `Running` state of the `ApiServer` struct.
pub struct Running {
    pub binding: SocketAddr,
    pub halt_task: tokio::sync::oneshot::Sender<Halted>,
    pub task: tokio::task::JoinHandle<Launcher>,
}

impl ApiServer<Stopped> {
    #[must_use]
    pub fn new(launcher: Launcher) -> Self {
        Self {
            state: Stopped { launcher },
        }
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
        let (tx_start, rx_start) = tokio::sync::oneshot::channel::<Started>();
        let (tx_halt, rx_halt) = tokio::sync::oneshot::channel::<Halted>();

        let launcher = self.state.launcher;

        let task = tokio::spawn(async move {
            launcher.start(tracker, tx_start, rx_halt).await;
            launcher
        });

        //let address = rx_start.await.expect("unable to start service").address;
        let api_server = match rx_start.await {
            Ok(started) => ApiServer {
                state: Running {
                    binding: started.address,
                    halt_task: tx_halt,
                    task,
                },
            },
            Err(err) => {
                let msg = format!("unable to start API server: {err}");
                error!("{}", msg);
                panic!("{}", msg);
            }
        };

        Ok(api_server)
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
            .halt_task
            .send(Halted::Normal)
            .map_err(|_| Error::Error("Task killer channel was closed.".to_string()))?;

        let launcher = self.state.task.await.map_err(|e| Error::Error(e.to_string()))?;

        Ok(ApiServer {
            state: Stopped { launcher },
        })
    }
}

/// A struct responsible for starting the API server.
#[derive(Constructor, Debug)]
pub struct Launcher {
    bind_to: SocketAddr,
    tls: Option<RustlsConfig>,
}

impl Launcher {
    /// Starts the API server with graceful shutdown.
    ///
    /// If TLS is enabled in the configuration, it will start the server with
    /// TLS. See [`torrust-tracker-configuration`](torrust_tracker_configuration)
    /// for more  information about configuration.
    ///
    /// # Panics
    ///
    /// Will panic if unable to bind to the socket, or unable to get the address of the bound socket.
    /// Will also panic if unable to send message regarding the bound socket address.
    pub fn start(&self, tracker: Arc<Tracker>, tx_start: Sender<Started>, rx_halt: Receiver<Halted>) -> BoxFuture<'static, ()> {
        let router = router(tracker);
        let socket = std::net::TcpListener::bind(self.bind_to).expect("Could not bind tcp_listener to address.");
        let address = socket.local_addr().expect("Could not get local_addr from tcp_listener.");

        let handle = Handle::new();

        tokio::task::spawn(graceful_shutdown(
            handle.clone(),
            rx_halt,
            format!("Shutting down http server on socket address: {address}"),
        ));

        let tls = self.tls.clone();

        let running = Box::pin(async {
            match tls {
                Some(tls) => axum_server::from_tcp_rustls(socket, tls)
                    .handle(handle)
                    .serve(router.into_make_service_with_connect_info::<std::net::SocketAddr>())
                    .await
                    .expect("Axum server crashed."),
                None => axum_server::from_tcp(socket)
                    .handle(handle)
                    .serve(router.into_make_service_with_connect_info::<std::net::SocketAddr>())
                    .await
                    .expect("Axum server crashed."),
            }
        });

        tx_start
            .send(Started { address })
            .expect("the HTTP(s) Tracker service should not be dropped");

        running
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_tracker_test_helpers::configuration::ephemeral_mode_public;

    use crate::bootstrap::app::initialize_with_configuration;
    use crate::bootstrap::jobs::make_rust_tls;
    use crate::servers::apis::server::{ApiServer, Launcher};

    #[tokio::test]
    async fn it_should_be_able_to_start_and_stop() {
        let cfg = Arc::new(ephemeral_mode_public());
        let tracker = initialize_with_configuration(&cfg);
        let config = &cfg.http_api;

        let bind_to = config
            .bind_address
            .parse::<std::net::SocketAddr>()
            .expect("Tracker API bind_address invalid.");

        let tls = make_rust_tls(config.ssl_enabled, &config.ssl_cert_path, &config.ssl_key_path)
            .await
            .map(|tls| tls.expect("tls config failed"));

        let stopped = ApiServer::new(Launcher::new(bind_to, tls));
        let started = stopped.start(tracker).await.expect("it should start the server");
        let stopped = started.stop().await.expect("it should stop the server");

        assert_eq!(stopped.state.launcher.bind_to, bind_to);
    }
}
