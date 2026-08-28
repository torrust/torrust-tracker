---
doc-type: defect-investigation
issue: 2107
status: confirmed
last-updated-utc: 2026-08-28
---

# Download Counter Semantics Investigation

## Confirmed Defect

`tracker_core_persistent_torrents_downloads_total` is an in-memory metrics
repository counter whose name and description claim that its value is
persisted. Its event handler increments it for every `PeerDownloadCompleted`
event whenever the tracker-core event listener runs. The listener is started
when either tracker usage statistics or persistent completed statistics is
enabled.

When persistent completed statistics are enabled with tracker usage statistics,
the intended startup path loads the database aggregate into this in-memory
counter and the persistent listener updates the database aggregate. That
represents a total retained across process restarts.

When persistence is disabled but tracker usage statistics is enabled, the same
in-memory counter is incremented but cannot survive restart. If the counter is
exported as the persisted total, its current name and description are false.

The retention behavior is intentional. Commit `b0e74439` records that the
tracker-core metric is available regardless of persistence: it contains the
session download count without database persistence and a restored historical
count when persistence is enabled. The defect is the metric name, its
description, and the REST `completed` field documentation, which do not state
that distinction.

## Known Evidence

- `packages/tracker-core/src/statistics/mod.rs` defines the counter as
  `tracker_core_persistent_torrents_downloads_total` and describes it as "The
  total number of torrent downloads (persisted)."
- `packages/tracker-core/src/statistics/persisted/mod.rs` loads the database
  aggregate into that in-memory counter at persistence-enabled startup.
- `packages/tracker-core/src/statistics/event/handler.rs` currently increments
  that in-memory counter for every completed-download event before conditional
  database writes.
- `src/bootstrap/jobs/tracker_core.rs` currently starts one event listener when
  either `tracker_usage_statistics` or
  `tracker_policy.persistent_torrent_completed_stat` is enabled.
- `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs` exposes this
  counter as `Stats.completed` for `GET /api/v1/stats`.
- `packages/rest-api-protocol/src/v1/context/stats/resources/stats.rs`
  describes `completed` as the total number of peers that have ever completed
  downloading, without a persistence-free qualification.
- `GET /api/v1/metrics` exposes the labeled counter, including its current
  `persistent` name and persisted description.
- Commit `b0e74439` states that the counter is deliberately session-scoped
  when download persistence is disabled and persistent when it is enabled.

## Questions To Confirm

1. [x] Which REST API responses, metrics exporters, or internal consumers
       expose `Repository::get_torrents_downloads_total`? `GET /api/v1/stats`
       maps it to `completed`; `GET /api/v1/metrics` exports the labeled
       tracker-core metric.
2. [x] Does a no-persistence tracker intentionally expose a session-only
       completed total? Yes; the behavior is recorded in `b0e74439`.
3. [ ] Add a regression test proving that a restart without persistence resets
       the exposed total to zero.
4. [x] Does a restart with persistence enabled restore the exposed total?
       Yes; tracker-core integration coverage verifies restoration.
5. [x] Does tracker usage statistics require event-derived completed-download
       metrics? Yes; the listener's only observed in-memory output is the
       session completed-download counter used by the stats endpoints.

## Candidate Resolution

Retain one exported in-memory counter whose value is session-scoped without
persistence and restored from database persistence when enabled. Give its
description and REST field documentation an explicit retention contract.

Renaming the current labeled metric would change a public Prometheus-style
identifier and must be treated as a compatibility decision. First establish
whether v3 permits that rename or whether the existing name must remain while
its description is corrected. Do not expose a second counter or change REST
response shapes in this work. The #2107 scope already permits process-local
completed counts in persistence-free operation.

## Listener Design Decision

The existing listener appears to have two responsibilities:

- update the in-memory completed-download counter for tracker usage statistics;
- update per-torrent and global database counters when persistent completed
  statistics are enabled.

After confirming actual consumers, compare these options:

| Option                                               | Benefits                                                                                                   | Costs                                                                                                                   | Status   |
| ---------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- | -------- |
| Retain one listener with explicit persistence branch | One receiver and task; preserves event order in one handler.                                               | Persistence branch remains in a usage-statistics listener and must receive an optional dependency.                      | Rejected |
| Split in-memory and persistence listeners            | Each listener receives only its concrete dependencies; no database work/task when persistence is disabled. | Two receivers/tasks when both features are enabled; duplicated event filtering and more bootstrap lifecycle management. | Selected |
| Start only a persistence listener                    | Avoids the listener without persistence.                                                                   | Incorrect because tracker usage statistics needs the session counter.                                                   | Rejected |

## Plan

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `BLOCKED`, `NOT_APPLICABLE`.

| ID  | Status | Task                                                                                               | Evidence / Completion Condition                                                                                   |
| --- | ------ | -------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| D1  | DONE   | Record the potential counter semantics defect separately from the persistence-capability refactor. | This document.                                                                                                    |
| D2  | DONE   | Trace the counter's API, exporter, internal consumers, and historical intent.                      | `GET /api/v1/stats` and `GET /api/v1/metrics` expose it; `b0e74439` confirms session-versus-persistent semantics. |
| D3  | TODO   | Add a focused persistence-free restart regression test.                                            | Demonstrate that the exposed count resets to zero.                                                                |
| D4  | DONE   | Confirm whether tracker usage statistics requires event-derived completed-download metrics.        | The listener maintains the session counter exposed by stats endpoints.                                            |
| D5  | DONE   | Decide listener topology from D2-D4.                                                               | Selected independent in-memory and persistence listeners; implementation is tracked by P2-P4.                     |
| D6  | TODO   | Decide the labeled metric compatibility strategy.                                                  | Approve a v3 rename or retain the identifier with corrected descriptions.                                         |
| D7  | TODO   | Implement the confirmed counter naming/documentation fix and listener refactor, if required.       | Focused tests and updated documentation.                                                                          |
| D8  | TODO   | Run applicable checks and record results.                                                          | Relevant tests, formatting, linting, and manual evidence when necessary.                                          |

## Non-Goals

- Do not merge this investigation with the persistence-capability refactor.
- Do not change REST response shapes or add historical-data provenance in this
  task.
- Do not assume the current metric behavior is a bug until D2-D4 are complete.

## Progress Log

- 2026-08-28 - Created after discovering that the metric named and described
  as persisted is incremented in memory even for a no-persistence runtime. The
  behavior may correctly represent a session total, but its naming or exposed
  semantics may be defective. Investigation precedes implementation.
- 2026-08-28 - Confirmed the retention behavior through public API/export
  tracing and commit `b0e74439`: session total without persistence, restored
  historical total with it. The defect is the inaccurate metric name and
  descriptions. The listener must remain active for tracker usage statistics;
  splitting database writes into a second listener remains a performance and
  complexity trade-off, not a prerequisite for correcting the documentation.
- 2026-08-28 - Selected two independent listeners for the target runtime:
  tracker usage statistics starts the mandatory in-memory listener; persistent
  completed statistics starts an additional listener with a concrete database
  repository. Statistics disabled starts neither listener. Implementation is
  intentionally deferred until the committed persistence-capability plan is
  reviewed.
