---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2406-scrape-ignores-persisted-torrent-downloads/ISSUE.md
last-updated-utc: 2026-10-04 07:57
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-02 13:29 to 13:30
- Artifact under test: debug binary built with `cargo run` from the spec PR branch at `docs(issues): [#2406] add issue specification for scrape ignoring persisted downloads`, before the planned production fix
- Operating system / environment: Linux, local workspace
- Runtime: `./target/debug/torrust-tracker` with Rust-built workspace artifacts
- Database backend: isolated local SQLite database at `.tmp/scrape-persisted-downloads-bug.sqlite3`
- Tracker configuration: `.tmp/scrape-persisted-downloads-bug.toml`, reproduced verbatim in [Tracker Configuration](#tracker-configuration), with `persistent_torrent_completed_stat = true`, UDP `127.0.0.1:17696`, and HTTP `127.0.0.1:17070`
- Client: `cargo run -q -p torrust-tracker-client --bin tracker_client -- ...` on the stable Rust toolchain

`.tmp/` is git-ignored, so the paths above are local to the verification run. The configuration is preserved below; the SQLite databases and tracker logs were runtime outputs whose relevant content (queries, responses, shutdown lines) is recorded inline in each process.

### Tracker Configuration

V1 used this file as `.tmp/scrape-persisted-downloads-bug.toml`. The access token is a throwaway value for the local run.

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
inactive_peer_cleanup_interval = 120
listed = false
private = false

[core.database]
driver = "sqlite3"
path = ".tmp/scrape-persisted-downloads-bug.sqlite3"

[core.tracker_policy]
max_peer_timeout = 60
persistent_torrent_completed_stat = true
remove_peerless_torrents = true

[[udp_trackers]]
bind_address = "127.0.0.1:17696"
tracker_usage_statistics = true

[[http_trackers]]
bind_address = "127.0.0.1:17070"
tracker_usage_statistics = true

[http_api]
bind_address = "127.0.0.1:11212"

[http_api.access_tokens]
admin = "ManualVerificationToken"
```

V2 and V3 used copies that differ only in `core.database.path`, so each run started from a fresh database:

```sh
sed 's/scrape-persisted-downloads-bug.sqlite3/scrape-persisted-downloads-fix-v2.sqlite3/' \
  .tmp/scrape-persisted-downloads-bug.toml > .tmp/scrape-persisted-downloads-fix-v2.toml
sed 's/scrape-persisted-downloads-bug.sqlite3/scrape-persisted-downloads-fix-v3.sqlite3/' \
  .tmp/scrape-persisted-downloads-bug.toml > .tmp/scrape-persisted-downloads-fix-v3.toml
```

## Verification Processes

### V1 - Persisted Torrent Scrape After Restart

- Goal: reproduce an incorrect zero scrape count after a clean restart even though the torrent completion count was persisted.
- Initial state: a new SQLite file and an otherwise empty tracker; info hash `1111111111111111111111111111111111111111`; peer ID `ABCDEFGHIJKLMNOPQRST`.
- Status: `DONE` (reproduced)

#### Steps Performed

1. Created the isolated configuration and started the debug tracker:

   ```text
   ./target/debug/torrust-tracker --config-toml-path .tmp/scrape-persisted-downloads-bug.toml
   ```

2. Sent a UDP `started` announce, a UDP `completed` announce, and a pre-restart UDP scrape:

   ```text
   cargo run -q -p torrust-tracker-client --bin tracker_client -- udp announce 127.0.0.1:17696 1111111111111111111111111111111111111111 --event started --uploaded 0 --downloaded 0 --left 1000 --port 16881 --peer-id ABCDEFGHIJKLMNOPQRST --key 1 --peers-wanted 0
   cargo run -q -p torrust-tracker-client --bin tracker_client -- udp announce 127.0.0.1:17696 1111111111111111111111111111111111111111 --event completed --uploaded 0 --downloaded 1000 --left 0 --port 16881 --peer-id ABCDEFGHIJKLMNOPQRST --key 1 --peers-wanted 0
   cargo run -q -p torrust-tracker-client --bin tracker_client -- udp scrape 127.0.0.1:17696 1111111111111111111111111111111111111111
   ```

3. Stopped the tracker with `SIGINT`, which completed its cooperative shutdown and persistence listener. Queried the SQLite database before restart:

   ```text
   sqlite3 -header -column .tmp/scrape-persisted-downloads-bug.sqlite3 "SELECT info_hash, completed FROM torrents WHERE info_hash = '1111111111111111111111111111111111111111';"

   info_hash                                 completed
   ----------------------------------------  ---------
   1111111111111111111111111111111111111111  1
   ```

4. Restarted the same binary with the same configuration and SQLite file, without a new announce.
5. Scraped the original info hash over HTTP and UDP:

   ```text
   cargo run -q -p torrust-tracker-client --bin tracker_client -- http scrape http://127.0.0.1:17070 1111111111111111111111111111111111111111
   cargo run -q -p torrust-tracker-client --bin tracker_client -- udp scrape 127.0.0.1:17696 1111111111111111111111111111111111111111
   ```

6. Stopped the restarted tracker with `SIGINT`; it logged a successful cooperative shutdown.

#### Observed Result

```text
Pre-restart UDP scrape:
{"Scrape":{"transaction_id":-888840697,"torrent_stats":[{"seeders":1,"completed":1,"leechers":0}]}}

Post-restart HTTP scrape:
{"1111111111111111111111111111111111111111":{"complete":0,"downloaded":0,"incomplete":0}}

Post-restart UDP scrape:
{"Scrape":{"transaction_id":-888840697,"torrent_stats":[{"seeders":0,"completed":0,"leechers":0}]}}
```

The tracker startup log confirmed the same SQLite path and `persistent_torrent_completed_stat: true` for both runs. Each process shut down with `Torrust tracker successfully shutdown.` The direct post-shutdown query confirms that the per-torrent row was persisted before the fresh process returned zero counts.

#### Conclusion

Reproduced. The completion count was observable as one before restart, but both protocol scrape responses reported zero immediately after restart and before another announce. This supports the hypothesis that scrape reads only in-memory swarm metadata, while announce is the only lazy persistence-hydration path.

### V2 - Post-Fix Like-for-Like Recheck

- Goal: confirm HTTP and UDP scrape report the persisted count after a clean restart and before any new announce.
- Date and time (UTC): 2026-10-02 17:57 to 17:58
- Artifact under test: debug binaries built with `cargo build --bin torrust-tracker` and `cargo build -p torrust-tracker-client --bin tracker_client` from the code state committed as `fix(tracker-core): [#2406] scrape reports persisted downloads for swarms absent from memory`
- Configuration: `.tmp/scrape-persisted-downloads-fix-v2.toml`, the [V1 configuration](#tracker-configuration) with only the SQLite path changed to `.tmp/scrape-persisted-downloads-fix-v2.sqlite3`; `persistent_torrent_completed_stat = true`, `remove_peerless_torrents = true`, UDP `127.0.0.1:17696`, HTTP `127.0.0.1:17070`
- Same info hash `1111111111111111111111111111111111111111` and peer ID `ABCDEFGHIJKLMNOPQRST` as V1
- Status: `DONE` (fixed)

#### Steps Performed

1. Started `./target/debug/torrust-tracker --config-toml-path .tmp/scrape-persisted-downloads-fix-v2.toml`.
2. Ran the same UDP `started` and `completed` announces and the pre-restart UDP scrape as V1, using `./target/debug/tracker_client`.
3. Stopped the tracker with `SIGINT` (exit code 0, `Torrust tracker successfully shutdown.`) and queried SQLite:

   ```text
   info_hash                                 completed
   ----------------------------------------  ---------
   1111111111111111111111111111111111111111  1
   ```

4. Restarted the same binary with the same configuration and database, without any announce.
5. Scraped over HTTP once and over UDP twice, then stopped with `SIGINT` (exit code 0). The startup log shows the V2 SQLite path and `"persistent_torrent_completed_stat": true`.

#### Observed Result

```text
Pre-restart UDP scrape:
{"Scrape":{"transaction_id":-888840697,"torrent_stats":[{"seeders":1,"completed":1,"leechers":0}]}}

Post-restart HTTP scrape:
{"1111111111111111111111111111111111111111":{"complete":0,"downloaded":1,"incomplete":0}}

Post-restart UDP scrape (first and second):
{"Scrape":{"transaction_id":-888840697,"torrent_stats":[{"seeders":0,"completed":1,"leechers":0}]}}
{"Scrape":{"transaction_id":-888840697,"torrent_stats":[{"seeders":0,"completed":1,"leechers":0}]}}
```

#### Conclusion

Fixed. After a clean restart and before any announce, HTTP `downloaded` and UDP `completed` both equal the persisted count `1`, while seeders and leechers remain zero.

### V3 - Multi-Torrent Scrape After Restart (Batch Lookup)

- Goal: confirm the batch lookup returns correct per-torrent counts for a mixed scrape request after a restart.
- Date and time (UTC): 2026-10-02 18:32 to 18:33
- Artifact under test: debug binaries rebuilt at `perf(tracker-core): [#2406] load persisted scrape downloads in one batch query`
- Configuration: `.tmp/scrape-persisted-downloads-fix-v3.toml`, the [V1 configuration](#tracker-configuration) with only the SQLite path changed to `.tmp/scrape-persisted-downloads-fix-v3.sqlite3`
- Info hashes: `1111…` and `2222…` (each announced `started` then `completed` before the restart), `3333…` (never announced)
- Status: `DONE` (fixed)

#### Steps Performed

1. Started the tracker, sent UDP `started` and `completed` announces for `1111…` and `2222…`, and stopped it with `SIGINT` (exit code 0). SQLite contained `1111…|1` and `2222…|1`.
2. Restarted with the same configuration and database, without announcing.
3. Scraped all three info-hashes in one HTTP request and one UDP request, then stopped with `SIGINT` (exit code 0, `Torrust tracker successfully shutdown.`).

#### Observed Result

```text
HTTP:
{"2222222222222222222222222222222222222222":{"complete":0,"downloaded":1,"incomplete":0},"1111111111111111111111111111111111111111":{"complete":0,"downloaded":1,"incomplete":0},"3333333333333333333333333333333333333333":{"complete":0,"downloaded":0,"incomplete":0}}

UDP (request order 1111…, 2222…, 3333…):
{"Scrape":{"transaction_id":-888840697,"torrent_stats":[{"seeders":0,"completed":1,"leechers":0},{"seeders":0,"completed":1,"leechers":0},{"seeders":0,"completed":0,"leechers":0}]}}
```

#### Conclusion

Fixed. Both persisted torrents report `1` and the unknown torrent reports `0` in a single request on both protocols; UDP keeps the request order.

## Regression-Test Boundary

Use a `tracker-core` SQLite-backed restart integration test. It is the smallest current deterministic boundary that can observe the contract: a fresh in-memory repository with the same persisted database must return the saved count from `ScrapeHandler`. A pure unit test is not currently available because `ScrapeHandler` does not own a persistence dependency; reassess after the implementation design chooses its seam.

### Selected Tests (T3)

Added to `packages/tracker-core/tests/integration.rs`:

- `it_should_scrape_the_persisted_downloads_of_a_torrent_absent_from_memory` (AC1, AC2, AC5): a started tracker core with an empty in-memory repository and a SQLite row of `7` downloads written directly through the torrent metrics store (`TestEnv::persist_torrent_downloads`). This is the state a restart leaves: a persisted row and no swarm in memory. Writing the row directly states the persisted count independently of the announce path and avoids waiting for the asynchronous persistence listener. The test also asserts that scrape leaves the torrent absent from memory (Option A).
- `it_should_not_scrape_persisted_downloads_when_the_persistent_completed_stat_is_disabled` (AC3): same database row, `persistent_torrent_completed_stat = false`, scrape returns zeroed metadata.

Prose-first Arrange-Act-Assert review: the one visible causal difference between the two tests is the `persistent_torrent_completed_stat` flag; the persisted count `7` is inline in the Arrange; `TestEnv` owns only incidental composition; Act is the production `ScrapeHandler::handle_scrape` call via `TestEnv::scrape`; assertions compare typed `SwarmMetadata` values.

### Red Run Against the Broken Implementation (T3)

Stable toolchain, code state `test(tracker-core): [#2406] add regression tests for scrape of persisted downloads` (tests committed before the fix):

```text
$ cargo test -p torrust-tracker-core --test integration scrape
running 3 tests
test it_should_handle_the_scrape_request ... ok
test it_should_scrape_the_persisted_downloads_of_a_torrent_absent_from_memory ... FAILED
test it_should_not_scrape_persisted_downloads_when_the_persistent_completed_stat_is_disabled ... ok

thread 'it_should_scrape_the_persisted_downloads_of_a_torrent_absent_from_memory' panicked at packages/tracker-core/tests/integration.rs:73:5:
assertion `left == right` failed
  left: SwarmMetadata { downloaded: 0, complete: 0, incomplete: 0 }
 right: SwarmMetadata { downloaded: 7, complete: 0, incomplete: 0 }

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out
```

The AC3 test passes before the fix by design: it guards that the fix does not read the database when persistence of completed stats is disabled.

### Green Run After the Fix (T4/T5)

```text
$ cargo test -p torrust-tracker-core --test integration
test it_should_handle_the_announce_request ... ok
test it_should_not_return_the_peer_making_the_announce_request ... ok
test it_should_handle_the_scrape_request ... ok
test it_should_not_scrape_persisted_downloads_when_the_persistent_completed_stat_is_disabled ... ok
test it_should_scrape_the_persisted_downloads_of_a_torrent_absent_from_memory ... ok
test it_should_persist_the_number_of_completed_peers_for_each_torrent_into_the_database ... ok
test it_should_reset_in_session_completed_downloads_after_a_persistence_free_restart ... ok
test it_should_persist_the_global_number_of_completed_peers_into_the_database ... ok
test result: ok. 8 passed; 0 failed
```

The fix also added unit tests at the handler seam created by the design:

- `scrape_handler::tests::it_should_report_an_in_memory_swarm_without_reading_the_persisted_downloads`: a `MockTorrentMetricsStore` without expectations panics on any database call, so the in-memory swarm (`downloaded = 3`, one seeder) must be reported without a lookup.
- `scrape_handler::tests::it_should_fail_when_loading_the_persisted_downloads_fails`: a failing store yields `ScrapeError::Database`.
- `udp-server` `event::tests::it_should_classify_a_scrape_database_error`: the new variant maps to `ErrorKind::Database`.

Affected packages (`torrust-tracker-core`, `-http-core`, `-udp-core`, `-udp-server`, `-axum-http-server`) pass with `cargo test`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic` is clean.

### Batch Lookup (T7)

Code state: `perf(tracker-core): [#2406] load persisted scrape downloads in one batch query`.

Tests added:

- Unit test `scrape_handler::tests::it_should_load_the_persisted_downloads_of_all_torrents_absent_from_memory_in_one_query`: one torrent in memory, one persisted, one unknown. The mock store expects exactly one `load_torrents_downloads` call with only the two absent info-hashes, so it guards against both N+1 queries and querying in-memory torrents.
- Driver tests (shared by all backends): `it_should_load_the_persisted_downloads_of_the_requested_torrents_only` and `it_should_load_the_persisted_downloads_of_more_torrents_than_fit_in_one_query` (101 torrents, more than `MAX_INFO_HASHES_PER_QUERY`).

Mutation proof: the scrape handler was temporarily changed to call the lookup once per info-hash (`absent_from_memory.chunks(1)`), not staged, and restored by hand afterwards:

```text
$ cargo test -p torrust-tracker-core --lib scrape_handler::tests::it_should_load_the_persisted
test scrape_handler::tests::it_should_load_the_persisted_downloads_of_all_torrents_absent_from_memory_in_one_query ... FAILED
MockTorrentMetricsStore::load_torrents_downloads([InfoHash([2, 2, ...])]): No matching expectation found
```

Green runs after restoring:

```text
$ cargo test -p torrust-tracker-core
test result: ok. 150 passed; 0 failed (lib, includes run_sqlite_driver_tests)
test result: ok. 8 passed; 0 failed (integration)

$ TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true cargo test -p torrust-tracker-core --features db-compatibility-tests run_mysql_driver_tests
test result: ok. 1 passed; 0 failed

$ TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST=true cargo test -p torrust-tracker-core --features db-compatibility-tests run_postgres_driver_tests
test result: ok. 1 passed; 0 failed
```

## Automated Equivalents of the Manual Scenarios (T8)

| Manual aspect | Automated test |
| --- | --- |
| HTTP scrape returns the persisted count | `http-core` `services::scrape::tests::with_real_data::it_should_return_the_persisted_downloads_of_a_torrent_absent_from_memory` |
| UDP scrape returns persisted counts in request order | `udp-server` `handlers::scrape::tests::scrape_request::it_should_return_the_persisted_downloads_of_torrents_absent_from_memory_in_request_order` |
| Completion by a real announce, graceful shutdown, restart on the same `SQLite` file, HTTP and UDP scrape of a completed and an unknown torrent | root binary `persistence-scrape-after-restart` (`tests/persistence/scrape_after_restart.rs`) |

The root test restarts the application in-process with `TrackerApplicationFixture::restart`, which gracefully stops all jobs and starts a new application on the same workspace. It does not start a separate OS process; the manual V2/V3 checks remain the evidence for the real binary.

Mutation proof: the persisted lookup was disabled in `scrape_handler.rs` (`load_many(&[])`), not staged, and restored with `git checkout --` (the file had no uncommitted changes). All three tests failed, for example:

```text
$ cargo test --test persistence-scrape-after-restart
test it_should_scrape_the_persisted_downloads_of_a_torrent_after_a_restart ... FAILED
  left: [SwarmMetadata { downloaded: 0, complete: 0, incomplete: 0 }, SwarmMetadata { downloaded: 0, complete: 0, incomplete: 0 }]
 right: [SwarmMetadata { downloaded: 1, complete: 0, incomplete: 0 }, SwarmMetadata { downloaded: 0, complete: 0, incomplete: 0 }]
```

Stability: before restarting, the test waits under a 5-second deadline until the persisted row exists (`wait_for_persisted_downloads`). Graceful shutdown cancels the persistence listener without draining queued events, so without this wait the test could race the asynchronous write. With the wait, it passed 20 of 20 consecutive runs. An earlier version of this note claimed that 20 passing runs without the wait proved the write completed before shutdown; that was a sample, not a guarantee (review finding F7 on PR #2423).

## Test Review Against the Write-Unit-Test Guide (T9)

Code state: `test: [#2406] align new scrape tests with the write-unit-test guide`. Every test added by this PR was reviewed against `.github/skills/dev/testing/write-unit-test/SKILL.md`, `docs/testing.md`, and `tests/AGENTS.md`.

| Test | Smell found | Change |
| --- | --- | --- |
| `tracker-core` `it_should_scrape_the_persisted_downloads_of_a_torrent_absent_from_memory` | Multiple assertions: returned value and the absence of a memory insert are two behaviors | Split; new `it_should_not_load_a_scraped_torrent_into_memory` |
| `scrape_handler` unit tests | Hidden behavioral data: the expected seeder came from `sample_peer()` being a seeder | Arrange uses the state-named `seeder()` fixture |
| `scrape_handler` one-query test | Multiple field assertions on one result | One semantic assertion against an independently built `ScrapeData` |
| Driver batch tests | No AAA markers | Added `// Arrange`, `// Act`, `// Assert` |
| Root `persistence-scrape-after-restart` | One scenario asserted two protocol contracts, against `tests/AGENTS.md` | One runner with one scenario function per protocol |
| All new tests | No assertion messages | Messages state the scenario and the causal values |

Tests left unchanged after review: the persistence-disabled integration test, the repeated-info-hash and database-failure unit tests (messages added), the `udp-server` error-kind test (matches its module's style), and the `http-core`/`udp-server` protocol tests (messages added). The `http-core` test keeps its client IP and binding setup inline, as its neighboring tests do; it is incidental and does not vary the behavior.

Prose-first AAA comparison (summary; the temporary prose was removed once the code expressed it):

| Test | Arrange (causal state) | Act | Assert (independent result) |
| --- | --- | --- | --- |
| Persisted scrape (`tracker-core`) | Persistence on; row of 7 written; nothing in memory | `ScrapeHandler::handle_scrape` | `downloaded = 7`, zero peers |
| No memory insert | Same state | Same Act | No swarm for the torrent afterwards |
| Persistence disabled | Persistence off; row of 7 | Same Act | Zeroed metadata |
| In-memory swarm, no DB read | Swarm with 3 downloads and one seeder; store mock with no expectations | Scrape that torrent | `(3, 1, 0)`; any DB call panics |
| One batch query | In-memory, persisted (5), unknown | Scrape all three | Whole `ScrapeData`; mock expects one call with the two absent hashes |
| Repeated info-hash | Persisted (5); scrape lists it twice | Scrape | `(5, 0, 0)`; mock expects one call with one hash |
| Database failure | Failing store | Scrape | `ScrapeError::Database` |
| Driver batch tests | Rows for requested, not-requested, missing hashes; 101 rows | `load_torrents_downloads` | Only requested rows; all 101 across chunks |
| Protocol tests | Persistence on; rows written | HTTP scrape service / UDP `handle_scrape` | `downloaded = 7`; UDP `(7, 0, 9)` in request order |
| Root restart | UDP completion, graceful restart | HTTP scrape; UDP scrape | `[(1, 0, 0), zeroed]` per protocol |

Failure messages were read cold by disabling the persisted lookup (`load_many(&absent_from_memory[..0])`, unstaged, restored with `git checkout --` after the refactor was committed). Each failing test printed its scenario and expected value, for example:

```text
assertion `left == right` failed: UDP scrape should report persisted downloads (7, 0, 9) in request order (persisted, unknown, other persisted)
assertion `left == right` failed: HTTP scrape after restart: completed torrent should report 1 persisted download, unknown torrent zeros
```

The earlier T8 mutation `load_many(&[])` no longer compiles after the F4 deduplication change (unused variable), so this compiling equivalent replaces it.

## Failures and Follow-up

The first announce attempt used a 40-character hexadecimal peer ID. The client rejected it before sending a request because this CLI argument expects a 20-byte text peer ID. The successful reproduction used `ABCDEFGHIJKLMNOPQRST`; no tracker state was created by the rejected command.

V2 records the like-for-like post-fix manual recheck; the red and green regression-test output is recorded under Regression-Test Boundary.
