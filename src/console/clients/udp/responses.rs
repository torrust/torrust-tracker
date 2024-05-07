//! Aquatic responses are not serializable. These are the serializable wrappers.
use std::net::{Ipv4Addr, Ipv6Addr};

use aquatic_udp_protocol::{AnnounceResponse, Ipv4AddrBytes, Ipv6AddrBytes, ScrapeResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct AnnounceResponseDto {
    transaction_id: i32,
    announce_interval: i32,
    leechers: i32,
    seeders: i32,
    peers: Vec<String>,
}

impl From<AnnounceResponse<Ipv4AddrBytes>> for AnnounceResponseDto {
    fn from(announce: AnnounceResponse<Ipv4AddrBytes>) -> Self {
        Self {
            transaction_id: announce.fixed.transaction_id.0.into(),
            announce_interval: announce.fixed.announce_interval.0.into(),
            leechers: announce.fixed.leechers.0.into(),
            seeders: announce.fixed.seeders.0.into(),
            peers: announce
                .peers
                .iter()
                .map(|peer| format!("{}:{}", Ipv4Addr::from(peer.ip_address), peer.port.0))
                .collect::<Vec<_>>(),
        }
    }
}

impl From<AnnounceResponse<Ipv6AddrBytes>> for AnnounceResponseDto {
    fn from(announce: AnnounceResponse<Ipv6AddrBytes>) -> Self {
        Self {
            transaction_id: announce.fixed.transaction_id.0.into(),
            announce_interval: announce.fixed.announce_interval.0.into(),
            leechers: announce.fixed.leechers.0.into(),
            seeders: announce.fixed.seeders.0.into(),
            peers: announce
                .peers
                .iter()
                .map(|peer| format!("{}:{}", Ipv6Addr::from(peer.ip_address), peer.port.0))
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
            transaction_id: scrape.transaction_id.0.into(),
            torrent_stats: scrape
                .torrent_stats
                .iter()
                .map(|torrent_scrape_statistics| TorrentStats {
                    seeders: torrent_scrape_statistics.seeders.0.into(),
                    completed: torrent_scrape_statistics.completed.0.into(),
                    leechers: torrent_scrape_statistics.leechers.0.into(),
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Serialize)]
struct TorrentStats {
    seeders: i32,
    completed: i32,
    leechers: i32,
}
