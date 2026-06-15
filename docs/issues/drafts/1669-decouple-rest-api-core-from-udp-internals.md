---
doc-type: spec
issue-type: task
status: draft
priority: p2
epic: 1669
spec-path: docs/issues/drafts/1669-decouple-rest-api-core-from-udp-internals.md
last-updated-utc: 2026-06-15
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-06-10.md
    - docs/issues/open/1903-1669-si-23-relocate-axum-rest-api-server-test-environment.md
---

# Decouple `rest-api-core` and `axum-rest-api-server` from Concrete UDP Server Internals

## Subissue of EPIC #1669 — Overhaul: Packages

**Note**: this is a **production code** decoupling (unlike the server `environment.rs` relocations
which only move test infrastructure). It changes `rest-api-core` containers/services and
`axum-rest-api-server` handlers/routes to use trait abstractions instead of concrete UDP types.

Implemented after the `environment.rs` relocations (subissues SI-23/SI-24/SI-25) — those were
test infrastructure moves, while this is a production dependency decoupling.

## Problem

Two packages import concrete UDP types, forcing runtime dependencies on `udp-server` and
`udp-tracker-core`:

### `rest-api-core`

**Production imports** in `src/container.rs` and `src/statistics/services.rs`:

| Import                                      | Location              |
| ------------------------------------------- | --------------------- |
| `BanService`                                | `container.rs`        |
| `UdpTrackerCoreContainer`                   | `container.rs`        |
| `UdpTrackerServerContainer`                 | `container.rs`        |
| `udp_stats_repository` types (`Repository`) | `container.rs`        |
| `BanService`                                | `statistics/services` |
| `udp_server::statistics`                    | `statistics/services` |

**Test-only imports** (follow from production deps):

| Import                            | Concern                     |
| --------------------------------- | --------------------------- |
| `MAX_CONNECTION_ID_ERRORS_PER_IP` | Test ban init constant      |
| `BanService` (concrete)           | Test BanService constructor |

### `axum-rest-api-server`

**Production imports** in `src/v1/context/stats/handlers.rs`:

| Import                                                                 | Concern                          |
| ---------------------------------------------------------------------- | -------------------------------- |
| `BanService`                                                           | Handler state type               |
| `torrust_tracker_udp_server::statistics::repository::Repository`       | Handler state type (get_stats)   |
| `torrust_tracker_udp_tracker_core::statistics::repository::Repository` | Handler state type (get_metrics) |

**Production references** in `src/v1/context/stats/routes.rs`:

| Reference                                        | Concern                   |
| ------------------------------------------------ | ------------------------- |
| `http_api_container.ban_service`                 | Passed into handler state |
| `http_api_container.udp_server_stats_repository` | Passed into handler state |
| `http_api_container.udp_core_stats_repository`   | Passed into handler state |

### Consequence

Both `rest-api-core/Cargo.toml` and `axum-rest-api-server/Cargo.toml` list `udp-server` and
`udp-tracker-core` as runtime dependencies. After the decoupling, they can be demoted to
dev-dependencies (or removed entirely) in both files.

## Scope

### 1. Add decisions to DECISIONS.md

Record DEC-14 (or next available) with the chosen approaches.

### 2. Trait extraction

Define shared trait abstractions for the three concrete UDP types that leak into REST code:

| Trait                         | Extracted from                                         | Host location (suggested)                    |
| ----------------------------- | ------------------------------------------------------ | -------------------------------------------- |
| `BanningService` (or similar) | `BanService` (concrete struct)                         | `tracker-core` or new `primitives` submodule |
| `UdpCoreStatsRepository`      | `udp_tracker_core::statistics::repository::Repository` | `tracker-core` or new `primitives` submodule |
| `UdpServerStatsRepository`    | `udp_server::statistics::repository::Repository`       | `tracker-core` or new `primitives` submodule |

Each trait exposes only the methods the REST layer actually calls (e.g. `get_stats()`,
`ban()`, etc.). This keeps the interface minimal and avoids leaking UDP internals.

