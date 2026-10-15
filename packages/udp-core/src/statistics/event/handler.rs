use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::{LabelSet, LabelValue};
use torrust_metrics::{label_name, metric_name};

use crate::event::Event;
use crate::statistics::UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;
use crate::statistics::repository::Repository;

pub async fn handle_event(event: Event, stats_repository: &Repository, now: DurationSinceUnixEpoch) {
    let (mut label_set, request_kind) = labels_and_request_kind(event);
    label_set.upsert(label_name!("request_kind"), LabelValue::new(request_kind));

    if let Err(err) = stats_repository
        .increase_counter(&metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), &label_set, now)
        .await
    {
        tracing::error!("Failed to increase the counter: {}", err);
    }

    tracing::debug!("stats: {:?}", stats_repository.get_stats().await);
}

fn labels_and_request_kind(event: Event) -> (LabelSet, &'static str) {
    match event {
        Event::UdpConnect { connection } => (LabelSet::from(connection), "connect"),
        Event::UdpAnnounce { connection, .. } => (LabelSet::from(connection), "announce"),
        Event::UdpScrape { connection } => (LabelSet::from(connection), "scrape"),
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use rstest::rstest;
    use torrust_clock::clock::Time;
    use torrust_metrics::label::{LabelSet, LabelValue};
    use torrust_metrics::metric_collection::aggregate::sum::Sum;
    use torrust_metrics::{label_name, metric_name};
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_primitives::peer::PeerAnnouncement;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

    use crate::CurrentClock;
    use crate::event::{ConnectionContext, Event};
    use crate::statistics::UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::repository::Repository;
    use crate::tests::sample_info_hash;

    fn connection() -> ConnectionContext {
        ConnectionContext::new(
            ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
            ServiceBinding::new(
                Protocol::UDP,
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
            )
            .unwrap(),
        )
    }

    #[rstest]
    #[case::connect(Event::UdpConnect { connection: connection() }, "connect")]
    #[case::announce(
        Event::UdpAnnounce { connection: connection(), info_hash: sample_info_hash(), announcement: PeerAnnouncement::default() },
        "announce"
    )]
    #[case::scrape(Event::UdpScrape { connection: connection() }, "scrape")]
    #[tokio::test]
    async fn it_should_count_each_event_as_one_request_of_its_kind(#[case] event: Event, #[case] request_kind: &str) {
        // Arrange
        let stats_repository = Repository::new();

        // Act
        handle_event(event, &stats_repository, CurrentClock::now()).await;

        // Assert
        let requests_of_this_kind = stats_repository
            .get_stats()
            .await
            .metric_collection
            .sum(
                &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
                &[("request_kind", request_kind)].into(),
            )
            .unwrap();
        assert!((requests_of_this_kind - 1.0).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn it_should_propagate_all_connection_labels_and_request_kind() {
        // Arrange
        let stats_repository = Repository::new();
        let event = Event::UdpScrape {
            connection: ConnectionContext::new(
                ConfigurationInstanceId::new(ServiceRole::UdpTracker, 7),
                SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc000, 0x0201)), 49152),
                ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 7443)).unwrap(),
            )
            .with_public_url(Some("udp://tracker.example.test:7443/announce".to_string())),
        };
        let expected_labels = LabelSet::from([
            (label_name!("server_binding_protocol"), LabelValue::new("udp")),
            (label_name!("server_binding_ip"), LabelValue::new("::1")),
            (label_name!("server_binding_address_ip_type"), LabelValue::new("plain")),
            (label_name!("server_binding_address_ip_family"), LabelValue::new("inet6")),
            (label_name!("server_binding_port"), LabelValue::new("7443")),
            (label_name!("client_address_ip_family"), LabelValue::new("inet6")),
            (label_name!("client_address_ip_type"), LabelValue::new("v4_mapped_v6")),
            (
                label_name!("public_url"),
                LabelValue::new("udp://tracker.example.test:7443/announce"),
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
                .sum(&metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), &expected_labels)
                .unwrap()
        };
        assert!((counter_value - 1.0).abs() < f64::EPSILON);
    }
}
