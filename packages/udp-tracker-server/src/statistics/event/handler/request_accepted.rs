use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::{label_name, metric_name};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::{ConnectionContext, UdpRequestKind};
use crate::statistics::repository::Repository;
use crate::statistics::UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL;

pub async fn handle_event(
    context: ConnectionContext,
    kind: UdpRequestKind,
    stats_repository: &Repository,
    now: DurationSinceUnixEpoch,
) {
    let mut label_set = LabelSet::from(context);
    label_set.upsert(label_name!("request_kind"), LabelValue::new(&kind.to_string()));
    match stats_repository
        .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &label_set, now)
        .await
    {
        Ok(()) => {
            tracing::debug!("Successfully increased the counter for UDP requests accepted: {}", label_set);
        }
        Err(err) => tracing::error!("Failed to increase the counter: {}", err),
    };
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

    use crate::event::{ConnectionContext, Event};
    use crate::handlers::announce::tests::announce_request::AnnounceRequestBuilder;
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::repository::Repository;
    use crate::CurrentClock;

    #[tokio::test]
    async fn should_increase_the_udp4_connect_requests_counter_when_it_receives_a_udp4_request_event_of_connect_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestAccepted {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
                kind: crate::event::UdpRequestKind::Connect,
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_connect_requests_accepted_total(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_announce_requests_counter_when_it_receives_a_udp4_request_event_of_announce_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestAccepted {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
                kind: crate::event::UdpRequestKind::Announce {
                    announce_request: AnnounceRequestBuilder::default().into(),
                },
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_announce_requests_accepted_total(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_scrape_requests_counter_when_it_receives_a_udp4_request_event_of_scrape_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestAccepted {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
                kind: crate::event::UdpRequestKind::Scrape,
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_scrape_requests_accepted_total(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_connect_requests_counter_when_it_receives_a_udp6_request_event_of_connect_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestAccepted {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
                kind: crate::event::UdpRequestKind::Connect,
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_connect_requests_accepted_total(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_announce_requests_counter_when_it_receives_a_udp6_request_event_of_announce_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestAccepted {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
                kind: crate::event::UdpRequestKind::Announce {
                    announce_request: AnnounceRequestBuilder::default().into(),
                },
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_announce_requests_accepted_total(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_scrape_requests_counter_when_it_receives_a_udp6_request_event_of_scrape_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestAccepted {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
                kind: crate::event::UdpRequestKind::Scrape,
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_scrape_requests_accepted_total(), 1);
    }
}
