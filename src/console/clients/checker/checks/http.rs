use std::str::FromStr;

use colored::Colorize;
use reqwest::Url as ServiceUrl;
use torrust_tracker_primitives::info_hash::InfoHash;
use tracing::debug;
use url::Url;

use crate::console::clients::checker::console::Console;
use crate::console::clients::checker::printer::Printer;
use crate::console::clients::checker::service::{CheckError, CheckResult};
use crate::shared::bit_torrent::tracker::http::client::requests::announce::QueryBuilder;
use crate::shared::bit_torrent::tracker::http::client::responses::announce::Announce;
use crate::shared::bit_torrent::tracker::http::client::responses::scrape;
use crate::shared::bit_torrent::tracker::http::client::{requests, Client};

pub async fn run(http_trackers: &Vec<ServiceUrl>, console: &Console, check_results: &mut Vec<CheckResult>) {
    console.println("HTTP trackers ...");

    for http_tracker in http_trackers {
        let colored_tracker_url = http_tracker.to_string().yellow();

        match check_http_announce(http_tracker).await {
            Ok(()) => {
                check_results.push(Ok(()));
                console.println(&format!("{} - Announce at {} is OK", "✓".green(), colored_tracker_url));
            }
            Err(err) => {
                check_results.push(Err(err));
                console.println(&format!("{} - Announce at {} is failing", "✗".red(), colored_tracker_url));
            }
        }

        match check_http_scrape(http_tracker).await {
            Ok(()) => {
                check_results.push(Ok(()));
                console.println(&format!("{} - Scrape at {} is OK", "✓".green(), colored_tracker_url));
            }
            Err(err) => {
                check_results.push(Err(err));
                console.println(&format!("{} - Scrape at {} is failing", "✗".red(), colored_tracker_url));
            }
        }
    }
}

async fn check_http_announce(tracker_url: &Url) -> Result<(), CheckError> {
    let info_hash_str = "9c38422213e30bff212b30c360d26f9a02136422".to_string(); // # DevSkim: ignore DS173237
    let info_hash = InfoHash::from_str(&info_hash_str).expect("a valid info-hash is required");

    // todo: HTTP request could panic.For example, if the server is not accessible.
    // We should change the client to catch that error and return a `CheckError`.
    // Otherwise the checking process will stop. The idea is to process all checks
    // and return a final report.
    let response = Client::new(tracker_url.clone())
        .announce(&QueryBuilder::with_default_values().with_info_hash(&info_hash).query())
        .await;

    if let Ok(body) = response.bytes().await {
        if let Ok(_announce_response) = serde_bencode::from_bytes::<Announce>(&body) {
            Ok(())
        } else {
            debug!("announce body {:#?}", body);
            Err(CheckError::HttpError {
                url: tracker_url.clone(),
            })
        }
    } else {
        Err(CheckError::HttpError {
            url: tracker_url.clone(),
        })
    }
}

async fn check_http_scrape(url: &Url) -> Result<(), CheckError> {
    let info_hashes: Vec<String> = vec!["9c38422213e30bff212b30c360d26f9a02136422".to_string()]; // # DevSkim: ignore DS173237
    let query = requests::scrape::Query::try_from(info_hashes).expect("a valid array of info-hashes is required");

    // todo: HTTP request could panic.For example, if the server is not accessible.
    // We should change the client to catch that error and return a `CheckError`.
    // Otherwise the checking process will stop. The idea is to process all checks
    // and return a final report.
    let response = Client::new(url.clone()).scrape(&query).await;

    if let Ok(body) = response.bytes().await {
        if let Ok(_scrape_response) = scrape::Response::try_from_bencoded(&body) {
            Ok(())
        } else {
            debug!("scrape body {:#?}", body);
            Err(CheckError::HttpError { url: url.clone() })
        }
    } else {
        Err(CheckError::HttpError { url: url.clone() })
    }
}
