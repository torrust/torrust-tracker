use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::time::Instant;
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_core::event::ConnectionContext;
use torrust_tracker_udp_core::{self, ConnectionIdValidationPolicy};
use torrust_tracker_udp_protocol::Response;
use tracing::{Level, instrument};

use super::bound_socket::BoundSocket;
use crate::container::UdpTrackerServerContainer;
use crate::event::{self, Event, UdpRequestKind};
use crate::handlers::CookieTimeValues;
use crate::{RawRequest, handlers};

pub struct Processor {
    socket: Arc<BoundSocket>,
    udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
    udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
    cookie_lifetime: f64,
    server_service_binding: ServiceBinding,
    connection_id_validation: ConnectionIdValidationPolicy,
}

impl Processor {
    pub fn new(
        socket: Arc<BoundSocket>,
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        cookie_lifetime: f64,
        connection_id_validation: ConnectionIdValidationPolicy,
    ) -> Self {
        // BoundSocket guarantees a non-zero port by construction, so
        // service_binding() cannot fail.
        let server_service_binding = socket.service_binding();

        Self {
            socket,
            udp_tracker_core_container,
            udp_tracker_server_container,
            cookie_lifetime,
            server_service_binding,
            connection_id_validation,
        }
    }

    // issue-spec: docs/issues/drafts/simplify-udp-server-main-loop/ISSUE.md
    #[instrument(skip(self, request))]
    pub async fn process_request(self, request: RawRequest) {
        let client_socket_addr = request.from;

        // Guard: discard requests from clients with port 0.
        //
        // Sending a UDP response to port 0 is rejected by the OS with EINVAL.
        // We discard such requests immediately and record them in statistics so
        // operators can detect scanner activity or misconfigured clients without
        // filling the log with noise.
        //
        // In production the launcher loop already discards port-0 requests
        // before spawning a processing task (so they never enter the
        // active-requests buffer); this guard is kept as defense-in-depth for
        // any other caller of `process_request`.
        if client_socket_addr.port() == 0 {
            tracing::trace!(%client_socket_addr, "discarding request: client source port is 0");

            if let Some(sender) = self.udp_tracker_server_container.stats_event_sender.as_deref() {
                sender
                    .send(Event::UdpRequestDiscarded {
                        context: ConnectionContext::new(
                            self.udp_tracker_core_container.configuration_instance_id,
                            client_socket_addr,
                            self.server_service_binding,
                        ),
                    })
                    .await;
            }

            return;
        }

        let start_time = Instant::now();

        let (response, opt_req_kind) = handlers::handle_packet(
            request,
            self.udp_tracker_core_container.clone(),
            self.udp_tracker_server_container.clone(),
            self.server_service_binding.clone(),
            CookieTimeValues::new(self.cookie_lifetime),
            self.connection_id_validation,
        )
        .await;

        let elapsed_time = start_time.elapsed();

        self.send_response(client_socket_addr, response, opt_req_kind, elapsed_time)
            .await;
    }

    #[instrument(skip(self))]
    async fn send_response(
        self,
        client_socket_addr: SocketAddr,
        response: Response,
        opt_req_kind: Option<UdpRequestKind>,
        req_processing_time: Duration,
    ) {
        tracing::debug!("send response");

        let response_type = match &response {
            Response::Connect(_) => "Connect".to_string(),
            Response::AnnounceIpv4(_) => "AnnounceIpv4".to_string(),
            Response::AnnounceIpv6(_) => "AnnounceIpv6".to_string(),
            Response::Scrape(_) => "Scrape".to_string(),
            Response::Error(e) => format!("Error: {e:?}"),
        };

        let udp_response_kind = match &response {
            Response::Error(_e) => event::UdpResponseKind::Error { opt_req_kind: None },
            _ => {
                if let Some(req_kind) = opt_req_kind {
                    event::UdpResponseKind::Ok { req_kind }
                } else {
                    // code-review: this case should never happen.
                    event::UdpResponseKind::Error { opt_req_kind }
                }
            }
        };

        let mut writer = Cursor::new(Vec::with_capacity(200));

        match response.write_bytes(&mut writer) {
            Ok(()) => {
                let bytes_count = writer.get_ref().len();
                let payload = writer.get_ref();

                let () = match self.send_packet(&client_socket_addr, payload).await {
                    Ok(sent_bytes) => {
                        if tracing::event_enabled!(Level::TRACE) {
                            tracing::debug!(%bytes_count, %sent_bytes, ?payload, "sent {response_type}");
                        } else {
                            tracing::debug!(%bytes_count, %sent_bytes, "sent {response_type}");
                        }

                        if let Some(udp_server_stats_event_sender) =
                            self.udp_tracker_server_container.stats_event_sender.as_deref()
                        {
                            udp_server_stats_event_sender
                                .send(Event::UdpResponseSent {
                                    context: ConnectionContext::new(
                                        self.udp_tracker_core_container.configuration_instance_id,
                                        client_socket_addr,
                                        self.server_service_binding,
                                    ),
                                    kind: udp_response_kind,
                                    req_processing_time,
                                })
                                .await;
                        }
                    }
                    Err(error) => tracing::warn!(%bytes_count, %error, ?payload, "failed to send"),
                };
            }
            Err(e) => {
                tracing::error!(%e, "error");
            }
        }
    }

