use crate::statistics::event::{Event, UdpRequestKind, UdpResponseKind};
use crate::statistics::repository::Repository;

/// # Panics
///
/// This function panics if the client IP version does not match the expected
/// version.
#[allow(clippy::too_many_lines)]
pub async fn handle_event(event: Event, stats_repository: &Repository) {
    match event {
        Event::UdpRequestAborted { .. } => {
            stats_repository.increase_udp_requests_aborted().await;
        }
        Event::UdpRequestBanned { .. } => {
            stats_repository.increase_udp_requests_banned().await;
        }
        Event::UdpIncomingRequest { context } => match context.client_socket_addr().ip() {
            std::net::IpAddr::V4(_) => {
                stats_repository.increase_udp4_requests().await;
            }
            std::net::IpAddr::V6(_) => {
                stats_repository.increase_udp6_requests().await;
            }
        },
        Event::UdpRequest { context, kind } => match kind {
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
        },
        Event::UdpResponse {
            context,
            kind,
            req_processing_time,
        } => {
            match context.client_socket_addr().ip() {
                std::net::IpAddr::V4(_) => {
                    stats_repository.increase_udp4_responses().await;
                }
                std::net::IpAddr::V6(_) => {
                    stats_repository.increase_udp6_responses().await;
                }
            }

            match kind {
                UdpResponseKind::Ok { req_kind } => match req_kind {
                    UdpRequestKind::Connect => {
                        stats_repository
                            .recalculate_udp_avg_connect_processing_time_ns(req_processing_time)
                            .await;
                    }
                    UdpRequestKind::Announce => {
                        stats_repository
                            .recalculate_udp_avg_announce_processing_time_ns(req_processing_time)
                            .await;
                    }
                    UdpRequestKind::Scrape => {
                        stats_repository
                            .recalculate_udp_avg_scrape_processing_time_ns(req_processing_time)
                            .await;
                    }
                },
                UdpResponseKind::Error => {}
            }
        }
        Event::UdpError { context } => match context.client_socket_addr().ip() {
            std::net::IpAddr::V4(_) => {
                stats_repository.increase_udp4_errors().await;
            }
            std::net::IpAddr::V6(_) => {
                stats_repository.increase_udp6_errors().await;
            }
        },
    }

    tracing::debug!("stats: {:?}", stats_repository.get_stats().await);
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use crate::statistics::event::handler::handle_event;
    use crate::statistics::event::{ConnectionContext, Event, UdpRequestKind};
    use crate::statistics::repository::Repository;

    #[tokio::test]
    async fn should_increase_the_number_of_aborted_requests_when_it_receives_a_udp_request_aborted_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestAborted {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp_requests_aborted, 1);
    }

    #[tokio::test]
    async fn should_increase_the_number_of_banned_requests_when_it_receives_a_udp_request_banned_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestBanned {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp_requests_banned, 1);
    }

    #[tokio::test]
    async fn should_increase_the_number_of_incoming_requests_when_it_receives_a_udp4_incoming_request_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpIncomingRequest {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_requests, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp_abort_counter_when_it_receives_a_udp_abort_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestAborted {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;
        let stats = stats_repository.get_stats().await;
        assert_eq!(stats.udp_requests_aborted, 1);
    }
    #[tokio::test]
    async fn should_increase_the_udp_ban_counter_when_it_receives_a_udp_banned_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestBanned {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;
        let stats = stats_repository.get_stats().await;
        assert_eq!(stats.udp_requests_banned, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_connect_requests_counter_when_it_receives_a_udp4_request_event_of_connect_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequest {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
                kind: crate::statistics::event::UdpRequestKind::Connect,
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_connections_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_announce_requests_counter_when_it_receives_a_udp4_request_event_of_announce_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequest {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
                kind: crate::statistics::event::UdpRequestKind::Announce,
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_announces_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_scrape_requests_counter_when_it_receives_a_udp4_request_event_of_scrape_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequest {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
                kind: crate::statistics::event::UdpRequestKind::Scrape,
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_scrapes_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_responses_counter_when_it_receives_a_udp4_response_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpResponse {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
                kind: crate::statistics::event::UdpResponseKind::Ok {
                    req_kind: UdpRequestKind::Announce,
                },
                req_processing_time: std::time::Duration::from_secs(1),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_responses, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_errors_counter_when_it_receives_a_udp4_error_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpError {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_errors_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_connect_requests_counter_when_it_receives_a_udp6_request_event_of_connect_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequest {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                ),
                kind: crate::statistics::event::UdpRequestKind::Connect,
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_connections_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_announce_requests_counter_when_it_receives_a_udp6_request_event_of_announce_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequest {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                ),
                kind: crate::statistics::event::UdpRequestKind::Announce,
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_announces_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_scrape_requests_counter_when_it_receives_a_udp6_request_event_of_scrape_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequest {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                ),
                kind: crate::statistics::event::UdpRequestKind::Scrape,
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_scrapes_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_response_counter_when_it_receives_a_udp6_response_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpResponse {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                ),
                kind: crate::statistics::event::UdpResponseKind::Ok {
                    req_kind: UdpRequestKind::Announce,
                },
                req_processing_time: std::time::Duration::from_secs(1),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_responses, 1);
    }
    #[tokio::test]
    async fn should_increase_the_udp6_errors_counter_when_it_receives_a_udp6_error_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpError {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                ),
            },
            &stats_repository,
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_errors_handled, 1);
    }
}
