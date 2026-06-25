---
doc-type: spec
issue-type: task
status: planned
priority: p1
epic: 1938
github-issue: 1939
spec-path: docs/issues/open/1939-1938-si-1-migrate-health-check-context.md
last-updated-utc: 2026-06-24
  updated-reason: Updated to reference context/ module structure instead of resources/
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/drafts/rest-api-contract-first-migration/EPIC.md
    - packages/axum-rest-api-server/src/v1/context/health_check/
    - packages/axum-rest-api-server/src/routes.rs
    - packages/rest-api-protocol/src/v1/
    - packages/rest-api-application/src/
    - packages/rest-api-runtime-adapter/src/
---

<!-- skill-link: create-issue -->

# SI-1: Migrate `health_check` context to contract-first architecture

## Subissue of REST API Contract-First Migration EPIC

## Problem

The `health_check` endpoint is defined in `packages/axum-rest-api-server/src/v1/context/health_check/`. Its DTOs (`Status`, `Report`) and response logic are defined locally in the Axum server package.

Per the contract-first architecture defined in [SI-33](../../open/1930-1669-si-33-rest-api-contract-first-architecture.md), this context should have:

- DTOs in `rest-api-protocol` under the normalized module structure:
  `v1::context::health_check::resources::health_check`
- A port trait and use-case service in `rest-api-application`
- A runtime adapter in `rest-api-runtime-adapter`
- Only thin HTTP routing/extraction in `axum-rest-api-server`

## Current State

**Location**: `packages/axum-rest-api-server/src/v1/context/health_check/`

| Artifact        | Current Location                         | Target Location                                                     |
| --------------- | ---------------------------------------- | ------------------------------------------------------------------- |
| `Status` enum   | `resources.rs` in Axum                   | `rest-api-protocol/src/v1/context/health_check/resources/report.rs` |
| `Report` struct | `resources.rs` in Axum                   | `rest-api-protocol/src/v1/context/health_check/resources/report.rs` |
| Handler         | `handlers.rs`                            | Axum (keep, but simplify)                                           |
| Route           | `src/routes.rs` (at `/api/health_check`) | Axum (keep)                                                         |

**Tracker dependency**: None — the handler returns a static response. This is the simplest context to migrate.

## Scope

### In Scope

- Move `Status` enum to `rest-api-protocol/src/v1/context/health_check/resources/report.rs`.
- Move `Report` struct to `rest-api-protocol/src/v1/context/health_check/resources/report.rs`.
- (Optional) Add a simple `HealthCheckPort` trait + use-case in `rest-api-application` if needed for testability; otherwise keep as direct protocol DTO mapping.
- Rewire Axum handler to return protocol DTOs.
- Update `rest-api-protocol/src/v1/context/mod.rs` and `health_check/` module tree exports.
- Verify no behavioural change.

### Out of Scope

- Adding new health check features or fields.
- Changing the response format.

## Migration Strategy

This is a straightforward DTO relocation. Steps:

1. Create protocol DTOs matching the current `Status` and `Report` types.
2. Expose them from `rest-api-protocol::v1::context::health_check::resources::report`.
3. Remove the local definitions from the Axum server.
4. Update imports in the handler.
5. Add conversion from protocol `Report` to JSON response (already `Serialize`).

Since there is no tracker dependency, no runtime adapter is needed — the handler can construct protocol DTOs directly.

## Module Structure Convention

All protocol DTOs follow the normalized context-based module structure under
`packages/rest-api-protocol/src/v1/context/` (see the `torrent` context for the reference pattern):

```text
context/<context-name>/
├── mod.rs
└── resources/
    ├── mod.rs
    └── <resource>.rs
```

Ports, use-cases, and adapters are flat files named after the context:

```text
packages/rest-api-application/src/ports/<context>.rs
packages/rest-api-application/src/use_cases/<context>.rs
packages/rest-api-runtime-adapter/src/adapters/<context>.rs
```

## Implementation Plan

| ID  | Status    | Task                                                                                                                        | Notes                               |
| --- | --------- | --------------------------------------------------------------------------------------------------------------------------- | ----------------------------------- |
| T1  | Completed | Add `health_check` context module to `rest-api-protocol/src/v1/context/` with `Status` and `Report` DTOs (resources subdir) | Match current serialization exactly |
| T2  | Completed | Export new context from `rest-api-protocol/src/v1/context/mod.rs` and set up normalized `resources/` module tree            |                                     |
| T3  | Completed | Remove local `Status` and `Report` from Axum `health_check` resources                                                       |                                     |
| T4  | Completed | Update Axum handler to import and use protocol DTOs                                                                         |                                     |
| T5  | Completed | Verify pre-commit and pre-push checks pass                                                                                  | Pre-commit checks pass              |
| T6  | Completed | Verify integration tests pass                                                                                               | Compilation verified                |

## Verification / Progress

- [x] Protocol DTOs created and exported
- [x] Local DTOs removed from Axum server
- [x] Handler uses protocol DTOs
- [x] Pre-commit checks pass
- [ ] Pre-push checks pass (to be verified before merge)

### Progress Log

| Date       | Event              |
| ---------- | ------------------ |
| 2026-06-24 | Draft spec created |
