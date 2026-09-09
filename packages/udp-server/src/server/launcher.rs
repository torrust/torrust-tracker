use std::sync::Arc;
use std::time::Duration;

use derive_more::Constructor;
use futures_util::StreamExt;
use tokio::select;
use tokio::sync::oneshot;
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
use crate::container::UdpTrackerServerContainer;
use crate::event::Event;
use crate::event::sender::Sender;
use crate::server::bound_socket::BoundSocket;
use crate::server::processor::Processor;
use crate::server::receiver::Receiver;

/// A UDP server instance launcher.
#[derive(Constructor)]
pub struct Launcher;

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
        let bind_to = bound_socket.address();
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

        let service_binding = bound_socket.service_binding();
        let address = bound_socket.address();
        let local_udp_url = bound_socket.url().to_string();

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "{STARTED_ON}: {local_udp_url}");

        let receiver = Receiver::new(bound_socket.into());

        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (spawning main loop)");

        let mut running = {
            let local_addr = local_udp_url.clone();
            tokio::task::spawn(async move {
                tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_with_graceful_shutdown::task (listening...)");
                let () = Self::run_udp_server_main(
                    receiver,
                    udp_tracker_core_container,
                    udp_tracker_server_container,
                    cookie_lifetime,
                    connection_id_validation,
                )
                .await;
            })
        };

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

    #[must_use]
    #[instrument(skip(service_binding))]
    pub fn check(service_binding: &ServiceBinding) -> ServiceHealthCheckJob {
        let info = format!("checking the udp tracker health check at: {}", service_binding.bind_address());

        let service_binding_clone = service_binding.clone();

        let job = tokio::spawn(async move { check(&service_binding_clone).await });

        ServiceHealthCheckJob::new(info, job)
    }

    // issue-spec: docs/issues/drafts/simplify-udp-server-main-loop.md
    #[instrument(skip(receiver, udp_tracker_core_container, udp_tracker_server_container))]
    async fn run_udp_server_main(
        mut receiver: Receiver,
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        cookie_lifetime: Duration,
        connection_id_validation: ConnectionIdValidationPolicy,
    ) {
        let active_requests = &mut ActiveRequests::default();

        let server_socket_addr = receiver.bound_socket_address();

        let server_service_binding =
            ServiceBinding::new(Protocol::UDP, server_socket_addr).expect("Bound socket to service binding should not fail");

        let local_addr = server_service_binding.clone().to_string();

        let cookie_lifetime = cookie_lifetime.as_secs_f64();

        loop {
            let server_service_binding =
                ServiceBinding::new(Protocol::UDP, server_socket_addr).expect("Bound socket to service binding should not fail");

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
            } else {
                tokio::task::yield_now().await;

                // the request iterator returned `None`.
                tracing::error!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server breaking: (ran dry, should not happen in production!)");
                break;
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

    struct UdpLauncherDependencies {
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        cookie_lifetime: Duration,
        bind_address: SocketAddr,
    }

    impl UdpLauncherDependencies {
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
            }
        }
    }

    fn sample_udp_service_binding(bind_address: SocketAddr) -> ServiceBinding {
        ServiceBinding::new(Protocol::UDP, SocketAddr::new(bind_address.ip(), 6969))
            .expect("sample UDP service binding should be valid")
    }

    #[tokio::test]
    async fn it_should_release_the_socket_when_the_startup_notification_receiver_is_dropped() {
        // Arrange
        let dependencies = UdpLauncherDependencies::new().await;
        let bound_socket = BoundSocket::bind(dependencies.bind_address, false).expect("UDP socket should bind");
        let bound_address = bound_socket.address();
        let (startup_notification_sender, startup_notification_receiver) = oneshot::channel::<Started>();
        let (_halt_sender, halt_receiver) = oneshot::channel::<Halted>();
        drop(startup_notification_receiver);

        // Act
        let result = Launcher::run_with_graceful_shutdown(
            dependencies.udp_tracker_core_container,
            dependencies.udp_tracker_server_container,
            bound_socket,
            dependencies.cookie_lifetime,
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
    async fn it_should_discard_a_request_when_its_source_port_is_zero() {
        // Arrange
        let dependencies = UdpLauncherDependencies::new().await;
        let client_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)), 0);
        let request = RawRequest {
            payload: Vec::new(),
            from: client_socket_addr,
        };
        let server_service_binding = sample_udp_service_binding(dependencies.bind_address);
        let mut event_receiver = dependencies.udp_tracker_server_container.event_bus.receiver();

        // Act
        let should_discard = Launcher::should_discard_request(
            &request,
            &dependencies.udp_tracker_core_container,
            &dependencies.udp_tracker_server_container,
            &server_service_binding,
            TEST_LOG_TARGET,
            torrust_tracker_udp_core::ConnectionIdValidationPolicy::Strict,
        )
        .await;

        // Assert
        assert!(should_discard);
        // Bound the event await so a publication regression fails diagnostically instead of hanging.
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(1), event_receiver.recv())
                .await
                .expect("request-discarded event should be published before the test deadline")
                .expect("request-discarded event receiver should remain connected"),
            Event::UdpRequestDiscarded {
                context: ConnectionContext::new(
                    dependencies.udp_tracker_core_container.configuration_instance_id,
                    client_socket_addr,
                    server_service_binding,
                ),
            }
        );
    }
}
