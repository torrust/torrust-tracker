//! UDP tracker error handling.
use std::net::SocketAddr;
use std::ops::Range;
use std::sync::Arc;

use aquatic_udp_protocol::{ErrorResponse, RequestParseError, Response, TransactionId};
use bittorrent_udp_tracker_core::connection_cookie::{check, gen_remote_fingerprint};
use bittorrent_udp_tracker_core::{self, UDP_TRACKER_LOG_TARGET};
use torrust_tracker_primitives::service_binding::ServiceBinding;
use tracing::{instrument, Level};
use uuid::Uuid;
use zerocopy::network_endian::I32;

use crate::error::Error;
use crate::event::{self, ConnectionContext, Event, UdpRequestKind};

#[allow(clippy::too_many_arguments)]
#[instrument(fields(transaction_id), skip(opt_udp_server_stats_event_sender), ret(level = Level::TRACE))]
pub async fn handle_error(
    req_kind: Option<UdpRequestKind>,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    request_id: Uuid,
    opt_udp_server_stats_event_sender: &Arc<Option<Box<dyn event::sender::Sender>>>,
    cookie_valid_range: Range<f64>,
    e: &Error,
    transaction_id: Option<TransactionId>,
) -> Response {
    tracing::trace!("handle error");

    let server_socket_addr = server_service_binding.bind_address();

    match transaction_id {
        Some(transaction_id) => {
            let transaction_id = transaction_id.0.to_string();
            tracing::error!(target: UDP_TRACKER_LOG_TARGET, error = %e, %client_socket_addr, %server_socket_addr, %request_id, %transaction_id, "response error");
        }
        None => {
            tracing::error!(target: UDP_TRACKER_LOG_TARGET, error = %e, %client_socket_addr, %server_socket_addr, %request_id, "response error");
        }
    }

    let e = if let Error::RequestParseError { request_parse_error } = e {
        match request_parse_error {
            RequestParseError::Sendable {
                connection_id,
                transaction_id,
                err,
            } => {
                if let Err(e) = check(connection_id, gen_remote_fingerprint(&client_socket_addr), cookie_valid_range) {
                    (e.to_string(), Some(*transaction_id))
                } else {
                    ((*err).to_string(), Some(*transaction_id))
                }
            }
            RequestParseError::Unsendable { err } => (err.to_string(), transaction_id),
        }
    } else {
        (e.to_string(), transaction_id)
    };

    if e.1.is_some() {
        if let Some(udp_server_stats_event_sender) = opt_udp_server_stats_event_sender.as_deref() {
            udp_server_stats_event_sender
                .send_event(Event::UdpError {
                    context: ConnectionContext::new(client_socket_addr, server_service_binding),
                })
                .await;
        }
    }

    Response::from(ErrorResponse {
        transaction_id: e.1.unwrap_or(TransactionId(I32::new(0))),
        message: e.0.into(),
    })
}
