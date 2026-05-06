# Step 5: UDP Protocol Module Refactor Plan

## Goal

Refactor `packages/udp-protocol/src` so module boundaries reflect BEP 15 actions and shared
wire primitives are isolated. Keep behavior and external API stable during the migration.

## Scope

In scope:

- Reorganize internal modules in `packages/udp-protocol/src`
- Split action-specific types and logic into `connect`, `announce`, and `scrape`
- Keep shared protocol-wide wire types in `common`
- Preserve compatibility through `pub use` exports in `lib.rs`
- Keep all workspace users building without behavior changes

Out of scope:

- Redesigning protocol semantics
- Changing wire format
- Cross-crate public API breaks in one step

## Current Layout

Current source files:

- `common.rs`
- `request.rs`
- `response.rs`
- `peer_id.rs`
- `lib.rs`

Current problem:

- Request and response logic are grouped by message direction, not by BEP 15 action.
- Action-specific types are split across files, which makes ownership harder to follow.

## Target Layout

Planned source files:

- `common.rs` (shared wire primitives only)
- `connect.rs` (connect request and response)
- `announce.rs` (announce request and response)
- `scrape.rs` (scrape request and response)
- `request.rs` (kept as stable wrapper/orchestration entrypoint)
- `response.rs` (kept as stable wrapper/orchestration entrypoint)
- `peer_id.rs`
- `lib.rs`

## Final Module Map (Implemented)

- `common.rs`: shared wire primitives and helpers (`ConnectionId`, `TransactionId`, `InfoHash`,
  `NumberOfBytes`, `Port`, `PeerKey`, `NumberOfPeers`, `NumberOfDownloads`,
  `Ipv4AddrBytes`/`Ipv6AddrBytes`, `ResponsePeer<I>`, read helpers, `invalid_data`)
- `connect.rs`: connect action request/response types
- `announce.rs`: announce action request/response types and announce-only wire helpers
  (`AnnounceInterval`, `AnnounceActionPlaceholder`, `AnnounceEvent*`)
- `scrape.rs`: scrape action request/response types and scrape statistics
- `request.rs`: stable top-level request wrapper/orchestration
- `response.rs`: stable top-level response wrapper/orchestration (including `ErrorResponse`)
- `lib.rs`: compatibility-preserving re-exports

## Type Ownership Rules

`common.rs` owns protocol-wide shared types and helpers:

- `ConnectionId`
- `TransactionId`
- `InfoHash`
- `NumberOfBytes`
- `Port`
- `PeerKey`
- `NumberOfPeers`
- `NumberOfDownloads`
- `Ipv4AddrBytes`, `Ipv6AddrBytes`, `ResponsePeer<I>`
- read helpers and shared error helper (`invalid_data`)

`announce.rs` owns announce-only types and wire conversions:

- `AnnounceRequest`
- `AnnounceResponse*`
- `AnnounceInterval`
- `AnnounceActionPlaceholder`
- `AnnounceEvent`, `AnnounceEventBytes`

Current note:

- `InfoHash` and `NumberOfBytes` are intentionally retained in `common.rs` for now.
- These types mirror equivalents in other packages and can be unified in a separate future task.

`connect.rs` owns connect-only types:

- `ConnectRequest`
- `ConnectResponse`

`scrape.rs` owns scrape-only types:

- `ScrapeRequest`
- `ScrapeResponse`
- `TorrentScrapeStatistics`

`request.rs` and `response.rs` are intentionally retained:

- `Request` and `Response` enums stay as top-level wrappers
- top-level parse/write orchestration stays there
- concrete type implementations are delegated to action modules
- `ErrorResponse` remains in `response.rs` as the top-level error wrapper type

## Constraints

- Preserve all existing tests and behavior.
- Keep re-export compatibility from `lib.rs` during migration.
- Avoid changing call sites outside `udp-protocol` until compatibility exports are in place.

## Implementation Decisions (Agreed)

- Start migration with the `connect` action types first.
- Keep `request.rs` and `response.rs` as stable wrapper/orchestration modules.
- Use one signed commit per action (`connect`, `announce`, `scrape`).

## Execution Plan

### Phase 0: Baseline and Safety Net

- [ ] Record baseline:
  - [x] `cargo check --workspace`
  - [ ] `cargo test --workspace`
  - [x] `linter all`
- [x] Capture current public exports in `lib.rs`
- [x] Capture current import usage in workspace (`rg` search)

Exit criteria:

- [x] Baseline green and recorded in issue comments/notes

### Phase 1: Introduce New Action Modules

