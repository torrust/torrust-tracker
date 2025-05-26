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
use torrust_tracker_primitives::service_binding::ServiceBinding;

use crate::connection_cookie::{check, gen_remote_fingerprint, ConnectionCookieError};
use crate::event::{ConnectionContext, Event};

/// The `ScrapeService` is responsible for handling the `scrape` requests.
///
/// The service sends an statistics event that increments:
///
/// - The number of UDP `scrape` requests handled by the UDP tracker.
pub struct ScrapeService {
    scrape_handler: Arc<ScrapeHandler>,
    opt_udp_stats_event_sender: crate::event::sender::Sender,
}

impl ScrapeService {
    #[must_use]
    pub fn new(scrape_handler: Arc<ScrapeHandler>, opt_udp_stats_event_sender: crate::event::sender::Sender) -> Self {
        Self {
            scrape_handler,
            opt_udp_stats_event_sender,
        }
    }

    /// It handles the `Scrape` request.
    ///
    /// # Errors
    ///
    /// It will return an error if the tracker core scrape handler returns an error.
    pub async fn handle_scrape(
        &self,
        client_socket_addr: SocketAddr,
        server_service_binding: ServiceBinding,
        request: &ScrapeRequest,
        cookie_valid_range: Range<f64>,
    ) -> Result<ScrapeData, UdpScrapeError> {
        Self::authenticate(client_socket_addr, request, cookie_valid_range)?;

        let scrape_data = self
            .scrape_handler
            .handle_scrape(&Self::convert_from_aquatic(&request.info_hashes))
            .await?;

        self.send_event(client_socket_addr, server_service_binding).await;

        Ok(scrape_data)
    }

    fn authenticate(
        remote_addr: SocketAddr,
        request: &ScrapeRequest,
        cookie_valid_range: Range<f64>,
    ) -> Result<f64, ConnectionCookieError> {
        check(
            &request.connection_id,
            gen_remote_fingerprint(&remote_addr),
            cookie_valid_range,
        )
    }

    fn convert_from_aquatic(aquatic_infohashes: &[aquatic_udp_protocol::common::InfoHash]) -> Vec<InfoHash> {
        aquatic_infohashes.iter().map(|&x| x.into()).collect()
    }

    async fn send_event(&self, client_socket_addr: SocketAddr, server_service_binding: ServiceBinding) {
        if let Some(udp_stats_event_sender) = self.opt_udp_stats_event_sender.as_deref() {
            let event = Event::UdpScrape {
                connection: ConnectionContext::new(client_socket_addr, server_service_binding),
            };

            tracing::debug!(target = crate::UDP_TRACKER_LOG_TARGET, "Sending UdpScrape event: {event:?}");

            udp_stats_event_sender.send(event).await;
        }
    }
}

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
