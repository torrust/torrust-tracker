mod helpers;

use clap::Parser;
use torrust_tracker_torrent_repository::{
    TorrentsRwLockStd, TorrentsRwLockStdMutexStd, TorrentsRwLockStdMutexTokio, TorrentsRwLockTokio, TorrentsRwLockTokioMutexStd,
    TorrentsRwLockTokioMutexTokio,
};

use crate::helpers::args::Args;
use crate::helpers::{asyn, sync};

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
        rt.block_on(asyn::add_one_torrent::<TorrentsRwLockTokio, _>(1_000_000))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "update_one_torrent_in_parallel",
        rt.block_on(asyn::update_one_torrent_in_parallel::<TorrentsRwLockTokio, _>(&rt, 10))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "add_multiple_torrents_in_parallel",
        rt.block_on(asyn::add_multiple_torrents_in_parallel::<TorrentsRwLockTokio, _>(&rt, 10))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "update_multiple_torrents_in_parallel",
        rt.block_on(asyn::update_multiple_torrents_in_parallel::<TorrentsRwLockTokio, _>(&rt, 10))
    );

    if let Some(true) = args.compare {
        println!();

        println!("TorrentsRwLockStd");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            sync::add_one_torrent::<TorrentsRwLockStd, _>(1_000_000)
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(sync::update_one_torrent_in_parallel::<TorrentsRwLockStd, _>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(sync::add_multiple_torrents_in_parallel::<TorrentsRwLockStd, _>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(sync::update_multiple_torrents_in_parallel::<TorrentsRwLockStd, _>(&rt, 10))
        );

        println!();

        println!("TorrentsRwLockStdMutexStd");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            sync::add_one_torrent::<TorrentsRwLockStdMutexStd, _>(1_000_000)
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(sync::update_one_torrent_in_parallel::<TorrentsRwLockStdMutexStd, _>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(sync::add_multiple_torrents_in_parallel::<TorrentsRwLockStdMutexStd, _>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(sync::update_multiple_torrents_in_parallel::<TorrentsRwLockStdMutexStd, _>(
                &rt, 10
            ))
        );

        println!();

        println!("TorrentsRwLockStdMutexTokio");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            rt.block_on(asyn::add_one_torrent::<TorrentsRwLockStdMutexTokio, _>(1_000_000))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(asyn::update_one_torrent_in_parallel::<TorrentsRwLockStdMutexTokio, _>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(asyn::add_multiple_torrents_in_parallel::<TorrentsRwLockStdMutexTokio, _>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(asyn::update_multiple_torrents_in_parallel::<TorrentsRwLockStdMutexTokio, _>(
                &rt, 10
            ))
        );

        println!();

        println!("TorrentsRwLockTokioMutexStd");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            rt.block_on(asyn::add_one_torrent::<TorrentsRwLockTokioMutexStd, _>(1_000_000))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(asyn::update_one_torrent_in_parallel::<TorrentsRwLockTokioMutexStd, _>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(asyn::add_multiple_torrents_in_parallel::<TorrentsRwLockTokioMutexStd, _>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(asyn::update_multiple_torrents_in_parallel::<TorrentsRwLockTokioMutexStd, _>(
                &rt, 10
            ))
        );

        println!();

        println!("TorrentsRwLockTokioMutexTokio");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            rt.block_on(asyn::add_one_torrent::<TorrentsRwLockTokioMutexTokio, _>(1_000_000))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(asyn::update_one_torrent_in_parallel::<TorrentsRwLockTokioMutexTokio, _>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(asyn::add_multiple_torrents_in_parallel::<TorrentsRwLockTokioMutexTokio, _>(
                &rt, 10
            ))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(asyn::update_multiple_torrents_in_parallel::<TorrentsRwLockTokioMutexTokio, _>(&rt, 10))
        );
    }
}
