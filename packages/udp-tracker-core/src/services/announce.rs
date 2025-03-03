//! The `announce` service.
//!
//! The service is responsible for handling the `announce` requests.
//!
//! It delegates the `announce` logic to the [`AnnounceHandler`] and it returns
//! the [`AnnounceData`].
//!
//! It also sends an [`udp_tracker_core::statistics::event::Event`]
//! because events are specific for the HTTP tracker.
use std::net::{IpAddr, SocketAddr};
use std::ops::Range;
use std::sync::Arc;

use aquatic_udp_protocol::AnnounceRequest;
use bittorrent_tracker_core::announce_handler::{AnnounceHandler, PeersWanted};
use bittorrent_tracker_core::error::{AnnounceError, WhitelistError};
use bittorrent_tracker_core::whitelist;
use bittorrent_udp_tracker_protocol::peer_builder;
use torrust_tracker_primitives::core::AnnounceData;

use crate::connection_cookie::{check, gen_remote_fingerprint, ConnectionCookieError};
use crate::statistics;

/// The `AnnounceService` is responsible for handling the `announce` requests.
///
/// The service sends an statistics event that increments:
///
/// - The number of UDP `announce` requests handled by the UDP tracker.
pub struct AnnounceService {
    announce_handler: Arc<AnnounceHandler>,
    whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
    opt_udp_core_stats_event_sender: Arc<Option<Box<dyn statistics::event::sender::Sender>>>,
}

impl AnnounceService {
    #[must_use]
    pub fn new(
        announce_handler: Arc<AnnounceHandler>,
        whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
        opt_udp_core_stats_event_sender: Arc<Option<Box<dyn statistics::event::sender::Sender>>>,
    ) -> Self {
        Self {
            announce_handler,
            whitelist_authorization,
            opt_udp_core_stats_event_sender,
        }
    }

    /// It handles the `Announce` request.
    ///
    /// # Errors
    ///
    /// It will return an error if:
    ///
    /// - The tracker is running in listed mode and the torrent is not in the
    ///   whitelist.
    pub async fn handle_announce(
        &self,
        remote_addr: SocketAddr,
        request: &AnnounceRequest,
        cookie_valid_range: Range<f64>,
    ) -> Result<AnnounceData, UdpAnnounceError> {
        // Authentication
        check(
            &request.connection_id,
            gen_remote_fingerprint(&remote_addr),
            cookie_valid_range,
        )?;

        let info_hash = request.info_hash.into();
        let remote_client_ip = remote_addr.ip();

        // Authorization
        self.whitelist_authorization.authorize(&info_hash).await?;

        let mut peer = peer_builder::from_request(request, &remote_client_ip);
        let peers_wanted: PeersWanted = i32::from(request.peers_wanted.0).into();

        let original_peer_ip = peer.peer_addr.ip();

        // The tracker could change the original peer ip
        let announce_data = self
            .announce_handler
            .announce(&info_hash, &mut peer, &original_peer_ip, &peers_wanted)
            .await?;

        if let Some(udp_stats_event_sender) = self.opt_udp_core_stats_event_sender.as_deref() {
            match original_peer_ip {
                IpAddr::V4(_) => {
                    udp_stats_event_sender
                        .send_event(statistics::event::Event::Udp4Announce)
                        .await;
                }
                IpAddr::V6(_) => {
                    udp_stats_event_sender
                        .send_event(statistics::event::Event::Udp6Announce)
                        .await;
                }
            }
        }

        Ok(announce_data)
    }
}

/// Errors related to announce requests.
#[derive(thiserror::Error, Debug, Clone)]
pub enum UdpAnnounceError {
    /// Error returned when there was an error with the connection cookie.
    #[error("Connection cookie error: {source}")]
    ConnectionCookieError { source: ConnectionCookieError },

    /// Error returned when there was an error with the tracker core announce handler.
    #[error("Tracker core announce error: {source}")]
    TrackerCoreAnnounceError { source: AnnounceError },

    /// Error returned when there was an error with the tracker core whitelist.
    #[error("Tracker core whitelist error: {source}")]
    TrackerCoreWhitelistError { source: WhitelistError },
}

impl From<ConnectionCookieError> for UdpAnnounceError {
    fn from(connection_cookie_error: ConnectionCookieError) -> Self {
        Self::ConnectionCookieError {
            source: connection_cookie_error,
        }
    }
}

impl From<AnnounceError> for UdpAnnounceError {
    fn from(announce_error: AnnounceError) -> Self {
        Self::TrackerCoreAnnounceError { source: announce_error }
    }
}

impl From<WhitelistError> for UdpAnnounceError {
    fn from(whitelist_error: WhitelistError) -> Self {
        Self::TrackerCoreWhitelistError { source: whitelist_error }
    }
}
