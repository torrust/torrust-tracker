//! HTTP Tracker client:
//!
//! Examples:
//!
//! `Announce` request:
//!
//! ```text
//! cargo run --bin http_tracker_client announce http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! ```
//!
//! `Announce` request (all optional parameters):
//!
//! ```text
//! cargo run --bin http_tracker_client announce \
//!   http://127.0.0.1:7070 443c7602b4fde83d1154d6d9da48808418b181b6 \
//!   --event completed \
//!   --uploaded 1234 \
//!   --downloaded 5678 \
//!   --left 0 \
//!   --port 6881 \
//!   --peer-addr 10.0.0.1 \
//!   '--peer-id=-RC00000000000000001' \
//!   --compact 1 | jq
//! ```
//!
//! `Scrape` request:
//!
//! ```text
//! cargo run --bin http_tracker_client scrape http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! ```
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::Context;
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_client::http::client::requests::announce::{Compact, Event, QueryBuilder};
use bittorrent_tracker_client::http::client::responses::announce::{Announce, DeserializedCompact};
use bittorrent_tracker_client::http::client::responses::scrape;
use bittorrent_tracker_client::http::client::{requests, Client};
use bittorrent_udp_tracker_protocol::PeerId;
use clap::{Parser, Subcommand, ValueEnum};
use reqwest::Url;
use torrust_tracker_configuration::DEFAULT_TIMEOUT;

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliEvent {
    Started,
    Stopped,
    Completed,
}

impl From<CliEvent> for Event {
    fn from(value: CliEvent) -> Self {
        match value {
            CliEvent::Started => Event::Started,
            CliEvent::Stopped => Event::Stopped,
            CliEvent::Completed => Event::Completed,
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliCompact {
    #[value(name = "0")]
    NotAccepted,
    #[value(name = "1")]
    Accepted,
}

impl From<CliCompact> for Compact {
    fn from(value: CliCompact) -> Self {
        match value {
            CliCompact::NotAccepted => Compact::NotAccepted,
            CliCompact::Accepted => Compact::Accepted,
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
        tracker_url: String,
        info_hash: String,
        #[arg(long)]
        event: Option<CliEvent>,
        #[arg(long)]
        uploaded: Option<u64>,
        #[arg(long)]
        downloaded: Option<u64>,
        #[arg(long)]
        left: Option<u64>,
        #[arg(long, value_parser = parse_non_zero_port)]
        port: Option<u16>,
        #[arg(long = "peer-addr")]
        peer_addr: Option<IpAddr>,
        #[arg(long = "peer-id", value_parser = parse_peer_id)]
        peer_id: Option<PeerId>,
        #[arg(long, value_enum)]
        compact: Option<CliCompact>,
    },
    Scrape {
        tracker_url: String,
        info_hashes: Vec<String>,
    },
}

struct AnnounceOptions {
    tracker_url: String,
    info_hash: String,
    event: Option<CliEvent>,
    uploaded: Option<u64>,
    downloaded: Option<u64>,
    left: Option<u64>,
    port: Option<u16>,
    peer_addr: Option<IpAddr>,
    peer_id: Option<PeerId>,
    compact: Option<CliCompact>,
}

/// # Errors
///
/// Will return an error if the command fails.
pub async fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Announce {
            tracker_url,
            info_hash,
            event,
            uploaded,
            downloaded,
            left,
            port,
            peer_addr,
            peer_id,
            compact,
        } => {
            announce_command(
                AnnounceOptions {
                    tracker_url,
                    info_hash,
                    event,
                    uploaded,
                    downloaded,
                    left,
                    port,
                    peer_addr,
                    peer_id,
                    compact,
                },
                DEFAULT_TIMEOUT,
            )
            .await?;
        }
        Command::Scrape {
            tracker_url,
            info_hashes,
        } => {
            scrape_command(&tracker_url, &info_hashes, DEFAULT_TIMEOUT).await?;
        }
    }

    Ok(())
}

async fn announce_command(options: AnnounceOptions, timeout: Duration) -> anyhow::Result<()> {
    let base_url = Url::parse(&options.tracker_url).context("failed to parse HTTP tracker base URL")?;
    let info_hash = InfoHash::from_str(&options.info_hash)
        .expect("Invalid infohash. Example infohash: `9c38422213e30bff212b30c360d26f9a02136422`");

    let mut query_builder = QueryBuilder::with_default_values().with_info_hash(&info_hash);

    if let Some(event) = options.event {
        query_builder = query_builder.with_event(event.into());
    }
    if let Some(uploaded) = options.uploaded {
        query_builder = query_builder.with_uploaded(uploaded);
    }
    if let Some(downloaded) = options.downloaded {
        query_builder = query_builder.with_downloaded(downloaded);
    }
    if let Some(left) = options.left {
        query_builder = query_builder.with_left(left);
    }
    if let Some(port) = options.port {
        query_builder = query_builder.with_port(port);
    }
    if let Some(peer_addr) = options.peer_addr {
        query_builder = query_builder.with_peer_addr(&peer_addr);
    }
    if let Some(peer_id) = options.peer_id {
        query_builder = query_builder.with_peer_id(&peer_id);
    }
    if let Some(compact) = options.compact {
        query_builder = query_builder.with_compact(compact.into());
    }

    let response = Client::new(base_url, timeout)?.announce(&query_builder.query()).await?;

    let body = response.bytes().await?;

    let json = if let Ok(announce_response) = serde_bencode::from_bytes::<Announce>(&body) {
        serde_json::to_string(&announce_response).context("failed to serialize announce response into JSON")?
    } else if let Ok(compact_response) = serde_bencode::from_bytes::<DeserializedCompact>(&body) {
        serde_json::to_string(&compact_response).context("failed to serialize compact announce response into JSON")?
    } else {
        panic!("response body should be a valid announce response, got: \"{body:#?}\"")
    };

    println!("{json}");

    Ok(())
}

fn parse_peer_id(peer_id_str: &str) -> anyhow::Result<PeerId> {
    let bytes = peer_id_str.as_bytes();
    if bytes.len() != 20 {
        return Err(anyhow::anyhow!(
            "peer-id must be exactly 20 bytes, got {} bytes for `{peer_id_str}`",
            bytes.len()
        ));
    }

    let mut arr = [0u8; 20];
    arr.copy_from_slice(bytes);

    Ok(PeerId(arr))
}

fn parse_non_zero_port(port_str: &str) -> anyhow::Result<u16> {
    let port = u16::from_str(port_str).with_context(|| format!("invalid port value: `{port_str}`"))?;

    if port == 0 {
        anyhow::bail!("port must be greater than zero")
    }

    Ok(port)
}

async fn scrape_command(tracker_url: &str, info_hashes: &[String], timeout: Duration) -> anyhow::Result<()> {
    let base_url = Url::parse(tracker_url).context("failed to parse HTTP tracker base URL")?;

    let query = requests::scrape::Query::try_from(info_hashes).context("failed to parse infohashes")?;

    let response = Client::new(base_url, timeout)?.scrape(&query).await?;

    let body = response.bytes().await?;

    let scrape_response = scrape::Response::try_from_bencoded(&body)
        .unwrap_or_else(|_| panic!("response body should be a valid scrape response, got: \"{body:#?}\""));

    let json = serde_json::to_string(&scrape_response).context("failed to serialize scrape response into JSON")?;

    println!("{json}");

    Ok(())
}
