---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2479-fix-http-core-announce-benchmark/ISSUE.md
last-updated-utc: "2026-10-13 18:06"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

`.tmp/` and other git-ignored paths are not part of the repository, so a path
there is not evidence once the run ends. Record everything a reviewer needs in
this issue folder:

- Embed small artifacts, such as a tracker configuration file, verbatim in
  this document.
- Store larger artifacts, such as full logs or captured responses, in an
  `evidence/` subfolder of the issue folder and link them from here.
- Record runtime outputs (database rows, responses, relevant log lines) inline
  in each process.

A local `.tmp/` path may still be mentioned to show where a run wrote its
files, but never as the only copy of something the evidence relies on.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-07 13:24-13:27.
- Artifact under test: the benchmark code of `torrust-tracker-http-core` as on `develop`
  `c7cb2b3fa`. The runs used a checkout of the issue #2458 branch rebased onto that commit; that
  branch does not change `packages/http-core/`.
- Operating system / environment: Linux x86_64, AMD Ryzen 9 7950X (32 logical CPUs), a shared
  desktop (load average 9.63 at the start).
- Toolchain: stable Rust 1.99.0 (`b940084d7`), Cargo 1.99.0; `bench` profile for `cargo bench`
  and `test` profile for `cargo test`.

## Verification Processes

### V1 - M1: The Benchmark Measures Nothing

- Goal: show that the routine Criterion times never runs the announce code.
- Status: `DONE` — **Reproduced**.

#### Steps Performed

1. Ran the unmodified benchmark.
2. Added a temporary counter to the helper and a print after the benchmark (diff below), ran the
   benchmark again, and restored both files from backup copies. `git status --short` was empty
   afterwards.
3. Ran the unmodified benchmark in Criterion's test mode, as the pre-push hook and CI do.

Temporary probe, verbatim (`git diff`):

```diff
diff --git a/packages/http-core/benches/helpers/sync.rs b/packages/http-core/benches/helpers/sync.rs
--- a/packages/http-core/benches/helpers/sync.rs
+++ b/packages/http-core/benches/helpers/sync.rs
@@ -1,6 +1,9 @@
 use std::net::{IpAddr, Ipv4Addr, SocketAddr};
+use std::sync::atomic::{AtomicU64, Ordering};
 use std::time::{Duration, Instant};
 
+pub static BODY_RUNS: AtomicU64 = AtomicU64::new(0);
+
 use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
 use torrust_tracker_http_core::services::announce::AnnounceService;
 
@@ -8,6 +11,7 @@ use crate::helpers::util::{initialize_core_tracker_services, sample_announce_req
 
 #[must_use]
 pub async fn return_announce_data_once(samples: u64) -> Duration {
+    BODY_RUNS.fetch_add(1, Ordering::Relaxed);
     let (core_tracker_services, core_http_tracker_services) = initialize_core_tracker_services().await;
 
     let peer = sample_peer();
diff --git a/packages/http-core/benches/http_tracker_core_benchmark.rs b/packages/http-core/benches/http_tracker_core_benchmark.rs
--- a/packages/http-core/benches/http_tracker_core_benchmark.rs
+++ b/packages/http-core/benches/http_tracker_core_benchmark.rs
@@ -17,6 +17,12 @@ fn announce_once(c: &mut Criterion) {
     group.bench_function("handle_announce_data", |b| {
         b.iter(|| sync::return_announce_data_once(100));
     });
+
+    eprintln!(
+        "PROBE: function bodies run = {}, future size = {} bytes",
+        sync::BODY_RUNS.load(std::sync::atomic::Ordering::Relaxed),
+        std::mem::size_of_val(&sync::return_announce_data_once(100))
+    );
 }
 
 criterion_group!(benches, announce_once);
```

#### Observed Result

