---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2479-fix-http-core-announce-benchmark/ISSUE.md
last-updated-utc: "2026-10-07 13:30"
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

## Failures and Follow-up

None.
