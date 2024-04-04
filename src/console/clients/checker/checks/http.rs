use std::str::FromStr as _;
use std::time::Duration;

use colored::Colorize;
use torrust_tracker_primitives::info_hash::InfoHash;
use url::Url;

use crate::console::clients;
use crate::console::clients::checker::console::Console;
use crate::console::clients::checker::printer::Printer;
use crate::console::clients::checker::service::{CheckError, CheckResult};
use crate::console::clients::http::Error;
use crate::shared::bit_torrent::tracker::http::client::responses;

pub async fn run(http_trackers: Vec<Url>, timeout: Duration, console: Console) -> Vec<CheckResult> {
    let mut check_results = Vec::default();

    console.println("HTTP trackers ...");

    for ref url in http_trackers {
        let colored_url = url.to_string().yellow();

        check_results.push(match check_http_announce(url, &timeout).await {
            Ok(_) => {
                console.println(&format!("{} - Announce at {} is OK", "✓".green(), colored_url));
                Ok(())
            }
            Err(err) => {
                console.println(&format!("{} - Announce at {} is failing", "✗".red(), colored_url));
                Err(CheckError::HttpCheckError { url: url.clone(), err })
            }
        });

        check_results.push(match check_http_scrape(url, &timeout).await {
            Ok(_) => {
                console.println(&format!("{} - Scrape at {} is OK", "✓".green(), colored_url));
                Ok(())
            }
            Err(err) => {
                console.println(&format!("{} - Scrape at {} is failing", "✗".red(), colored_url));
                Err(CheckError::HttpCheckError { url: url.clone(), err })
            }
        });
    }

    check_results
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
