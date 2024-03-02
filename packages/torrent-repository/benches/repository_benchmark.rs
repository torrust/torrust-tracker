use std::time::Duration;

mod helpers;

use criterion::{criterion_group, criterion_main, Criterion};
use torrust_tracker_torrent_repository::{
    TorrentsRwLockStd, TorrentsRwLockStdMutexStd, TorrentsRwLockStdMutexTokio, TorrentsRwLockTokio, TorrentsRwLockTokioMutexStd,
    TorrentsRwLockTokioMutexTokio,
};

use crate::helpers::{asyn, sync};

fn add_one_torrent(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).build().unwrap();

    let mut group = c.benchmark_group("add_one_torrent");

    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_millis(1000));

    group.bench_function("RwLockStd", |b| {
        b.iter_custom(sync::add_one_torrent::<TorrentsRwLockStd, _>);
    });

    group.bench_function("RwLockStdMutexStd", |b| {
        b.iter_custom(sync::add_one_torrent::<TorrentsRwLockStdMutexStd, _>);
    });

    group.bench_function("RwLockStdMutexTokio", |b| {
        b.to_async(&rt)
            .iter_custom(asyn::add_one_torrent::<TorrentsRwLockStdMutexTokio, _>);
    });

    group.bench_function("RwLockTokio", |b| {
        b.to_async(&rt).iter_custom(asyn::add_one_torrent::<TorrentsRwLockTokio, _>);
    });

    group.bench_function("RwLockTokioMutexStd", |b| {
        b.to_async(&rt)
            .iter_custom(asyn::add_one_torrent::<TorrentsRwLockTokioMutexStd, _>);
    });

    group.bench_function("RwLockTokioMutexTokio", |b| {
        b.to_async(&rt)
            .iter_custom(asyn::add_one_torrent::<TorrentsRwLockTokioMutexTokio, _>);
    });

    group.finish();
}

fn add_multiple_torrents_in_parallel(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).build().unwrap();

    let mut group = c.benchmark_group("add_multiple_torrents_in_parallel");

    //group.sampling_mode(criterion::SamplingMode::Flat);
    //group.sample_size(10);

    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_millis(1000));

    group.bench_function("RwLockStd", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| sync::add_multiple_torrents_in_parallel::<TorrentsRwLockStd, _>(&rt, iters, None));
    });

    group.bench_function("RwLockStdMutexStd", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| sync::add_multiple_torrents_in_parallel::<TorrentsRwLockStdMutexStd, _>(&rt, iters, None));
    });

    group.bench_function("RwLockStdMutexTokio", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| asyn::add_multiple_torrents_in_parallel::<TorrentsRwLockStdMutexTokio, _>(&rt, iters, None));
    });

    group.bench_function("RwLockTokio", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| asyn::add_multiple_torrents_in_parallel::<TorrentsRwLockTokio, _>(&rt, iters, None));
    });

    group.bench_function("RwLockTokioMutexStd", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| asyn::add_multiple_torrents_in_parallel::<TorrentsRwLockTokioMutexStd, _>(&rt, iters, None));
    });

    group.bench_function("RwLockTokioMutexTokio", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| asyn::add_multiple_torrents_in_parallel::<TorrentsRwLockTokioMutexTokio, _>(&rt, iters, None));
    });

    group.finish();
}

fn update_one_torrent_in_parallel(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).build().unwrap();

    let mut group = c.benchmark_group("update_one_torrent_in_parallel");

    //group.sampling_mode(criterion::SamplingMode::Flat);
    //group.sample_size(10);

    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_millis(1000));

    group.bench_function("RwLockStd", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| sync::update_one_torrent_in_parallel::<TorrentsRwLockStd, _>(&rt, iters, None));
    });

    group.bench_function("RwLockStdMutexStd", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| sync::update_one_torrent_in_parallel::<TorrentsRwLockStdMutexStd, _>(&rt, iters, None));
    });

    group.bench_function("RwLockStdMutexTokio", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| asyn::update_one_torrent_in_parallel::<TorrentsRwLockStdMutexTokio, _>(&rt, iters, None));
    });

    group.bench_function("RwLockTokio", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| asyn::update_one_torrent_in_parallel::<TorrentsRwLockTokio, _>(&rt, iters, None));
    });

    group.bench_function("RwLockTokioMutexStd", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| asyn::update_one_torrent_in_parallel::<TorrentsRwLockTokioMutexStd, _>(&rt, iters, None));
    });

    group.bench_function("RwLockTokioMutexTokio", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| asyn::update_one_torrent_in_parallel::<TorrentsRwLockTokioMutexTokio, _>(&rt, iters, None));
    });

    group.finish();
}

fn update_multiple_torrents_in_parallel(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).build().unwrap();

    let mut group = c.benchmark_group("update_multiple_torrents_in_parallel");

    //group.sampling_mode(criterion::SamplingMode::Flat);
    //group.sample_size(10);

    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_millis(1000));

    group.bench_function("RwLockStd", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| sync::update_multiple_torrents_in_parallel::<TorrentsRwLockStd, _>(&rt, iters, None));
    });

    group.bench_function("RwLockStdMutexStd", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| sync::update_multiple_torrents_in_parallel::<TorrentsRwLockStdMutexStd, _>(&rt, iters, None));
    });

    group.bench_function("RwLockStdMutexTokio", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| asyn::update_multiple_torrents_in_parallel::<TorrentsRwLockStdMutexTokio, _>(&rt, iters, None));
    });

    group.bench_function("RwLockTokio", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| asyn::update_multiple_torrents_in_parallel::<TorrentsRwLockTokio, _>(&rt, iters, None));
    });

    group.bench_function("RwLockTokioMutexStd", |b| {
        b.to_async(&rt)
            .iter_custom(|iters| asyn::update_multiple_torrents_in_parallel::<TorrentsRwLockTokioMutexStd, _>(&rt, iters, None));
    });

    group.bench_function("RwLockTokioMutexTokio", |b| {
        b.to_async(&rt).iter_custom(|iters| {
            asyn::update_multiple_torrents_in_parallel::<TorrentsRwLockTokioMutexTokio, _>(&rt, iters, None)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    add_one_torrent,
    add_multiple_torrents_in_parallel,
    update_one_torrent_in_parallel,
    update_multiple_torrents_in_parallel
);
criterion_main!(benches);
