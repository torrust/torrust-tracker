use std::net::IpAddr;
use std::sync::Arc;

use crate::protocol::info_hash::InfoHash;
use crate::tracker::{statistics, ScrapeData, Tracker};

pub async fn invoke(tracker: Arc<Tracker>, info_hashes: &Vec<InfoHash>, original_peer_ip: &IpAddr) -> ScrapeData {
    let scrape_data = tracker.scrape(info_hashes).await;

    match original_peer_ip {
        IpAddr::V4(_) => {
            tracker.send_stats_event(statistics::Event::Tcp4Scrape).await;
        }
        IpAddr::V6(_) => {
            tracker.send_stats_event(statistics::Event::Tcp6Scrape).await;
        }
    }

    scrape_data
}
