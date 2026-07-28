use std::net::SocketAddr;
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
use crate::server::bound_socket::BoundSocket;
use crate::server::processor::Processor;
use crate::server::receiver::Receiver;

const TYPE_STRING: &str = "udp_tracker";
/// A UDP server instance launcher.
#[derive(Constructor)]
pub struct Launcher;

impl Launcher {
    /// It starts the UDP server instance with graceful shutdown.
    ///
    /// # Panics
    ///
    /// It panics if unable to bind to udp socket, and get the address from the udp socket.
    /// It panics if unable to send address of socket.
    /// It panics if the udp server is loaded when the tracker is private.
    #[instrument(skip(udp_tracker_core_container, udp_tracker_server_container, bind_to, tx_start, rx_halt))]
    pub async fn run_with_graceful_shutdown(
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        bind_to: SocketAddr,
        cookie_lifetime: Duration,
        connection_id_validation: ConnectionIdValidationPolicy,
        tx_start: oneshot::Sender<Started>,
        rx_halt: oneshot::Receiver<Halted>,
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

        if udp_tracker_core_container.tracker_core_container.core_config.private {
            tracing::error!("udp services cannot be used for private trackers");
            panic!("it should not use udp if using authentication");
        }

        let socket = BoundSocket::bind(bind_to, udp_tracker_core_container.udp_tracker_config.ipv6_v6only);

        let bound_socket = match socket {
            Ok(socket) => socket,
            Err(e) => {
                tracing::error!(target: UDP_TRACKER_LOG_TARGET, addr = %bind_to, err = %e, "Udp::run_with_graceful_shutdown panic! (error when building socket)" );
                panic!("could not bind to socket!");
            }
        };

        let service_binding = bound_socket.service_binding().clone();
        let address = bound_socket.address();
        let local_udp_url = bound_socket.url().to_string();

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "{STARTED_ON}: {local_udp_url}");

        let receiver = Receiver::new(bound_socket.into());

        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (spawning main loop)");

        let running = {
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

        tx_start
            .send(Started {
                service_binding,
                address,
            })
            .expect("the UDP Tracker service should not be dropped");

        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (started)");

        let stop = running.abort_handle();

        let halt_task = tokio::task::spawn(shutdown_signal_with_message(
            rx_halt,
            format!("Halting UDP Service Bound to Socket: {address}"),
        ));

        select! {
            _ = running => { tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (stopped)"); },
            _ = halt_task => { tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_udp_url, "Udp::run_with_graceful_shutdown (halting)"); }
        }
        stop.abort();

        tokio::task::yield_now().await; // lets allow the other threads to complete.
    }

    #[must_use]
    #[instrument(skip(service_binding))]
    pub fn check(service_binding: &ServiceBinding) -> ServiceHealthCheckJob {
        let info = format!("checking the udp tracker health check at: {}", service_binding.bind_address());

        let service_binding_clone = service_binding.clone();

        let job = tokio::spawn(async move { check(&service_binding_clone).await });

        ServiceHealthCheckJob::new(service_binding.clone(), info, TYPE_STRING.to_string(), job)
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

                if let Some(udp_server_stats_event_sender) = udp_tracker_server_container.stats_event_sender.as_deref() {
                    udp_server_stats_event_sender
                        .send(Event::UdpRequestReceived {
                            context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                        })
                        .await;
                }

                // Discard requests from clients with source port 0 before
                // spawning a processing task. Responses to port 0 are rejected
                // by the OS with EINVAL, so processing them wastes resources
                // and — worse — pushing them into the active-requests buffer
                // could evict legitimate in-flight requests under a port-0
                // flood. See also the defensive guard in
                // `Processor::process_request`.
                if client_socket_addr.port() == 0 {
                    tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_addr, %client_socket_addr, "Udp::run_udp_server::loop continue: (discarded: client source port is 0)");

                    if let Some(udp_server_stats_event_sender) = udp_tracker_server_container.stats_event_sender.as_deref() {
                        udp_server_stats_event_sender
                            .send(Event::UdpRequestDiscarded {
                                context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                            })
                            .await;
                    }

                    continue;
                }

                // When connection ID validation is disabled, the tracker is
                // intentionally accepting requests with invalid or arbitrary
                // connection IDs. Enforcing IP bans in that mode is
                // contradictory — the banning listener still counts invalid
                // cookies for observability, but the ban is not acted upon.
                let ban_enforcement_active = connection_id_validation == ConnectionIdValidationPolicy::Strict;

                if ban_enforcement_active && udp_tracker_core_container.ban_service.read().await.is_banned(&req.from.ip()) {
                    tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr,  "Udp::run_udp_server::loop continue: (banned ip)");

                    if let Some(udp_server_stats_event_sender) = udp_tracker_server_container.stats_event_sender.as_deref() {
                        udp_server_stats_event_sender
                            .send(Event::UdpRequestBanned {
                                context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                            })
                            .await;
                    }

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

                    if let Some(udp_server_stats_event_sender) = udp_tracker_server_container.stats_event_sender.as_deref() {
                        udp_server_stats_event_sender
                            .send(Event::UdpRequestAborted {
                                context: ConnectionContext::new(client_socket_addr, server_service_binding),
                            })
                            .await;
                    }
                }
            } else {
                tokio::task::yield_now().await;

                // the request iterator returned `None`.
                tracing::error!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server breaking: (ran dry, should not happen in production!)");
                break;
            }
        }
    }
}
