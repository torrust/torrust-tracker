//! Module to handle the UDP server instances.
//!
//! There are two main types in this module:
//!
//! - [`UdpServer`]: a controller to start and stop the server.
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
//!

use std::cell::RefCell;
use std::fmt::Debug;
use std::io::Cursor;
use std::net::SocketAddr;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use aquatic_udp_protocol::Response;
use derive_more::Constructor;
use futures::{Stream, StreamExt};
use ringbuf::traits::{Consumer, Observer, Producer};
use ringbuf::StaticRb;
use tokio::select;
use tokio::sync::oneshot;
use tokio::task::{AbortHandle, JoinHandle};
use url::Url;

use super::UdpRequest;
use crate::bootstrap::jobs::Started;
use crate::core::Tracker;
use crate::servers::logging::STARTED_ON;
use crate::servers::registar::{ServiceHealthCheckJob, ServiceRegistration, ServiceRegistrationForm};
use crate::servers::signals::{shutdown_signal_with_message, Halted};
use crate::servers::udp::{handlers, UDP_TRACKER_LOG_TARGET};
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
/// - The [`UdpServer`] cannot send the shutdown signal to the spawned UDP service thread.
#[derive(Debug)]
pub enum UdpError {
    /// Any kind of error starting or stopping the server.
    Socket(std::io::Error),
    Error(String),
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
/// > reset to the initial value after stopping the server. This struct is not
/// > intended to persist configurations between runs.
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
    pub async fn start(self, tracker: Arc<Tracker>, form: ServiceRegistrationForm) -> Result<UdpServer<Running>, std::io::Error> {
        let (tx_start, rx_start) = tokio::sync::oneshot::channel::<Started>();
        let (tx_halt, rx_halt) = tokio::sync::oneshot::channel::<Halted>();

        assert!(!tx_halt.is_closed(), "Halt channel for UDP tracker should be open");

        // May need to wrap in a task to about a tokio bug.
        let task = self.state.launcher.start(tracker, tx_start, rx_halt);

        let binding = rx_start.await.expect("it should be able to start the service").address;
        let local_addr = format!("udp://{binding}");

        form.send(ServiceRegistration::new(binding, Udp::check))
            .expect("it should be able to send service registration");

        let running_udp_server: UdpServer<Running> = UdpServer {
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
    pub async fn stop(self) -> Result<UdpServer<Stopped>, UdpError> {
        self.state
            .halt_task
            .send(Halted::Normal)
            .map_err(|e| UdpError::Error(e.to_string()))?;

        let launcher = self.state.task.await.expect("it should shutdown service");

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

/// Ring-Buffer of Active Requests
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

/// Wrapper for Tokio [`UdpSocket`][`tokio::net::UdpSocket`] that is bound to a particular socket.
struct BoundSocket {
    socket: Arc<tokio::net::UdpSocket>,
}

impl BoundSocket {
    async fn new(addr: SocketAddr) -> Result<Self, Box<std::io::Error>> {
        let bind_addr = format!("udp://{addr}");
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, bind_addr, "UdpSocket::new (binding)");

        let socket = tokio::net::UdpSocket::bind(addr).await;

        let socket = match socket {
            Ok(socket) => socket,
            Err(e) => Err(e)?,
        };

        let local_addr = format!("udp://{addr}");
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr, "UdpSocket::new (bound)");

        Ok(Self {
            socket: Arc::new(socket),
        })
    }

    fn local_addr(&self) -> SocketAddr {
        self.socket.local_addr().expect("it should get local address")
    }

    fn url(&self) -> Url {
        Url::parse(&format!("udp://{}", self.local_addr())).expect("UDP socket address should be valid")
    }
}

impl Deref for BoundSocket {
    type Target = tokio::net::UdpSocket;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

impl Debug for BoundSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let local_addr = match self.socket.local_addr() {
            Ok(socket) => format!("Receiving From: {socket}"),
            Err(err) => format!("Socket Broken: {err}"),
        };

        f.debug_struct("UdpSocket").field("addr", &local_addr).finish_non_exhaustive()
    }
}

struct Receiver {
    bound_socket: Arc<BoundSocket>,
    tracker: Arc<Tracker>,
    data: RefCell<[u8; MAX_PACKET_SIZE]>,
}

impl Receiver {
    pub fn new(bound_socket: Arc<BoundSocket>, tracker: Arc<Tracker>) -> Self {
        Receiver {
            bound_socket,
            tracker,
            data: RefCell::new([0; MAX_PACKET_SIZE]),
        }
    }
}

impl Stream for Receiver {
    type Item = std::io::Result<AbortHandle>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = *self.data.borrow_mut();
        let mut buf = tokio::io::ReadBuf::new(&mut buf);

        let Poll::Ready(ready) = self.bound_socket.poll_recv_from(cx, &mut buf) else {
            return Poll::Pending;
        };

