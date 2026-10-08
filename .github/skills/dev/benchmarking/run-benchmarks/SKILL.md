---
name: run-benchmarks
description: Choose, run, and record Torrust Tracker benchmarks, including before-and-after performance evidence for changes to performance-critical code. Covers the benchmark levels (Criterion microbenchmarks per package, the persistence benchmark, the aquatic UDP load test, comparative and workflow benchmarks), how to pick one, how to measure a baseline before a change, and how to check that a benchmark measures anything. Use when a change touches the UDP or HTTP request path or other hot code, when an issue needs performance evidence, or when asked to benchmark. Triggers on "benchmark", "run benchmarks", "performance regression", "measure performance", "before and after performance", "load test", "aquatic", or "cargo bench".
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - docs/benchmarking.md
    - docs/profiling.md
    - docs/issues/open/2458-inject-udp-cookie-cipher/performance-evidence.md
    - contrib/dev-tools/benches/run-benches.sh
    - share/default/config/tracker.udp.benchmarking.toml
metadata:
  author: torrust
  version: "1.0"
---

# Running Benchmarks

[`docs/benchmarking.md`](../../../../../docs/benchmarking.md) is the source of truth: it lists every
benchmark with its command, says what each one measures, and gives the criteria for choosing one.
This skill is the workflow for using it.

## When a Change Needs Performance Evidence

Plan a measurement in the issue specification when the change touches code on the request path
(connection IDs, announce, scrape, the receive loop), the torrent repository, persistence, or any
code the maintainer calls performance-critical. Reasoning that the change is cheap is not enough;
measure it.

## Workflow

1. **Choose the instruments.** Use the
   [Choosing a Benchmark](../../../../../docs/benchmarking.md#choosing-a-benchmark) criteria. For
   the request path, use a Criterion microbenchmark of the changed code plus the end-to-end UDP load
   test.
2. **Check that each instrument measures something.** Compare its result with a known cost. Watch
   for an `async fn` passed to `b.iter` without being awaited; see
   [Checking That a Benchmark Measures Something](../../../../../docs/benchmarking.md#checking-that-a-benchmark-measures-something).
   If the changed code has no benchmark, add one in its own commit before any production change.
3. **Write the pass rule in the specification:** one-sided, against the baseline (after mean not
   below the lowest baseline run; after median not above the highest baseline median).
4. **Measure the baseline** before any production change, following
   [Recording Before-and-After Evidence](../../../../../docs/benchmarking.md#recording-before-and-after-evidence).
   Run `uptime` first; on a shared machine, wait for other builds to finish.
5. **Implement the change**, then **measure again** on the same machine with the same settings.
6. **Record both in issue-local `performance-evidence.md`**: machine, toolchain, commands, every
   run, the noise, and the comparison against the pass rule. Take times from output files or
   `git log`, not from memory. Investigate a failure, for example with
   [profiling](../../../../../docs/profiling.md), before closing the issue.

## Quick Commands

```sh
# Criterion microbenchmarks of one package (all benches, or one with --bench <name>)
cargo bench -p torrust-tracker-udp-core
cargo bench -p torrust-tracker-udp-core --bench connection_cookie_benchmark

# Release build for the end-to-end UDP load test (see docs/benchmarking.md for aquatic)
cargo build --release --bin torrust-tracker
./target/release/torrust-tracker --config-toml-path ./share/default/config/tracker.udp.benchmarking.toml
```

## Pitfalls

- Criterion's `time:` line is a mean estimate; read the median from
  `target/criterion/<group>/<function>/new/estimates.json`. The group's `/` becomes `_`, and each
  run overwrites the file, so read it after every run.
- Verbose logging cuts UDP throughput by about ten times; the benchmarking configuration uses
  `trace_filter = "error"`.
- The load test varies by about ±5–10% between runs on a desktop. It rules out only large
  regressions; rely on the microbenchmark for small ones.
- A benchmark is not a correctness test. Keep the regression tests the change needs.

## Skill Links

Review this skill when changing:

- `docs/benchmarking.md` — benchmark catalog, selection criteria, and evidence procedure.
- `contrib/dev-tools/benches/run-benches.sh` — local script that runs the package benchmarks.
- `share/default/config/tracker.udp.benchmarking.toml` — the load-test tracker configuration.