```text
$ cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark
Benchmarking http_tracker_handle_announce_once/handle_announce_data: Collecting 100 samples in estimated 1.0002 s (12M iterations)
http_tracker_handle_announce_once/handle_announce_data
                        time:   [85.250 ns 85.524 ns 85.942 ns]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe

$ # with the temporary probe
$ cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark
Benchmarking http_tracker_handle_announce_once/handle_announce_data: Collecting 100 samples in estimated 1.0003 s (12M iterations)
                        time:   [86.776 ns 87.118 ns 87.565 ns]
PROBE: function bodies run = 0, future size = 1160 bytes

$ cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark
Testing http_tracker_handle_announce_once/handle_announce_data
Success
```

#### Conclusion

**Reproduced.** Over about 12 million timed iterations, plus warm-up, the helper's body ran zero
times. The reported 85-87 ns is the cost of creating and dropping a 1160-byte future that is never
polled. Criterion's test mode reports `Success` for the same routine, so no gate notices; it does,
however, execute each benchmark once, which is what makes the planned in-benchmark guard
practical.

## Regression Guard Evidence

Toolchain for every run in this section: stable Rust 1.99.0 (`b940084d7`), Cargo 1.99.0, on the
same Linux x86_64 AMD Ryzen 9 7950X desktop as V1. The branch is
`2479-fix-http-core-announce-benchmark`, cut from `develop` `48e11787f`; `git log` on that commit
shows no change to `packages/http-core/benches` since V1, so V1 still describes the code under
test.

### R1 - Red Before the Fix (T1)

- Date and time (UTC): 2026-10-13 17:35.
- Boundary: the guard inside the benchmark target, as the spec's Regression Test Strategy selects.
  The defect is in how the benchmark calls Criterion, so only the benchmark target can observe it.
- Change under test: the new guard helper `packages/http-core/benches/helpers/routine_guard.rs`
  wired into the **unfixed** benchmark. The red run used an earlier draft of the helper with the
  same counting and assertion logic; the committed helper counts with `AtomicU64` instead of
  `Cell<u64>` (Clippy `future_not_send`) and has two more documentation lines, so its panic line
  number differs. The routine is still passed to
  the synchronous `b.iter`; only the guard calls were added:

```rust
fn announce_once(c: &mut Criterion) {
    let _rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).build().unwrap();
    let guard = RoutineGuard::default();

    let mut group = c.benchmark_group("http_tracker_handle_announce_once");

    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(1));

    group.bench_function("handle_announce_data", |b| {
        guard.record_benchmark_invocation();
        b.iter(|| guard.track(sync::return_announce_data_once(100)));
    });

    group.finish();

    guard.assert_routine_ran("http_tracker_handle_announce_once/handle_announce_data");
}
```

Red run, in Criterion's test mode as the pre-push hook and CI run it:

```text
$ cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark
     Running benches/http_tracker_core_benchmark.rs (target/debug/deps/http_tracker_core_benchmark-6057a23ea1d2d43b)
Gnuplot not found, using plotters backend
Testing http_tracker_handle_announce_once/handle_announce_data
Success


thread 'main' (2087018) panicked at packages/http-core/benches/helpers/routine_guard.rs:41:9:
benchmark `http_tracker_handle_announce_once/handle_announce_data` was invoked 1 time(s) but its measured routine never ran: an async routine must be awaited, for example with `b.to_async(&runtime).iter(...)`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `-p torrust-tracker-http-core --bench http_tracker_core_benchmark`
```

Criterion still prints `Success` for the routine itself, as in V1; the guard's panic after
`bench_function` is what fails the run.

Maintainer decision D1 (the guard must not fail a benchmark Criterion skips) checked on the same
red code: a filter that matches nothing and `--list` both exit 0, because Criterion never invokes
the benchmark closure for them (Criterion 0.5.1 `BenchmarkGroup::run_bench`, which only calls the
closure when `filter_matches` is true and the mode is not `List`):

```text
$ cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark -- no_such_benchmark
     Running benches/http_tracker_core_benchmark.rs (target/debug/deps/http_tracker_core_benchmark-6057a23ea1d2d43b)
Gnuplot not found, using plotters backend
exit=0

$ cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark -- --list
Gnuplot not found, using plotters backend
http_tracker_handle_announce_once/handle_announce_data: benchmark

exit=0
```

