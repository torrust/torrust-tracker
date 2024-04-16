use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ops::Sub;
use std::time::Duration;

use rstest::{fixture, rstest};
use torrust_tracker_clock::clock::stopped::Stopped as _;
use torrust_tracker_clock::clock::{self, Time as _};
use torrust_tracker_configuration::{TrackerPolicy, TORRENT_PEERS_LIMIT};
use torrust_tracker_primitives::announce_event::AnnounceEvent;
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::{peer, NumberOfBytes};
use torrust_tracker_torrent_repository::{
    EntryMutexParkingLot, EntryMutexStd, EntryMutexTokio, EntryRwLockParkingLot, EntrySingle,
};

use crate::common::torrent::Torrent;
use crate::common::torrent_peer_builder::{a_completed_peer, a_started_peer};
use crate::CurrentClock;

#[fixture]
fn single() -> Torrent {
    Torrent::Single(EntrySingle::default())
}
#[fixture]
fn mutex_std() -> Torrent {
    Torrent::MutexStd(EntryMutexStd::default())
}

#[fixture]
fn mutex_tokio() -> Torrent {
    Torrent::MutexTokio(EntryMutexTokio::default())
}

#[fixture]
fn mutex_parking_lot() -> Torrent {
    Torrent::MutexParkingLot(EntryMutexParkingLot::default())
}

#[fixture]
fn rw_lock_parking_lot() -> Torrent {
    Torrent::RwLockParkingLot(EntryRwLockParkingLot::default())
}

#[fixture]
fn policy_none() -> TrackerPolicy {
    TrackerPolicy::new(false, 0, false)
}

#[fixture]
fn policy_persist() -> TrackerPolicy {
    TrackerPolicy::new(false, 0, true)
}

#[fixture]
fn policy_remove() -> TrackerPolicy {
    TrackerPolicy::new(true, 0, false)
}

#[fixture]
fn policy_remove_persist() -> TrackerPolicy {
    TrackerPolicy::new(true, 0, true)
}

pub enum Makes {
    Empty,
    Started,
    Completed,
    Downloaded,
    Three,
}

async fn make(torrent: &mut Torrent, makes: &Makes) -> Vec<Peer> {
    match makes {
        Makes::Empty => vec![],
        Makes::Started => {
            let peer = a_started_peer(1);
            torrent.upsert_peer(&peer).await;
            vec![peer]
        }
        Makes::Completed => {
            let peer = a_completed_peer(2);
            torrent.upsert_peer(&peer).await;
            vec![peer]
        }
        Makes::Downloaded => {
            let mut peer = a_started_peer(3);
            torrent.upsert_peer(&peer).await;
            peer.event = AnnounceEvent::Completed;
            peer.left = NumberOfBytes(0);
            torrent.upsert_peer(&peer).await;
            vec![peer]
        }
        Makes::Three => {
            let peer_1 = a_started_peer(1);
            torrent.upsert_peer(&peer_1).await;

            let peer_2 = a_completed_peer(2);
            torrent.upsert_peer(&peer_2).await;

            let mut peer_3 = a_started_peer(3);
            torrent.upsert_peer(&peer_3).await;
            peer_3.event = AnnounceEvent::Completed;
            peer_3.left = NumberOfBytes(0);
            torrent.upsert_peer(&peer_3).await;
            vec![peer_1, peer_2, peer_3]
        }
    }
}

#[rstest]
#[case::empty(&Makes::Empty)]
#[tokio::test]
async fn it_should_be_empty_by_default(
    #[values(single(), mutex_std(), mutex_tokio(), mutex_parking_lot(), rw_lock_parking_lot())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    make(&mut torrent, makes).await;

    assert_eq!(torrent.get_peers_len().await, 0);
}

