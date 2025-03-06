//! UDP tracker connect handler.
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use aquatic_udp_protocol::{ConnectRequest, ConnectResponse, ConnectionId, Response};
use bittorrent_udp_tracker_core::services::connect::ConnectService;
use tracing::{instrument, Level};

use crate::statistics as server_statistics;
use crate::statistics::event::UdpResponseKind;

/// It handles the `Connect` request.
#[instrument(fields(transaction_id), skip(connect_service, opt_udp_server_stats_event_sender), ret(level = Level::TRACE))]
pub async fn handle_connect(
    remote_addr: SocketAddr,
    request: &ConnectRequest,
    connect_service: &Arc<ConnectService>,
    opt_udp_server_stats_event_sender: &Arc<Option<Box<dyn server_statistics::event::sender::Sender>>>,
    cookie_issue_time: f64,
) -> Response {
    tracing::Span::current().record("transaction_id", request.transaction_id.0.to_string());
    tracing::trace!("handle connect");

    if let Some(udp_server_stats_event_sender) = opt_udp_server_stats_event_sender.as_deref() {
        match remote_addr.ip() {
            IpAddr::V4(_) => {
                udp_server_stats_event_sender
                    .send_event(server_statistics::event::Event::Udp4Request {
                        kind: UdpResponseKind::Connect,
                    })
                    .await;
            }
            IpAddr::V6(_) => {
                udp_server_stats_event_sender
                    .send_event(server_statistics::event::Event::Udp6Request {
                        kind: UdpResponseKind::Connect,
                    })
                    .await;
            }
        }
    }

    let connection_id = connect_service.handle_connect(remote_addr, cookie_issue_time).await;

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
        use std::sync::Arc;

        use aquatic_udp_protocol::{ConnectRequest, ConnectResponse, Response, TransactionId};
        use bittorrent_udp_tracker_core::connection_cookie::make;
        use bittorrent_udp_tracker_core::services::connect::ConnectService;
        use bittorrent_udp_tracker_core::statistics as core_statistics;
        use mockall::predicate::eq;

        use crate::handlers::handle_connect;
        use crate::handlers::tests::{
            sample_ipv4_remote_addr, sample_ipv4_remote_addr_fingerprint, sample_ipv4_socket_address, sample_ipv6_remote_addr,
            sample_ipv6_remote_addr_fingerprint, sample_issue_time, MockUdpCoreStatsEventSender, MockUdpServerStatsEventSender,
        };
        use crate::statistics as server_statistics;
        use crate::statistics::event::UdpResponseKind;

        fn sample_connect_request() -> ConnectRequest {
            ConnectRequest {
                transaction_id: TransactionId(0i32.into()),
            }
        }

        #[tokio::test]
        async fn a_connect_response_should_contain_the_same_transaction_id_as_the_connect_request() {
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
            let mut udp_core_stats_event_sender_mock = MockUdpCoreStatsEventSender::new();
            udp_core_stats_event_sender_mock
                .expect_send_event()
                .with(eq(core_statistics::event::Event::Udp4Connect))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(())))));
            let udp_core_stats_event_sender: Arc<Option<Box<dyn core_statistics::event::sender::Sender>>> =
                Arc::new(Some(Box::new(udp_core_stats_event_sender_mock)));

            let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
            udp_server_stats_event_sender_mock
                .expect_send_event()
                .with(eq(server_statistics::event::Event::Udp4Request {
                    kind: UdpResponseKind::Connect,
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(())))));
            let udp_server_stats_event_sender: Arc<Option<Box<dyn server_statistics::event::sender::Sender>>> =
                Arc::new(Some(Box::new(udp_server_stats_event_sender_mock)));

            let client_socket_address = sample_ipv4_socket_address();

            let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));

            handle_connect(
                client_socket_address,
                &sample_connect_request(),
                &connect_service,
                &udp_server_stats_event_sender,
                sample_issue_time(),
            )
            .await;
        }

        #[tokio::test]
        async fn it_should_send_the_upd6_connect_event_when_a_client_tries_to_connect_using_a_ip6_socket_address() {
            let mut udp_core_stats_event_sender_mock = MockUdpCoreStatsEventSender::new();
            udp_core_stats_event_sender_mock
                .expect_send_event()
                .with(eq(core_statistics::event::Event::Udp6Connect))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(())))));
            let udp_core_stats_event_sender: Arc<Option<Box<dyn core_statistics::event::sender::Sender>>> =
                Arc::new(Some(Box::new(udp_core_stats_event_sender_mock)));

            let mut udp_server_stats_event_sender_mock = MockUdpServerStatsEventSender::new();
            udp_server_stats_event_sender_mock
                .expect_send_event()
                .with(eq(server_statistics::event::Event::Udp6Request {
                    kind: UdpResponseKind::Connect,
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(())))));
            let udp_server_stats_event_sender: Arc<Option<Box<dyn server_statistics::event::sender::Sender>>> =
                Arc::new(Some(Box::new(udp_server_stats_event_sender_mock)));

            let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));

            handle_connect(
                sample_ipv6_remote_addr(),
                &sample_connect_request(),
                &connect_service,
                &udp_server_stats_event_sender,
                sample_issue_time(),
            )
            .await;
        }
    }
}
