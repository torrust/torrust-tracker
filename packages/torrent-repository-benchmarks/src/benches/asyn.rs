use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use futures::stream::FuturesUnordered;
use torrust_tracker::core::torrent::repository::TRepositoryAsync;
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash;

use crate::args::Args;
use crate::benches::utils::{generate_unique_info_hashes, get_average_and_adjusted_average_from_results, DEFAULT_PEER};

pub async fn async_add_one_torrent<T: TRepositoryAsync + Send + Sync + 'static>(samples: usize) -> (Duration, Duration) {
    let mut results: Vec<Duration> = Vec::with_capacity(samples);

    for _ in 0..samples {
        let torrent_repository = Arc::new(T::new());

        let info_hash = InfoHash([0; 20]);

        let start_time = std::time::Instant::now();

        torrent_repository
            .update_torrent_with_peer_and_get_stats(&info_hash, &DEFAULT_PEER)
            .await;

        let result = start_time.elapsed();

        results.push(result);
    }

    get_average_and_adjusted_average_from_results(results)
}

// Add one torrent ten thousand times in parallel (depending on the set worker threads)
pub async fn async_update_one_torrent_in_parallel<T: TRepositoryAsync + Send + Sync + 'static>(
    runtime: &tokio::runtime::Runtime,
    samples: usize,
) -> (Duration, Duration) {
    let args = Args::parse();
    let mut results: Vec<Duration> = Vec::with_capacity(samples);

    for _ in 0..samples {
        let torrent_repository = Arc::new(T::new());
        let info_hash: &'static InfoHash = &InfoHash([0; 20]);
        let handles = FuturesUnordered::new();

        // Add the torrent/peer to the torrent repository
        torrent_repository
            .update_torrent_with_peer_and_get_stats(info_hash, &DEFAULT_PEER)
            .await;

        let start_time = std::time::Instant::now();

        for _ in 0..10_000 {
            let torrent_repository_clone = torrent_repository.clone();

            let handle = runtime.spawn(async move {
                torrent_repository_clone
                    .update_torrent_with_peer_and_get_stats(info_hash, &DEFAULT_PEER)
                    .await;

                if let Some(sleep_time) = args.sleep {
                    let start_time = std::time::Instant::now();

                    while start_time.elapsed().as_nanos() < u128::from(sleep_time) {}
                }
            });

            handles.push(handle);
        }

        // Await all tasks
        futures::future::join_all(handles).await;

        let result = start_time.elapsed();

        results.push(result);
    }

    get_average_and_adjusted_average_from_results(results)
}

// Add ten thousand torrents in parallel (depending on the set worker threads)
pub async fn async_add_multiple_torrents_in_parallel<T: TRepositoryAsync + Send + Sync + 'static>(
    runtime: &tokio::runtime::Runtime,
    samples: usize,
) -> (Duration, Duration) {
    let args = Args::parse();
    let mut results: Vec<Duration> = Vec::with_capacity(samples);

    for _ in 0..samples {
        let torrent_repository = Arc::new(T::new());
        let info_hashes = generate_unique_info_hashes(10_000);
        let handles = FuturesUnordered::new();

        let start_time = std::time::Instant::now();

        for info_hash in info_hashes {
            let torrent_repository_clone = torrent_repository.clone();

            let handle = runtime.spawn(async move {
                torrent_repository_clone
                    .update_torrent_with_peer_and_get_stats(&info_hash, &DEFAULT_PEER)
                    .await;

                if let Some(sleep_time) = args.sleep {
                    let start_time = std::time::Instant::now();

                    while start_time.elapsed().as_nanos() < u128::from(sleep_time) {}
                }
            });

            handles.push(handle);
        }

        // Await all tasks
        futures::future::join_all(handles).await;

        let result = start_time.elapsed();

        results.push(result);
    }

    get_average_and_adjusted_average_from_results(results)
}

// Async update ten thousand torrents in parallel (depending on the set worker threads)
pub async fn async_update_multiple_torrents_in_parallel<T: TRepositoryAsync + Send + Sync + 'static>(
    runtime: &tokio::runtime::Runtime,
    samples: usize,
) -> (Duration, Duration) {
    let args = Args::parse();
    let mut results: Vec<Duration> = Vec::with_capacity(samples);

    for _ in 0..samples {
        let torrent_repository = Arc::new(T::new());
        let info_hashes = generate_unique_info_hashes(10_000);
        let handles = FuturesUnordered::new();

        // Add the torrents/peers to the torrent repository
        for info_hash in &info_hashes {
            torrent_repository
                .update_torrent_with_peer_and_get_stats(info_hash, &DEFAULT_PEER)
                .await;
        }

        let start_time = std::time::Instant::now();

        for info_hash in info_hashes {
            let torrent_repository_clone = torrent_repository.clone();

            let handle = runtime.spawn(async move {
                torrent_repository_clone
                    .update_torrent_with_peer_and_get_stats(&info_hash, &DEFAULT_PEER)
                    .await;

                if let Some(sleep_time) = args.sleep {
                    let start_time = std::time::Instant::now();

                    while start_time.elapsed().as_nanos() < u128::from(sleep_time) {}
                }
            });

            handles.push(handle);
        }

        // Await all tasks
        futures::future::join_all(handles).await;

        let result = start_time.elapsed();

        results.push(result);
    }

    get_average_and_adjusted_average_from_results(results)
}
