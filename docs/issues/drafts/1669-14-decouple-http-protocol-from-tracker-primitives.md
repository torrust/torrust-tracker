---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1669-14-decouple-http-protocol-from-tracker-primitives.md
branch: null
related-pr: null
last-updated-utc: 2026-05-26 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - packages/http-protocol/Cargo.toml
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/primitives/src/announce.rs
    - packages/http-tracker-core/src/services/announce.rs
    - packages/axum-http-tracker-server/src/v1/handlers/announce.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Decouple `http-protocol` from `torrust-tracker-primitives`

Subissue ID: SI-14 (1669-14).

## Goal

Remove direct protocol-to-domain dependency from `http-protocol` by eliminating
`torrust-tracker-primitives` usage in `packages/http-protocol` and introducing
explicit boundary mapping in higher layers.

This draft is step 2 of the protocol decoupling strategy after edge cleanup
subissues SI-12 and SI-13.

This is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md).

## Design Decision (Scope Clarification)

This subissue follows DEC-06 from
[`docs/issues/open/1669-overhaul-packages/DECISIONS.md`](../open/1669-overhaul-packages/DECISIONS.md):

- Alternative considered: move `torrust_tracker_primitives::AnnounceEvent` to a
  new shared protocol package.
- Adopted approach: keep domain `AnnounceEvent` in primitives, keep protocol
  event types local to protocol crates, and map at boundary layers.

## Layer Impact Summary

Current edge:

- `http-protocol (protocol layer) -> tracker-primitives (domain layer)`

Why this is a concern:

- Protocol crates should own protocol DTOs/types and focus on BEP parsing.
- Depending on domain primitives from protocol makes extraction/reuse harder and
  leaks domain concepts into protocol-layer APIs.

Target edge:

- Remove `http-protocol -> torrust-tracker-primitives`.
- Keep mappings between protocol event types and domain event types in boundary
  layers (`http-tracker-core` and/or `axum-http-tracker-server`).

## Concrete Dependency Evidence

Manifest-level dependency:

- `packages/http-protocol/Cargo.toml`: `torrust-tracker-primitives = { ... path = "../primitives" }`

Symbol-level usage inside protocol:

- `packages/http-protocol/src/v1/requests/announce.rs`
  - conversion impls between HTTP protocol `Event` and
    `torrust_tracker_primitives::AnnounceEvent`

## Scope

### In Scope

- Remove conversion impls in `http-protocol` that directly reference
  `torrust_tracker_primitives::AnnounceEvent`.
- Remove `torrust-tracker-primitives` dependency from
  `packages/http-protocol/Cargo.toml`.
- Add/adjust mappings in boundary layer(s) to preserve behavior.
- Update tests and call sites to use boundary mapping instead of protocol crate
  domain type coupling.
- Update EPIC tracking references if needed.

### Out of Scope

- Decoupling `http-protocol` from `tracker-core` (covered in SI-12).
- Decoupling `http-protocol` from `udp-protocol` (covered in SI-13).
- BEP behavior changes.
- Broader tracker-wide domain type redesign outside this boundary.
- Moving `torrust_tracker_primitives::AnnounceEvent` to a new shared package.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                  | Notes / Expected Output                                                                                                                  |
| --- | ------ | ----------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Confirm all `torrust-tracker-primitives` usages in `http-protocol` and document symbol-level evidence | Evidence captured in PR description                                                                                                      |
| T2  | TODO   | Remove direct primitive conversion impls from `packages/http-protocol/src/v1/requests/announce.rs`    | No direct `torrust_tracker_primitives::` references remain in source                                                                     |
| T3  | TODO   | Remove `torrust-tracker-primitives` from `packages/http-protocol/Cargo.toml`                          | `cargo tree -p bittorrent-http-tracker-protocol --depth 1` shows no edge                                                                 |
| T4  | TODO   | Add/adjust mapping in higher layers (`http-tracker-core` and/or `axum-http-tracker-server`)           | Event behavior remains equivalent                                                                                                        |
| T5  | TODO   | Update tests and fixtures                                                                             | Tests compile and pass without direct protocol->domain coupling                                                                          |
| T6  | TODO   | Run verification commands                                                                             | Build/tests/lints pass                                                                                                                   |
| T7  | TODO   | Update EPIC tracking rows and draft list as needed                                                    | Active Subissues remain consistent                                                                                                       |
| T8  | TODO   | Update EPIC after implementation                                                                      | Update Active Subissues progress and EPIC sections: Package Inventory, Desired Package State, Torrust Dependency Lists (Direct, Non-dev) |

## Acceptance Criteria

- [ ] `packages/http-protocol/Cargo.toml` has no `torrust-tracker-primitives` dependency.
- [ ] `packages/http-protocol` has no source-level references to
      `torrust_tracker_primitives::`.
- [ ] HTTP announce event behavior remains unchanged for
      `started/stopped/completed/none` mappings.
- [ ] `cargo build --workspace` passes.
- [ ] Relevant tests in HTTP protocol/core/server packages pass.
- [ ] `linter all` exits with code `0`.
- [ ] EPIC tracking includes this subissue.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test -p bittorrent-http-tracker-protocol`
- `cargo test -p bittorrent-http-tracker-core`
- `cargo test -p torrust-tracker-axum-http-server`
- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                 | Command / Steps                                            | Expected Result                                            | Status | Evidence |
| --- | ---------------------------------------- | ---------------------------------------------------------- | ---------------------------------------------------------- | ------ | -------- |
| M1  | No protocol->domain edge remains         | `cargo tree -p bittorrent-http-tracker-protocol --depth 1` | No dependency on `torrust-tracker-primitives`              | TODO   |          |
| M2  | No primitives symbols in protocol source | `rg "torrust_tracker_primitives::" packages/http-protocol` | No matches                                                 | TODO   |          |
| M3  | Event conversion behavior preserved      | Run existing announce request parsing/unit tests           | Mappings for `started/stopped/completed/none` stay correct | TODO   |          |

## Risks and Trade-offs

- Mapping logic may be split across boundary layers; keep mapping ownership
  clear and avoid duplicate conversion logic.
- Temporary compatibility helpers may be needed while call sites migrate.

## References

- EPIC: [docs/issues/open/1669-overhaul-packages/EPIC.md](../open/1669-overhaul-packages/EPIC.md)
- HTTP protocol announce request: [packages/http-protocol/src/v1/requests/announce.rs](../../packages/http-protocol/src/v1/requests/announce.rs)
- HTTP protocol manifest: [packages/http-protocol/Cargo.toml](../../packages/http-protocol/Cargo.toml)
- Shared announce event type: [packages/primitives/src/announce.rs](../../packages/primitives/src/announce.rs)
