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
use std::future::Future;
use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;

use aquatic_udp_protocol::Response;
use futures::pin_mut;
use log::{debug, error, info};
use tokio::net::UdpSocket;
use tokio::task::JoinHandle;

use crate::servers::signals::shutdown_signal;
use crate::servers::udp::handlers::handle_packet;
use crate::servers::udp::MAX_PACKET_SIZE;
use crate::tracker::Tracker;

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
    /// The configuration of the server that will be used every time the server
    /// is started.    
    pub cfg: torrust_tracker_configuration::UdpTracker,
    /// The state of the server: `running` or `stopped`.
    pub state: S,
}

/// A stopped UDP server state.
pub struct Stopped;

/// A running UDP server state.
pub struct Running {
    /// The address where the server is bound.
    pub bind_address: SocketAddr,
    stop_job_sender: tokio::sync::oneshot::Sender<u8>,
    job: JoinHandle<()>,
}

impl UdpServer<Stopped> {
    /// Creates a new `UdpServer` instance in `stopped`state.
    #[must_use]
    pub fn new(cfg: torrust_tracker_configuration::UdpTracker) -> Self {
        Self { cfg, state: Stopped {} }
    }

    /// It starts the server and returns a `UdpServer` controller in `running`
    /// state.
    ///
    /// # Errors
    ///
    /// Will return `Err` if UDP can't bind to given bind address.
    pub async fn start(self, tracker: Arc<Tracker>) -> Result<UdpServer<Running>, Error> {
        let udp = Udp::new(&self.cfg.bind_address)
            .await
            .map_err(|e| Error::Error(e.to_string()))?;

        let bind_address = udp.socket.local_addr().map_err(|e| Error::Error(e.to_string()))?;

        let (sender, receiver) = tokio::sync::oneshot::channel::<u8>();

        let job = tokio::spawn(async move {
            udp.start_with_graceful_shutdown(tracker, shutdown_signal(receiver)).await;
        });

        let running_udp_server: UdpServer<Running> = UdpServer {
            cfg: self.cfg,
            state: Running {
                bind_address,
                stop_job_sender: sender,
                job,
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
    pub async fn stop(self) -> Result<UdpServer<Stopped>, Error> {
        self.state.stop_job_sender.send(1).map_err(|e| Error::Error(e.to_string()))?;

        drop(self.state.job.await);

        let stopped_api_server: UdpServer<Stopped> = UdpServer {
            cfg: self.cfg,
            state: Stopped {},
        };

        Ok(stopped_api_server)
    }
}

/// A UDP server instance launcher.
pub struct Udp {
    socket: Arc<UdpSocket>,
}

impl Udp {
    /// Creates a new `Udp` instance.
    ///
    /// # Errors
    ///
    /// Will return `Err` unable to bind to the supplied `bind_address`.
    pub async fn new(bind_address: &str) -> tokio::io::Result<Udp> {
        let socket = UdpSocket::bind(bind_address).await?;

        Ok(Udp {
            socket: Arc::new(socket),
        })
    }

    /// It starts the UDP server instance.
    ///
    /// # Panics
    ///
    /// It would panic if unable to resolve the `local_addr` from the supplied ´socket´.
    pub async fn start(&self, tracker: Arc<Tracker>) {
        loop {
            let mut data = [0; MAX_PACKET_SIZE];
            let socket = self.socket.clone();

            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    info!("Stopping UDP server: {}..", socket.local_addr().unwrap());
                    break;
                }
                Ok((valid_bytes, remote_addr)) = socket.recv_from(&mut data) => {
                    let payload = data[..valid_bytes].to_vec();

                    info!("Received {} bytes", payload.len());
                    debug!("From: {}", &remote_addr);
                    debug!("Payload: {:?}", payload);

                    let response = handle_packet(remote_addr, payload, &tracker).await;

                    Udp::send_response(socket, remote_addr, response).await;
                }
            }
        }
    }

    /// It starts the UDP server instance with graceful shutdown.
    ///
    /// # Panics
    ///
    /// It would panic if unable to resolve the `local_addr` from the supplied ´socket´.
    async fn start_with_graceful_shutdown<F>(&self, tracker: Arc<Tracker>, shutdown_signal: F)
    where
        F: Future<Output = ()>,
    {
        // Pin the future so that it doesn't move to the first loop iteration.
        pin_mut!(shutdown_signal);

        loop {
            let mut data = [0; MAX_PACKET_SIZE];
            let socket = self.socket.clone();

            tokio::select! {
                () = &mut shutdown_signal => {
                    info!("Stopping UDP server: {}..", self.socket.local_addr().unwrap());
                    break;
                }
                Ok((valid_bytes, remote_addr)) = socket.recv_from(&mut data) => {
                    let payload = data[..valid_bytes].to_vec();

                    info!("Received {} bytes", payload.len());
                    debug!("From: {}", &remote_addr);
                    debug!("Payload: {:?}", payload);

                    let response = handle_packet(remote_addr, payload, &tracker).await;

                    Udp::send_response(socket, remote_addr, response).await;
                }
            }
        }
    }

    async fn send_response(socket: Arc<UdpSocket>, remote_addr: SocketAddr, response: Response) {
        let buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(buffer);

        match response.write(&mut cursor) {
            Ok(()) => {
                #[allow(clippy::cast_possible_truncation)]
                let position = cursor.position() as usize;
                let inner = cursor.get_ref();

                info!("Sending {} bytes ...", &inner[..position].len());
                debug!("To: {:?}", &remote_addr);
                debug!("Payload: {:?}", &inner[..position]);

                Udp::send_packet(socket, &remote_addr, &inner[..position]).await;

                info!("{} bytes sent", &inner[..position].len());
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
}
