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

use futures::future::BoxFuture;
use futures::FutureExt;
use ringbuf::traits::{Consumer as _, Observer as _, RingBuffer as _};
use ringbuf::StaticRb;
use tokio::net::UdpSocket;
use tokio::select;
use tokio::task::AbortHandle;
use torrust_tracker_configuration::MAX_PACKET_SIZE;
use tracing::{debug, error, info, instrument, trace, warn, Level};

use super::v0::UdpRequest;
use super::Error;
use crate::core::Tracker;
use crate::servers::udp::v0::handlers;
use crate::shared::handle::{Handle, Watcher};

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
pub(super) struct Udp {
    pub handle: Handle,
    tracker: Arc<Tracker>,
    socket: Arc<UdpSocket>,
    addr: SocketAddr,
    reqs: ActiveRequests,
}

enum Task {
    Shutdown,
    Graceful,
    Listening(Option<SocketAddr>),
    Readable(std::io::Result<()>),
}

impl Udp {
    fn new(tracker: &Arc<Tracker>, socket: &Arc<UdpSocket>, handle: &Handle) -> Result<Self, Error> {
        let addr = socket
            .local_addr()
            .map_err(|e| Error::UnableToGetLocalAddress { err: e.into() })?;

        Ok(Self {
            tracker: tracker.clone(),
            socket: socket.clone(),
            handle: handle.clone(),
            reqs: ActiveRequests::default(),
            addr,
        })
    }

    async fn run(self) -> Result<(), Error> {
        {
            let listen_job = tokio::task::spawn(Self::wait_listening(self.handle.clone()));

            {
                let notify = match Self::notify_listening(&self.handle, &self.socket) {
                    Ok(addr) => addr,
                    Err(e) => {
                        error!("failed to get active socket, aborting...");
                        self.handle.shutdown();
                        return Err(e);
                    }
                };

                let listening = listen_job.await??;

                assert_eq!(notify, listening, "it should never get a different socket");
            }
        };

        let () = tokio::task::spawn_local(self.main_loop()).await??;

        Ok(())
    }

    fn main_loop<'a>(mut self) -> BoxFuture<'a, Result<(), Error>> {
        async move {
            let mut task: Task;
            loop {
                let tracker = self.tracker.clone();
                let socket = self.socket.clone();

                trace!("await readable socket");
                {
                    task = select! {
                        biased;
                        () = self.handle.wait_shutdown() => Task::Shutdown,
                        r = socket.readable() => Task::Readable(r),
                        () = self.handle.wait_graceful_shutdown() => Task::Graceful,
                    };
                }

                trace!("brake if interrupted");
                {
                    let () = match task {
                        Task::Shutdown | Task::Graceful => {
                            debug!("(shutting down): stop processing new requests");
                            break;
                        }
                        Task::Readable(r) => r.map_err(|e| Error::UnableToGetReadableSocket {
                            addr: self.addr,
                            err: e.into(),
                        })?,
                        Task::Listening(_) => unreachable!(),
                    };
                }

                trace!("read socket and fulfil request");
                {
                    trace!("receive request");
                    let request = self.receive_request().await?;

                    trace!("create new job to respond to {}", request);
                    let respond_job =
                        tokio::task::spawn(fulfil_request(tracker, socket, self.addr, request, self.handle.watcher()));

                    trace!("limit number of active jobs");
                    if let Some(h) = self.reqs.rb.push_overwrite(respond_job.abort_handle()) {
                        if !h.is_finished() {
                            debug!("job was still active and will be aborted");
                            let () = tokio::task::yield_now().await; // yield to give it a chance...
                            h.abort();
                        }
                    }
                }
            }
            let () = match task {
                Task::Shutdown => {
                    warn!("main loop interrupted!");
                }
                Task::Graceful => {
                    info!("main_loop finished, now wait for all connections to complete..");

                    let () = tokio::task::yield_now().await;

                    let () = self.handle.wait_connections_end().await;
                }
                Task::Readable(_) => unreachable!("it should not return readable"),

                Task::Listening(_) => unreachable!(),
            };

            Ok(())
        }
        .boxed()
    }

    fn notify_listening(handle: &Handle, socket: &UdpSocket) -> Result<SocketAddr, Error> {
        trace!("try to get the local socket address to notify listeners");

        match socket.local_addr() {
            Ok(addr) => {
                info!("the socket at: {addr} is ready");
                handle.notify_listening(Some(addr));
                Ok(addr)
            }
            Err(e) => {
                error!("the socket is not available: {e}");
                handle.notify_listening(None);
                Err(Error::UnableToGetLocalAddress { err: e.into() })
            }
        }
    }

