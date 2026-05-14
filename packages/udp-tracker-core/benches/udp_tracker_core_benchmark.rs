mod helpers;

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

use crate::helpers::sync;

fn bench_connect_once(c: &mut Criterion) {
    let mut group = c.benchmark_group("udp_tracker/connect_once");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(1));

    group.bench_function("connect_once", |b| {
        b.iter(|| sync::connect_once(100));
    });
}

criterion_group!(benches, bench_connect_once);
criterion_main!(benches);
