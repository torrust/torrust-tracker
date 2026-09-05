//! UDP tracker error handling.
use std::net::SocketAddr;
use std::ops::Range;

use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_primitives::ConfigurationInstanceId;
use torrust_tracker_udp_core::UDP_TRACKER_LOG_TARGET;
use torrust_tracker_udp_core::event::ConnectionContext;
use torrust_tracker_udp_core::services::announce::UdpAnnounceError;
use torrust_tracker_udp_core::services::scrape::UdpScrapeError;
use torrust_tracker_udp_protocol::{ErrorResponse, Response, TransactionId};
use tracing::{Level, instrument};
use uuid::Uuid;
use zerocopy::byteorder::network_endian::I32;

use crate::error::Error;
use crate::event::{Event, UdpRequestKind};

#[allow(clippy::too_many_arguments)]
#[instrument(fields(transaction_id), skip(opt_udp_server_stats_event_sender), ret(level = Level::TRACE))]
pub async fn handle_error(
    req_kind: Option<UdpRequestKind>,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    configuration_instance_id: ConfigurationInstanceId,
    public_url: Option<String>,
    request_id: Uuid,
    opt_udp_server_stats_event_sender: &crate::event::sender::Sender,
    cookie_valid_range: Range<f64>,
    error: &Error,
    opt_transaction_id: Option<TransactionId>,
) -> Response {
    tracing::trace!("handle error");

    log_error(
        error,
        client_socket_addr,
        &server_service_binding,
        opt_transaction_id,
        request_id,
    );

    trigger_udp_error_event(
        error,
        client_socket_addr,
        server_service_binding,
        configuration_instance_id,
        public_url,
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
    server_service_binding: &ServiceBinding,
    opt_transaction_id: Option<TransactionId>,
    request_id: Uuid,
) {
    let server_socket_addr = server_service_binding.bind_address();

    if is_connection_cookie_error(error) {
        log_connection_cookie_error(
            error,
            client_socket_addr,
            server_service_binding,
            server_socket_addr,
            opt_transaction_id,
            request_id,
        );
    } else {
        log_non_cookie_error(
            error,
            client_socket_addr,
            server_service_binding,
            server_socket_addr,
            opt_transaction_id,
            request_id,
        );
    }
}

fn log_connection_cookie_error(
    error: &Error,
    client_socket_addr: SocketAddr,
    server_service_binding: &ServiceBinding,
    server_socket_addr: SocketAddr,
    opt_transaction_id: Option<TransactionId>,
    request_id: Uuid,
) {
    if let Some(transaction_id) = opt_transaction_id {
        let transaction_id = transaction_id.0.to_string();
        tracing::warn!(target: UDP_TRACKER_LOG_TARGET, error = %error, %client_socket_addr, %server_socket_addr, service_binding = %server_service_binding, %request_id, %transaction_id, "response error");
    } else {
        tracing::warn!(target: UDP_TRACKER_LOG_TARGET, error = %error, %client_socket_addr, %server_socket_addr, service_binding = %server_service_binding, %request_id, "response error");
    }
}

fn log_non_cookie_error(
    error: &Error,
    client_socket_addr: SocketAddr,
    server_service_binding: &ServiceBinding,
    server_socket_addr: SocketAddr,
    opt_transaction_id: Option<TransactionId>,
    request_id: Uuid,
) {
    if let Some(transaction_id) = opt_transaction_id {
        let transaction_id = transaction_id.0.to_string();
        tracing::error!(target: UDP_TRACKER_LOG_TARGET, error = %error, %client_socket_addr, %server_socket_addr, service_binding = %server_service_binding, %request_id, %transaction_id, "response error");
    } else {
        tracing::error!(target: UDP_TRACKER_LOG_TARGET, error = %error, %client_socket_addr, %server_socket_addr, service_binding = %server_service_binding, %request_id, "response error");
    }
}

const fn is_connection_cookie_error(error: &Error) -> bool {
    matches!(
        error,
        Error::AnnounceFailed {
            source: UdpAnnounceError::ConnectionCookieError { .. }
        } | Error::ScrapeFailed {
            source: UdpScrapeError::ConnectionCookieError { .. }
        }
    )
}

async fn trigger_udp_error_event(
    error: &Error,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    configuration_instance_id: ConfigurationInstanceId,
    public_url: Option<String>,
    opt_udp_server_stats_event_sender: &crate::event::sender::Sender,
    req_kind: Option<UdpRequestKind>,
) {
    if let Some(udp_server_stats_event_sender) = opt_udp_server_stats_event_sender.as_deref() {
        udp_server_stats_event_sender
            .send(Event::UdpError {
                context: ConnectionContext::new(configuration_instance_id, client_socket_addr, server_service_binding)
                    .with_public_url(public_url),
                kind: req_kind,
                error: error.clone().into(),
            })
            .await;
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_udp_protocol::{ErrorResponse, Response, TransactionId};
    use uuid::Uuid;
    use zerocopy::byteorder::network_endian::I32;

    use super::handle_error;
    use crate::error::Error;
    use crate::event::{ErrorKind, Event, UdpRequestKind};

    fn service_binding() -> ServiceBinding {
        ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap()
    }

    fn internal_error() -> Error {
        Error::Internal {
            location: std::panic::Location::caller(),
            message: "failure".into(),
        }
    }

    #[tokio::test]
    async fn it_should_publish_the_exact_error_with_the_supplied_transaction_id() {
        // Arrange
        let broadcaster = crate::event::sender::Broadcaster::default();
        let mut receiver = broadcaster.subscribe();
        let sender = Some(Arc::new(broadcaster) as Arc<dyn torrust_tracker_events::sender::Sender<Event = Event>>);
        let transaction_id = TransactionId(I32::new(42));
        let error = internal_error();

        // Act
        let response = handle_error(
            Some(UdpRequestKind::Connect),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            service_binding(),
            ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
            None,
            Uuid::nil(),
            &sender,
            0.0..1.0,
            &error,
            Some(transaction_id),
        )
        .await;

        // Assert
        assert!(matches!(response, Response::Error(ErrorResponse { transaction_id: actual, .. }) if actual == transaction_id));
        assert!(matches!(
            receiver.recv().await.unwrap(),
            Event::UdpError { kind: Some(UdpRequestKind::Connect), error: ErrorKind::InternalServer(message), .. } if message == "failure"
        ));
    }

    #[tokio::test]
    async fn it_should_return_a_zero_transaction_id_without_an_event_sender() {
        // Arrange
        let sender = None;
        let error = internal_error();

        // Act
        let response = handle_error(
            None,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            service_binding(),
            ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
            None,
            Uuid::nil(),
            &sender,
            0.0..1.0,
            &error,
            None,
        )
        .await;

        // Assert
        assert!(
            matches!(response, Response::Error(ErrorResponse { transaction_id: TransactionId(value), .. }) if value == I32::new(0))
        );
    }
}
