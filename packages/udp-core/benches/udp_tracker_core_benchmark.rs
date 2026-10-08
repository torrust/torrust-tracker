mod helpers;

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

use crate::helpers::connect::ConnectBenchmarkContext;

fn bench_connect_once(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().expect("it should build a Tokio runtime");
    let context = ConnectBenchmarkContext::new();

    let mut group = c.benchmark_group("udp_tracker/connect_once");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(1));

    group.bench_function("connect_once", |b| {
        b.to_async(&runtime).iter(|| context.connect_once());
    });

    group.finish();
}

criterion_group!(benches, bench_connect_once);
criterion_main!(benches);
