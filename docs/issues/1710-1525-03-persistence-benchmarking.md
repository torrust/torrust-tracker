# Subissue Draft for #1525-03: Add Persistence Benchmarking

## Goal

Establish reproducible before/after persistence benchmarks so later refactors can be evaluated
against a concrete performance baseline.

## Why After Testing

Correctness comes first. Benchmarking is useful only after the core persistence behaviors are
already covered by tests, otherwise performance comparisons risk masking regressions in behavior.

## Scope

- Implement the benchmark runner in Rust (a new binary, consistent with the `e2e_tests_runner`
  pattern), following the same docker compose approach used in subissue #1525-02.
- Use one docker compose file per database backend. Each compose file defines the database
  container and the tracker container together. The runner launches the compose stack,
  discovers the ports, runs the workloads, and tears down. No manual `docker run` calls.
- Run the benchmark against SQLite and MySQL only. PostgreSQL is not available yet; the runner
  must be designed so PostgreSQL can be added in subissue #1525-08 without redesign.
- The benchmark compares two tracker Docker images: a `bench-before` image and a `bench-after`
  image. The tracker image tag is passed to compose via an environment variable so the runner
  can swap it per variant. This allows the same compose files and runner to be re-used after
  each subsequent subissue.
- On the first run (this subissue), before and after use the same image built from the current
  `develop` HEAD, giving an identical-baseline comparison. The committed report records this.
- Commit the first benchmark report into `docs/benchmarks/` as a baseline reference. Re-run
  and update the report in each subsequent subissue that changes persistence behavior.

## Measurement Tool Rationale

**Why not Criterion?** `criterion` is a micro-benchmark framework: it runs the same in-process
function thousands of times in a tight loop, applies warm-up phases, and performs statistical
outlier detection for nanosecond-to-millisecond measurements. It is the right tool for the
existing `torrent-repository-benchmarking` crate (in-memory data structures). It is the wrong
tool here because:

- Each operation involves a real HTTP round-trip to a containerized tracker talking to a real
  database. The overhead dwarfs what criterion's sampling model expects.
- We need _aggregate_ metrics across N concurrent workers (ops/sec, p95 latency), not per-call
  statistics from a single thread.
- The before/after comparison is across two different Docker images, not across two functions
  in the same process — criterion has no model for that.

**What to use instead**: `std::time::Instant` per-call timing, collected into a `Vec<Duration>`,
then sorted for percentile extraction. This is exactly what the Python reference script does.
For concurrency, spawn N OS threads via `std::thread::spawn` (one per worker up to
`--concurrency`), each running blocking `reqwest` calls in a loop. Join all threads and
collect their `Duration` measurements into a shared `Vec` for percentile computation. Do
not use `rayon` — its work-stealing pool is designed for CPU-bound tasks and will stall
under I/O-bound HTTP workloads. Output is written as JSON (via `serde_json`) and Markdown.

## Reference Workflow

The PR #1695 review branch includes a Python reference:

- `contrib/dev-tools/qa/run-before-after-db-benchmark.py`

That script defines the full benchmark approach: it starts a real tracker binary, starts
database containers with free ports, sends HTTP workloads concurrently, collects latency
percentiles and throughput, and prints a before/after comparison. The Rust implementation
must replicate this approach.

### What the Python script measures

- **Startup time** — how long the tracker takes to reach `200 OK` on the health endpoint,
  measured for both an empty database and a populated database (after the workloads have run).
- **Workloads** (each run sequentially and concurrently):
  - `announce_lifecycle` — HTTP `started` announce followed by `completed` announce for each
    unique infohash
  - `whitelist_add` — REST API `POST /api/v1/whitelist/{info_hash}`
  - `whitelist_reload` — REST API `GET /api/v1/whitelist/reload`
  - `auth_key_add` — REST API `POST /api/v1/keys`
  - `auth_key_reload` — REST API `GET /api/v1/keys/reload`
- **Metrics per workload**: count, total time, ops/sec, mean latency, median latency, p95
  latency, min/max latency.
