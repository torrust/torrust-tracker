---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2406-scrape-ignores-persisted-torrent-downloads/ISSUE.md
last-updated-utc: 2026-10-02 17:59
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-02 13:29 to 13:30
- Artifact under test: debug binary built with `cargo run` from PR branch commit `83a1f6c4fe677fb117ecdfa1ad674a789c2bc981` (`docs(issues): [#2406] add issue specification for scrape ignoring persisted downloads`), before the planned production fix
- Operating system / environment: Linux, local workspace
- Runtime: `./target/debug/torrust-tracker` with Rust-built workspace artifacts
- Database backend: isolated local SQLite database at `.tmp/scrape-persisted-downloads-bug.sqlite3`
- Tracker configuration: `.tmp/scrape-persisted-downloads-bug.toml`, with `persistent_torrent_completed_stat = true`, UDP `127.0.0.1:17696`, and HTTP `127.0.0.1:17070`
- Client: `cargo run -q -p torrust-tracker-client --bin tracker_client -- ...` on the stable Rust toolchain

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
- Configuration: `.tmp/scrape-persisted-downloads-fix-v2.toml`, identical to V1 except the fresh SQLite path `.tmp/scrape-persisted-downloads-fix-v2.sqlite3`; `persistent_torrent_completed_stat = true`, `remove_peerless_torrents = true`, UDP `127.0.0.1:17696`, HTTP `127.0.0.1:17070`
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

## Failures and Follow-up

The first announce attempt used a 40-character hexadecimal peer ID. The client rejected it before sending a request because this CLI argument expects a 20-byte text peer ID. The successful reproduction used `ABCDEFGHIJKLMNOPQRST`; no tracker state was created by the rejected command.

V2 records the like-for-like post-fix manual recheck; the red and green regression-test output is recorded under Regression-Test Boundary.
