use std::ops::ControlFlow;
use std::sync::Arc;
use std::time::Duration;

use derive_more::Constructor;
use futures_util::StreamExt;
use tokio::select;
use tokio::sync::oneshot;
use tokio::task::{JoinHandle, JoinSet};
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;
use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_server_lib::logging::STARTED_ON;
use torrust_server_lib::registar::ServiceHealthCheckJob;
use torrust_server_lib::signals::{Halted, Started, global_shutdown_signal};
use torrust_tracker_client::udp::client::check;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_core::event::ConnectionContext;
use torrust_tracker_udp_core::{self, ConnectionIdValidationPolicy, UDP_TRACKER_LOG_TARGET};
use tracing::instrument;

use super::request_buffer::ActiveRequests;
use crate::RawRequest;
use crate::container::UdpTrackerServerContainer;
use crate::event::Event;
use crate::event::sender::Sender;
use crate::server::bound_socket::BoundSocket;
use crate::server::processor::{Processor, ProcessorError};
use crate::server::receiver::Receiver;

/// A UDP server instance launcher.
#[derive(Constructor)]
pub struct Launcher;

/// A started receive loop and the binding it serves.
pub(crate) struct StartedReceiveLoop {
    pub service_binding: ServiceBinding,
    pub address: std::net::SocketAddr,
    pub task: JoinHandle<Result<(), std::io::Error>>,
}

/// Aborts the receive loop if its owner is dropped before joining it or handing it on.
pub(crate) struct OwnedReceiveLoop(Option<JoinHandle<Result<(), std::io::Error>>>);

impl OwnedReceiveLoop {
    pub(crate) const fn new(task: JoinHandle<Result<(), std::io::Error>>) -> Self {
        Self(Some(task))
    }

    async fn join(&mut self) -> Result<(), std::io::Error> {
        match &mut self.0 {
            Some(task) => task.await.map_err(std::io::Error::other)?,
            None => Ok(()),
        }
    }

    /// Hands the task to a new owner without aborting it.
    pub(crate) fn into_task(mut self) -> JoinHandle<Result<(), std::io::Error>> {
        self.0.take().expect("the receive loop is handed on at most once")
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct RequestDrainOutcome {
    completed: u64,
    failed: u64,
    aborted: u64,
    evicted: u64,
}

/// How long accepted UDP requests may keep running after cancellation (issue #2370, Q4).
// Private until the shutdown policy configuration makes it configurable (SI-20).
const REQUEST_DRAIN_DEADLINE: Duration = Duration::from_secs(5);

/// Waits up to `deadline` for every request processor, then aborts and joins the rest.
///
/// Cancellations seen before the deadline come from overload eviction, not from this drain.
// ADR: packages/udp-server/docs/adrs/20260929181216_bound_udp_request_concurrency_with_task_per_request_ring.md
// issue: #2370
async fn drain_request_processors<E>(tasks: &mut JoinSet<Result<(), E>>, deadline: Duration) -> RequestDrainOutcome
where
    E: 'static,
{
    let mut outcome = RequestDrainOutcome::default();

    let drained_before_deadline = timeout(deadline, async {
        while let Some(result) = tasks.join_next().await {
            record_request_processor_outcome(result, &mut outcome, false);
        }
    })
    .await;

    if drained_before_deadline.is_err() {
        while let Some(result) = tasks.try_join_next() {
            record_request_processor_outcome(result, &mut outcome, false);
        }
        tracing::warn!(target: UDP_TRACKER_LOG_TARGET, remaining = tasks.len(), ?deadline, "UDP request drain deadline reached: aborting remaining request processors");
        tasks.abort_all();
        while let Some(result) = tasks.join_next().await {
            record_request_processor_outcome(result, &mut outcome, true);
        }
    }

    outcome
}

fn record_request_processor_outcome<E>(
    result: Result<Result<(), E>, tokio::task::JoinError>,
    outcome: &mut RequestDrainOutcome,
    aborted_by_drain: bool,
) {
    match result {
        Ok(Ok(())) => outcome.completed += 1,
        // A returned error was already logged by the processor.
        Ok(Err(_)) => outcome.failed += 1,
        Err(error) if error.is_cancelled() && aborted_by_drain => outcome.aborted += 1,
        Err(error) if error.is_cancelled() => outcome.evicted += 1,
        Err(error) => {
            tracing::error!(target: UDP_TRACKER_LOG_TARGET, %error, "UDP request processor failed during shutdown drain");
            outcome.failed += 1;
        }
    }
}

/// Drains the request processors after cancellation and logs one outcome summary.
async fn drain_request_processors_on_shutdown(
    processors: &mut JoinSet<Result<(), ProcessorError>>,
    deadline: Duration,
    service_binding: &str,
) {
    log_request_drain_start(processors.len(), deadline, service_binding);

    let started = tokio::time::Instant::now();
    let outcome = drain_request_processors(processors, deadline).await;

    log_request_drain_outcome(&outcome, started.elapsed(), service_binding);
}

fn log_request_drain_start(active: usize, deadline: Duration, service_binding: &str) {
    if active == 0 {
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, service_binding, "Draining UDP request processors: none active");
    } else {
        tracing::info!(target: UDP_TRACKER_LOG_TARGET, service_binding, active, ?deadline, "Draining UDP request processors");
    }
}

/// Warns when the drain lost work (failed or aborted processors); otherwise reports at info.
fn log_request_drain_outcome(outcome: &RequestDrainOutcome, elapsed: Duration, service_binding: &str) {
    let RequestDrainOutcome {
        completed,
        failed,
        aborted,
        evicted,
    } = *outcome;

    if failed > 0 || aborted > 0 {
        tracing::warn!(target: UDP_TRACKER_LOG_TARGET, service_binding, completed, failed, aborted, evicted, ?elapsed, "UDP request processors drained");
    } else {
        tracing::info!(target: UDP_TRACKER_LOG_TARGET, service_binding, completed, failed, aborted, evicted, ?elapsed, "UDP request processors drained");
    }
}

/// Aborts and joins every processor, without the graceful deadline, before handing back the receive error.
async fn join_request_processors_after_receive_error<T: 'static>(
    processors: &mut JoinSet<T>,
    error: std::io::Error,
) -> std::io::Error {
    processors.shutdown().await;
    error
}

