pub mod auth;
pub mod error;
pub mod mode;
pub mod peer;
pub mod services;
pub mod statistics;
pub mod torrent;

use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, HashMap};
use std::net::IpAddr;
use std::panic::Location;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc::error::SendError;
use tokio::sync::{RwLock, RwLockReadGuard};

use self::auth::KeyId;
use self::error::Error;
use self::peer::Peer;
use self::torrent::{SwamStats, SwarmMetadata};
use crate::config::Configuration;
use crate::databases::driver::Driver;
use crate::databases::{self, Database};
use crate::protocol::info_hash::InfoHash;

pub struct Tracker {
    pub config: Arc<Configuration>,
    mode: mode::Mode,
    keys: RwLock<std::collections::HashMap<KeyId, auth::ExpiringKey>>,
    whitelist: RwLock<std::collections::HashSet<InfoHash>>,
    torrents: RwLock<std::collections::BTreeMap<InfoHash, torrent::Entry>>,
    stats_event_sender: Option<Box<dyn statistics::EventSender>>,
    stats_repository: statistics::Repo,
    pub database: Box<dyn Database>,
}

#[derive(Debug, PartialEq, Default)]
pub struct TorrentsMetrics {
    pub seeders: u64,
    pub completed: u64,
    pub leechers: u64,
    pub torrents: u64,
}

pub struct AnnounceData {
    pub peers: Vec<Peer>,
    pub swam_stats: SwamStats,
    pub interval: u32,
    pub interval_min: u32,
}

#[derive(Debug, PartialEq, Default)]
pub struct ScrapeData {
    pub files: HashMap<InfoHash, SwarmMetadata>,
}

impl ScrapeData {
    #[must_use]
    pub fn empty() -> Self {
        let files: HashMap<InfoHash, SwarmMetadata> = HashMap::new();
        Self { files }
    }

    pub fn add_file(&mut self, info_hash: &InfoHash, swarm_metadata: SwarmMetadata) {
        self.files.insert(*info_hash, swarm_metadata);
    }

    pub fn add_file_with_no_metadata(&mut self, info_hash: &InfoHash) {
        self.files.insert(*info_hash, SwarmMetadata::default());
    }
}

impl Tracker {
    /// # Errors
    ///
    /// Will return a `databases::error::Error` if unable to connect to database.
    pub fn new(
        config: &Arc<Configuration>,
        stats_event_sender: Option<Box<dyn statistics::EventSender>>,
        stats_repository: statistics::Repo,
    ) -> Result<Tracker, databases::error::Error> {
        let database = Driver::build(&config.db_driver, &config.db_path)?;

        Ok(Tracker {
            config: config.clone(),
            mode: config.mode,
            keys: RwLock::new(std::collections::HashMap::new()),
            whitelist: RwLock::new(std::collections::HashSet::new()),
            torrents: RwLock::new(std::collections::BTreeMap::new()),
            stats_event_sender,
            stats_repository,
            database,
        })
    }

    pub fn is_public(&self) -> bool {
        self.mode == mode::Mode::Public
    }

    pub fn is_private(&self) -> bool {
        self.mode == mode::Mode::Private || self.mode == mode::Mode::PrivateListed
    }

    pub fn is_whitelisted(&self) -> bool {
        self.mode == mode::Mode::Listed || self.mode == mode::Mode::PrivateListed
    }

    /// It handles an announce request.
    ///
    /// BEP 03: [The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html).
    pub async fn announce(&self, info_hash: &InfoHash, peer: &mut Peer, remote_client_ip: &IpAddr) -> AnnounceData {
        // code-review: maybe instead of mutating the peer we could just return
        // a tuple with the new peer and the announce data: (Peer, AnnounceData).
        // It could even be a different struct: `StoredPeer` or `PublicPeer`.

        peer.change_ip(&assign_ip_address_to_peer(remote_client_ip, self.config.get_ext_ip()));

        let swam_stats = self.update_torrent_with_peer_and_get_stats(info_hash, peer).await;

        let peers = self.get_peers_for_peer(info_hash, peer).await;

        AnnounceData {
            peers,
            swam_stats,
            interval: self.config.announce_interval,
            interval_min: self.config.min_announce_interval,
        }
    }