    #[instrument(skip(handle))]
    async fn wait_listening(handle: Handle) -> Result<SocketAddr, Error> {
        let wait = tokio::select! {
            maybe = handle.listening() => {Task::Listening(maybe)}
            () = handle.wait_shutdown() => { Task::Shutdown}
            () = handle.wait_graceful_shutdown() => { Task::Graceful}
        };

        match wait {
            Task::Shutdown | Task::Graceful => {
                warn!("shutdown called before listening");
                Err(Error::StopBeforeStarting {})
            }
            Task::Listening(maybe) => {
                if let Some(addr) = maybe {
                    debug!("socket ready and listening to: {addr}");
                    Ok(addr)
                } else {
                    error!("failed to open socket!");
                    Err(Error::UnableToGetListeningAddress {})
                }
            }
            Task::Readable(_) => unreachable!(),
        }
    }

    #[instrument(err)]
    pub fn make_task(
        tracker: Arc<Tracker>,
        socket: Arc<UdpSocket>,
    ) -> Result<(BoxFuture<'static, Result<(), Error>>, super::handle::Handle), Error> {
        let handle = Handle::default();
        let server = Udp::new(&tracker, &socket, &handle)?;
        let udp_handle = super::handle::Handle::new(handle);

        let task = server.run().boxed();

        Ok((task, udp_handle))
    }

    #[instrument(skip(self))]
    async fn receive_request(&self) -> Result<UdpRequest, Error> {
        let mut payload = Vec::with_capacity(MAX_PACKET_SIZE);

        let (len, from) = self
            .socket
            .recv_buf_from(&mut payload)
            .await
            .map_err(|e| Error::UnableToReadFromSocket {
                addr: self.addr,
                err: e.into(),
            })?;

        Vec::truncate(&mut payload, len);
        trace!("GOT {payload:?}");

        Ok(UdpRequest { payload, from })
    }
}

#[instrument(, level = Level::TRACE)]
async fn fulfil_request(tracker: Arc<Tracker>, socket: Arc<UdpSocket>, addr: SocketAddr, request: UdpRequest, watcher: Watcher) {
    let res_type = |response: &aquatic_udp_protocol::Response| -> Result<String, String> {
        use aquatic_udp_protocol::Response;

        match response {
            Response::Connect(_) => Ok("connect".to_string()),
            Response::AnnounceIpv4(_) => Ok("announce_ipv4".to_string()),
            Response::AnnounceIpv6(_) => Ok("announce_ipv6".to_string()),
            Response::Scrape(_) => Ok("scrape".to_string()),
            Response::Error(e) => Err(format!("error: {}, id: {:?}", e.message, e.transaction_id)),
        }
    };
    let target = request.from;

    trace!("handling request into a response");
    let response = select! {
        r = handlers::handle_packet(&tracker, addr, &request  ) => r,
        () = watcher.wait_shutdown() => return
    };

    let payload = get_response_payload(&response);
    trace!("Got Response Payload: {payload:?}");

    debug!("Sending {} bytes ... To: {:?}", payload.len(), &target);
    let success = select! {
        r = socket.send_to(&payload, target) => r,
        () = watcher.wait_shutdown() => return
    };

    // lets trace the success value...
    match success {
        Ok(bytes) => match res_type(&response) {
            Ok(r) => debug!("fulfilled an {r} request for {target} by sending {bytes} response"),

            Err(e) => debug!("sent {e} response to {target} by sending {bytes}"),
        },
        Err(e) => {
            error!(
                "failed to send a {:?} response of {} bytes to {target}, with error {e}",
                res_type(&response),
                payload.len()
            );
            debug!("unfulfilled response: {response:?}");
        }
    }
}

#[instrument]
fn get_response_payload(response: &aquatic_udp_protocol::Response) -> Vec<u8> {
    let buffer = vec![0u8; MAX_PACKET_SIZE];
    let mut cursor = std::io::Cursor::new(buffer);

    let () = response
        .write_bytes(&mut cursor)
        .expect("it should be able to write to buffer");

    #[allow(clippy::cast_possible_truncation)]
    let len = cursor.position() as usize;
    let mut payload = cursor.into_inner();
    payload.truncate(len);

    payload
}

//         match handle.listening().await {
//             Some(addr) => {

//                 handle.wait_graceful_shutdown()

//                 shutdown_signal_with_message(rx_shutdown, format!("{message}, on socket address: {addr}")).await;

//                 info!("Sending graceful shutdown signal");
//                 handle.graceful_shutdown(Some(Duration::from_secs(90)));

//                 println!("!! shuting down in 90 seconds !!");

//                 loop {
//                     tokio::time::sleep(Duration::from_secs(1)).await;

//                     info!("remaining alive connections: {}", handle.connection_count());
//                 }
//             }
//             None => handle.shutdown(),
//         }
//     });
// }
