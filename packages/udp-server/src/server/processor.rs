use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::time::Instant;
use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_core::event::ConnectionContext;
use torrust_tracker_udp_core::{self};
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
}

impl Processor {
    /// # Panics
    ///
    /// It will panic if a bound socket address port is 0. It should never
    /// happen.
    pub fn new(
        socket: Arc<BoundSocket>,
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        cookie_lifetime: f64,
    ) -> Self {
        let server_service_binding =
            ServiceBinding::new(Protocol::UDP, socket.address()).expect("Bound socket port should't be 0");

        Self {
            socket,
            udp_tracker_core_container,
            udp_tracker_server_container,
            cookie_lifetime,
            server_service_binding,
        }
    }

    #[instrument(skip(self, request))]
    pub async fn process_request(self, request: RawRequest) {
        let client_socket_addr = request.from;

        // Guard: discard requests from clients with port 0.
        //
        // Sending a UDP response to port 0 is rejected by the OS with EINVAL.
        // We discard such requests immediately and record them in statistics so
        // operators can detect scanner activity or misconfigured clients without
        // filling the log with noise.
        if client_socket_addr.port() == 0 {
            tracing::trace!(%client_socket_addr, "discarding request: client source port is 0");

            if let Some(sender) = self.udp_tracker_server_container.stats_event_sender.as_deref() {
                sender
                    .send(Event::UdpRequestDiscarded {
                        context: ConnectionContext::new(client_socket_addr, self.server_service_binding),
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
                                    context: ConnectionContext::new(client_socket_addr, self.server_service_binding),
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

    use tokio_util::sync::CancellationToken;
    use torrust_tracker_test_helpers::configuration;

    use crate::RawRequest;
    use crate::server::bound_socket::BoundSocket;
    use crate::server::processor::Processor;
    use crate::statistics::event::listener;
    use crate::testing::environment::EnvContainer;

    fn request_from(addr: SocketAddr) -> RawRequest {
        RawRequest {
            payload: vec![],
            from: addr,
        }
    }

    /// Scenario: the tracker receives a UDP request whose source port is 0.
    ///
    /// A source port of 0 is invalid — the OS rejects `send_to` with EINVAL, so
    /// there is no point in processing or responding to the request. The tracker
    /// should:
    ///
    ///  1. Discard the request immediately (no handlers invoked, no response sent).
    ///  2. Count the discarded request in statistics so operators can detect
    ///     scanner activity or abuse without relying on log noise.
    #[tokio::test]
    async fn udp_tracker_discards_requests_from_clients_with_port_0_and_counts_them_in_statistics() {
        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());

        let container = Arc::new(EnvContainer::initialize(&core_config, &udp_tracker_config).await);

        // Start the stats event listener so that emitted events update the repository.
        let cancellation_token = CancellationToken::new();
        let _event_listener_job = listener::run_event_listener(
            container.udp_tracker_server_container.event_bus.receiver(),
            cancellation_token.clone(),
            &container.udp_tracker_server_container.stats_repository,
        );

        // Create a processor backed by an ephemeral socket.
        let socket = Arc::new(BoundSocket::new("0.0.0.0:0".parse().unwrap(), false).expect("Failed to bind socket"));
        let processor = Processor::new(
            socket,
            container.udp_tracker_core_container.clone(),
            container.udp_tracker_server_container.clone(),
            udp_tracker_config.cookie_lifetime.as_secs_f64(),
        );

        // Submit a request from a client whose source port is 0.
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)), 0);
        processor.process_request(request_from(client_addr)).await;

        // Give the async event listener time to process the emitted event.
        tokio::time::sleep(Duration::from_millis(50)).await;

        // The discarded counter must be 1; all other counters must stay at 0.
        let stats = container.udp_tracker_server_container.stats_repository.get_stats().await;
        assert_eq!(stats.udp_requests_discarded_total(), 1, "expected 1 discarded request");
        assert_eq!(
            stats.udp4_requests_received_total(),
            0,
            "a port-0 request must not count as received"
        );

        cancellation_token.cancel();
    }
}
