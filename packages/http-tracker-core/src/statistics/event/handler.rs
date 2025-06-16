use std::sync::Arc;

use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::{label_name, metric_name};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::Event;
use crate::statistics::repository::Repository;
use crate::statistics::HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL;

pub async fn handle_event(event: Event, stats_repository: &Arc<Repository>, now: DurationSinceUnixEpoch) {
    match event {
        Event::TcpAnnounce { connection, .. } => {
            let mut label_set = LabelSet::from(connection);
            label_set.upsert(label_name!("request_kind"), LabelValue::new("announce"));

            match stats_repository
                .increase_counter(&metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), &label_set, now)
                .await
            {
                Ok(()) => {
                    tracing::debug!(
                        "Successfully increased the counter for HTTP announce requests received: {}",
                        label_set
                    );
                }
                Err(err) => tracing::error!("Failed to increase the counter: {}", err),
            };
        }
        Event::TcpScrape { connection } => {
            let mut label_set = LabelSet::from(connection);
            label_set.upsert(label_name!("request_kind"), LabelValue::new("scrape"));

            match stats_repository
                .increase_counter(&metric_name!(HTTP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL), &label_set, now)
                .await
            {
                Ok(()) => {
                    tracing::debug!(
                        "Successfully increased the counter for HTTP scrape requests received: {}",
                        label_set
                    );
                }
                Err(err) => tracing::error!("Failed to increase the counter: {}", err),
            };
        }
    }

    tracing::debug!("stats: {:?}", stats_repository.get_stats().await);
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
    use std::sync::Arc;

    use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::{RemoteClientAddr, ResolvedIp};
    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

    use crate::event::{ConnectionContext, Event};
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::repository::Repository;
    use crate::tests::{sample_info_hash, sample_peer_using_ipv4, sample_peer_using_ipv6};
    use crate::CurrentClock;

    #[tokio::test]
    async fn should_increase_the_tcp4_announces_counter_when_it_receives_a_tcp4_announce_event() {
        let stats_repository = Arc::new(Repository::new());
        let peer = sample_peer_using_ipv4();
        let remote_client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2));

        handle_event(
            Event::TcpAnnounce {
                connection: ConnectionContext::new(
                    RemoteClientAddr::new(ResolvedIp::FromSocketAddr(remote_client_ip), Some(8080)),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
                ),
                info_hash: sample_info_hash(),
                announcement: peer,
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp4_announces_handled(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_tcp4_scrapes_counter_when_it_receives_a_tcp4_scrape_event() {
        let stats_repository = Arc::new(Repository::new());

        handle_event(
            Event::TcpScrape {
                connection: ConnectionContext::new(
                    RemoteClientAddr::new(
                        ResolvedIp::FromSocketAddr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2))),
                        Some(8080),
                    ),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp4_scrapes_handled(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_tcp6_announces_counter_when_it_receives_a_tcp6_announce_event() {
        let stats_repository = Arc::new(Repository::new());
        let peer = sample_peer_using_ipv6();
        let remote_client_ip = IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969));

        handle_event(
            Event::TcpAnnounce {
                connection: ConnectionContext::new(
                    RemoteClientAddr::new(ResolvedIp::FromSocketAddr(remote_client_ip), Some(8080)),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 7070)).unwrap(),
                ),
                info_hash: sample_info_hash(),
                announcement: peer,
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp6_announces_handled(), 1);
    }

    #[tokio::test]
    async fn should_increase_the_tcp6_scrapes_counter_when_it_receives_a_tcp6_scrape_event() {
        let stats_repository = Arc::new(Repository::new());

        handle_event(
            Event::TcpScrape {
                connection: ConnectionContext::new(
                    RemoteClientAddr::new(
                        ResolvedIp::FromSocketAddr(IpAddr::V6(Ipv6Addr::new(
                            0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969,
                        ))),
                        Some(8080),
                    ),
                    ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 7070)).unwrap(),
                ),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.tcp6_scrapes_handled(), 1);
    }
}
