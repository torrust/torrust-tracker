---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/statistics/persisted/downloads.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/statistics/persisted/downloads.rs
    - packages/tracker-core/src/databases/traits/torrent_metrics.rs
    - packages/tracker-core/src/statistics/event/handler.rs
---

# Statistics Persisted Downloads File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/tracker-core/src/statistics/persisted/downloads.rs`, ledger row 6.

## Current State

- **Unit-only coverage:** 77 / 77 lines at the T1 baseline, with the `?` of
  `increase_global_downloads` (line 133) as the only uncovered region. After row 5 the file is
  79 / 79 lines and 114 / 114 regions (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`,
  `--all-features --lib`, 2026-11-18): the global-save failure test of the handler reaches line 133.
- **Existing tests:** 3 in the file, over a `TempDir` SQLite database: save a torrent's downloads,
  increase a torrent's downloads once from nothing, load all torrents' downloads.
- **Module-owned decisions:** `increase_downloads_for_torrent` and `increase_global_downloads` each
  read the stored value first; with none they save 1, with one they increment it; a failed read is
  returned as the error, and nothing is written.

## Mutation Probe

Coverage is complete, so six hand-made mutants were run against the whole unit suite before
planning (2026-11-18):

| ID | Mutation | Result |
| --- | --- | --- |
| P1 | The `?` of the global read swallowed (`.ok().flatten()`) | Killed by the handler's global-save failure test |
| P2 | The `?` of the torrent read swallowed | Killed by the handler's torrent-failure test |
| P3 | A stored global total is overwritten with 1 instead of incremented | Killed only by a listener test that drains two events |
| P4 | A stored torrent total is overwritten with 1 instead of incremented | **Survived** |
| P5 | The first global total is saved as 0 | Killed by the handler and listener tests |
| P6 | The first torrent total is saved as 0 | Killed, also by this file's increase test |

P4 is a gap: no test increases the same torrent's downloads twice. P3 is caught only far from this
file.

## Maintainer Decision

Plan approved on 2026-11-18, without direct error-propagation tests.

## Test Design

Both tests use the file's `TempDir` SQLite fixture (`ephemeral_configuration()`, kept by the test).

| Test | Act | Assert |
| --- | --- | --- |
| `it_should_increment_the_stored_downloads_of_a_torrent` | `increase_downloads_for_torrent` twice for the same info-hash | the torrent's stored downloads are 2 |
| `it_should_save_then_increment_the_global_downloads` | `increase_global_downloads` twice | `load_global_downloads` gives `Some(1)` after the first call and `Some(2)` after the second |

**Not added:** direct error-propagation tests for the two reads (P1, P2). The handler tests already
kill both mutants with a mocked failing store, and the handler is the only caller.

## Property-Test Decision

No `quickcheck`: the rule is "first save 1, then add 1", which two examples pin; the counter's
overflow belongs to the drivers (S2).

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| D1 | TODO | Add the two tests. | They pass. |
| D2 | TODO | Rerun P3 and P4. | Both are killed by the new tests in this file. |
| D3 | TODO | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

To be filled in after D3.