impl Drop for OwnedReceiveLoop {
    fn drop(&mut self) {
        if let Some(task) = &self.0 {
            task.abort();
        }
    }
}

impl Launcher {
    /// It starts the UDP server instance with graceful shutdown.
    ///
    /// This legacy entry point adapts the token-aware receive loop: a halt
    /// message, a dropped halt sender, or the global OS shutdown signal cancels
    /// the loop, which is then joined. Dropping this future aborts the loop.
    ///
    /// # Errors
    ///
    /// Returns an error if the startup notification receiver is dropped or the
    /// receive loop stops with an error.
    #[instrument(skip(udp_tracker_core_container, udp_tracker_server_container, bound_socket, tx_start, rx_halt))]
    pub async fn run_with_graceful_shutdown(
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        bound_socket: BoundSocket,
        cookie_lifetime: Duration,
        connection_id_validation: ConnectionIdValidationPolicy,
        tx_start: oneshot::Sender<Started>,
        rx_halt: oneshot::Receiver<Halted>,
    ) -> Result<(), std::io::Error> {
        let local_udp_url = bound_socket.url().to_string();
        let cancellation_token = CancellationToken::new();

        let StartedReceiveLoop {
            service_binding,
            address,
            task,
        } = Self::start_receive_loop(
            udp_tracker_core_container,
            udp_tracker_server_container,
            bound_socket,
            cookie_lifetime,
            connection_id_validation,
            cancellation_token.clone(),
        );
        let mut receive_loop = OwnedReceiveLoop::new(task);

        if tx_start
            .send(Started {
                service_binding,
                address,
            })
            .is_err()
        {
            cancellation_token.cancel();
            drop(receive_loop.join().await);
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "UDP startup receiver was dropped",
            ));
        }

        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (started)");

        select! {
            result = receive_loop.join() => {
                tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (stopped)");
                result
            },
            () = legacy_stop_requested(rx_halt, format!("Halting UDP Service Bound to Socket: {address}")) => {
                tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (halting)");
                cancellation_token.cancel();
                receive_loop.join().await
            }
        }
    }

    /// Logs the listener startup and spawns the receive loop, which stops
    /// admitting datagrams when `cancellation_token` is cancelled, drains its
    /// request processors, and returns `Ok(())`. The caller owns the returned task.
    pub(crate) fn start_receive_loop(
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        bound_socket: BoundSocket,
        cookie_lifetime: Duration,
        connection_id_validation: ConnectionIdValidationPolicy,
        cancellation_token: CancellationToken,
    ) -> StartedReceiveLoop {
        Self::start_receive_loop_with_request_drain_deadline(
            udp_tracker_core_container,
            udp_tracker_server_container,
            bound_socket,
            cookie_lifetime,
            connection_id_validation,
            cancellation_token,
            REQUEST_DRAIN_DEADLINE,
        )
    }

    pub(crate) fn start_receive_loop_with_request_drain_deadline(
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        bound_socket: BoundSocket,
        cookie_lifetime: Duration,
        connection_id_validation: ConnectionIdValidationPolicy,
        cancellation_token: CancellationToken,
        request_drain_deadline: Duration,
    ) -> StartedReceiveLoop {
        let service_binding = bound_socket.service_binding();
        let address = bound_socket.address();
        let local_udp_url = bound_socket.url().to_string();

        log_listener_startup(address, connection_id_validation, &local_udp_url);

        let receiver = Receiver::new(bound_socket.into());

        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::start_receive_loop (spawning main loop)");

        let task = tokio::task::spawn(async move {
            tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr = local_udp_url, "Udp::start_receive_loop::task (listening...)");
            Self::run_udp_server_main(
                receiver,
                udp_tracker_core_container,
                udp_tracker_server_container,
                cookie_lifetime,
                connection_id_validation,
                cancellation_token,
                request_drain_deadline,
            )
            .await
        });

        StartedReceiveLoop {
            service_binding,
            address,
            task,
        }
    }

    #[must_use]
    #[instrument(skip(service_binding))]
    pub fn check(service_binding: &ServiceBinding) -> ServiceHealthCheckJob {
        let info = format!("checking the udp tracker health check at: {}", service_binding.bind_address());

        let service_binding_clone = service_binding.clone();

        let job = tokio::spawn(async move { check(&service_binding_clone).await });

        ServiceHealthCheckJob::new(info, job)
    }

    // ADR: packages/udp-server/docs/adrs/20260929181216_bound_udp_request_concurrency_with_task_per_request_ring.md
    // issue: #2370
    // issue-spec: docs/issues/drafts/simplify-udp-server-main-loop/ISSUE.md
    #[instrument(skip(receiver, udp_tracker_core_container, udp_tracker_server_container, cancellation_token))]
    async fn run_udp_server_main(
        mut receiver: Receiver,
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        cookie_lifetime: Duration,
        connection_id_validation: ConnectionIdValidationPolicy,
        cancellation_token: CancellationToken,
        request_drain_deadline: Duration,
    ) -> Result<(), std::io::Error> {
        let active_requests = &mut ActiveRequests::default();
        // Owns every processor for shutdown; the ring above only decides overload eviction.
        let mut processors: JoinSet<Result<(), ProcessorError>> = JoinSet::new();

        let server_socket_addr = receiver.bound_socket_address();

        let server_service_binding =
            ServiceBinding::new(Protocol::UDP, server_socket_addr).expect("Bound socket to service binding should not fail");

        let local_addr = server_service_binding.clone().to_string();

        let cookie_lifetime = cookie_lifetime.as_secs_f64();

        // Created once: a per-iteration future would register a new notifier waiter for every datagram.
        let cancelled = cancellation_token.cancelled();
        tokio::pin!(cancelled);

        loop {
            let server_service_binding =
                ServiceBinding::new(Protocol::UDP, server_socket_addr).expect("Bound socket to service binding should not fail");

            let next = {
                tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server (wait for request)");
                select! {
                    biased;
                    () = &mut cancelled => {
                        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server (cancelled: stop admitting requests)");
                        drain_request_processors_on_shutdown(&mut processors, request_drain_deadline, &local_addr).await;
                        return Ok(());
                    }
                    next = receiver.next() => next,
                }
            };

            let req = match admit_received(next, &local_addr) {
                ControlFlow::Continue(req) => req,
                ControlFlow::Break(error) => {
                    return Err(join_request_processors_after_receive_error(&mut processors, error).await);
                }
            };

            tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server::loop (in)");

            let client_socket_addr = req.from;
            publish_event_if_sender_available(
                &udp_tracker_server_container.stats_event_sender,
                Event::UdpRequestReceived {
                    context: ConnectionContext::new(
                        udp_tracker_core_container.configuration_instance_id,
                        client_socket_addr,
                        server_service_binding.clone(),
                    ),
                },
            )
            .await;

            if Self::should_discard_request(
                &req,
                &udp_tracker_core_container,
                &udp_tracker_server_container,
                &server_service_binding,
                &local_addr,
                connection_id_validation,
            )
            .await
            {
                continue;
            }

            let processor = Processor::new(
                receiver.socket.clone(),
                udp_tracker_core_container.clone(),
                udp_tracker_server_container.clone(),
                cookie_lifetime,
                connection_id_validation,
            );

            /* We spawn the new task even if the active requests buffer is
            full. This could seem counterintuitive because we are accepting
            more request and consuming more memory even if the server is
            already busy. However, we "force_push" the new tasks in the
            buffer. That means, in the worst scenario we will abort a
            running task to make place for the new task.

            Once concern could be to reach an starvation point were we are
            only adding and removing tasks without given them the chance to
            finish. However, the buffer is yielding before aborting one
            tasks, giving it the chance to finish. */
            while processors.try_join_next().is_some() {}
            let abort_handle: tokio::task::AbortHandle = processors.spawn(processor.process_request(req));

            if abort_handle.is_finished() {
                continue;
            }

            let old_request_aborted = active_requests.force_push(abort_handle, &local_addr).await;

            if old_request_aborted {
                // Evicted task from active requests buffer was aborted.

                publish_event_if_sender_available(
                    &udp_tracker_server_container.stats_event_sender,
                    Event::UdpRequestAborted {
                        context: ConnectionContext::new(
                            udp_tracker_core_container.configuration_instance_id,
                            client_socket_addr,
                            server_service_binding,
                        ),
                    },
                )
                .await;
            }
        }
    }

    async fn should_discard_request(
        req: &crate::RawRequest,
        udp_tracker_core_container: &UdpTrackerCoreContainer,
        udp_tracker_server_container: &UdpTrackerServerContainer,
        server_service_binding: &ServiceBinding,
        local_addr: &str,
        connection_id_validation: ConnectionIdValidationPolicy,
    ) -> bool {
        let client_socket_addr = req.from;

        // Discard source-port-zero requests before processing: they cannot
        // receive a response and could evict active work. See the defensive
        // guard in `Processor::process_request`.
        if client_socket_addr.port() == 0 {
            tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_addr, %client_socket_addr, "Udp::run_udp_server::loop continue: (discarded: client source port is 0)");

            publish_event_if_sender_available(
                &udp_tracker_server_container.stats_event_sender,
                Event::UdpRequestDiscarded {
                    context: ConnectionContext::new(
                        udp_tracker_core_container.configuration_instance_id,
                        client_socket_addr,
                        server_service_binding.clone(),
                    ),
                },
            )
            .await;

            return true;
        }

        // When connection ID validation is disabled, the tracker accepts invalid
        // IDs. Banning still observes cookie errors, but enforcement is skipped.
        let ban_enforcement_active = connection_id_validation == ConnectionIdValidationPolicy::Strict;
        if ban_enforcement_active
            && udp_tracker_core_container
                .ban_service
                .read()
                .await
                .is_banned(&client_socket_addr.ip())
        {
            tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server::loop continue: (banned ip)");

            publish_event_if_sender_available(
                &udp_tracker_server_container.stats_event_sender,
                Event::UdpRequestBanned {
                    context: ConnectionContext::new(
                        udp_tracker_core_container.configuration_instance_id,
                        client_socket_addr,
                        server_service_binding.clone(),
                    ),
                },
            )
            .await;

            return true;
        }

        false
    }
}