    /// It handles a scrape request.
    ///
    /// BEP 48: [Tracker Protocol Extension: Scrape](https://www.bittorrent.org/beps/bep_0048.html).
    pub async fn scrape(&self, info_hashes: &Vec<InfoHash>) -> ScrapeData {
        let mut scrape_data = ScrapeData::empty();

        for info_hash in info_hashes {
            scrape_data.add_file(info_hash, self.get_swarm_metadata(info_hash).await);
        }

        scrape_data
    }

    // It return empty swarm metadata for all the infohashes.
    pub fn empty_scrape_for(&self, info_hashes: &Vec<InfoHash>) -> ScrapeData {
        let mut scrape_data = ScrapeData::empty();

        for info_hash in info_hashes {
            scrape_data.add_file(info_hash, SwarmMetadata::default());
        }

        scrape_data
    }

    async fn get_swarm_metadata(&self, info_hash: &InfoHash) -> SwarmMetadata {
        let torrents = self.get_torrents().await;
        match torrents.get(info_hash) {
            Some(torrent_entry) => torrent_entry.get_swarm_metadata(),
            None => SwarmMetadata::default(),
        }
    }

    /// # Errors
    ///
    /// Will return a `database::Error` if unable to add the `auth_key` to the database.
    pub async fn generate_auth_key(&self, lifetime: Duration) -> Result<auth::ExpiringKey, databases::error::Error> {
        let auth_key = auth::generate(lifetime);
        self.database.add_key_to_keys(&auth_key).await?;
        self.keys.write().await.insert(auth_key.id.clone(), auth_key.clone());
        Ok(auth_key)
    }

    /// # Errors
    ///
    /// Will return a `database::Error` if unable to remove the `key` to the database.
    ///
    /// # Panics
    ///
    /// Will panic if key cannot be converted into a valid `KeyId`.
    pub async fn remove_auth_key(&self, key: &str) -> Result<(), databases::error::Error> {
        self.database.remove_key_from_keys(key).await?;
        self.keys.write().await.remove(&key.parse::<KeyId>().unwrap());
        Ok(())
    }

    /// # Errors
    ///
    /// Will return a `key::Error` if unable to get any `auth_key`.
    pub async fn verify_auth_key(&self, key_id: &KeyId) -> Result<(), auth::Error> {
        match self.keys.read().await.get(key_id) {
            None => Err(auth::Error::UnableToReadKey {
                location: Location::caller(),
                key_id: Box::new(key_id.clone()),
            }),
            Some(key) => auth::verify(key),
        }
    }

    /// # Errors
    ///
    /// Will return a `database::Error` if unable to `load_keys` from the database.
    pub async fn load_keys(&self) -> Result<(), databases::error::Error> {
        let keys_from_database = self.database.load_keys().await?;
        let mut keys = self.keys.write().await;

        keys.clear();

        for key in keys_from_database {
            keys.insert(key.id.clone(), key);
        }

        Ok(())
    }

    /// Adding torrents is not relevant to public trackers.
    ///
    /// # Errors
    ///
    /// Will return a `database::Error` if unable to add the `info_hash` into the whitelist database.
    pub async fn add_torrent_to_whitelist(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        self.add_torrent_to_database_whitelist(info_hash).await?;
        self.add_torrent_to_memory_whitelist(info_hash).await;
        Ok(())
    }

    /// It adds a torrent to the whitelist if it has not been whitelisted previously
    async fn add_torrent_to_database_whitelist(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        let is_whitelisted = self.database.is_info_hash_whitelisted(info_hash).await?;

        if is_whitelisted {
            return Ok(());
        }

        self.database.add_info_hash_to_whitelist(*info_hash).await?;

        Ok(())
    }

    pub async fn add_torrent_to_memory_whitelist(&self, info_hash: &InfoHash) -> bool {
        self.whitelist.write().await.insert(*info_hash)
    }

    /// Removing torrents is not relevant to public trackers.
    ///
    /// # Errors
    ///
    /// Will return a `database::Error` if unable to remove the `info_hash` from the whitelist database.
    pub async fn remove_torrent_from_whitelist(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        self.remove_torrent_from_database_whitelist(info_hash).await?;
        self.remove_torrent_from_memory_whitelist(info_hash).await;
        Ok(())
    }

    /// # Errors
    ///
    /// Will return a `database::Error` if unable to remove the `info_hash` from the whitelist database.
    pub async fn remove_torrent_from_database_whitelist(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        let is_whitelisted = self.database.is_info_hash_whitelisted(info_hash).await?;

        if !is_whitelisted {
            return Ok(());
        }

        self.database.remove_info_hash_from_whitelist(*info_hash).await?;

        Ok(())
    }

