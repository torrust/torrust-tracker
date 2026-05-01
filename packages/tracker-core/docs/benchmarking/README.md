# Persistence Benchmarking Reports

This folder stores benchmark artifacts produced by
`persistence_benchmark_runner` for `bittorrent-tracker-core`.

Goals:

- Keep reproducible baseline reports in-repo.
- Track benchmark evolution across major persistence changes.
- Enable before/after comparisons (for example, before and after SQLx migration).

## Layout

- `machine/`: machine and toolchain characteristics for each run date.
- `runs/<date>/`: raw JSON benchmark output files and a run summary.

## Baseline run (pre-SQLx)

- Date: `2026-04-28`
- Commit: `51c27fda813876afc1cb26ea1d5bbb0fa49dfdd2`
- Issue context: `docs/issues/1710-1525-03-persistence-benchmarking.md`
- Run summary: `runs/2026-04-28/REPORT.md`
- Machine profile: `machine/2026-04-28-josecelano-desktop.txt`

Raw JSON artifacts:

- `runs/2026-04-28/sqlite3.json`
- `runs/2026-04-28/mysql-8.4.json`
- `runs/2026-04-28/mysql-8.0.json`

## Post-SQLx run (SQLite and MySQL only)

- Date: `2026-04-30`
- Commit (HEAD at run time): `a4dbc63a6c713e115bfc11374b72743aa51ebfb5`
- Issue context: `docs/issues/1717-1525-05-migrate-sqlite-and-mysql-to-sqlx.md`
- Run summary (with comparison vs `2026-04-28`): `runs/2026-04-30/REPORT.md`
- Machine profile: `machine/2026-04-30-josecelano-desktop.txt`

Raw JSON artifacts:

- `runs/2026-04-30/sqlite3.json`
- `runs/2026-04-30/mysql-8.4.json`
- `runs/2026-04-30/mysql-8.0.json`

## PostgreSQL baseline run

- Date: `2026-05-01`
- Commit (HEAD at run time): `74f5c8a9305912db8873024156cc006662ad1902`
- Issue context: `docs/issues/1723-1525-08-add-postgresql-driver.md`
- Run summary (first run with PostgreSQL): `runs/2026-05-01/REPORT.md`
- Machine profile: `machine/2026-05-01-josecelano-desktop.txt`

Raw JSON artifacts:

- `runs/2026-05-01/sqlite3.json`
- `runs/2026-05-01/mysql-8.4.json`
- `runs/2026-05-01/mysql-8.0.json`
- `runs/2026-05-01/postgresql-17.json`

## How to add a new run

1. Create a new run folder:

   `mkdir -p packages/tracker-core/docs/benchmarking/runs/YYYY-MM-DD`

2. Run benchmarks and save JSON artifacts:

   `cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- --driver sqlite3 > packages/tracker-core/docs/benchmarking/runs/YYYY-MM-DD/sqlite3.json`

   `cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- --driver mysql --db-version 8.4 > packages/tracker-core/docs/benchmarking/runs/YYYY-MM-DD/mysql-8.4.json`

   `cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- --driver postgresql --db-version 17 > packages/tracker-core/docs/benchmarking/runs/YYYY-MM-DD/postgresql-17.json`

3. Capture machine profile:

   `mkdir -p packages/tracker-core/docs/benchmarking/machine`

   Save at least OS, kernel, CPU, RAM, Rust toolchain and container runtime versions to:

   `packages/tracker-core/docs/benchmarking/machine/YYYY-MM-DD-<host>.txt`

4. Add `runs/YYYY-MM-DD/REPORT.md` with:
   - benchmark context (commit, command, ops)
   - high-level summary (total benchmark time)
   - important per-operation medians
   - comparison versus a prior run when relevant

5. Update this index file with links to the new run and machine profile.

## Planned comparison point

After implementing `docs/issues/1717-1525-05-migrate-sqlite-and-mysql-to-sqlx.md`, the
benchmark was re-run at `runs/2026-04-30` to compare against the `2026-04-28` baseline.

After adding the PostgreSQL driver (`docs/issues/1723-1525-08-add-postgresql-driver.md`),
the benchmark was run again at `runs/2026-05-01` to establish the PostgreSQL baseline.

The next planned comparison point is after any major persistence refactor that touches all
drivers (e.g., schema migrations or async `sqlx` pool changes).
