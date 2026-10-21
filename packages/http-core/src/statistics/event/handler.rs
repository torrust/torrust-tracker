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

    tracing::debug!(
        "stats: {}",
        stats_repository.read_metrics(|metrics| format!("{metrics:?}")).await
    );
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
    use std::net::{IpAddr, SocketAddr};
    use std::sync::Arc;

    use rstest::rstest;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_name;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::{RemoteClientAddr, ResolvedIp};
    use torrust_tracker_primitives::peer::Peer;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

    use super::handle_event;
    use crate::event::{ConnectionContext, Event};
    use crate::statistics::repository::Repository;
    use crate::statistics::{HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL, describe_metrics};
    use crate::tests::{sample_info_hash, sample_peer_using_ipv4, sample_peer_using_ipv6};

    const NOW: DurationSinceUnixEpoch = DurationSinceUnixEpoch::ZERO;
    const CLIENT_PORT: u16 = 8080;
    const SERVER_PORT: u16 = 7070;

    fn ip(text: &str) -> IpAddr {
        text.parse().expect("the test IP address should be valid")
    }

    /// A connection from `client_ip` to an HTTP tracker bound to `bind_ip`.
    fn connection(client_ip: &str, bind_ip: &str) -> ConnectionContext {
        connection_on_port(client_ip, bind_ip, SERVER_PORT)
    }

    fn connection_on_port(client_ip: &str, bind_ip: &str, server_port: u16) -> ConnectionContext {
        ConnectionContext::new(
            ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(ip(client_ip)), Some(CLIENT_PORT)),
            ServiceBinding::new(Protocol::HTTP, SocketAddr::new(ip(bind_ip), server_port)).unwrap(),
        )
    }

    fn announce_event(connection: ConnectionContext, announcement: Peer) -> Event {
        Event::TcpAnnounce {
            connection,
            info_hash: sample_info_hash(),
            announcement,
        }
    }

    fn scrape_event(connection: ConnectionContext) -> Event {
        Event::TcpScrape { connection }
    }

    fn labels(pairs: &[(&str, &str)]) -> LabelSet {
        LabelSet::from(pairs.to_vec())
    }

    /// The four public counters: IPv4 announces, IPv4 scrapes, IPv6 announces, IPv6 scrapes.
    async fn counters(repository: &Repository) -> (u64, u64, u64, u64) {
        repository
            .read_metrics(|metrics| {
                (
                    metrics.tcp4_announces_handled(),
                    metrics.tcp4_scrapes_handled(),
                    metrics.tcp6_announces_handled(),
                    metrics.tcp6_scrapes_handled(),
                )
            })
            .await
    }

    /// The value of the sample that has exactly `labels`, or `None` when there is no such sample.
    async fn sample_with_exactly(repository: &Repository, labels: &LabelSet) -> Option<u64> {
        repository
            .read_metrics(|metrics| {
                metrics
                    .metric_collection()
                    .get_counter_value(&metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), labels)
            })
            .await
            .map(u64::from)
    }

    #[rstest]
    #[case::ipv4_announce(announce_event(connection("126.0.0.2", "127.0.0.1"), sample_peer_using_ipv4()), (1, 0, 0, 0))]
    #[case::ipv4_scrape(scrape_event(connection("126.0.0.2", "127.0.0.1")), (0, 1, 0, 0))]
    #[case::ipv6_announce(announce_event(connection("2001:db8::1", "::1"), sample_peer_using_ipv6()), (0, 0, 1, 0))]
    #[case::ipv6_scrape(scrape_event(connection("2001:db8::1", "::1")), (0, 0, 0, 1))]
    #[tokio::test]
    async fn it_should_increase_only_the_counter_of_the_request_kind_and_family_of_the_event(
        #[case] event: Event,
        #[case] expected_counters: (u64, u64, u64, u64),
    ) {
        // Arrange
        let repository = Arc::new(Repository::new());

        // Act
        handle_event(event, &repository, NOW).await;

        // Assert
        assert_eq!(counters(&repository).await, expected_counters);
    }

    #[rstest]
    #[case::ipv4_announce(
        announce_event(connection("126.0.0.2", "127.0.0.1"), sample_peer_using_ipv4()),
        &[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "127.0.0.1"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet"),
            ("server_binding_port", "7070"),
            ("client_address_ip_family", "inet"),
            ("client_address_ip_type", "plain"),
            ("request_kind", "announce"),
        ]
    )]
    #[case::ipv6_announce(
        announce_event(connection("2001:db8::1", "::1"), sample_peer_using_ipv6()),
        &[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "::1"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet6"),
            ("server_binding_port", "7070"),
            ("client_address_ip_family", "inet6"),
            ("client_address_ip_type", "plain"),
            ("request_kind", "announce"),
        ]
    )]
    #[case::ipv4_scrape(
        scrape_event(connection("126.0.0.2", "127.0.0.1")),
        &[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "127.0.0.1"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet"),
            ("server_binding_port", "7070"),
            ("client_address_ip_family", "inet"),
            ("client_address_ip_type", "plain"),
            ("request_kind", "scrape"),
        ]
    )]
    #[case::ipv6_scrape_of_an_ipv4_mapped_client_with_a_public_url_on_another_port(
        scrape_event(
            connection_on_port("::ffff:192.0.2.1", "::1", 7443).with_public_url(Some("https://tracker.example.test/announce".to_string()))
        ),
        &[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "::1"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet6"),
            ("server_binding_port", "7443"),
            ("client_address_ip_family", "inet6"),
            ("client_address_ip_type", "v4_mapped_v6"),
            ("public_url", "https://tracker.example.test/announce"),
            ("request_kind", "scrape"),
        ]
    )]
    #[tokio::test]
    async fn it_should_count_the_event_in_the_sample_with_exactly_the_connection_labels_and_the_request_kind(
        #[case] event: Event,
        #[case] expected_labels: &[(&str, &str)],
    ) {
        // Arrange
        let repository = Arc::new(Repository::new());

        // Act
        handle_event(event, &repository, NOW).await;

        // Assert: an exact lookup, so a missing, extra or wrong-valued label finds no sample.
        assert_eq!(sample_with_exactly(&repository, &labels(expected_labels)).await, Some(1));
    }

    #[tokio::test]
    async fn it_should_accumulate_events_with_the_same_labels_in_one_sample() {
        // Arrange
        let repository = Arc::new(Repository::new());
        let same_labels = labels(&[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "127.0.0.1"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet"),
            ("server_binding_port", "7070"),
            ("client_address_ip_family", "inet"),
            ("client_address_ip_type", "plain"),
            ("request_kind", "announce"),
        ]);

        // Act
        handle_event(
            announce_event(connection("126.0.0.2", "127.0.0.1"), sample_peer_using_ipv4()),
            &repository,
            NOW,
        )
        .await;
        handle_event(
            announce_event(connection("126.0.0.3", "127.0.0.1"), sample_peer_using_ipv4()),
            &repository,
            NOW,
        )
        .await;

        // Assert: two clients of one server share the sample, because the client address is not a label.
        assert_eq!(sample_with_exactly(&repository, &same_labels).await, Some(2));
        assert_eq!(counters(&repository).await, (2, 0, 0, 0));
    }

    #[tokio::test]
    async fn it_should_keep_events_with_different_labels_in_separate_samples() {
        // Arrange
        let repository = Arc::new(Repository::new());
        let without_public_url = labels(&[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "127.0.0.1"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet"),
            ("server_binding_port", "7070"),
            ("client_address_ip_family", "inet"),
            ("client_address_ip_type", "plain"),
            ("request_kind", "scrape"),
        ]);
        let with_public_url = labels(&[
            ("server_binding_protocol", "http"),
            ("server_binding_ip", "127.0.0.1"),
            ("server_binding_address_ip_type", "plain"),
            ("server_binding_address_ip_family", "inet"),
            ("server_binding_port", "7070"),
            ("client_address_ip_family", "inet"),
            ("client_address_ip_type", "plain"),
            ("public_url", "https://tracker.example.test/announce"),
            ("request_kind", "scrape"),
        ]);

        // Act
        handle_event(scrape_event(connection("126.0.0.2", "127.0.0.1")), &repository, NOW).await;
        handle_event(
            scrape_event(
                connection("126.0.0.2", "127.0.0.1").with_public_url(Some("https://tracker.example.test/announce".to_string())),
            ),
            &repository,
            NOW,
        )
        .await;

        // Assert
        assert_eq!(sample_with_exactly(&repository, &without_public_url).await, Some(1));
        assert_eq!(sample_with_exactly(&repository, &with_public_url).await, Some(1));
        assert_eq!(counters(&repository).await, (0, 2, 0, 0));
    }

    #[tokio::test]
    async fn it_should_record_the_given_time_on_the_sample_it_increases() {
        // Arrange: the whole metrics value that one announce at 7 seconds must leave, built from literals.
        let at_seven_seconds = DurationSinceUnixEpoch::from_secs(7);
        let repository = Arc::new(Repository::new());
        let mut expected = describe_metrics();
        expected
            .metric_collection
            .set_counter(
                &metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
                &labels(&[
                    ("server_binding_protocol", "http"),
                    ("server_binding_ip", "127.0.0.1"),
                    ("server_binding_address_ip_type", "plain"),
                    ("server_binding_address_ip_family", "inet"),
                    ("server_binding_port", "7070"),
                    ("client_address_ip_family", "inet"),
                    ("client_address_ip_type", "plain"),
                    ("request_kind", "announce"),
                ]),
                1,
                at_seven_seconds,
            )
            .unwrap();

        // Act
        handle_event(
            announce_event(connection("126.0.0.2", "127.0.0.1"), sample_peer_using_ipv4()),
            &repository,
            at_seven_seconds,
        )
        .await;

        // Assert: the sample, and the time it was recorded at, are part of the value.
        assert_eq!(repository.read_metrics(Clone::clone).await, expected);
    }
}
