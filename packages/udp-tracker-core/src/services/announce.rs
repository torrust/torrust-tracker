//! The `announce` service.
//!
//! The service is responsible for handling the `announce` requests.
//!
//! It delegates the `announce` logic to the [`AnnounceHandler`] and it returns
//! the [`AnnounceData`].
//!
//! It also sends an [`udp_tracker_core::statistics::event::Event`]
//! because events are specific for the HTTP tracker.
use std::net::SocketAddr;
use std::ops::Range;
use std::sync::Arc;

use aquatic_udp_protocol::AnnounceRequest;
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::announce_handler::{AnnounceHandler, PeersWanted};
use bittorrent_tracker_core::error::{AnnounceError, WhitelistError};
use bittorrent_tracker_core::whitelist;
use bittorrent_udp_tracker_protocol::peer_builder;
use torrust_tracker_primitives::core::AnnounceData;
use torrust_tracker_primitives::peer::PeerAnnouncement;
use torrust_tracker_primitives::service_binding::ServiceBinding;

use crate::connection_cookie::{check, gen_remote_fingerprint, ConnectionCookieError};
use crate::event::{ConnectionContext, Event};

/// The `AnnounceService` is responsible for handling the `announce` requests.
///
/// The service sends an statistics event that increments:
///
/// - The number of UDP `announce` requests handled by the UDP tracker.
pub struct AnnounceService {
    announce_handler: Arc<AnnounceHandler>,
    whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
    opt_udp_core_stats_event_sender: crate::event::sender::Sender,
}

impl AnnounceService {
    #[must_use]
    pub fn new(
        announce_handler: Arc<AnnounceHandler>,
        whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
        opt_udp_core_stats_event_sender: crate::event::sender::Sender,
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
        client_socket_addr: SocketAddr,
        server_service_binding: ServiceBinding,
        request: &AnnounceRequest,
        cookie_valid_range: Range<f64>,
    ) -> Result<AnnounceData, UdpAnnounceError> {
        Self::authenticate(client_socket_addr, request, cookie_valid_range)?;

        let info_hash = request.info_hash.into();

        self.authorize(&info_hash).await?;

        let remote_client_ip = client_socket_addr.ip();

        let mut peer = peer_builder::from_request(request, &remote_client_ip);

        let peers_wanted: PeersWanted = i32::from(request.peers_wanted.0).into();

        let announce_data = self
            .announce_handler
            .handle_announcement(&info_hash, &mut peer, &remote_client_ip, &peers_wanted)
            .await?;

        self.send_event(info_hash, peer, client_socket_addr, server_service_binding)
            .await;

        Ok(announce_data)
    }

    fn authenticate(
        remote_addr: SocketAddr,
        request: &AnnounceRequest,
        cookie_valid_range: Range<f64>,
    ) -> Result<f64, ConnectionCookieError> {
        check(
            &request.connection_id,
            gen_remote_fingerprint(&remote_addr),
            cookie_valid_range,
        )
    }

    async fn authorize(&self, info_hash: &InfoHash) -> Result<(), WhitelistError> {
        self.whitelist_authorization.authorize(info_hash).await
    }

    async fn send_event(
        &self,
        info_hash: InfoHash,
        announcement: PeerAnnouncement,
        client_socket_addr: SocketAddr,
        server_service_binding: ServiceBinding,
    ) {
        if let Some(udp_stats_event_sender) = self.opt_udp_core_stats_event_sender.as_deref() {
            let event = Event::UdpAnnounce {
                connection: ConnectionContext::new(client_socket_addr, server_service_binding),
                info_hash,
                announcement,
            };

            tracing::debug!(target = crate::UDP_TRACKER_LOG_TARGET, "Sending UdpAnnounce event: {event:?}");

            udp_stats_event_sender.send(event).await;
        }
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
