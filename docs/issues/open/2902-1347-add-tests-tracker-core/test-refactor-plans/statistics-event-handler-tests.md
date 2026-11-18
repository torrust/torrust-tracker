---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/statistics/event/handler.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/statistics/event/handler.rs
    - packages/tracker-core/src/statistics/persisted/downloads.rs
    - packages/tracker-core/src/statistics/repository.rs
    - packages/swarm-coordination-registry/src/event.rs
---

# Statistics Event Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/tracker-core/src/statistics/event/handler.rs`, ledger row 5.

## Current State

- **Unit-only coverage:** 175 / 196 lines at the T1 baseline and after rows 1 to 4 (stable Rust
  1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, 2026-11-18). Uncovered:
  - 19-20, 24 and 29: the `TorrentAdded`, `TorrentRemoved`, `PeerRemoved` and `PeerUpdated` arms of
    `handle_in_memory_event`;
  - 36-58: the five logging-only handlers (`handle_torrent_added`, `_removed`, `handle_peer_added`,
    `_removed`, `_updated`), reached only through those arms (`PeerAdded` runs in a test, but its
    `tracing::debug!` region does not);
  - 66, 106, 108 and 120: log-level regions of `tracing` calls;
  - 129: the error arm of `increase_global_downloads`.
- **Existing tests:** 5. A completion increments both in-memory totals; `PeerAdded` changes no
  in-memory total; a completion is persisted (torrent row, global row, persisted total); `PeerAdded`
  persists nothing; a failed torrent write still persists the global total.
- **Module-owned decisions not yet tested:**
  - Only `PeerDownloadCompleted` changes a total: the other five event kinds only log. One of the
    five is tested, for each handler.
  - The persisted total follows the database: when the global write fails, the in-memory persisted
    total is not incremented (line 129), even though the torrent row was written.

## Maintainer Decision

Plan approved on 2026-11-18, including replacing the two `PeerAdded`-only tests by the tables.

## Test Design

| Test | Arrange | Assert |
| --- | --- | --- |
| `it_should_change_no_in_memory_total_for_an_event_other_than_a_completed_download` | each of `TorrentAdded`, `TorrentRemoved`, `PeerAdded`, `PeerRemoved` and `PeerUpdated`, in a table, on a fresh repository | the persistent and in-session totals read 0, with the event's name in the message |
| `it_should_persist_nothing_for_an_event_other_than_a_completed_download` | the same table, on a fresh `TempDir` database | no torrent row, no global row, persisted total 0 |
| `it_should_not_increment_the_persisted_total_when_the_global_downloads_cannot_be_saved` | `MockTorrentMetricsStore`: the torrent write succeeds (`load_torrent_downloads` → `None`, `save_torrent_downloads` → `Ok`), `load_global_downloads` fails | the persisted total reads 0 |

The two table tests replace the two existing `PeerAdded`-only tests
(`it_should_not_increment_in_memory_completion_metrics_when_an_event_is_not_a_completion` and
`it_should_not_persist_completion_statistics_when_an_event_is_not_a_completion`), which become one
case of each table. The other three existing tests stay as they are.

**Not tested:** the log output of the five handlers. A log line is not a contract of this package,
and asserting it would need a tracing subscriber in the tests; the table tests run the handlers, so
a panic in one would fail them.

## Property-Test Decision

No `quickcheck`: the input is an enum of six cases, all listed in the table.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | The `PeerRemoved` arm counts a completed download (`handle_peer_download_completed(info_hash, peer, ...)`) | the in-memory table (`PeerRemoved` case) |
| M2 | The `PeerUpdated` arm counts a completed download (with `new_peer`) | the in-memory table (`PeerUpdated` case) |
| M3 | The error arm of `increase_global_downloads` also increments the persisted total | the global-failure test |
| M4 | The persistent handler persists every event (`if let` replaced by an unconditional call with a sample info-hash) | the persistence table |

M1 and M2 survive today: the only non-completion event under test is `PeerAdded`.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| H1 | TODO | Add the three tests and remove the two replaced ones. | They pass; lines 19-58 and 129 run. |
| H2 | TODO | Run M1-M4. | Each is killed by its expected test. |
| H3 | TODO | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

To be filled in after H3.
