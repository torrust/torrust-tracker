use std::collections::{BTreeMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, Mutex};

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use bittorrent_primitives::info_hash::InfoHash;
use rstest::{fixture, rstest};
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::PersistentTorrents;
use torrust_tracker_torrent_repository::swarm::Swarm;
use torrust_tracker_torrent_repository::{LockTrackedTorrent, Swarms};

use crate::common::torrent_peer_builder::{a_completed_peer, a_started_peer};

#[fixture]
fn skip_list_mutex_std() -> Swarms {
    Swarms::default()
}

type Entries = Vec<(InfoHash, Swarm)>;

#[fixture]
fn empty() -> Entries {
    vec![]
}

#[fixture]
fn default() -> Entries {
    vec![(InfoHash::default(), Swarm::default())]
}

#[fixture]
fn started() -> Entries {
    let mut torrent = Swarm::default();
    torrent.handle_announcement(&a_started_peer(1));
    vec![(InfoHash::default(), torrent)]
}

#[fixture]
fn completed() -> Entries {
    let mut torrent = Swarm::default();
    torrent.handle_announcement(&a_completed_peer(2));
    vec![(InfoHash::default(), torrent)]
}

#[fixture]
fn downloaded() -> Entries {
    let mut torrent = Swarm::default();
    let mut peer = a_started_peer(3);
    torrent.handle_announcement(&peer);
    peer.event = AnnounceEvent::Completed;
    peer.left = NumberOfBytes::new(0);
    torrent.handle_announcement(&peer);
    vec![(InfoHash::default(), torrent)]
}

#[fixture]
fn three() -> Entries {
    let mut started = Swarm::default();
    let started_h = &mut DefaultHasher::default();
    started.handle_announcement(&a_started_peer(1));
    started.hash(started_h);

    let mut completed = Swarm::default();
    let completed_h = &mut DefaultHasher::default();
    completed.handle_announcement(&a_completed_peer(2));
    completed.hash(completed_h);

    let mut downloaded = Swarm::default();
    let downloaded_h = &mut DefaultHasher::default();
    let mut downloaded_peer = a_started_peer(3);
    downloaded.handle_announcement(&downloaded_peer);
    downloaded_peer.event = AnnounceEvent::Completed;
    downloaded_peer.left = NumberOfBytes::new(0);
    downloaded.handle_announcement(&downloaded_peer);
    downloaded.hash(downloaded_h);

    vec![
        (InfoHash::from(&started_h.clone()), started),
        (InfoHash::from(&completed_h.clone()), completed),
        (InfoHash::from(&downloaded_h.clone()), downloaded),
    ]
}

#[fixture]
fn many_out_of_order() -> Entries {
    let mut entries: HashSet<(InfoHash, Swarm)> = HashSet::default();

    for i in 0..408 {
        let mut entry = Swarm::default();
        entry.handle_announcement(&a_started_peer(i));

        entries.insert((InfoHash::from(&i), entry));
    }

    // we keep the random order from the hashed set for the vector.
    entries.iter().map(|(i, e)| (*i, e.clone())).collect()
}

#[fixture]
fn many_hashed_in_order() -> Entries {
    let mut entries: BTreeMap<InfoHash, Swarm> = BTreeMap::default();

    for i in 0..408 {
        let mut entry = Swarm::default();
        entry.handle_announcement(&a_started_peer(i));

        let hash: &mut DefaultHasher = &mut DefaultHasher::default();
        hash.write_i32(i);

        entries.insert(InfoHash::from(&hash.clone()), entry);
    }

    // We return the entries in-order from from the b-tree map.
    entries.iter().map(|(i, e)| (*i, e.clone())).collect()
}

#[fixture]
fn persistent_empty() -> PersistentTorrents {
    PersistentTorrents::default()
}

#[fixture]
fn persistent_single() -> PersistentTorrents {
    let hash = &mut DefaultHasher::default();

    hash.write_u8(1);
    let t = [(InfoHash::from(&hash.clone()), 0_u32)];

    t.iter().copied().collect()
}