#[rstest]
#[case::empty(&Makes::Empty)]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_check_if_entry_is_good(
    #[values(single(), mutex_std(), mutex_tokio(), mutex_parking_lot(), rw_lock_parking_lot())] mut torrent: Torrent,
    #[case] makes: &Makes,
    #[values(policy_none(), policy_persist(), policy_remove(), policy_remove_persist())] policy: TrackerPolicy,
) {
    make(&mut torrent, makes).await;

    let has_peers = !torrent.peers_is_empty().await;
    let has_downloads = torrent.get_stats().await.downloaded != 0;

    match (policy.remove_peerless_torrents, policy.persistent_torrent_completed_stat) {
        // remove torrents without peers, and keep completed download stats
        (true, true) => match (has_peers, has_downloads) {
            // no peers, but has downloads
            // peers, with or without downloads
            (false, true) | (true, true | false) => assert!(torrent.is_good(&policy).await),
            // no peers and no downloads
            (false, false) => assert!(!torrent.is_good(&policy).await),
        },
        // remove torrents without peers and drop completed download stats
        (true, false) => match (has_peers, has_downloads) {
            // peers, with or without downloads
            (true, true | false) => assert!(torrent.is_good(&policy).await),
            // no peers and with or without downloads
            (false, true | false) => assert!(!torrent.is_good(&policy).await),
        },
        // keep torrents without peers, but keep or drop completed download stats
        (false, true | false) => assert!(torrent.is_good(&policy).await),
    }
}

#[rstest]
#[case::empty(&Makes::Empty)]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_get_peers_for_torrent_entry(
    #[values(single(), mutex_std(), mutex_tokio(), mutex_parking_lot(), rw_lock_parking_lot())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    let peers = make(&mut torrent, makes).await;

    let torrent_peers = torrent.get_peers(None).await;

    assert_eq!(torrent_peers.len(), peers.len());

    for peer in torrent_peers {
        assert!(peers.contains(&peer));
    }
}

#[rstest]
#[case::empty(&Makes::Empty)]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_update_a_peer(#[values(single(), mutex_std(), mutex_tokio())] mut torrent: Torrent, #[case] makes: &Makes) {
    make(&mut torrent, makes).await;

    // Make and insert a new peer.
    let mut peer = a_started_peer(-1);
    torrent.upsert_peer(&peer).await;

    // Get the Inserted Peer by Id.
    let peers = torrent.get_peers(None).await;
    let original = peers
        .iter()
        .find(|p| peer::ReadInfo::get_id(*p) == peer::ReadInfo::get_id(&peer))
        .expect("it should find peer by id");

    assert_eq!(original.event, AnnounceEvent::Started, "it should be as created");

    // Announce "Completed" torrent download event.
    peer.event = AnnounceEvent::Completed;
    torrent.upsert_peer(&peer).await;

    // Get the Updated Peer by Id.
    let peers = torrent.get_peers(None).await;
    let updated = peers
        .iter()
        .find(|p| peer::ReadInfo::get_id(*p) == peer::ReadInfo::get_id(&peer))
        .expect("it should find peer by id");

    assert_eq!(updated.event, AnnounceEvent::Completed, "it should be updated");
}

#[rstest]
#[case::empty(&Makes::Empty)]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_remove_a_peer_upon_stopped_announcement(
    #[values(single(), mutex_std(), mutex_tokio(), mutex_parking_lot(), rw_lock_parking_lot())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    use torrust_tracker_primitives::peer::ReadInfo as _;

    make(&mut torrent, makes).await;

    let mut peer = a_started_peer(-1);

    torrent.upsert_peer(&peer).await;

    // The started peer should be inserted.
    let peers = torrent.get_peers(None).await;
    let original = peers
        .iter()
        .find(|p| p.get_id() == peer.get_id())
        .expect("it should find peer by id");

    assert_eq!(original.event, AnnounceEvent::Started);

    // Change peer to "Stopped" and insert.
    peer.event = AnnounceEvent::Stopped;
    torrent.upsert_peer(&peer).await;

    // It should be removed now.
    let peers = torrent.get_peers(None).await;

    assert_eq!(
        peers.iter().find(|p| p.get_id() == peer.get_id()),
        None,
        "it should be removed"
    );
}

#[rstest]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_handle_a_peer_completed_announcement_and_update_the_downloaded_statistic(
    #[values(single(), mutex_std(), mutex_tokio(), mutex_parking_lot(), rw_lock_parking_lot())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    make(&mut torrent, makes).await;
    let downloaded = torrent.get_stats().await.downloaded;

    let peers = torrent.get_peers(None).await;
    let mut peer = **peers.first().expect("there should be a peer");

    let is_already_completed = peer.event == AnnounceEvent::Completed;

    // Announce "Completed" torrent download event.
    peer.event = AnnounceEvent::Completed;

    torrent.upsert_peer(&peer).await;
    let stats = torrent.get_stats().await;

    if is_already_completed {
        assert_eq!(stats.downloaded, downloaded);
    } else {
        assert_eq!(stats.downloaded, downloaded + 1);
    }
}