- [x] Create `connect.rs`, `announce.rs`, `scrape.rs`
- [x] Keep `Request`/`Response` enums and top-level parse/write wrappers in
      `request.rs`/`response.rs`
- [x] Move concrete action-specific type implementations from
      `request.rs` and `response.rs` into action modules without behavior changes
- [x] Re-export moved types from `lib.rs` to preserve public API for workspace consumers
- [x] Ensure `lib.rs` re-exports old symbols and new module symbols

Exit criteria:

- [x] `cargo check --workspace` passes
- [x] `cargo test --workspace` passes

### Phase 2: Normalize `common.rs`

- [x] Move action-specific types out of `common.rs`
- [x] Keep only shared wire primitives and generic helpers in `common.rs`
- [x] Ensure no announce/scrape-specific parsing logic remains in `common.rs`

Exit criteria:

- [x] `common.rs` content matches ownership rules
- [x] All tests still pass

### Phase 3: Compatibility and Call Site Stability

- [x] Verify existing imports in dependent crates still compile via re-exports
- [x] Update internal imports to use new module boundaries where beneficial
- [x] Keep `request.rs` and `response.rs` as stable wrapper/orchestration modules

Exit criteria:

- [x] Zero workspace build regressions
- [x] No behavior changes in protocol encode/decode tests

### Phase 4: Optional Cleanup

- [x] Keep wrappers and evaluate only internal simplification (not removal)
- [x] Remove dead internal aliases/helpers if any remain after migration
- [x] Update docs with final module map

Exit criteria:

- [x] Final module structure agreed and documented
- [x] Lints/tests/checks green

## Tracking Checklist

### Deliverables

- [x] New action modules implemented
- [x] `common.rs` narrowed to shared primitives
- [x] Compatibility exports preserved
- [x] Docs updated

### Type-by-Type Progress Tracker

Use this checklist to track migration one type at a time.

Status legend: `pending` | `moved` | `re-exported` | `consumers-updated` | `validated`

- [x] `ConnectRequest`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `ConnectResponse`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `AnnounceRequest`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `AnnounceActionPlaceholder`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `AnnounceEvent`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `AnnounceEventBytes`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `ScrapeRequest`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `AnnounceResponse<Ipv4AddrBytes>` / `AnnounceResponse<Ipv6AddrBytes>`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `AnnounceResponseFixedData`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `AnnounceInterval`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `ScrapeResponse`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `TorrentScrapeStatistics`
  - [x] moved
  - [x] re-exported from `lib.rs`
  - [x] consumers updated
  - [x] validated (`cargo check --workspace`, `linter all`)
- [x] `ErrorResponse`
  - [x] retained in `response.rs` by design
  - [x] re-exported from `lib.rs`
  - [x] consumers unchanged
  - [x] validated (`cargo check --workspace`, `linter all`)

### Per-Type Migration Workflow (Implementation Strategy)

For each type, execute this sequence before starting the next one:

1. Move one type to its target module.
2. Add/adjust `pub use` re-export in `lib.rs`.
3. Update consumers/imports.
4. Run validation gate for that single move:
   - `cargo check --workspace`
   - `linter all`
5. Mark the type row/checklist as validated.

### Validation Gate (must be green)

- [x] `cargo check --workspace`
- [x] `cargo test --workspace`
- [x] `cargo test --doc --workspace`
- [x] `linter all`

Additionally, run `linter all` at the end of every per-type move, not only at the end of the
full refactor.

## Risk Register

### Risk 1: Re-export breakage

Impact: high

Mitigation:

- Keep `lib.rs` compatibility exports during transition
- Validate downstream crates with full workspace build

### Risk 2: Silent protocol behavior regressions

Impact: high

Mitigation:

- Keep existing encode/decode tests unchanged
- Add focused tests if code moves require it

### Risk 3: Mixed ownership of types

Impact: medium

Mitigation:

- Apply and enforce ownership rules in this plan
- Review each moved type before merge

## Review Checklist

- [x] Module boundaries are action-oriented and coherent
- [x] Shared types remain in `common.rs`
- [x] No wire format behavior changes introduced
- [x] No unnecessary cross-module coupling
- [x] Public API compatibility preserved during migration

## Suggested Commit Slicing

1. [x] `refactor(udp-protocol): move connect types to connect module`
2. [x] `refactor(udp-protocol): move announce types to announce module`
3. [x] `refactor(udp-protocol): move scrape types to scrape module`
4. [x] `docs(issue-1732): document final udp-protocol module layout`
