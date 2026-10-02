---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2406-scrape-ignores-persisted-torrent-downloads/ISSUE.md
last-updated-utc: 2026-10-02 11:09
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-02 11:05 to 11:09
- Artifact under test: current workspace debug binary before the planned fix
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

3. Stopped the tracker with `SIGINT`, which completed its cooperative shutdown and persistence listener.
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

The tracker startup log confirmed the same SQLite path and `persistent_torrent_completed_stat: true` for both runs. Each process shut down with `Torrust tracker successfully shutdown.`

#### Conclusion

Reproduced. The completion count was observable as one before restart, but both protocol scrape responses reported zero immediately after restart and before another announce. This supports the hypothesis that scrape reads only in-memory swarm metadata, while announce is the only lazy persistence-hydration path.

## Regression-Test Boundary

Use a `tracker-core` SQLite-backed restart integration test. It is the smallest current deterministic boundary that can observe the contract: a fresh in-memory repository with the same persisted database must return the saved count from `ScrapeHandler`. A pure unit test is not currently available because `ScrapeHandler` does not own a persistence dependency; reassess after the implementation design chooses its seam.

## Failures and Follow-up

The first announce attempt used a 40-character hexadecimal peer ID. The client rejected it before sending a request because this CLI argument expects a 20-byte text peer ID. The successful reproduction used `ABCDEFGHIJKLMNOPQRST`; no tracker state was created by the rejected command.

V2 will record the red and green regression-test output and the like-for-like post-fix manual recheck.
