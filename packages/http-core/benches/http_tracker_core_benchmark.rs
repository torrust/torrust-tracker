mod helpers;

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

use crate::helpers::announce::AnnounceBenchmarkContext;
use crate::helpers::routine_guard::RoutineGuard;

const GROUP: &str = "http_tracker/announce_once";
const BENCHMARK: &str = "announce_once";

fn bench_announce_once(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().expect("it should build a Tokio runtime");
    let context = runtime.block_on(AnnounceBenchmarkContext::new());
    let guard = RoutineGuard::default();

    {
        let mut group = c.benchmark_group(GROUP);
        group.warm_up_time(Duration::from_millis(500));
        group.measurement_time(Duration::from_secs(1));

        group.bench_function(BENCHMARK, |b| {
            guard.record_benchmark_invocation();
            b.to_async(&runtime).iter(|| guard.track(context.announce_once()));
        });

        group.finish();
    }

    guard.assert_routine_ran(&format!("{GROUP}/{BENCHMARK}"));
}

criterion_group!(benches, bench_announce_once);
criterion_main!(benches);
