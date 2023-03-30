//! Module to handle the HTTP server instances.
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use futures::future::BoxFuture;

use crate::servers::signals::shutdown_signal;
use crate::tracker::Tracker;

/// Trait to be implemented by a HTTP server launcher for the tracker.
///
/// A launcher is responsible for starting the server and returning the
/// `SocketAddr` it is bound to.
#[allow(clippy::module_name_repetitions)]
pub trait HttpServerLauncher: Sync + Send {
    fn new() -> Self;

    fn start_with_graceful_shutdown<F>(
        &self,
        cfg: torrust_tracker_configuration::HttpTracker,
        tracker: Arc<Tracker>,
        shutdown_signal: F,
    ) -> (SocketAddr, BoxFuture<'static, ()>)
    where
        F: Future<Output = ()> + Send + 'static;
}

/// Error that can occur when starting or stopping the HTTP server.
///
/// Some errors triggered while starting the server are:
///
/// - The spawned server cannot send its `SocketAddr` back to the main thread.
/// - The launcher cannot receive the `SocketAddr` from the spawned server.
///
/// Some errors triggered while stopping the server are:
///
/// - The channel to send the shutdown signal to the server is closed.
/// - The task to shutdown the server on the spawned server failed to execute to
/// completion.
#[derive(Debug)]
pub enum Error {
    /// Any kind of error starting or stopping the server.
    Error(String), // todo: refactor to use thiserror and add more variants for specific errors.
}

/// A stopped HTTP server.
#[allow(clippy::module_name_repetitions)]
pub type StoppedHttpServer<I> = HttpServer<Stopped<I>>;

/// A running HTTP server.
#[allow(clippy::module_name_repetitions)]
pub type RunningHttpServer<I> = HttpServer<Running<I>>;

/// A HTTP running server controller.
///
/// It's responsible for:
///
/// - Keeping the initial configuration of the server.
/// - Starting and stopping the server.
/// - Keeping the state of the server: `running` or `stopped`.
///
/// It's an state machine. Configurations cannot be changed. This struct
/// represents concrete configuration and state. It allows to start and stop the
/// server but always keeping the same configuration.
///
/// > **NOTICE**: if the configurations changes after running the server it will
/// reset to the initial value after stopping the server. This struct is not
/// intended to persist configurations between runs.
#[allow(clippy::module_name_repetitions)]
pub struct HttpServer<S> {
    /// The configuration of the server that will be used every time the server
    /// is started.
    pub cfg: torrust_tracker_configuration::HttpTracker,
    /// The state of the server: `running` or `stopped`.
    pub state: S,
}

/// A stopped HTTP server state.
pub struct Stopped<I: HttpServerLauncher> {
    launcher: I,
}

/// A running HTTP server state.
pub struct Running<I: HttpServerLauncher> {
    pub bind_addr: SocketAddr,
    task_killer: tokio::sync::oneshot::Sender<u8>,
    task: tokio::task::JoinHandle<I>,
}

impl<I: HttpServerLauncher + 'static> HttpServer<Stopped<I>> {
    pub fn new(cfg: torrust_tracker_configuration::HttpTracker, launcher: I) -> Self {
        Self {
            cfg,
            state: Stopped { launcher },
        }
    }

    /// It starts the server and returns a `HttpServer` controller in `running`
    /// state.
    ///
    /// # Errors
    ///
    /// It would return an error if no `SocketAddr` is returned after launching the server.
    pub async fn start(self, tracker: Arc<Tracker>) -> Result<HttpServer<Running<I>>, Error> {
        let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel::<u8>();
        let (addr_sender, addr_receiver) = tokio::sync::oneshot::channel::<SocketAddr>();

        let configuration = self.cfg.clone();
        let launcher = self.state.launcher;

        let task = tokio::spawn(async move {
            let (bind_addr, server) =
                launcher.start_with_graceful_shutdown(configuration, tracker, shutdown_signal(shutdown_receiver));

            addr_sender.send(bind_addr).expect("Could not return SocketAddr.");

            server.await;

            launcher
        });

        let bind_address = addr_receiver
            .await
            .map_err(|_| Error::Error("Could not receive bind_address.".to_string()))?;

        Ok(HttpServer {
            cfg: self.cfg,
            state: Running {
                bind_addr: bind_address,
                task_killer: shutdown_sender,
                task,
            },
        })
    }
}

impl<I: HttpServerLauncher> HttpServer<Running<I>> {
    /// It stops the server and returns a `HttpServer` controller in `stopped`
    /// state.
    ///
    /// # Errors
    ///
    /// It would return an error if the channel for the task killer signal was closed.
    pub async fn stop(self) -> Result<HttpServer<Stopped<I>>, Error> {
        self.state
            .task_killer
            .send(0)
            .map_err(|_| Error::Error("Task killer channel was closed.".to_string()))?;

        let launcher = self.state.task.await.map_err(|e| Error::Error(e.to_string()))?;

        Ok(HttpServer {
            cfg: self.cfg,
            state: Stopped { launcher },
        })
    }
}