#[rstest]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_update_a_peer_as_a_seeder(
    #[values(single(), mutex_std(), mutex_tokio(), mutex_parking_lot(), rw_lock_parking_lot())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    let peers = make(&mut torrent, makes).await;
    let completed = u32::try_from(peers.iter().filter(|p| p.is_seeder()).count()).expect("it_should_not_be_so_many");

    let peers = torrent.get_peers(None).await;
    let mut peer = **peers.first().expect("there should be a peer");

    let is_already_non_left = peer.left == NumberOfBytes(0);

    // Set Bytes Left to Zero
    peer.left = NumberOfBytes(0);
    torrent.upsert_peer(&peer).await;
    let stats = torrent.get_stats().await;

    if is_already_non_left {
        // it was already complete
        assert_eq!(stats.complete, completed);
    } else {
        // now it is complete
        assert_eq!(stats.complete, completed + 1);
    }
}

#[rstest]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_update_a_peer_as_incomplete(
    #[values(single(), mutex_std(), mutex_tokio(), mutex_parking_lot(), rw_lock_parking_lot())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    let peers = make(&mut torrent, makes).await;
    let incomplete = u32::try_from(peers.iter().filter(|p| !p.is_seeder()).count()).expect("it should not be so many");

    let peers = torrent.get_peers(None).await;
    let mut peer = **peers.first().expect("there should be a peer");

    let completed_already = peer.left == NumberOfBytes(0);

    // Set Bytes Left to no Zero
    peer.left = NumberOfBytes(1);
    torrent.upsert_peer(&peer).await;
    let stats = torrent.get_stats().await;

    if completed_already {
        // now it is incomplete
        assert_eq!(stats.incomplete, incomplete + 1);
    } else {
        // was already incomplete
        assert_eq!(stats.incomplete, incomplete);
    }
}

#[rstest]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_get_peers_excluding_the_client_socket(
    #[values(single(), mutex_std(), mutex_tokio(), mutex_parking_lot(), rw_lock_parking_lot())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    make(&mut torrent, makes).await;

    let peers = torrent.get_peers(None).await;
    let mut peer = **peers.first().expect("there should be a peer");

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081);

    // for this test, we should not already use this socket.
    assert_ne!(peer.peer_addr, socket);

    // it should get the peer as it dose not share the socket.
    assert!(torrent.get_peers_for_client(&socket, None).await.contains(&peer.into()));

    // set the address to the socket.
    peer.peer_addr = socket;
    torrent.upsert_peer(&peer).await; // Add peer

    // It should not include the peer that has the same socket.
    assert!(!torrent.get_peers_for_client(&socket, None).await.contains(&peer.into()));
}

#[rstest]
#[case::empty(&Makes::Empty)]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_limit_the_number_of_peers_returned(
    #[values(single(), mutex_std(), mutex_tokio(), mutex_parking_lot(), rw_lock_parking_lot())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    make(&mut torrent, makes).await;

    // We add one more peer than the scrape limit
    for peer_number in 1..=74 + 1 {
        let mut peer = a_started_peer(1);
        peer.peer_id = peer::Id::from(peer_number);
        torrent.upsert_peer(&peer).await;
    }

    let peers = torrent.get_peers(Some(TORRENT_PEERS_LIMIT)).await;

    assert_eq!(peers.len(), 74);
}

#[rstest]
#[case::empty(&Makes::Empty)]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_remove_inactive_peers_beyond_cutoff(
    #[values(single(), mutex_std(), mutex_tokio(), mutex_parking_lot(), rw_lock_parking_lot())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    const TIMEOUT: Duration = Duration::from_secs(120);
    const EXPIRE: Duration = Duration::from_secs(121);

    let peers = make(&mut torrent, makes).await;

    let mut peer = a_completed_peer(-1);

    let now = clock::Working::now();
    clock::Stopped::local_set(&now);

    peer.updated = now.sub(EXPIRE);

    torrent.upsert_peer(&peer).await;

    assert_eq!(torrent.get_peers_len().await, peers.len() + 1);

    let current_cutoff = CurrentClock::now_sub(&TIMEOUT).unwrap_or_default();
    torrent.remove_inactive_peers(current_cutoff).await;

    assert_eq!(torrent.get_peers_len().await, peers.len());
}
