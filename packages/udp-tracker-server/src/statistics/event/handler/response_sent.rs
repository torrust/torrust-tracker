use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::{label_name, metric_name};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::{ConnectionContext, UdpRequestKind, UdpResponseKind};
use crate::statistics::repository::Repository;
use crate::statistics::UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL;

pub async fn handle_event(
    context: ConnectionContext,
    kind: UdpResponseKind,
    req_processing_time: std::time::Duration,
    stats_repository: &Repository,
    now: DurationSinceUnixEpoch,
) {
    let (result_label_value, kind_label_value) = match kind {
        UdpResponseKind::Ok { req_kind } => match req_kind {
            UdpRequestKind::Connect => {
                let mut label_set = LabelSet::from(context.clone());
                label_set.upsert(label_name!("request_kind"), LabelValue::new(&req_kind.to_string()));

                let _new_avg = stats_repository
                    .recalculate_udp_avg_processing_time_ns(req_processing_time, &label_set, now)
                    .await;

                (LabelValue::new("ok"), UdpRequestKind::Connect.into())
            }
            UdpRequestKind::Announce { announce_request } => {
                let mut label_set = LabelSet::from(context.clone());
                label_set.upsert(label_name!("request_kind"), LabelValue::new(&req_kind.to_string()));

                let _new_avg = stats_repository
                    .recalculate_udp_avg_processing_time_ns(req_processing_time, &label_set, now)
                    .await;

                (LabelValue::new("ok"), UdpRequestKind::Announce { announce_request }.into())
            }
            UdpRequestKind::Scrape => {
                let mut label_set = LabelSet::from(context.clone());
                label_set.upsert(label_name!("request_kind"), LabelValue::new(&req_kind.to_string()));

                let _new_avg = stats_repository
                    .recalculate_udp_avg_processing_time_ns(req_processing_time, &label_set, now)
                    .await;

                (LabelValue::new("ok"), LabelValue::new(&UdpRequestKind::Scrape.to_string()))
            }
        },
        UdpResponseKind::Error { opt_req_kind: _ } => (LabelValue::new("error"), LabelValue::ignore()),
    };

    // Increase the number of responses sent
    let mut label_set = LabelSet::from(context);
    if result_label_value == LabelValue::new("ok") {
        label_set.upsert(label_name!("request_kind"), kind_label_value);
    }
    label_set.upsert(label_name!("result"), result_label_value);
    match stats_repository
        .increase_counter(&metric_name!(UDP_TRACKER_SERVER_RESPONSES_SENT_TOTAL), &label_set, now)
        .await
    {
        Ok(()) => {}
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
                    req_kind: crate::event::UdpRequestKind::Announce {
                        announce_request: AnnounceRequestBuilder::default().into(),
                    },
                },
                req_processing_time: std::time::Duration::from_secs(1),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_responses_sent_total(), 1);
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
                    req_kind: crate::event::UdpRequestKind::Announce {
                        announce_request: AnnounceRequestBuilder::default().into(),
                    },
                },
                req_processing_time: std::time::Duration::from_secs(1),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_responses_sent_total(), 1);
    }
}
