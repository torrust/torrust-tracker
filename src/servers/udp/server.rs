//! Module to handle the UDP server instances.
//!
//! There are two main types in this module:
//!
//! - [`UdpServer`]: a controller to
//! start and stop the server.
//! - [`Udp`]: the server launcher.
//!
//! The `UdpServer` is an state machine for a given configuration. This struct
//! represents concrete configuration and state. It allows to start and
//! stop the server but always keeping the same configuration.
//!
//! The `Udp` is the server launcher. It's responsible for launching the UDP
//! but without keeping any state.
//!
//! For the time being, the `UdpServer` is only used for testing purposes,
//! because we want to be able to start and stop the server multiple times, and
//! we want to know the bound address and the current state of the server.
//! In production, the `Udp` launcher is used directly.
use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;

use aquatic_udp_protocol::Response;
use derive_more::Constructor;
use ringbuf::{Rb, StaticRb};
use tokio::net::UdpSocket;
use tokio::sync::oneshot;
use tokio::task::{AbortHandle, JoinHandle};
use tokio::{select, task};
use tracing::{debug, error, info, trace};

use super::UdpRequest;
use crate::bootstrap::jobs::Started;
use crate::core::Tracker;
use crate::servers::registar::{ServiceHealthCheckJob, ServiceRegistration, ServiceRegistrationForm};
use crate::servers::signals::{shutdown_signal_with_message, Halted};
use crate::servers::udp::handlers;
use crate::shared::bit_torrent::tracker::udp::client::check;
use crate::shared::bit_torrent::tracker::udp::MAX_PACKET_SIZE;

/// Error that can occur when starting or stopping the UDP server.
///
/// Some errors triggered while starting the server are:
///
/// - The server cannot bind to the given address.
/// - It cannot get the bound address.
///
/// Some errors triggered while stopping the server are:
///
/// - The [`UdpServer`] cannot send the
///  shutdown signal to the spawned UDP service thread.
#[derive(Debug)]
pub enum Error {
    /// Any kind of error starting or stopping the server.
    Error(String), // todo: refactor to use thiserror and add more variants for specific errors.
}

/// A UDP server instance controller with no UDP instance running.
#[allow(clippy::module_name_repetitions)]
pub type StoppedUdpServer = UdpServer<Stopped>;

/// A UDP server instance controller with a running UDP instance.
#[allow(clippy::module_name_repetitions)]
pub type RunningUdpServer = UdpServer<Running>;

/// A UDP server instance controller.
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
pub struct UdpServer<S> {
    /// The state of the server: `running` or `stopped`.
    pub state: S,
}

/// A stopped UDP server state.

pub struct Stopped {
    launcher: Launcher,
}

/// A running UDP server state.
#[derive(Debug, Constructor)]
pub struct Running {
    /// The address where the server is bound.
    pub binding: SocketAddr,
    pub halt_task: tokio::sync::oneshot::Sender<Halted>,
    pub task: JoinHandle<Launcher>,
}

