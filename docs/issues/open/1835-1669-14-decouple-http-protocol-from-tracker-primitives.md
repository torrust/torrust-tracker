---
doc-type: issue
issue-type: task
status: in_progress
priority: p1
github-issue: 1835
spec-path: docs/issues/open/1835-1669-14-decouple-http-protocol-from-tracker-primitives.md
branch: 1835-1669-14-decouple-http-protocol-from-tracker-primitives
related-pr: null
last-updated-utc: 2026-05-27 18:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md
    - packages/http-protocol/Cargo.toml
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/http-protocol/src/v1/responses/announce.rs
    - packages/http-protocol/src/v1/responses/scrape.rs
    - packages/primitives/src/announce.rs
    - packages/primitives/src/number_of_bytes.rs
    - packages/udp-protocol/src/common.rs
    - packages/http-tracker-core/src/services/announce.rs
    - packages/axum-http-server/src/v1/handlers/announce.rs
    - packages/axum-http-server/src/v1/handlers/scrape.rs
---

<!-- skill-link: create-issue -->

# Issue #1835 - Decouple `http-protocol` from `torrust-tracker-primitives`

Subissue ID: SI-14 (1669-14).

## Goal

Remove direct protocol-to-domain dependency from `http-protocol` by eliminating
`torrust-tracker-primitives` usage in `packages/http-protocol` and introducing
explicit boundary mapping in higher layers.

This spec is step 2 of the protocol decoupling strategy after edge cleanup
subissues SI-12 and SI-13.

This is a subissue of EPIC [#1669](1669-overhaul-packages/EPIC.md).

## Execution Order

- Execute SI-13 first, then SI-14, to reduce merge-conflict risk and keep
  the dependency cleanup sequence explicit.

## Design Decision (Scope Clarification)

This subissue follows DEC-06 from
[`docs/issues/open/1669-overhaul-packages/DECISIONS.md`](1669-overhaul-packages/DECISIONS.md):

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
  layers, with ownership primarily in `http-tracker-core` and transport
  adaptation only in `axum-http-server` where needed.

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

| ID  | Status | Task                                                                                                          | Notes / Expected Output                                                                          |
| --- | ------ | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Confirm all `torrust-tracker-primitives` usages in `http-protocol` and document symbol-level evidence         | Evidence captured via `rg` and `cargo tree` outputs                                              |
| T2  | DONE   | Remove direct primitive conversion impls from `packages/http-protocol/src/v1/requests/announce.rs`            | No direct `torrust_tracker_primitives::` references remain in source                             |
| T3  | DONE   | Remove `torrust-tracker-primitives` from `packages/http-protocol/Cargo.toml`                                  | `cargo tree -p torrust-tracker-http-tracker-protocol --depth 1` shows no edge                    |
| T4  | DONE   | Add/adjust mapping in higher layers (`http-tracker-core` as primary owner; `axum-http-server` only if needed) | Event mapping now lives in `http-tracker-core`; response DTO mapping lives in `axum-http-server` |
| T5  | DONE   | Update tests and fixtures                                                                                     | Protocol/core/server tests and benchmark fixtures updated                                        |
| T6  | DONE   | Run verification commands                                                                                     | Build/tests/lints pass                                                                           |
| T7  | DONE   | Update EPIC tracking rows and draft list as needed                                                            | Active Subissues row updated                                                                     |
| T8  | DONE   | Update EPIC after implementation                                                                              | EPIC dependency notes updated for `http-protocol`                                                |

## Acceptance Criteria

- [x] `packages/http-protocol/Cargo.toml` has no `torrust-tracker-primitives` dependency.
- [x] `packages/http-protocol` has no source-level references to
      `torrust_tracker_primitives::`.
- [x] HTTP announce event behavior remains unchanged for
      `started/stopped/completed/none` mappings.
- [x] `cargo build --workspace` passes.
- [x] Relevant tests in HTTP protocol/core/server packages pass.
- [x] `linter all` exits with code `0`.
- [x] EPIC tracking includes this subissue.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test -p torrust-tracker-http-tracker-protocol`
- `cargo test -p torrust-tracker-http-tracker-core`
- `cargo test -p torrust-tracker-axum-http-server`
- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                 | Command / Steps                                                 | Expected Result                                            | Status | Evidence                                                                                                                                                              |
| --- | ---------------------------------------- | --------------------------------------------------------------- | ---------------------------------------------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M1  | No protocol->domain edge remains         | `cargo tree -p torrust-tracker-http-tracker-protocol --depth 1` | No dependency on `torrust-tracker-primitives`              | DONE   | Output shows `bittorrent-peer-id` and no `torrust-tracker-primitives`                                                                                                 |
| M2  | No primitives symbols in protocol source | `rg "torrust_tracker_primitives::" packages/http-protocol`      | No matches                                                 | DONE   | No matches returned                                                                                                                                                   |
| M3  | Event conversion behavior preserved      | Run existing announce request parsing/unit tests                | Mappings for `started/stopped/completed/none` stay correct | DONE   | `cargo test -p torrust-tracker-http-tracker-protocol`, `cargo test -p torrust-tracker-http-tracker-core`, and `cargo test -p torrust-tracker-axum-http-server` passed |

## Risks and Trade-offs

- Mapping logic may be split across boundary layers; keep mapping ownership
  clear and avoid duplicate conversion logic.
- Temporary compatibility helpers may be needed while call sites migrate.

## Post-Implementation Reasoning (Intentional Duplication)

The implementation introduces protocol-local DTOs that can look similar to
domain types (for example `SwarmMetadata` and `ScrapeData`). This duplication
is intentional and preserves a clean layering boundary:

- Protocol crates model BEP/wire semantics and should evolve with protocol
  changes.
- Similar concepts may also appear across protocol crates (for example
  `NumberOfBytes` in HTTP and UDP). This inter-protocol duplication is also
  intentional so one protocol can change wire representation/constraints
  without forcing synchronized changes in other protocols.
- Tracker/domain crates model application semantics and should evolve with
  tracker policy and product decisions.
- Boundary adapters (`http-tracker-core` and `axum-http-server`) absorb
  translation costs and prevent protocol-change blast radius across the app.

Trade-off acknowledgement:

- There is a small conversion overhead at boundaries.
- In exchange, coupling is reduced and protocol/domain life cycles stay
  independent.

This is aligned with DEC-06 and is preferred over re-coupling higher layers to
protocol DTOs.

## Follow-up Proposal

Consider extracting protocol crates to a dedicated protocol-focused repository
in a future EPIC phase. This would make lifecycle boundaries explicit:

- Protocol crates evolve with BEP/spec evolution.
- Tracker application crates evolve with product/domain evolution.

This subissue does not perform that extraction; it only prepares for it by
removing protocol -> domain coupling.

## References

- EPIC: [docs/issues/open/1669-overhaul-packages/EPIC.md](1669-overhaul-packages/EPIC.md)
- HTTP protocol announce request: [packages/http-protocol/src/v1/requests/announce.rs](../../packages/http-protocol/src/v1/requests/announce.rs)
- HTTP protocol manifest: [packages/http-protocol/Cargo.toml](../../packages/http-protocol/Cargo.toml)
- Shared announce event type: [packages/primitives/src/announce.rs](../../packages/primitives/src/announce.rs)
