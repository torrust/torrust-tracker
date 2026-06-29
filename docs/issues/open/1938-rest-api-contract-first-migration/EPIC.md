---
doc-type: epic
issue-type: task
status: in_progress
priority: p1
epic: 1938
github-issue: 1938
spec-path: docs/issues/open/1938-rest-api-contract-first-migration/EPIC.md
epic-owner: josecelano
last-updated-utc: 2026-06-29
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1930-1669-si-33-rest-api-contract-first-architecture.md
    - docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md
    - docs/packages.md
    - packages/rest-api-protocol/
    - packages/rest-api-application/
    - packages/rest-api-runtime-adapter/
    - packages/axum-rest-api-server/
    - docs/issues/open/1938-rest-api-contract-first-migration/
---

<!-- skill-link: create-issue -->

# REST API Contract-First Migration (follow-up to SI-33 PoC)

## Goal

Migrate all remaining REST API contexts (`health_check`, `whitelist`, `auth_key`, `stats`) from direct tracker-internal wiring to the contract-first layered architecture (protocol → application → runtime-adapter → axum transport), following the pattern validated by [SI-33 (#1930)](../../open/1930-1669-si-33-rest-api-contract-first-architecture.md) PoC.

All context migrations are **complete** (SI-1 through SI-5 closed). The only remaining open item is SI-6 (`ApiClient` high-level typed client).

## Why This Is Needed

Before this EPIC, the REST API had a mixture of architectures:

- **`torrent` context** (SI-33 PoC) already used the contract-first architecture.
- **All other contexts** (`health_check`, `whitelist`, `auth_key`, `stats`) still had the old coupling:
  - Axum handlers calling tracker internals directly (`tracker-core`, `udp-core`, `http-core`, `udp-server`).
  - DTO/response types defined locally in the Axum server, not in `rest-api-protocol`.
  - No port traits or use-case services existed for these contexts.
  - Forbidden dependency edges (`axum-rest-api-server → tracker-core` etc.) still existed for non-torrent contexts.

This EPIC eliminated that coupling. The remaining open item (SI-6) is about improving the client API, not the server architecture.

## Relationship to SI-33

This EPIC is the follow-up work identified in [SI-33](../../open/1930-1669-si-33-rest-api-contract-first-architecture.md) (Stage 2). SI-33 defined the architecture, validated it with a PoC, and documented the plan. This EPIC executes the migration for all remaining contexts.

## Migration Order (Recommended)

The contexts are ordered by complexity and dependency depth. Follow-up tasks (SI-5, SI-6) come after all contexts are migrated:

| Order | Context / Task                   | Effort | Handlers | Tracker Deps             | Status |
| ----- | -------------------------------- | ------ | -------- | ------------------------ | ------ |
| 1     | SI-1: `health_check`             | Small  | 1        | None                     | ✅     |
| 2     | SI-2: `whitelist`                | Medium | 3        | `tracker-core` only      | ✅     |
| 3     | SI-3: `auth_key`                 | Medium | 4        | `tracker-core` + `clock` | ✅     |
| 4     | SI-4: `stats`                    | Large  | 2        | 5+ crates                | ✅     |
| 5     | SI-5: deprecate `rest-api-core`  | Small  | —        | —                        | ✅     |
| 6     | SI-6: introduce `ApiClient`      | Medium | —        | —                        | ❌     |
| 7     | SI-7: review tests + align v1 ns | Small  | —        | —                        | 🏗️     |

## Context Status Summary

| Context / Task                  | Axum Handlers | Protocol DTOs? | Port Trait? | Use-case? | Runtime Adapter? | Notes                                                                             |
| ------------------------------- | :-----------: | :------------: | :---------: | :-------: | :--------------: | --------------------------------------------------------------------------------- |
| `torrent`                       |   2 ✅ done   |       ✅       |     ✅      |    ✅     |        ✅        | Reference pattern — lives under `v1::context::torrent::resources::torrent`        |
| SI-1: `health_check`            |   1 ✅ done   |       ✅       |   ❌ N/A    |  ❌ N/A   |      ❌ N/A      | No tracker deps — DTOs under `v1::context::health_check::resources::health_check` |
| SI-2: `whitelist`               |   3 ✅ done   |       ✅       |     ✅      |    ✅     |        ✅        | Reuses `ActionStatus`                                                             |
| SI-3: `auth_key`                |   4 ✅ done   |       ✅       |     ✅      |    ✅     |        ✅        | Form DTOs + `clock`                                                               |
| SI-4: `stats`                   |   2 ✅ done   |       ✅       |     ✅      |    ✅     |        ✅        | 28-field DTO, SI-30 traits                                                        |
| SI-5: deprecate `rest-api-core` |       —       |       —        |      —      |     —     |        —         | ✅ done — crate removed from workspace                                            |
| SI-6: introduce `ApiClient`     |       —       |       —        |      —      |     —     |        —         | ❌ pending — typed wrapper over `ApiHttpClient`                                   |

## Scope

### In Scope (completed for SI-1 through SI-5)

The following scope items have been completed across sub-issues SI-1 through SI-5:

- ✅ Create protocol DTOs (request/response/error types) in `rest-api-protocol` for each context.
- ✅ Define port traits in `rest-api-application` for each context's operations.
- ✅ Implement use-case services in `rest-api-application`.
- ✅ Implement runtime adapters in `rest-api-runtime-adapter` wrapping tracker internals.
- ✅ Rewire Axum handlers to dispatch through use cases instead of direct internals.
- ✅ Remove internal crate dependencies from `axum-rest-api-server` as contexts were migrated.
- ✅ Update `deny.toml` layer bans as dependencies were removed.
- ✅ Deprecate and clean up `rest-api-core` (SI-5).
- ❌ **SI-6 (pending)**: Introduce `ApiClient` — a high-level typed client wrapping `ApiHttpClient` with protocol DTOs.
- 🏗️ **SI-7 (in progress)**: Review tests and align v1 namespace across REST API packages.

### Out of Scope

- API v2 behavior changes (tracked in issue #144).
- Extracting any package to a standalone repository (per EPIC #1669 policy).
- Publishing any REST API package as a stable external contract.
- Changing the HTTP tracker or UDP tracker layers.
- Renaming the `updated_milliseconds_ago` field (tracked in draft `rename-peer-updated-milliseconds-ago-to-updated-at-ms.md`).

## Sub-issues

- [#1939](https://github.com/torrust/torrust-tracker/issues/1939) — [SI-1](../../closed/1939-1938-si-1-migrate-health-check-context.md): Migrate `health_check` context ✅ closed
- [#1940](https://github.com/torrust/torrust-tracker/issues/1940) — [SI-2](../../closed/1940-1938-si-2-migrate-whitelist-context.md): Migrate `whitelist` context ✅ closed
- [#1941](https://github.com/torrust/torrust-tracker/issues/1941) — [SI-3](../../closed/1941-1938-si-3-migrate-auth-key-context.md): Migrate `auth_key` context ✅ closed
- [#1942](https://github.com/torrust/torrust-tracker/issues/1942) — [SI-4](../../closed/1942-1938-si-4-migrate-stats-context.md): Migrate `stats` context ✅ closed
- [#1943](https://github.com/torrust/torrust-tracker/issues/1943) — [SI-5](../../closed/1943-1938-si-5-deprecate-rest-api-core.md): Deprecate `rest-api-core` and remove from workspace ✅ closed
- [#1944](https://github.com/torrust/torrust-tracker/issues/1944) — [SI-6](../1944-1938-si-6-align-rest-api-client.md): Introduce `ApiClient` — a high-level typed client over protocol DTOs
- [#1959](https://github.com/torrust/torrust-tracker/issues/1959) — [SI-7](../1959-1938-si-7-review-tests-align-v1-namespace.md): Review tests and align v1 namespace across REST API packages

## Contract Evolution Governance

As the protocol package grows with context migrations, the following rules govern v1 contract changes to prevent breaking existing clients:

### v1 Additive-Only Rule

- **New fields, new endpoints, new response variants** are allowed in v1 — they are backward-compatible additions.
- **Removing or renaming fields** is forbidden in v1. Such changes must go through API v2 (tracked in issue #144).
- **Deprecating a field** is allowed — mark the old field with a doc comment indicating deprecation and the target v2 release where it will be removed.

### Exception for Internal-Only Types

Types that are not exposed over the wire (e.g., internal Rust enums used only for deserialization) may be refactored freely within v1 as long as the serialized JSON shape is unchanged.

### Enforcement

- Protocol DTO changes are reviewed against this policy during PR review.
- Any breaking change to the v1 wire format must be accompanied by a v2 alternative and a migration path.
- This policy should be documented in the `rest-api-protocol` crate README once the first v2 types are introduced.

## Dependency Removal Tracking

The following table maps each internal crate dependency to the sub-issue that removed it from `axum-rest-api-server/Cargo.toml`:

| Dependency                    | Removed by                         | Status |
| ----------------------------- | ---------------------------------- | ------ |
| `tracker-core`                | SI-2 (whitelist) + SI-3 (auth_key) | ✅     |
| `http-core`                   | SI-4 (stats)                       | ✅     |
| `udp-core`                    | SI-4 (stats)                       | ✅     |
| `udp-server`                  | SI-4 (stats)                       | ✅     |
| `rest-api-core`               | SI-5 (deprecate)                   | ✅     |
| `swarm-coordination-registry` | SI-4 (stats)                       | ✅     |
| `clock`                       | SI-3 (auth_key)                    | ✅     |

## Success Criteria

- ✅ All 10 non-torrent Axum handler functions dispatch through application use-case services.
- ✅ All response DTOs live in `rest-api-protocol`; none are defined locally in Axum server.
- ✅ All direct `tracker-core`, `udp-core`, `http-core`, `udp-server`, `rest-api-core`, and `swarm-coordination-registry` imports are removed from `axum-rest-api-server`.
- ✅ `deny.toml` layer bans enforce the new dependency rules.
- ✅ All pre-commit and pre-push checks pass.
- ✅ Integration tests continue to pass without behavioural changes.
- ❌ **SI-6 pending**: Introduce `ApiClient` high-level typed client.
- 🏗️ **SI-7 in progress**: Review tests and align v1 namespace.

## Progress Tracking

### Progress Log

| Date       | Event                                                                                  |
| ---------- | -------------------------------------------------------------------------------------- |
| 2026-06-24 | Draft EPIC created after SI-33 PoC validation                                          |
| 2026-06-24 | SI-1 (health_check) implemented — protocol DTOs migrated                               |
| 2026-06-24 | Specs updated to document normalized `context/` module structure for all protocol DTOs |
| 2026-06-25 | SI-1 closed on GitHub                                                                  |
| 2026-06-26 | SI-2 (whitelist) and SI-3 (auth_key) closed on GitHub                                  |
| 2026-06-27 | SI-4 (stats) closed on GitHub                                                          |
| 2026-06-29 | SI-5 (rest-api-core deprecation) closed on GitHub                                      |
| 2026-06-29 | Closed issue specs moved to `docs/issues/closed/` with updated frontmatter             |
| 2026-06-29 | SI-7 (review tests + align v1 ns) added — remaining task: SI-6 (ApiClient)             |
