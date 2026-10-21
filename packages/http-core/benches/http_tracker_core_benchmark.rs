mod helpers;

use std::time::{Duration, Instant};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use crate::helpers::announce::AnnounceBenchmarkContext;
use crate::helpers::first_announce::FirstAnnounceBenchmarkContext;
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

const FIRST_ANNOUNCE_GROUP: &str = "http_tracker/first_announce";

/// Numbers of other peers in the swarm: none (the insert alone), the default maximum peers per
/// announce (a full peer list), and a swarm much larger than the list.
const SWARM_SIZES: [usize; 3] = [0, 74, 1000];

/// Times one `started` announce of a peer that is not in the swarm, for each swarm size.
///
/// The peer leaves again with a `stopped` announce outside the measured time, so the swarm has
/// exactly the same size at every iteration. Criterion cannot do this: its `iter_batched` setup is
/// synchronous, so the departure is awaited here and only the first announce is timed.
///
/// The measured time includes the two clock reads around each announce, a constant that is the
/// same in every run. The results are therefore comparable with each other, not with
/// `announce_once`, whose iterations are timed in bulk.
fn bench_first_announce(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().expect("it should build a Tokio runtime");

    let mut group = c.benchmark_group(FIRST_ANNOUNCE_GROUP);
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(1));

    for swarm_size in SWARM_SIZES {
        let context = runtime.block_on(FirstAnnounceBenchmarkContext::new(swarm_size));
        let guard = RoutineGuard::default();

        group.bench_function(BenchmarkId::from_parameter(swarm_size), |b| {
            guard.record_benchmark_invocation();

            let context = &context;
            let guard = &guard;

            b.to_async(&runtime).iter_custom(|iterations| async move {
                let mut measured = Duration::ZERO;

                for _ in 0..iterations {
                    let started_at = Instant::now();
                    guard.track(context.first_announce()).await;
                    measured += started_at.elapsed();

                    context.depart().await;
                }

                measured
            });
        });

        let benchmark_id = format!("{FIRST_ANNOUNCE_GROUP}/{swarm_size}");
        guard.assert_routine_ran(&benchmark_id);
        runtime.block_on(context.assert_swarm_unchanged_by_run(&benchmark_id));
    }

    group.finish();
}

criterion_group!(benches, bench_announce_once, bench_first_announce);
criterion_main!(benches);
