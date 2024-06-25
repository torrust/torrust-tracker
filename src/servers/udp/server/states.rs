use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

use derive_more::Constructor;
use tokio::task::JoinHandle;

use super::spawner::Spawner;
use super::{Server, UdpError};
use crate::bootstrap::jobs::Started;
use crate::core::Tracker;
use crate::servers::registar::{ServiceRegistration, ServiceRegistrationForm};
use crate::servers::signals::Halted;
use crate::servers::udp::server::launcher::Launcher;
use crate::servers::udp::UDP_TRACKER_LOG_TARGET;

/// A UDP server instance controller with no UDP instance running.
#[allow(clippy::module_name_repetitions)]
pub type StoppedUdpServer = Server<Stopped>;

/// A UDP server instance controller with a running UDP instance.
#[allow(clippy::module_name_repetitions)]
pub type RunningUdpServer = Server<Running>;

/// A stopped UDP server state.

pub struct Stopped {
    pub launcher: Spawner,
}

/// A running UDP server state.
#[derive(Debug, Constructor)]
pub struct Running {
    /// The address where the server is bound.
    pub binding: SocketAddr,
    pub halt_task: tokio::sync::oneshot::Sender<Halted>,
    pub task: JoinHandle<Spawner>,
}

impl Server<Stopped> {
    /// Creates a new `UdpServer` instance in `stopped`state.
    #[must_use]
    pub fn new(launcher: Spawner) -> Self {
        Self {
            state: Stopped { launcher },
        }
    }

    /// It starts the server and returns a `UdpServer` controller in `running`
    /// state.
    ///
    /// # Errors
    ///
    /// Will return `Err` if UDP can't bind to given bind address.
    ///
    /// # Panics
    ///
    /// It panics if unable to receive the bound socket address from service.
    ///
    pub async fn start(self, tracker: Arc<Tracker>, form: ServiceRegistrationForm) -> Result<Server<Running>, std::io::Error> {
        let (tx_start, rx_start) = tokio::sync::oneshot::channel::<Started>();
        let (tx_halt, rx_halt) = tokio::sync::oneshot::channel::<Halted>();

        assert!(!tx_halt.is_closed(), "Halt channel for UDP tracker should be open");

        // May need to wrap in a task to about a tokio bug.
        let task = self.state.launcher.start(tracker, tx_start, rx_halt);

        let binding = rx_start.await.expect("it should be able to start the service").address;
        let local_addr = format!("udp://{binding}");

        form.send(ServiceRegistration::new(binding, Launcher::check))
            .expect("it should be able to send service registration");

        let running_udp_server: Server<Running> = Server {
            state: Running {
                binding,
                halt_task: tx_halt,
                task,
            },
        };

        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_addr, "UdpServer<Stopped>::start (running)");

        Ok(running_udp_server)
    }
}

impl Server<Running> {
    /// It stops the server and returns a `UdpServer` controller in `stopped`
    /// state.
    ///     
    /// # Errors
    ///
    /// Will return `Err` if the oneshot channel to send the stop signal
    /// has already been called once.
    ///
    /// # Panics
    ///
    /// It panics if unable to shutdown service.
    pub async fn stop(self) -> Result<Server<Stopped>, UdpError> {
        self.state
            .halt_task
            .send(Halted::Normal)
            .map_err(|e| UdpError::Error(e.to_string()))?;

        let launcher = self.state.task.await.expect("it should shutdown service");

        let stopped_api_server: Server<Stopped> = Server {
            state: Stopped { launcher },
        };

        Ok(stopped_api_server)
    }
}
