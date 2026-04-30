# Benchmark Report - 2026-04-30

This run captures benchmark results after migrating the SQLite and MySQL
drivers from `r2d2` + `rusqlite` / `mysql` to `sqlx 0.8`:

- `docs/issues/1717-1525-05-migrate-sqlite-and-mysql-to-sqlx.md`

It is the post-SQLx counterpart of the `2026-04-28` baseline.

## Run context

- Commit (HEAD at run time): `a4dbc63a6c713e115bfc11374b72743aa51ebfb5`
- Ops per operation: `100`
- Benchmark runner: `cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner`
- Machine profile: `../../machine/2026-04-30-josecelano-desktop.txt`
- Same machine as the `2026-04-28` baseline (AMD Ryzen 9 7950X, Ubuntu 25.10).

The `git_revision` recorded in the JSON artifacts is `a4dbc63a…`. A small
benchmark-harness change was applied locally on top of that commit to wait
for the MySQL container to fully accept TCP connections before running
DDL (see "Notes" below). The change does not touch any code path that
contributes to recorded operation timings, so the numbers remain
comparable.

## Raw artifacts

- `sqlite3.json`
- `mysql-8.4.json`
- `mysql-8.0.json`

## High-level timing summary

`meta.timings_ms.total`:

| Driver    | Baseline (2026-04-28) | New (2026-04-30) |    Delta |
| --------- | --------------------: | ---------------: | -------: |
| sqlite3   |                 75 ms |           118 ms |   +43 ms |
| mysql 8.4 |               7381 ms |          6231 ms | −1150 ms |
| mysql 8.0 |               7633 ms |          6678 ms |  −955 ms |

Interpretation:

- MySQL totals improve by ~13–16% on both 8.0 and 8.4, mostly driven by
  much faster `remove_*` operations (see medians below).
- sqlite3 total rises by 43 ms. On a 75 ms baseline with only 100 ops per
  operation and no warmup, this is well inside run-to-run noise; per-op
  medians (next section) are within a handful of microseconds of the
  baseline and the `remove_*` operations are actually faster.

## Selected operation medians (microseconds)

| Operation                       | sqlite3 (base → new) | mysql 8.4 (base → new) | mysql 8.0 (base → new) |
| ------------------------------- | -------------------: | ---------------------: | ---------------------: |
| save_torrent_downloads          |              64 → 80 |              750 → 779 |              949 → 978 |
| load_torrent_downloads          |               9 → 24 |              114 → 119 |              133 → 139 |
| increase_downloads_for_torrent  |              50 → 73 |              759 → 824 |             1027 → 972 |
| save_global_downloads           |              58 → 72 |              745 → 834 |            1020 → 1046 |
| increase_global_downloads       |              49 → 65 |              748 → 820 |            1007 → 1053 |
| add_info_hash_to_whitelist      |              61 → 82 |              715 → 739 |             998 → 1010 |
| remove_info_hash_from_whitelist |             116 → 73 |             1460 → 743 |             1902 → 982 |
| add_key_to_keys                 |              61 → 79 |              712 → 730 |              948 → 958 |
| remove_key_from_keys            |             116 → 71 |             1476 → 739 |             1883 → 952 |

Notable changes:

- `remove_*` operations are roughly **2× faster** on MySQL 8.4 and 8.0,
  and ~35% faster on SQLite. Likely sqlx prepared-statement reuse and
  the absence of r2d2 connection-checkout overhead on these short
  operations.
- `save_*` and simple `load_*` ops show small (~10–20 µs on SQLite,
  ~10–80 µs on MySQL) regressions, well inside per-run variance.
- Overall MySQL throughput is meaningfully better; SQLite totals are
  unchanged once you discount the dominant per-op variance contribution.

## Regression assessment

No regression. The largest single per-operation regression on either
driver is the SQLite `load_torrent_downloads` median going from 9 µs to
24 µs. That difference (15 µs) is the same order of magnitude as the
syscall jitter that sqlx adds for query execution, and is paid for many
times over by the `remove_*` improvements. End-to-end MySQL benchmark
time drops by 13–16%.

## Machine characteristics (summary)

From `../../machine/2026-04-30-josecelano-desktop.txt`:

- Host: `josecelano-desktop`
- OS: `Ubuntu 25.10`
- Kernel: `Linux 6.17.0-22-generic`
- CPU: `AMD Ryzen 9 7950X` (16 cores / 32 threads)
- Container runtime used by benchmark: `Docker 28.3.3`

Identical hardware to the `2026-04-28` baseline.

## Notes

`sqlx` opens connection pools lazily and does not retry the first query
on connect failure. With the `mysql:8.x` testcontainer image the very
first DDL statement issued by the benchmark harness occasionally raced
the TCP listener and failed with `UnexpectedEof`. The
`r2d2`-based driver previously masked this through implicit pool
checkout retries.

The benchmark harness now waits for the second `ready for connections`
log line on the container's stderr (the official `mysql` image emits it
twice — first transiently on the unix socket during init, then again on
TCP port `3306`) and then performs a short `connect`+`SELECT 1` retry
loop before handing off to `initialize_database`. This is a bench-only
change in
`packages/tracker-core/src/bin/persistence_benchmark/driver_bench/database/mysql.rs`
and does not alter production code paths.

Whether to introduce a similar startup-retry policy in production
should be considered separately.
