---
doc-type: spec
issue-type: task
status: draft
priority: p2
epic: 1669
spec-path: docs/issues/drafts/1669-decouple-rest-api-core-from-udp-internals.md
last-updated-utc: 2026-06-11
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-06-10.md
---

# Decouple `rest-api-core` from Concrete UDP Server Internals

## Subissue of EPIC #1669 — Overhaul: Packages

**Note**: this is a **production code** decoupling (unlike the server `environment.rs` relocations
which only move test infrastructure). This subissue changes `rest-api-core/src/container.rs`
and related production types. It must be implemented before the server environment relocations
because it defines the `BanService` trait they both consume.

## Problem

`rest-api-core` imports concrete UDP types for statistics and banning:

**Production imports**:

| Import                                      | Concern                      |
| ------------------------------------------- | ---------------------------- |
| `BanService`                                | Banning service (field type) |
| `udp_stats_repository` types (`Repository`) | Statistics repository types  |

**Test-only imports** (follow from production deps):

| Import                            | Concern                     |
| --------------------------------- | --------------------------- |
| `MAX_CONNECTION_ID_ERRORS_PER_IP` | Test ban init constant      |
| `Repository::new()`               | Test stats repo constructor |

The production deps force `rest-api-core` to depend on both `udp-server` and
`udp-tracker-core` as runtime dependencies in `Cargo.toml`.

## Scope

### 1. Add a decision to DECISIONS.md

Record a new decision (DEC-14 or next available) with the chosen approach.

### 2. Decouple options

| Option                                                              | Change                                                                               | Effort   |
| ------------------------------------------------------------------- | ------------------------------------------------------------------------------------ | -------- |
| Define `BanService` trait in `tracker-core` or `primitives`         | Move banning interface to shared location; concrete impl stays in `udp-tracker-core` | Low      |
| Define `UdpStatsRepository` trait in `tracker-core` or `primitives` | Have both UDP and REST layers depend on the trait instead of the concrete type       | Low      |
| Move `MAX_CONNECTION_ID_ERRORS_PER_IP` to `primitives`              | Small constant move                                                                  | Very low |

### 3. Update consumers

- `rest-api-core`: depend on trait abstractions instead of concrete types
- `udp-server` + `udp-tracker-core`: implement the new traits
- `tracker-core` or `primitives`: host the new trait definitions

### 4. Clean up

- Run `cargo machete` to verify no unused deps
- Update `Cargo.toml` files
- Verify `linter all` and `cargo test --workspace`

## Acceptance Criteria

1. `rest-api-core/Cargo.toml` has no `udp-server` or `udp-tracker-core` runtime dependency
   (dev-dep only, if tests still reference concrete constructors).
2. `rest-api-core/src/` imports only trait abstractions from UDP packages, not concrete types.
3. `cargo test --workspace` passes.
4. `cargo machete` passes.
5. `linter all` passes.

## Out of Scope

- Decoupling `axum-rest-api-server` from UDP containers (separate subissue).
- Extracting any UDP package to a standalone repository.
- Changing the HTTP tracker side of the REST layer.

## Verification

- [ ] DEC-14 added to `docs/issues/open/1669-overhaul-packages/DECISIONS.md`
- [ ] `rest-api-core/Cargo.toml` has no `udp-server` or `udp-tracker-core` runtime dep
- [ ] `BanService` trait defined in shared location
- [ ] `UdpStatsRepository` trait defined in shared location
- [ ] `rest-api-core/src/` uses only trait references
- [ ] `cargo test --workspace` — pass
- [ ] `cargo machete` — pass
- [ ] `linter all` — pass
