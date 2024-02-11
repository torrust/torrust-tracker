use std::sync::Arc;

use clap::Parser;
use torrust_torrent_repository_benchmarks::args::Args;
use torrust_torrent_repository_benchmarks::benches::{asyn, sync};
use torrust_tracker::core::torrent::{
    TorrentsRwLockStd, TorrentsRwLockStdMutexStd, TorrentsRwLockStdMutexTokio, TorrentsRwLockTokio, TorrentsRwLockTokioMutexStd,
    TorrentsRwLockTokioMutexTokio,
};

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

    println!("TorrentsRwLockTokio");
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "add_one_torrent",
        rt.block_on(asyn::add_one_torrent::<Arc<TorrentsRwLockTokio>>(1_000_000))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "update_one_torrent_in_parallel",
        rt.block_on(asyn::update_one_torrent_in_parallel::<Arc<TorrentsRwLockTokio>>(&rt, 10))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "add_multiple_torrents_in_parallel",
        rt.block_on(asyn::add_multiple_torrents_in_parallel::<Arc<TorrentsRwLockTokio>>(&rt, 10))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "update_multiple_torrents_in_parallel",
        rt.block_on(asyn::update_multiple_torrents_in_parallel::<Arc<TorrentsRwLockTokio>>(
            &rt, 10
        ))
    );

    if let Some(true) = args.compare {
        println!();

        println!("TorrentsRwLockStd");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            sync::add_one_torrent::<Arc<TorrentsRwLockStd>>(1_000_000)
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(sync::update_one_torrent_in_parallel::<Arc<TorrentsRwLockStd>>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(sync::add_multiple_torrents_in_parallel::<Arc<TorrentsRwLockStd>>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(sync::update_multiple_torrents_in_parallel::<Arc<TorrentsRwLockStd>>(&rt, 10))
        );

        println!();

        println!("TorrentsRwLockStdMutexStd");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            sync::add_one_torrent::<Arc<TorrentsRwLockStdMutexStd>>(1_000_000)
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(sync::update_one_torrent_in_parallel::<Arc<TorrentsRwLockStdMutexStd>>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(sync::add_multiple_torrents_in_parallel::<Arc<TorrentsRwLockStdMutexStd>>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(sync::update_multiple_torrents_in_parallel::<Arc<TorrentsRwLockStdMutexStd>>(
                &rt, 10
            ))
        );

        println!();

        println!("TorrentsRwLockStdMutexTokio");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            rt.block_on(asyn::add_one_torrent::<Arc<TorrentsRwLockStdMutexTokio>>(1_000_000))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(asyn::update_one_torrent_in_parallel::<Arc<TorrentsRwLockStdMutexTokio>>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(asyn::add_multiple_torrents_in_parallel::<Arc<TorrentsRwLockStdMutexTokio>>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(asyn::update_multiple_torrents_in_parallel::<Arc<TorrentsRwLockStdMutexTokio>>(&rt, 10))
        );

        println!();

        println!("TorrentsRwLockTokioMutexStd");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            rt.block_on(asyn::add_one_torrent::<Arc<TorrentsRwLockTokioMutexStd>>(1_000_000))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(asyn::update_one_torrent_in_parallel::<Arc<TorrentsRwLockTokioMutexStd>>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(asyn::add_multiple_torrents_in_parallel::<Arc<TorrentsRwLockTokioMutexStd>>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(asyn::update_multiple_torrents_in_parallel::<Arc<TorrentsRwLockTokioMutexStd>>(&rt, 10))
        );

        println!();

        println!("TorrentsRwLockTokioMutexTokio");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            rt.block_on(asyn::add_one_torrent::<Arc<TorrentsRwLockTokioMutexTokio>>(1_000_000))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(asyn::update_one_torrent_in_parallel::<Arc<TorrentsRwLockTokioMutexTokio>>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(asyn::add_multiple_torrents_in_parallel::<Arc<TorrentsRwLockTokioMutexTokio>>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(asyn::update_multiple_torrents_in_parallel::<Arc<TorrentsRwLockTokioMutexTokio>>(&rt, 10))
        );
    }
}
