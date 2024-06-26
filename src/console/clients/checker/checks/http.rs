use std::str::FromStr as _;
use std::time::Duration;

use serde::Serialize;
use torrust_tracker_primitives::info_hash::InfoHash;
use url::Url;

use crate::console::clients;
use crate::console::clients::http::Error;
use crate::shared::bit_torrent::tracker::http::client::responses;

#[derive(Debug, Clone, Serialize)]
pub struct Checks {
    url: Url,
    results: Vec<(Check, Result<(), Error>)>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Check {
    Announce,
    Scrape,
}

pub async fn run(http_trackers: Vec<Url>, timeout: Duration) -> Vec<Result<Checks, Checks>> {
    let mut results = Vec::default();

    tracing::debug!("HTTP trackers ...");

    for ref url in http_trackers {
        let mut checks = Checks {
            url: url.clone(),
            results: Vec::default(),
        };

        // Announce
        {
            let check = check_http_announce(url, &timeout).await.map(|_| ());

            checks.results.push((Check::Announce, check));
        }

        // Scrape
        {
            let check = check_http_scrape(url, &timeout).await.map(|_| ());

            checks.results.push((Check::Scrape, check));
        }

        if checks.results.iter().any(|f| f.1.is_err()) {
            results.push(Err(checks));
        } else {
            results.push(Ok(checks));
        }
    }

    results
}

async fn check_http_announce(url: &Url, timeout: &Duration) -> Result<responses::Announce, Error> {
    let info_hash_str = "9c38422213e30bff212b30c360d26f9a02136422".to_string(); // # DevSkim: ignore DS173237
    let info_hash = InfoHash::from_str(&info_hash_str).expect("a valid info-hash is required");

    clients::http::check_http_announce(url, timeout, &info_hash).await
}

async fn check_http_scrape(url: &Url, timeout: &Duration) -> Result<responses::Scrape, Error> {
    let info_hash_str = "9c38422213e30bff212b30c360d26f9a02136422".to_string(); // # DevSkim: ignore DS173237
    let info_hashes = vec![InfoHash::from_str(&info_hash_str).expect("a valid info-hash is required")]; // # DevSkim: ignore DS173237

    clients::http::check_http_scrape(url, timeout, &info_hashes).await
}
