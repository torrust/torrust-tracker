---
doc-type: issue-inventory
issue-spec: docs/issues/closed/2394-2392-audit-shared-test-resources/ISSUE.md
last-updated-utc: "2026-11-17 15:22"
---

# Inventory of Shared Test Resources (tasks T1 to T5)

Snapshot of `torrust/develop` at `b009b6425` (2026-11-16, after PR #2883). It covers the Rust
tests, test fixtures and test helpers of every workspace member, the two E2E runners in
`src/console/ci/`, the three `compose.qbittorrent-e2e.*.yaml` files, the container test script in
`contrib/dev-tools/containers/tests/`, the git hooks that run the test suite, and the CI workflow
steps that run tests. Line numbers are those of this snapshot. After the first review round the
searches were rerun after merging `develop` at `0373f05f8` into the branch; the only new test resource is
`tests/http_health_check.rs` (P5).

## Where Tests Run

The answer to "who else can use this resource" depends on where a test runs:

| Runner | What runs there | Concurrency on one host |
| --- | --- | --- |
| Development machine | `cargo test` / `cargo nextest run`, the pre-push hook (`cargo test --workspace`), the E2E runners | Several agents or developers in several checkouts at once |
| Self-hosted runner (`torrust-hetzner`) | `container.yaml` job `test` and `testing.yaml` job `docker-e2e`: image build (its `Containerfile` `test` stage runs the whole Rust suite with `cargo nextest run`, line 231), E2E runner, qBittorrent E2E runner, container test script | One runner instance per server today (#2386); several later. Each `RUN` step of a BuildKit build has its own network namespace, because neither the `Containerfile` nor the workflows ask for `--network=host` |
| GitHub-hosted runner (`ubuntu-latest`) | Every other job, including all `cargo test` jobs on the host | One job per virtual machine (out of scope) |

`cargo test` runs the tests of one test binary as threads of one process; `cargo nextest run` runs
each test in its own process. A process-global finding is only real under `cargo test`.

## Dispositions

- **Isolated**: already safe; the reason is recorded.
- **Deliberate exception**: the test contract needs the fixed resource and the reason is documented.
- **Covered**: an existing subissue owns it.
- **New problem**: needs a new subissue (drafted in T6).

Summary, after the maintainer's answers to Q1 to Q4 (see [Maintainer Decisions](#maintainer-decisions)):

| Class | Isolated | Deliberate exception | Covered | New problem |
| --- | --- | --- | --- | --- |
| Network ports (P) | 8 | 1 | 2 | 1 |
| Container resources (C) | 5 | 0 | 3 | 1 |
| Filesystem paths (F) | 6 | 0 | 0 | 1 |
| Process-global state (G) | 7 | 0 | 0 | 0 |
| External services (X) | 3 | 0 | 0 | 0 |
| **Total (38)** | **29** | **1** | **5** | **3** |

## T1 - Search Method

Run every command from the repository root. `git grep` searches tracked files only, so `target/`
and untracked files are excluded. Each command lists candidates; every candidate was then read in
context and assigned to a finding below.

### Network ports

```bash
# Literal non-zero ports in address strings (bound, parsed, or used as registry keys).
git grep -nE '"(127\.0\.0\.1|0\.0\.0\.0|localhost|\[::1?\]):[1-9][0-9]{1,4}"' -- '*.rs'
# Non-zero ports in TOML configuration embedded in Rust.
git grep -nE 'bind_address *= *\\?"[^"]*:[1-9][0-9]*\\?"' -- '*.rs'
# Socket addresses built from a literal non-zero port.
git grep -nE '(SocketAddr(V4|V6)?::(new|from)|\.bind\(|set_port\()[^;]*[^0-9.:a-z_][1-9][0-9]{2,4}\)' -- '*.rs'
# Every direct bind site, to check the four racing shapes of #2393.
git grep -nE '(TcpListener|UdpSocket)::bind' -- '*.rs' ':!contrib'
# Pre-selected ports handed to a child process.
git grep -n 'reserve_free_ports' -- '*.rs'
# Published host ports in runners, Compose files, scripts and workflows.
git grep -nE -- '--publish|-p [0-9]|ports:|"[0-9]+:[0-9]+(/udp|/tcp)?"' -- '*.rs' '*.yaml' '*.yml' '*.sh'
```

### Container resources

```bash
# Image tags, container names, Compose project names, networks and volumes.
git grep -nE 'torrust-tracker:|IMAGE_TAG|TRACKER_IMAGE|--name|container_name|^name:|-p "\$|--project-name|docker (network|volume)' -- '*.rs' '*.yaml' '*.yml' '*.sh'
# testcontainers users (external services, below).
git grep -lE 'testcontainers' -- '*.rs'
# Which jobs run on the self-hosted runner.
git grep -n 'self-hosted' -- .github/workflows
```

### Filesystem paths

```bash
# Fixed paths under the temporary directory or the repository.
git grep -nE 'env::temp_dir\(\)|"/tmp|/tmp/|mktemp' -- '*.rs' '*.sh' '*.yaml'
git grep -nE '"\.?/?storage/' -- '*.rs'
# Temporary directory users that clean up on drop.
git grep -lE 'tempfile::|TempDir|tempdir\(' -- '*.rs'
# File removal, to see what is cleaned up.
git grep -nE 'remove_file|remove_dir_all' -- '*.rs'
# Configurations whose default database is the repository SQLite file.
git grep -nE 'Configuration::default\(\)|Core::default\(\)' -- '*.rs' ':!packages/configuration' ':!contrib'
```

### Process-global state

```bash
git grep -nE 'env::(set_var|remove_var)|set_var\(' -- '*.rs'
git grep -lE 'Jail::expect_with' -- '*.rs'
git grep -nE 'set_current_dir' -- '*.rs'
git grep -nE 'set_global_default|\.init\(\)|try_init\(\)' -- '*.rs'
git grep -nE 'logs_contains_a_line_with|captured_logs_buffer' -- '*.rs'
git grep -nE '^\s*(pub(\([a-z]+\))? )?static (ref )?[A-Z_0-9]+ *:' -- '*.rs' ':!contrib' ':!.github'
git grep -nE 'thread_local!|lazy_static!' -- '*.rs'
```

### External services

```bash
git grep -lE 'testcontainers' -- '*.rs'
git grep -nE 'image: (mysql|postgres)' -- '*.yaml'
```

## T2 - Network Ports

Issue #2393 inventoried Rust tests for races within one suite run; its per-test table is in
`docs/issues/closed/2393-2392-test-port-allocation-races/ISSUE.md` (Inventory). This audit marks
Covered only the rows #2393 owns and adds collisions between concurrent runs on one host.

| ID | Location | Resource | Who else can use it | Disposition |
| --- | --- | --- | --- | --- |
| P1 | The 25 rows of #2393's per-test table (9 restored on `loopback_ipv4()`, 3 health-check contract tests rewritten, 4 other tests rewritten, 9 deleted) | Ports released and rebound or probed (the four racing shapes) | Concurrent tests in the run and in other runs | **Covered** by #2393 (DONE). The 9 restored tests bind port `0` on a random `127.a.b.c`, so another run's `127.0.0.1:0` bind does not block the rebind; the remaining chance of a `0.0.0.0` collision was measured by #2393 V6 (150 of 150 runs). |
| P2 | `tests/metrics/fixed_ports.rs:46-58` (`metrics-fixed-ports`) | Fixed `0.0.0.0:17091`-`17094`, two TCP and two UDP | Not other tests in the run (one binary, one process). Another run of the same binary on the host network: two agents running `cargo test` or the pre-push hook at once. Any other process on those ports. Not the self-hosted image builds: their `test` stage runs this test in each build step's own network namespace. | **New problem** (Q1), draft D1. Outside the ephemeral range (rule 6) and the test contract needs fixed ports, but two concurrent runs on one machine collide with `AddrInUse`. |
| P3 | `tests/configuration/cli_configuration/base_source_precedence.rs:12`, `per_value_overrides.rs:12` | Random ports from `10000..32768`, released before the tracker child binds them (`reserve_free_ports`, #2581) | A process that listens on that exact port in the window; another run that draws the same random port | **Deliberate exception.** The test must name distinct ports before the child starts. Documented in `tests/common/native_tracker/ports.rs` and kept by the EPIC decision of 2026-11-14 (no new port helper). |
| P4 | `packages/test-helpers/src/configuration.rs` (`ephemeral*`), `tests/common/` fixtures (`127.0.0.1:0`, `127.0.0.2:0`), the `testing` environments of the server crates, `src/app.rs` and `src/bootstrap/jobs/*` tests | Port `0`, kept bound by the service until the test ends | Nobody: the kernel assigns a free port and the service holds it | **Isolated.** |
| P5 | `console/tracker-client/tests/common/fake_trackers/{http,udp}.rs`, `packages/tracker-client/src/udp/client.rs:299`, `packages/udp-server/src/server/{processor,receiver}.rs`, `packages/axum-server/src/signals.rs:285`, `packages/axum-server/tests/idle_connections.rs:84`, `tests/http_health_check.rs` (from `develop` after the snapshot) | Test sockets on `127.0.0.1:0`, kept for the whole test | Nobody | **Isolated.** |
| P6 | `tests/lifecycle/shutdown_policy.rs:199`, `tests/common/native_tracker/failed_start.rs` | Occupied-port tests: a listener on `127.0.0.1:0` held until the tracker child has exited | Nobody: the test holds the port for the whole assertion | **Isolated.** |
| P7 | `packages/axum-http-server/tests/server/v1/contract/for_all_config_modes/receiving_an_announce_request.rs:961,1091`, `receiving_an_scrape_request.rs:290` | `[::1]:0` bound and dropped to detect IPv6 support | Nobody: the address is never reused | **Isolated.** A capability probe, not a pre-selected port. |
| P8 | Literal ports that are only parsed, compared or used as registry keys: `src/bootstrap/config.rs` (`42151`-`42158`), `packages/configuration/src/lib.rs` (`41001`-`41017`), `packages/configuration/src/{v2_0_0,v3_0_0}` (`1313`, `6969`, `6970`, `7070`), `tests/common/native_tracker/health.rs` (`43210`), `src/console/ci/e2e/logs_parser.rs`, `packages/axum-health-check-api-server/{src/environment.rs,tests/server/contract.rs}` (registrations with fake probes), `packages/axum-http-server/src/{server.rs,v1/routes.rs}`, `packages/axum-rest-api-server/src/{server.rs,routes.rs}`, `packages/udp-server/src/server/request_buffer.rs`, `console/tracker-client` checker tests, `packages/http-core` and `packages/primitives` peer addresses | No socket | Nobody | **Isolated.** Never bound. |
| P9 | `src/console/ci/e2e/runner.rs:98-103` (`e2e_tests_runner`) | Fixed host ports `6969/udp`, `7070`, `1212`, `1313` | A second E2E run on the same Docker host (self-hosted runner, development machine) | **Covered** by #2395. |
| P10 | `compose.qbittorrent-e2e.{sqlite3,mysql,postgresql}.yaml` (`qbittorrent_e2e_runner`) | Host port `0` for every published port, read back with `docker compose port` | Nobody | **Isolated.** |
| P11 | `contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh` | No published ports | Nobody | **Isolated.** |
| P12 | `packages/tracker-core/src/databases/driver/{mysql,postgres}/mod.rs` (testcontainers) | Docker-assigned host port, read with `get_host_port_ipv4` | Nobody | **Isolated.** |

## T3 - Container Resources

| ID | Location | Resource | Who else can use it | Disposition |
| --- | --- | --- | --- | --- |
| C1 | `.github/workflows/container.yaml:128-155` (`torrust-tracker:local`), `.github/workflows/testing.yaml:294-315` (`torrust-tracker:e2e-local`) | Fixed image tags on the self-hosted runner's Docker daemon | Another job on the same daemon | **Covered** by #2396. |
| C2 | `src/console/ci/e2e/runner.rs:39` (`e2e_tests_runner` default `torrust-tracker:local` when it builds the image) | Fixed local image tag | Another E2E run, or `docker-build`-style local work, on the same machine | **Covered** by #2396 (its scope asks to decide this default). |
| C3 | `src/console/ci/qbittorrent_e2e/runner.rs:22` (default `torrust-tracker:qbt-e2e-local`, built unless `--skip-build`); `contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh:18-19` (default `torrust-tracker:test-mounted-no-persistence-configuration`, built because `BUILD_IMAGE` defaults to `true`) | Fixed local image tags | Two local runs from two checkouts on one machine: run B rebuilds the tag from its commit while run A is still starting containers from it | **Covered** by #2396, whose scope was extended to both defaults (Q2). In CI both get `--skip-build` / `BUILD_IMAGE=false` and the CI tag (C1). |
| C4 | `src/console/ci/e2e/tracker_container.rs:115` | Container name `tracker_<20 random characters>`, removed on drop | Nobody | **Isolated.** |
| C5 | `src/console/ci/qbittorrent_e2e/types/compose_project_name.rs`, `src/console/ci/compose.rs:108,314` | Compose project `<prefix>-<10 random characters>` passed with `-p`; it overrides the files' `name: qbittorrent-e2e`, and Compose derives container, network and volume names from it; torn down on drop after a successful start | Nobody | **Isolated** (the names). The failure path is C9. |
| C6 | `test-mounted-no-persistence-configuration.sh:38`, `container.yaml:86` (hadolint) | `docker run --rm` without a name | Nobody: Docker assigns a unique name | **Isolated.** |
| C7 | `packages/tracker-core/src/databases/driver/{mysql,postgres}/mod.rs`, `packages/persistence-benchmark` | testcontainers containers, Docker-assigned names | Nobody | **Isolated.** |
| C8 | `.github/workflows/security-scan.yaml:52` (`torrust-tracker:local`) | Fixed tag | Nobody: runs on `ubuntu-latest` only | **Isolated** (one job per virtual machine; also excluded by #2396). |
| C9 | `src/console/ci/compose.rs:73-93`, `src/console/ci/qbittorrent_e2e/services_setup.rs:47` | The containers, network and volumes of a Compose project whose `docker compose up --wait --detach` failed part-way: the `down --volumes` guard exists only after a successful `up` | Nobody else uses them (per-run names, host port `0`), but they stay running and accumulate on the Docker daemon (rule 1); the runner's prune script removes only stopped containers | **New problem** (Q4), draft D3. Found by Copilot in the review of PR #2891. |

## T4 - Filesystem Paths

| ID | Location | Resource | Who else can use it | Disposition |
| --- | --- | --- | --- | --- |
| F1 | `tests/common/workspace.rs`, `tests/common/native_tracker/{command,failed_start}.rs`, `src/bootstrap/jobs/http_tracker.rs:195`, `src/bootstrap/config.rs` tests, `src/console/ci/qbittorrent_e2e/filesystem_setup.rs` (default) | `tempfile::TempDir` per test or per run, removed on drop | Nobody | **Isolated.** |
| F2 | `packages/test-helpers/src/configuration.rs:85` (`ephemeral_sqlite_database`), used by `ephemeral*` and by `packages/tracker-core` SQLite tests; `packages/configuration/src/{v2_0_0,v3_0_0}/mod.rs` (`test_config_<uuid>.toml`); `packages/axum-server/src/tls.rs:72` (`<prefix>-<nanos>.pem`) | Files in the temporary directory with a random name | Nobody: the names are unique per call | **New problem** (Q3), draft D2. Unique, so no collision, but the SQLite and TOML files are never removed and the PEM files only on success (rule 1). |
| F3 | `src/console/ci/qbittorrent_e2e/filesystem_setup.rs:67` (`--keep-containers`) | `storage/qbt-e2e/<random project name>` | Nobody | **Isolated** (per-run name; kept on purpose for debugging). |
| F4 | `contrib/dev-tools/{analysis,checks}/*/tests/*.rs` | Temporary directories named with PID and timestamp, or `tempfile` | Nobody | **Isolated.** |
| F5 | `share/default/config/*.toml`, `packages/*/tests/fixtures/*` | Repository files read by tests | Readers only | **Isolated** (read-only). |
| F6 | `contrib/dev-tools/git/hooks/{pre-commit,pre-push}.sh` | Step logs via `mktemp` in `TORRUST_GIT_HOOKS_LOG_DIR` (default `/tmp`) | Nobody | **Isolated.** |
| F7 | `packages/{axum-http-server,udp-server}/examples/*_public_tracker.rs` | Fixed `$TMPDIR/torrust-{http,udp}-example.db` | Nobody in tests: `cargo test --examples` only builds examples; no test or CI step runs them | **Isolated** (not a test resource). |

## T5 - Process-Global State and External Services

| ID | Location | Resource | Who else can use it | Disposition |
| --- | --- | --- | --- | --- |
| G1 | `src/bootstrap/config.rs:60-145` | `TORRUST_TRACKER_CONFIG_TOML` and `TORRUST_TRACKER_CONFIG_TOML_PATH` set and removed with `std::env::set_var` | Other tests of the `torrust-tracker` lib binary under `cargo test` | **Isolated.** Every test in the module takes `ENVIRONMENT_LOCK`, a guard restores the values, and no other test in that binary reads these variables (`initialize_configuration` and `Info::new` are only called there). |
| G2 | `packages/configuration/src/{lib.rs,v2_0_0,v3_0_0}` | Environment and current directory changed by `figment::Jail` | Other tests of the configuration lib binary | **Isolated.** `Jail` serializes on its own global lock and restores both. |
| G3 | `packages/test-helpers/src/logging.rs`, `packages/configuration/src/logging.rs` and `v{2,3}_0_0/logging.rs` | Global tracing subscriber and the shared captured-log buffer | Every test in the same binary | **Isolated.** Initialization is `Once`-guarded, and every log assertion matches a value unique to the test (a UUID request ID, a random info-hash, a transaction ID, or a documentation address). |
| G4 | `packages/axum-health-check-api-server/tests/server/contract.rs:568-569` | Static probe counters | Nobody: one counter per test, as its comment states | **Isolated.** |
| G5 | `torrust-clock` 3.0.0 `clock::Stopped` (the `CurrentClock` of every crate's tests) | Stopped time | Nobody: thread-local, and each test runs on its own thread | **Isolated.** |
| G6 | `RUSTLS_CRYPTO_PROVIDER` / `ONCE` statics in `packages/axum-health-check-api-server/tests/server/client.rs:5`, `packages/axum-rest-api-server/src/server.rs:614`, `packages/axum-server/tests/idle_connections.rs:48`; `DEFAULT_PEER_ID`, `TEST_COOKIE_CIPHER`, `NEXT_MANAGER_ID` | Write-once or monotonically increasing statics | Every test in the same binary, read-only after first use | **Isolated.** |
| G7 | `tests/common/native_tracker/command.rs` | Environment of the tracker child (`Command::env`, `env_remove`) | Nobody: it changes the child's environment, not the test process's; no test calls `std::env::set_current_dir` | **Isolated.** |
| X1 | `packages/tracker-core/src/databases/driver/{mysql,postgres}/mod.rs` | One MySQL or PostgreSQL container per test (testcontainers), gated by `TORRUST_TRACKER_CORE_RUN_*_DRIVER_TEST` | Nobody | **Isolated.** |
| X2 | `compose.qbittorrent-e2e.{mysql,postgresql}.yaml` | Database service inside the per-run Compose project (C5) | Nobody | **Isolated.** |
| X3 | SQLite databases of the in-process and native-process suites | Per-test file (F1, F2) | Nobody | **Isolated.** |

## Excluded by Scope

- The persistent Cargo target directory of the self-hosted runner jobs
  (`testing.yaml` `docker-e2e`: `CARGO_TARGET_DIR=$HOME/.cache/torrust-tracker/docker-e2e-target`),
  and the shared `target/` of one checkout: excluded by the EPIC (Cargo's lock keeps builds correct).
- Resources shared only across GitHub-hosted jobs, which run on separate virtual machines.
- Developer and debugging tools that are not tests and no test or CI step runs:
  `contrib/dev-tools/containers/docker-run-local.sh` (fixed host ports and `./storage/`),
  `contrib/dev-tools/debugging/qbt/check-qbittorrent-e2e-compose.sh` (timestamped project name,
  `torrust-tracker:qbt-e2e-local`), and `contrib/dev-tools/github/github-review-threads`
  (`/tmp/pr_threads_<pr>.json`).

## Maintainer Decisions

The maintainer answered the four judgement calls on 2026-11-16 (Q4 after the first review round):

- **Q1 (P2): new problem.** `tests/metrics/fixed_ports.rs` binds `0.0.0.0:17091`-`17094`, so two
  concurrent runs of `cargo test` on one machine (two agents, or two pre-push hooks) collide.
  Draft D1. Confirmed on 2026-11-16: running the built test binary twice at once failed one of
  the two runs with `AddrInUse` in 20 of 20 pairs.
- **Q2 (C3): extend #2396.** The local default image tags of `qbittorrent_e2e_runner` and the
  container test script are the same problem as the `e2e_tests_runner` default that #2396 already
  has to decide. The EPIC coordinator adds them to #2396's scope.
- **Q3 (F2): new problem.** Temporary files with unique names that are never removed break rule 1
  even though they never collide. Draft D2. On the maintainer's machine on 2026-11-16, `/tmp`
  held 232,229 `data_*.db` files (about 10.6 GiB, created since 2026-11-13) and 1,590
  `test_config_*.toml` files.

- **Q4 (C9): new problem, own draft.** A failed `docker compose up` leaves its project's
  containers, network and volumes on the daemon. Draft D3.

## New-Problem Drafts (T6)

| Draft | Findings | Issue and spec |
| --- | --- | --- |
| D1 | P2 | #2906, `docs/issues/open/2906-2392-fixed-port-metrics-test-concurrent-runs/ISSUE.md` |
| D2 | F2 | #2907, `docs/issues/open/2907-2392-remove-leaked-test-temporary-files/ISSUE.md` |
| D3 | C9 | #2908, `docs/issues/open/2908-2392-compose-cleanup-after-failed-start/ISSUE.md` |
