//! UDP tracker connect handler.
use std::net::SocketAddr;
use std::sync::Arc;

use aquatic_udp_protocol::{ConnectRequest, ConnectResponse, ConnectionId, Response};
use bittorrent_udp_tracker_core::services::connect::ConnectService;
use torrust_tracker_primitives::service_binding::ServiceBinding;
use tracing::{instrument, Level};

use crate::event::{self, ConnectionContext, Event, UdpRequestKind};

/// It handles the `Connect` request.
#[instrument(fields(transaction_id), skip(connect_service, opt_udp_server_stats_event_sender), ret(level = Level::TRACE))]
pub async fn handle_connect(
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    request: &ConnectRequest,
    connect_service: &Arc<ConnectService>,
    opt_udp_server_stats_event_sender: &Arc<Option<Box<dyn event::sender::Sender>>>,
    cookie_issue_time: f64,
) -> Response {
    tracing::Span::current().record("transaction_id", request.transaction_id.0.to_string());
    tracing::trace!("handle connect");

    if let Some(udp_server_stats_event_sender) = opt_udp_server_stats_event_sender.as_deref() {
        udp_server_stats_event_sender
            .send_event(Event::UdpRequestAccepted {
                context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                kind: UdpRequestKind::Connect,
            })
            .await;
    }

    let connection_id = connect_service
        .handle_connect(client_socket_addr, server_service_binding, cookie_issue_time)
        .await;

    build_response(*request, connection_id)
}

fn build_response(request: ConnectRequest, connection_id: ConnectionId) -> Response {
    let response = ConnectResponse {
        transaction_id: request.transaction_id,
        connection_id,
    };

    Response::from(response)
}

#[cfg(test)]
mod tests {

    mod connect_request {

        use std::future;
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::sync::Arc;

        use aquatic_udp_protocol::{ConnectRequest, ConnectResponse, Response, TransactionId};
        use bittorrent_udp_tracker_core::connection_cookie::make;
        use bittorrent_udp_tracker_core::event as core_event;
        use bittorrent_udp_tracker_core::services::connect::ConnectService;
        use mockall::predicate::eq;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

        use crate::event::{self, ConnectionContext, Event, UdpRequestKind};
        use crate::handlers::handle_connect;
        use crate::handlers::tests::{
            sample_ipv4_remote_addr, sample_ipv4_remote_addr_fingerprint, sample_ipv4_socket_address, sample_ipv6_remote_addr,
            sample_ipv6_remote_addr_fingerprint, sample_issue_time, MockUdpCoreStatsEventSender, MockUdpServerStatsEventSender,
        };

        fn sample_connect_request() -> ConnectRequest {
            ConnectRequest {
                transaction_id: TransactionId(0i32.into()),
            }
        }

        #[tokio::test]
        async fn a_connect_response_should_contain_the_same_transaction_id_as_the_connect_request() {
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let (udp_core_stats_event_sender, _udp_core_stats_repository) =
                bittorrent_udp_tracker_core::statistics::setup::factory(false);
            let udp_core_stats_event_sender = Arc::new(udp_core_stats_event_sender);

            let (udp_server_stats_event_sender, _udp_server_stats_repository) = crate::statistics::setup::factory(false);
            let udp_server_stats_event_sender = Arc::new(udp_server_stats_event_sender);

            let request = ConnectRequest {
                transaction_id: TransactionId(0i32.into()),
            };

            let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));

            let response = handle_connect(
                sample_ipv4_remote_addr(),
                server_service_binding,
                &request,
                &connect_service,
                &udp_server_stats_event_sender,
                sample_issue_time(),
            )
            .await;

