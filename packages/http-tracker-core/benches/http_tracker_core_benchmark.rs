mod helpers;

use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

use crate::helpers::sync;

fn announce_once(c: &mut Criterion) {
    let _rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).build().unwrap();

    let mut group = c.benchmark_group("http_tracker_handle_announce_once");

    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_millis(1000));

    group.bench_function("handle_announce_data", |b| {
        b.iter(|| sync::return_announce_data_once(100));
    });
}

criterion_group!(benches, announce_once);
criterion_main!(benches);
