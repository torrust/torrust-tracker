---
doc-type: epic
issue-type: task
status: planned
priority: p1
epic: 1938
github-issue: 1938
spec-path: docs/issues/open/1938-rest-api-contract-first-migration/EPIC.md
epic-owner: josecelano
last-updated-utc: 2026-06-24
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

Progressively migrate all remaining REST API contexts (`health_check`, `whitelist`, `auth_key`, `stats`) from direct tracker-internal wiring to the contract-first layered architecture (protocol → application → runtime-adapter → axum transport), following the pattern validated by [SI-33 (#1930)](../../open/1930-1669-si-33-rest-api-contract-first-architecture.md) PoC.

## Why This Is Needed

[SI-33](../../open/1930-1669-si-33-rest-api-contract-first-architecture.md) validated the contract-first architecture with a single endpoint (torrent detail). The remaining contexts still have the old coupling:

- Axum handlers call tracker internals directly (`tracker-core`, `udp-core`, `http-core`, `udp-server`).
- DTO/response types are defined locally in the Axum server, not in `rest-api-protocol`.
- No port traits or use-case services exist for these contexts.
- The forbidden dependency edges (`axum-rest-api-server → tracker-core` etc.) still exist for non-torrent contexts.

Migrating all contexts to the new architecture will:

- Allow removing direct internal crate dependencies from `axum-rest-api-server` (currently 7+ internal crate deps for non-torrent contexts).
- Make each context testable at the application layer without Axum.
- Provide a clear path toward a tracker-agnostic REST API standard.
- Complete the architectural vision started by SI-33.

## Relationship to SI-33

This EPIC is the follow-up work identified in [SI-33](../../open/1930-1669-si-33-rest-api-contract-first-architecture.md) (Stage 2). SI-33 defined the architecture, validated it with a PoC, and documented the plan. This EPIC executes the migration for all remaining contexts.

## Migration Order (Recommended)

The contexts are ordered by complexity and dependency depth. Follow-up tasks (SI-5, SI-6) come after all contexts are migrated:

| Order | Context / Task                  | Effort | Handlers | Tracker Deps             | Rationale                                        |
| ----- | ------------------------------- | ------ | -------- | ------------------------ | ------------------------------------------------ |
| 1     | SI-1: `health_check`            | Small  | 1        | None                     | Trivial starter — no tracker deps                |
| 2     | SI-2: `whitelist`               | Medium | 3        | `tracker-core` only      | Clean pattern, no DTOs needed                    |
| 3     | SI-3: `auth_key`                | Medium | 4        | `tracker-core` + `clock` | Form DTOs + validation, 4 endpoints              |
| 4     | SI-4: `stats`                   | Large  | 2        | 5+ crates                | 28-field DTO, Prometheus, multi-repo aggregation |
| 5     | SI-5: deprecate `rest-api-core` | Small  | —        | —                        | Cleanup after all contexts migrated              |
| 6     | SI-6: introduce `ApiClient`     | Medium | —        | —                        | Typed high-level wrapper over `ApiHttpClient`    |

## Context Status Summary

| Context / Task                  | Axum Handlers | Protocol DTOs? | Port Trait? | Use-case? | Runtime Adapter? | Notes                              |
| ------------------------------- | :-----------: | :------------: | :---------: | :-------: | :--------------: | ---------------------------------- |
| `torrent`                       |   2 ✅ done   |       ✅       |     ✅      |    ✅     |        ✅        | Reference pattern                  |
| SI-1: `health_check`            |       1       |       ❌       |     ❌      |    ❌     |        ❌        | No tracker deps needed             |
| SI-2: `whitelist`               |       3       |       ❌       |     ❌      |    ❌     |        ❌        | Reuses `ActionStatus`              |
| SI-3: `auth_key`                |       4       |       ❌       |     ❌      |    ❌     |        ❌        | Form DTOs + `clock`                |
| SI-4: `stats`                   |       2       |       ❌       |     ❌      |    ❌     |        ❌        | 28-field DTO, SI-30 traits         |
| SI-5: deprecate `rest-api-core` |       —       |       —        |      —      |     —     |        —         | Post-migration cleanup             |
| SI-6: introduce `ApiClient`     |       —       |       —        |      —      |     —     |        —         | Typed wrapper over `ApiHttpClient` |

## Scope

### In Scope

- Create protocol DTOs (request/response/error types) in `rest-api-protocol` for each remaining context.
- Define port traits in `rest-api-application` for each context's query/command operations.
- Implement use-case services in `rest-api-application`.
- Implement runtime adapters in `rest-api-runtime-adapter` wrapping tracker internals.
- Rewire Axum handlers to dispatch through use cases instead of direct internals.
- Update tests to use adapter conversion functions.
- Remove internal crate dependencies from `axum-rest-api-server` as contexts are migrated.
- Update `deny.toml` layer bans as dependencies are removed.
- Deprecate and clean up `rest-api-core` after all contexts are migrated (SI-5).
- Introduce `ApiClient` — a high-level typed client wrapping `ApiHttpClient` with protocol DTOs (SI-6).

### Out of Scope

- API v2 behavior changes (tracked in issue #144).
- Extracting any package to a standalone repository (per EPIC #1669 policy).
- Publishing any REST API package as a stable external contract.
- Changing the HTTP tracker or UDP tracker layers.
- Renaming the `updated_milliseconds_ago` field (tracked in draft `rename-peer-updated-milliseconds-ago-to-updated-at-ms.md`).

## Sub-issues

- [#1939](https://github.com/torrust/torrust-tracker/issues/1939) — [SI-1](../1939-1938-si-1-migrate-health-check-context.md): Migrate `health_check` context
- [#1940](https://github.com/torrust/torrust-tracker/issues/1940) — [SI-2](../1940-1938-si-2-migrate-whitelist-context.md): Migrate `whitelist` context
- [#1941](https://github.com/torrust/torrust-tracker/issues/1941) — [SI-3](../1941-1938-si-3-migrate-auth-key-context.md): Migrate `auth_key` context
- [#1942](https://github.com/torrust/torrust-tracker/issues/1942) — [SI-4](../1942-1938-si-4-migrate-stats-context.md): Migrate `stats` context
- [#1943](https://github.com/torrust/torrust-tracker/issues/1943) — [SI-5](../1943-1938-si-5-deprecate-rest-api-core.md): Deprecate `rest-api-core` and remove from workspace
- [#1944](https://github.com/torrust/torrust-tracker/issues/1944) — [SI-6](../1944-1938-si-6-align-rest-api-client.md): Introduce `ApiClient` — a high-level typed client over protocol DTOs

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

The following table maps each internal crate dependency to the sub-issue that removes it from `axum-rest-api-server/Cargo.toml`:

| Dependency                    | Removed by                         | Notes                                              |
| ----------------------------- | ---------------------------------- | -------------------------------------------------- |
| `tracker-core`                | SI-2 (whitelist) + SI-3 (auth_key) | Both contexts must finish                          |
| `http-core`                   | SI-4 (stats)                       | Via stats repository port                          |
| `udp-core`                    | SI-4 (stats)                       | Via SI-30 `BanningStats`, `UdpCoreStatsRepository` |
| `udp-server`                  | SI-4 (stats)                       | Via SI-30 `UdpServerStatsRepository`               |
| `rest-api-core`               | SI-5 (deprecate)                   | After all contexts migrated                        |
| `swarm-coordination-registry` | SI-4 (stats)                       | Via stats repository port                          |
| `clock`                       | SI-3 (auth_key)                    | Moved to runtime adapter                           |

## Success Criteria

- All 10 non-torrent Axum handler functions dispatch through application use-case services.
- All response DTOs live in `rest-api-protocol`; none are defined locally in Axum server.
- All direct `tracker-core`, `udp-core`, `http-core`, `udp-server`, `rest-api-core`, and `swarm-coordination-registry` imports are removed from `axum-rest-api-server`.
- `deny.toml` layer bans enforce the new dependency rules.
- All pre-commit and pre-push checks pass.
- Integration tests continue to pass without behavioural changes.

## Progress Tracking

### Progress Log

| Date       | Event                                         |
| ---------- | --------------------------------------------- |
| 2026-06-24 | Draft EPIC created after SI-33 PoC validation |
