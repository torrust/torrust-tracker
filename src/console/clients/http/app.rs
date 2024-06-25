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
//! `Scrape` request:
//!
//! ```text
//! cargo run --bin http_tracker_client scrape http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! ```
use std::time::Duration;

use anyhow::Context;
use clap::{Parser, Subcommand};
use reqwest::Url;
use torrust_tracker_primitives::info_hash::InfoHash;
use tracing::Level;

use crate::console::clients::http::{check_http_announce, check_http_scrape};
use crate::console::clients::{parse_info_hash, parse_url, DEFAULT_TIMEOUT_SEC};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(value_parser = parse_url, help = "tracker url")]
    addr: Url,

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
pub async fn run() -> anyhow::Result<()> {
    let () = tracing_subscriber::fmt().compact().with_max_level(Level::TRACE).init();

    let args = Args::parse();

    let timeout = Duration::from_secs(args.timeout_sec);

    match args.command {
        Command::Announce { info_hash } => {
            announce_command(args.addr, &timeout, &info_hash).await?;
        }
        Command::Scrape { info_hashes } => {
            scrape_command(&args.addr, &timeout, &info_hashes).await?;
        }
    }

    Ok(())
}

async fn announce_command(addr: Url, timeout: &Duration, info_hash: &InfoHash) -> anyhow::Result<()> {
    let response = check_http_announce(&addr, timeout, info_hash)
        .await
        .context("it should get a announce response")?;

    let json = serde_json::to_string(&response).context("failed to serialize scrape response into JSON")?;

    println!("{json}");

    Ok(())
}

async fn scrape_command(addr: &Url, timeout: &Duration, info_hashes: &[InfoHash]) -> anyhow::Result<()> {
    let response = check_http_scrape(addr, timeout, info_hashes)
        .await
        .context("it should get the scrape result")?;

    let json = serde_json::to_string(&response).context("failed to serialize scrape response into JSON")?;

    println!("{json}");

    Ok(())
}
