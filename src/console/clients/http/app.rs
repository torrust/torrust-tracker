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
use std::str::FromStr;
use std::time::Duration;

use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use reqwest::Url;
use torrust_tracker_primitives::info_hash::InfoHash;

use crate::console::clients::http::{check_http_announce, check_http_scrape};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
    timeout: u64,
}

#[derive(Subcommand, Debug)]
enum Command {
    Announce { tracker_url: String, info_hash: String },
    Scrape { tracker_url: String, info_hashes: Vec<String> },
}

/// # Errors
///
/// Will return an error if the command fails.
pub async fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    let timeout = Duration::from_secs(args.timeout);

    match args.command {
        Command::Announce { tracker_url, info_hash } => {
            announce_command(tracker_url, timeout, info_hash).await?;
        }
        Command::Scrape {
            tracker_url,
            info_hashes,
        } => {
            scrape_command(&tracker_url, timeout, &info_hashes).await?;
        }
    }

    Ok(())
}

async fn announce_command(tracker_url: String, timeout: Duration, info_hash: String) -> anyhow::Result<()> {
    let url = Url::parse(&tracker_url).context("failed to parse HTTP tracker base URL")?;
    let info_hash = InfoHash::from_str(&info_hash).context("Unable to parse info_hash provided as a string")?;

    let response = check_http_announce(&url, timeout, info_hash)
        .await
        .context("it should get a announce response")?;

    let json = serde_json::to_string(&response).context("failed to serialize scrape response into JSON")?;

    println!("{json}");

    Ok(())
}

async fn scrape_command(tracker_url: &str, timeout: Duration, info_hashes: &[String]) -> anyhow::Result<()> {
    let i = info_hashes.iter().map(|s| InfoHash::from_str(s));

    if i.clone().any(|i| i.is_err()) {
        bail!("supplied bad infohash: {:?}", i);
    }

    let url = Url::parse(tracker_url).context("failed to parse HTTP tracker base URL")?;

    let response = check_http_scrape(&url, timeout, &i.flatten().collect::<Vec<_>>())
        .await
        .context("it should get the scrape result")?;

    let json = serde_json::to_string(&response).context("failed to serialize scrape response into JSON")?;

    println!("{json}");

    Ok(())
}
