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
    // Global fixed metrics
    match kind {
        UdpRequestKind::Connect => match context.client_socket_addr().ip() {
            std::net::IpAddr::V4(_) => {
                stats_repository.increase_udp4_connections().await;
            }
            std::net::IpAddr::V6(_) => {
                stats_repository.increase_udp6_connections().await;
            }
        },
        UdpRequestKind::Announce => match context.client_socket_addr().ip() {
            std::net::IpAddr::V4(_) => {
                stats_repository.increase_udp4_announces().await;
            }
            std::net::IpAddr::V6(_) => {
                stats_repository.increase_udp6_announces().await;
            }
        },
        UdpRequestKind::Scrape => match context.client_socket_addr().ip() {
            std::net::IpAddr::V4(_) => {
                stats_repository.increase_udp4_scrapes().await;
            }
            std::net::IpAddr::V6(_) => {
                stats_repository.increase_udp6_scrapes().await;
            }
        },
    }

    // Extendable metrics
    let mut label_set = LabelSet::from(context);
    label_set.upsert(label_name!("request_kind"), LabelValue::new(&kind.to_string()));
    match stats_repository
        .increase_counter(&metric_name!(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &label_set, now)
        .await
    {
        Ok(()) => {}
        Err(err) => tracing::error!("Failed to increase the counter: {}", err),
    };
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
    use std::sync::Arc;

    use bittorrent_udp_tracker_core::services::banning::BanService;
    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

    use crate::event::{ConnectionContext, Event};
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::repository::Repository;
    use crate::CurrentClock;

    #[tokio::test]
    async fn should_increase_the_udp4_connect_requests_counter_when_it_receives_a_udp4_request_event_of_connect_kind() {
        let stats_repository = Repository::new();
        let ban_service = Arc::new(tokio::sync::RwLock::new(BanService::new(1)));

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
            &ban_service,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_connections_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_announce_requests_counter_when_it_receives_a_udp4_request_event_of_announce_kind() {
        let stats_repository = Repository::new();
        let ban_service = Arc::new(tokio::sync::RwLock::new(BanService::new(1)));

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
                kind: crate::event::UdpRequestKind::Announce,
            },
            &stats_repository,
            &ban_service,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_announces_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_scrape_requests_counter_when_it_receives_a_udp4_request_event_of_scrape_kind() {
        let stats_repository = Repository::new();
        let ban_service = Arc::new(tokio::sync::RwLock::new(BanService::new(1)));

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
            &ban_service,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_scrapes_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_connect_requests_counter_when_it_receives_a_udp6_request_event_of_connect_kind() {
        let stats_repository = Repository::new();
        let ban_service = Arc::new(tokio::sync::RwLock::new(BanService::new(1)));

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
            &ban_service,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_connections_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_announce_requests_counter_when_it_receives_a_udp6_request_event_of_announce_kind() {
        let stats_repository = Repository::new();
        let ban_service = Arc::new(tokio::sync::RwLock::new(BanService::new(1)));

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
                kind: crate::event::UdpRequestKind::Announce,
            },
            &stats_repository,
            &ban_service,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_announces_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_scrape_requests_counter_when_it_receives_a_udp6_request_event_of_scrape_kind() {
        let stats_repository = Repository::new();
        let ban_service = Arc::new(tokio::sync::RwLock::new(BanService::new(1)));

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
            &ban_service,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_scrapes_handled, 1);
    }
}