        let res = match ready {
            Ok(from) => {
                let payload = buf.filled().to_vec();
                let request = UdpRequest { payload, from };

                Some(Ok(tokio::task::spawn(Udp::process_request(
                    request,
                    self.tracker.clone(),
                    self.bound_socket.clone(),
                ))
                .abort_handle()))
            }
            Err(err) => Some(Err(err)),
        };

        Poll::Ready(res)
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
        let halt_task = tokio::task::spawn(shutdown_signal_with_message(
            rx_halt,
            format!("Halting UDP Service Bound to Socket: {bind_to}"),
        ));

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Starting on: {bind_to}");

        let socket = tokio::time::timeout(Duration::from_millis(5000), BoundSocket::new(bind_to))
            .await
            .expect("it should bind to the socket within five seconds");

        let bound_socket = match socket {
            Ok(socket) => socket,
            Err(e) => {
                tracing::error!(target: UDP_TRACKER_LOG_TARGET, addr = %bind_to, err = %e, "Udp::run_with_graceful_shutdown panic! (error when building socket)" );
                panic!("could not bind to socket!");
            }
        };

        let address = bound_socket.local_addr();
        let local_udp_url = bound_socket.url().to_string();

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "{STARTED_ON}: {local_udp_url}");

        let receiver = Receiver::new(bound_socket.into(), tracker);

        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (spawning main loop)");

        let running = {
            let local_addr = local_udp_url.clone();
            tokio::task::spawn(async move {
                tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_with_graceful_shutdown::task (listening...)");
                let () = Self::run_udp_server_main(receiver).await;
            })
        };

        tx_start
            .send(Started { address })
            .expect("the UDP Tracker service should not be dropped");

        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (started)");

        let stop = running.abort_handle();

        select! {
            _ = running => { tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (stopped)"); },
            _ = halt_task => { tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (halting)"); }
        }
        stop.abort();

        tokio::task::yield_now().await; // lets allow the other threads to complete.
    }

    async fn run_udp_server_main(mut receiver: Receiver) {
        let reqs = &mut ActiveRequests::default();

        let addr = receiver.bound_socket.local_addr();
        let local_addr = format!("udp://{addr}");

        loop {
            if let Some(req) = {
                tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server (wait for request)");
                receiver.next().await
            } {
                tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server::loop (in)");

                let req = match req {
                    Ok(req) => req,
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::Interrupted {
                            tracing::warn!(target: UDP_TRACKER_LOG_TARGET, local_addr, err = %e,  "Udp::run_udp_server::loop (interrupted)");
                            return;
                        }
                        tracing::error!(target: UDP_TRACKER_LOG_TARGET, local_addr, err = %e,  "Udp::run_udp_server::loop break: (got error)");
                        break;
                    }
                };

                if req.is_finished() {
                    continue;
                }

                // fill buffer with requests
                let Err(req) = reqs.rb.try_push(req) else {
                    continue;
                };

                let mut finished: u64 = 0;
                let mut unfinished_task = None;
                // buffer is full.. lets make some space.
                for h in reqs.rb.pop_iter() {
                    // remove some finished tasks
                    if h.is_finished() {
                        finished += 1;
                        continue;
                    }

                    // task is unfinished.. give it another chance.
                    tokio::task::yield_now().await;

                    // if now finished, we continue.
                    if h.is_finished() {
                        finished += 1;
                        continue;
                    }

                    tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr, removed_count = finished, "Udp::run_udp_server::loop (got unfinished task)");

                    if finished == 0 {
                        // we have _no_ finished tasks.. will abort the unfinished task to make space...
                        h.abort();

                        tracing::warn!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server::loop aborting request: (no finished tasks)");
                        break;
                    }

                    // we have space, return unfinished task for re-entry.
                    unfinished_task = Some(h);
                }

                // re-insert the previous unfinished task.
                if let Some(h) = unfinished_task {
                    reqs.rb.try_push(h).expect("it was previously inserted");
                }

                // insert the new task.
                if !req.is_finished() {
                    reqs.rb.try_push(req).expect("it should remove at least one element.");
                }
            } else {
                tokio::task::yield_now().await;
                // the request iterator returned `None`.
                tracing::error!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server breaking: (ran dry, should not happen in production!)");
                break;
            }
        }
    }

    async fn process_request(request: UdpRequest, tracker: Arc<Tracker>, socket: Arc<BoundSocket>) {
        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, request = %request.from, "Udp::process_request (receiving)");
        Self::process_valid_request(tracker, socket, request).await;
    }

    async fn process_valid_request(tracker: Arc<Tracker>, socket: Arc<BoundSocket>, udp_request: UdpRequest) {
        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, "Udp::process_valid_request. Making Response to {udp_request:?}");
        let from = udp_request.from;
        let response = handlers::handle_packet(udp_request, &tracker.clone(), socket.local_addr()).await;
        Self::send_response(&socket.clone(), from, response).await;
    }

    async fn send_response(bound_socket: &Arc<BoundSocket>, to: SocketAddr, response: Response) {
        let response_type = match &response {
            Response::Connect(_) => "Connect".to_string(),
            Response::AnnounceIpv4(_) => "AnnounceIpv4".to_string(),
            Response::AnnounceIpv6(_) => "AnnounceIpv6".to_string(),
            Response::Scrape(_) => "Scrape".to_string(),
            Response::Error(e) => format!("Error: {e:?}"),
        };

        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, target = ?to, response_type,  "Udp::send_response (sending)");

        let buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(buffer);

        match response.write_bytes(&mut cursor) {
            Ok(()) => {
                #[allow(clippy::cast_possible_truncation)]
                let position = cursor.position() as usize;
                let inner = cursor.get_ref();

                tracing::debug!(target: UDP_TRACKER_LOG_TARGET, ?to, bytes_count = &inner[..position].len(), "Udp::send_response (sending...)" );
                tracing::trace!(target: UDP_TRACKER_LOG_TARGET, ?to, bytes_count = &inner[..position].len(), payload = ?&inner[..position], "Udp::send_response (sending...)");

                Self::send_packet(bound_socket, &to, &inner[..position]).await;

                tracing::trace!(target:UDP_TRACKER_LOG_TARGET, ?to, bytes_count = &inner[..position].len(), "Udp::send_response (sent)");
            }
            Err(e) => {
                tracing::error!(target: UDP_TRACKER_LOG_TARGET, ?to, response_type, err = %e, "Udp::send_response (error)");
            }
        }
    }

    async fn send_packet(socket: &Arc<BoundSocket>, remote_addr: &SocketAddr, payload: &[u8]) {
        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, to = %remote_addr, ?payload, "Udp::send_response (sending)");

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

    use torrust_tracker_test_helpers::configuration::ephemeral_mode_public;

    use crate::bootstrap::app::initialize_with_configuration;
    use crate::servers::registar::Registar;
    use crate::servers::udp::server::{Launcher, UdpServer};

    #[tokio::test]
    async fn it_should_be_able_to_start_and_stop() {
        let cfg = Arc::new(ephemeral_mode_public());
        let tracker = initialize_with_configuration(&cfg);
        let udp_trackers = cfg.udp_trackers.clone().expect("missing UDP trackers configuration");
        let config = &udp_trackers[0];
        let bind_to = config.bind_address;
        let register = &Registar::default();

        let stopped = UdpServer::new(Launcher::new(bind_to));

        let started = stopped
            .start(tracker, register.give_form())
            .await
            .expect("it should start the server");

        let stopped = started.stop().await.expect("it should stop the server");

        tokio::time::sleep(Duration::from_secs(1)).await;

        assert_eq!(stopped.state.launcher.bind_to, bind_to);
    }

    #[tokio::test]
    async fn it_should_be_able_to_start_and_stop_with_wait() {
        let cfg = Arc::new(ephemeral_mode_public());
        let tracker = initialize_with_configuration(&cfg);
        let config = &cfg.udp_trackers.as_ref().unwrap().first().unwrap();
        let bind_to = config.bind_address;
        let register = &Registar::default();

        let stopped = UdpServer::new(Launcher::new(bind_to));

        let started = stopped
            .start(tracker, register.give_form())
            .await
            .expect("it should start the server");

        tokio::time::sleep(Duration::from_secs(1)).await;

        let stopped = started.stop().await.expect("it should stop the server");

        tokio::time::sleep(Duration::from_secs(1)).await;

        assert_eq!(stopped.state.launcher.bind_to, bind_to);
    }
}