- **Comparison output**: startup speedup (after/before), ops/s speedup, p95 latency improvement
  ratio for each workload × driver combination.

## Proposed Branch

- `1525-03-persistence-benchmarking`

## Testing Principles

- **Isolation**: Each run uses a unique compose project name (e.g.
  `torrust-bench-<driver>-<variant>-<random>`) so container names, networks, and volumes
  never collide with a parallel invocation. This mirrors the isolation strategy in
  subissue #1525-02.
- **Independent system resources**: Do not bind to fixed host ports. Discover the ports
  assigned by compose using `docker compose port`. Place all temporary files (SQLite database
  file, tracker config, logs) in a `tempfile`-managed directory that is removed on exit.
- **Cleanup**: Use a `RunningCompose` `Drop` guard (from the `DockerCompose` wrapper in
  subissue #1525-02) to call `docker compose down --volumes` unconditionally on success,
  failure, and panic.
- **Verified before done**: Run the benchmark in a clean environment and include the output in
  the PR description alongside the committed report.

## Tasks

### 1) Add docker compose files for each database backend

Add one compose file per database under `contrib/dev-tools/bench/`:

- `compose.bench-sqlite3.yaml` — tracker service + a volume for the SQLite database file.
- `compose.bench-mysql.yaml` — tracker service + MySQL service.

Design notes:

- Parameterize the tracker image tag with an env var (e.g.
  `TORRUST_TRACKER_BENCH_IMAGE`, defaulting to `torrust-tracker:bench`) so the runner can
  swap before/after images without editing the file.
- Set `TORRUST_TRACKER_CONFIG_TOML` via the compose `environment` key so the runner can inject
  a generated config without mounting a file.
- Do not expose fixed host ports in the compose files; expose only the container ports and let
  Docker assign ephemeral host ports. The runner discovers them with `docker compose port`.
- Ensure `healthcheck` is defined for each service so `docker compose up --wait` blocks until
  everything is ready.

Acceptance criteria:

- [ ] `docker compose -f compose.bench-sqlite3.yaml up --wait` starts successfully.
- [ ] `docker compose -f compose.bench-mysql.yaml up --wait` starts successfully.
- [ ] `docker compose -f <file> down --volumes` leaves no orphaned resources.

### 2) Implement the Rust benchmark runner binary

Add a new binary `src/bin/persistence_benchmark_runner.rs` following the `e2e_tests_runner`
pattern. Reuse the `DockerCompose` wrapper introduced in subissue #1525-02 at
`src/console/ci/compose.rs`.

**Dependencies** — add to the workspace `Cargo.toml` and the binary's crate:

```toml
reqwest = { version = "...", features = ["blocking"] }
serde_json = { version = "..." }
```

`rayon` is not needed (see the concurrent workloads approach below). Run `cargo machete`
after to verify no unused dependencies remain.

**Architecture** — add a module `src/console/ci/bench/` containing:

- `runner.rs` — main orchestration and CLI argument parsing
- `workloads.rs` — HTTP client calls for each workload (announce, whitelist, auth key)
- `metrics.rs` — `Instant`-based latency collection, sorting, percentile and throughput
  computation (no external stats crate needed)
- `report.rs` — JSON (`serde_json`) and Markdown formatting

**CLI arguments** (mirroring the Python script):

- `--before-image <tag>` — tracker Docker image for the "before" variant
  (default: `torrust-tracker:bench`)
- `--after-image <tag>` — tracker Docker image for the "after" variant
  (default: same as `--before-image`)
- `--dbs <sqlite3|mysql>` — space/comma-separated list of drivers (default: `sqlite3 mysql`)
- `--mysql-version <tag>` — MySQL Docker image tag (default `8.4`)
- `--ops <n>` — number of operations per workload (default `200`)
- `--reload-iterations <n>` — iterations for reload workloads (default `30`)
- `--concurrency <n>` — worker threads for concurrent workloads (default `16`)
- `--json-output <path>` — write machine-readable JSON to this path
- `--report-output <path>` — write the human-readable Markdown report to this path

**Per-suite lifecycle** (one suite = one `(driver, variant)` pair):

1. Select the compose file for the driver.
2. Build or tag the tracker image as `TORRUST_TRACKER_BENCH_IMAGE` for this variant.
3. Create a unique compose project name.
4. `DockerCompose::up()` — blocks until all services are healthy.
5. Discover the tracker HTTP, REST API, and health check host ports via
   `DockerCompose::port()`.
6. Record `startup_empty_ms` (time from `up` call to first successful health check response).
7. Run a warm-up iteration.
8. Run each workload sequentially then concurrently; collect per-operation `Duration` values.
9. Restart the tracker service only (or call `down` then `up` again) to measure
   `startup_populated_ms` against the now-populated database.
10. `DockerCompose::down()` — unconditional, via `Drop` guard.

**HTTP client**: use `reqwest` (blocking feature) for workload calls.

**Concurrent workloads**: spawn `--concurrency` OS threads via `std::thread::spawn`, each
running blocking `reqwest` calls in a loop; collect per-thread `Duration` measurements into
a shared `Vec` (via `Arc<Mutex<Vec<Duration>>>` or join handles). Do not use `rayon` —
its work-stealing pool blocks under I/O-bound workloads.

Acceptance criteria:

- [ ] The binary runs successfully against SQLite and MySQL.
- [ ] Startup times (empty and populated) are recorded for each driver.
- [ ] All five workload families are measured sequentially and concurrently.
- [ ] JSON output schema matches the Python reference (`results`, `comparisons` keys).
- [ ] Human-readable Markdown report is produced.
- [ ] All compose stacks are cleaned up unconditionally via `Drop` guards.
- [ ] No hard-coded host ports; all ports are discovered via `docker compose port`.

### 3) Commit the baseline benchmark report

After the binary is working:

- Build a Docker image from the current `develop` HEAD:
  `docker build -t torrust-tracker:bench .`
- Run the benchmark with `--before-image torrust-tracker:bench` and
  `--after-image torrust-tracker:bench` (both pointing to the same freshly built image,
  producing an identical-baseline comparison).
- Save the JSON output to `docs/benchmarks/baseline.json`.
- Save the Markdown report to `docs/benchmarks/baseline.md`.
- Commit both files as part of this subissue's PR.

Acceptance criteria:

- [ ] `docs/benchmarks/baseline.json` and `docs/benchmarks/baseline.md` are committed.
- [ ] The Markdown report is readable without tooling and identifies the git revision used.

### 4) Document the workflow

Steps:

- Document how to invoke the benchmark locally.
- Document how to produce an updated report after each subsequent subissue.
- Note that PostgreSQL support will be added to the benchmark in subissue #1525-08.

Acceptance criteria:

- [ ] The benchmark is documented and runnable without ad hoc manual steps.

## Out of Scope

- PostgreSQL support (reserved for subissue #1525-08).
- Defining hard performance gates for CI.
- Replacing correctness-focused tests.
- The existing `torrent-repository-benchmarking` criterion micro-benchmarks (those measure
  in-memory data structures, not the full persistence stack).

## Definition of Done

- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `linter all` exits with code `0`.
- [ ] The benchmark has been executed successfully; `docs/benchmarks/baseline.md` and
      `docs/benchmarks/baseline.json` are committed.
- [ ] A passing run log is included in the PR description.

## References

- EPIC: #1525
- Reference PR: #1695
- Reference implementation branch: `josecelano:pr-1684-review` — see EPIC for checkout
  instructions (`docs/issues/1525-overhaul-persistence.md`)
- Reference script: `contrib/dev-tools/qa/run-before-after-db-benchmark.py`
- Docker compose wrapper: `src/console/ci/e2e/docker.rs` (pattern reused for compose wrapper)
- Subissue #1525-02 compose wrapper: `src/console/ci/compose.rs`
