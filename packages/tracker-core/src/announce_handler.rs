//! Announce handler.
//!
//! Handling `announce` requests is the most important task for a `BitTorrent`
//! tracker.
//!
//! A `BitTorrent` swarm is a network of peers that are all trying to download
//! the same torrent. When a peer wants to find other peers it announces itself
//! to the swarm via the tracker. The peer sends its data to the tracker so that
//! the tracker can add it to the swarm. The tracker responds to the peer with
//! the list of other peers in the swarm so that the peer can contact them to
//! start downloading pieces of the file from them.
//!
//! Once you have instantiated the `AnnounceHandler` you can `announce` a new [`peer::Peer`](torrust_tracker_primitives) with:
//!
//! ```rust,no_run
//! use std::net::SocketAddr;
//! use std::net::IpAddr;
//! use std::net::Ipv4Addr;
//! use std::str::FromStr;
//!
//! use torrust_tracker_primitives::{AnnounceEvent, NumberOfBytes, PeerId};
//! use torrust_clock::DurationSinceUnixEpoch;
//! use torrust_tracker_primitives::peer;
//! use torrust_info_hash::InfoHash;
//!
//! let info_hash = InfoHash::from_str("3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0").unwrap();
//!
//! let peer = peer::Peer {
//!     peer_id: PeerId(*b"-qB00000000000000001"),
//!     peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8081),
//!     updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
//!     uploaded: NumberOfBytes::new(0),
//!     downloaded: NumberOfBytes::new(0),
//!     left: NumberOfBytes::new(0),
//!     event: AnnounceEvent::Completed,
//! };
//!
//! let peer_ip = IpAddr::V4(Ipv4Addr::from_str("126.0.0.1").unwrap());
//! ```
//!
//! ```text
//! let announce_data = announce_handler.announce(&info_hash, &mut peer, &peer_ip).await;
//! ```
//!
//! The handler returns the list of peers for the torrent with the infohash
//! `3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0`, filtering out the peer that is
//! making the `announce` request.
//!
//! > **NOTICE**: that the peer argument is mutable because the handler can
//! > change the peer IP if the peer is using a loopback IP.
//!
//! The `peer_ip` argument is the resolved peer ip. It's a common practice that
//! trackers ignore the peer ip in the `announce` request params, and resolve
//! the peer ip using the IP of the client making the request. As the tracker is
//! a domain service, the peer IP must be provided for the handler user, which
//! is usually a higher component with access the the request metadata, for
//! example, connection data, proxy headers, etcetera.
//!
//! The returned struct is:
//!
//! ```rust,no_run
//! use torrust_tracker_primitives::peer;
//! use torrust_tracker_primitives::AnnouncePolicy;
//!
//! pub struct AnnounceData {
//!     pub peers: Vec<peer::Peer>,
//!     pub swarm_stats: SwarmMetadata,
//!     pub policy: AnnouncePolicy, // the tracker announce policy.
//! }
//!
//! pub struct SwarmMetadata {
//!     pub completed: u32, // The number of peers that have ever completed downloading
//!     pub seeders: u32,   // The number of active peers that have completed downloading (seeders)
//!     pub leechers: u32,  // The number of active peers that have not completed downloading (leechers)
//! }
//!
//! // Core tracker configuration
//! pub struct AnnounceInterval {
//!     // ...
//!     pub interval: u32, // Interval in seconds that the client should wait between sending regular announce requests to the tracker
//!     pub interval_min: u32, // Minimum announce interval. Clients must not reannounce more frequently than this
//!     // ...
//! }
//! ```
//!
//! ## Related BEPs:
//!
//! Refer to `BitTorrent` BEPs and other sites for more information about the `announce` request:
//!
//! - [BEP 3. The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
//! - [BEP 23. Tracker Returns Compact Peer Lists](https://www.bittorrent.org/beps/bep_0023.html)
//! - [Vuze docs](https://wiki.vuze.com/w/Announce)
use std::net::IpAddr;
use std::sync::Arc;

