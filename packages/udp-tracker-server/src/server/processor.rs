use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use aquatic_udp_protocol::Response;
use bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer;
use bittorrent_udp_tracker_core::{self};
use tokio::time::Instant;
use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};
use tracing::{instrument, Level};

use super::bound_socket::BoundSocket;
use crate::container::UdpTrackerServerContainer;
use crate::event::{self, ConnectionContext, Event, UdpRequestKind};
use crate::handlers::CookieTimeValues;
use crate::{handlers, RawRequest};

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
