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
use std::num::NonZeroU16;
use std::time::Duration;

use anyhow::Context;
use aquatic_udp_protocol::{Port, Response, TransactionId};
use clap::{Parser, Subcommand};
use torrust_tracker_primitives::info_hash::InfoHash;
use tracing::Level;

use crate::console::clients::udp::checker;
use crate::console::clients::udp::responses::dto::ResponseDto;
use crate::console::clients::udp::responses::json::ToJson;
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

    let response_dto: ResponseDto = response.into();
    let response_json = response_dto.to_json_string()?;

    print!("{response_json}");

    Ok(())
}

async fn handle_announce(addr: &SocketAddr, timeout: &Duration, info_hash: &InfoHash) -> anyhow::Result<Response> {
    let transaction_id = TransactionId::new(RANDOM_TRANSACTION_ID);

    let client = checker::Client::bind_and_connect(addr, timeout).await?;

    let bound_to = client.client.local_addr()?;

    let ctx = client.send_connection_request(transaction_id).await?;

    let port = NonZeroU16::new(bound_to.port()).expect("it should be non-zero");

    client
        .send_announce_request(&ctx, *info_hash, Port::new(port))
        .await
        .context("failed to handle announce")
}

async fn handle_scrape(addr: &SocketAddr, timeout: &Duration, info_hashes: &[InfoHash]) -> anyhow::Result<Response> {
    let transaction_id = TransactionId::new(RANDOM_TRANSACTION_ID);

    let client = checker::Client::bind_and_connect(addr, timeout).await?;

    let ctx = client.send_connection_request(transaction_id).await?;

    client
        .send_scrape_request(&ctx, info_hashes)
        .await
        .context("failed to handle scrape")
}
