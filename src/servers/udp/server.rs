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
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use derive_more::{Constructor, Display};
use futures::future::BoxFuture;
use futures::{FutureExt, TryFutureExt as _};
use ringbuf::{Rb, StaticRb};
use thiserror::Error;
use tokio::net::UdpSocket;
use tokio::select;
use tokio::task::{AbortHandle, JoinHandle};
use torrust_tracker_configuration::{CLIENT_TIMEOUT_DEFAULT, MAX_PACKET_SIZE};
use tracing::{debug, error, info, instrument, trace, Level};

use super::handle::Watcher;
use super::UdpRequest;
use crate::core::Tracker;
use crate::servers::registar::{FnSpawnServiceHeathCheck, ServiceHealthCheckJob};
use crate::servers::service::{AddrFuture, Error, Handle, Launcher, TaskFuture};
use crate::servers::signals::{shutdown_signal_with_message, Halted};
use crate::servers::udp::handlers;
use crate::shared::bit_torrent::tracker::udp::Client;

#[derive(Error, Debug, Clone)]
pub enum UdpError {
    #[error("Gracefully Canceled Processing Requests For: {addr}")]
    GracefullyCanceled { addr: SocketAddr },
    #[error("Canceled Sending Response to Target: {target}")]
    CanceledSending { target: SocketAddr },

    #[error("Failed to get Local Address from Socket: {err:?}")]
    UnableToGetLocalAddress { err: Arc<std::io::Error> },

    #[error("Failed to get Listening Address from Socket")]
    UnableToGetListeningAddress {},
    #[error("Socket Errored when waiting to read from: {addr}, with error: {err:?}")]
    UnableToGetReadableSocket { addr: SocketAddr, err: Arc<std::io::Error> },
    #[error("Socket Errored when reading from: {addr}, with error: {err:?}")]
    UnableToReadFromSocket { addr: SocketAddr, err: Arc<std::io::Error> },
    #[error("Socket Errored when sending to: {target}, with error: {err:?}")]
    UnableToSendToSocket { target: SocketAddr, err: Arc<std::io::Error> },
    #[error("Socket Errored when writing to: {target}, with error: {err:?}")]
    UnableToWriteToSocket { target: SocketAddr, err: Arc<std::io::Error> },
}

fn check_fn(&addr: &SocketAddr) -> ServiceHealthCheckJob {
    let info = format!("checking the udp tracker health check at: {addr}");

    let job = tokio::spawn(
        Client::connect(addr, CLIENT_TIMEOUT_DEFAULT)
            .and_then(Client::check)
            .map_err(|e| e.to_string()),
    );

    ServiceHealthCheckJob::new(addr, info, job)
}

#[derive(Debug)]
pub struct UdpHandle {
    tx_shutdown: Option<tokio::sync::oneshot::Sender<Halted>>,
    pub udp_handle: super::handle::Handle,
}

impl UdpHandle {
    #[instrument(err, ret)]
    fn shutdown(&mut self) -> Result<(), Error> {
        if let Some(tx) = self.tx_shutdown.take() {
            tx.send(Halted::Normal)
                .map_err(|err| Error::UnableToSendHaltingMessage { err })?;
        } else {
            panic!("it has already taken the channel?");
        };
        Ok(())
    }
}

impl Default for UdpHandle {
    fn default() -> Self {
        let (tx_shutdown, rx_shutdown) = tokio::sync::oneshot::channel::<Halted>();

        let udp_handle = super::handle::Handle::default();

        let () = Udp::graceful_udp_shutdown(udp_handle.clone(), rx_shutdown, "UDP service".to_string());

        Self {
            udp_handle: super::handle::Handle::new(),
            tx_shutdown: Some(tx_shutdown),
        }
    }
}

impl Handle for UdpHandle {
    fn stop(mut self) -> Result<(), Error> {
        self.shutdown()
    }

    fn listening(&self) -> AddrFuture<'_> {
        self.udp_handle.listening().boxed()
    }
}

impl Drop for UdpHandle {
    fn drop(&mut self) {
        self.shutdown().expect("it should shutdown when dropped");
    }
}

#[derive(Constructor, Clone, Debug, Display)]
#[display(fmt = "intended_address: {addr}, with tracker")]
pub struct UdpLauncher {
    pub tracker: Arc<Tracker>,
    pub addr: SocketAddr,
}

