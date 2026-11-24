---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/statistics/event/handler.rs
status: done
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
  - The persisted total follows the database: when the global downloads cannot be increased (the test fails their read), the in-memory persisted
    total is not incremented (line 129), even though the torrent row was written.

## Maintainer Decision

Plan approved on 2026-11-18, including replacing the two `PeerAdded`-only tests by the tables.

## Test Design

| Test | Arrange | Assert |
| --- | --- | --- |
| `it_should_change_no_in_memory_total_for_an_event_other_than_a_completed_download` | each of `TorrentAdded`, `TorrentRemoved`, `PeerAdded`, `PeerRemoved` and `PeerUpdated`, one `rstest` case each (see Changes After Review), on a fresh repository | the persistent and in-session totals read 0; the case name names the event |
| `it_should_persist_nothing_for_an_event_other_than_a_completed_download` | the same five `rstest` cases, on a fresh `TempDir` database | no torrent row for any info-hash, no global row, persisted total 0 |
| `it_should_not_increment_the_persisted_total_when_the_global_downloads_cannot_be_increased` | `MockTorrentMetricsStore`: the torrent write succeeds (`load_torrent_downloads` → `None`, `save_torrent_downloads` → `Ok`), `load_global_downloads` fails | the persisted total reads 0 |

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
| H1 | DONE | Add the three tests and remove the two replaced ones. | They pass; lines 19-58 and 129 run. |
| H2 | DONE | Run M1-M4. | Each is killed by its expected test. |
| H3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Prose-First AAA Review

Temporary prose, written from this plan's Test Design when the review of #3009 found this section
missing (it belongs before the completed-file review), then compared with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_increment_in_memory_completion_metrics_when_a_peer_download_is_completed` | A default statistics repository and a time. | Handle a completed-download event in memory. | The persistent and in-session totals read 1. | Matches the code. |
| `it_should_change_no_in_memory_total_for_an_event_other_than_a_completed_download` (cases) | The same, with one event of each other kind per case. | Handle the event in memory. | Both totals still read 0. | Matches the code. |
| `it_should_persist_completion_statistics_when_a_peer_download_is_completed` | A repository with persisted statistics enabled and a temporary database. | Handle a completed-download event. | The torrent and the global total read 1 in the database and the persisted total reads 1. | Matches the code. |
| `it_should_persist_nothing_for_an_event_other_than_a_completed_download` (cases) | The same, with one other event per case. | Handle the event. | No torrent, no global total and a persisted total of 0. | Matches the code. |
| `it_should_persist_global_completion_statistics_when_torrent_persistence_fails` | A mock store whose torrent read fails and whose global read and save succeed. | Handle a completed-download event. | The persisted total reads 1. | Matches the code. The mock expectations do not pin call counts (no `.times`), unlike the next test, so the Assert checks only the total. |
| `it_should_not_increment_the_persisted_total_when_the_global_downloads_cannot_be_increased` | A mock store that reads and saves the torrent but fails to read the global total, each expected once. | Handle a completed-download event. | The persisted total reads 0. | Matches the code. |

Smells found: the mock expectations of the fifth test have no call counts. The tests have AAA comments and one Act each.

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean
  coverage target directory, 2026-11-18): the file 175 / 196 lines to 311 / 311, regions 204 / 249
  to 264 / 289, functions 30 / 34 to 39 / 39 (the counts include the test module's helpers, which
  grew with the event table). The package's unit-only lines 4,553 / 5,450 (83.54%) to
  4,688 / 5,564 (84.26%), regions 4,993 / 6,056 (82.45%), functions 764 / 1,008 (75.79%). Every
  line runs; the regions left are the log-level checks of the `tracing` calls (lines 37, 41, 45,
  49, 57, 66, 106, 108, 120 and 129; at 129 the error arm runs, its log call does not): no decision.
- **Tests:** three added, two replaced by the tables (their `PeerAdded` case is kept as one row),
  three unchanged: six in the file, 177 unit tests in the package.
- **Mutations** (each applied alone and reverted; a mutant that did not compile was rewritten to
  keep calling the logging handler, which the `unused` lint requires):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the in-memory table: "PeerRemoved must not count as a completed download". It survived the tests before this change (checked against the committed file). |
  | M2 | Killed by the in-memory table: "PeerUpdated must not count as a completed download" |
  | M3 | Killed by the global-failure test |
  | M4 | Killed by the persistence table and by the existing completion test |

- **Review outcome:** the maintainer approved the completed file on 2026-11-18.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the
  existing `axum::serve` warning) and `cargo +nightly fmt --all -- --check` pass.

## Changes After Review (#2988)

- **`rstest` cases (finding F2 of #2988, maintainer decision of 2026-11-22):** the two tables loop
  no longer. Each event kind is an `rstest` case with its own name (`case_1_torrent_added` to
  `case_5_peer_updated`), as the `write-unit-test` skill prescribes, so each broken event kind fails
  under its own name: with M1 applied, only `case_4_peer_removed` fails. `rstest` 0.28.0, already in
  the lock file, became a dev-dependency of the package. The ten cases replace the two table tests:
  191 unit test cases in the package.
- **Any torrent row (Copilot, #2988):** the persistence cases expect no torrent row at all
  (`load_all_torrents_downloads()` is empty), not only no row for the sample info-hash; a mutant
  that writes a row under another info-hash fails all five cases.
- **Name (finding F3 of #2988):** the global failure test is
  `it_should_not_increment_the_persisted_total_when_the_global_downloads_cannot_be_increased`: its
  mock fails the read of the global downloads, before any save.
