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

use clap::{Parser, Subcommand};
use reqwest::Url;
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash;
use torrust_tracker::shared::bit_torrent::tracker::http::client::requests::announce::QueryBuilder;
use torrust_tracker::shared::bit_torrent::tracker::http::client::responses::announce::Announce;
use torrust_tracker::shared::bit_torrent::tracker::http::client::Client;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Announce { tracker_url: String, info_hash: String },
    Scrape { tracker_url: String, info_hashes: Vec<String> },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Command::Announce { tracker_url, info_hash } => {
            announce_command(tracker_url, info_hash).await;
        }
        Command::Scrape {
            tracker_url,
            info_hashes,
        } => {
            scrape_command(&tracker_url, &info_hashes);
        }
    }
}

async fn announce_command(tracker_url: String, info_hash: String) {
    let base_url = Url::parse(&tracker_url).expect("Invalid HTTP tracker base URL");
    let info_hash = InfoHash::from_str(&info_hash).expect("Invalid infohash");

    let response = Client::new(base_url)
        .announce(&QueryBuilder::with_default_values().with_info_hash(&info_hash).query())
        .await;

    let body = response.bytes().await.unwrap();

    let announce_response: Announce = serde_bencode::from_bytes(&body)
        .unwrap_or_else(|_| panic!("response body should be a valid announce response, got \"{:#?}\"", &body));

    let json = serde_json::to_string(&announce_response).expect("announce response should be a valid JSON");

    println!("{json}");
}

fn scrape_command(tracker_url: &str, info_hashes: &[String]) {
    println!("URL: {tracker_url}");
    println!("Infohashes: {info_hashes:#?}");
    todo!();
}
