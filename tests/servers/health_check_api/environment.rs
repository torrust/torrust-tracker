use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::oneshot::{self, Sender};
use tokio::task::JoinHandle;
use torrust_tracker::bootstrap::jobs::Started;
use torrust_tracker::servers::health_check_api::server;
use torrust_tracker::servers::registar::Registar;
use torrust_tracker::servers::signals::{self, Halted};
use torrust_tracker_configuration::HealthCheckApi;
use tracing::debug;

#[derive(Debug)]
pub enum Error {
    #[allow(dead_code)]
    Error(String),
}

pub struct Running {
    pub binding: SocketAddr,
    pub halt_task: Sender<signals::Halted>,
    pub task: JoinHandle<SocketAddr>,
}

pub struct Stopped {
    pub bind_to: SocketAddr,
}

pub struct Environment<S> {
    pub registar: Registar,
    pub state: S,
}

impl Environment<Stopped> {
    pub fn new(config: &Arc<HealthCheckApi>, registar: Registar) -> Self {
        let bind_to = config
            .bind_address
            .parse::<std::net::SocketAddr>()
            .expect("Tracker API bind_address invalid.");

        Self {
            registar,
            state: Stopped { bind_to },
        }
    }

    /// Start the test environment for the Health Check API.
    /// It runs the API server.
    pub async fn start(self) -> Environment<Running> {
        let (tx_start, rx_start) = oneshot::channel::<Started>();
        let (tx_halt, rx_halt) = tokio::sync::oneshot::channel::<Halted>();

        let register = self.registar.entries();

        debug!(target: "HEALTH CHECK API", "Spawning task to launch the service ...");

        let server = tokio::spawn(async move {
            debug!(target: "HEALTH CHECK API", "Starting the server in a spawned task ...");

            server::start(self.state.bind_to, tx_start, rx_halt, register)
                .await
                .expect("it should start the health check service");

            debug!(target: "HEALTH CHECK API", "Server started. Sending the binding {} ...", self.state.bind_to);

            self.state.bind_to
        });

        debug!(target: "HEALTH CHECK API", "Waiting for spawning task to send the binding ...");

        let binding = rx_start.await.expect("it should send service binding").address;

        Environment {
            registar: self.registar.clone(),
            state: Running {
                task: server,
                halt_task: tx_halt,
                binding,
            },
        }
    }
}

impl Environment<Running> {
    pub async fn new(config: &Arc<HealthCheckApi>, registar: Registar) -> Self {
        Environment::<Stopped>::new(config, registar).start().await
    }

    pub async fn stop(self) -> Result<Environment<Stopped>, Error> {
        self.state
            .halt_task
            .send(Halted::Normal)
            .map_err(|e| Error::Error(e.to_string()))?;

        let bind_to = self.state.task.await.expect("it should shutdown the service");

        Ok(Environment {
            registar: self.registar.clone(),
            state: Stopped { bind_to },
        })
    }
}
