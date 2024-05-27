//! Aquatic responses are not serializable. These are the serializable wrappers.
use std::net::{Ipv4Addr, Ipv6Addr};

use anyhow::Context;
use aquatic_udp_protocol::Response::{self};
use aquatic_udp_protocol::{AnnounceResponse, ConnectResponse, ErrorResponse, Ipv4AddrBytes, Ipv6AddrBytes, ScrapeResponse};
use serde::Serialize;

pub trait DtoToJson {
    /// # Errors
    ///
    /// Will return an error if serialization fails.
    ///
    fn print_response(&self) -> anyhow::Result<()>
    where
        Self: Serialize,
    {
        let pretty_json = serde_json::to_string_pretty(self).context("response JSON serialization")?;
        println!("{pretty_json}");

        Ok(())
    }
}

#[derive(Serialize)]
pub enum ResponseDto {
    Connect(ConnectResponseDto),
    AnnounceIpv4(AnnounceResponseDto),
    AnnounceIpv6(AnnounceResponseDto),
    Scrape(ScrapeResponseDto),
    Error(ErrorResponseDto),
}

impl From<Response> for ResponseDto {
    fn from(response: Response) -> Self {
        match response {
            Response::Connect(response) => ResponseDto::Connect(ConnectResponseDto::from(response)),
            Response::AnnounceIpv4(response) => ResponseDto::AnnounceIpv4(AnnounceResponseDto::from(response)),
            Response::AnnounceIpv6(response) => ResponseDto::AnnounceIpv6(AnnounceResponseDto::from(response)),
            Response::Scrape(response) => ResponseDto::Scrape(ScrapeResponseDto::from(response)),
            Response::Error(response) => ResponseDto::Error(ErrorResponseDto::from(response)),
        }
    }
}

impl DtoToJson for ResponseDto {}

#[derive(Serialize)]
pub struct ConnectResponseDto {
    transaction_id: i32,
    connection_id: i64,
}

impl From<ConnectResponse> for ConnectResponseDto {
    fn from(connect: ConnectResponse) -> Self {
        Self {
            transaction_id: connect.transaction_id.0.into(),
            connection_id: connect.connection_id.0.into(),
        }
    }
}

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
pub struct ErrorResponseDto {
    transaction_id: i32,
    message: String,
}

impl From<ErrorResponse> for ErrorResponseDto {
    fn from(error: ErrorResponse) -> Self {
        Self {
            transaction_id: error.transaction_id.0.into(),
            message: error.message.to_string(),
        }
    }
}

#[derive(Serialize)]
struct TorrentStats {
    seeders: i32,
    completed: i32,
    leechers: i32,
}
