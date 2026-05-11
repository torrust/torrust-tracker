//! UDP Tracker client:
//!
//! Examples:
//!
//! Announce request (minimal):
//!
//! ```text
//! cargo run --bin udp_tracker_client announce 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! ```
//!
//! Announce request (all optional parameters):
//!
//! ```text
//! cargo run --bin udp_tracker_client announce \
//!   127.0.0.1:6969 443c7602b4fde83d1154d6d9da48808418b181b6 \
//!   --event completed \
//!   --uploaded 1234 \
//!   --downloaded 5678 \
//!   --left 0 \
//!   --port 6881 \
//!   --ip-address 10.0.0.1 \
//!   '--peer-id=-RC00000000000000001' \
//!   --key 42 \
//!   --peers-wanted 50 | jq
//! ```
//!
//! Announce response:
//!
//! ```json
//! {
//!   "transaction_id": -888840697
//!   "announce_interval": 120,
//!   "leechers": 0,
//!   "seeders": 1,
//!   "peers": [
//!     "123.123.123.123:51289"
//!   ],
//! }
//! ```
//!
//! Scrape request:
//!
//! ```text
//! cargo run --bin udp_tracker_client scrape 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! ```
//!
//! Scrape response:
//!
//! ```json
//! {
//!   "transaction_id": -888840697,
//!   "torrent_stats": [
//!     {
//!       "completed": 0,
//!       "leechers": 0,
//!       "seeders": 0
//!     },
//!     {
//!       "completed": 0,
//!       "leechers": 0,
//!       "seeders": 0
//!    }
//!  ]
//! }
//! ```
//!
//! Unrecognized UDP response:
//!
//! ```text
//! Error: Unrecognized UDP tracker response. Expected a valid UDP response, got: [0, 0, 0, 1]
//! ```
//!
//! You can use an URL with instead of the socket address. For example:
//!
//! ```text
//! cargo run --bin udp_tracker_client scrape udp://localhost:6969 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! cargo run --bin udp_tracker_client scrape udp://localhost:6969/scrape 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! ```
//!
//! The protocol (`udp://`) in the URL is mandatory. The path (`\scrape`) is optional. It always uses `\scrape`.
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::str::FromStr;

use anyhow::Context;
use bittorrent_primitives::info_hash::InfoHash as TorrustInfoHash;
use bittorrent_udp_tracker_protocol::{AnnounceEvent, Response, TransactionId};
use clap::{Parser, Subcommand, ValueEnum};
use torrust_tracker_configuration::DEFAULT_TIMEOUT;
use tracing::level_filters::LevelFilter;
use url::Url;

use super::Error;
use crate::console::clients::udp::checker;
use crate::console::clients::udp::checker::AnnounceParams;
use crate::console::clients::udp::responses::dto::SerializableResponse;
use crate::console::clients::udp::responses::json::ToJson;

const RANDOM_TRANSACTION_ID: i32 = -888_840_697;

/// CLI representation of `AnnounceEvent`. Keeps `clap` out of the protocol layer.
#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliAnnounceEvent {
    None,
    Completed,
    Started,
    Stopped,
}

