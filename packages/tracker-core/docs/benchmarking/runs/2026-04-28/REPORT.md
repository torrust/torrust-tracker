# Benchmark Report - 2026-04-28

This is the baseline benchmark run captured after implementing:

- `docs/issues/1710-1525-03-persistence-benchmarking.md`

## Run context

- Commit: `51c27fda813876afc1cb26ea1d5bbb0fa49dfdd2`
- Ops per operation: `100`
- Benchmark runner: `cargo run -p torrust-tracker-core --bin persistence_benchmark_runner`
- Machine profile: `../../machine/2026-04-28-josecelano-desktop.txt`

## Raw artifacts

- `sqlite3.json`
- `mysql-8.4.json`
- `mysql-8.0.json`

## High-level timing summary

`meta.timings_ms.total`:

- sqlite3: `75 ms`
- mysql 8.4: `7381 ms`
- mysql 8.0: `7633 ms`

Interpretation:

- sqlite3 is much faster on this local setup.
- mysql 8.4 is slightly faster than mysql 8.0 in this run set.

## Selected operation medians (microseconds)

| Operation                       | sqlite3 | mysql 8.4 | mysql 8.0 |
| ------------------------------- | ------: | --------: | --------: |
| save_torrent_downloads          |      64 |       750 |       949 |
| load_torrent_downloads          |       9 |       114 |       133 |
| increase_downloads_for_torrent  |      50 |       759 |      1027 |
| save_global_downloads           |      58 |       745 |      1020 |
| increase_global_downloads       |      49 |       748 |      1007 |
| add_info_hash_to_whitelist      |      61 |       715 |       998 |
| remove_info_hash_from_whitelist |     116 |      1460 |      1902 |
| add_key_to_keys                 |      61 |       712 |       948 |
| remove_key_from_keys            |     116 |      1476 |      1883 |

## Machine characteristics (summary)

From `../../machine/2026-04-28-josecelano-desktop.txt`:

- Host: `josecelano-desktop`
- OS: `Ubuntu 25.10`
- Kernel: `Linux 6.17.0-22-generic`
- CPU: `AMD Ryzen 9 7950X` (16 cores / 32 threads)
- RAM: `61 GiB`
- Rust: `rustc 1.97.0-nightly (LLVM 22.1.2)`
- Cargo: `1.97.0-nightly`
- Container runtime used by benchmark: `Docker 28.3.3`

## Next comparison milestone

After implementing:

- `docs/issues/1525-05-migrate-sqlite-and-mysql-to-sqlx.md`

run the same commands, store results under a new date folder, and compare medians and totals against this baseline.
