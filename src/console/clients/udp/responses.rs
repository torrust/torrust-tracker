//! Aquatic responses are not serializable. These are the serializable wrappers.
use std::net::{Ipv4Addr, Ipv6Addr};

use aquatic_udp_protocol::{AnnounceResponse, ScrapeResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct AnnounceResponseDto {
    transaction_id: i32,
    announce_interval: i32,
    leechers: i32,
    seeders: i32,
    peers: Vec<String>,
}

impl From<AnnounceResponse<Ipv4Addr>> for AnnounceResponseDto {
    fn from(announce: AnnounceResponse<Ipv4Addr>) -> Self {
        Self {
            transaction_id: announce.transaction_id.0,
            announce_interval: announce.announce_interval.0,
            leechers: announce.leechers.0,
            seeders: announce.seeders.0,
            peers: announce
                .peers
                .iter()
                .map(|peer| format!("{}:{}", peer.ip_address, peer.port.0))
                .collect::<Vec<_>>(),
        }
    }
}

impl From<AnnounceResponse<Ipv6Addr>> for AnnounceResponseDto {
    fn from(announce: AnnounceResponse<Ipv6Addr>) -> Self {
        Self {
            transaction_id: announce.transaction_id.0,
            announce_interval: announce.announce_interval.0,
            leechers: announce.leechers.0,
            seeders: announce.seeders.0,
            peers: announce
                .peers
                .iter()
                .map(|peer| format!("{}:{}", peer.ip_address, peer.port.0))
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Serialize)]
pub struct ScrapeResponseDto {
    transaction_id: i32,
    torrent_stats: Vec<TorrentStats>,
}

impl From<ScrapeResponse> for ScrapeResponseDto {
    fn from(scrape: ScrapeResponse) -> Self {
        Self {
            transaction_id: scrape.transaction_id.0,
            torrent_stats: scrape
                .torrent_stats
                .iter()
                .map(|torrent_scrape_statistics| TorrentStats {
                    seeders: torrent_scrape_statistics.seeders.0,
                    completed: torrent_scrape_statistics.completed.0,
                    leechers: torrent_scrape_statistics.leechers.0,
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Serialize)]
struct Peer {
    seeders: i32,
    completed: i32,
    leechers: i32,
}

#[derive(Serialize)]
struct TorrentStats {
    seeders: i32,
    completed: i32,
    leechers: i32,
}
