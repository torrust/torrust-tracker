---
doc-type: spec
issue-type: task
status: planned
priority: p2
epic: 1938
github-issue: 1943
spec-path: docs/issues/open/1943-1938-si-5-deprecate-rest-api-core/ISSUE.md
last-updated-utc: 2026-06-24
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/drafts/rest-api-contract-first-migration/EPIC.md
    - packages/rest-api-core/
    - packages/rest-api-runtime-adapter/
    - packages/rest-api-application/
    - packages/rest-api-protocol/
    - packages/axum-rest-api-server/Cargo.toml
---

<!-- skill-link: create-issue -->

# SI-5: Deprecate `rest-api-core` and remove from workspace

## Subissue of REST API Contract-First Migration EPIC

## Problem

After SI-1 through SI-4 migrate all contexts to the contract-first architecture, the `rest-api-core` package (`torrust-tracker-rest-api-core`) becomes an empty shell:

| Current component                                        | Absorbed by                                                   |
| -------------------------------------------------------- | ------------------------------------------------------------- |
| `TrackerHttpApiCoreContainer` (DI wiring)                | `rest-api-runtime-adapter` adapters                           |
| `TorrentsMetrics`, `ProtocolMetrics` (metric types)      | `rest-api-protocol` DTOs                                      |
| `get_metrics()`, `get_labeled_metrics()` (orchestration) | `rest-api-application` use-cases + `rest-api-runtime-adapter` |

It has only **one consumer** in the entire workspace: `axum-rest-api-server`. Once that consumer is migrated (SI-4 removes the stats dependency), the crate is unused.

## Prerequisites

- [ ] SI-4 (stats migration) completed — this removes the last consumer of `rest-api-core` types from `axum-rest-api-server`.
- [ ] Verify no other crate in the workspace depends on `rest-api-core`.

## Scope

### In Scope

- Move any remaining useful types (metrics structs, if not already ported) to their target layers.
- Remove `torrust-tracker-rest-api-core` from `axum-rest-api-server/Cargo.toml`.
- Remove the crate from workspace `Cargo.toml` members list.
- Delete the `packages/rest-api-core/` directory.
- Remove any `deny.toml` wrapper rules referencing the crate.
- Verify no build/test breakage.

### Out of Scope

- Changing behaviour of existing stats endpoints (done in SI-4).

## Implementation Plan

| ID  | Status | Task                                                                              | Notes                         |
| --- | ------ | --------------------------------------------------------------------------------- | ----------------------------- |
| T1  | DONE   | Verify all ported types exist in target layers                                    | Must wait for SI-4 completion |
| T2  | DONE   | Remove `torrust-tracker-rest-api-core` dep from `axum-rest-api-server/Cargo.toml` |                               |
| T3  | DONE   | Remove crate from workspace `Cargo.toml` members                                  |                               |
| T4  | DONE   | Delete `packages/rest-api-core/` directory                                        |                               |
| T5  | DONE   | Update `deny.toml` if crate had wrapper rules                                     |                               |
| T6  | DONE   | Run pre-commit and pre-push checks                                                |                               |

## Verification / Progress

- [x] No crate in workspace references `torrust-tracker-rest-api-core`
- [ ] Workspace builds cleanly
- [ ] Integration tests pass
- [x] Pre-commit checks pass
- [ ] Pre-push checks pass

## Manual Verification

Before committing, manually verify the REST API works correctly after removing `rest-api-core`:

1. **Run the tracker locally** with the REST API enabled:

   ```console
   cargo run
   ```

2. **Make test requests**:
   - Request the stats endpoint:

     ```console
     curl http://localhost:1212/api/v1/stats
     ```

   - Request the metrics endpoint:

     ```console
     curl http://localhost:1212/api/v1/metrics
     ```

   - Make an announce request using the tracker client:

     ```console
     cargo run -p torrust-tracker-client --bin tracker_client -- udp announce udp://localhost:6969/announce 0123456789abcdef0123456789abcdef01234567
     ```

3. **Verify stats and metrics changed**:
   - Repeat the `/api/v1/stats` request and confirm the values changed (and verify in the tracker console logs that the request was received).

   - Repeat the `/api/v1/metrics` request and confirm the values changed (and verify in the tracker console logs that the request was received).

### Progress Log

| Date       | Event                                                                                      |
| ---------- | ------------------------------------------------------------------------------------------ |
| 2026-06-24 | Draft spec created                                                                         |
| 2026-06-29 | Implementation confirmed: move `TrackerHttpApiCoreContainer` to `rest-api-runtime-adapter` |
| 2026-06-29 | Implementation: container moved, deps removed, directory deleted                           |
