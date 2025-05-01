use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ops::Sub;
use std::time::Duration;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use rstest::{fixture, rstest};
use torrust_tracker_clock::clock::stopped::Stopped as _;
use torrust_tracker_clock::clock::{self, Time as _};
use torrust_tracker_configuration::{TrackerPolicy, TORRENT_PEERS_LIMIT};
use torrust_tracker_primitives::peer;
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_torrent_repository::{entry, TrackedTorrentHandle};

use crate::common::torrent::Torrent;
use crate::common::torrent_peer_builder::{a_completed_peer, a_started_peer};
use crate::CurrentClock;

#[fixture]
fn single() -> Torrent {
    Torrent::Single(entry::torrent::TrackedTorrent::default())
}
#[fixture]
fn mutex_std() -> Torrent {
    Torrent::MutexStd(TrackedTorrentHandle::default())
}

#[fixture]
fn policy_none() -> TrackerPolicy {
    TrackerPolicy::new(0, false, false)
}

#[fixture]
fn policy_persist() -> TrackerPolicy {
    TrackerPolicy::new(0, true, false)
}

#[fixture]
fn policy_remove() -> TrackerPolicy {
    TrackerPolicy::new(0, false, true)
}

#[fixture]
fn policy_remove_persist() -> TrackerPolicy {
    TrackerPolicy::new(0, true, true)
}

pub enum Makes {
    Empty,
    Started,
    Completed,
    Downloaded,
    Three,
}

fn make(torrent: &mut Torrent, makes: &Makes) -> Vec<Peer> {
    match makes {
        Makes::Empty => vec![],
        Makes::Started => {
            let peer = a_started_peer(1);
            torrent.upsert_peer(&peer);
            vec![peer]
        }
        Makes::Completed => {
            let peer = a_completed_peer(2);
            torrent.upsert_peer(&peer);
            vec![peer]
        }
        Makes::Downloaded => {
            let mut peer = a_started_peer(3);
            torrent.upsert_peer(&peer);
            peer.event = AnnounceEvent::Completed;
            peer.left = NumberOfBytes::new(0);
            torrent.upsert_peer(&peer);
            vec![peer]
        }
        Makes::Three => {
            let peer_1 = a_started_peer(1);
            torrent.upsert_peer(&peer_1);

            let peer_2 = a_completed_peer(2);
            torrent.upsert_peer(&peer_2);

            let mut peer_3 = a_started_peer(3);
            torrent.upsert_peer(&peer_3);
            peer_3.event = AnnounceEvent::Completed;
            peer_3.left = NumberOfBytes::new(0);
            torrent.upsert_peer(&peer_3);
            vec![peer_1, peer_2, peer_3]
        }
    }
}

#[rstest]
#[case::empty(&Makes::Empty)]
#[tokio::test]
async fn it_should_be_empty_by_default(#[values(single(), mutex_std())] mut torrent: Torrent, #[case] makes: &Makes) {
    make(&mut torrent, makes);

    assert_eq!(torrent.get_peers_len(), 0);
}

#[rstest]
#[case::empty(&Makes::Empty)]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_check_if_entry_should_be_retained_based_on_the_tracker_policy(
    #[values(single(), mutex_std())] mut torrent: Torrent,
    #[case] makes: &Makes,
    #[values(policy_none(), policy_persist(), policy_remove(), policy_remove_persist())] policy: TrackerPolicy,
) {
    make(&mut torrent, makes);

    let has_peers = !torrent.peers_is_empty();
    let has_downloads = torrent.get_stats().downloaded != 0;

    match (policy.remove_peerless_torrents, policy.persistent_torrent_completed_stat) {
        // remove torrents without peers, and keep completed download stats
        (true, true) => match (has_peers, has_downloads) {
            // no peers, but has downloads
            // peers, with or without downloads
            (false, true) | (true, true | false) => assert!(torrent.meets_retaining_policy(&policy)),
            // no peers and no downloads
            (false, false) => assert!(!torrent.meets_retaining_policy(&policy)),
        },
        // remove torrents without peers and drop completed download stats
        (true, false) => match (has_peers, has_downloads) {
            // peers, with or without downloads
            (true, true | false) => assert!(torrent.meets_retaining_policy(&policy)),
            // no peers and with or without downloads
            (false, true | false) => assert!(!torrent.meets_retaining_policy(&policy)),
        },
        // keep torrents without peers, but keep or drop completed download stats
        (false, true | false) => assert!(torrent.meets_retaining_policy(&policy)),
    }
}

