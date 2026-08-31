---
semantic-links:
  related-artifacts:
    - packages/udp-core/benches/ban_service_benchmark.rs
    - packages/udp-core/src/services/banning.rs
    - docs/issues/open/2114-consider-removing-bloom-filter/ISSUE.md
---

# UDP Ban Service Benchmarking

## Purpose

This benchmark measures the exact-map `BanService` implementation. Its
pre-removal baseline compared the former two-level implementation with an
exact-map reference that preserves the production threshold rule: an address is
banned only when its exact error count is greater than the configured limit.

The comparison established that the Bloom filter provided no measured CPU
benefit. The retained benchmark protects the exact-map counter from future
performance regressions. It measures only counter operations; it does not
measure UDP socket I/O, Tokio lock contention, event handling, metrics, or
end-to-end tracker throughput.

## Run The Benchmark

From the repository root, run:

```sh
cargo bench -p torrust-tracker-udp-core --bench ban_service_benchmark
```

Criterion prints 95% confidence intervals and writes detailed HTML reports and
raw samples to `target/criterion/`. These build outputs are intentionally not
version-controlled.

For a before-and-after comparison, run the command on a clean checkout of each
revision on the same machine. Close unnecessary background workloads, keep the
same power and CPU-scaling policy, retain the raw Criterion output, and compare
the reported confidence intervals rather than a single run's point estimate.

## Workloads

The benchmark source is `benches/ban_service_benchmark.rs`. It uses the
following deterministic workload matrix:

- Counter limit: 10 errors.
- Repeated updates: 10,000 increments for one address per measured batch.
- Distinct updates: 10,000 unique addresses per measured batch.
- Address families: IPv4 and IPv6.
- Lookup states: below threshold (9), at threshold (10), and above threshold
  (11) errors.
- Lookup cardinalities: 10, 1,000, and 10,000 exact-map entries.

`BanService` uses `HashMap<IpAddr, u32>` with the existing strictly-greater-than
threshold rule.

## Baseline Results

This initial baseline was collected on 2026-08-29 with:

- OS: Linux 7.0.0-30-generic x86_64 GNU/Linux.
- CPU: AMD Ryzen 9 7950X 16-Core Processor, 32 logical CPUs.
- Compiler: rustc 1.98.0 (88d9e12ae 2026-08-18), LLVM 22.1.8.
- Benchmark framework: Criterion 0.5.1.

Criterion reported these 95% confidence intervals. Each increment result
covers the complete 10,000-request batch.

| Operation                   | Address family                 | Current two-level service | Exact-map reference | Relative result                 |
| --------------------------- | ------------------------------ | ------------------------- | ------------------- | ------------------------------- |
| Repeated `increase_counter` | IPv4                           | 860.38-864.08 us          | 97.553-97.758 us    | Exact map about 8.8x faster     |
| Repeated `increase_counter` | IPv6                           | 795.16-797.68 us          | 125.22-125.38 us    | Exact map about 6.4x faster     |
| Distinct `increase_counter` | IPv4                           | 1.1150-1.1185 ms          | 279.58-280.66 us    | Exact map about 4.0x faster     |
| Distinct `increase_counter` | IPv6                           | 1.1365-1.1375 ms          | 329.81-330.77 us    | Exact map about 3.4x faster     |
| `is_banned`                 | IPv4, all states/cardinalities | 73.530-87.815 ns          | 9.2101-9.3379 ns    | Exact map about 7.9-9.5x faster |
| `is_banned`                 | IPv6, all states/cardinalities | 64.640-78.360 ns          | 11.194-11.502 ns    | Exact map about 5.7-7.0x faster |

The exact-map lookup time remained effectively stable over the tested
cardinalities. The current path was slower even below and at the threshold,
where its Bloom estimate avoids the exact-map lookup.

## Pre-removal Conclusion

The benchmark provides no performance reason to retain the Bloom filter. The
exact-map reference was faster in every measured counter operation, including
the sub-threshold lookup path that the filter was intended to optimize.

The approved decision removes `bloom` and retains the exact map. The map was
already unbounded before this change because the former implementation inserted
every invalid source into it. A bounded-memory admission-control or rate-limit
design remains deferred to a future issue if operational evidence requires it.
See `../adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md`.
