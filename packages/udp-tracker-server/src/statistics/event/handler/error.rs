use aquatic_udp_protocol::PeerClient;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::{label_name, metric_name};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::{ConnectionContext, ErrorKind, UdpRequestKind};
use crate::statistics::repository::Repository;
use crate::statistics::{UDP_TRACKER_SERVER_CONNECTION_ID_ERRORS_TOTAL, UDP_TRACKER_SERVER_ERRORS_TOTAL};

pub async fn handle_event(
    connection_context: ConnectionContext,
    opt_udp_request_kind: Option<UdpRequestKind>,
    error_kind: ErrorKind,
    repository: &Repository,
    now: DurationSinceUnixEpoch,
) {
    update_extendable_metrics(&connection_context, opt_udp_request_kind, error_kind, repository, now).await;
}

async fn update_extendable_metrics(
    connection_context: &ConnectionContext,
    opt_udp_request_kind: Option<UdpRequestKind>,
    error_kind: ErrorKind,
    repository: &Repository,
    now: DurationSinceUnixEpoch,
) {
    update_all_errors_counter(connection_context, opt_udp_request_kind.clone(), repository, now).await;
    update_connection_id_errors_counter(opt_udp_request_kind, error_kind, repository, now).await;
}

async fn update_all_errors_counter(
    connection_context: &ConnectionContext,
    opt_udp_request_kind: Option<UdpRequestKind>,
    repository: &Repository,
    now: DurationSinceUnixEpoch,
) {
    let mut label_set = LabelSet::from(connection_context.clone());

    if let Some(kind) = opt_udp_request_kind.clone() {
        label_set.upsert(label_name!("request_kind"), kind.to_string().into());
    }

    match repository
        .increase_counter(&metric_name!(UDP_TRACKER_SERVER_ERRORS_TOTAL), &label_set, now)
        .await
    {
        Ok(()) => {}
        Err(err) => tracing::error!("Failed to increase the counter: {}", err),
    }
}

async fn update_connection_id_errors_counter(
    opt_udp_request_kind: Option<UdpRequestKind>,
    error_kind: ErrorKind,
    repository: &Repository,
    now: DurationSinceUnixEpoch,
) {
    if let ErrorKind::ConnectionCookie(_) = error_kind {
        if let Some(UdpRequestKind::Announce { announce_request }) = opt_udp_request_kind {
            let (client_software_name, client_software_version) = extract_name_and_version(&announce_request.peer_id.client());

            let label_set = LabelSet::from([
                (label_name!("client_software_name"), client_software_name.into()),
                (label_name!("client_software_version"), client_software_version.into()),
            ]);

            match repository
                .increase_counter(&metric_name!(UDP_TRACKER_SERVER_CONNECTION_ID_ERRORS_TOTAL), &label_set, now)
                .await
            {
                Ok(()) => {}
                Err(err) => tracing::error!("Failed to increase the counter: {}", err),
            };
        }
    }
}

fn extract_name_and_version(peer_client: &PeerClient) -> (String, String) {
    match peer_client {
        PeerClient::BitTorrent(compact_string) => ("BitTorrent".to_string(), compact_string.as_str().to_owned()),
        PeerClient::Deluge(compact_string) => ("Deluge".to_string(), compact_string.as_str().to_owned()),
        PeerClient::LibTorrentRakshasa(compact_string) => ("lt (rakshasa)".to_string(), compact_string.as_str().to_owned()),
        PeerClient::LibTorrentRasterbar(compact_string) => ("lt (rasterbar)".to_string(), compact_string.as_str().to_owned()),
        PeerClient::QBitTorrent(compact_string) => ("QBitTorrent".to_string(), compact_string.as_str().to_owned()),
        PeerClient::Transmission(compact_string) => ("Transmission".to_string(), compact_string.as_str().to_owned()),
        PeerClient::UTorrent(compact_string) => ("µTorrent".to_string(), compact_string.as_str().to_owned()),
        PeerClient::UTorrentEmbedded(compact_string) => ("µTorrent Emb.".to_string(), compact_string.as_str().to_owned()),
        PeerClient::UTorrentMac(compact_string) => ("µTorrent Mac".to_string(), compact_string.as_str().to_owned()),
        PeerClient::UTorrentWeb(compact_string) => ("µTorrent Web".to_string(), compact_string.as_str().to_owned()),
        PeerClient::Vuze(compact_string) => ("Vuze".to_string(), compact_string.as_str().to_owned()),
        PeerClient::WebTorrent(compact_string) => ("WebTorrent".to_string(), compact_string.as_str().to_owned()),
        PeerClient::WebTorrentDesktop(compact_string) => ("WebTorrent Desktop".to_string(), compact_string.as_str().to_owned()),
        PeerClient::Mainline(compact_string) => ("Mainline".to_string(), compact_string.as_str().to_owned()),
        PeerClient::OtherWithPrefixAndVersion { prefix, version } => {
            (format!("Other ({})", prefix.as_str()), version.as_str().to_owned())
        }
        PeerClient::OtherWithPrefix(compact_string) => (format!("Other ({compact_string})"), String::new()),
        PeerClient::Other => ("Other".to_string(), String::new()),
        _ => ("Unknown".to_string(), String::new()),
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use torrust_tracker_clock::clock::Time;
    use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

    use crate::event::{ConnectionContext, Event};
    use crate::statistics::event::handler::error::ErrorKind;
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::repository::Repository;
    use crate::CurrentClock;

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
                kind: None,
                error: ErrorKind::RequestParse("Invalid request format".to_string()),
            },
            &stats_repository,
            CurrentClock::now(),
        )
        .await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_errors_total(), 1);
    }
}
