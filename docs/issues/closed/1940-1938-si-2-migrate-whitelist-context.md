---
doc-type: spec
issue-type: task
status: done
priority: p1
epic: 1938
github-issue: 1940
spec-path: docs/issues/closed/1940-1938-si-2-migrate-whitelist-context.md
last-updated-utc: 2026-06-26
updated-reason: Closed — issue implemented
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/1938-rest-api-contract-first-migration/EPIC.md
    - packages/axum-rest-api-server/src/v1/context/whitelist/
    - packages/rest-api-protocol/src/v1/
    - packages/rest-api-application/src/
    - packages/rest-api-runtime-adapter/src/
    - packages/tracker-core/src/whitelist/
    - packages/axum-rest-api-server/src/v1/routes.rs
    - packages/axum-rest-api-server/src/main.rs
    - packages/axum-rest-api-server/src/v1/state.rs
---

<!-- skill-link: create-issue -->

# SI-2: Migrate `whitelist` context to contract-first architecture

## Subissue of REST API Contract-First Migration EPIC

## Problem

The `whitelist` context (`add_torrent_to_whitelist`, `remove_torrent_from_whitelist`, `reload_whitelist` handlers) in `axum-rest-api-server` currently calls `tracker_core::whitelist::manager::WhitelistManager` directly. It has no protocol DTOs, no port trait, and no use-case service.

Per the contract-first architecture, the migration needs to:

- Define a whitelist command port in `rest-api-application`.
- Implement a runtime adapter wrapping `WhitelistManager`.
- Rewire Axum handlers to dispatch through the use-case service.

All protocol types follow the normalized context-based module structure under `packages/rest-api-protocol/src/v1/context/`:

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

See the `torrent` and `health_check` contexts for the reference pattern.

## Current State

**Location**: `packages/axum-rest-api-server/src/v1/context/whitelist/`

| Artifact       | Details                                                                                                    |
| -------------- | ---------------------------------------------------------------------------------------------------------- |
| Handlers       | 3: `add_torrent_to_whitelist_handler`, `remove_torrent_from_whitelist_handler`, `reload_whitelist_handler` |
| Routes         | 3: `POST /whitelist/{info_hash}`, `DELETE /whitelist/{info_hash}`, `GET /whitelist/reload`                 |
| Response types | 3 error response functions + shared `ok_response`                                                          |
| Tracker deps   | `torrust_tracker_core::whitelist::manager::WhitelistManager`                                               |
| Protocol DTOs  | None needed (no forms/request bodies — only path params and success/error responses)                       |

The whitelist context is simpler than `auth_key` because it has no request body forms — only `InfoHash` path parameters and success/error responses.

## Analysis

The whitelist operations are pure commands (no query/read operations):

- `add_torrent_to_whitelist(info_hash)` → success or error
- `remove_torrent_from_whitelist(info_hash)` → success or error
- `reload_whitelist()` → success or error

This maps naturally to a single port trait with three methods. The `ActionStatus` response enum already defined in `rest-api-protocol` can be reused for success/error responses.

## Scope

### In Scope

- Define `WhitelistCommandPort` trait in `rest-api-application/src/ports/`.
- Implement `WhitelistApiService` use-case in `rest-api-application/src/use_cases/`.
- Implement `TrackerWhitelistAdapter` in `rest-api-runtime-adapter/src/adapters/`.
- Add any needed protocol DTOs to `rest-api-protocol` (likely minimal — response types can reuse `ActionStatus`).
- Rewire Axum handlers to use `WhitelistApiService`.
- Update Axum state/routes to wire the new adapter.
- Verify no behavioural change.

### Out of Scope

- Adding new whitelist operations.
- Changing error response format.

## Implementation Plan

| ID  | Status | Task                                                                               | Notes                                     |
| --- | ------ | ---------------------------------------------------------------------------------- | ----------------------------------------- |
| T1  | DONE   | Add `WhitelistCommandPort` to `rest-api-application/src/ports/`                    | Three methods matching current operations |
| T2  | DONE   | Add `WhitelistApiService` to `rest-api-application/src/use_cases/`                 | Calls port trait, maps errors             |
| T3  | DONE   | Add domain→protocol error mapping for whitelist errors                             | `WhitelistError` in protocol package      |
| T4  | DONE   | Implement `TrackerWhitelistAdapter` in `rest-api-runtime-adapter/src/adapters/`    | Wraps `WhitelistManager`                  |
| T5  | DONE   | Add conversion functions to `rest-api-runtime-adapter/src/conversion.rs` if needed | Not needed — adapter maps inline          |
| T6  | DONE   | Update Axum handlers to use `WhitelistApiService`                                  |                                           |
| T7  | DONE   | Update Axum state to inject `TrackerWhitelistAdapter`                              | In `v1/routes.rs`                         |
| T8  | DONE   | Verify pre-commit and pre-push checks pass                                         |                                           |

## Verification / Progress

- [x] `WhitelistCommandPort` trait defined in `rest-api-application`
- [x] `WhitelistApiService` use-case implemented
- [x] `TrackerWhitelistAdapter` implemented in `rest-api-runtime-adapter`
- [x] Axum handlers dispatch through use-case instead of direct `WhitelistManager`
- [x] Pre-commit checks pass
- [x] Pre-push checks pass

### Progress Log

| Date       | Event                                                     |
| ---------- | --------------------------------------------------------- |
| 2026-06-24 | Draft spec created                                        |
| 2026-06-25 | Whitelist context migrated to contract-first architecture |