/// Resolves when a legacy consumer asks the UDP server to stop.
///
/// A dropped halt sender counts as a stop request instead of a panic. The
/// global OS signal is still observed until the legacy API is removed (SI-19).
async fn legacy_stop_requested(rx_halt: oneshot::Receiver<Halted>, message: String) {
    select! {
        () = halt_requested(rx_halt) => (),
        () = global_shutdown_signal() => tracing::debug!(target: UDP_TRACKER_LOG_TARGET, "Global shutdown signal processed"),
    }

    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "{message}");
}

async fn halt_requested(rx_halt: oneshot::Receiver<Halted>) {
    if let Ok(signal) = rx_halt.await {
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, "Halt signal processed: {signal}");
    } else {
        tracing::warn!(target: UDP_TRACKER_LOG_TARGET, "UDP halt sender dropped; stopping the server");
    }
}

fn log_listener_startup(
    bind_to: std::net::SocketAddr,
    connection_id_validation: ConnectionIdValidationPolicy,
    local_udp_url: &str,
) {
    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Starting on: {bind_to}");

    if connection_id_validation == ConnectionIdValidationPolicy::Disabled {
        tracing::warn!(
            target: UDP_TRACKER_LOG_TARGET,
            %bind_to,
            "UDP connection ID validation is DISABLED for this listener. \
             Anti-spoofing and replay protection are reduced. \
             Ensure this listener is isolated through external network controls."
        );
    }

    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "{STARTED_ON}: {local_udp_url}");
}

