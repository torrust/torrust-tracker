//! The `scrape` service.
//!
//! The service is responsible for handling the `scrape` requests.
//!
//! It delegates the `scrape` logic to the [`ScrapeHandler`] and it returns the
//! [`ScrapeData`].
//!
//! It also sends an [`udp_tracker_core::statistics::event::Event`]
//! because events are specific for the UDP tracker.
use std::net::SocketAddr;
use std::ops::Range;
use std::sync::Arc;

use aquatic_udp_protocol::ScrapeRequest;
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::error::{ScrapeError, WhitelistError};
use bittorrent_tracker_core::scrape_handler::ScrapeHandler;
use torrust_tracker_primitives::core::ScrapeData;

use crate::connection_cookie::{check, gen_remote_fingerprint, ConnectionCookieError};
use crate::statistics;

/// Errors related to scrape requests.
#[derive(thiserror::Error, Debug, Clone)]
pub enum UdpScrapeError {
    /// Error returned when there was an error with the connection cookie.
    #[error("Connection cookie error: {source}")]
    ConnectionCookieError { source: ConnectionCookieError },

    /// Error returned when there was an error with the tracker core scrape handler.
    #[error("Tracker core scrape error: {source}")]
    TrackerCoreScrapeError { source: ScrapeError },

    /// Error returned when there was an error with the tracker core whitelist.
    #[error("Tracker core whitelist error: {source}")]
    TrackerCoreWhitelistError { source: WhitelistError },
}

impl From<ConnectionCookieError> for UdpScrapeError {
    fn from(connection_cookie_error: ConnectionCookieError) -> Self {
        Self::ConnectionCookieError {
            source: connection_cookie_error,
        }
    }
}

impl From<ScrapeError> for UdpScrapeError {
    fn from(scrape_error: ScrapeError) -> Self {
        Self::TrackerCoreScrapeError { source: scrape_error }
    }
}

impl From<WhitelistError> for UdpScrapeError {
    fn from(whitelist_error: WhitelistError) -> Self {
        Self::TrackerCoreWhitelistError { source: whitelist_error }
    }
}

/// It handles the `Scrape` request.
///
/// # Errors
///
/// It will return an error if the tracker core scrape handler returns an error.
pub async fn handle_scrape(
    remote_addr: SocketAddr,
    request: &ScrapeRequest,
    scrape_handler: &Arc<ScrapeHandler>,
    opt_udp_stats_event_sender: &Arc<Option<Box<dyn statistics::event::sender::Sender>>>,
    cookie_valid_range: Range<f64>,
) -> Result<ScrapeData, UdpScrapeError> {
    check(
        &request.connection_id,
        gen_remote_fingerprint(&remote_addr),
        cookie_valid_range,
    )?;

    // Convert from aquatic infohashes
    let info_hashes: Vec<InfoHash> = request.info_hashes.iter().map(|&x| x.into()).collect();

    let scrape_data = scrape_handler.scrape(&info_hashes).await?;

    if let Some(udp_stats_event_sender) = opt_udp_stats_event_sender.as_deref() {
        match remote_addr {
            SocketAddr::V4(_) => {
                udp_stats_event_sender.send_event(statistics::event::Event::Udp4Scrape).await;
            }
            SocketAddr::V6(_) => {
                udp_stats_event_sender.send_event(statistics::event::Event::Udp6Scrape).await;
            }
        }
    }

    Ok(scrape_data)
}
