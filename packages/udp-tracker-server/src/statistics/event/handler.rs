use torrust_tracker_metrics::label::{LabelName, LabelSet, LabelValue};
use torrust_tracker_metrics::metric::MetricName;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::{Event, UdpRequestKind, UdpResponseKind};
use crate::statistics::repository::Repository;
use crate::statistics::{
    UDP_TRACKER_SERVER_ERRORS_TOTAL, UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS,
    UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL, UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL,
    UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL, UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL,
    UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL,
};

/// # Panics
///
/// This function panics if the client IP version does not match the expected
/// version.
#[allow(clippy::too_many_lines)]
pub async fn handle_event(event: Event, stats_repository: &Repository, now: DurationSinceUnixEpoch) {
    match event {
        Event::UdpRequestAborted { context } => {
            // Global fixed metrics
            stats_repository.increase_udp_requests_aborted().await;

            // Extendable metrics
            stats_repository
                .increase_counter(
                    &MetricName::new(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL),
                    &LabelSet::from(context),
                    now,
                )
                .await;
        }
        Event::UdpRequestBanned { context } => {
            // Global fixed metrics
            stats_repository.increase_udp_requests_banned().await;

            // Extendable metrics
            stats_repository
                .increase_counter(
                    &MetricName::new(UDP_TRACKER_SERVER_REQUESTS_BANNED_TOTAL),
                    &LabelSet::from(context),
                    now,
                )
                .await;
        }
        Event::UdpRequestReceived { context } => {
            // Global fixed metrics
            match context.client_socket_addr().ip() {
                std::net::IpAddr::V4(_) => {
                    stats_repository.increase_udp4_requests().await;
                }
                std::net::IpAddr::V6(_) => {
                    stats_repository.increase_udp6_requests().await;
                }
            }

            // Extendable metrics
            stats_repository
                .increase_counter(
                    &MetricName::new(UDP_TRACKER_SERVER_REQUESTS_RECEIVED_TOTAL),
                    &LabelSet::from(context),
                    now,
                )
                .await;
        }
        Event::UdpRequestAccepted { context, kind } => {
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

            label_set.upsert(LabelName::new("kind"), LabelValue::new(&kind.to_string()));

            stats_repository
                .increase_counter(&MetricName::new(UDP_TRACKER_SERVER_REQUESTS_ACCEPTED_TOTAL), &label_set, now)
                .await;
        }
        Event::UdpResponseSent {
            context,
            kind,
            req_processing_time,
        } => {
            // Global fixed metrics
            match context.client_socket_addr().ip() {
                std::net::IpAddr::V4(_) => {
                    stats_repository.increase_udp4_responses().await;
                }
                std::net::IpAddr::V6(_) => {
                    stats_repository.increase_udp6_responses().await;
                }
            }

            let (result_label_value, kind_label_value) = match kind {
                UdpResponseKind::Ok { req_kind } => match req_kind {
                    UdpRequestKind::Connect => {
                        let new_avg = stats_repository
                            .recalculate_udp_avg_connect_processing_time_ns(req_processing_time)
                            .await;

                        // Extendable metrics

                        let mut label_set = LabelSet::from(context.clone());
                        label_set.upsert(LabelName::new("request_kind"), LabelValue::new(&req_kind.to_string()));

                        stats_repository
                            .set_gauge(
                                &MetricName::new(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                                &label_set,
                                new_avg,
                                now,
                            )
                            .await;

                        (LabelValue::new("ok"), LabelValue::new(&UdpRequestKind::Connect.to_string()))
                    }
                    UdpRequestKind::Announce => {
                        let new_avg = stats_repository
                            .recalculate_udp_avg_announce_processing_time_ns(req_processing_time)
                            .await;

                        // Extendable metrics

                        let mut label_set = LabelSet::from(context.clone());
                        label_set.upsert(LabelName::new("request_kind"), LabelValue::new(&req_kind.to_string()));

                        stats_repository
                            .set_gauge(
                                &MetricName::new(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                                &label_set,
                                new_avg,
                                now,
                            )
                            .await;

                        (LabelValue::new("ok"), LabelValue::new(&UdpRequestKind::Announce.to_string()))
                    }
                    UdpRequestKind::Scrape => {
                        let new_avg = stats_repository
                            .recalculate_udp_avg_scrape_processing_time_ns(req_processing_time)
                            .await;

                        // Extendable metrics

                        let mut label_set = LabelSet::from(context.clone());
                        label_set.upsert(LabelName::new("request_kind"), LabelValue::new(&req_kind.to_string()));

                        stats_repository
                            .set_gauge(
                                &MetricName::new(UDP_TRACKER_SERVER_PERFORMANCE_AVG_PROCESSING_TIME_NS),
                                &label_set,
                                new_avg,
                                now,
                            )
                            .await;

                        (LabelValue::new("ok"), LabelValue::new(&UdpRequestKind::Scrape.to_string()))
                    }
                },
                UdpResponseKind::Error { opt_req_kind: _ } => (LabelValue::new("error"), LabelValue::ignore()),
            };

            // Extendable metrics

            let mut label_set = LabelSet::from(context);

            if result_label_value == LabelValue::new("ok") {
                label_set.upsert(LabelName::new("request_kind"), kind_label_value);
            }
            label_set.upsert(LabelName::new("result"), result_label_value);

            stats_repository
                .increase_counter(&MetricName::new(UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL), &label_set, now)
                .await;
        }
        Event::UdpError { context } => {
            // Global fixed metrics
            match context.client_socket_addr().ip() {
                std::net::IpAddr::V4(_) => {
                    stats_repository.increase_udp4_errors().await;
                }
                std::net::IpAddr::V6(_) => {
                    stats_repository.increase_udp6_errors().await;
                }
            }

            // Extendable metrics
            stats_repository
                .increase_counter(
                    &MetricName::new(UDP_TRACKER_SERVER_ERRORS_TOTAL),
                    &LabelSet::from(context),
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

    use crate::event::{ConnectionContext, Event, UdpRequestKind};
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::repository::Repository;
    use crate::CurrentClock;

    #[tokio::test]
    async fn should_increase_the_number_of_aborted_requests_when_it_receives_a_udp_request_aborted_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestAborted {
                context: ConnectionContext::new(
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

        assert_eq!(stats.udp_requests_aborted, 1);
    }

    #[tokio::test]
    async fn should_increase_the_number_of_banned_requests_when_it_receives_a_udp_request_banned_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestBanned {
                context: ConnectionContext::new(
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

        assert_eq!(stats.udp_requests_banned, 1);
    }

    #[tokio::test]
    async fn should_increase_the_number_of_incoming_requests_when_it_receives_a_udp4_incoming_request_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestReceived {
                context: ConnectionContext::new(
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

        assert_eq!(stats.udp4_requests, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp_abort_counter_when_it_receives_a_udp_abort_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestAborted {
                context: ConnectionContext::new(
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
        assert_eq!(stats.udp_requests_aborted, 1);
    }
    #[tokio::test]
    async fn should_increase_the_udp_ban_counter_when_it_receives_a_udp_banned_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpRequestBanned {
                context: ConnectionContext::new(
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
        assert_eq!(stats.udp_requests_banned, 1);
    }

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

        assert_eq!(stats.udp4_connections_handled, 1);
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
                kind: crate::event::UdpRequestKind::Announce,
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_announces_handled, 1);
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

        assert_eq!(stats.udp4_scrapes_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_responses_counter_when_it_receives_a_udp4_response_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpResponseSent {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
                kind: crate::event::UdpResponseKind::Ok {
                    req_kind: UdpRequestKind::Announce,
                },
                req_processing_time: std::time::Duration::from_secs(1),
            },
            &stats_repository,
            CurrentClock::now(),
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

        assert_eq!(stats.udp4_errors_handled, 1);
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

        assert_eq!(stats.udp6_connections_handled, 1);
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
                kind: crate::event::UdpRequestKind::Announce,
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_announces_handled, 1);
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

        assert_eq!(stats.udp6_scrapes_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_response_counter_when_it_receives_a_udp6_response_event() {
        let stats_repository = Repository::new();

        handle_event(
            Event::UdpResponseSent {
                context: ConnectionContext::new(
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 195)), 8080),
                    ServiceBinding::new(
                        Protocol::UDP,
                        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 203, 0, 113, 196)), 6969),
                    )
                    .unwrap(),
                ),
                kind: crate::event::UdpResponseKind::Ok {
                    req_kind: UdpRequestKind::Announce,
                },
                req_processing_time: std::time::Duration::from_secs(1),
            },
            &stats_repository,
            CurrentClock::now(),
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

        assert_eq!(stats.udp6_errors_handled, 1);
    }
}
