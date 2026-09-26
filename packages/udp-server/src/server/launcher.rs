use std::ops::ControlFlow;
use std::sync::Arc;
use std::time::Duration;

use derive_more::Constructor;
use futures_util::StreamExt;
use tokio::select;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_server_lib::logging::STARTED_ON;
use torrust_server_lib::registar::ServiceHealthCheckJob;
use torrust_server_lib::signals::{Halted, Started, shutdown_signal_with_message};
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
use crate::server::processor::Processor;
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

impl Launcher {
    /// It starts the UDP server instance with graceful shutdown.
    ///
    /// # Errors
    ///
    /// Returns an error if the startup notification receiver is dropped.
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

        let StartedReceiveLoop {
            service_binding,
            address,
            task: mut running,
        } = Self::start_receive_loop(
            udp_tracker_core_container,
            udp_tracker_server_container,
            bound_socket,
            cookie_lifetime,
            connection_id_validation,
            CancellationToken::new(),
        );

        if tx_start
            .send(Started {
                service_binding,
                address,
            })
            .is_err()
        {
            running.abort();
            drop(running.await);
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "UDP startup receiver was dropped",
            ));
        }

        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (started)");

        select! {
            _ = &mut running => {
                tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (stopped)");
            },
            () = shutdown_signal_with_message(rx_halt, format!("Halting UDP Service Bound to Socket: {address}")) => {
                tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (halting)");
                running.abort();
                drop(running.await);
            }
        }

        Ok(())
    }

    /// Logs the listener startup and spawns the receive loop, which stops
    /// admitting datagrams and returns `Ok(())` when `cancellation_token` is
    /// cancelled. The caller owns the returned task.
    pub(crate) fn start_receive_loop(
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        bound_socket: BoundSocket,
        cookie_lifetime: Duration,
        connection_id_validation: ConnectionIdValidationPolicy,
        cancellation_token: CancellationToken,
    ) -> StartedReceiveLoop {
        let service_binding = bound_socket.service_binding();
        let address = bound_socket.address();
        let local_udp_url = bound_socket.url().to_string();

        log_listener_startup(address, connection_id_validation, &local_udp_url);

        let receiver = Receiver::new(bound_socket.into());

        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (spawning main loop)");

        let task = tokio::task::spawn(async move {
            tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr = local_udp_url, "Udp::run_with_graceful_shutdown::task (listening...)");
            Self::run_udp_server_main(
                receiver,
                udp_tracker_core_container,
                udp_tracker_server_container,
                cookie_lifetime,
                connection_id_validation,
                cancellation_token,
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

    // issue-spec: docs/issues/drafts/simplify-udp-server-main-loop/ISSUE.md
    #[instrument(skip(receiver, udp_tracker_core_container, udp_tracker_server_container, cancellation_token))]
    async fn run_udp_server_main(
        mut receiver: Receiver,
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        cookie_lifetime: Duration,
        connection_id_validation: ConnectionIdValidationPolicy,
        cancellation_token: CancellationToken,
    ) -> Result<(), std::io::Error> {
        let active_requests = &mut ActiveRequests::default();

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
                        return Ok(());
                    }
                    next = receiver.next() => next,
                }
            };

            let req = match admit_received(next, &local_addr) {
                ControlFlow::Continue(req) => req,
                ControlFlow::Break(error) => return Err(error),
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
            let abort_handle: tokio::task::AbortHandle = tokio::task::spawn(processor.process_request(req)).abort_handle();

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
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;
    use std::time::Duration;

    use tokio::sync::oneshot;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::signals::{Halted, Started};
    use torrust_tracker_configuration::v3_0_0::logging;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_test_helpers::configuration::ephemeral_public;
    use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
    use torrust_tracker_udp_core::event::ConnectionContext;

    use super::Launcher;
    use crate::RawRequest;
    use crate::container::UdpTrackerServerContainer;
    use crate::event::Event;
    use crate::server::bound_socket::BoundSocket;

    const TEST_LOG_TARGET: &str = "udp://test";
    // This is an absolute failure bound, not a scheduling delay. Event-publication regressions
    // must fail diagnostically instead of leaving the test process waiting indefinitely.
    const EVENT_PUBLICATION_TIMEOUT: Duration = Duration::from_secs(1);

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
    }

    /// Pure fixture: admission only clones this binding into the published event
    /// context, so any valid UDP binding works and no launcher state is involved.
    fn sample_udp_service_binding() -> ServiceBinding {
        ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969))
            .expect("sample UDP service binding should be valid")
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
        let result = Launcher::run_with_graceful_shutdown(
            launcher.udp_tracker_core_container,
            launcher.udp_tracker_server_container,
            bound_socket,
            launcher.cookie_lifetime,
            torrust_tracker_udp_core::ConnectionIdValidationPolicy::Strict,
            startup_notification_sender,
            halt_receiver,
        )
        .await;

        // Assert
        assert_eq!(
            result.expect_err("startup notification should fail").kind(),
            std::io::ErrorKind::BrokenPipe
        );
        BoundSocket::bind(bound_address, false).expect("UDP socket should be released after startup notification failure");
    }

    #[tokio::test]
    async fn it_should_require_discarding_a_request_when_its_source_port_is_zero() {
        // Arrange
        let launcher = UdpLauncherTestContext::new().await;
        let client_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)), 0);
        let request = RawRequest {
            payload: Vec::new(),
            from: client_socket_addr,
        };
        let server_service_binding = sample_udp_service_binding();

        // Act
        // The source-port-zero guard runs before ban policy is evaluated, so the
        // validation-policy argument is inert for this contract.
        let should_discard = Launcher::should_discard_request(
            &request,
            &launcher.udp_tracker_core_container,
            &launcher.udp_tracker_server_container,
            &server_service_binding,
            TEST_LOG_TARGET,
            torrust_tracker_udp_core::ConnectionIdValidationPolicy::Strict,
        )
        .await;

        // Assert
        assert!(should_discard);
    }

    #[tokio::test]
    async fn it_should_require_discarding_a_request_when_its_client_ip_is_banned_in_strict_mode() {
        // Arrange
        let client_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)), 8080);
        let launcher = UdpLauncherTestContext::with_banned_client_ip(client_socket_addr.ip()).await;
        let request = RawRequest {
            payload: Vec::new(),
            from: client_socket_addr,
        };
        let server_service_binding = sample_udp_service_binding();

        // Act
        let should_discard = Launcher::should_discard_request(
            &request,
            &launcher.udp_tracker_core_container,
            &launcher.udp_tracker_server_container,
            &server_service_binding,
            TEST_LOG_TARGET,
            torrust_tracker_udp_core::ConnectionIdValidationPolicy::Strict,
        )
        .await;

        // Assert
        assert!(should_discard);
    }

    #[tokio::test]
    async fn it_should_publish_a_request_banned_event_when_its_client_ip_is_banned_in_strict_mode() {
        // Arrange
        let client_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)), 8080);
        let launcher = UdpLauncherTestContext::with_banned_client_ip(client_socket_addr.ip()).await;
        let request = RawRequest {
            payload: Vec::new(),
            from: client_socket_addr,
        };
        let server_service_binding = sample_udp_service_binding();
        let mut event_receiver = launcher.udp_tracker_server_container.event_bus.receiver();

        // Act
        let _ = Launcher::should_discard_request(
            &request,
            &launcher.udp_tracker_core_container,
            &launcher.udp_tracker_server_container,
            &server_service_binding,
            TEST_LOG_TARGET,
            torrust_tracker_udp_core::ConnectionIdValidationPolicy::Strict,
        )
        .await;

        // Assert
        assert_eq!(
            tokio::time::timeout(EVENT_PUBLICATION_TIMEOUT, event_receiver.recv())
                .await
                .expect("request-banned event should be published before the test deadline")
                .expect("request-banned event receiver should remain connected"),
            Event::UdpRequestBanned {
                context: ConnectionContext::new(
                    launcher.udp_tracker_core_container.configuration_instance_id,
                    client_socket_addr,
                    server_service_binding,
                ),
            }
        );
    }

    #[tokio::test]
    async fn it_should_publish_a_request_discarded_event_when_its_source_port_is_zero() {
        // Arrange
        let launcher = UdpLauncherTestContext::new().await;
        let client_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)), 0);
        let request = RawRequest {
            payload: Vec::new(),
            from: client_socket_addr,
        };
        let server_service_binding = sample_udp_service_binding();
        let mut event_receiver = launcher.udp_tracker_server_container.event_bus.receiver();

        // Act
        // The source-port-zero guard runs before ban policy is evaluated, so the
        // validation-policy argument is inert for this contract.
        let _ = Launcher::should_discard_request(
            &request,
            &launcher.udp_tracker_core_container,
            &launcher.udp_tracker_server_container,
            &server_service_binding,
            TEST_LOG_TARGET,
            torrust_tracker_udp_core::ConnectionIdValidationPolicy::Strict,
        )
        .await;

        // Assert
        assert_eq!(
            tokio::time::timeout(EVENT_PUBLICATION_TIMEOUT, event_receiver.recv())
                .await
                .expect("request-discarded event should be published before the test deadline")
                .expect("request-discarded event receiver should remain connected"),
            Event::UdpRequestDiscarded {
                context: ConnectionContext::new(
                    launcher.udp_tracker_core_container.configuration_instance_id,
                    client_socket_addr,
                    server_service_binding,
                ),
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
}