impl Launcher<UdpHandle> for UdpLauncher {
    #[instrument(err)]
    fn start(self) -> Result<(TaskFuture<'static, (), Error>, UdpHandle, FnSpawnServiceHeathCheck), Error> {
        let handle = UdpHandle::default();

        let std_socket = std::net::UdpSocket::bind(self.addr).map_err(|e| Error::UnableToBindToSocket {
            addr: self.addr,
            err: e.into(),
        })?;

        let socket = UdpSocket::from_std(std_socket).map_err(|e| Error::UnableToBindToSocket {
            addr: self.addr,
            err: e.into(),
        })?;

        let task = Udp::run_udp_server(self.tracker, socket, handle.udp_handle.clone())
            .map_err(|err| Error::UnableToStartUdpService { err })?
            .map_err(|err| Error::UnableToStartUdpService { err })
            .boxed();

        Ok((task, handle, check_fn))
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
pub(super) struct Udp {}

impl Udp {
    #[instrument(err)]
    pub fn run_udp_server(
        tracker: Arc<Tracker>,
        socket: UdpSocket,
        handle: super::Handle,
    ) -> Result<BoxFuture<'static, Result<(), UdpError>>, UdpError> {
        let socket = Arc::new(socket);

        let addr = socket
            .local_addr()
            .map_err(|e| UdpError::UnableToGetLocalAddress { err: e.into() })?;

        let () = handle.notify_listening(Some(addr));

        Ok(async move {
            let reqs = &mut ActiveRequests::default();

            // Main Waiting Loop, awaits on async [`receive_request`].
            loop {
                let Ok(request) = Self::receive_request(socket.clone(), handle.clone()).await else {
                    break;
                };

                if let Some(h) = reqs
                    .rb
                    .push_overwrite(Self::spawn_request_processor(request, tracker.clone(), socket.clone()).abort_handle())
                {
                    if !h.is_finished() {
                        // the task is still running, lets yield and give it a chance to flush.
                        let () = tokio::task::yield_now().await;
                        h.abort();
                    }
                }
            }

            let () = tokio::task::yield_now().await;

            let () = handle.wait_connections_end().await;

            Ok(())
        }
        .boxed())
    }

    #[instrument(err, ret)]
    async fn receive_request(socket: Arc<UdpSocket>, handle: super::Handle) -> Result<UdpRequest, UdpError> {
        let addr = handle
            .listening()
            .await
            .ok_or_else(|| UdpError::UnableToGetListeningAddress {})?;

        let () = select! {
            () = handle.wait_graceful_shutdown() => Err(UdpError::GracefullyCanceled { addr }),
            r = socket.readable() => r.map_err(|e| UdpError::UnableToGetReadableSocket { addr ,err: e.into() }),
        }?;

        let mut buf = Vec::with_capacity(MAX_PACKET_SIZE);

        let (n, from) = socket
            .recv_buf_from(&mut buf)
            .await
            .map_err(|e| UdpError::UnableToReadFromSocket { addr, err: e.into() })?;

        Vec::truncate(&mut buf, n);
        trace!("GOT {buf:?}");
        Ok(UdpRequest {
            payload: buf,
            from,
            watcher: handle.watcher(),
        })
    }

    #[instrument(level = Level::TRACE,)]
    fn spawn_request_processor(request: UdpRequest, tracker: Arc<Tracker>, socket: Arc<UdpSocket>) -> JoinHandle<()> {
        tokio::task::spawn(Self::process_request(request, tracker, socket))
    }

    #[instrument(level = Level::TRACE,)]
    async fn process_request<'a>(request: UdpRequest, tracker: Arc<Tracker>, socket: Arc<UdpSocket>) {
        select! {
            _ = Self::process_valid_request(tracker.clone(), socket.clone(), &request) => (),
            () = request.watcher.wait_shutdown() => ()
        };
    }

    #[instrument(err, ret)]
    async fn process_valid_request(tracker: Arc<Tracker>, socket: Arc<UdpSocket>, request: &UdpRequest) -> Result<(), UdpError> {
        trace!("Making Response to {request:?}");
        let target = request.from;
        let response = handlers::handle_packet(request, &tracker.clone(), socket.clone()).await;
        let _ = Self::send_response(&socket.clone(), target, response, &request.watcher).await?;

        Ok(())
    }

    #[instrument(err, ret)]
    async fn send_response(
        socket: &Arc<UdpSocket>,
        target: SocketAddr,
        response: aquatic_udp_protocol::Response,
        watcher: &Watcher,
    ) -> Result<usize, UdpError> {
        trace!("Sending Response: {response:?} to: {target:?}");

        let buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = std::io::Cursor::new(buffer);

        let () = response
            .write(&mut cursor)
            .map_err(|e| UdpError::UnableToWriteToSocket { target, err: e.into() })?;

        #[allow(clippy::cast_possible_truncation)]
        let position = cursor.position() as usize;
        let inner = cursor.get_ref();

        debug!("Sending {} bytes ...", &inner[..position].len());
        debug!("To: {:?}", &target);
        debug!("Payload: {:?}", &inner[..position]);

        let result = Self::send_packet(socket, &target, &inner[..position], watcher).await;

        debug!("{} bytes sent", &inner[..position].len());

        result
    }

    #[instrument(err, ret)]
    async fn send_packet(
        socket: &Arc<UdpSocket>,
        &target: &SocketAddr,
        payload: &[u8],
        watcher: &Watcher,
    ) -> Result<usize, UdpError> {
        trace!("Sending Packets: {payload:?} to: {target:?}");

        select! {
            r  = socket.send_to(payload, target) => r.map_err(|e| UdpError::UnableToSendToSocket{ target, err: e.into()}),
            () = watcher.wait_shutdown() => Err(UdpError::CanceledSending{target})
        }
    }

    #[instrument(ret)]
    pub fn graceful_udp_shutdown(handle: super::Handle, rx_shutdown: tokio::sync::oneshot::Receiver<Halted>, message: String) {
        tokio::task::spawn(async move {
            match handle.listening().await {
                Some(addr) => {
                    shutdown_signal_with_message(rx_shutdown, format!("{message}, on socket address: {addr}")).await;

                    info!("Sending graceful shutdown signal");
                    handle.graceful_shutdown(Some(Duration::from_secs(90)));

                    println!("!! shuting down in 90 seconds !!");

                    loop {
                        tokio::time::sleep(Duration::from_secs(1)).await;

                        info!("remaining alive connections: {}", handle.connection_count());
                    }
                }
                None => handle.shutdown(),
            }
        });
    }
}
