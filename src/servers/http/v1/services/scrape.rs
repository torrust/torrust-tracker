use std::net::IpAddr;
use std::sync::Arc;

use crate::shared::bit_torrent::info_hash::InfoHash;
use crate::tracker::{statistics, ScrapeData, Tracker};

pub async fn invoke(tracker: &Arc<Tracker>, info_hashes: &Vec<InfoHash>, original_peer_ip: &IpAddr) -> ScrapeData {
    let scrape_data = tracker.scrape(info_hashes).await;

    send_scrape_event(original_peer_ip, tracker).await;

    scrape_data
}

/// When the peer is not authenticated and the tracker is running in `private` mode,
/// the tracker returns empty stats for all the torrents.
pub async fn fake(tracker: &Arc<Tracker>, info_hashes: &Vec<InfoHash>, original_peer_ip: &IpAddr) -> ScrapeData {
    send_scrape_event(original_peer_ip, tracker).await;

    ScrapeData::zeroed(info_hashes)
}

async fn send_scrape_event(original_peer_ip: &IpAddr, tracker: &Arc<Tracker>) {
    match original_peer_ip {
        IpAddr::V4(_) => {
            tracker.send_stats_event(statistics::Event::Tcp4Scrape).await;
        }
        IpAddr::V6(_) => {
            tracker.send_stats_event(statistics::Event::Tcp6Scrape).await;
        }
    }
}

#[cfg(test)]
mod tests {

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
    use torrust_tracker_test_helpers::configuration;

    use crate::shared::bit_torrent::info_hash::InfoHash;
    use crate::shared::clock::DurationSinceUnixEpoch;
    use crate::tracker::services::common::tracker_factory;
    use crate::tracker::{peer, Tracker};

    fn public_tracker() -> Tracker {
        tracker_factory(configuration::ephemeral_mode_public().into())
    }

    fn sample_info_hashes() -> Vec<InfoHash> {
        vec![sample_info_hash()]
    }

    fn sample_info_hash() -> InfoHash {
        "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap()
    }

    fn sample_peer() -> peer::Peer {
        peer::Peer {
            peer_id: peer::Id(*b"-qB00000000000000000"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes(0),
            downloaded: NumberOfBytes(0),
            left: NumberOfBytes(0),
            event: AnnounceEvent::Started,
        }
    }

    mod with_real_data {

        use std::future;
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
        use std::sync::Arc;

        use mockall::predicate::eq;
        use torrust_tracker_test_helpers::configuration;

        use crate::servers::http::v1::services::scrape::invoke;
        use crate::servers::http::v1::services::scrape::tests::{
            public_tracker, sample_info_hash, sample_info_hashes, sample_peer,
        };
        use crate::tracker::torrent::SwarmMetadata;
        use crate::tracker::{statistics, ScrapeData, Tracker};

        #[tokio::test]
        async fn it_should_return_the_scrape_data_for_a_torrent() {
            let tracker = Arc::new(public_tracker());

            let info_hash = sample_info_hash();
            let info_hashes = vec![info_hash];

            // Announce a new peer to force scrape data to contain not zeroed data
            let mut peer = sample_peer();
            let original_peer_ip = peer.ip();
            tracker.announce(&info_hash, &mut peer, &original_peer_ip).await;

            let scrape_data = invoke(&tracker, &info_hashes, &original_peer_ip).await;

            let mut expected_scrape_data = ScrapeData::empty();
            expected_scrape_data.add_file(
                &info_hash,
                SwarmMetadata {
                    complete: 1,
                    downloaded: 0,
                    incomplete: 0,
                },
            );

            assert_eq!(scrape_data, expected_scrape_data);
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_4_scrape_event_when_the_peer_uses_ipv4() {
            let mut stats_event_sender_mock = statistics::MockEventSender::new();
            stats_event_sender_mock
                .expect_send_event()
                .with(eq(statistics::Event::Tcp4Scrape))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(())))));
            let stats_event_sender = Box::new(stats_event_sender_mock);

            let tracker = Arc::new(
                Tracker::new(
                    Arc::new(configuration::ephemeral()),
                    Some(stats_event_sender),
                    statistics::Repo::new(),
                )
                .unwrap(),
            );

            let peer_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1));

            invoke(&tracker, &sample_info_hashes(), &peer_ip).await;
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_6_scrape_event_when_the_peer_uses_ipv6() {
            let mut stats_event_sender_mock = statistics::MockEventSender::new();
            stats_event_sender_mock
                .expect_send_event()
                .with(eq(statistics::Event::Tcp6Scrape))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(())))));
            let stats_event_sender = Box::new(stats_event_sender_mock);

            let tracker = Arc::new(
                Tracker::new(
                    Arc::new(configuration::ephemeral()),
                    Some(stats_event_sender),
                    statistics::Repo::new(),
                )
                .unwrap(),
            );

            let peer_ip = IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969));

            invoke(&tracker, &sample_info_hashes(), &peer_ip).await;
        }
    }

    mod with_zeroed_data {

        use std::future;
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
        use std::sync::Arc;

        use mockall::predicate::eq;
        use torrust_tracker_test_helpers::configuration;

        use crate::servers::http::v1::services::scrape::fake;
        use crate::servers::http::v1::services::scrape::tests::{
            public_tracker, sample_info_hash, sample_info_hashes, sample_peer,
        };
        use crate::tracker::{statistics, ScrapeData, Tracker};

        #[tokio::test]
        async fn it_should_always_return_the_zeroed_scrape_data_for_a_torrent() {
            let tracker = Arc::new(public_tracker());

            let info_hash = sample_info_hash();
            let info_hashes = vec![info_hash];

            // Announce a new peer to force scrape data to contain not zeroed data
            let mut peer = sample_peer();
            let original_peer_ip = peer.ip();
            tracker.announce(&info_hash, &mut peer, &original_peer_ip).await;

            let scrape_data = fake(&tracker, &info_hashes, &original_peer_ip).await;

            let expected_scrape_data = ScrapeData::zeroed(&info_hashes);

            assert_eq!(scrape_data, expected_scrape_data);
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_4_scrape_event_when_the_peer_uses_ipv4() {
            let mut stats_event_sender_mock = statistics::MockEventSender::new();
            stats_event_sender_mock
                .expect_send_event()
                .with(eq(statistics::Event::Tcp4Scrape))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(())))));
            let stats_event_sender = Box::new(stats_event_sender_mock);

            let tracker = Arc::new(
                Tracker::new(
                    Arc::new(configuration::ephemeral()),
                    Some(stats_event_sender),
                    statistics::Repo::new(),
                )
                .unwrap(),
            );

            let peer_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1));

            fake(&tracker, &sample_info_hashes(), &peer_ip).await;
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_6_scrape_event_when_the_peer_uses_ipv6() {
            let mut stats_event_sender_mock = statistics::MockEventSender::new();
            stats_event_sender_mock
                .expect_send_event()
                .with(eq(statistics::Event::Tcp6Scrape))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(())))));
            let stats_event_sender = Box::new(stats_event_sender_mock);

            let tracker = Arc::new(
                Tracker::new(
                    Arc::new(configuration::ephemeral()),
                    Some(stats_event_sender),
                    statistics::Repo::new(),
                )
                .unwrap(),
            );

            let peer_ip = IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969));

            fake(&tracker, &sample_info_hashes(), &peer_ip).await;
        }
    }
}
