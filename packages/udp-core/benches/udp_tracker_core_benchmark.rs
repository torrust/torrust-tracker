mod helpers;

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

use crate::helpers::connect::ConnectBenchmarkContext;
use crate::helpers::routine_guard::RoutineGuard;

const GROUP: &str = "udp_tracker/connect_once";
const BENCHMARK: &str = "connect_once";

fn bench_connect_once(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().expect("it should build a Tokio runtime");
    let context = ConnectBenchmarkContext::new();
    let guard = RoutineGuard::default();

    {
        let mut group = c.benchmark_group(GROUP);
        group.warm_up_time(Duration::from_millis(500));
        group.measurement_time(Duration::from_secs(1));

        group.bench_function(BENCHMARK, |b| {
            guard.record_benchmark_invocation();
            b.to_async(&runtime).iter(|| guard.track(context.connect_once()));
        });

        group.finish();
    }

    guard.assert_routine_ran(&format!("{GROUP}/{BENCHMARK}"));
}

criterion_group!(benches, bench_connect_once);
criterion_main!(benches);
