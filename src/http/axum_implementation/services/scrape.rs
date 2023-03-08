use std::net::IpAddr;
use std::sync::Arc;

use crate::protocol::info_hash::InfoHash;
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