#[fixture]
fn persistent_three() -> PersistentTorrents {
    let hash = &mut DefaultHasher::default();

    hash.write_u8(1);
    let info_1 = InfoHash::from(&hash.clone());
    hash.write_u8(2);
    let info_2 = InfoHash::from(&hash.clone());
    hash.write_u8(3);
    let info_3 = InfoHash::from(&hash.clone());

    let t = [(info_1, 1_u32), (info_2, 2_u32), (info_3, 3_u32)];

    t.iter().copied().collect()
}

fn make(repo: &Swarms, entries: &Entries) {
    for (info_hash, entry) in entries {
        let new = Arc::new(Mutex::new(entry.clone()));
        // todo: use a public method to insert an empty swarm.
        repo.swarms.insert(*info_hash, new);
    }
}

#[fixture]
fn paginated_limit_zero() -> Pagination {
    Pagination::new(0, 0)
}

#[fixture]
fn paginated_limit_one() -> Pagination {
    Pagination::new(0, 1)
}

#[fixture]
fn paginated_limit_one_offset_one() -> Pagination {
    Pagination::new(1, 1)
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

#[rstest]
#[case::empty(empty())]
#[case::default(default())]
#[case::started(started())]
#[case::completed(completed())]
#[case::downloaded(downloaded())]
#[case::three(three())]
#[case::out_of_order(many_out_of_order())]
#[case::in_order(many_hashed_in_order())]
#[tokio::test]
async fn it_should_get_a_torrent_entry(#[values(skip_list_mutex_std())] repo: Swarms, #[case] entries: Entries) {
    make(&repo, &entries);

    if let Some((info_hash, torrent)) = entries.first() {
        assert_eq!(
            Some(repo.get(info_hash).unwrap().lock_or_panic().clone()),
            Some(torrent.clone())
        );
    } else {
        assert!(repo.get(&InfoHash::default()).is_none());
    }
}

#[rstest]
#[case::empty(empty())]
#[case::default(default())]
#[case::started(started())]
#[case::completed(completed())]
#[case::downloaded(downloaded())]
#[case::three(three())]
#[case::out_of_order(many_out_of_order())]
#[case::in_order(many_hashed_in_order())]
#[tokio::test]
async fn it_should_get_paginated_entries_in_a_stable_or_sorted_order(
    #[values(skip_list_mutex_std())] repo: Swarms,
    #[case] entries: Entries,
    many_out_of_order: Entries,
) {
    make(&repo, &entries);

    let entries_a = repo.get_paginated(None).iter().map(|(i, _)| *i).collect::<Vec<_>>();

    make(&repo, &many_out_of_order);

    let entries_b = repo.get_paginated(None).iter().map(|(i, _)| *i).collect::<Vec<_>>();

    let is_equal = entries_b.iter().take(entries_a.len()).copied().collect::<Vec<_>>() == entries_a;

    let is_sorted = entries_b.windows(2).all(|w| w[0] <= w[1]);

    assert!(
        is_equal || is_sorted,
        "The order is unstable: {is_equal}, or is sorted {is_sorted}."
    );
}

#[rstest]
#[case::empty(empty())]
#[case::default(default())]
#[case::started(started())]
#[case::completed(completed())]
#[case::downloaded(downloaded())]
#[case::three(three())]
#[case::out_of_order(many_out_of_order())]
#[case::in_order(many_hashed_in_order())]
#[tokio::test]
async fn it_should_get_paginated(
    #[values(skip_list_mutex_std())] repo: Swarms,
    #[case] entries: Entries,
    #[values(paginated_limit_zero(), paginated_limit_one(), paginated_limit_one_offset_one())] paginated: Pagination,
) {
    make(&repo, &entries);

    let mut info_hashes = repo.get_paginated(None).iter().map(|(i, _)| *i).collect::<Vec<_>>();
    info_hashes.sort();

    match paginated {
        // it should return empty if limit is zero.
        Pagination { limit: 0, .. } => {
            let torrents: Vec<(InfoHash, Swarm)> = repo
                .get_paginated(Some(&paginated))
                .iter()
                .map(|(i, lock_tracked_torrent)| (*i, lock_tracked_torrent.lock_or_panic().clone()))
                .collect();

            assert_eq!(torrents, vec![]);
        }

        // it should return a single entry if the limit is one.
        Pagination { limit: 1, offset: 0 } => {
            if info_hashes.is_empty() {
                assert_eq!(repo.get_paginated(Some(&paginated)).len(), 0);
            } else {
                let page = repo.get_paginated(Some(&paginated));
                assert_eq!(page.len(), 1);
                assert_eq!(page.first().map(|(i, _)| i), info_hashes.first());
            }
        }

        // it should return the only the second entry if both the limit and the offset are one.
        Pagination { limit: 1, offset: 1 } => {
            if info_hashes.len() > 1 {
                let page = repo.get_paginated(Some(&paginated));
                assert_eq!(page.len(), 1);
                assert_eq!(page[0].0, info_hashes[1]);
            }
        }
        // the other cases are not yet tested.
        _ => {}
    }
}

#[rstest]
#[case::empty(empty())]
#[case::default(default())]
#[case::started(started())]
#[case::completed(completed())]
#[case::downloaded(downloaded())]
#[case::three(three())]
#[case::out_of_order(many_out_of_order())]
#[case::in_order(many_hashed_in_order())]
#[tokio::test]
async fn it_should_get_metrics(#[values(skip_list_mutex_std())] repo: Swarms, #[case] entries: Entries) {
    use torrust_tracker_primitives::swarm_metadata::AggregateSwarmMetadata;

    make(&repo, &entries);

    let mut metrics = AggregateSwarmMetadata::default();

    for (_, torrent) in entries {
        let stats = torrent.metadata();

        metrics.total_torrents += 1;
        metrics.total_incomplete += u64::from(stats.incomplete);
        metrics.total_complete += u64::from(stats.complete);
        metrics.total_downloaded += u64::from(stats.downloaded);
    }

    assert_eq!(repo.get_aggregate_swarm_metadata(), metrics);
}

#[rstest]
#[case::empty(empty())]
#[case::default(default())]
#[case::started(started())]
#[case::completed(completed())]
#[case::downloaded(downloaded())]
#[case::three(three())]
#[case::out_of_order(many_out_of_order())]
#[case::in_order(many_hashed_in_order())]
#[tokio::test]
async fn it_should_import_persistent_torrents(
    #[values(skip_list_mutex_std())] repo: Swarms,
    #[case] entries: Entries,
    #[values(persistent_empty(), persistent_single(), persistent_three())] persistent_torrents: PersistentTorrents,
) {
    make(&repo, &entries);

    let mut downloaded = repo.get_aggregate_swarm_metadata().total_downloaded;
    persistent_torrents.iter().for_each(|(_, d)| downloaded += u64::from(*d));

    repo.import_persistent(&persistent_torrents);

    assert_eq!(repo.get_aggregate_swarm_metadata().total_downloaded, downloaded);

    for (entry, _) in persistent_torrents {
        assert!(repo.get(&entry).is_some());
    }
}

#[rstest]
#[case::empty(empty())]
#[case::default(default())]
#[case::started(started())]
#[case::completed(completed())]
#[case::downloaded(downloaded())]
#[case::three(three())]
#[case::out_of_order(many_out_of_order())]
#[case::in_order(many_hashed_in_order())]
#[tokio::test]
async fn it_should_remove_an_entry(#[values(skip_list_mutex_std())] repo: Swarms, #[case] entries: Entries) {
    make(&repo, &entries);

    for (info_hash, torrent) in entries {
        assert_eq!(
            Some(repo.get(&info_hash).unwrap().lock_or_panic().clone()),
            Some(torrent.clone())
        );
        assert_eq!(Some(repo.remove(&info_hash).unwrap().lock_or_panic().clone()), Some(torrent));

        assert!(repo.get(&info_hash).is_none());
        assert!(repo.remove(&info_hash).is_none());
    }

    assert_eq!(repo.get_aggregate_swarm_metadata().total_torrents, 0);
}

#[rstest]
#[case::empty(empty())]
#[case::default(default())]
#[case::started(started())]
#[case::completed(completed())]
#[case::downloaded(downloaded())]
#[case::three(three())]
#[case::out_of_order(many_out_of_order())]
#[case::in_order(many_hashed_in_order())]
#[tokio::test]
async fn it_should_remove_inactive_peers(#[values(skip_list_mutex_std())] repo: Swarms, #[case] entries: Entries) {
    use std::ops::Sub as _;
    use std::time::Duration;

    use torrust_tracker_clock::clock::stopped::Stopped as _;
    use torrust_tracker_clock::clock::{self, Time as _};
    use torrust_tracker_primitives::peer;

    use crate::CurrentClock;

    const TIMEOUT: Duration = Duration::from_secs(120);
    const EXPIRE: Duration = Duration::from_secs(121);

    make(&repo, &entries);

    let info_hash: InfoHash;
    let mut peer: peer::Peer;

    // Generate a new infohash and peer.
    {
        let hash = &mut DefaultHasher::default();
        hash.write_u8(255);
        info_hash = InfoHash::from(&hash.clone());
        peer = a_completed_peer(-1);
    }

    // Set the last updated time of the peer to be 121 seconds ago.
    {
        let now = clock::Working::now();
        clock::Stopped::local_set(&now);

        peer.updated = now.sub(EXPIRE);
    }

    // Insert the infohash and peer into the repository
    // and verify there is an extra torrent entry.
    {
        repo.upsert_peer(&info_hash, &peer, None);
        assert_eq!(repo.get_aggregate_swarm_metadata().total_torrents, entries.len() as u64 + 1);
    }

    // Insert the infohash and peer into the repository
    // and verify the swarm metadata was updated.
    {
        repo.upsert_peer(&info_hash, &peer, None);
        let stats = repo.get_swarm_metadata(&info_hash);
        assert_eq!(
            stats,
            Some(SwarmMetadata {
                downloaded: 0,
                complete: 1,
                incomplete: 0
            })
        );
    }

    // Verify that this new peer was inserted into the repository.
    {
        let lock_tracked_torrent = repo.get(&info_hash).expect("it_should_get_some");
        let entry = lock_tracked_torrent.lock_or_panic();
        assert!(entry.peers(None).contains(&peer.into()));
    }

    // Remove peers that have not been updated since the timeout (120 seconds ago).
    {
        repo.remove_inactive_peers(CurrentClock::now_sub(&TIMEOUT).expect("it should get a time passed"));
    }

    // Verify that the this peer was removed from the repository.
    {
        let lock_tracked_torrent = repo.get(&info_hash).expect("it_should_get_some");
        let entry = lock_tracked_torrent.lock_or_panic();
        assert!(!entry.peers(None).contains(&peer.into()));
    }
}

#[rstest]
#[case::empty(empty())]
#[case::default(default())]
#[case::started(started())]
#[case::completed(completed())]
#[case::downloaded(downloaded())]
#[case::three(three())]
#[case::out_of_order(many_out_of_order())]
#[case::in_order(many_hashed_in_order())]
#[tokio::test]
async fn it_should_remove_peerless_torrents(
    #[values(skip_list_mutex_std())] repo: Swarms,
    #[case] entries: Entries,
    #[values(policy_none(), policy_persist(), policy_remove(), policy_remove_persist())] policy: TrackerPolicy,
) {
    make(&repo, &entries);

    repo.remove_peerless_torrents(&policy);

    let torrents: Vec<(InfoHash, Swarm)> = repo
        .get_paginated(None)
        .iter()
        .map(|(i, lock_tracked_torrent)| (*i, lock_tracked_torrent.lock_or_panic().clone()))
        .collect();

    for (_, entry) in torrents {
        assert!(entry.meets_retaining_policy(&policy));
    }
}
