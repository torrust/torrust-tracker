---
semantic-links:
  skill-links:
    - write-markdown-docs
    - fix-bug
    - run-benchmarks
  related-artifacts:
    - docs/issues/open/2479-fix-http-core-announce-benchmark/ISSUE.md
    - docs/issues/open/2479-fix-http-core-announce-benchmark/manual-verification-evidence.md
    - docs/benchmarking.md
---

# Implementation Retrospective — Make the HTTP Announce Benchmark Measure the Announce

## Purpose

Record evidence-based process improvements discovered while implementing issue #2479. This is a
blameless review of the implementation approach; it does not replace acceptance-criteria
verification.

## Outcome

`http_tracker/announce_once/announce_once` now awaits one `AnnounceService::handle_announce` per
iteration (about 342-348 ns, against the 78-87 ns it took to create a future before). Both async
Criterion benchmarks, the HTTP announce and the `udp-core` `connect_once`, carry a routine guard
that fails `cargo test --benches` when the measured routine never runs and ignores benchmarks
Criterion skips. Red, green, and like-for-like recheck evidence is in
`manual-verification-evidence.md`.

## What Went Well

1. Asking the design questions before implementing surfaced a flaw in the planned guard: a
   zero-count assertion would have failed every `cargo bench -- <filter>` run that skipped the
   benchmark. Reading Criterion's `BenchmarkGroup::run_bench` confirmed that skipped benchmarks
   never invoke their closure, which gave a precise rule (D1).
2. Measuring the guard's cost on the cheapest async benchmark, rather than assuming it was
   negligible because it was negligible on the announce, caught a 10-15 ns distortion before it
   was committed for `connect_once`.
3. A control run (guard code present, timed loop byte-identical to `develop`) separated a
   code-layout shift of about 7 ns from the real cost of the wrapper. Without it, the layout shift
   would have been blamed on the guard or hidden.

## What Changed During Implementation

- The guard's routine wrapper changed from an `async fn` to `futures::FutureExt::inspect` after
  the overhead investigation (R2 section of the evidence). The `http-core` copy was already
  committed, so the change needed a separate `refactor(http-core)` commit.
- Editing the `connect_once` benchmark moves its result from about 58 ns to about 65 ns even with
  nothing added to the timed loop. This is now documented in `docs/benchmarking.md`, because
  anyone comparing new results with issue #2458's will see it.
- Clippy (`future_not_send`) rejected the `Cell` counter, so the guard uses relaxed atomics.

## Root Cause

- The specification treated the guard as "a counter and one assertion" (Risks and Trade-offs) and
  did not ask what the guard itself costs inside the timed loop. A benchmark guard is code in
  the measured path, so its overhead is a measurement question, like any other change to a
  benchmark.
- The red-run step for R2 assumed the original defect could be restored by changing one call; the
  strict `-D unused` flags also require the unused runtime binding to be restored.

## Improvements for Future Work

1. When a change adds code to a benchmark, measure the benchmark with and without it, and include a
   control that keeps the timed loop byte-identical, so a code-layout shift is not mistaken for a
   cost. `docs/benchmarking.md` now records the observed effect in "Recording Before-and-After
   Evidence".
2. Write evidence and progress-log timestamps from `date -u` output at the time of writing.
   Several entries in this issue were first written in local time (UTC+1) or ahead of the actual
   time and had to be corrected.
3. Record a prose-first test-design review in the evidence file before the commit it gates, as the
   Commit Points table asks; here the R1 review was done before the fix commit but written down
   after it.
4. The pre-commit hook lints the working tree, not only the staged files, so a documentation-only
   commit fails while uncommitted code in progress has Clippy errors. Keep the working tree
   compiling cleanly before committing evidence, or commit the evidence before starting the next
   code change.

## Avoiding Overcorrection

- No shared benchmark-helper crate is justified yet: two 60-line copies of the guard are cheaper
  than a new dependency for `udp-core` (D4). Revisit if a third async benchmark needs it.
- No automatic lint for futures that benchmarks never await; the guard covers the known benchmarks and
  `docs/benchmarking.md` asks new async benchmarks to adopt it.
- The code-layout shift does not call for a rule against editing benchmarks; it calls for
  comparing runs of the same benchmark code, which the evidence procedure already says.

## Evidence

- Specification: `ISSUE.md` (Maintainer Decisions D1-D4, Progress Log).
- `manual-verification-evidence.md`: V1, R1, R2 (including Guard Overhead Investigation), V2.
- Commits on the issue branch, by subject: `fix(http-core): [#2479] make the HTTP announce
  benchmark measure the announce`, `refactor(http-core): [#2479] track benchmark routines with a
  poll-level combinator`, `test(udp-core): [#2479] guard the connect_once benchmark against a
  routine that never runs`, `docs(benchmarking): [#2479] document the routine guard and the fixed
  announce benchmark`.