impl UdpServer<Stopped> {
    /// Creates a new `UdpServer` instance in `stopped`state.
    #[must_use]
    pub fn new(launcher: Launcher) -> Self {
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
    pub async fn start(self, tracker: Arc<Tracker>, form: ServiceRegistrationForm) -> Result<UdpServer<Running>, Error> {
        let (tx_start, rx_start) = tokio::sync::oneshot::channel::<Started>();
        let (tx_halt, rx_halt) = tokio::sync::oneshot::channel::<Halted>();

        assert!(!tx_halt.is_closed(), "Halt channel for UDP tracker should be open");

        // May need to wrap in a task to about a tokio bug.
        let task = self.state.launcher.start(tracker, tx_start, rx_halt);

        let binding = rx_start.await.expect("it should be able to start the service").address;

        form.send(ServiceRegistration::new(binding, Udp::check))
            .expect("it should be able to send service registration");

        let running_udp_server: UdpServer<Running> = UdpServer {
            state: Running {
                binding,
                halt_task: tx_halt,
                task,
            },
        };

        info!("Running UDP Tracker on Socket: {}", running_udp_server.state.binding);

        Ok(running_udp_server)
    }
}

impl UdpServer<Running> {
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
    pub async fn stop(self) -> Result<UdpServer<Stopped>, Error> {
        self.state
            .halt_task
            .send(Halted::Normal)
            .map_err(|e| Error::Error(e.to_string()))?;

        let launcher = self.state.task.await.expect("unable to shutdown service");

        let stopped_api_server: UdpServer<Stopped> = UdpServer {
            state: Stopped { launcher },
        };

        Ok(stopped_api_server)
    }
}

#[derive(Constructor, Copy, Clone, Debug)]
pub struct Launcher {
    bind_to: SocketAddr,
}

impl Launcher {
    /// It starts the UDP server instance.
    ///
    /// # Panics
    ///
    /// It would panic if unable to resolve the `local_addr` from the supplied ´socket´.
    pub fn start(
        &self,
        tracker: Arc<Tracker>,
        tx_start: oneshot::Sender<Started>,
        rx_halt: oneshot::Receiver<Halted>,
    ) -> JoinHandle<Launcher> {
        let launcher = Launcher::new(self.bind_to);
        tokio::spawn(async move {
            Udp::run_with_graceful_shutdown(tracker, launcher.bind_to, tx_start, rx_halt).await;
            launcher
        })
    }
}

#[derive(Default)]
struct ActiveRequests {
    rb: StaticRb<AbortHandle, 50>, // the number of requests we handle at the same time.
}

impl std::fmt::Debug for ActiveRequests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (left, right) = &self.rb.as_slices();
        let dbg = format!("capacity: {}, left: {left:?}, right: {right:?}", &self.rb.capacity());
        f.debug_struct("ActiveRequests").field("rb", &dbg).finish()
    }
}

impl Drop for ActiveRequests {
    fn drop(&mut self) {
        for h in self.rb.pop_iter() {
            if !h.is_finished() {
                h.abort();
            }
        }
    }
}

/// A UDP server instance launcher.
#[derive(Constructor)]
pub struct Udp;

impl Udp {
    /// It starts the UDP server instance with graceful shutdown.
    ///
    /// # Panics
    ///
    /// It panics if unable to bind to udp socket, and get the address from the udp socket.
    /// It also panics if unable to send address of socket.
    async fn run_with_graceful_shutdown(
        tracker: Arc<Tracker>,
        bind_to: SocketAddr,
        tx_start: oneshot::Sender<Started>,
        rx_halt: oneshot::Receiver<Halted>,
    ) {
        let socket = Arc::new(
            UdpSocket::bind(bind_to)
                .await
                .unwrap_or_else(|_| panic!("Could not bind to {bind_to}.")),
        );
        let address = socket
            .local_addr()
            .unwrap_or_else(|_| panic!("Could not get local_addr from {bind_to}."));
        let halt = shutdown_signal_with_message(rx_halt, format!("Halting Http Service Bound to Socket: {address}"));

        info!(target: "UDP TRACKER", "Starting on: udp://{}", address);

        let running = tokio::task::spawn(async move {
            debug!(target: "UDP TRACKER", "Started: Waiting for packets on socket address: udp://{address} ...");
            Self::run_udp_server(tracker, socket).await;
        });

        tx_start
            .send(Started { address })
            .expect("the UDP Tracker service should not be dropped");

        debug!(target: "UDP TRACKER", "Started on: udp://{}", address);

        let stop = running.abort_handle();

        select! {
            _ = running => { debug!(target: "UDP TRACKER", "Socket listener stopped on address: udp://{address}"); },
            () = halt => { debug!(target: "UDP TRACKER", "Halt signal spawned task stopped on address: udp://{address}"); }
        }
        stop.abort();

        task::yield_now().await; // lets allow the other threads to complete.
    }

    async fn run_udp_server(tracker: Arc<Tracker>, socket: Arc<UdpSocket>) {
        let tracker = tracker.clone();
        let socket = socket.clone();

        let reqs = &mut ActiveRequests::default();

        // Main Waiting Loop, awaits on async [`receive_request`].
        loop {
            if let Some(h) = reqs.rb.push_overwrite(
                Self::spawn_request_processor(Self::receive_request(socket.clone()).await, tracker.clone(), socket.clone())
                    .abort_handle(),
            ) {
                if !h.is_finished() {
                    // the task is still running, lets yield and give it a chance to flush.
                    tokio::task::yield_now().await;
                    h.abort();
                }
            }
        }
    }