impl From<CliAnnounceEvent> for AnnounceEvent {
    fn from(value: CliAnnounceEvent) -> Self {
        match value {
            CliAnnounceEvent::None => AnnounceEvent::None,
            CliAnnounceEvent::Completed => AnnounceEvent::Completed,
            CliAnnounceEvent::Started => AnnounceEvent::Started,
            CliAnnounceEvent::Stopped => AnnounceEvent::Stopped,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Announce {
        #[arg(value_parser = parse_socket_addr)]
        tracker_socket_addr: SocketAddr,
        #[arg(value_parser = parse_info_hash)]
        info_hash: TorrustInfoHash,
        #[arg(long)]
        event: Option<CliAnnounceEvent>,
        #[arg(long)]
        uploaded: Option<u64>,
        #[arg(long)]
        downloaded: Option<u64>,
        #[arg(long)]
        left: Option<u64>,
        #[arg(long, value_parser = parse_non_zero_port)]
        port: Option<u16>,
        #[arg(long = "ip-address")]
        ip_address: Option<Ipv4Addr>,
        #[arg(long = "peer-id", value_parser = parse_peer_id)]
        peer_id: Option<[u8; 20]>,
        #[arg(long)]
        key: Option<i32>,
        #[arg(long = "peers-wanted")]
        peers_wanted: Option<i32>,
    },
    Scrape {
        #[arg(value_parser = parse_socket_addr)]
        tracker_socket_addr: SocketAddr,
        #[arg(value_parser = parse_info_hash, num_args = 1..=74, value_delimiter = ' ')]
        info_hashes: Vec<TorrustInfoHash>,
    },
}

/// # Errors
///
/// Will return an error if the command fails.
///
///
pub async fn run() -> anyhow::Result<()> {
    tracing_stdout_init(LevelFilter::INFO);

    let args = Args::parse();

    let response = match args.command {
        Command::Announce {
            tracker_socket_addr: remote_addr,
            info_hash,
            event,
            uploaded,
            downloaded,
            left,
            port,
            ip_address,
            peer_id,
            key,
            peers_wanted,
        } => {
            let params = AnnounceParams {
                event: event.map(Into::into),
                uploaded: uploaded
                    .map(i64::try_from)
                    .transpose()
                    .context("--uploaded value is too large to fit in i64")?,
                downloaded: downloaded
                    .map(i64::try_from)
                    .transpose()
                    .context("--downloaded value is too large to fit in i64")?,
                left: left
                    .map(i64::try_from)
                    .transpose()
                    .context("--left value is too large to fit in i64")?,
                port,
                ip_address,
                peer_id,
                key,
                peers_wanted,
            };
            handle_announce(remote_addr, &info_hash, &params).await?
        }
        Command::Scrape {
            tracker_socket_addr: remote_addr,
            info_hashes,
        } => handle_scrape(remote_addr, &info_hashes).await?,
    };

    let response: SerializableResponse = response.into();
    let response_json = response.to_json_string()?;

    print!("{response_json}");

    Ok(())
}

fn tracing_stdout_init(filter: LevelFilter) {
    tracing_subscriber::fmt().with_max_level(filter).init();
    tracing::debug!("Logging initialized");
}

async fn handle_announce(
    remote_addr: SocketAddr,
    info_hash: &TorrustInfoHash,
    params: &AnnounceParams,
) -> Result<Response, Error> {
    let transaction_id = TransactionId::new(RANDOM_TRANSACTION_ID);

    let client = checker::Client::new(remote_addr, DEFAULT_TIMEOUT).await?;

    let connection_id = client.send_connection_request(transaction_id).await?;

    client
        .send_announce_request(transaction_id, connection_id, *info_hash, params)
        .await
}

async fn handle_scrape(remote_addr: SocketAddr, info_hashes: &[TorrustInfoHash]) -> Result<Response, Error> {
    let transaction_id = TransactionId::new(RANDOM_TRANSACTION_ID);

    let client = checker::Client::new(remote_addr, DEFAULT_TIMEOUT).await?;

    let connection_id = client.send_connection_request(transaction_id).await?;

    client.send_scrape_request(connection_id, transaction_id, info_hashes).await
}

fn parse_socket_addr(tracker_socket_addr_str: &str) -> anyhow::Result<SocketAddr> {
    tracing::debug!("Tracker socket address: {tracker_socket_addr_str:#?}");

    // Check if the address is a valid URL. If so, extract the host and port.
    let resolved_addr = if let Ok(url) = Url::parse(tracker_socket_addr_str) {
        tracing::debug!("Tracker socket address URL: {url:?}");

        let host = url
            .host_str()
            .with_context(|| format!("invalid host in URL: `{tracker_socket_addr_str}`"))?
            .to_owned();

        let port = url
            .port()
            .with_context(|| format!("port not found in URL: `{tracker_socket_addr_str}`"))?
            .to_owned();

        (host, port)
    } else {
        // If not a URL, assume it's a host:port pair.

        let parts: Vec<&str> = tracker_socket_addr_str.split(':').collect();

        if parts.len() != 2 {
            return Err(anyhow::anyhow!(
                "invalid address format: `{tracker_socket_addr_str}`. Expected format is host:port"
            ));
        }

        let host = parts[0].to_owned();

        let port = parts[1]
            .parse::<u16>()
            .with_context(|| format!("invalid port: `{}`", parts[1]))?
            .to_owned();

        (host, port)
    };

    tracing::debug!("Resolved address: {resolved_addr:#?}");

    // Perform DNS resolution.
    let socket_addrs: Vec<_> = resolved_addr.to_socket_addrs()?.collect();
    if socket_addrs.is_empty() {
        Err(anyhow::anyhow!("DNS resolution failed for `{tracker_socket_addr_str}`"))
    } else {
        Ok(socket_addrs[0])
    }
}

fn parse_info_hash(info_hash_str: &str) -> anyhow::Result<TorrustInfoHash> {
    TorrustInfoHash::from_str(info_hash_str)
        .map_err(|e| anyhow::Error::msg(format!("failed to parse info-hash `{info_hash_str}`: {e:?}")))
}

fn parse_peer_id(peer_id_str: &str) -> anyhow::Result<[u8; 20]> {
    let bytes = peer_id_str.as_bytes();
    if bytes.len() != 20 {
        return Err(anyhow::anyhow!(
            "peer-id must be exactly 20 bytes, got {} bytes for `{peer_id_str}`",
            bytes.len()
        ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(bytes);

    Ok(arr)
}

fn parse_non_zero_port(port_str: &str) -> anyhow::Result<u16> {
    let port = u16::from_str(port_str).with_context(|| format!("invalid port value: `{port_str}`"))?;

    if port == 0 {
        anyhow::bail!("port must be greater than zero")
    }

    Ok(port)
}