    #[instrument(skip(self))]
    async fn send_packet(&self, target: &SocketAddr, payload: &[u8]) -> std::io::Result<usize> {
        tracing::trace!("send packet");

        // doesn't matter if it reaches or not
        self.socket.send_to(payload, target).await
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;
    use std::time::Duration;

    use torrust_tracker_test_helpers::configuration;
    use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
    use torrust_tracker_udp_protocol::{ConnectRequest, Request, TransactionId};

    use crate::RawRequest;
    use crate::event::Event;
    use crate::event::receiver::Receiver;
    use crate::server::bound_socket::BoundSocket;
    use crate::server::processor::Processor;
    use crate::testing::environment::EnvContainer;

    const EVENT_PUBLICATION_TIMEOUT: Duration = Duration::from_secs(1);

    // -----------------------------------------------------------------------
    // Test helpers
    // -----------------------------------------------------------------------

    /// Builds a raw request carrying a valid UDP connect payload.
    ///
    /// The test uses a parsable payload on purpose: if the discard guard
    /// regresses (for example, by moving after packet handling), the expected
    /// discard event is not the direct outcome of this processor boundary.
    fn connect_request_from(addr: SocketAddr) -> RawRequest {
        let connect_request = Request::from(ConnectRequest {
            transaction_id: TransactionId(0i32.into()),
        });

        let mut payload = Vec::new();
        connect_request
            .write_bytes(&mut payload)
            .expect("a valid connect request should serialize");

        RawRequest { payload, from: addr }
    }

    /// Creates an ephemeral tracker environment and returns a ready-to-use
    /// `Processor` with a direct receiver for its server events.
    ///
    /// The caller receives:
    /// - `processor` — consumes itself in `process_request`.
    /// - `event_receiver` — observes the processor's emitted server events.
    async fn setup_processor_with_event_receiver() -> (Processor, Receiver) {
        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());

        let container = Arc::new(
            EnvContainer::initialize(
                &core_config,
                &udp_tracker_config,
                cfg.udp_tracker_server.max_connection_id_errors_per_ip,
            )
            .await,
        );

        let event_receiver = container.udp_tracker_server_container.event_bus.receiver();

        let socket = Arc::new(BoundSocket::bind("0.0.0.0:0".parse().unwrap(), false).expect("Failed to bind socket"));
        let processor = Processor::new(
            socket,
            container.udp_tracker_core_container.clone(),
            container.udp_tracker_server_container.clone(),
            udp_tracker_config.cookie_lifetime.as_secs_f64(),
            ConnectionIdValidationPolicy::Strict,
        );

        (processor, event_receiver)
    }

    async fn receive_event(event_receiver: &mut Receiver) -> Event {
        tokio::time::timeout(EVENT_PUBLICATION_TIMEOUT, event_receiver.recv())
            .await
            .expect("processor should publish an event before the test deadline")
            .expect("event receiver should remain connected")
    }

    // -----------------------------------------------------------------------
    // Tests
    // -----------------------------------------------------------------------

    /// Scenario: the tracker receives a UDP request whose source port is 0.
    ///
    /// The processor must emit `Event::UdpRequestDiscarded` so that the stats
    /// counter increments. This gives operators a clean signal (via the REST
    /// stats endpoint) to detect scanner activity or abuse without relying on
    /// log noise.
    #[tokio::test]
    async fn it_should_publish_a_discard_event_when_a_client_uses_port_zero() {
        // Arrange
        let (processor, mut event_receiver) = setup_processor_with_event_receiver().await;
        let client_with_port_0 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)), 0);

        // Act
        processor.process_request(connect_request_from(client_with_port_0)).await;

        // Assert
        assert!(matches!(
            receive_event(&mut event_receiver).await,
            Event::UdpRequestDiscarded { .. }
        ));
    }
}
