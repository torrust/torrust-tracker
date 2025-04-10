use std::net::IpAddr;

use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric::MetricName;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::Event;
use crate::statistics::repository::Repository;

/// # Panics
///
/// This function panics if the client IP address is not the same as the IP
/// version of the event.
pub async fn handle_event(event: Event, stats_repository: &Repository, now: DurationSinceUnixEpoch) {
    match event {
        Event::TcpAnnounce { connection } => {
            // Global fixed metrics

            match connection.client_ip_addr() {
                IpAddr::V4(_) => {
                    stats_repository.increase_tcp4_announces().await;
                }
                IpAddr::V6(_) => {
                    stats_repository.increase_tcp6_announces().await;
                }
            }

            // Extendable metrics

            stats_repository
                .increase_counter(
                    &MetricName::new("http_tracker_core_announce_requests_received_total"),
                    &LabelSet::from(connection),
                    now,
                )
                .await;
        }
        Event::TcpScrape { connection } => {
            // Global fixed metrics

            match connection.client_ip_addr() {
                IpAddr::V4(_) => {
                    stats_repository.increase_tcp4_scrapes().await;
                }
                IpAddr::V6(_) => {
                    stats_repository.increase_tcp6_scrapes().await;
                }
            }

            // Extendable metrics

            stats_repository
                .increase_counter(
                    &MetricName::new("http_tracker_core_scrape_requests_received_total"),
                    &LabelSet::from(connection),
                    now,
                )
                .await;
        }
    }

    tracing::debug!("stats: {:?}", stats_repository.get_stats().await);
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

    use crate::event::{ConnectionContext, Event};
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::repository::Repository;
    use crate::CurrentClock;

    #[tokio::test]
    async fn should_increase_the_tcp4_announces_counter_when_it_receives_a_tcp4_announce_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::TcpAnnounce {
                connection: ConnectionContext::new(
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)),
                    Some(8080),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7070)).unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp4_announces_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_tcp4_scrapes_counter_when_it_receives_a_tcp4_scrape_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::TcpScrape {
                connection: ConnectionContext::new(
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)),
                    Some(8080),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7070)).unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp4_scrapes_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_tcp6_announces_counter_when_it_receives_a_tcp6_announce_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::TcpAnnounce {
                connection: ConnectionContext::new(
                    IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
                    Some(8080),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7070)).unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp6_announces_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_tcp6_scrapes_counter_when_it_receives_a_tcp6_scrape_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::TcpScrape {
                connection: ConnectionContext::new(
                    IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
                    Some(8080),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7070)).unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp6_scrapes_handled, 1);
    }
}
