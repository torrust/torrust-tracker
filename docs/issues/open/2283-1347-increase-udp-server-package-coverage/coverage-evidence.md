---
doc-type: coverage-evidence
issue: 2283
package: torrust-tracker-udp-server
measured-commit: cfb93157
measured-utc: 2026-09-22T06:41:00Z
---

# UDP Server Coverage Evidence

This document records the refreshed package-source baseline and complete module inventory for issue
2283. The reports were collected at the merge commit containing this issue specification, before
any Issue #2283 test or production change.

## Measurement Scope

`cargo-llvm-cov 0.6.16` generated three clean reports at `cfb93157`. Each report was filtered to
files below `packages/udp-server/src/`; raw JSON files remain ignored local artifacts in
`.tmp/2283-coverage/` because they are approximately 60 MB each.

```text
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-udp-server --all-features --json --output-path .tmp/2283-coverage/aggregate.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json --output-path .tmp/2283-coverage/unit.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-udp-server --all-features --test integration --json --output-path .tmp/2283-coverage/integration.json
```

The commands completed successfully. The last integration-only command ran the 11 package
real-loopback tests. The reports have different denominators and must not be combined.

## Test-Level Baseline

### Aggregate/Global Coverage

Aggregate coverage includes all selected package test binaries and test-support code. It is broad
navigation evidence only, not evidence that a seam has adequate unit coverage.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 5,566 / 5,688 (97.86%) | 7,281 / 7,534 (96.64%) | 578 / 592 (97.64%) |

### Unit-Only Coverage

Unit-only (`--lib`) coverage is the primary Issue #2283 metric. It represents the fast,
deterministic package-local safety net that this issue aims to improve where a maintainable unit
boundary exists.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 5,471 / 5,688 (96.18%) | 7,183 / 7,534 (95.34%) | 562 / 592 (94.93%) |

### Integration-Only Coverage

Integration-only (`--test integration`) coverage identifies the retained real UDP loopback
boundary. It is not a substitute for unit-only protection.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 1,118 / 1,469 (76.11%) | 1,190 / 1,654 (71.95%) | 148 / 187 (79.14%) |

## Module Inventory And T1 Hypotheses

