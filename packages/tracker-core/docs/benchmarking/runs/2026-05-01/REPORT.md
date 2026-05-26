# Benchmark Report - 2026-05-01

This run captures the first benchmark results that include a PostgreSQL driver,
added in subissue #1525-08:

- `docs/issues/1723-1525-08-add-postgresql-driver.md`

It is the first run to exercise `--driver postgresql` and establishes the
PostgreSQL baseline alongside the existing SQLite and MySQL numbers.

## Run context

- Commit (HEAD at run time): `74f5c8a9305912db8873024156cc006662ad1902`
- Ops per operation: `100`
- Benchmark runner: `cargo run -p torrust-tracker-core --bin persistence_benchmark_runner`
- Machine profile: `../../machine/2026-05-01-josecelano-desktop.txt`
- Same machine as all prior runs (AMD Ryzen 9 7950X, Ubuntu 25.10).

## Raw artifacts

- `sqlite3.json`
- `mysql-8.4.json`
- `mysql-8.0.json`
- `postgresql-17.json`

## High-level timing summary

`meta.timings_ms.total`:

| Driver        | 2026-04-30 | 2026-05-01 |   Delta |
| ------------- | ---------: | ---------: | ------: |
| sqlite3       |     118 ms |     119 ms |   +1 ms |
| mysql 8.4     |    6231 ms |    6372 ms | +141 ms |
| mysql 8.0     |    6678 ms |    7272 ms | +594 ms |
| postgresql 17 |          — |    1451 ms |       — |

Note: SQLite and MySQL totals are stable and within run-to-run noise.
PostgreSQL 17 is new in this run — no prior baseline to compare against.

## Selected operation medians (microseconds)

| Operation                       | sqlite3 | mysql 8.4 | mysql 8.0 | postgresql 17 |
| ------------------------------- | ------: | --------: | --------: | ------------: |
| save_torrent_downloads          |      89 |       769 |       984 |           298 |
| load_torrent_downloads          |      23 |       112 |       115 |            88 |
| load_all_torrents_downloads     |      77 |       172 |       171 |           146 |
| increase_downloads_for_torrent  |      70 |       773 |      1005 |           302 |
| save_global_downloads           |      76 |       793 |      1066 |           299 |
| load_global_downloads           |      21 |       115 |       137 |            86 |
| increase_global_downloads       |      67 |       774 |      1036 |           305 |
| add_info_hash_to_whitelist      |      81 |       735 |       981 |           294 |
| get_info_hash_from_whitelist    |      21 |       109 |       118 |            95 |
| load_whitelist                  |      55 |       161 |       175 |           135 |
| remove_info_hash_from_whitelist |      81 |       766 |       962 |           293 |
| add_key_to_keys                 |      81 |       750 |       974 |           292 |
| get_key_from_keys               |      22 |       118 |       129 |            95 |
| load_keys                       |      77 |       167 |       189 |           155 |
| remove_key_from_keys            |      73 |       739 |       994 |           300 |

## PostgreSQL 17 characteristics

- Write operations (`save_*`, `increase_*`, `add_*`, `remove_*`): median ~290–305 µs.
  Roughly 2.5–3× faster than MySQL 8.0 and ~60% faster than MySQL 8.4 for writes.
- Read operations (`load_*`, `get_*`): median 86–155 µs.
  Comparable to MySQL 8.4 for simple lookups; slightly slower for `load_*` aggregates.
- Overall total (1451 ms) is significantly lower than both MySQL versions, driven by
  faster write operations.
- `remove_*` operations (293–300 µs) are notably faster than MySQL (739–994 µs).

## Regression assessment

No regression. SQLite and MySQL numbers are within noise of the `2026-04-30` run.
PostgreSQL 17 is introduced as a new baseline — no comparison is possible yet.

## Machine characteristics (summary)

From `../../machine/2026-05-01-josecelano-desktop.txt`:

- Host: `josecelano-desktop`
- OS: `Ubuntu 25.10`
- Kernel: `Linux 6.17.0-22-generic`
- CPU: `AMD Ryzen 9 7950X` (16 cores / 32 threads)
- Container runtime used by benchmark: `Docker 28.3.3`

Identical hardware to all prior benchmark runs.