use torrust_info_hash::InfoHash;
use torrust_tracker_configuration::Core;
use torrust_tracker_primitives::{AnnounceData, NumberOfDownloads, peer};

use super::torrent::repository::in_memory::InMemoryTorrentRepository;
use crate::databases;
use crate::error::AnnounceError;
use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
use crate::whitelist::authorization::WhitelistAuthorization;

/// Handles `announce` requests from `BitTorrent` clients.
pub struct AnnounceHandler {
    /// The tracker configuration.
    config: Core,

    /// Service for authorizing access to whitelisted torrents.
    whitelist_authorization: Arc<WhitelistAuthorization>,

    /// Repository for in-memory torrent data.
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,

    /// Repository for persistent torrent data (database).
    db_downloads_metric_repository: Arc<DatabaseDownloadsMetricRepository>,
}

impl AnnounceHandler {
    /// Creates a new `AnnounceHandler`.
    #[must_use]
    pub fn new(
        config: &Core,
        whitelist_authorization: &Arc<WhitelistAuthorization>,
        in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>,
        db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
    ) -> Self {
        Self {
            whitelist_authorization: whitelist_authorization.clone(),
            config: config.clone(),
            in_memory_torrent_repository: in_memory_torrent_repository.clone(),
            db_downloads_metric_repository: db_downloads_metric_repository.clone(),
        }
    }

    /// Processes an announce request from a peer.
    ///
    /// BEP 03: [The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html).
    ///
    /// # Parameters
    ///
    /// - `info_hash`: The unique identifier of the torrent.
    /// - `peer`: The peer announcing itself (may be updated if IP is adjusted).
    /// - `remote_client_ip`: The IP address of the client making the request.
    /// - `peers_wanted`: Specifies how many peers the client wants in the response.
    ///
    /// # Returns
    ///
    /// An `AnnounceData` struct containing the list of peers, swarm statistics, and tracker policy.
    ///
    /// # Errors
    ///
    /// Returns an error if the tracker is running in `listed` mode and the
    /// torrent is not whitelisted.
    pub async fn handle_announcement(
        &self,
        info_hash: &InfoHash,
        peer: &mut peer::Peer,
        remote_client_ip: &IpAddr,
        peers_wanted: &PeersWanted,
    ) -> Result<AnnounceData, AnnounceError> {
        self.whitelist_authorization.authorize(info_hash).await?;

        peer.change_ip(&assign_ip_address_to_peer(
            remote_client_ip,
            self.config.net.external_ip.map(Into::into),
        ));

        self.in_memory_torrent_repository
            .handle_announcement(info_hash, peer, self.load_downloads_metric_if_needed(info_hash).await?)
            .await;

        Ok(self.build_announce_data(info_hash, peer, peers_wanted).await)
    }

    /// Loads the number of downloads for a torrent if needed.
    async fn load_downloads_metric_if_needed(
        &self,
        info_hash: &InfoHash,
    ) -> Result<Option<NumberOfDownloads>, databases::error::Error> {
        if self.config.tracker_policy.persistent_torrent_completed_stat && !self.in_memory_torrent_repository.contains(info_hash)
        {
            Ok(self.db_downloads_metric_repository.load_torrent_downloads(info_hash).await?)
        } else {
            Ok(None)
        }
    }

    /// Builds the announce data for the peer making the request.
    async fn build_announce_data(&self, info_hash: &InfoHash, peer: &peer::Peer, peers_wanted: &PeersWanted) -> AnnounceData {
        let peers = self
            .in_memory_torrent_repository
            .get_peers_for(
                info_hash,
                peer,
                peers_wanted.limit(self.config.announce_policy.max_peers_per_announce),
            )
            .await;

        let swarm_metadata = self
            .in_memory_torrent_repository
            .get_swarm_metadata_or_default(info_hash)
            .await;

        AnnounceData {
            peers,
            stats: swarm_metadata,
            policy: self.config.announce_policy,
        }
    }
}

