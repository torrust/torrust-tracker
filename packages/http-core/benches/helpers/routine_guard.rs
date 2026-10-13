use std::sync::atomic::{AtomicU64, Ordering};

/// Fails a benchmark whose measured routine never ran.
///
/// Passing an `async fn` to Criterion's synchronous `b.iter` times only the
/// creation of a future that is never polled. This guard detects it: it counts
/// how often Criterion invoked the benchmark closure and how often the measured
/// routine ran to completion, and [`RoutineGuard::assert_routine_ran`] panics
/// when the closure was invoked but the routine never completed.
///
/// A benchmark that Criterion skips (a `cargo bench -- <filter>` that does not
/// match it, or `--list`) never has its closure invoked, so it does not trip
/// the guard. For the same reason the closure must call
/// [`RoutineGuard::record_benchmark_invocation`]: without it the guard can
/// never fail.
///
/// Criterion runs every benchmark once under `cargo test --benches`, so the
/// pre-push hook and CI catch a routine that does not run.
#[derive(Default)]
pub struct RoutineGuard {
    benchmark_invocations: AtomicU64,
    routine_runs: AtomicU64,
}

impl RoutineGuard {
    /// Call at the start of the closure passed to `bench_function`.
    pub fn record_benchmark_invocation(&self) {
        self.benchmark_invocations.fetch_add(1, Ordering::Relaxed);
    }

    /// Wraps the measured routine and counts it once it has run to completion.
    pub async fn track<F: Future>(&self, routine: F) -> F::Output {
        let output = routine.await;
        self.routine_runs.fetch_add(1, Ordering::Relaxed);
        output
    }

    /// # Panics
    ///
    /// Panics if Criterion invoked the benchmark but the measured routine never
    /// ran to completion.
    pub fn assert_routine_ran(&self, benchmark_id: &str) {
        let benchmark_invocations = self.benchmark_invocations.load(Ordering::Relaxed);
        let routine_runs = self.routine_runs.load(Ordering::Relaxed);

        assert!(
            benchmark_invocations == 0 || routine_runs > 0,
            "benchmark `{benchmark_id}` was invoked {benchmark_invocations} time(s) but its measured routine never ran: \
             an async routine must be awaited, for example with `b.to_async(&runtime).iter(...)`"
        );
    }
}