#[rstest]
#[case::empty(&Makes::Empty)]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_get_peers_for_torrent_entry(#[values(single(), mutex_std())] mut torrent: Torrent, #[case] makes: &Makes) {
    let peers = make(&mut torrent, makes);

    let torrent_peers = torrent.get_peers(None);

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
async fn it_should_update_a_peer(#[values(single(), mutex_std())] mut torrent: Torrent, #[case] makes: &Makes) {
    make(&mut torrent, makes);

    // Make and insert a new peer.
    let mut peer = a_started_peer(-1);
    torrent.upsert_peer(&peer);

    // Get the Inserted Peer by Id.
    let peers = torrent.get_peers(None);
    let original = peers
        .iter()
        .find(|p| peer::ReadInfo::get_id(*p) == peer::ReadInfo::get_id(&peer))
        .expect("it should find peer by id");

    assert_eq!(original.event, AnnounceEvent::Started, "it should be as created");

    // Announce "Completed" torrent download event.
    peer.event = AnnounceEvent::Completed;
    torrent.upsert_peer(&peer);

    // Get the Updated Peer by Id.
    let peers = torrent.get_peers(None);
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
    #[values(single(), mutex_std())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    use torrust_tracker_primitives::peer::ReadInfo as _;

    make(&mut torrent, makes);

    let mut peer = a_started_peer(-1);

    torrent.upsert_peer(&peer);

    // The started peer should be inserted.
    let peers = torrent.get_peers(None);
    let original = peers
        .iter()
        .find(|p| p.get_id() == peer.get_id())
        .expect("it should find peer by id");

    assert_eq!(original.event, AnnounceEvent::Started);

    // Change peer to "Stopped" and insert.
    peer.event = AnnounceEvent::Stopped;
    torrent.upsert_peer(&peer);

    // It should be removed now.
    let peers = torrent.get_peers(None);

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
    #[values(single(), mutex_std())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    make(&mut torrent, makes);
    let downloaded = torrent.get_stats().downloaded;

    let peers = torrent.get_peers(None);
    let mut peer = **peers.first().expect("there should be a peer");

    let is_already_completed = peer.event == AnnounceEvent::Completed;

    // Announce "Completed" torrent download event.
    peer.event = AnnounceEvent::Completed;

    torrent.upsert_peer(&peer);
    let stats = torrent.get_stats();

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
async fn it_should_update_a_peer_as_a_seeder(#[values(single(), mutex_std())] mut torrent: Torrent, #[case] makes: &Makes) {
    let peers = make(&mut torrent, makes);
    let completed = u32::try_from(peers.iter().filter(|p| p.is_seeder()).count()).expect("it_should_not_be_so_many");

    let peers = torrent.get_peers(None);
    let mut peer = **peers.first().expect("there should be a peer");

    let is_already_non_left = peer.left == NumberOfBytes::new(0);

    // Set Bytes Left to Zero
    peer.left = NumberOfBytes::new(0);
    torrent.upsert_peer(&peer);
    let stats = torrent.get_stats();

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
async fn it_should_update_a_peer_as_incomplete(#[values(single(), mutex_std())] mut torrent: Torrent, #[case] makes: &Makes) {
    let peers = make(&mut torrent, makes);
    let incomplete = u32::try_from(peers.iter().filter(|p| !p.is_seeder()).count()).expect("it should not be so many");

    let peers = torrent.get_peers(None);
    let mut peer = **peers.first().expect("there should be a peer");

    let completed_already = peer.left == NumberOfBytes::new(0);

    // Set Bytes Left to no Zero
    peer.left = NumberOfBytes::new(1);
    torrent.upsert_peer(&peer);
    let stats = torrent.get_stats();

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
    #[values(single(), mutex_std())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    make(&mut torrent, makes);

    let peers = torrent.get_peers(None);
    let mut peer = **peers.first().expect("there should be a peer");

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081);

    // for this test, we should not already use this socket.
    assert_ne!(peer.peer_addr, socket);

    // it should get the peer as it dose not share the socket.
    assert!(torrent.get_peers_for_client(&socket, None).contains(&peer.into()));

    // set the address to the socket.
    peer.peer_addr = socket;
    torrent.upsert_peer(&peer); // Add peer

    // It should not include the peer that has the same socket.
    assert!(!torrent.get_peers_for_client(&socket, None).contains(&peer.into()));
}

#[rstest]
#[case::empty(&Makes::Empty)]
#[case::started(&Makes::Started)]
#[case::completed(&Makes::Completed)]
#[case::downloaded(&Makes::Downloaded)]
#[case::three(&Makes::Three)]
#[tokio::test]
async fn it_should_limit_the_number_of_peers_returned(
    #[values(single(), mutex_std())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    make(&mut torrent, makes);

    // We add one more peer than the scrape limit
    for peer_number in 1..=74 + 1 {
        let mut peer = a_started_peer(1);
        peer.peer_id = *peer::Id::new(peer_number);
        torrent.upsert_peer(&peer);
    }

    let peers = torrent.get_peers(Some(TORRENT_PEERS_LIMIT));

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
    #[values(single(), mutex_std())] mut torrent: Torrent,
    #[case] makes: &Makes,
) {
    const TIMEOUT: Duration = Duration::from_secs(120);
    const EXPIRE: Duration = Duration::from_secs(121);

    let peers = make(&mut torrent, makes);

    let mut peer = a_completed_peer(-1);

    let now = clock::Working::now();
    clock::Stopped::local_set(&now);

    peer.updated = now.sub(EXPIRE);

    torrent.upsert_peer(&peer);

    assert_eq!(torrent.get_peers_len(), peers.len() + 1);

    let current_cutoff = CurrentClock::now_sub(&TIMEOUT).unwrap_or_default();
    torrent.remove_inactive_peers(current_cutoff);

    assert_eq!(torrent.get_peers_len(), peers.len());
}
