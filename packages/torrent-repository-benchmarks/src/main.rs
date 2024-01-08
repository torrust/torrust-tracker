use clap::Parser;
use torrust_torrent_repository_benchmarks::args::Args;
use torrust_torrent_repository_benchmarks::benches::asyn::{
    async_add_multiple_torrents_in_parallel, async_add_one_torrent, async_update_multiple_torrents_in_parallel,
    async_update_one_torrent_in_parallel,
};
use torrust_torrent_repository_benchmarks::benches::sync::{
    add_multiple_torrents_in_parallel, add_one_torrent, update_multiple_torrents_in_parallel, update_one_torrent_in_parallel,
};
use torrust_tracker::core::torrent::repository::{AsyncSync, RepositoryAsync, RepositoryAsyncSingle, Sync, SyncSingle};

#[allow(clippy::too_many_lines)]
#[allow(clippy::print_literal)]
fn main() {
    let args = Args::parse();

    // Add 1 to worker_threads since we need a thread that awaits the benchmark
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.threads + 1)
        .enable_time()
        .build()
        .unwrap();

    println!("tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Entry>>");
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "add_one_torrent",
        rt.block_on(async_add_one_torrent::<RepositoryAsyncSingle>(1_000_000))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "update_one_torrent_in_parallel",
        rt.block_on(async_update_one_torrent_in_parallel::<RepositoryAsyncSingle>(&rt, 10))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "add_multiple_torrents_in_parallel",
        rt.block_on(async_add_multiple_torrents_in_parallel::<RepositoryAsyncSingle>(&rt, 10))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "update_multiple_torrents_in_parallel",
        rt.block_on(async_update_multiple_torrents_in_parallel::<RepositoryAsyncSingle>(&rt, 10))
    );

    if let Some(true) = args.compare {
        println!();

        println!("std::sync::RwLock<std::collections::BTreeMap<InfoHash, Entry>>");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            add_one_torrent::<SyncSingle>(1_000_000)
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(update_one_torrent_in_parallel::<SyncSingle>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(add_multiple_torrents_in_parallel::<SyncSingle>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(update_multiple_torrents_in_parallel::<SyncSingle>(&rt, 10))
        );

        println!();

        println!("std::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>>");
        println!("{}: Avg/AdjAvg: {:?}", "add_one_torrent", add_one_torrent::<Sync>(1_000_000));
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(update_one_torrent_in_parallel::<Sync>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(add_multiple_torrents_in_parallel::<Sync>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(update_multiple_torrents_in_parallel::<Sync>(&rt, 10))
        );

        println!();

        println!("tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>>");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            rt.block_on(async_add_one_torrent::<AsyncSync>(1_000_000))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(async_update_one_torrent_in_parallel::<AsyncSync>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(async_add_multiple_torrents_in_parallel::<AsyncSync>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(async_update_multiple_torrents_in_parallel::<AsyncSync>(&rt, 10))
        );

        println!();

        println!("tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<tokio::sync::Mutex<Entry>>>>");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            rt.block_on(async_add_one_torrent::<RepositoryAsync>(1_000_000))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(async_update_one_torrent_in_parallel::<RepositoryAsync>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(async_add_multiple_torrents_in_parallel::<RepositoryAsync>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(async_update_multiple_torrents_in_parallel::<RepositoryAsync>(&rt, 10))
        );
    }
}