/// Decides whether the receive loop admits the next item or stops with an error.
///
/// Any receive error, including `Interrupted`, and the end of the stream stop
/// the loop with an error so the owning component reports a failure.
fn admit_received(next: Option<std::io::Result<RawRequest>>, local_addr: &str) -> ControlFlow<std::io::Error, RawRequest> {
    match next {
        Some(Ok(req)) => ControlFlow::Continue(req),
        Some(Err(error)) if error.kind() == std::io::ErrorKind::Interrupted => {
            tracing::warn!(target: UDP_TRACKER_LOG_TARGET, local_addr, err = %error,  "Udp::run_udp_server::loop (interrupted)");
            ControlFlow::Break(error)
        }
        Some(Err(error)) => {
            tracing::error!(target: UDP_TRACKER_LOG_TARGET, local_addr, err = %error,  "Udp::run_udp_server::loop break: (got error)");
            ControlFlow::Break(error)
        }
        None => {
            tracing::error!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server breaking: (ran dry, should not happen in production!)");
            ControlFlow::Break(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "UDP receive stream ended",
            ))
        }
    }
}

async fn publish_event_if_sender_available(sender: &Sender, event: Event) {
    if let Some(sender) = sender.as_deref() {
        sender.send(event).await;
    }
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use tokio::sync::oneshot;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::signals::{Halted, Started};
    use torrust_tracker_configuration::v3_0_0::logging;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_test_helpers::configuration::ephemeral_public;
    use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
    use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
    use torrust_tracker_udp_core::event::ConnectionContext;

    use super::Launcher;
    use crate::RawRequest;
    use crate::container::UdpTrackerServerContainer;
    use crate::event::Event;
    use crate::event::receiver::Receiver;
    use crate::server::bound_socket::BoundSocket;

    const TEST_LOG_TARGET: &str = "udp://test";
    // This is an absolute failure bound, not a scheduling delay. Event-publication regressions
    // must fail diagnostically instead of leaving the test process waiting indefinitely.
    const EVENT_PUBLICATION_TIMEOUT: Duration = Duration::from_secs(1);
    const LIFECYCLE_TIMEOUT: Duration = Duration::from_secs(5);
    const BIND_RETRY_INTERVAL: Duration = Duration::from_millis(10);

    /// Counts a processor as running until its future is dropped by completion or abort.
    struct RunningProcessor(Arc<AtomicUsize>);

    impl RunningProcessor {
        fn start(running: &Arc<AtomicUsize>) -> Self {
            running.fetch_add(1, Ordering::SeqCst);
            Self(running.clone())
        }
    }

    impl Drop for RunningProcessor {
        fn drop(&mut self) {
            self.0.fetch_sub(1, Ordering::SeqCst);
        }
    }

    struct UdpLauncherTestContext {
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        cookie_lifetime: Duration,
        bind_address: SocketAddr,
        max_connection_id_errors_per_ip: u32,
    }

    impl UdpLauncherTestContext {
        async fn new() -> Self {
            let configuration = Arc::new(ephemeral_public());
            let core_config = Arc::new(configuration.core.clone());
            let udp_tracker_config = Arc::new(
                configuration
                    .udp_trackers
                    .clone()
                    .expect("UDP test configuration should include a tracker")
                    .into_iter()
                    .next()
                    .expect("UDP test configuration should include one tracker"),
            );
            torrust_clock::initialize_static();
            torrust_tracker_udp_core::initialize_static();
            logging::setup(&configuration.logging);

            let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
            let udp_tracker_core_container = UdpTrackerCoreContainer::initialize(
                &core_config,
                &udp_tracker_config,
                configuration.udp_tracker_server.max_connection_id_errors_per_ip,
                configuration_instance_id,
            )
            .await;
            let udp_tracker_server_container = UdpTrackerServerContainer::initialize(&core_config);

            Self {
                udp_tracker_core_container,
                udp_tracker_server_container,
                cookie_lifetime: udp_tracker_config.cookie_lifetime,
                bind_address: udp_tracker_config.bind_address,
                max_connection_id_errors_per_ip: configuration.udp_tracker_server.max_connection_id_errors_per_ip,
            }
        }

        async fn with_banned_client_ip(client_ip: IpAddr) -> Self {
            let context = Self::new().await;
            let mut ban_service = context.udp_tracker_core_container.ban_service.write().await;

            for _ in 0..=context.max_connection_id_errors_per_ip {
                ban_service.increase_counter(&client_ip);
            }

            drop(ban_service);
            context
        }

        /// Calls `should_discard_request` with the fixture's containers, binding, and log target.
        async fn should_discard(&self, request: &RawRequest, connection_id_validation: ConnectionIdValidationPolicy) -> bool {
            Launcher::should_discard_request(
                request,
                &self.udp_tracker_core_container,
                &self.udp_tracker_server_container,
                &sample_udp_service_binding(),
                TEST_LOG_TARGET,
                connection_id_validation,
            )
            .await
        }

        fn connection_context(&self, client: SocketAddr) -> ConnectionContext {
            ConnectionContext::new(
                self.udp_tracker_core_container.configuration_instance_id,
                client,
                sample_udp_service_binding(),
            )
        }

        fn subscribe_to_events(&self) -> Receiver {
            self.udp_tracker_server_container.event_bus.receiver()
        }
    }

    /// Pure fixture: admission only clones this binding into the published event
    /// context, so any valid UDP binding works and no launcher state is involved.
    fn sample_udp_service_binding() -> ServiceBinding {
        ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969))
            .expect("sample UDP service binding should be valid")
    }

    fn sample_client() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)), 8080)
    }

    /// Admission reads only the source address; the payload is irrelevant.
    fn request_from(client: SocketAddr) -> RawRequest {
        RawRequest {
            payload: Vec::new(),
            from: client,
        }
    }

    async fn next_published_event(events: &mut Receiver) -> Event {
        tokio::time::timeout(EVENT_PUBLICATION_TIMEOUT, events.recv())
            .await
            .expect("an event should be published before the test deadline")
            .expect("the event receiver should remain connected")
    }

    #[tokio::test]
    async fn it_should_report_a_broken_pipe_when_the_startup_notification_receiver_is_dropped() {
        // Arrange
        let launcher = UdpLauncherTestContext::new().await;
        let bound_socket = BoundSocket::bind(launcher.bind_address, false).expect("UDP socket should bind");
        let (startup_notification_sender, startup_notification_receiver) = oneshot::channel::<Started>();
        let (_halt_sender, halt_receiver) = oneshot::channel::<Halted>();
        drop(startup_notification_receiver);

        // Act
        let result = Launcher::run_with_graceful_shutdown(
            launcher.udp_tracker_core_container,
            launcher.udp_tracker_server_container,
            bound_socket,
            launcher.cookie_lifetime,
            ConnectionIdValidationPolicy::Strict,
            startup_notification_sender,
            halt_receiver,
        )
        .await;

        // Assert
        assert_eq!(
            result.expect_err("startup notification should fail").kind(),
            ErrorKind::BrokenPipe
        );
    }

    #[tokio::test]
    async fn it_should_release_the_socket_when_the_startup_notification_receiver_is_dropped() {
        // Arrange
        let launcher = UdpLauncherTestContext::new().await;
        let bound_socket = BoundSocket::bind(launcher.bind_address, false).expect("UDP socket should bind");
        let bound_address = bound_socket.address();
        let (startup_notification_sender, startup_notification_receiver) = oneshot::channel::<Started>();
        let (_halt_sender, halt_receiver) = oneshot::channel::<Halted>();
        drop(startup_notification_receiver);

        // Act
        let _result = Launcher::run_with_graceful_shutdown(
            launcher.udp_tracker_core_container,
            launcher.udp_tracker_server_container,
            bound_socket,
            launcher.cookie_lifetime,
            ConnectionIdValidationPolicy::Strict,
            startup_notification_sender,
            halt_receiver,
        )
        .await;

        // Assert
        BoundSocket::bind(bound_address, false).expect("UDP socket should be released after startup notification failure");
    }

    /// A legacy launcher that has sent its startup notification and is waiting for a halt.
    struct RunningLegacyLauncher {
        bound_address: SocketAddr,
        halt_sender: oneshot::Sender<Halted>,
        task: tokio::task::JoinHandle<Result<(), std::io::Error>>,
    }

    impl RunningLegacyLauncher {
        async fn start() -> Self {
            let launcher = UdpLauncherTestContext::new().await;
            let bound_socket = BoundSocket::bind(launcher.bind_address, false).expect("UDP socket should bind");
            let bound_address = bound_socket.address();
            let (startup_notification_sender, startup_notification_receiver) = oneshot::channel::<Started>();
            let (halt_sender, halt_receiver) = oneshot::channel::<Halted>();
            let task = tokio::spawn(Launcher::run_with_graceful_shutdown(
                launcher.udp_tracker_core_container,
                launcher.udp_tracker_server_container,
                bound_socket,
                launcher.cookie_lifetime,
                torrust_tracker_udp_core::ConnectionIdValidationPolicy::Strict,
                startup_notification_sender,
                halt_receiver,
            ));
            tokio::time::timeout(LIFECYCLE_TIMEOUT, startup_notification_receiver)
                .await
                .expect("the legacy launcher should start within the test deadline")
                .expect("the legacy launcher should send its startup notification");

            Self {
                bound_address,
                halt_sender,
                task,
            }
        }
    }

    async fn wait_until_bindable(address: SocketAddr) -> bool {
        tokio::time::timeout(LIFECYCLE_TIMEOUT, async {
            while std::net::UdpSocket::bind(address).is_err() {
                tokio::time::sleep(BIND_RETRY_INTERVAL).await;
            }
        })
        .await
        .is_ok()
    }

    #[tokio::test]
    async fn it_should_stop_without_panicking_and_release_the_socket_when_the_legacy_halt_sender_is_dropped() {
        // Arrange
        let launcher = RunningLegacyLauncher::start().await;

        // Act
        drop(launcher.halt_sender);
        let result = tokio::time::timeout(LIFECYCLE_TIMEOUT, launcher.task)
            .await
            .expect("the legacy launcher should stop within the test deadline");

        // Assert
        assert!(
            matches!(result, Ok(Ok(()))),
            "a dropped halt sender should stop the legacy launcher cleanly: {result:?}"
        );
        assert!(
            wait_until_bindable(launcher.bound_address).await,
            "the legacy launcher must release its socket at {} within {LIFECYCLE_TIMEOUT:?}",
            launcher.bound_address
        );
    }

    #[tokio::test]
    async fn it_should_release_the_socket_when_the_legacy_launcher_task_is_aborted() {
        // Arrange
        let launcher = RunningLegacyLauncher::start().await;
        let _halt_sender = launcher.halt_sender;

        // Act
        launcher.task.abort();
        drop(launcher.task.await);

        // Assert
        assert!(
            wait_until_bindable(launcher.bound_address).await,
            "aborting the legacy launcher must release its socket at {} within {LIFECYCLE_TIMEOUT:?}",
            launcher.bound_address
        );
    }

    #[tokio::test]
    async fn it_should_discard_a_request_whose_source_port_is_zero() {
        // Arrange
        let launcher = UdpLauncherTestContext::new().await;
        let client = SocketAddr::new(sample_client().ip(), 0);

        // Act
        // The source-port-zero guard runs before ban policy, so the policy is inert here.
        let should_discard = launcher
            .should_discard(&request_from(client), ConnectionIdValidationPolicy::Strict)
            .await;

        // Assert
        assert!(should_discard);
    }

    #[tokio::test]
    async fn it_should_discard_a_request_from_a_banned_client_in_strict_mode() {
        // Arrange
        let client = sample_client();
        let launcher = UdpLauncherTestContext::with_banned_client_ip(client.ip()).await;

        // Act
        let should_discard = launcher
            .should_discard(&request_from(client), ConnectionIdValidationPolicy::Strict)
            .await;

        // Assert
        assert!(should_discard);
    }

    #[tokio::test]
    async fn it_should_admit_a_request_from_a_client_that_is_not_banned() {
        // Arrange
        let launcher = UdpLauncherTestContext::new().await;

        // Act
        let should_discard = launcher
            .should_discard(&request_from(sample_client()), ConnectionIdValidationPolicy::Strict)
            .await;

        // Assert
        assert!(!should_discard);
    }

    #[tokio::test]
    async fn it_should_admit_a_request_from_a_banned_client_when_connection_id_validation_is_disabled() {
        // Arrange
        let client = sample_client();
        let launcher = UdpLauncherTestContext::with_banned_client_ip(client.ip()).await;

        // Act
        let should_discard = launcher
            .should_discard(&request_from(client), ConnectionIdValidationPolicy::Disabled)
            .await;

        // Assert
        assert!(!should_discard);
    }

    #[tokio::test]
    async fn it_should_publish_a_request_banned_event_when_its_client_ip_is_banned_in_strict_mode() {
        // Arrange
        let client = sample_client();
        let launcher = UdpLauncherTestContext::with_banned_client_ip(client.ip()).await;
        let mut events = launcher.subscribe_to_events();

        // Act
        launcher
            .should_discard(&request_from(client), ConnectionIdValidationPolicy::Strict)
            .await;

        // Assert
        assert_eq!(
            next_published_event(&mut events).await,
            Event::UdpRequestBanned {
                context: launcher.connection_context(client)
            }
        );
    }

    #[tokio::test]
    async fn it_should_publish_a_request_discarded_event_when_its_source_port_is_zero() {
        // Arrange
        let launcher = UdpLauncherTestContext::new().await;
        let client = SocketAddr::new(sample_client().ip(), 0);
        let mut events = launcher.subscribe_to_events();

        // Act
        // The source-port-zero guard runs before ban policy, so the policy is inert here.
        launcher
            .should_discard(&request_from(client), ConnectionIdValidationPolicy::Strict)
            .await;

        // Assert
        assert_eq!(
            next_published_event(&mut events).await,
            Event::UdpRequestDiscarded {
                context: launcher.connection_context(client)
            }
        );
    }

    mod receive_loop_admission {
        use std::io::ErrorKind;
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::ops::ControlFlow;

        use super::TEST_LOG_TARGET;
        use crate::RawRequest;
        use crate::server::launcher::admit_received;

        fn datagram() -> RawRequest {
            RawRequest {
                payload: vec![0u8; 16],
                from: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6881),
            }
        }

        #[test]
        fn it_should_admit_a_received_datagram() {
            let datagram = datagram();

            let admission = admit_received(Some(Ok(datagram.clone())), TEST_LOG_TARGET);

            assert!(matches!(admission, ControlFlow::Continue(admitted) if admitted == datagram));
        }

        #[test]
        fn it_should_stop_with_the_receive_error_when_receiving_fails() {
            let admission = admit_received(Some(Err(ErrorKind::ConnectionReset.into())), TEST_LOG_TARGET);

            assert!(matches!(admission, ControlFlow::Break(error) if error.kind() == ErrorKind::ConnectionReset));
        }

        #[test]
        fn it_should_stop_with_the_receive_error_when_receiving_is_interrupted() {
            let admission = admit_received(Some(Err(ErrorKind::Interrupted.into())), TEST_LOG_TARGET);

            assert!(matches!(admission, ControlFlow::Break(error) if error.kind() == ErrorKind::Interrupted));
        }

        #[test]
        fn it_should_stop_with_an_unexpected_end_error_when_the_receive_stream_ends() {
            let admission = admit_received(None, TEST_LOG_TARGET);

            assert!(matches!(admission, ControlFlow::Break(error) if error.kind() == ErrorKind::UnexpectedEof));
        }
    }

    mod request_drain {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::time::Duration;

        use tokio::sync::oneshot;
        use tokio::task::JoinSet;
        use tokio::time::Instant;

        use super::RunningProcessor;
        use crate::server::launcher::{
            RequestDrainOutcome, drain_request_processors, join_request_processors_after_receive_error,
        };

        const DRAIN_DEADLINE: Duration = Duration::from_secs(5);

        type ProcessorResult = Result<(), &'static str>;

        #[tokio::test(start_paused = true)]
        async fn it_should_count_processors_that_finish_before_the_deadline_as_completed() {
            // Arrange
            let mut processors: JoinSet<ProcessorResult> = JoinSet::new();
            processors.spawn(async { Ok(()) });
            processors.spawn(async { Ok(()) });

            // Act
            let outcome = drain_request_processors(&mut processors, DRAIN_DEADLINE).await;

            // Assert
            assert_eq!(
                outcome,
                RequestDrainOutcome {
                    completed: 2,
                    ..RequestDrainOutcome::default()
                }
            );
        }

        #[tokio::test(start_paused = true)]
        async fn it_should_abort_and_join_processors_still_running_at_the_deadline() {
            // Arrange
            let mut processors: JoinSet<ProcessorResult> = JoinSet::new();
            processors.spawn(std::future::pending());

            // Act
            let outcome = drain_request_processors(&mut processors, DRAIN_DEADLINE).await;

            // Assert
            assert_eq!(
                outcome,
                RequestDrainOutcome {
                    aborted: 1,
                    ..RequestDrainOutcome::default()
                }
            );
        }

        #[tokio::test(start_paused = true)]
        async fn it_should_count_failed_processors_and_keep_draining_the_rest() {
            // Arrange
            let mut processors: JoinSet<ProcessorResult> = JoinSet::new();
            let (panicked, after_panic) = oneshot::channel();
            processors.spawn(async { Err("the response could not be sent") });
            processors.spawn(async move {
                panicked
                    .send(())
                    .expect("the processor waiting for the panic should be alive");
                panic!("request processor panic under test");
            });
            // Finishes only after the panic, so a drain that stopped at a failure would miss it.
            processors.spawn(async move { after_panic.await.map_err(|_| "the panicking processor did not signal") });

            // Act
            let outcome = drain_request_processors(&mut processors, DRAIN_DEADLINE).await;

            // Assert
            assert_eq!(
                outcome,
                RequestDrainOutcome {
                    completed: 1,
                    failed: 2,
                    ..RequestDrainOutcome::default()
                }
            );
        }

        #[tokio::test(start_paused = true)]
        async fn it_should_count_a_processor_aborted_before_the_drain_as_evicted() {
            // Arrange
            let mut processors: JoinSet<ProcessorResult> = JoinSet::new();
            let evicted_by_overload = processors.spawn(std::future::pending());
            evicted_by_overload.abort();

            // Act
            let outcome = drain_request_processors(&mut processors, DRAIN_DEADLINE).await;

            // Assert
            assert_eq!(
                outcome,
                RequestDrainOutcome {
                    evicted: 1,
                    ..RequestDrainOutcome::default()
                }
            );
        }

        #[tokio::test(start_paused = true)]
        async fn it_should_return_immediately_when_there_are_no_processors() {
            // Arrange
            let mut processors: JoinSet<ProcessorResult> = JoinSet::new();
            let started = Instant::now();

            // Act
            drain_request_processors(&mut processors, DRAIN_DEADLINE).await;

            // Assert
            assert_eq!(
                started.elapsed(),
                Duration::ZERO,
                "draining no processors must not wait for the {DRAIN_DEADLINE:?} deadline"
            );
        }

        #[tokio::test(start_paused = true)]
        async fn it_should_join_every_processor_before_returning_the_receive_error() {
            // Arrange
            let running = Arc::new(AtomicUsize::new(0));
            let mut processors: JoinSet<ProcessorResult> = JoinSet::new();
            let processor = RunningProcessor::start(&running);
            processors.spawn(async move {
                let _running = processor;
                std::future::pending().await
            });

            // Act
            let error = join_request_processors_after_receive_error(
                &mut processors,
                std::io::Error::new(std::io::ErrorKind::ConnectionReset, "receive failed under test"),
            )
            .await;

            // Assert
            assert_eq!(error.kind(), std::io::ErrorKind::ConnectionReset);
            assert_eq!(
                running.load(Ordering::SeqCst),
                0,
                "no processor may outlive the receive error"
            );
            assert!(processors.is_empty());
        }
    }

    mod receive_loop_shutdown {
        use std::net::SocketAddr;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::time::Duration;

        use futures::future::BoxFuture;
        use tokio::net::UdpSocket;
        use tokio::sync::{mpsc, watch};
        use tokio::task::{JoinError, JoinHandle};
        use tokio_util::sync::CancellationToken;
        use torrust_tracker_events::sender::{SendError, Sender};
        use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
        use torrust_tracker_udp_protocol::{ConnectRequest, Request, TransactionId};

        use super::{LIFECYCLE_TIMEOUT, RunningProcessor, UdpLauncherTestContext, wait_until_bindable};
        use crate::container::UdpTrackerServerContainer;
        use crate::event::Event;
        use crate::server::bound_socket::BoundSocket;
        use crate::server::launcher::{Launcher, StartedReceiveLoop};
        use crate::server::request_buffer::ACTIVE_REQUESTS_CAPACITY;

        /// Shorter than the time a held processor waits, so the drain must abort it.
        const SHORT_DRAIN_DEADLINE: Duration = Duration::from_millis(100);

        /// Holds every processor at its `UdpRequestAccepted` publication until released,
        /// except the processors serving `released_client`.
        struct HoldAcceptedRequestsSender {
            released_client: SocketAddr,
            running_held_processors: Arc<AtomicUsize>,
            held: mpsc::UnboundedSender<()>,
            release: watch::Receiver<bool>,
        }

        impl Sender for HoldAcceptedRequestsSender {
            type Event = Event;

            fn send(&self, event: Event) -> BoxFuture<'_, Option<Result<usize, SendError<Event>>>> {
                let is_held = matches!(
                    &event,
                    Event::UdpRequestAccepted { context, .. } if context.client_socket_addr() != self.released_client
                );
                if !is_held {
                    return Box::pin(async { None });
                }

                let running = RunningProcessor::start(&self.running_held_processors);
                self.held
                    .send(())
                    .expect("the held-processor receiver should outlive the scenario");
                let mut release = self.release.clone();

                Box::pin(async move {
                    let _running = running;
                    // The scenario keeps the release sender alive, so this waits for an explicit release.
                    let _released = release.wait_for(|released| *released).await;
                    None
                })
            }
        }

        /// A receive loop whose processors hold at acceptance until the scenario releases them.
        struct ReceiveLoopHoldingRequests {
            address: SocketAddr,
            cancellation_token: CancellationToken,
            receive_loop: JoinHandle<Result<(), std::io::Error>>,
            running_held_processors: Arc<AtomicUsize>,
            held: mpsc::UnboundedReceiver<()>,
            release: watch::Sender<bool>,
        }

        impl ReceiveLoopHoldingRequests {
            async fn start(released_client: SocketAddr, request_drain_deadline: Duration) -> Self {
                let context = UdpLauncherTestContext::new().await;
                let running_held_processors = Arc::new(AtomicUsize::new(0));
                let (held_sender, held) = mpsc::unbounded_channel();
                let (release, release_receiver) = watch::channel(false);
                let server_container = Arc::new(UdpTrackerServerContainer {
                    event_bus: context.udp_tracker_server_container.event_bus.clone(),
                    stats_event_sender: Some(Arc::new(HoldAcceptedRequestsSender {
                        released_client,
                        running_held_processors: running_held_processors.clone(),
                        held: held_sender,
                        release: release_receiver,
                    })),
                    stats_repository: context.udp_tracker_server_container.stats_repository.clone(),
                });
                let cancellation_token = CancellationToken::new();
                let StartedReceiveLoop { address, task, .. } = Launcher::start_receive_loop_with_request_drain_deadline(
                    context.udp_tracker_core_container,
                    server_container,
                    BoundSocket::bind(context.bind_address, false).expect("UDP socket should bind"),
                    context.cookie_lifetime,
                    ConnectionIdValidationPolicy::Strict,
                    cancellation_token.clone(),
                    request_drain_deadline,
                );

                Self {
                    address,
                    cancellation_token,
                    receive_loop: task,
                    running_held_processors,
                    held,
                    release,
                }
            }

            async fn wait_until_held(&mut self, processors: usize) {
                tokio::time::timeout(LIFECYCLE_TIMEOUT, async {
                    for _ in 0..processors {
                        self.held.recv().await.expect("the held-processor channel should stay open");
                    }
                })
                .await
                .expect("the requests should reach their processors within the test deadline");
            }

            fn release_held_processors(&self) {
                self.release.send_replace(true);
            }

            fn running_held_processors(&self) -> usize {
                self.running_held_processors.load(Ordering::SeqCst)
            }

            async fn join_receive_loop(&mut self) -> Result<Result<(), std::io::Error>, JoinError> {
                tokio::time::timeout(LIFECYCLE_TIMEOUT, &mut self.receive_loop)
                    .await
                    .expect("the receive loop should stop within the test deadline")
            }
        }

        /// A receive loop whose full request ring has dropped the only handles of held processors.
        ///
        /// Its drain deadline is shorter than the hold, so shutdown must abort them.
        struct ReceiveLoopWithOrphanedProcessors;

        impl ReceiveLoopWithOrphanedProcessors {
            async fn start() -> ReceiveLoopHoldingRequests {
                let released_client = bind_loopback_client().await;
                let held_client = bind_loopback_client().await;
                let mut scenario = ReceiveLoopHoldingRequests::start(
                    released_client
                        .local_addr()
                        .expect("the released client should have an address"),
                    SHORT_DRAIN_DEADLINE,
                )
                .await;

                // The released request finishes first, so it is the oldest handle when the ring fills.
                send_connect_request(&released_client, scenario.address).await;
                wait_for_response(&released_client).await;

                for _ in 1..ACTIVE_REQUESTS_CAPACITY {
                    send_connect_request(&held_client, scenario.address).await;
                }
                scenario.wait_until_held(ACTIVE_REQUESTS_CAPACITY - 1).await;

                // Inserting into the full ring keeps only the last live handle it traverses.
                send_connect_request(&held_client, scenario.address).await;
                scenario.wait_until_held(1).await;

                scenario
            }
        }

        /// A receive loop with one accepted request held in flight and a drain deadline
        /// longer than the test waits, so only completion can end the drain.
        struct ReceiveLoopWithOneHeldRequest;

        impl ReceiveLoopWithOneHeldRequest {
            async fn start(held_client: &UdpSocket) -> ReceiveLoopHoldingRequests {
                let unused_client = SocketAddr::from(([127, 0, 0, 1], 0));
                let mut scenario = ReceiveLoopHoldingRequests::start(unused_client, LIFECYCLE_TIMEOUT * 2).await;

                send_connect_request(held_client, scenario.address).await;
                scenario.wait_until_held(1).await;

                scenario
            }
        }

        async fn bind_loopback_client() -> UdpSocket {
            UdpSocket::bind("127.0.0.1:0")
                .await
                .expect("a loopback client socket should bind")
        }

        async fn send_connect_request(client: &UdpSocket, tracker: SocketAddr) {
            let mut payload = Vec::new();
            Request::from(ConnectRequest {
                transaction_id: TransactionId(0i32.into()),
            })
            .write_bytes(&mut payload)
            .expect("a connect request should serialize");

            client
                .send_to(&payload, tracker)
                .await
                .expect("the connect request should be sent");
        }

        async fn wait_for_response(client: &UdpSocket) {
            let mut buffer = [0u8; 1024];

            tokio::time::timeout(LIFECYCLE_TIMEOUT, client.recv(&mut buffer))
                .await
                .expect("the released request should be answered within the test deadline")
                .expect("the released client should receive its response");
        }

        #[tokio::test]
        async fn it_should_not_return_while_processors_orphaned_by_the_request_ring_are_still_running() {
            // Arrange
            let mut scenario = ReceiveLoopWithOrphanedProcessors::start().await;

            // Act
            scenario.cancellation_token.cancel();
            let result = scenario.join_receive_loop().await;

            // Assert
            let running = scenario.running_held_processors();
            assert_eq!(
                running, 0,
                "the receive loop returned {result:?} while {running} request processors it spawned were still running"
            );
            assert!(
                matches!(result, Ok(Ok(()))),
                "cancellation should stop cleanly, got {result:?}"
            );
            assert!(
                wait_until_bindable(scenario.address).await,
                "the socket should be released once the aborted processors are joined"
            );
        }

        #[tokio::test]
        async fn it_should_let_an_accepted_request_finish_and_answer_before_returning_on_cancellation() {
            // Arrange
            let held_client = bind_loopback_client().await;
            let mut scenario = ReceiveLoopWithOneHeldRequest::start(&held_client).await;

            // Act
            scenario.cancellation_token.cancel();
            scenario.release_held_processors();
            let result = scenario.join_receive_loop().await;

            // Assert
            assert!(
                matches!(result, Ok(Ok(()))),
                "cancellation should stop cleanly, got {result:?}"
            );
            assert_eq!(scenario.running_held_processors(), 0);
            wait_for_response(&held_client).await;
        }
    }
}
