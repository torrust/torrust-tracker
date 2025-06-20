use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric_name;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::ConnectionContext;
use crate::statistics::repository::Repository;
use crate::statistics::UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL;

pub async fn handle_event(context: ConnectionContext, stats_repository: &Repository, now: DurationSinceUnixEpoch) {
    match stats_repository
        .increase_counter(
            &metric_name!(UDP_TRACKER_SERVER_REQUESTS_ABORTED_TOTAL),
            &LabelSet::from(context),
            now,
        )
        .await
    {
        Ok(()) => {}
        Err(err) => tracing::error!("Failed to increase the counter: {}", err),
    };
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

    use crate::event::{ConnectionContext, Event};
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

        assert_eq!(stats.udp_requests_aborted_total(), 1);
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
        assert_eq!(stats.udp_requests_aborted_total(), 1);
    }
}
