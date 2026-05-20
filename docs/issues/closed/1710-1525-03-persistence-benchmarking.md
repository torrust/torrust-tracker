---
doc-type: issue
issue-type: task
status: done
priority: p1
github-issue: 1710
spec-path: docs/issues/closed/1710-1525-03-persistence-benchmarking.md
branch: 1710-1525-03-persistence-benchmarking
related-pr: null
last-updated-utc: null
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/README.md
    - docs/issues/closed/1525-overhaul-persistence.md
    - packages/torrent-repository-benchmarking/
    - packages/tracker-core/
---

# Issue #1710 / Subissue #1525-03: Add Persistence Benchmarking

## Goal

Establish reproducible before/after persistence benchmarks so later refactors can be evaluated
against a concrete performance baseline.

## Why After Testing

Correctness comes first. Benchmarking is useful only after the core persistence behaviors are
already covered by tests, otherwise performance comparisons risk masking regressions in behavior.

## Scope

- Implement the benchmark runner as a binary inside `packages/tracker-core`, the package
  that owns the persistence layer. No Docker Compose, no image building or swapping.
- Keep the benchmark helper modules private to the binary target instead of exposing them from
  the `bittorrent-tracker-core` library API. This keeps development tooling out of the
  production module surface while still allowing `cargo run` execution from the same package.
