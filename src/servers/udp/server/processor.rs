use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;

use aquatic_udp_protocol::Response;

use super::bound_socket::BoundSocket;
use crate::core::Tracker;
use crate::servers::udp::{handlers, RawRequest, UDP_TRACKER_LOG_TARGET};

pub struct Processor {
    socket: Arc<BoundSocket>,
    tracker: Arc<Tracker>,
}

impl Processor {
    pub fn new(socket: Arc<BoundSocket>, tracker: Arc<Tracker>) -> Self {
        Self { socket, tracker }
    }

    pub async fn process_request(self, request: RawRequest) {
        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, request = %request.from, "Udp::process_request (receiving)");

        let from = request.from;
        let response = handlers::handle_packet(request, &self.tracker, self.socket.address()).await;
        self.send_response(from, response).await;
    }

    async fn send_response(self, to: SocketAddr, response: Response) {
        let response_type = match &response {
            Response::Connect(_) => "Connect".to_string(),
            Response::AnnounceIpv4(_) => "AnnounceIpv4".to_string(),
            Response::AnnounceIpv6(_) => "AnnounceIpv6".to_string(),
            Response::Scrape(_) => "Scrape".to_string(),
            Response::Error(e) => format!("Error: {e:?}"),
        };

        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, target = ?to, response_type,  "Udp::send_response (sending)");

        let mut writer = Cursor::new(Vec::with_capacity(200));

        match response.write_bytes(&mut writer) {
            Ok(()) => {
                let bytes_count = writer.get_ref().len();
                let payload = writer.get_ref();

                tracing::debug!(target: UDP_TRACKER_LOG_TARGET, ?to, bytes_count, "Udp::send_response (sending...)" );
                tracing::trace!(target: UDP_TRACKER_LOG_TARGET, ?to, bytes_count, ?payload, "Udp::send_response (sending...)");

                self.send_packet(&to, payload).await;

                tracing::trace!(target:UDP_TRACKER_LOG_TARGET, ?to, bytes_count, "Udp::send_response (sent)");
            }
            Err(e) => {
                tracing::error!(target: UDP_TRACKER_LOG_TARGET, ?to, response_type, err = %e, "Udp::send_response (error)");
            }
        }
    }

    async fn send_packet(&self, remote_addr: &SocketAddr, payload: &[u8]) {
        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, to = %remote_addr, ?payload, "Udp::send_response (sending)");

        // doesn't matter if it reaches or not
        drop(self.socket.send_to(payload, remote_addr).await);
    }
}