    pub async fn remove_torrent_from_memory_whitelist(&self, info_hash: &InfoHash) -> bool {
        self.whitelist.write().await.remove(info_hash)
    }

    pub async fn is_info_hash_whitelisted(&self, info_hash: &InfoHash) -> bool {
        self.whitelist.read().await.contains(info_hash)
    }

    /// # Errors
    ///
    /// Will return a `database::Error` if unable to load the list whitelisted `info_hash`s from the database.
    pub async fn load_whitelist(&self) -> Result<(), databases::error::Error> {
        let whitelisted_torrents_from_database = self.database.load_whitelist().await?;
        let mut whitelist = self.whitelist.write().await;

        whitelist.clear();

        for info_hash in whitelisted_torrents_from_database {
            let _ = whitelist.insert(info_hash);
        }

        Ok(())
    }

    /// # Errors
    ///
    /// Will return a `torrent::Error::PeerKeyNotValid` if the `key` is not valid.
    ///
    /// Will return a `torrent::Error::PeerNotAuthenticated` if the `key` is `None`.
    ///
    /// Will return a `torrent::Error::TorrentNotWhitelisted` if the the Tracker is in listed mode and the `info_hash` is not whitelisted.
    pub async fn authenticate_request(&self, info_hash: &InfoHash, key: &Option<KeyId>) -> Result<(), Error> {
        // no authentication needed in public mode
        if self.is_public() {
            return Ok(());
        }

        // check if auth_key is set and valid
        if self.is_private() {
            match key {
                Some(key) => {
                    if let Err(e) = self.verify_auth_key(key).await {
                        return Err(Error::PeerKeyNotValid {
                            key_id: key.clone(),
                            source: (Arc::new(e) as Arc<dyn std::error::Error + Send + Sync>).into(),
                        });
                    }
                }
                None => {
                    return Err(Error::PeerNotAuthenticated {
                        location: Location::caller(),
                    });
                }
            }
        }

        // check if info_hash is whitelisted
        if self.is_whitelisted() && !self.is_info_hash_whitelisted(info_hash).await {
            return Err(Error::TorrentNotWhitelisted {
                info_hash: *info_hash,
                location: Location::caller(),
            });
        }

        Ok(())
    }

    /// Loading the torrents from database into memory
    ///
    /// # Errors
    ///
    /// Will return a `database::Error` if unable to load the list of `persistent_torrents` from the database.
    pub async fn load_persistent_torrents(&self) -> Result<(), databases::error::Error> {
        let persistent_torrents = self.database.load_persistent_torrents().await?;
        let mut torrents = self.torrents.write().await;

        for (info_hash, completed) in persistent_torrents {
            // Skip if torrent entry already exists
            if torrents.contains_key(&info_hash) {
                continue;
            }

            let torrent_entry = torrent::Entry {
                peers: BTreeMap::default(),
                completed,
            };

            torrents.insert(info_hash, torrent_entry);
        }

        Ok(())
    }

    async fn get_peers_for_peer(&self, info_hash: &InfoHash, peer: &Peer) -> Vec<peer::Peer> {
        let read_lock = self.torrents.read().await;

        match read_lock.get(info_hash) {
            None => vec![],
            Some(entry) => entry.get_peers_for_peer(peer).into_iter().copied().collect(),
        }
    }

    /// Get all torrent peers for a given torrent
    pub async fn get_all_torrent_peers(&self, info_hash: &InfoHash) -> Vec<peer::Peer> {
        let read_lock = self.torrents.read().await;

        match read_lock.get(info_hash) {
            None => vec![],
            Some(entry) => entry.get_all_peers().into_iter().copied().collect(),
        }
    }

    pub async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> torrent::SwamStats {
        // code-review: consider splitting the function in two (command and query segregation).
        // `update_torrent_with_peer` and `get_stats`

        let mut torrents = self.torrents.write().await;

        let torrent_entry = match torrents.entry(*info_hash) {
            Entry::Vacant(vacant) => vacant.insert(torrent::Entry::new()),
            Entry::Occupied(entry) => entry.into_mut(),
        };

        let stats_updated = torrent_entry.update_peer(peer);

        // todo: move this action to a separate worker
        if self.config.persistent_torrent_completed_stat && stats_updated {
            let _ = self
                .database
                .save_persistent_torrent(info_hash, torrent_entry.completed)
                .await;
        }

        let (seeders, completed, leechers) = torrent_entry.get_stats();

        torrent::SwamStats {
            completed,
            seeders,
            leechers,
        }
    }