- Benchmark every method of the `Database` trait directly, using real driver instances
  (SQLite file on disk; MySQL container via testcontainers — the same mechanism already used
  in the package's integration tests).
- Run the benchmark against SQLite and MySQL only. PostgreSQL is not available yet; the runner
  must be designed so PostgreSQL can be added in subissue #1525-08 without redesign.
- One invocation produces results for one driver/version combination. Run it three times to
  cover `sqlite3`, `mysql:8.0`, and `mysql:8.4`.
- Commit one JSON report per combination under `packages/tracker-core/docs/benchmarking/runs/`
  as the baseline. Re-run and update the reports in each subsequent subissue that changes
  persistence behavior. The git diff of those JSON files is the before/after comparison.

## Measurement Tool Rationale

**Why not Criterion?** `criterion` is a micro-benchmark framework designed for in-process
function calls. It is the right tool for the existing `torrent-repository-benchmarking` crate
(in-memory data structures). It is the wrong tool here because:

- Each operation involves a real database round-trip via an `r2d2` connection pool. The
  overhead and variance are orders of magnitude larger than what criterion's sampling model
  expects.
- The before/after comparison spans different branches (and later, different driver
  implementations), not two functions in the same process — criterion has no model for that.

**What to use instead**: `std::time::Instant` per-call timing, collected into a `Vec<Duration>`,
then sorted to extract `best`, `median`, and `worst`. No external stats crate is needed.
Output is JSON only (via `serde_json`).

## What Gets Measured

Every method on the `Database` trait, grouped by category:

| Category          | Methods                                                                                                             |
| ----------------- | ------------------------------------------------------------------------------------------------------------------- |
| Torrent metrics   | `save_torrent_downloads`, `load_torrent_downloads`, `load_all_torrents_downloads`, `increase_downloads_for_torrent` |
| Aggregate metrics | `save_global_downloads`, `load_global_downloads`, `increase_global_downloads`                                       |
| Whitelist         | `add_info_hash_to_whitelist`, `get_info_hash_from_whitelist`, `load_whitelist`, `remove_info_hash_from_whitelist`   |
| Auth keys         | `add_key_to_keys`, `get_key_from_keys`, `load_keys`, `remove_key_from_keys`                                         |

Each method is called `--ops N` times (default `100`). The collected `Vec<Duration>` is sorted
to produce `count`, `best`, `median`, and `worst` per operation.

A default of `100` matches the committed baseline reports and produces stable medians.
Pass a larger `--ops` value when tighter statistics are needed.

## What Is NOT Measured

- **Startup time** — not a persistence-layer concern; constant across persistence refactors.
- **Concurrent throughput** — the existing drivers are synchronous (`r2d2`); a single-threaded
  loop gives stable, comparable numbers. Concurrent load is relevant after the async `sqlx`
  migration (subissue #1525-05), but even then the comparison should be single-threaded first.
- **HTTP roundtrip latency** — noise relative to what is being refactored.
- **Before/after image swapping** — the benchmark runs once per branch; the committed report
  is the baseline; the git diff is the comparison.

## Proposed Branch

- `1710-add-persistence-benchmarking`

## Testing Principles

- **Real drivers**: SQLite uses a temporary file on disk; MySQL uses a testcontainers
  `GenericImage` — the same mechanism already present in the package's integration tests.
- **MySQL container lifecycle**: reuse the retry logic in
  `packages/tracker-core/src/databases/driver/mod.rs` to wait for container readiness.
- **Cleanup**: the testcontainers container is dropped (and therefore stopped) automatically
  when the `RunningMysqlContainer` goes out of scope.
- **Verified before done**: run the benchmark in a clean environment and include a copy of
  the console output in the PR description alongside the committed JSON reports.

## Tasks

### 1) Implement the benchmark runner binary inside `packages/tracker-core`

Add a new binary and binary-private support module tree to the `bittorrent-tracker-core`
package.

**Module placement rationale:**

- Do **not** expose the benchmark implementation from `packages/tracker-core/src/lib.rs`.
  Benchmark orchestration is a developer tool, not part of the production library API.
- Do **not** place this implementation under `packages/tracker-core/benches/`. In this
  repository, `benches/` is used for Criterion-style `cargo bench` targets. This persistence
  runner is different: it has a CLI, writes JSON files, selects database drivers and versions,
  and is intended to be run manually with `cargo run`.
- Therefore, keep the executable in `src/bin/` and place its helper modules under a
  binary-private directory next to it.

**New files:**

```text
packages/tracker-core/src/bin/persistence_benchmark_runner.rs   ← thin entry point (3 lines)
packages/tracker-core/src/bin/persistence_benchmark/
  mod.rs           ← module doc, re-exports
  runner.rs        ← CLI args (clap), orchestration, tracing init
  driver_bench.rs  ← driver setup, measurement loops, RawResults
  metrics.rs       ← Vec<Duration> → OperationStats (count, best, median, worst)
  report.rs        ← OperationStats → JSON (serde_json)
  types.rs         ← newtype wrappers (BenchDriver, Ops, …)
```

**Dependencies** — add only to `packages/tracker-core/Cargo.toml` (not the workspace root):

```toml
clap        = { version = "...", features = ["derive"] }
serde_json  = { version = "..." }   # already present; confirm it is not dev-only
anyhow      = { version = "..." }
tracing     = { version = "..." }   # already present
```

Run `cargo machete` after to verify no unused dependencies remain.

**CLI:**

```text
cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- \
    --driver sqlite3|mysql      # exactly one driver per run
    --db-version 8.4            # DB image tag; ignored for sqlite3; default "8.4" for mysql
    --ops 100                   # samples per operation; default 100
                                # JSON report is printed to stdout; redirect to save it
```

**Driver setup:**

- `sqlite3` — create a temporary file path; build the `r2d2_sqlite` pool; create tables.
- `mysql` — start a testcontainers `GenericImage` with the requested `--db-version` tag;
  reuse the container readiness retry logic from
  `packages/tracker-core/src/databases/driver/mod.rs`.

**Measurement loop** (per operation):

1. Prepare realistic input data (a random `InfoHash`, `AuthKey`, etc.).
2. Time each call with `std::time::Instant`.
3. Repeat `--ops` times; collect into a `Vec<Duration>`.
4. Sort and derive `count`, `best`, `median`, `worst`.

**JSON output schema:**

```json
{
  "meta": {
    "git_revision": "<sha>",
    "driver": "sqlite3",
    "db_version": "-",
    "ops": 100,
    "timestamp": "2026-04-28T12:00:00Z"
  },
  "operations": [
    {
      "name": "add_info_hash_to_whitelist",
      "count": 10,
      "best_us": 42,
      "median_us": 55,
      "worst_us": 120
    }
  ]
}
```

Acceptance criteria:

- [ ] `cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- --driver sqlite3`
      runs to completion and prints a JSON report to stdout.
- [ ] `cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- --driver mysql --db-version 8.4`
      runs to completion and prints a JSON report to stdout.
- [ ] JSON schema matches the structure above.
- [ ] `cargo machete` reports no unused dependencies.

### 2) Commit the baseline benchmark reports

Run the binary once per driver/version combination on the current branch HEAD and commit the
resulting JSON files. Each subsequent subissue reruns the same commands and commits updated
reports alongside the code change. The git diff is the before/after comparison.

```bash
cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- \
    --driver sqlite3 \
    > packages/tracker-core/docs/benchmarking/runs/$(date +%F)/sqlite3.json

cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- \
    --driver mysql --db-version 8.0 \
    > packages/tracker-core/docs/benchmarking/runs/$(date +%F)/mysql-8.0.json

cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- \
    --driver mysql --db-version 8.4 \
    > packages/tracker-core/docs/benchmarking/runs/$(date +%F)/mysql-8.4.json
```

Acceptance criteria:

- [ ] `packages/tracker-core/docs/benchmarking/runs/<date>/sqlite3.json`,
      `mysql-8.0.json`, and `mysql-8.4.json` are committed.
- [ ] Each file identifies the git revision, driver, db-version, ops count, and timestamp.

### 3) Document the workflow

- Add a section to `docs/benchmarking.md` explaining how to invoke the benchmark locally, how
  to interpret the JSON output, and how to produce an updated report after each subsequent
  subissue.
- Note that PostgreSQL support will be added in subissue #1525-08.

Acceptance criteria:

- [ ] `docs/benchmarking.md` documents the full workflow without ad hoc manual steps.

## Out of Scope

- PostgreSQL support (reserved for subissue #1525-08).
- Concurrent throughput measurement (deferred until after the async `sqlx` migration in
  subissue #1525-05).
- Startup time measurement (not a persistence-layer concern).
- HTTP-level benchmarking (noise relative to what is being refactored).
- Defining hard performance gates for CI.
- Replacing correctness-focused tests.
- The existing `torrent-repository-benchmarking` criterion micro-benchmarks (those measure
  in-memory data structures, not the full persistence stack).

## Definition of Done

- [ ] `cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- --driver sqlite3`
      runs to completion and prints a summary.
- [ ] `cargo run -p bittorrent-tracker-core --bin persistence_benchmark_runner -- --driver mysql --db-version 8.4`
      runs to completion and prints a summary.
- [ ] `packages/tracker-core/docs/benchmarking/runs/<date>/sqlite3.json`,
      `mysql-8.0.json`, and `mysql-8.4.json` are committed.
- [ ] `docs/benchmarking.md` documents the workflow.
- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `linter all` exits with code `0`.
- [ ] A passing run log is included in the PR description.

## References

- EPIC: #1525
- GitHub issue: #1710
- Existing driver test infrastructure: `packages/tracker-core/src/databases/driver/mod.rs`
- MySQL container helper: `packages/tracker-core/src/databases/driver/mysql.rs`
  (`StoppedMysqlContainer`, `RunningMysqlContainer`)
- Style reference for binary layout: `src/console/ci/qbittorrent_e2e/runner.rs`
- Benchmarking docs: `docs/benchmarking.md`
