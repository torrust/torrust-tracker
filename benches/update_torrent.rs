use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use criterion::{criterion_group, criterion_main, Criterion};
use futures::future;
use once_cell::sync::Lazy;
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash;
use torrust_tracker::shared::clock::DurationSinceUnixEpoch;
use torrust_tracker::tracker::peer::{Id, Peer};
use torrust_tracker::tracker::{statistics, Tracker};
use torrust_tracker_test_helpers::configuration;

const PEER: Peer = Peer {
    peer_id: Id([0; 20]),
    peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    updated: DurationSinceUnixEpoch::from_secs(0),
    uploaded: NumberOfBytes(0),
    downloaded: NumberOfBytes(0),
    left: NumberOfBytes(0),
    event: AnnounceEvent::Started,
};

// Define a vector of 20 different info hashes
// Create an info hash with a different value for each byte
static INFO_HASHES: Lazy<Vec<InfoHash>> = Lazy::new(|| (0..20).map(|i| InfoHash([i; 20])).collect());

#[allow(clippy::missing_panics_doc)]
pub fn update_single_torrent_benchmark(c: &mut Criterion) {
    c.bench_function("update_single_torrent_benchmark", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let tracker = Arc::new(Tracker::new(Arc::new(configuration::ephemeral()), None, statistics::Repo::new()).unwrap());

        b.to_async(&rt).iter(|| async {
            let tracker_clone = tracker.clone();

            tracker_clone
                .update_torrent_with_peer_and_get_stats(&INFO_HASHES[0], &PEER)
                .await;
        });
    });
}

#[allow(clippy::missing_panics_doc)]
pub fn update_multiple_torrents_simultaneously_benchmark(c: &mut Criterion) {
    c.bench_function("update_multiple_torrents_simultaneously_benchmark", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let tracker = Arc::new(Tracker::new(Arc::new(configuration::ephemeral()), None, statistics::Repo::new()).unwrap());

        b.to_async(&rt).iter(|| async {
            // Call the function with each info hash in parallel
            let tasks: Vec<_> = INFO_HASHES
                .iter()
                .map(|info_hash| {
                    let tracker_clone = tracker.clone();

                    tokio::spawn(async move {
                        tracker_clone.update_torrent_with_peer_and_get_stats(info_hash, &PEER).await;
                    })
                })
                .collect();

            future::join_all(tasks).await;
        });
    });
}

// Use the criterion_group macro to group the benchmarks together
criterion_group!(
    benches,
    update_single_torrent_benchmark,
    update_multiple_torrents_simultaneously_benchmark
);

// Use the criterion_main macro to run the benchmarks
criterion_main!(benches);