Every source file has one T1 hypothesis. It orders the per-file work and records the ownership
reasoning at inventory time; it is **not** a terminal decision. Every file is still processed through
its own file test plan (see the issue's Per-File Workflow), and only that plan can conclude "no test
change" for the file. `property candidate` records a selection decision, not a requirement to add a
property-testing dependency.

| Source module | Unit-only lines | Property candidate | T1 hypothesis | Ownership rationale |
| --- | ---: | --- | --- | --- |
| `banning/event/handler.rs` | 81 / 82 (98.78%) | No | No change | Direct cookie-error IP forwarding and distinct-IP gauge contracts cover the package-owned handler decisions; residual behavior is logging or ban-service policy. |
| `banning/event/listener.rs` | 147 / 150 (98.00%) | No | Deferred with owner | Listener receive/termination and task cleanup are shutdown lifecycle concerns owned by #1488. |
| `banning/event/mod.rs` | No executable entries | No | No change | Module declaration wiring has no independent observable contract. |
| `banning/mod.rs` | No executable entries | No | No change | Namespace wiring has no independent observable contract. |
| `container.rs` | 59 / 59 (100.00%) | No | No change | Direct event-publication composition is fully unit-covered. |
| `error.rs` | 52 / 62 (83.87%) | No | Test selected | `From<UdpAnnounceError>` and `From<UdpScrapeError>` are package-owned deterministic adapters not covered by the existing parse-error tests. Review/refactor current tests before adding one focused conversion contract per variant. |
| `event.rs` | 125 / 131 (95.42%) | No | No change | Request/error classification and labels have direct contracts; residual formatting does not justify coverage-only tests. |
| `handlers/announce.rs` | 781 / 800 (97.63%) | No | No change | Tracker business rules belong to `udp-core`; retained package adapter coverage is already strong. |
| `handlers/connect.rs` | 251 / 251 (100.00%) | No | No change | Existing tests fully cover the package adapter. |
| `handlers/error.rs` | 155 / 178 (87.08%) | No | No change | Response/error-event routing is directly covered; residual logging, sender absence, and collaborator conversion do not create a clearer package contract. |
| `handlers/mod.rs` | 184 / 214 (85.98%) | No | No change | Packet dispatch has a direct unit contract; residual time construction, logging, and handler/core behavior are not an additional package-owned unit seam. |
| `handlers/scrape.rs` | 319 / 321 (99.38%) | No | No change | Tracker business rules belong to `udp-core`; adapter coverage is strong. |
| `lib.rs` | 28 / 29 (96.55%) | No | No change | Crate wiring and exported configuration contain no independent behavior gap. |
| `server/bound_socket.rs` | 55 / 65 (84.62%) | No | Deferred with owner | Current tests cover portable port allocation and metadata. Bind failure, socket debug failure, and dual-stack defaults require OS/platform fault injection and are not portable unit contracts. |
| `server/launcher.rs` | 278 / 290 (95.86%) | No | Deferred with owner | Remaining admission/task-management and shutdown behavior is owned by #1488 lifecycle policy. |
| `server/mod.rs` | 176 / 177 (99.44%) | No | No change | Startup and registration-failure cleanup already have package-owned coverage. |
| `server/processor.rs` | 180 / 180 (100.00%) | No | Test selected | Direct discard and response-publication events are stable positive processor outputs. Successful connect publication is covered here; #2354 owns parsed and unparsable error classification. |
| `server/receiver.rs` | 55 / 56 (98.21%) | No | No change | Datagram adaptation is directly covered; readiness failure and termination remain socket/lifecycle boundaries. |
| `server/request_buffer.rs` | 162 / 174 (93.10%) | Yes, eviction invariant | Deferred with owner | #2149 covered capacity, oldest-first eviction, and cleanup. Scheduler-sensitive completion races and active-request policy belong to #1488 SI-15; do not add a new property harness before that policy is settled. |
| `server/spawner.rs` | 17 / 17 (100.00%) | No | No change | Fully covered thin task-spawn wrapper; task lifecycle remains #1488-owned. |
| `server/states.rs` | 72 / 77 (93.51%) | No | Deferred with owner | Startup-notification mapping is covered. Bind/public-start and stop/task paths remain socket and #1488 lifecycle boundaries. |
| `statistics/event/handler/error.rs` | 131 / 151 (86.75%) | No | No change | Existing tests cover observable general and connection-cookie metric routing. Peer-client string mapping belongs to the peer-ID collaborator; remaining repository failures are logging-only. |
| `statistics/event/handler/mod.rs` | 19 / 21 (90.48%) | No | No change | Dispatcher routing is already indirect through specialized handlers; no isolated module decision is observable without asserting collaborator semantics. |
| `statistics/event/handler/request_aborted.rs` | 56 / 57 (98.25%) | No | No change | Existing metric mapping coverage is sufficient. |
| `statistics/event/handler/request_accepted.rs` | 162 / 163 (99.39%) | No | No change | Existing metric mapping coverage is sufficient. |
| `statistics/event/handler/request_banned.rs` | 56 / 57 (98.25%) | No | No change | Existing metric mapping coverage is sufficient. |
| `statistics/event/handler/request_discarded.rs` | 33 / 34 (97.06%) | No | No change | Existing metric mapping coverage is sufficient. |
| `statistics/event/handler/request_received.rs` | 33 / 34 (97.06%) | No | No change | Existing metric mapping coverage is sufficient. |
| `statistics/event/handler/response_sent.rs` | 122 / 130 (93.85%) | No | No change | Successful connect processing-average and parent response-count routes are covered; residual negative collaborator assertions are not selected. |
| `statistics/event/listener.rs` | 164 / 172 (95.35%) | No | Deferred with owner | Listener shutdown and receive-loop lifecycle remain #1488-owned. |
| `statistics/event/mod.rs` | No executable entries | No | No change | Module declaration wiring has no independent observable contract. |
| `statistics/metrics.rs` | 827 / 834 (99.16%) | Yes, metric arithmetic | No change | Existing focused metric tests cover arithmetic and aggregation; no new property harness is justified by the small residual gap. |
| `statistics/mod.rs` | 52 / 52 (100.00%) | No | No change | Metric declaration composition is fully covered through initialization and specialized metrics tests. |
| `statistics/repository.rs` | 547 / 549 (99.64%) | Yes, aggregation | No change | Repository concurrency and metric aggregation are already directly covered; residual lines do not expose a distinct behavior. |
| `statistics/services.rs` | 32 / 32 (100.00%) | No | No change | Service aggregation is fully covered. |
| `testing/environment.rs` | 168 / 184 (91.30%) | No | Deferred with owner | Fixture startup/stop, listener joining, and cleanup are explicitly owned by #1488 SI-14 and SI-17. |
| `testing/mod.rs` | No executable entries | No | No change | Test-module wiring has no independent observable contract. |

## Processing Order

The inventory hypotheses placed `error.rs` first. The full processing order and the per-file state
live in the issue's [Source File Ledger](ISSUE.md#source-file-ledger); the workflow and gates live in
its [Per-File Workflow](ISSUE.md#per-file-workflow). Per-file results are appended below as each
file completes.

## Per-File Results

| Source module | Unit-only lines before | Unit-only lines after | Tests refactored | Tests added | Integration selected |
| --- | ---: | ---: | ---: | ---: | --- |
| `error.rs` | 52 / 62 (83.87%) | 86 / 86 (100.00%) | 2 | 2 | No |
| `lib.rs` | 28 / 29 (96.55%) | 28 / 29 (96.55%) | 0 | 0 | No; crate-root declarations and test-support helper have no real UDP boundary |
| `container.rs` | 59 / 59 (100.00%) | 59 / 59 (100.00%) | 0 | 0 | No; enabled publication is already directly protected, while generic bus behavior and real UDP boundaries are owned elsewhere |
| `event.rs` | 125 / 131 (95.42%) | 190 / 190 (100.00%) | 1 | 4 | No; event classification has no real UDP boundary |
| `handlers/mod.rs` | 184 / 214 (85.98%) | 241 / 250 (96.40%) | 0 | 2 | No; remaining fixture-only and generated mock entries are not package-owned production decisions |
| `handlers/connect.rs` | 251 / 251 (100.00%) | 212 / 212 (100.00%) | 3 | 0 | No; R1 removes duplicate test code while retaining all direct adapter contracts |
| `handlers/announce.rs` | 781 / 800 (97.63%) | 903 / 905 (99.78%) | 2 | 2 | No; direct adapter contracts cover error routing and address-family response conversion, while the handler has no socket Act |
| `handlers/scrape.rs` | 319 / 321 (99.38%) | 356 / 357 (99.72%) | 3 | 1 | No; direct error-routing coverage and strict test extraction cover the handler, while it has no socket Act |
| `banning/mod.rs` | No executable entries | No executable entries | 0 | 0 | No; namespace declaration has no runtime boundary |

## Relationship To Issue #2149

Issue #2149 merged as `Merge torrust/torrust-tracker#2174: test(udp-server): [#2149] add focused
UDP server tests`. Its final evidence used an unavailable local measurement SHA (`8683d543`), so
this issue records the current reproducible baseline rather than treating that SHA as a load-bearing
reference. The aggregate and integration-only totals are nearly unchanged; unit-only coverage is
exactly 96.18% lines. The six-line aggregate denominator increase is therefore treated as a fresh
inventory trigger, not proof that prior no-change decisions remain correct.

## Next Evidence

- One `test-refactor-plans/<module>-tests.md` per source file, created when that file starts.
- `mutation-evidence.md`, `manual-verification-evidence.md`, and
  `implementation-retrospective.md` are created when their corresponding implementation stages
  produce evidence.
