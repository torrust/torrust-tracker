use std::sync::Arc;

use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::{LabelSet, LabelValue};
use torrust_metrics::{label_name, metric_name};

use crate::event::Event;
use crate::statistics::HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;
use crate::statistics::repository::Repository;

pub async fn handle_event(event: Event, stats_repository: &Arc<Repository>, now: DurationSinceUnixEpoch) {
    let (mut label_set, request_kind) = labels_and_request_kind(event);
    label_set.upsert(label_name!("request_kind"), LabelValue::new(request_kind.as_str()));

    match stats_repository
        .increase_counter(&metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), &label_set, now)
        .await
    {
        Ok(()) => log_counter_increased(request_kind, &label_set),
        Err(err) => tracing::error!("Failed to increase the counter: {}", err),
    }

    tracing::debug!("stats: {:?}", stats_repository.get_stats().await);
}

fn labels_and_request_kind(event: Event) -> (LabelSet, RequestKind) {
    match event {
        Event::TcpAnnounce { connection, .. } => (LabelSet::from(connection), RequestKind::Announce),
        Event::TcpScrape { connection } => (LabelSet::from(connection), RequestKind::Scrape),
    }
}

fn log_counter_increased(request_kind: RequestKind, label_set: &LabelSet) {
    match request_kind {
        RequestKind::Announce => {
            tracing::debug!(
                "Successfully increased the counter for HTTP announce requests received: {}",
                label_set
            );
        }
        RequestKind::Scrape => {
            tracing::debug!(
                "Successfully increased the counter for HTTP scrape requests received: {}",
                label_set
            );
        }
    }
}

#[derive(Clone, Copy)]
enum RequestKind {
    Announce,
    Scrape,
}

impl RequestKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Announce => "announce",
            Self::Scrape => "scrape",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
    use std::sync::Arc;

    use torrust_clock::clock::Time;
    use torrust_metrics::label::{LabelSet, LabelValue};
    use torrust_metrics::metric_collection::aggregate::sum::Sum;
    use torrust_metrics::{label_name, metric_name};
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::{RemoteClientAddr, ResolvedIp};
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

    use crate::CurrentClock;
    use crate::event::{ConnectionContext, Event};
    use crate::statistics::HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::repository::Repository;
    use crate::tests::{sample_info_hash, sample_peer_using_ipv4, sample_peer_using_ipv6};

    #[tokio::test]
    async fn should_increase_the_tcp4_announces_counter_when_it_receives_a_tcp4_announce_event() {
        let http_test_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let stats_repository = Arc::new(Repository::new());
        let peer = sample_peer_using_ipv4();
        let remote_client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2));

        handle_event(
            Event::TcpAnnounce {
                connection: ConnectionContext::new(
                    http_test_configuration_instance_id,
                    RemoteClientAddr::new(ResolvedIp::FromSocketAddr(remote_client_ip), Some(8080)),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
                ),
                info_hash: sample_info_hash(),
                announcement: peer,
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp4_announces_handled(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_tcp4_scrapes_counter_when_it_receives_a_tcp4_scrape_event() {
        let http_test_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let stats_repository = Arc::new(Repository::new());

        handle_event(
            Event::TcpScrape {
                connection: ConnectionContext::new(
                    http_test_configuration_instance_id,
                    RemoteClientAddr::new(
                        ResolvedIp::FromSocketAddr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2))),
                        Some(8080),
                    ),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp4_scrapes_handled(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_tcp6_announces_counter_when_it_receives_a_tcp6_announce_event() {
        let http_test_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let stats_repository = Arc::new(Repository::new());
        let peer = sample_peer_using_ipv6();
        let remote_client_ip = IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969));

        handle_event(
            Event::TcpAnnounce {
                connection: ConnectionContext::new(
                    http_test_configuration_instance_id,
                    RemoteClientAddr::new(ResolvedIp::FromSocketAddr(remote_client_ip), Some(8080)),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 7070)).unwrap(),
                ),
                info_hash: sample_info_hash(),
                announcement: peer,
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp6_announces_handled(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_tcp6_scrapes_counter_when_it_receives_a_tcp6_scrape_event() {
        let http_test_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let stats_repository = Arc::new(Repository::new());

        handle_event(
            Event::TcpScrape {
                connection: ConnectionContext::new(
                    http_test_configuration_instance_id,
                    RemoteClientAddr::new(
                        ResolvedIp::FromSocketAddr(IpAddr::V6(Ipv6Addr::new(
                            0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969,
                        ))),
                        Some(8080),
                    ),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 7070)).unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp6_scrapes_handled(), 1);
    }

    #[tokio::test]
    async fn it_should_propagate_all_connection_labels_and_request_kind() {
        // Arrange
        let stats_repository = Arc::new(Repository::new());
        let event = Event::TcpScrape {
            connection: ConnectionContext::new(
                ConfigurationInstanceId::new(ServiceRole::HttpTracker, 7),
                RemoteClientAddr::new(
                    ResolvedIp::FromSocketAddr(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc000, 0x0201))),
                    Some(49152),
                ),
                ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 7443)).unwrap(),
            )
            .with_public_url(Some("https://tracker.example.test:7443/announce".to_string())),
        };
        let expected_labels = LabelSet::from([
            (label_name!("server_binding_protocol"), LabelValue::new("http")),
            (label_name!("server_binding_ip"), LabelValue::new("::1")),
            (label_name!("server_binding_address_ip_type"), LabelValue::new("plain")),
            (label_name!("server_binding_address_ip_family"), LabelValue::new("inet6")),
            (label_name!("server_binding_port"), LabelValue::new("7443")),
            (label_name!("client_address_ip_family"), LabelValue::new("inet6")),
            (label_name!("client_address_ip_type"), LabelValue::new("v4_mapped_v6")),
            (
                label_name!("public_url"),
                LabelValue::new("https://tracker.example.test:7443/announce"),
            ),
            (label_name!("request_kind"), LabelValue::new("scrape")),
        ]);

        // Act
        handle_event(event, &stats_repository, CurrentClock::now()).await;

        // Assert
        let counter_value = {
            let stats = stats_repository.get_stats().await;
            stats
                .metric_collection
                .sum(&metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), &expected_labels)
                .unwrap()
        };
        assert!((counter_value - 1.0).abs() < f64::EPSILON);
    }
}