/// Specifies how many peers a client wants in the announce response.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum PeersWanted {
    /// Request as many peers as possible (default behavior).
    #[default]
    AsManyAsPossible,

    /// Request a specific number of peers.
    Only { amount: usize },
}

impl PeersWanted {
    /// Request a specific number of peers, without applying the tracker-side cap.
    ///
    /// The cap is applied when [`limit`](PeersWanted::limit) is called.
    #[must_use]
    pub fn only(amount: u32) -> Self {
        PeersWanted::Only { amount: amount as usize }
    }

    /// Constructs a `PeersWanted` from a raw client-supplied value.
    ///
    /// A value of `0` or negative means "as many as possible";
    /// any positive value is stored as-is and capped at the tracker limit
    /// when [`limit`](PeersWanted::limit) is called.
    #[must_use]
    pub fn from_client_request(value: i32) -> Self {
        if value <= 0 {
            PeersWanted::AsManyAsPossible
        } else {
            // Safe: value > 0, so casting to usize is lossless on all supported platforms.
            #[allow(clippy::cast_sign_loss)]
            PeersWanted::Only { amount: value as usize }
        }
    }

    /// Returns the effective number of peers to return, capped at `max_peers`.
    ///
    /// - `AsManyAsPossible` resolves to `max_peers`.
    /// - `Only { amount }` resolves to `amount.min(max_peers)`.
    pub(crate) fn limit(&self, max_peers: usize) -> usize {
        match self {
            PeersWanted::AsManyAsPossible => max_peers,
            PeersWanted::Only { amount } => (*amount).min(max_peers),
        }
    }
}

/// Assigns the correct IP address to a peer based on tracker settings.
///
/// If the client IP is a loopback address and the tracker has an external IP
/// configured, the external IP will be assigned to the peer. Wildcard
/// addresses (`0.0.0.0`, `::`) are rejected at parse/construction time
/// by the `ExternalIp` newtype and should never reach this function.
///
/// If no external IP is configured (`None`), the original remote client IP
/// is returned unchanged, even for loopback addresses.
#[must_use]
fn assign_ip_address_to_peer(remote_client_ip: &IpAddr, tracker_external_ip: Option<IpAddr>) -> IpAddr {
    // Use the external IP only if it is configured with a valid (non-unspecified) address
    // and the client is connecting from a loopback address.
    // Unspecified addresses like 0.0.0.0 or :: are rejected by the ExternalIp newtype
    // at parse time, but we also guard here for defense-in-depth.
    if let Some(host_ip) = tracker_external_ip.filter(|_| remote_client_ip.is_loopback())
        && !host_ip.is_unspecified()
    {
        return host_ip;
    }
    *remote_client_ip
}

#[cfg(test)]
mod tests {
    mod the_announce_handler {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::str::FromStr;
        use std::sync::Arc;

        use torrust_clock::DurationSinceUnixEpoch;
        use torrust_tracker_primitives::peer::Peer;
        use torrust_tracker_primitives::{AnnounceEvent, NumberOfBytes, PeerId};
        use torrust_tracker_test_helpers::configuration;

        use crate::announce_handler::AnnounceHandler;
        use crate::scrape_handler::ScrapeHandler;
        use crate::test_helpers::tests::initialize_handlers;

        async fn public_tracker() -> (Arc<AnnounceHandler>, Arc<ScrapeHandler>) {
            let config = configuration::ephemeral_public();
            initialize_handlers(&config).await
        }

        // The client peer IP
        fn peer_ip() -> IpAddr {
            IpAddr::V4(Ipv4Addr::from_str("126.0.0.1").unwrap())
        }

