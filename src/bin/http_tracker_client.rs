use std::env;
use std::str::FromStr;

use reqwest::Url;
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash;
use torrust_tracker::shared::bit_torrent::tracker::http::client::requests::announce::QueryBuilder;
use torrust_tracker::shared::bit_torrent::tracker::http::client::responses::announce::Announce;
use torrust_tracker::shared::bit_torrent::tracker::http::client::Client;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Error: invalid number of arguments!");
        eprintln!("Usage:   cargo run --bin http_tracker_client <HTTP_TRACKER_URL> <INFO_HASH>");
        eprintln!("Example: cargo run --bin http_tracker_client https://tracker.torrust-demo.com 9c38422213e30bff212b30c360d26f9a02136422");
        std::process::exit(1);
    }

    let base_url = Url::parse(&args[1]).expect("arg 1 should be a valid HTTP tracker base URL");
    let info_hash = InfoHash::from_str(&args[2]).expect("arg 2 should be a valid infohash");

    let response = Client::new(base_url)
        .announce(&QueryBuilder::with_default_values().with_info_hash(&info_hash).query())
        .await;

    let body = response.bytes().await.unwrap();

    let announce_response: Announce = serde_bencode::from_bytes(&body)
        .unwrap_or_else(|_| panic!("response body should be a valid announce response, got \"{:#?}\"", &body));

    let json = serde_json::to_string(&announce_response).expect("announce response should be a valid JSON");

    print!("{json}");
}
