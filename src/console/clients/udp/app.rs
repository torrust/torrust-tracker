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
use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Context;
use aquatic_udp_protocol::Response::{self, AnnounceIpv4, AnnounceIpv6, Scrape};
use aquatic_udp_protocol::{Port, TransactionId};
use clap::{Parser, Subcommand};
use torrust_tracker_primitives::info_hash::InfoHash;
use tracing::Level;

use crate::console::clients::udp::checker;
use crate::console::clients::udp::responses::{AnnounceResponseDto, ScrapeResponseDto};
use crate::console::clients::{parse_info_hash, parse_socket_addr, DEFAULT_TIMEOUT_SEC};

const RANDOM_TRANSACTION_ID: i32 = -888_840_697;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(value_parser = parse_socket_addr, help = "tracker url")]
    addr: SocketAddr,

    /// Name of the person to greet
    #[arg(long, default_value = DEFAULT_TIMEOUT_SEC, help = "connection timeout in seconds")]
    timeout_sec: u64,
}

#[derive(Subcommand, Debug)]
enum Command {
    Announce {
        #[arg(value_parser = parse_info_hash)]
        info_hash: InfoHash,
    },
    Scrape {
        #[arg(value_parser = parse_info_hash, num_args = 1..=74, value_delimiter = ' ')]
        info_hashes: Vec<InfoHash>,
    },
}

/// # Errors
///
/// Will return an error if the command fails.
///
///
pub async fn run() -> anyhow::Result<()> {
    let () = tracing_subscriber::fmt().compact().with_max_level(Level::TRACE).init();

    let args = Args::parse();

    let timeout = Duration::from_secs(args.timeout_sec);

    let response = match args.command {
        Command::Announce { info_hash } => handle_announce(&args.addr, &timeout, &info_hash).await?,
        Command::Scrape { info_hashes } => handle_scrape(&args.addr, &timeout, &info_hashes).await?,
    };

    print_response(response)
}

async fn handle_announce(addr: &SocketAddr, timeout: &Duration, info_hash: &InfoHash) -> anyhow::Result<Response> {
    let transaction_id = TransactionId(RANDOM_TRANSACTION_ID);

    let client = checker::Client::bind_and_connect(addr, timeout).await?;

    let bound_to = client.client.local_addr()?;

    let ctx = client.send_connection_request(transaction_id).await?;

    client
        .send_announce_request(&ctx, *info_hash, Port(bound_to.port()))
        .await
        .context("failed to handle announce")
}

async fn handle_scrape(addr: &SocketAddr, timeout: &Duration, info_hashes: &[InfoHash]) -> anyhow::Result<Response> {
    let transaction_id = TransactionId(RANDOM_TRANSACTION_ID);

    let client = checker::Client::bind_and_connect(addr, timeout).await?;

    let ctx = client.send_connection_request(transaction_id).await?;

    client
        .send_scrape_request(&ctx, info_hashes)
        .await
        .context("failed to handle scrape")
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
