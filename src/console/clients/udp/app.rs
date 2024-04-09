//! UDP Tracker client:
//!
//! Examples:
//!
//! Announce request:
//!
//! ```text
//! cargo run --bin udp_tracker_client announce 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422 | jq
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
//! You can use an URL with instead of the socket address. For example:
//!
//! ```text
//! cargo run --bin udp_tracker_client scrape udp://localhost:6969 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! cargo run --bin udp_tracker_client scrape udp://localhost:6969/scrape 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! ```
//!
//! The protocol (`udp://`) in the URL is mandatory. The path (`\scrape`) is optional. It always uses `\scrape`.
use std::net::{SocketAddr, ToSocketAddrs};
use std::str::FromStr;

use anyhow::Context;
use aquatic_udp_protocol::Response::{self, AnnounceIpv4, AnnounceIpv6, Scrape};
use aquatic_udp_protocol::{Port, TransactionId};
use clap::{Parser, Subcommand};
use torrust_tracker_primitives::info_hash::InfoHash as TorrustInfoHash;
use tracing::debug;
use url::Url;

use crate::console::clients::udp::checker;
use crate::console::clients::udp::responses::{AnnounceResponseDto, ScrapeResponseDto};

const ASSIGNED_BY_OS: u16 = 0;
const RANDOM_TRANSACTION_ID: i32 = -888_840_697;

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
    setup_tracing(tracing::Level::INFO);

    let args = Args::parse();

    let response = match args.command {
        Command::Announce {
            tracker_socket_addr,
            info_hash,
        } => handle_announce(&tracker_socket_addr, &info_hash).await?,
        Command::Scrape {
            tracker_socket_addr,
            info_hashes,
        } => handle_scrape(&tracker_socket_addr, &info_hashes).await?,
    };

    print_response(response)
}

fn setup_tracing(level: tracing::Level) {
    let () = tracing_subscriber::fmt().pretty().with_max_level(level).init();

    debug!("tracing initialized.");
}

async fn handle_announce(tracker_socket_addr: &SocketAddr, info_hash: &TorrustInfoHash) -> anyhow::Result<Response> {
    let transaction_id = TransactionId(RANDOM_TRANSACTION_ID);

    let mut client = checker::Client::default();

    let bound_to = client.bind_and_connect(ASSIGNED_BY_OS, tracker_socket_addr).await?;

    let connection_id = client.send_connection_request(transaction_id).await?;

    client
        .send_announce_request(connection_id, transaction_id, *info_hash, Port(bound_to.port()))
        .await
}

async fn handle_scrape(tracker_socket_addr: &SocketAddr, info_hashes: &[TorrustInfoHash]) -> anyhow::Result<Response> {
    let transaction_id = TransactionId(RANDOM_TRANSACTION_ID);

    let mut client = checker::Client::default();

    let _bound_to = client.bind_and_connect(ASSIGNED_BY_OS, tracker_socket_addr).await?;

    let connection_id = client.send_connection_request(transaction_id).await?;

    client
        .send_scrape_request(connection_id, transaction_id, info_hashes.to_vec())
        .await
}

fn print_response(response: Response) -> anyhow::Result<()> {
    match response {
        AnnounceIpv4(response) => {
            let pretty_json = serde_json::to_string_pretty(&AnnounceResponseDto::from(response))
                .context("announce IPv4 response JSON serialization")?;
            println!("{pretty_json}");
        }
        AnnounceIpv6(response) => {
            let pretty_json = serde_json::to_string_pretty(&AnnounceResponseDto::from(response))
                .context("announce IPv6 response JSON serialization")?;
            println!("{pretty_json}");
        }
        Scrape(response) => {
            let pretty_json =
                serde_json::to_string_pretty(&ScrapeResponseDto::from(response)).context("scrape response JSON serialization")?;
            println!("{pretty_json}");
        }
        _ => println!("{response:#?}"), // todo: serialize to JSON all aquatic responses.
    };

    Ok(())
}

fn parse_socket_addr(tracker_socket_addr_str: &str) -> anyhow::Result<SocketAddr> {
    debug!("Tracker socket address: {tracker_socket_addr_str:#?}");

    // Check if the address is a valid URL. If so, extract the host and port.
    let resolved_addr = if let Ok(url) = Url::parse(tracker_socket_addr_str) {
        debug!("Tracker socket address URL: {url:?}");

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
                "invalid address format: `{}`. Expected format is host:port",
                tracker_socket_addr_str
            ));
        }

        let host = parts[0].to_owned();

        let port = parts[1]
            .parse::<u16>()
            .with_context(|| format!("invalid port: `{}`", parts[1]))?
            .to_owned();

        (host, port)
    };

    debug!("Resolved address: {resolved_addr:#?}");

    // Perform DNS resolution.
    let socket_addrs: Vec<_> = resolved_addr.to_socket_addrs()?.collect();
    if socket_addrs.is_empty() {
        Err(anyhow::anyhow!("DNS resolution failed for `{}`", tracker_socket_addr_str))
    } else {
        Ok(socket_addrs[0])
    }
}

fn parse_info_hash(info_hash_str: &str) -> anyhow::Result<TorrustInfoHash> {
    TorrustInfoHash::from_str(info_hash_str)
        .map_err(|e| anyhow::Error::msg(format!("failed to parse info-hash `{info_hash_str}`: {e:?}")))
}
