//! The `connect` service.
//!
//! The service is responsible for handling the `connect` requests.
use std::net::SocketAddr;

use aquatic_udp_protocol::ConnectionId;
use torrust_tracker_primitives::service_binding::ServiceBinding;

use crate::connection_cookie::{gen_remote_fingerprint, make};
use crate::event::{ConnectionContext, Event};

/// The `ConnectService` is responsible for handling the `connect` requests.
///
/// It is responsible for generating the connection cookie and sending the
/// appropriate statistics events.
pub struct ConnectService {
    pub opt_udp_core_stats_event_sender: crate::event::sender::Sender,
}

impl ConnectService {
    #[must_use]
    pub fn new(opt_udp_core_stats_event_sender: crate::event::sender::Sender) -> Self {
        Self {
            opt_udp_core_stats_event_sender,
        }
    }

    /// Handles a `connect` request.
    ///
    /// # Panics
    ///
    /// It will panic if there was an error making the connection cookie.
    pub async fn handle_connect(
        &self,
        client_socket_addr: SocketAddr,
        server_service_binding: ServiceBinding,
        cookie_issue_time: f64,
    ) -> ConnectionId {
        let connection_id =
            make(gen_remote_fingerprint(&client_socket_addr), cookie_issue_time).expect("it should be a normal value");

        if let Some(udp_stats_event_sender) = self.opt_udp_core_stats_event_sender.as_deref() {
            udp_stats_event_sender
                .send(Event::UdpConnect {
                    connection: ConnectionContext::new(client_socket_addr, server_service_binding),
                })
                .await;
        }

        connection_id
    }
}

#[cfg(test)]
mod tests {

    mod connect_request {

        use std::future;
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::sync::Arc;

        use mockall::predicate::eq;
        use torrust_tracker_events::bus::SenderStatus;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

        use crate::connection_cookie::make;
        use crate::event::bus::EventBus;
        use crate::event::sender::Broadcaster;
        use crate::event::{ConnectionContext, Event};
        use crate::services::connect::ConnectService;
        use crate::services::tests::{
            sample_ipv4_remote_addr, sample_ipv4_remote_addr_fingerprint, sample_ipv4_socket_address, sample_ipv6_remote_addr,
            sample_ipv6_remote_addr_fingerprint, sample_issue_time, MockUdpCoreStatsEventSender,
        };

        #[tokio::test]
        async fn a_connect_response_should_contain_the_same_transaction_id_as_the_connect_request() {
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let udp_core_broadcaster = Broadcaster::default();
            let event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, udp_core_broadcaster.clone()));
            let udp_core_stats_event_sender = event_bus.sender();

            let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));

            let response = connect_service
                .handle_connect(sample_ipv4_remote_addr(), server_service_binding, sample_issue_time())
                .await;

            assert_eq!(
                response,
                make(sample_ipv4_remote_addr_fingerprint(), sample_issue_time()).unwrap()
            );
        }

        #[tokio::test]
        async fn a_connect_response_should_contain_a_new_connection_id() {
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let udp_core_broadcaster = Broadcaster::default();
            let event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, udp_core_broadcaster.clone()));
            let udp_core_stats_event_sender = event_bus.sender();

            let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));

            let response = connect_service
                .handle_connect(sample_ipv4_remote_addr(), server_service_binding, sample_issue_time())
                .await;

            assert_eq!(
                response,
                make(sample_ipv4_remote_addr_fingerprint(), sample_issue_time()).unwrap(),
            );
        }

        #[tokio::test]
        async fn a_connect_response_should_contain_a_new_connection_id_ipv6() {
            let client_socket_addr = sample_ipv6_remote_addr();
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let udp_core_broadcaster = Broadcaster::default();
            let event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, udp_core_broadcaster.clone()));
            let udp_core_stats_event_sender = event_bus.sender();

            let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));

            let response = connect_service
                .handle_connect(client_socket_addr, server_service_binding, sample_issue_time())
                .await;

            assert_eq!(
                response,
                make(sample_ipv6_remote_addr_fingerprint(), sample_issue_time()).unwrap(),
            );
        }

        #[tokio::test]
        async fn it_should_send_the_upd4_connect_event_when_a_client_tries_to_connect_using_a_ip4_socket_address() {
            let client_socket_addr = sample_ipv4_socket_address();
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let mut udp_stats_event_sender_mock = MockUdpCoreStatsEventSender::new();
            udp_stats_event_sender_mock
                .expect_send()
                .with(eq(Event::UdpConnect {
                    connection: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let opt_udp_stats_event_sender: crate::event::sender::Sender = Some(Arc::new(udp_stats_event_sender_mock));

            let connect_service = Arc::new(ConnectService::new(opt_udp_stats_event_sender));

            connect_service
                .handle_connect(client_socket_addr, server_service_binding, sample_issue_time())
                .await;
        }

        #[tokio::test]
        async fn it_should_send_the_upd6_connect_event_when_a_client_tries_to_connect_using_a_ip6_socket_address() {
            let client_socket_addr = sample_ipv6_remote_addr();
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
            let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();

            let mut udp_stats_event_sender_mock = MockUdpCoreStatsEventSender::new();
            udp_stats_event_sender_mock
                .expect_send()
                .with(eq(Event::UdpConnect {
                    connection: ConnectionContext::new(client_socket_addr, server_service_binding.clone()),
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let opt_udp_stats_event_sender: crate::event::sender::Sender = Some(Arc::new(udp_stats_event_sender_mock));

            let connect_service = Arc::new(ConnectService::new(opt_udp_stats_event_sender));

            connect_service
                .handle_connect(client_socket_addr, server_service_binding, sample_issue_time())
                .await;
        }
    }
}
