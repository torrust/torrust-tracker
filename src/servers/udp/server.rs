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
use std::time::Duration;

use aquatic_udp_protocol::Response;
use derive_more::Constructor;
use futures::pin_mut;
use log::{debug, error, info};
use tokio::net::UdpSocket;
use tokio::sync::oneshot::{Receiver, Sender};
use tokio::task::JoinHandle;

use crate::bootstrap::jobs::Started;
use crate::core::Tracker;
use crate::servers::registar::{ServiceHealthCheckJob, ServiceRegistration, ServiceRegistrationForm};
use crate::servers::signals::{shutdown_signal_with_message, Halted};
use crate::servers::udp::handlers::handle_packet;
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

        let launcher = self.state.launcher;

        let task = tokio::spawn(async move {
            debug!(target: "UDP Tracker", "Launcher starting ...");

            let starting = launcher.start(tracker, tx_start, rx_halt).await;

            starting.await.expect("UDP server should have started running");

            launcher
        });

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

#[derive(Constructor, Debug)]
pub struct Launcher {
    bind_to: SocketAddr,
}

impl Launcher {
    /// It starts the UDP server instance.
    ///
    /// # Panics
    ///
    /// It would panic if unable to resolve the `local_addr` from the supplied ´socket´.
    pub async fn start(&self, tracker: Arc<Tracker>, tx_start: Sender<Started>, rx_halt: Receiver<Halted>) -> JoinHandle<()> {
        Udp::start_with_graceful_shutdown(tracker, self.bind_to, tx_start, rx_halt).await
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
    async fn start_with_graceful_shutdown(
        tracker: Arc<Tracker>,
        bind_to: SocketAddr,
        tx_start: Sender<Started>,
        rx_halt: Receiver<Halted>,
    ) -> JoinHandle<()> {
        let socket = Arc::new(UdpSocket::bind(bind_to).await.expect("Could not bind to {self.socket}."));
        let address = socket.local_addr().expect("Could not get local_addr from {binding}.");

        info!(target: "UDP Tracker", "Starting on: udp://{}", address);

        let running = tokio::task::spawn(async move {
            let halt = tokio::task::spawn(async move {
                debug!(target: "UDP Tracker", "Waiting for halt signal for socket address: udp://{address}  ...");

                shutdown_signal_with_message(
                    rx_halt,
                    format!("Shutting down UDP server on socket address: udp://{address}"),
                )
                .await;
            });

            let listen = async move {
                debug!(target: "UDP Tracker", "Waiting for packets on socket address: udp://{address} ...");

                loop {
                    let mut data = [0; MAX_PACKET_SIZE];
                    let socket_clone = socket.clone();

                    match socket_clone.recv_from(&mut data).await {
                        Ok((valid_bytes, remote_addr)) => {
                            let payload = data[..valid_bytes].to_vec();

                            debug!(target: "UDP Tracker", "Received {} bytes", payload.len());
                            debug!(target: "UDP Tracker", "From: {}", &remote_addr);
                            debug!(target: "UDP Tracker", "Payload: {:?}", payload);

                            let response_fut = handle_packet(remote_addr, payload, &tracker);

                            match tokio::time::timeout(Duration::from_secs(5), response_fut).await {
                                Ok(response) => {
                                    Udp::send_response(socket_clone, remote_addr, response).await;
                                }
                                Err(_) => {
                                    error!("Timeout occurred while processing the UDP request.");
                                }
                            }
                        }
                        Err(err) => {
                            error!("Error reading UDP datagram from socket. Error: {:?}", err);
                        }
                    }
                }
            };

            pin_mut!(halt);
            pin_mut!(listen);

            tx_start
                .send(Started { address })
                .expect("the UDP Tracker service should not be dropped");

            tokio::select! {
                _ = & mut halt => { debug!(target: "UDP Tracker", "Halt signal spawned task stopped on address: udp://{address}"); },
                () = & mut listen => { debug!(target: "UDP Tracker", "Socket listener stopped on address: udp://{address}"); },
            }
        });

        info!(target: "UDP Tracker", "Started on: udp://{}", address);

        running
    }

    async fn send_response(socket: Arc<UdpSocket>, remote_addr: SocketAddr, response: Response) {
        let buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(buffer);

        match response.write(&mut cursor) {
            Ok(()) => {
                #[allow(clippy::cast_possible_truncation)]
                let position = cursor.position() as usize;
                let inner = cursor.get_ref();

                debug!("Sending {} bytes ...", &inner[..position].len());
                debug!("To: {:?}", &remote_addr);
                debug!("Payload: {:?}", &inner[..position]);

                Udp::send_packet(socket, &remote_addr, &inner[..position]).await;

                debug!("{} bytes sent", &inner[..position].len());
            }
            Err(_) => {
                error!("could not write response to bytes.");
            }
        }
    }

    async fn send_packet(socket: Arc<UdpSocket>, remote_addr: &SocketAddr, payload: &[u8]) {
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

        assert_eq!(stopped.state.launcher.bind_to, bind_to);
    }
}
