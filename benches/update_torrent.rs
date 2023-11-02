use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use criterion::{criterion_group, criterion_main, Criterion};
use futures::stream::FuturesUnordered;
use futures::{future, StreamExt};
use once_cell::sync::Lazy;
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash;
use torrust_tracker::shared::clock::DurationSinceUnixEpoch;
use torrust_tracker::tracker::peer::{Id, Peer};
use torrust_tracker::tracker::{statistics, Tracker};
use torrust_tracker_test_helpers::configuration;

use crate::future::FutureExt;

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
static INFO_HASHES: Lazy<Vec<InfoHash>> = Lazy::new(|| (0..100).map(|i| InfoHash([i; 20])).collect());

#[allow(clippy::missing_panics_doc)]
pub fn add_a_single_torrent_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("add_a_single_torrent_benchmark", |b| {
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
pub fn add_and_update_a_single_torrent_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("add_and_update_a_single_torrent_benchmark", |b| {
        let tracker = Arc::new(Tracker::new(Arc::new(configuration::ephemeral()), None, statistics::Repo::new()).unwrap());

        b.to_async(&rt).iter(|| async {
            const NUM_UPDATES: usize = 20;
            let mut futures = FuturesUnordered::new();

            for _ in 0..NUM_UPDATES {
                let tracker_clone = tracker.clone();

                let future = async move {
                    tracker_clone
                        .update_torrent_with_peer_and_get_stats(&INFO_HASHES[0], &PEER)
                        .await;
                };
                futures.push(future.boxed());
            }

            while let Some(_) = futures.next().await {}
        });
    });
}

#[allow(clippy::missing_panics_doc)]
pub fn add_multiple_torrents_simultaneously_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("add_multiple_torrents_simultaneously_benchmark", |b| {
        let tracker = Arc::new(Tracker::new(Arc::new(configuration::ephemeral()), None, statistics::Repo::new()).unwrap());

        b.to_async(&rt).iter(|| async {
            let peer = PEER.clone();
            let mut futures = FuturesUnordered::new();

            for info_hash in INFO_HASHES.iter() {
                let tracker_clone = tracker.clone();
                let peer_clone = peer.clone();

                let future = async move {
                    tracker_clone
                        .update_torrent_with_peer_and_get_stats(&info_hash, &peer_clone)
                        .await;
                };
                futures.push(future.boxed());
            }

            while let Some(_) = futures.next().await {}
        });
    });
}

#[allow(clippy::missing_panics_doc)]
pub fn add_and_update_multiple_torrents_simultaneously_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("add_and_update_multiple_torrents_simultaneously_benchmark", |b| {
        let tracker = Arc::new(Tracker::new(Arc::new(configuration::ephemeral()), None, statistics::Repo::new()).unwrap());

        b.to_async(&rt).iter(|| async {
            const NUM_UPDATES: usize = 20;
            let peer = PEER.clone();
            let mut futures = FuturesUnordered::new();

            for info_hash in INFO_HASHES.iter() {
                for _ in 0..NUM_UPDATES {
                    let tracker_clone = tracker.clone();
                    let peer_clone = peer.clone();

                    let future = async move {
                        tracker_clone
                            .update_torrent_with_peer_and_get_stats(&info_hash, &peer_clone)
                            .await;
                    };
                    futures.push(future.boxed());
                }
            }

            while let Some(_) = futures.next().await {}
        });
    });
}

// Use the criterion_group macro to group the benchmarks together
criterion_group!(
    benches,
    add_a_single_torrent_benchmark,
    add_and_update_a_single_torrent_benchmark,
    add_multiple_torrents_simultaneously_benchmark,
    add_and_update_multiple_torrents_simultaneously_benchmark
);

// Use the criterion_main macro to run the benchmarks
criterion_main!(benches);
