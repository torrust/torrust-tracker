//! UDP tracker error handling.
use std::net::SocketAddr;
use std::ops::Range;

use aquatic_udp_protocol::{ErrorResponse, Response, TransactionId};
use bittorrent_udp_tracker_core::{self, UDP_TRACKER_LOG_TARGET};
use torrust_tracker_primitives::service_binding::ServiceBinding;
use tracing::{instrument, Level};
use uuid::Uuid;
use zerocopy::network_endian::I32;

use crate::error::Error;
use crate::event::{ConnectionContext, Event, UdpRequestKind};

#[allow(clippy::too_many_arguments)]
#[instrument(fields(transaction_id), skip(opt_udp_server_stats_event_sender), ret(level = Level::TRACE))]
pub async fn handle_error(
    req_kind: Option<UdpRequestKind>,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    request_id: Uuid,
    opt_udp_server_stats_event_sender: &crate::event::sender::Sender,
    cookie_valid_range: Range<f64>,
    error: &Error,
    opt_transaction_id: Option<TransactionId>,
) -> Response {
    tracing::trace!("handle error");

    let server_socket_addr = server_service_binding.bind_address();

    log_error(error, client_socket_addr, server_socket_addr, opt_transaction_id, request_id);

    trigger_udp_error_event(
        error,
        client_socket_addr,
        server_service_binding,
        opt_udp_server_stats_event_sender,
        req_kind,
    )
    .await;

    Response::from(ErrorResponse {
        transaction_id: opt_transaction_id.unwrap_or(TransactionId(I32::new(0))),
        message: error.to_string().into(),
    })
}

fn log_error(
    error: &Error,
    client_socket_addr: SocketAddr,
    server_socket_addr: SocketAddr,
    opt_transaction_id: Option<TransactionId>,
    request_id: Uuid,
) {
    match opt_transaction_id {
        Some(transaction_id) => {
            let transaction_id = transaction_id.0.to_string();
            tracing::error!(target: UDP_TRACKER_LOG_TARGET, error = %error, %client_socket_addr, %server_socket_addr, %request_id, %transaction_id, "response error");
        }
        None => {
            tracing::error!(target: UDP_TRACKER_LOG_TARGET, error = %error, %client_socket_addr, %server_socket_addr, %request_id, "response error");
        }
    }
}

async fn trigger_udp_error_event(
    error: &Error,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    opt_udp_server_stats_event_sender: &crate::event::sender::Sender,
    req_kind: Option<UdpRequestKind>,
) {
    if let Some(udp_server_stats_event_sender) = opt_udp_server_stats_event_sender.as_deref() {
        udp_server_stats_event_sender
            .send(Event::UdpError {
                context: ConnectionContext::new(client_socket_addr, server_service_binding),
                kind: req_kind,
                error: error.clone().into(),
            })
            .await;
    }
}