The red code was not committed alone, because the pre-push hook would reject it.

### R1 - Fix, Green Run, and Design Review (T2)

- Date and time (UTC): 2026-10-13 17:36-17:45.
- Code state: commit `fix(http-core): [#2479] make the HTTP announce benchmark measure the
  announce`. `AnnounceBenchmarkContext` (`packages/http-core/benches/helpers/announce.rs`) builds
  the services and the `AnnounceService` once with `runtime.block_on`, and the benchmark awaits
  one announce per iteration with `b.to_async(&runtime).iter(|| guard.track(context.announce_once()))`.
  `helpers/sync.rs` is removed.

Green run:

```text
$ cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark
Testing http_tracker/announce_once/announce_once
Success
```

Mutate-then-restore on the fixed code: replacing `b.to_async(&runtime).iter(...)` with
`b.iter(...)` (the original defect, keeping the guard) is red again; restoring the file is green:

```text
$ cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark   # mutated
Testing http_tracker/announce_once/announce_once
Success
thread 'main' (2108138) panicked at packages/http-core/benches/helpers/routine_guard.rs:43:9:
benchmark `http_tracker/announce_once/announce_once` was invoked 1 time(s) but its measured routine never ran: an async routine must be awaited, for example with `b.to_async(&runtime).iter(...)`

$ cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark   # restored
Testing http_tracker/announce_once/announce_once
Success
```

Before and after `cargo bench`, same machine, `bench` profile, load average about 3.5-3.9:

```text
$ # before: develop 48e11787f, unmodified (stashed working tree), 17:36 UTC
$ cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark
http_tracker_handle_announce_once/handle_announce_data
                        time:   [78.429 ns 78.453 ns 78.479 ns]
Found 10 outliers among 100 measurements (10.00%)

$ # after: the fix, 17:41 UTC
$ cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark
Benchmarking http_tracker/announce_once/announce_once: Collecting 100 samples in estimated 1.0011 s (2.9M iterations)
http_tracker/announce_once/announce_once
                        time:   [347.51 ns 347.87 ns 348.28 ns]
Found 1 outliers among 100 measurements (1.00%)
```

The "before" number is the creation of a future (V1). The "after" number is about 350 ns for one
in-memory repeat announce (D3): authentication and whitelist checks on a public tracker, client
IP resolution, the swarm update, and sending one HTTP statistics event. Criterion now fits 2.9M
iterations in the window instead of 12M. An intermediate draft that counted with `Cell<u64>`
measured 341-367 ns, so the guard's relaxed atomic increment has no measurable cost here.

Prose-first Arrange-Act-Assert design review of R1, done on the benchmark function before the fix
commit and written down here afterwards:

- Temporary prose:
  - **Arrange:** a Tokio runtime and an announce context are built once, outside the measured
    routine; a new guard has counted no benchmark invocation and no routine run.
  - **Act:** Criterion runs the benchmark. Each invocation of the benchmark closure is recorded,
    and each measured iteration awaits one announce through the guard, which counts it when it
    completes.
  - **Assert:** if Criterion invoked the benchmark, at least one announce completed; otherwise the
    run fails, naming the benchmark.
- Comparison and refactors:
  - The benchmark ID was written twice (group and function names, then a string literal in the
    assertion), so a rename could leave the failure message naming a stale benchmark (hidden
    fixture coupling). Refactored to `GROUP` and `BENCHMARK` constants, with the assertion
    formatting `{GROUP}/{BENCHMARK}`.
  - Forgetting `record_benchmark_invocation()` would make the guard unable to fail. This is the
    price of D1; it is documented on `RoutineGuard` rather than hidden behind a wrapper around
    `bench_function`, which would also hide the Criterion call the reviewer needs to see.
  - The measured Act stays visible as `b.to_async(&runtime).iter(|| guard.track(context.announce_once()))`;
    the context owns only incidental mechanics (service wiring, the fixed request, the server
    binding). The causal condition, whether the routine ran, is the guard's two counters.
  - The failure message was read without opening the code: it names the benchmark, the
    invocation count, the fact that the routine never ran, and the usual fix.
  - No `// Arrange` / `// Act` / `// Assert` comments were kept: this is a benchmark function, and
    the three steps are already separated by blocks and blank lines. The temporary prose was
    removed.
