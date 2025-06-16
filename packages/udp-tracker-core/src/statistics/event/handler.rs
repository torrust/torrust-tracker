use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::{label_name, metric_name};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::Event;
use crate::statistics::repository::Repository;
use crate::statistics::UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;

/// # Panics
///
/// This function panics if the IP version does not match the event type.
pub async fn handle_event(event: Event, stats_repository: &Repository, now: DurationSinceUnixEpoch) {
    match event {
        Event::UdpConnect { connection: context } => {
            let mut label_set = LabelSet::from(context);
            label_set.upsert(label_name!("request_kind"), LabelValue::new("connect"));

            match stats_repository
                .increase_counter(&metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), &label_set, now)
                .await
            {
                Ok(()) => {}
                Err(err) => tracing::error!("Failed to increase the counter: {}", err),
            };
        }
        Event::UdpAnnounce { connection: context, .. } => {
            let mut label_set = LabelSet::from(context);
            label_set.upsert(label_name!("request_kind"), LabelValue::new("announce"));

            match stats_repository
                .increase_counter(&metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), &label_set, now)
                .await
            {
                Ok(()) => {}
                Err(err) => tracing::error!("Failed to increase the counter: {}", err),
            };
        }
        Event::UdpScrape { connection: context } => {
            let mut label_set = LabelSet::from(context);
            label_set.upsert(label_name!("request_kind"), LabelValue::new("scrape"));

            match stats_repository
                .increase_counter(&metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), &label_set, now)
                .await
            {
                Ok(()) => {}
                Err(err) => tracing::error!("Failed to increase the counter: {}", err),
            };
        }
    }

    tracing::debug!("stats: {:?}", stats_repository.get_stats().await);
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_primitives::peer::PeerAnnouncement;
    use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

    use crate::event::{ConnectionContext, Event};
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::repository::Repository;
    use crate::tests::sample_info_hash;
    use crate::CurrentClock;

    #[tokio::test]
    async fn should_increase_the_udp4_connections_counter_when_it_receives_a_udp4_connect_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpConnect {
                connection: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_connections_handled(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_announces_counter_when_it_receives_a_udp4_announce_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpAnnounce {
                connection: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
                info_hash: sample_info_hash(),
                announcement: PeerAnnouncement::default(),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_announces_handled(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_scrapes_counter_when_it_receives_a_udp4_scrape_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpScrape {
                connection: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_scrapes_handled(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_connections_counter_when_it_receives_a_udp6_connect_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpConnect {
                connection: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_connections_handled(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_announces_counter_when_it_receives_a_udp6_announce_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpAnnounce {
                connection: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
                info_hash: sample_info_hash(),
                announcement: PeerAnnouncement::default(),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_announces_handled(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_scrapes_counter_when_it_receives_a_udp6_scrape_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpScrape {
                connection: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_scrapes_handled(), 1);
    }
}
