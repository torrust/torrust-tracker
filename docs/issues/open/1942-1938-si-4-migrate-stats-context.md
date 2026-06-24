---
doc-type: spec
issue-type: task
status: planned
priority: p1
epic: 1938
github-issue: 1942
spec-path: docs/issues/open/1942-1938-si-4-migrate-stats-context.md
last-updated-utc: 2026-06-24
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/drafts/rest-api-contract-first-migration/EPIC.md
    - packages/axum-rest-api-server/src/v1/context/stats/
    - packages/rest-api-protocol/src/v1/
    - packages/rest-api-application/src/
    - packages/rest-api-runtime-adapter/src/
    - packages/tracker-core/src/statistics/
    - packages/http-core/src/statistics/
    - packages/udp-core/src/statistics/
    - packages/udp-server/src/statistics/
    - packages/swarm-coordination-registry/src/statistics/
    - packages/rest-api-core/src/statistics/
    - packages/axum-rest-api-server/src/v1/routes.rs
    - packages/axum-rest-api-server/src/main.rs
---

<!-- skill-link: create-issue -->

# SI-4: Migrate `stats` context to contract-first architecture

## Subissue of REST API Contract-First Migration EPIC

## Problem

The `stats` context is the most complex in the REST API. It has two endpoints (`GET /stats`, `GET /metrics`) that aggregate data from **6+ tracker internal repositories/services** across `tracker-core`, `http-core`, `udp-core`, `udp-server`, `swarm-coordination-registry`, and `rest-api-core`.

The `Stats` response DTO has ~28 fields. The `metrics` endpoint produces Prometheus-formatted plaintext. The Axum server injects all these dependencies as a multi-element state tuple.

Per the contract-first architecture, this context needs:

- A `Stats` DTO (~28 fields) in `rest-api-protocol`.
- A stats query port in `rest-api-application`.
- A `TrackerStatsAdapter` that aggregates data from all internal repositories.
- A Prometheus serialization concern that should be separated from the DTO definition.

## Current State

**Location**: `packages/axum-rest-api-server/src/v1/context/stats/`

| Artifact                 | Details                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Handlers                 | 2: `get_stats_handler`, `get_metrics_handler`                                                                                                                                                                                                                                                                                                                                                                                             |
| Routes                   | 2: `GET /stats`, `GET /metrics`                                                                                                                                                                                                                                                                                                                                                                                                           |
| Local DTOs               | `Stats` (28 fields), `LabeledStats`, `Format` (JSON/Prometheus), `QueryParams`                                                                                                                                                                                                                                                                                                                                                            |
| Response types           | 4: `stats_response`, `metrics_response` (Prometheus plaintext), `labeled_stats_response`, `labeled_metrics_response`                                                                                                                                                                                                                                                                                                                      |
| Tracker deps (6+ crates) | `tracker_core::InMemoryTorrentRepository`, `tracker_core::statistics::repository::Repository`, `http_core::statistics::repository::Repository`, `udp_core::services::banning::BanService`, `udp_core::statistics::repository::Repository`, `udp_server::statistics::repository::Repository`, `swarm_coordination_registry::statistics::repository::Repository`, `rest_api_core::statistics::services::{get_metrics, get_labeled_metrics}` |

### DTO Complexity: `Stats` fields

The current `Stats` struct has approximately 28 fields covering:

- Torrent stats (total torrents, seeds, peers, leechers)
- Protocol breakdowns (TCP vs UDP)
- Per-protocol connection metrics
- Ban/block list stats
- Per-repository breakdowns

Two output formats are supported: JSON (serialize `Stats` struct) and Prometheus (plaintext key-value format with TYPE/HELP headers).

## Scope

### In Scope

- Define `Stats` DTO (~28 fields) and `LabeledStats` DTO in `rest-api-protocol/src/v1/resources/stats.rs`.
- Define `StatsQueryPort` trait in `rest-api-application/src/ports/` (methods: `get_stats`, `get_labeled_stats`).
- Implement `StatsApiService` use-case in `rest-api-application/src/use_cases/`.
- Implement `TrackerStatsAdapter` in `rest-api-runtime-adapter/src/adapters/`.
  - Consumes UDP-side traits from SI-30 (`BanningStats`, `UdpCoreStatsRepository`, `UdpServerStatsRepository`).
  - Wraps all 6+ internal repository/services.
