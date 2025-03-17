use crate::statistics::event::Event;
use crate::statistics::repository::Repository;

/// # Panics
///
/// This function panics if the IP version does not match the event type.
pub async fn handle_event(event: Event, stats_repository: &Repository) {
    match event {
        Event::UdpConnect { context } => match context.client_socket_addr.ip() {
            std::net::IpAddr::V4(_) => {
                stats_repository.increase_udp4_connections().await;
            }
            std::net::IpAddr::V6(_) => {
                stats_repository.increase_udp6_connections().await;
            }
        },
        Event::UdpAnnounce { context } => match context.client_socket_addr.ip() {
            std::net::IpAddr::V4(_) => {
                stats_repository.increase_udp4_announces().await;
            }
            std::net::IpAddr::V6(_) => {
                stats_repository.increase_udp6_announces().await;
            }
        },
        Event::UdpScrape { context } => match context.client_socket_addr.ip() {
            std::net::IpAddr::V4(_) => {
                stats_repository.increase_udp4_scrapes().await;
            }
            std::net::IpAddr::V6(_) => {
                stats_repository.increase_udp6_scrapes().await;
            }
        },
    }

    tracing::debug!("stats: {:?}", stats_repository.get_stats().await);
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use crate::statistics::event::handler::handle_event;
    use crate::statistics::event::{ConnectionContext, Event};
    use crate::statistics::repository::Repository;

    #[tokio::test]
    async fn should_increase_the_udp4_connections_counter_when_it_receives_a_udp4_connect_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpConnect {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_connections_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_announces_counter_when_it_receives_a_udp4_announce_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpAnnounce {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_announces_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_scrapes_counter_when_it_receives_a_udp4_scrape_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpScrape {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_scrapes_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_connections_counter_when_it_receives_a_udp6_connect_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpConnect {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_connections_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_announces_counter_when_it_receives_a_udp6_announce_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpAnnounce {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_announces_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_scrapes_counter_when_it_receives_a_udp6_scrape_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpScrape {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_scrapes_handled, 1);
    }
}