- Clippy follow-ups made before the commit: `Cell<u64>` became `AtomicU64` (`future_not_send`),
  the benchmark group is scoped in a block before the assertion (`significant_drop_tightening`),
  and the context moves the services instead of cloning them (`redundant_clone`).

### R2 - Guard on `udp-core` `connect_once` (T3)

- Date and time (UTC): 2026-10-13 17:43-17:53.
- Change: `packages/udp-core/benches/helpers/routine_guard.rs`, a copy of the `http-core` helper
  (D4; the two files are identical), wired into `udp_tracker_core_benchmark.rs` the same way as R1,
  with `GROUP` and `BENCHMARK` constants.
- Code state: commits `refactor(http-core): [#2479] track benchmark routines with a poll-level
  combinator` and `test(udp-core): [#2479] guard the connect_once benchmark against a routine
  that never runs`.

#### Guard Overhead Investigation

The first version of the guard wrapped the routine in an `async fn track`. On `connect_once`,
which takes about 60 ns, that version measured 75-80 ns. A guard that changes the time it guards
by up to a third is not acceptable, so the overhead was investigated before going further. Every
row is a separate `cargo bench -p torrust-tracker-udp-core --bench udp_tracker_core_benchmark`
run (Criterion's middle estimate; load average 2.7-4.0); every variant was reverted after its run.

| Variant of the `connect_once` benchmark                                                              | Time (ns)          |
| ---------------------------------------------------------------------------------------------------- | ------------------ |
| No guard (`develop`, stashed working tree)                                                           | 58.8, 58.0, 58.7   |
| `async fn track` wrapper with `fetch_add` (first version)                                            | 79.7, 75.3         |
| `async fn track` wrapper, counter removed                                                            | 69.9               |
| `async fn track` wrapper with a relaxed `store`                                                      | 72.6               |
| No wrapper; the context stores a relaxed `AtomicBool` after its own `.await`                         | 68.3, 65.1         |
| `iter_custom` loop, runs recorded once per batch outside the timed loop                              | 64.8, 64.8         |
| **Control:** guard code present, measured routine byte-identical to `develop` (no `track` in it)     | 65.7, 64.8         |
| `track` as `futures::FutureExt::inspect` (poll-level combinator, as committed)                       | 66.7, 65.0         |

Reading:

- The control has no instrumentation in the timed loop, yet it reads about 65 ns instead of 58 ns.
  Adding any code to the benchmark binary moves the result by about 7 ns, which points to code
  layout or inlining, not to work done per iteration. Even the `iter_custom` variant, which does
  nothing extra per iteration, lands there.
- The `inspect` combinator is indistinguishable from the control, so its per-iteration cost is not
  measurable here. The `async fn` wrapper costs another 10-15 ns on top of the control, so `track`
  was changed to `inspect` in both copies of the helper, with a doc comment saying why.
- Consequence for readers comparing with older results: `connect_once` now reads about 65 ns
  against the about 58 ns recorded by issue #2458 on similar hardware. The difference comes from
  the benchmark binary changing, not from `ConnectService`.
- On the HTTP announce (about 350 ns) neither wrapper made a measurable difference: 352 ns with
  the `Cell` draft, 348 ns with `fetch_add` in an `async fn`, 342 ns with `inspect`.

#### Red and Green

Mutate-then-restore. The mutation restores the defect exactly as it existed before issue #2458
fixed it: the runtime bound but unused (`let _runtime`, needed because `-D unused` rejects an
unused `runtime`) and the routine passed to the synchronous `b.iter`, with the guard kept. A first
attempt that changed only the `iter` call did not compile (`unused variable: runtime`), so it was
not a valid red run.

```diff
-    let runtime = tokio::runtime::Runtime::new().expect("it should build a Tokio runtime");
+    let _runtime = tokio::runtime::Runtime::new().expect("it should build a Tokio runtime");
...
-            b.to_async(&runtime).iter(|| guard.track(context.connect_once()));
+            b.iter(|| guard.track(context.connect_once()));
```

```text
$ cargo test -p torrust-tracker-udp-core --bench udp_tracker_core_benchmark   # mutated
Testing udp_tracker/connect_once/connect_once
Success
thread 'main' (2167244) panicked at packages/udp-core/benches/helpers/routine_guard.rs:52:9:
benchmark `udp_tracker/connect_once/connect_once` was invoked 1 time(s) but its measured routine never ran: an async routine must be awaited, for example with `b.to_async(&runtime).iter(...)`
error: test failed, to rerun pass `-p torrust-tracker-udp-core --bench udp_tracker_core_benchmark`

$ cargo test -p torrust-tracker-udp-core --bench udp_tracker_core_benchmark   # restored
Testing udp_tracker/connect_once/connect_once
Success
```

Because `track` changed after the R1 commit, R1 was mutated again with the `inspect` version
(`b.to_async(&runtime).iter(...)` replaced by `b.iter(...)`), and was still red:

```text
$ cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark   # mutated
Testing http_tracker/announce_once/announce_once
Success
thread 'main' (2168495) panicked at packages/http-core/benches/helpers/routine_guard.rs:52:9:
benchmark `http_tracker/announce_once/announce_once` was invoked 1 time(s) but its measured routine never ran: an async routine must be awaited, for example with `b.to_async(&runtime).iter(...)`
error: test failed, to rerun pass `-p torrust-tracker-http-core --bench http_tracker_core_benchmark`
```

After restoring both files, `cargo test -p torrust-tracker-http-core -p torrust-tracker-udp-core
--benches` reports `Success` for every benchmark in both packages, and `linter clippy` exits 0.

#### Prose-First Arrange-Act-Assert Design Review of R2

- Temporary prose:
  - **Arrange:** a Tokio runtime and a connect context are built once, outside the measured
    routine; a new guard has counted no benchmark invocation and no routine run.
  - **Act:** Criterion runs the benchmark. Each invocation of the benchmark closure is recorded,
    and each measured iteration awaits one connect through the guard, which counts it when it
    completes.
  - **Assert:** if Criterion invoked the benchmark, at least one connect completed; otherwise the
    run fails, naming the benchmark.
- Comparison: the code already expressed the prose, because it uses the shape R1 settled on
  (`GROUP` and `BENCHMARK` constants, the group scoped in a block, the visible
  `b.to_async(&runtime).iter(|| guard.track(context.connect_once()))`, the assertion after the
  block). No further refactor; the temporary prose was removed.
- The guard helper is duplicated rather than shared (D4). Both copies are byte-identical today;
  a change to one must be made to the other.

### V2 - M2: The Benchmark Measures the Announce

- Goal: repeat V1 like-for-like on the fixed benchmark and show that the routine Criterion times
  runs the announce on every iteration.
- Status: `DONE`.
- Date and time (UTC): 2026-10-13 17:59-18:01.
- Code state: commit `docs(issues): [#2479] record the guard green runs, overhead investigation,
  and reviews` (benchmark code as in `refactor(http-core): [#2479] track benchmark routines with a
  poll-level combinator`). Same machine and toolchain as V1. The load average was 7.2-9.3 because
  other processes were running, so these runs show how many times the routine ran, not a reliable
  time.

#### Steps Performed

1. Applied the same kind of temporary probe as V1: a counter incremented on the first line of the
   routine body (now `AnnounceBenchmarkContext::announce_once`) and a print after the benchmark,
   including the size of the routine's future.
2. Ran the benchmark with `cargo bench` twice; the second run was rerun only to capture Criterion's
   iteration-count line, which the first command's output filter dropped.
3. Ran it once in Criterion's test mode, as the pre-push hook and CI do.
4. Reverted the probe with `git checkout --` on both files; `git status --short` was empty, and
   `cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark` reported `Success`
   again without the probe line.

Temporary probe, verbatim (`git diff`):

```diff
diff --git a/packages/http-core/benches/helpers/announce.rs b/packages/http-core/benches/helpers/announce.rs
index 7faaa2f51..490343b00 100644
--- a/packages/http-core/benches/helpers/announce.rs
+++ b/packages/http-core/benches/helpers/announce.rs
@@ -1,4 +1,7 @@
 use std::net::{IpAddr, Ipv4Addr, SocketAddr};
+use std::sync::atomic::{AtomicU64, Ordering};
+
+pub static BODY_RUNS: AtomicU64 = AtomicU64::new(0);
 
 use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
 use torrust_tracker_http_core::services::announce::AnnounceService;
@@ -65,6 +68,7 @@ impl AnnounceBenchmarkContext {
     /// Panics if the announce is rejected, which would mean the benchmark
     /// measures an error path instead of an announce.
     pub async fn announce_once(&self) -> AnnounceData {
+        BODY_RUNS.fetch_add(1, Ordering::Relaxed);
         self.announce_service
             .handle_announce(
                 &self.announce_request,
diff --git a/packages/http-core/benches/http_tracker_core_benchmark.rs b/packages/http-core/benches/http_tracker_core_benchmark.rs
index 18ed4f85b..28fe3d31d 100644
--- a/packages/http-core/benches/http_tracker_core_benchmark.rs
+++ b/packages/http-core/benches/http_tracker_core_benchmark.rs
@@ -29,6 +29,12 @@ fn bench_announce_once(c: &mut Criterion) {
     }
 
     guard.assert_routine_ran(&format!("{GROUP}/{BENCHMARK}"));
+
+    eprintln!(
+        "PROBE: function bodies run = {}, future size = {} bytes",
+        crate::helpers::announce::BODY_RUNS.load(std::sync::atomic::Ordering::Relaxed),
+        std::mem::size_of_val(&context.announce_once())
+    );
 }
 
 criterion_group!(benches, bench_announce_once);
```

#### Observed Result

```text
$ cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark   # first run
http_tracker/announce_once/announce_once
                        time:   [363.20 ns 367.34 ns 372.69 ns]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe

PROBE: function bodies run = 4531251, future size = 736 bytes

$ cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark
Testing http_tracker/announce_once/announce_once
Success
PROBE: function bodies run = 1, future size = 736 bytes

$ cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark   # second run
Benchmarking http_tracker/announce_once/announce_once: Warming up for 500.00 ms
Benchmarking http_tracker/announce_once/announce_once: Collecting 100 samples in estimated 1.0012 s (2.7M iterations)
http_tracker/announce_once/announce_once
                        time:   [468.03 ns 498.09 ns 529.71 ns]
Found 3 outliers among 100 measurements (3.00%)
PROBE: function bodies run = 4819101, future size = 736 bytes
```

#### Conclusion

**Fixed.** V1 counted 0 body runs over about 12 million timed iterations. Now the routine body
ran 4,819,101 times in a run where Criterion timed 2.7 million iterations; the rest are warm-up
iterations, which Criterion also runs through the routine. In test mode it ran exactly once,
which is the single execution the guard relies on. The measured time is now in the hundreds of
nanoseconds, consistent with an in-memory announce (R1 section: about 342-348 ns on a quieter
machine), not the 78-87 ns it took to create a future.

## Failures and Follow-up

- R2's first mutation changed only the Criterion call and did not compile, because `-D unused`
  rejects the then-unused `runtime`. It was not counted as a red run; the recorded mutation also
  binds the runtime as `_runtime`, as the code did before issue #2458.
- The first version of the guard distorted `connect_once`; see Guard Overhead Investigation.
- Follow-up candidate, not opened: a first-announce benchmark with a bounded swarm (D3). It is
  left to the maintainer to decide whether it is worth an issue.
