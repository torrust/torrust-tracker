use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use futures::future::BoxFuture;

use crate::signals::shutdown_signal;
use crate::tracker::Tracker;

/// Trait to be implemented by a http server launcher for the tracker.
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

#[derive(Debug)]
pub enum Error {
    Error(String),
}

#[allow(clippy::module_name_repetitions)]
pub type StoppedHttpServer<I> = HttpServer<Stopped<I>>;
#[allow(clippy::module_name_repetitions)]
pub type RunningHttpServer<I> = HttpServer<Running<I>>;

pub struct HttpServer<S> {
    pub cfg: torrust_tracker_configuration::HttpTracker,
    pub state: S,
}

pub struct Stopped<I: HttpServerLauncher> {
    launcher: I,
}

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

    pub async fn start(self, tracker: Arc<Tracker>) -> Result<HttpServer<Running<I>>, Error> {
        let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel::<u8>();
        let (addr_sender, addr_receiver) = tokio::sync::oneshot::channel::<SocketAddr>();

        let configuration = self.cfg.clone();
        let launcher = self.state.launcher;

        let task = tokio::spawn(async move {
            let (bind_addr, server) =
                launcher.start_with_graceful_shutdown(configuration, tracker, shutdown_signal(shutdown_receiver));

            addr_sender.send(bind_addr).unwrap();

            server.await;

            launcher
        });

        let bind_address = addr_receiver.await.expect("Could not receive bind_address.");

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
    pub async fn stop(self) -> Result<HttpServer<Stopped<I>>, Error> {
        self.state.task_killer.send(0).unwrap();

        let launcher = self.state.task.await.map_err(|e| Error::Error(e.to_string()))?;

        Ok(HttpServer {
            cfg: self.cfg,
            state: Stopped { launcher },
        })
    }
}