            assert_eq!(
                response,
                Response::Connect(ConnectResponse {
                    connection_id: make(sample_ipv4_remote_addr_fingerprint(), sample_issue_time()).unwrap(),
                    transaction_id: request.transaction_id
                })
            );
        }

        #[tokio::test]
        async fn a_connect_response_should_contain_a_new_connection_id() {
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let (udp_core_stats_event_sender, _udp_core_stats_repository) =
                bittorrent_udp_tracker_core::statistics::setup::factory(false);
            let udp_core_stats_event_sender = Arc::new(udp_core_stats_event_sender);

            let (udp_server_stats_event_sender, _udp_server_stats_repository) = crate::statistics::setup::factory(false);
            let udp_server_stats_event_sender = Arc::new(udp_server_stats_event_sender);

            let request = ConnectRequest {
                transaction_id: TransactionId(0i32.into()),
            };

            let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));

            let response = handle_connect(
                sample_ipv4_remote_addr(),
                server_service_binding,
                &request,
                &connect_service,
                &udp_server_stats_event_sender,
                sample_issue_time(),
            )
            .await;

            assert_eq!(
                response,
                Response::Connect(ConnectResponse {
                    connection_id: make(sample_ipv4_remote_addr_fingerprint(), sample_issue_time()).unwrap(),
                    transaction_id: request.transaction_id
                })
            );
        }

        #[tokio::test]
        async fn a_connect_response_should_contain_a_new_connection_id_ipv6() {
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let (udp_core_stats_event_sender, _udp_core_stats_repository) =
                bittorrent_udp_tracker_core::statistics::setup::factory(false);
            let udp_core_stats_event_sender = Arc::new(udp_core_stats_event_sender);

            let (udp_server_stats_event_sender, _udp_server_stats_repository) = crate::statistics::setup::factory(false);
            let udp_server_stats_event_sender = Arc::new(udp_server_stats_event_sender);

            let request = ConnectRequest {
                transaction_id: TransactionId(0i32.into()),
            };

            let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));

            let response = handle_connect(
                sample_ipv6_remote_addr(),
                server_service_binding,
                &request,
                &connect_service,
                &udp_server_stats_event_sender,
                sample_issue_time(),
            )
            .await;

            assert_eq!(
                response,
                Response::Connect(ConnectResponse {
                    connection_id: make(sample_ipv6_remote_addr_fingerprint(), sample_issue_time()).unwrap(),
                    transaction_id: request.transaction_id
                })
            );
        }

        #[tokio::test]
        async fn it_should_send_the_upd4_connect_event_when_a_client_tries_to_connect_using_a_ip4_socket_address() {
            let client_socket_addr = sample_ipv4_socket_address();
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let mut udp_core_stats_event_sender_mock = MockUdpCoreStatsEventSender::new();
            udp_core_stats_event_sender_mock
                .expect_send_event()
                .with(eq(core_event::Event::UdpConnect {
                    connection: core_event::ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let udp_core_stats_event_sender: Arc<Option<Box<dyn core_event::sender::Sender>>> =
                Arc::new(Some(Box::new(udp_core_stats_event_sender_mock)));

            let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
            udp_server_stats_event_sender_mock
                .expect_send_event()
                .with(eq(Event::UdpRequestAccepted {
                    context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                    kind: UdpRequestKind::Connect,
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let udp_server_stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>> =
                Arc::new(Some(Box::new(udp_server_stats_event_sender_mock)));

            let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));

            handle_connect(
                client_socket_addr,
                server_service_binding,
                &sample_connect_request(),
                &connect_service,
                &udp_server_stats_event_sender,
                sample_issue_time(),
            )
            .await;
        }

        #[tokio::test]
        async fn it_should_send_the_upd6_connect_event_when_a_client_tries_to_connect_using_a_ip6_socket_address() {
            let client_socket_addr = sample_ipv6_remote_addr();
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let mut udp_core_stats_event_sender_mock = MockUdpCoreStatsEventSender::new();
            udp_core_stats_event_sender_mock
                .expect_send_event()
                .with(eq(core_event::Event::UdpConnect {
                    connection: core_event::ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let udp_core_stats_event_sender: Arc<Option<Box<dyn core_event::sender::Sender>>> =
                Arc::new(Some(Box::new(udp_core_stats_event_sender_mock)));

            let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
            udp_server_stats_event_sender_mock
                .expect_send_event()
                .with(eq(Event::UdpRequestAccepted {
                    context: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                    kind: UdpRequestKind::Connect,
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let udp_server_stats_event_sender: Arc<Option<Box<dyn event::sender::Sender>>> =
                Arc::new(Some(Box::new(udp_server_stats_event_sender_mock)));

            let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));

            handle_connect(
                client_socket_addr,
                server_service_binding,
                &sample_connect_request(),
                &connect_service,
                &udp_server_stats_event_sender,
                sample_issue_time(),
            )
            .await;
        }
    }
}