- Handle Prometheus serialization:
  - Option A: Keep Prometheus formatting in the Axum server as a response serializer (since it's a transport concern).
  - Option B: Move to `rest-api-runtime-adapter` if formatting logic needs access to internal types.
- Rewire Axum handlers to use `StatsApiService`.
- Remove direct internal dependencies from `axum-rest-api-server` stats wiring.
- Verify no behavioural change.

### Out of Scope

- Changing the stats data model or field semantics.
- Adding new stats aggregation logic.
- Performance optimization of the stats aggregation.

## Design Considerations

### Prometheus Serialization

The `get_metrics` and `get_labeled_metrics` functions in `rest-api-core/src/statistics/services.rs` currently produce Prometheus-formatted strings by calling into tracker-internal repositories. The Prometheus format is a transport-level serialization concern.

Two options for where to put Prometheus formatting:

**Option A (preferred)**: Keep Prometheus formatting as a transport concern in `axum-rest-api-server`. The use-case returns protocol DTOs, and the Axum handler converts to Prometheus format. This keeps the application layer clean.

**Option B**: Move Prometheus formatting to `rest-api-runtime-adapter` if the formatting logic requires internal type access that can't be surfaced through port traits.

The UDP-side traits from SI-30 (`BanningStats`, `UdpCoreStatsRepository`, `UdpServerStatsRepository`) are designed to abstract the internal repository access, so Option A should be feasible.

### Stats Query Port Shape

The port trait should expose methods that return protocol DTOs:

```rust
#[async_trait]
pub trait StatsQueryPort {
    async fn get_stats(&self) -> Stats;
    async fn get_labeled_stats(&self) -> LabeledStats;
}
```

The use-case maps domain errors to protocol error codes and returns protocol DTOs.

## Implementation Plan

| ID  | Status | Task                                                                                    | Notes                               |
| --- | ------ | --------------------------------------------------------------------------------------- | ----------------------------------- |
| T1  | TODO   | Define `Stats` and `LabeledStats` DTOs in `rest-api-protocol/src/v1/resources/stats.rs` | Match current serialization exactly |
| T2  | TODO   | Define `StatsQueryPort` trait in `rest-api-application/src/ports/`                      |                                     |
| T3  | TODO   | Implement `StatsApiService` use-case in `rest-api-application/src/use_cases/`           |                                     |
| T4  | TODO   | Implement `TrackerStatsAdapter` in `rest-api-runtime-adapter/src/adapters/`             | Consume SI-30 traits                |
| T5  | TODO   | Add conversion functions for domain→protocol stats types                                |                                     |
| T6  | TODO   | Handle Prometheus serialization — keep as transport concern in Axum (Option A)          |                                     |
| T7  | TODO   | Rewire Axum handlers to use `StatsApiService`                                           |                                     |
| T8  | TODO   | Update Axum state to inject `TrackerStatsAdapter` (replacing 6+ tuples)                 |                                     |
| T9  | TODO   | Remove direct internal deps from `axum-rest-api-server` stats wiring                    |                                     |
| T10 | TODO   | Verify pre-commit and pre-push checks pass                                              |                                     |

## Verification / Progress

- [ ] `Stats` and `LabeledStats` DTOs defined in `rest-api-protocol`
- [ ] `StatsQueryPort` trait defined in `rest-api-application`
- [ ] `StatsApiService` use-case implemented
- [ ] `TrackerStatsAdapter` implemented consuming SI-30 traits
- [ ] Prometheus serialization handled appropriately
- [ ] Axum handlers dispatch through use-case
- [ ] Direct internal crate deps removed from Axum server stats wiring
- [ ] Pre-commit checks pass
- [ ] Pre-push checks pass

### Progress Log

| Date       | Event              |
| ---------- | ------------------ |
| 2026-06-24 | Draft spec created |
