//! The `connect` service.
//!
//! The service is responsible for handling the `connect` requests.
use std::net::SocketAddr;
use std::sync::Arc;

use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_primitives::ConfigurationInstanceId;
use torrust_tracker_udp_protocol::ConnectionId;

use crate::connection_cookie::{gen_remote_fingerprint, make};
use crate::crypto::cookie_cipher::CookieCipher;
use crate::event::{ConnectionContext, Event};

/// The `ConnectService` is responsible for handling the `connect` requests.
///
/// It is responsible for generating the connection cookie and sending the
/// appropriate statistics events.
pub struct ConnectService {
    cookie_cipher: Arc<CookieCipher>,
    pub opt_udp_core_stats_event_sender: crate::event::sender::Sender,
    configuration_instance_id: ConfigurationInstanceId,
    public_url: Option<String>,
}

impl ConnectService {
    #[must_use]
    pub const fn configuration_instance_id(&self) -> ConfigurationInstanceId {
        self.configuration_instance_id
    }

    /// Creates the service. `cookie_cipher` must be the composition root's
    /// shared key, so the services that validate connection IDs accept the
    /// ones this service issues.
    #[must_use]
    pub fn new(
        cookie_cipher: Arc<CookieCipher>,
        opt_udp_core_stats_event_sender: crate::event::sender::Sender,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Self {
        Self {
            cookie_cipher,
            opt_udp_core_stats_event_sender,
            configuration_instance_id,
            public_url: None,
        }
    }

    #[must_use]
    pub fn with_public_url(mut self, public_url: Option<String>) -> Self {
        self.public_url = public_url;
        self
    }

    #[must_use]
    pub fn public_url(&self) -> Option<&str> {
        self.public_url.as_deref()
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
        let connection_id = make(
            &self.cookie_cipher,
            gen_remote_fingerprint(&client_socket_addr),
            cookie_issue_time,
        )
        .expect("it should be a normal value");

        if let Some(udp_stats_event_sender) = self.opt_udp_core_stats_event_sender.as_deref() {
            udp_stats_event_sender
                .send(Event::UdpConnect {
                    connection: ConnectionContext::new(
                        self.configuration_instance_id,
                        client_socket_addr,
                        server_service_binding,
                    )
                    .with_public_url(self.public_url.clone()),
                })
                .await;
        }

        connection_id
    }
}

#[cfg(test)]
mod tests {
    use std::future;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use mockall::predicate::eq;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

    use super::ConnectService;
    use crate::connection_cookie::{check, gen_remote_fingerprint};
    use crate::crypto::cookie_cipher::CookieCipher;
    use crate::event::{ConnectionContext, Event};
    use crate::services::tests::{MockUdpCoreStatsEventSender, sample_ipv4_remote_addr, sample_issue_time};

    const INSTANCE_ID: ConfigurationInstanceId = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
    const PUBLIC_URL: &str = "udp://tracker.example.test:6969/announce";

    fn fixed_test_cipher() -> Arc<CookieCipher> {
        Arc::new(CookieCipher::fixed_for_testing())
    }

    fn server_service_binding() -> ServiceBinding {
        ServiceBinding::new(
            Protocol::UDP,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn it_should_issue_a_connection_id_that_validates_for_the_client_at_the_issue_time() {
        // Arrange
        let cookie_cipher = fixed_test_cipher();
        let connect_service = ConnectService::new(cookie_cipher.clone(), None, INSTANCE_ID);
        let issue_time = sample_issue_time();

        // Act
        let connection_id = connect_service
            .handle_connect(sample_ipv4_remote_addr(), server_service_binding(), issue_time)
            .await;

        // Assert
        assert_eq!(
            check(
                &cookie_cipher,
                &connection_id,
                gen_remote_fingerprint(&sample_ipv4_remote_addr()),
                issue_time..issue_time
            ),
            Ok(issue_time)
        );
    }

    #[tokio::test]
    async fn it_should_publish_a_connect_event_with_the_connection_context() {
        // Arrange
        let expected_event = Event::UdpConnect {
            connection: ConnectionContext::new(INSTANCE_ID, sample_ipv4_remote_addr(), server_service_binding())
                .with_public_url(Some(PUBLIC_URL.to_string())),
        };
        let mut event_sender = MockUdpCoreStatsEventSender::new();
        event_sender
            .expect_send()
            .with(eq(expected_event))
            .times(1)
            .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
        let connect_service = ConnectService::new(fixed_test_cipher(), Some(Arc::new(event_sender)), INSTANCE_ID)
            .with_public_url(Some(PUBLIC_URL.to_string()));

        // Act
        connect_service
            .handle_connect(sample_ipv4_remote_addr(), server_service_binding(), sample_issue_time())
            .await;

        // Assert: the mock verifies the single expected event when it is dropped.
    }

    #[tokio::test]
    #[should_panic(expected = "it should be a normal value")]
    async fn it_should_panic_when_the_issue_time_is_not_a_normal_value() {
        // Arrange
        let connect_service = ConnectService::new(fixed_test_cipher(), None, INSTANCE_ID);

        // Act
        connect_service
            .handle_connect(sample_ipv4_remote_addr(), server_service_binding(), 0.0)
            .await;
    }
}