/// Todo: submit test to tokio documentation.
#[cfg(test)]
mod test_tokio {
    use std::sync::Arc;
    use std::time::Duration;

    use tokio::sync::Barrier;
    use tokio::task::JoinSet;

    #[tokio::test]
    async fn test_barrier_with_aborted_tasks() {
        // Create a barrier that requires 10 tasks to proceed.
        let barrier = Arc::new(Barrier::new(10));
        let mut tasks = JoinSet::default();
        let mut handles = Vec::default();

        // Set Barrier to 9/10.
        for _ in 0..9 {
            let c = barrier.clone();
            handles.push(tasks.spawn(async move {
                c.wait().await;
            }));
        }

        // Abort two tasks: Barrier: 7/10.
        for _ in 0..2 {
            if let Some(handle) = handles.pop() {
                handle.abort();
            }
        }

        // Spawn a single task: Barrier 8/10.
        let c = barrier.clone();
        handles.push(tasks.spawn(async move {
            c.wait().await;
        }));

        // give a chance fro the barrier to release.
        tokio::time::sleep(Duration::from_millis(50)).await;

        // assert that the barrier isn't removed, i.e. 8, not 10.
        for h in &handles {
            assert!(!h.is_finished());
        }

        // Spawn two more tasks to trigger the barrier release: Barrier 10/10.
        for _ in 0..2 {
            let c = barrier.clone();
            handles.push(tasks.spawn(async move {
                c.wait().await;
            }));
        }

        // give a chance fro the barrier to release.
        tokio::time::sleep(Duration::from_millis(50)).await;

        // assert that the barrier has been triggered
        for h in &handles {
            assert!(h.is_finished());
        }

        tasks.shutdown().await;
    }
}
