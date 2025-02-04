use crate::statistics::event::{Event, UdpResponseKind};
use crate::statistics::repository::Repository;

pub async fn handle_event(event: Event, stats_repository: &Repository) {
    match event {
        // UDP
        Event::UdpRequestAborted => {
            stats_repository.increase_udp_requests_aborted().await;
        }
        Event::UdpRequestBanned => {
            stats_repository.increase_udp_requests_banned().await;
        }

        // UDP4
        Event::Udp4IncomingRequest => {
            stats_repository.increase_udp4_requests().await;
        }
        Event::Udp4Request { kind } => match kind {
            UdpResponseKind::Connect => {
                stats_repository.increase_udp4_connections().await;
            }
            UdpResponseKind::Announce => {
                stats_repository.increase_udp4_announces().await;
            }
            UdpResponseKind::Scrape => {
                stats_repository.increase_udp4_scrapes().await;
            }
            UdpResponseKind::Error => {}
        },
        Event::Udp4Response {
            kind,
            req_processing_time,
        } => {
            stats_repository.increase_udp4_responses().await;

            match kind {
                UdpResponseKind::Connect => {
                    stats_repository
                        .recalculate_udp_avg_connect_processing_time_ns(req_processing_time)
                        .await;
                }
                UdpResponseKind::Announce => {
                    stats_repository
                        .recalculate_udp_avg_announce_processing_time_ns(req_processing_time)
                        .await;
                }
                UdpResponseKind::Scrape => {
                    stats_repository
                        .recalculate_udp_avg_scrape_processing_time_ns(req_processing_time)
                        .await;
                }
                UdpResponseKind::Error => {}
            }
        }
        Event::Udp4Error => {
            stats_repository.increase_udp4_errors().await;
        }

        // UDP6
        Event::Udp6IncomingRequest => {
            stats_repository.increase_udp6_requests().await;
        }
        Event::Udp6Request { kind } => match kind {
            UdpResponseKind::Connect => {
                stats_repository.increase_udp6_connections().await;
            }
            UdpResponseKind::Announce => {
                stats_repository.increase_udp6_announces().await;
            }
            UdpResponseKind::Scrape => {
                stats_repository.increase_udp6_scrapes().await;
            }
            UdpResponseKind::Error => {}
        },
        Event::Udp6Response {
            kind: _,
            req_processing_time: _,
        } => {
            stats_repository.increase_udp6_responses().await;
        }
        Event::Udp6Error => {
            stats_repository.increase_udp6_errors().await;
        }
    }

    tracing::debug!("stats: {:?}", stats_repository.get_stats().await);
}

#[cfg(test)]
mod tests {
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::event::Event;
    use crate::statistics::repository::Repository;

    #[tokio::test]
    async fn should_increase_the_number_of_aborted_requests_when_it_receives_a_udp_request_aborted_event() {
        let stats_repository = Repository::new();

        handle_event(Event::UdpRequestAborted, &stats_repository).await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp_requests_aborted, 1);
    }

    #[tokio::test]
    async fn should_increase_the_number_of_banned_requests_when_it_receives_a_udp_request_banned_event() {
        let stats_repository = Repository::new();

        handle_event(Event::UdpRequestBanned, &stats_repository).await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp_requests_banned, 1);
    }

    #[tokio::test]
    async fn should_increase_the_number_of_incoming_requests_when_it_receives_a_udp4_incoming_request_event() {
        let stats_repository = Repository::new();

        handle_event(Event::Udp4IncomingRequest, &stats_repository).await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_requests, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp_abort_counter_when_it_receives_a_udp_abort_event() {
        let stats_repository = Repository::new();

        handle_event(Event::UdpRequestAborted, &stats_repository).await;
        let stats = stats_repository.get_stats().await;
        assert_eq!(stats.udp_requests_aborted, 1);
    }
    #[tokio::test]
    async fn should_increase_the_udp_ban_counter_when_it_receives_a_udp_banned_event() {
        let stats_repository = Repository::new();

        handle_event(Event::UdpRequestBanned, &stats_repository).await;
        let stats = stats_repository.get_stats().await;
        assert_eq!(stats.udp_requests_banned, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_connect_requests_counter_when_it_receives_a_udp4_request_event_of_connect_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::Udp4Request {
                kind: crate::statistics::event::UdpResponseKind::Connect,
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
            Event::Udp4Request {
                kind: crate::statistics::event::UdpResponseKind::Announce,
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
            Event::Udp4Request {
                kind: crate::statistics::event::UdpResponseKind::Scrape,
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
            Event::Udp4Response {
                kind: crate::statistics::event::UdpResponseKind::Announce,
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

        handle_event(Event::Udp4Error, &stats_repository).await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_errors_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_connect_requests_counter_when_it_receives_a_udp6_request_event_of_connect_kind() {
        let stats_repository = Repository::new();

        handle_event(
            Event::Udp6Request {
                kind: crate::statistics::event::UdpResponseKind::Connect,
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
            Event::Udp6Request {
                kind: crate::statistics::event::UdpResponseKind::Announce,
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
            Event::Udp6Request {
                kind: crate::statistics::event::UdpResponseKind::Scrape,
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
            Event::Udp6Response {
                kind: crate::statistics::event::UdpResponseKind::Announce,
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

        handle_event(Event::Udp6Error, &stats_repository).await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_errors_handled, 1);
    }
}