        /// Sample peer when for tests that need more than one peer
        fn sample_peer_1() -> Peer {
            Peer {
                peer_id: PeerId(*b"-qB00000000000000001"),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8081),
                updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
                uploaded: NumberOfBytes::new(0),
                downloaded: NumberOfBytes::new(0),
                left: NumberOfBytes::new(0),
                event: AnnounceEvent::Completed,
            }
        }

        /// Sample peer when for tests that need more than one peer
        fn sample_peer_2() -> Peer {
            Peer {
                peer_id: PeerId(*b"-qB00000000000000002"),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2)), 8082),
                updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
                uploaded: NumberOfBytes::new(0),
                downloaded: NumberOfBytes::new(0),
                left: NumberOfBytes::new(0),
                event: AnnounceEvent::Completed,
            }
        }

        /// Sample peer when for tests that need more than two peer
        fn sample_peer_3() -> Peer {
            Peer {
                peer_id: PeerId(*b"-qB00000000000000003"),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 3)), 8082),
                updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
                uploaded: NumberOfBytes::new(0),
                downloaded: NumberOfBytes::new(0),
                left: NumberOfBytes::new(0),
                event: AnnounceEvent::Completed,
            }
        }

        mod for_all_tracker_config_modes {

            mod handling_an_announce_request {

                use std::sync::Arc;

                use crate::announce_handler::PeersWanted;
                use crate::announce_handler::tests::the_announce_handler::{
                    peer_ip, public_tracker, sample_peer_1, sample_peer_2, sample_peer_3,
                };
                use crate::test_helpers::tests::{sample_info_hash, sample_peer};

                mod should_assign_the_ip_to_the_peer {

                    use std::net::{IpAddr, Ipv4Addr};

                    use crate::announce_handler::assign_ip_address_to_peer;

                    #[test]
                    fn using_the_source_ip_instead_of_the_ip_in_the_announce_request() {
                        let remote_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2));

                        let peer_ip = assign_ip_address_to_peer(&remote_ip, None);

                        assert_eq!(peer_ip, remote_ip);
                    }

                    mod and_when_the_client_ip_is_a_ipv4_loopback_ip {

                        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
                        use std::str::FromStr;

                        use crate::announce_handler::assign_ip_address_to_peer;

                        #[test]
                        fn it_should_use_the_loopback_ip_if_the_tracker_does_not_have_the_external_ip_configuration() {
                            let remote_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, None);

                            assert_eq!(peer_ip, remote_ip);
                        }

                        #[test]
                        fn it_should_use_the_external_tracker_ip_in_tracker_configuration_if_it_is_defined() {
                            let remote_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);

                            let tracker_external_ip = IpAddr::V4(Ipv4Addr::from_str("126.0.0.1").unwrap());

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, Some(tracker_external_ip));

                            assert_eq!(peer_ip, tracker_external_ip);
                        }

                        #[test]
                        fn it_should_use_the_external_ip_in_the_tracker_configuration_if_it_is_defined_even_if_the_external_ip_is_an_ipv6_ip()
                         {
                            let remote_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);

                            let tracker_external_ip =
                                IpAddr::V6(Ipv6Addr::from_str("2345:0425:2CA1:0000:0000:0567:5673:23b5").unwrap());

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, Some(tracker_external_ip));

                            assert_eq!(peer_ip, tracker_external_ip);
                        }
                    }

                    mod and_when_client_ip_is_a_ipv6_loopback_ip {

                        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
                        use std::str::FromStr;

                        use crate::announce_handler::assign_ip_address_to_peer;

                        #[test]
                        fn it_should_use_the_loopback_ip_if_the_tracker_does_not_have_the_external_ip_configuration() {
                            let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, None);

                            assert_eq!(peer_ip, remote_ip);
                        }

                        #[test]
                        fn it_should_use_the_external_ip_in_tracker_configuration_if_it_is_defined() {
                            let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);

                            let tracker_external_ip =
                                IpAddr::V6(Ipv6Addr::from_str("2345:0425:2CA1:0000:0000:0567:5673:23b5").unwrap());

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, Some(tracker_external_ip));

                            assert_eq!(peer_ip, tracker_external_ip);
                        }

                        #[test]
                        fn it_should_use_the_external_ip_in_the_tracker_configuration_if_it_is_defined_even_if_the_external_ip_is_an_ipv4_ip()
                         {
                            let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);

                            let tracker_external_ip = IpAddr::V4(Ipv4Addr::from_str("126.0.0.1").unwrap());

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, Some(tracker_external_ip));

                            assert_eq!(peer_ip, tracker_external_ip);
                        }
                    }

                    mod and_when_the_external_ip_is_unspecified {
                        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

                        use crate::announce_handler::assign_ip_address_to_peer;

                        #[test]
                        fn it_should_keep_the_ipv4_loopback_ip_when_the_external_ip_is_the_unspecified_ipv4_address() {
                            let remote_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, Some(IpAddr::V4(Ipv4Addr::UNSPECIFIED)));

                            assert_eq!(peer_ip, remote_ip);
                            assert_eq!(peer_ip, IpAddr::V4(Ipv4Addr::LOCALHOST));
                        }

                        #[test]
                        fn it_should_keep_the_ipv6_loopback_ip_when_the_external_ip_is_the_unspecified_ipv6_address() {
                            let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, Some(IpAddr::V6(Ipv6Addr::UNSPECIFIED)));

                            assert_eq!(peer_ip, remote_ip);
                            assert_eq!(peer_ip, IpAddr::V6(Ipv6Addr::LOCALHOST));
                        }

                        #[test]
                        fn it_should_keep_the_ipv4_loopback_ip_when_the_external_ip_is_the_unspecified_ipv6_address() {
                            let remote_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, Some(IpAddr::V6(Ipv6Addr::UNSPECIFIED)));

                            assert_eq!(peer_ip, remote_ip);
                            assert_eq!(peer_ip, IpAddr::V4(Ipv4Addr::LOCALHOST));
                        }

                        #[test]
                        fn it_should_keep_the_ipv6_loopback_ip_when_the_external_ip_is_the_unspecified_ipv4_address() {
                            let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, Some(IpAddr::V4(Ipv4Addr::UNSPECIFIED)));

                            assert_eq!(peer_ip, remote_ip);
                            assert_eq!(peer_ip, IpAddr::V6(Ipv6Addr::LOCALHOST));
                        }

                        #[test]
                        fn it_should_keep_the_non_loopback_ip_when_the_external_ip_is_unspecified_ipv4() {
                            let remote_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2));

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, Some(IpAddr::V4(Ipv4Addr::UNSPECIFIED)));

                            assert_eq!(peer_ip, remote_ip);
                        }

                        #[test]
                        fn it_should_keep_the_non_loopback_ip_when_the_external_ip_is_unspecified_ipv6() {
                            let remote_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2));

                            let peer_ip = assign_ip_address_to_peer(&remote_ip, Some(IpAddr::V6(Ipv6Addr::UNSPECIFIED)));

                            assert_eq!(peer_ip, remote_ip);
                        }
                    }
                }

                #[tokio::test]
                async fn it_should_return_the_announce_data_with_an_empty_peer_list_when_it_is_the_first_announced_peer() {
                    let (announce_handler, _scrape_handler) = public_tracker().await;

                    let mut peer = sample_peer();

                    let announce_data = announce_handler
                        .handle_announcement(&sample_info_hash(), &mut peer, &peer_ip(), &PeersWanted::AsManyAsPossible)
                        .await
                        .unwrap();

                    assert_eq!(announce_data.peers, vec![]);
                }

                #[tokio::test]
                async fn it_should_return_the_announce_data_with_the_previously_announced_peers() {
                    let (announce_handler, _scrape_handler) = public_tracker().await;

                    let mut previously_announced_peer = sample_peer_1();
                    announce_handler
                        .handle_announcement(
                            &sample_info_hash(),
                            &mut previously_announced_peer,
                            &peer_ip(),
                            &PeersWanted::AsManyAsPossible,
                        )
                        .await
                        .unwrap();

                    let mut peer = sample_peer_2();
                    let announce_data = announce_handler
                        .handle_announcement(&sample_info_hash(), &mut peer, &peer_ip(), &PeersWanted::AsManyAsPossible)
                        .await
                        .unwrap();

                    assert_eq!(announce_data.peers, vec![Arc::new(previously_announced_peer)]);
                }

                #[tokio::test]
                async fn it_should_allow_peers_to_get_only_a_subset_of_the_peers_in_the_swarm() {
                    let (announce_handler, _scrape_handler) = public_tracker().await;

                    let mut previously_announced_peer_1 = sample_peer_1();
                    announce_handler
                        .handle_announcement(
                            &sample_info_hash(),
                            &mut previously_announced_peer_1,
                            &peer_ip(),
                            &PeersWanted::AsManyAsPossible,
                        )
                        .await
                        .unwrap();

                    let mut previously_announced_peer_2 = sample_peer_2();
                    announce_handler
                        .handle_announcement(
                            &sample_info_hash(),
                            &mut previously_announced_peer_2,
                            &peer_ip(),
                            &PeersWanted::AsManyAsPossible,
                        )
                        .await
                        .unwrap();

                    let mut peer = sample_peer_3();
                    let announce_data = announce_handler
                        .handle_announcement(&sample_info_hash(), &mut peer, &peer_ip(), &PeersWanted::only(1))
                        .await
                        .unwrap();

                    // It should return only one peer. There is no guarantee on
                    // which peer will be returned.
                    assert!(
                        announce_data.peers == vec![Arc::new(previously_announced_peer_1)]
                            || announce_data.peers == vec![Arc::new(previously_announced_peer_2)]
                    );
                }

                mod it_should_update_the_swarm_stats_for_the_torrent {

                    use crate::announce_handler::PeersWanted;
                    use crate::announce_handler::tests::the_announce_handler::{peer_ip, public_tracker};
                    use crate::test_helpers::tests::{completed_peer, leecher, sample_info_hash, seeder, started_peer};

                    #[tokio::test]
                    async fn when_the_peer_is_a_seeder() {
                        let (announce_handler, _scrape_handler) = public_tracker().await;

                        let mut peer = seeder();

                        let announce_data = announce_handler
                            .handle_announcement(&sample_info_hash(), &mut peer, &peer_ip(), &PeersWanted::AsManyAsPossible)
                            .await
                            .unwrap();

                        assert_eq!(announce_data.stats.complete, 1);
                    }

                    #[tokio::test]
                    async fn when_the_peer_is_a_leecher() {
                        let (announce_handler, _scrape_handler) = public_tracker().await;

                        let mut peer = leecher();

                        let announce_data = announce_handler
                            .handle_announcement(&sample_info_hash(), &mut peer, &peer_ip(), &PeersWanted::AsManyAsPossible)
                            .await
                            .unwrap();

                        assert_eq!(announce_data.stats.incomplete, 1);
                    }

                    #[tokio::test]
                    async fn when_a_previously_announced_started_peer_has_completed_downloading() {
                        let (announce_handler, _scrape_handler) = public_tracker().await;

                        // We have to announce with "started" event because peer does not count if peer was not previously known
                        let mut started_peer = started_peer();
                        announce_handler
                            .handle_announcement(
                                &sample_info_hash(),
                                &mut started_peer,
                                &peer_ip(),
                                &PeersWanted::AsManyAsPossible,
                            )
                            .await
                            .unwrap();

                        let mut completed_peer = completed_peer();
                        let announce_data = announce_handler
                            .handle_announcement(
                                &sample_info_hash(),
                                &mut completed_peer,
                                &peer_ip(),
                                &PeersWanted::AsManyAsPossible,
                            )
                            .await
                            .unwrap();

                        assert_eq!(announce_data.stats.downloaded, 1);
                    }
                }
            }
        }

        mod should_allow_the_client_peers_to_specified_the_number_of_peers_wanted {

            use crate::announce_handler::PeersWanted;

            const MAX_PEERS: usize = 74;

            #[test]
            fn it_should_return_the_maximin_number_of_peers_by_default() {
                let peers_wanted = PeersWanted::default();

                assert_eq!(peers_wanted.limit(MAX_PEERS), MAX_PEERS);
            }

            #[test]
            fn it_should_return_74_at_the_most_if_the_client_wants_them_all() {
                let peers_wanted = PeersWanted::AsManyAsPossible;

                assert_eq!(peers_wanted.limit(MAX_PEERS), MAX_PEERS);
            }

            #[test]
            fn it_should_allow_limiting_the_peer_list() {
                let peers_wanted = PeersWanted::only(10);

                assert_eq!(peers_wanted.limit(MAX_PEERS), 10);
            }

            fn maximum_as_u32() -> u32 {
                u32::try_from(MAX_PEERS).unwrap()
            }

            fn maximum_as_i32() -> i32 {
                i32::try_from(MAX_PEERS).unwrap()
            }

            #[test]
            fn it_should_return_the_maximum_when_wanting_more_than_the_maximum() {
                let peers_wanted = PeersWanted::only(maximum_as_u32() + 1);
                assert_eq!(peers_wanted.limit(MAX_PEERS), MAX_PEERS);
            }

            #[test]
            fn it_should_return_zero_when_wanting_only_zero() {
                let peers_wanted = PeersWanted::only(0);
                assert_eq!(peers_wanted.limit(MAX_PEERS), 0);
            }

            #[test]
            fn it_should_convert_the_peers_wanted_number_from_i32_via_from_client_request() {
                // Negative. It should return the maximum (AsManyAsPossible)
                let peers_wanted = PeersWanted::from_client_request(-1i32);
                assert_eq!(peers_wanted.limit(MAX_PEERS), MAX_PEERS);

                // Zero. It should return the maximum (AsManyAsPossible)
                let peers_wanted = PeersWanted::from_client_request(0i32);
                assert_eq!(peers_wanted.limit(MAX_PEERS), MAX_PEERS);

                // Greater than the maximum. It should be capped at limit time
                let peers_wanted = PeersWanted::from_client_request(maximum_as_i32() + 1);
                assert_eq!(peers_wanted.limit(MAX_PEERS), MAX_PEERS);

                // The maximum
                let peers_wanted = PeersWanted::from_client_request(maximum_as_i32());
                assert_eq!(peers_wanted.limit(MAX_PEERS), MAX_PEERS);

                // Smaller than the maximum
                let peers_wanted = PeersWanted::from_client_request(maximum_as_i32() - 1);
                assert_eq!(i32::try_from(peers_wanted.limit(MAX_PEERS)).unwrap(), maximum_as_i32() - 1);
            }

            #[test]
            fn it_should_cap_only_peers_wanted_at_limit_time() {
                // Zero — returns 0 (explicit request for zero peers)
                let peers_wanted = PeersWanted::only(0u32);
                assert_eq!(peers_wanted.limit(MAX_PEERS), 0);

                // Greater than the maximum — capped at limit time
                let peers_wanted = PeersWanted::only(maximum_as_u32() + 1);
                assert_eq!(peers_wanted.limit(MAX_PEERS), MAX_PEERS);

                // The maximum
                let peers_wanted = PeersWanted::only(maximum_as_u32());
                assert_eq!(peers_wanted.limit(MAX_PEERS), MAX_PEERS);

                // Smaller than the maximum
                let peers_wanted = PeersWanted::only(maximum_as_u32() - 1);
                assert_eq!(i32::try_from(peers_wanted.limit(MAX_PEERS)).unwrap(), maximum_as_i32() - 1);
            }
        }
    }
}