    pub async fn get_torrents(&self) -> RwLockReadGuard<'_, BTreeMap<InfoHash, torrent::Entry>> {
        self.torrents.read().await
    }

    pub async fn get_torrents_metrics(&self) -> TorrentsMetrics {
        let mut torrents_metrics = TorrentsMetrics {
            seeders: 0,
            completed: 0,
            leechers: 0,
            torrents: 0,
        };

        let db = self.get_torrents().await;

        db.values().for_each(|torrent_entry| {
            let (seeders, completed, leechers) = torrent_entry.get_stats();
            torrents_metrics.seeders += u64::from(seeders);
            torrents_metrics.completed += u64::from(completed);
            torrents_metrics.leechers += u64::from(leechers);
            torrents_metrics.torrents += 1;
        });

        torrents_metrics
    }

    pub async fn get_stats(&self) -> RwLockReadGuard<'_, statistics::Metrics> {
        self.stats_repository.get_stats().await
    }

    pub async fn send_stats_event(&self, event: statistics::Event) -> Option<Result<(), SendError<statistics::Event>>> {
        match &self.stats_event_sender {
            None => None,
            Some(stats_event_sender) => stats_event_sender.send_event(event).await,
        }
    }

    // Remove inactive peers and (optionally) peerless torrents
    pub async fn cleanup_torrents(&self) {
        let mut torrents_lock = self.torrents.write().await;

        // If we don't need to remove torrents we will use the faster iter
        if self.config.remove_peerless_torrents {
            torrents_lock.retain(|_, torrent_entry| {
                torrent_entry.remove_inactive_peers(self.config.max_peer_timeout);

                if self.config.persistent_torrent_completed_stat {
                    torrent_entry.completed > 0 || !torrent_entry.peers.is_empty()
                } else {
                    !torrent_entry.peers.is_empty()
                }
            });
        } else {
            for (_, torrent_entry) in torrents_lock.iter_mut() {
                torrent_entry.remove_inactive_peers(self.config.max_peer_timeout);
            }
        }
    }
}

#[must_use]
fn assign_ip_address_to_peer(remote_client_ip: &IpAddr, tracker_external_ip: Option<IpAddr>) -> IpAddr {
    if let Some(host_ip) = tracker_external_ip.filter(|_| remote_client_ip.is_loopback()) {
        host_ip
    } else {
        *remote_client_ip
    }
}

#[cfg(test)]
mod tests {

    mod the_tracker {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::sync::Arc;

        use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};

        use crate::config::{ephemeral_configuration, Configuration};
        use crate::protocol::clock::DurationSinceUnixEpoch;
        use crate::tracker::peer::{self, Peer};
        use crate::tracker::statistics::Keeper;
        use crate::tracker::{TorrentsMetrics, Tracker};

        pub fn tracker_configuration() -> Arc<Configuration> {
            Arc::new(ephemeral_configuration())
        }

        pub fn tracker_factory() -> Tracker {
            // code-review: the tracker initialization is duplicated in many places. Consider make this function public.

            // Configuration
            let configuration = tracker_configuration();

            // Initialize stats tracker
            let (stats_event_sender, stats_repository) = Keeper::new_active_instance();

            // Initialize Torrust tracker
            match Tracker::new(&configuration, Some(stats_event_sender), stats_repository) {
                Ok(tracker) => tracker,
                Err(error) => {
                    panic!("{}", error)
                }
            }
        }

        /// A peer that has completed downloading.
        fn complete_peer() -> Peer {
            Peer {
                peer_id: peer::Id(*b"-qB00000000000000000"),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
                updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
                uploaded: NumberOfBytes(0),
                downloaded: NumberOfBytes(0),
                left: NumberOfBytes(0), // No bytes left to download
                event: AnnounceEvent::Completed,
            }
        }

        /// A peer that has NOT completed downloading.
        fn incomplete_peer() -> Peer {
            Peer {
                peer_id: peer::Id(*b"-qB00000000000000000"),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
                updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
                uploaded: NumberOfBytes(0),
                downloaded: NumberOfBytes(0),
                left: NumberOfBytes(1000), // Still bytes to download
                event: AnnounceEvent::Started,
            }
        }

        #[tokio::test]
        async fn should_collect_torrent_metrics() {
            let tracker = tracker_factory();

            let torrents_metrics = tracker.get_torrents_metrics().await;

            assert_eq!(
                torrents_metrics,
                TorrentsMetrics {
                    seeders: 0,
                    completed: 0,
                    leechers: 0,
                    torrents: 0
                }
            );
        }