    async fn receive_request(socket: Arc<UdpSocket>) -> Result<UdpRequest, Box<std::io::Error>> {
        // Wait for the socket to be readable
        socket.readable().await?;

        let mut buf = Vec::with_capacity(MAX_PACKET_SIZE);

        match socket.recv_buf_from(&mut buf).await {
            Ok((n, from)) => {
                Vec::truncate(&mut buf, n);
                trace!("GOT {buf:?}");
                Ok(UdpRequest { payload: buf, from })
            }

            Err(e) => Err(Box::new(e)),
        }
    }

    fn spawn_request_processor(
        result: Result<UdpRequest, Box<std::io::Error>>,
        tracker: Arc<Tracker>,
        socket: Arc<UdpSocket>,
    ) -> JoinHandle<()> {
        tokio::task::spawn(Self::process_request(result, tracker, socket))
    }

    async fn process_request(result: Result<UdpRequest, Box<std::io::Error>>, tracker: Arc<Tracker>, socket: Arc<UdpSocket>) {
        match result {
            Ok(udp_request) => {
                trace!("Received Request from: {}", udp_request.from);
                Self::process_valid_request(tracker.clone(), socket.clone(), udp_request).await;
            }
            Err(error) => {
                debug!("error: {error}");
            }
        }
    }

    async fn process_valid_request(tracker: Arc<Tracker>, socket: Arc<UdpSocket>, udp_request: UdpRequest) {
        trace!("Making Response to {udp_request:?}");
        let from = udp_request.from;
        let response = handlers::handle_packet(udp_request, &tracker.clone(), socket.clone()).await;
        Self::send_response(&socket.clone(), from, response).await;
    }

    async fn send_response(socket: &Arc<UdpSocket>, to: SocketAddr, response: Response) {
        trace!("Sending Response: {response:?} to: {to:?}");

        let buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(buffer);

        match response.write(&mut cursor) {
            Ok(()) => {
                #[allow(clippy::cast_possible_truncation)]
                let position = cursor.position() as usize;
                let inner = cursor.get_ref();

                debug!("Sending {} bytes ...", &inner[..position].len());
                debug!("To: {:?}", &to);
                debug!("Payload: {:?}", &inner[..position]);

                Self::send_packet(socket, &to, &inner[..position]).await;

                debug!("{} bytes sent", &inner[..position].len());
            }
            Err(_) => {
                error!("could not write response to bytes.");
            }
        }
    }

    async fn send_packet(socket: &Arc<UdpSocket>, remote_addr: &SocketAddr, payload: &[u8]) {
        trace!("Sending Packets: {payload:?} to: {remote_addr:?}");

        // doesn't matter if it reaches or not
        drop(socket.send_to(payload, remote_addr).await);
    }

    fn check(binding: &SocketAddr) -> ServiceHealthCheckJob {
        let binding = *binding;
        let info = format!("checking the udp tracker health check at: {binding}");

        let job = tokio::spawn(async move { check(&binding).await });

        ServiceHealthCheckJob::new(binding, info, job)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use tokio::time::sleep;
    use torrust_tracker_test_helpers::configuration::ephemeral_mode_public;

    use crate::bootstrap::app::initialize_with_configuration;
    use crate::servers::registar::Registar;
    use crate::servers::udp::server::{Launcher, UdpServer};

    #[tokio::test]
    async fn it_should_be_able_to_start_and_stop() {
        let cfg = Arc::new(ephemeral_mode_public());
        let tracker = initialize_with_configuration(&cfg);
        let config = &cfg.udp_trackers[0];

        let bind_to = config
            .bind_address
            .parse::<std::net::SocketAddr>()
            .expect("Tracker API bind_address invalid.");

        let register = &Registar::default();

        let stopped = UdpServer::new(Launcher::new(bind_to));
        let started = stopped
            .start(tracker, register.give_form())
            .await
            .expect("it should start the server");
        let stopped = started.stop().await.expect("it should stop the server");

        sleep(Duration::from_secs(1)).await;

        assert_eq!(stopped.state.launcher.bind_to, bind_to);
    }
}
