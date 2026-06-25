---
doc-type: spec
issue-type: task
status: planned
priority: p1
epic: 1938
github-issue: 1941
spec-path: docs/issues/open/1941-1938-si-3-migrate-auth-key-context.md
last-updated-utc: 2026-06-24
  updated-reason: Updated paths to context/ and added module structure convention note
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/drafts/rest-api-contract-first-migration/EPIC.md
    - packages/axum-rest-api-server/src/v1/context/auth_key/
    - packages/rest-api-protocol/src/v1/
    - packages/rest-api-application/src/
    - packages/rest-api-runtime-adapter/src/
    - packages/tracker-core/src/authentication/
    - packages/axum-rest-api-server/src/v1/routes.rs
    - packages/axum-rest-api-server/src/main.rs
    - packages/axum-rest-api-server/src/v1/state.rs
    - packages/clock/
---

<!-- skill-link: create-issue -->

# SI-3: Migrate `auth_key` context to contract-first architecture

## Subissue of REST API Contract-First Migration EPIC

## Problem

The `auth_key` context in `axum-rest-api-server` manages authentication keys for private-mode HTTP trackers. It has 4 handlers (`add_auth_key`, `generate_auth_key`, `delete_auth_key`, `reload_keys`) that call `tracker_core::authentication::{Key, AddKeyRequest, KeysHandler}` directly.

The context has locally-defined DTOs (`AuthKey`, `AddKeyForm`, `KeyParam`) and 7 response functions. Per the contract-first architecture, these should live in `rest-api-protocol`.

## Current State

**Location**: `packages/axum-rest-api-server/src/v1/context/auth_key/`

| Artifact       | Details                                                                                                                                                                                                                                 |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Handlers       | 4: `add_auth_key_handler`, `generate_auth_key_handler`, `delete_auth_key_handler`, `reload_keys_handler`                                                                                                                                |
| Routes         | 3 unique paths: `POST /key/{param}` + `DELETE /key/{param}` (shared route), `POST /keys`, `GET /keys/reload`                                                                                                                            |
| Local DTOs     | `AuthKey` (struct: `key`, `valid_until` (deprecated), `expiry_time`), `AddKeyForm` (struct with `serde_as` `DefaultOnNull`), `KeyParam` (wrapper)                                                                                       |
| Response types | 7 functions: `auth_key_response`, `failed_to_generate_key_response`, `failed_to_add_key_response`, `failed_to_delete_key_response`, `failed_to_reload_keys_response`, `invalid_auth_key_response`, `invalid_auth_key_duration_response` |
| Tracker deps   | `tracker_core::authentication::{Key, AddKeyRequest, KeysHandler}`                                                                                                                                                                       |
| Other deps     | `torrust_clock::convert_from_iso_8601_to_timestamp`                                                                                                                                                                                     |

## Scope

### In Scope

- Move `AuthKey`, `AddKeyForm`, `KeyParam` DTOs to `rest-api-protocol/src/v1/context/auth_key/resources/auth_key.rs`.
- Add auth-key-specific response/error DTOs to protocol (or reuse `ActionStatus` where applicable).
- Define `AuthKeyCommandPort` trait in `rest-api-application/src/ports/`.
- Implement `AuthKeyApiService` use-case in `rest-api-application/src/use_cases/`.
- Implement `TrackerAuthKeyAdapter` in `rest-api-runtime-adapter/src/adapters/`.
- Add conversion functions for domain→protocol types.
- Rewire Axum handlers to use `AuthKeyApiService`.
- Verify no behavioural change.

### Out of Scope

- Changing the auth key data model or validation rules.
- Adding new auth key operations.

## Analysis

The auth key context has both command and query operations, and includes form validation (duration parsing via `clock`). The 7 response functions produce 4 distinct error types plus a success response. Some can be consolidated into protocol-level error codes.

All protocol DTOs follow the normalized context-based module structure under `packages/rest-api-protocol/src/v1/context/`:

```text
context/auth_key/
├── mod.rs               # pub mod resources;
└── resources/
    ├── mod.rs           # pub mod auth_key;
    └── auth_key.rs      # AuthKey, AddKeyForm, DTOs
```

Ports, use-cases, and adapters are flat files named after the context:

```text
packages/rest-api-application/src/ports/auth_key.rs
packages/rest-api-application/src/use_cases/auth_key.rs
packages/rest-api-runtime-adapter/src/adapters/auth_key.rs
```

See the `torrent` and `health_check` contexts for the reference pattern.

**Key considerations**:

- `KeyParam` is a path parameter wrapper — it may stay in Axum as an extractor while referencing protocol DTOs.
- Duration validation (`convert_from_iso_8601_to_timestamp`) is in `torrust-clock` — the runtime adapter can call it.
- The `AuthKey` response DTO already has a reference pattern from torrent's `Peer`/`Torrent` DTOs.

## Implementation Plan

| ID  | Status | Task                                                                                                       | Notes                         |
| --- | ------ | ---------------------------------------------------------------------------------------------------------- | ----------------------------- |
| T1  | TODO   | Add `auth_key` context module to `rest-api-protocol/src/v1/context/` with `AuthKey` DTO (resources subdir) |                               |
| T2  | TODO   | Add `AddKeyRequest` DTO to protocol (or reuse from domain with wrapper)                                    |                               |
| T3  | TODO   | Add auth-key error response types to protocol                                                              |                               |
| T4  | TODO   | Define `AuthKeyCommandPort` in `rest-api-application/src/ports/`                                           | Methods for CRUD + reload     |
| T5  | TODO   | Implement `AuthKeyApiService` in `rest-api-application/src/use_cases/`                                     |                               |
| T6  | TODO   | Implement `TrackerAuthKeyAdapter` in `rest-api-runtime-adapter/src/adapters/`                              | Wraps `KeysHandler` + `clock` |
| T7  | TODO   | Update Axum handlers to use `AuthKeyApiService`                                                            |                               |
| T8  | TODO   | Update Axum state/routes to wire the new adapter                                                           |                               |
| T9  | TODO   | Verify pre-commit and pre-push checks pass                                                                 |                               |

## Verification / Progress

- [ ] Protocol DTOs created and exported
- [ ] `AuthKeyCommandPort` trait defined in `rest-api-application`
- [ ] `AuthKeyApiService` use-case implemented
- [ ] `TrackerAuthKeyAdapter` implemented in `rest-api-runtime-adapter`
- [ ] Axum handlers dispatch through use-case
- [ ] Pre-commit checks pass
- [ ] Pre-push checks pass

### Progress Log

| Date       | Event              |
| ---------- | ------------------ |
| 2026-06-24 | Draft spec created |