        mod handling_an_announce_request {
            mod should_assign_the_ip_to_the_peer {

                use std::net::{IpAddr, Ipv4Addr};

                use crate::tracker::assign_ip_address_to_peer;

                #[test]
                fn using_the_source_ip_instead_of_the_ip_in_the_announce_request() {
                    let remote_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2));

                    let peer_ip = assign_ip_address_to_peer(&remote_ip, None);

                    assert_eq!(peer_ip, remote_ip);
                }

                mod and_when_the_client_ip_is_a_ipv4_loopback_ip {

                    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
                    use std::str::FromStr;

                    use crate::tracker::assign_ip_address_to_peer;

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
                    fn it_should_use_the_external_ip_in_the_tracker_configuration_if_it_is_defined_even_if_the_external_ip_is_an_ipv6_ip(
                    ) {
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

                    use crate::tracker::assign_ip_address_to_peer;

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
                    fn it_should_use_the_external_ip_in_the_tracker_configuration_if_it_is_defined_even_if_the_external_ip_is_an_ipv4_ip(
                    ) {
                        let remote_ip = IpAddr::V6(Ipv6Addr::LOCALHOST);

                        let tracker_external_ip = IpAddr::V4(Ipv4Addr::from_str("126.0.0.1").unwrap());

                        let peer_ip = assign_ip_address_to_peer(&remote_ip, Some(tracker_external_ip));

                        assert_eq!(peer_ip, tracker_external_ip);
                    }
                }
            }
        }

        mod handling_a_scrape_request {

            use std::net::{IpAddr, Ipv4Addr};

            use crate::protocol::info_hash::InfoHash;
            use crate::tracker::tests::the_tracker::{complete_peer, incomplete_peer, tracker_factory};
            use crate::tracker::{ScrapeData, SwarmMetadata};

            #[tokio::test]
            async fn it_should_return_a_zeroed_swarm_metadata_for_the_requested_file_if_the_tracker_does_not_have_that_torrent() {
                let tracker = tracker_factory();

                let info_hashes = vec!["3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap()];

                let scrape_data = tracker.scrape(&info_hashes).await;

                let mut expected_scrape_data = ScrapeData::empty();

                expected_scrape_data.add_file_with_no_metadata(&info_hashes[0]);

                assert_eq!(scrape_data, expected_scrape_data);
            }

            #[tokio::test]
            async fn it_should_return_the_swarm_metadata_for_the_requested_file_if_the_tracker_has_that_torrent() {
                let tracker = tracker_factory();

                let info_hash = "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap();

                // Announce a "complete" peer for the torrent
                let mut complete_peer = complete_peer();
                tracker
                    .announce(&info_hash, &mut complete_peer, &IpAddr::V4(Ipv4Addr::new(126, 0, 0, 10)))
                    .await;

                // Announce an "incomplete" peer for the torrent
                let mut incomplete_peer = incomplete_peer();
                tracker
                    .announce(&info_hash, &mut incomplete_peer, &IpAddr::V4(Ipv4Addr::new(126, 0, 0, 11)))
                    .await;

                // Scrape
                let scrape_data = tracker.scrape(&vec![info_hash]).await;

                // The expected swarm metadata for the file
                let mut expected_scrape_data = ScrapeData::empty();
                expected_scrape_data.add_file(
                    &info_hash,
                    SwarmMetadata {
                        complete: 0, // the "complete" peer does not count because it was not previously known
                        downloaded: 0,
                        incomplete: 1, // the "incomplete" peer we have just announced
                    },
                );

                assert_eq!(scrape_data, expected_scrape_data);
            }

            #[tokio::test]
            async fn it_should_allow_scraping_for_multiple_torrents() {
                let tracker = tracker_factory();

                let info_hashes = vec![
                    "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(),
                    "99c82bb73505a3c0b453f9fa0e881d6e5a32a0c1".parse::<InfoHash>().unwrap(),
                ];

                let scrape_data = tracker.scrape(&info_hashes).await;

                let mut expected_scrape_data = ScrapeData::empty();
                expected_scrape_data.add_file_with_no_metadata(&info_hashes[0]);
                expected_scrape_data.add_file_with_no_metadata(&info_hashes[1]);

                assert_eq!(scrape_data, expected_scrape_data);
            }
        }
    }
}