### 3. Move constants

Move `MAX_CONNECTION_ID_ERRORS_PER_IP` from `udp_tracker_core` to a shared location
(e.g. `tracker-core` or `primitives`) since `rest-api-core` tests reference it.

### 4. Update `rest-api-core`

- `src/container.rs`: `TrackerHttpApiCoreContainer` stores `Arc<dyn BanningService>` instead
  of `Arc<RwLock<BanService>>`, and `Arc<dyn UdpStatsRepository>` for stats repos.
  `UdpTrackerCoreContainer` and `UdpTrackerServerContainer` are no longer imported.
- `src/statistics/services.rs`: function signatures use trait references instead of concrete
  `udp_server_statistics::repository::Repository`.
- Test code: instantiate concrete types via `udp_tracker_core` / `udp_server` (dev-deps).
- `Cargo.toml`: demote `udp-server` and `udp-tracker-core` to `[dev-dependencies]`.

### 5. Update `udp-server` and `udp-tracker-core`

- Implement the new traits on their existing concrete types.
- The `BanService` struct gets `impl BanningService for BanService`.

### 6. Update `axum-rest-api-server` handlers and routes

- `src/v1/context/stats/handlers.rs`: State tuples use `Arc<dyn BanningService>` and
  `Arc<dyn UdpStatsRepository>` instead of concrete types. The `use` imports for UDP
  concrete types are removed.
- `src/v1/context/stats/routes.rs`: Routes pass trait objects from
  `TrackerHttpApiCoreContainer` fields (already trait objects after step 4).
- `Cargo.toml`: demote `udp-server` and `udp-tracker-core` to `[dev-dependencies]`.

### 7. Clean up

- Run `cargo machete` to verify unused deps are gone from both `Cargo.toml`s.
- Update `Cargo.toml` files for both packages.
- Verify `linter all` and `cargo test --workspace`.

## Acceptance Criteria

1. `rest-api-core/Cargo.toml` has no `udp-server` or `udp-tracker-core` runtime dependency.
2. `axum-rest-api-server/Cargo.toml` has no `udp-server` or `udp-tracker-core` runtime dependency.
3. `rest-api-core/src/` imports only trait abstractions from UDP packages, not concrete types.
4. `axum-rest-api-server/src/v1/context/stats/` uses trait objects in handler state tuples and
   routes, not concrete UDP types.
5. `cargo test --workspace` passes.
6. `cargo machete` passes.
7. `linter all` passes.

## Out of Scope

- Extracting any UDP package to a standalone repository.
- Changing the HTTP tracker side of the REST layer.
- Relocating test environments (already done in SI-23/SI-24/SI-25).

## Verification

- [ ] DEC-14 added to `docs/issues/open/1669-overhaul-packages/DECISIONS.md`
- [ ] `BanningService` trait defined in shared location
- [ ] `UdpCoreStatsRepository` trait defined in shared location
- [ ] `UdpServerStatsRepository` trait defined in shared location
- [ ] `MAX_CONNECTION_ID_ERRORS_PER_IP` moved to shared location
- [ ] `rest-api-core/src/container.rs`: stores `Arc<dyn BanningService>` + `Arc<dyn UdpStatsRepository>`
- [ ] `rest-api-core/src/statistics/services.rs`: uses trait refs, not concrete types
- [ ] `rest-api-core/Cargo.toml`: `udp-server` + `udp-tracker-core` are dev-deps only
- [ ] `axum-rest-api-server/src/v1/context/stats/handlers.rs`: uses trait objects, not concrete types
- [ ] `axum-rest-api-server/src/v1/context/stats/routes.rs`: uses trait objects from container
- [ ] `axum-rest-api-server/Cargo.toml`: `udp-server` + `udp-tracker-core` are dev-deps only
- [ ] `cargo test --workspace` — pass
- [ ] `cargo machete` — pass
- [ ] `linter all` — pass
