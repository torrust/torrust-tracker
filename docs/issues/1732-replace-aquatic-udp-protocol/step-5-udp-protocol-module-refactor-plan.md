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

## Type Ownership Rules

`common.rs` owns protocol-wide shared types and helpers:

- `ConnectionId`
- `TransactionId`
- `InfoHash`
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
- wire `NumberOfBytes`

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
  - [ ] `cargo check --workspace`
  - [ ] `cargo test --workspace`
  - [ ] `linter all`
- [ ] Capture current public exports in `lib.rs`
- [ ] Capture current import usage in workspace (`rg` search)

Exit criteria:

- [ ] Baseline green and recorded in issue comments/notes

### Phase 1: Introduce New Action Modules

- [ ] Create `connect.rs`, `announce.rs`, `scrape.rs`
- [ ] Keep `Request`/`Response` enums and top-level parse/write wrappers in
      `request.rs`/`response.rs`
- [ ] Move concrete action-specific type implementations from
      `request.rs` and `response.rs` into action modules without behavior changes
- [ ] Re-export moved types from `lib.rs` to preserve public API for workspace consumers
- [ ] Ensure `lib.rs` re-exports old symbols and new module symbols

Exit criteria:

- [ ] `cargo check --workspace` passes
- [ ] `cargo test --workspace` passes

### Phase 2: Normalize `common.rs`

- [ ] Move action-specific types out of `common.rs`
- [ ] Keep only shared wire primitives and generic helpers in `common.rs`
- [ ] Ensure no announce/scrape-specific parsing logic remains in `common.rs`

Exit criteria:

- [ ] `common.rs` content matches ownership rules
- [ ] All tests still pass

### Phase 3: Compatibility and Call Site Stability

- [ ] Verify existing imports in dependent crates still compile via re-exports
- [ ] Update internal imports to use new module boundaries where beneficial
- [ ] Keep `request.rs` and `response.rs` as stable wrapper/orchestration modules

Exit criteria:

- [ ] Zero workspace build regressions
- [ ] No behavior changes in protocol encode/decode tests

### Phase 4: Optional Cleanup

- [ ] Keep wrappers and evaluate only internal simplification (not removal)
- [ ] Remove dead internal aliases/helpers if any remain after migration
- [ ] Update docs with final module map

Exit criteria:

- [ ] Final module structure agreed and documented
- [ ] Lints/tests/checks green

## Tracking Checklist

### Deliverables

- [ ] New action modules implemented
- [ ] `common.rs` narrowed to shared primitives
- [ ] Compatibility exports preserved
- [ ] Docs updated

### Type-by-Type Progress Tracker

Use this checklist to track migration one type at a time.

Status legend: `pending` | `moved` | `re-exported` | `consumers-updated` | `validated`

- [ ] `ConnectRequest`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)
- [ ] `ConnectResponse`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)
- [ ] `AnnounceRequest`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)
- [ ] `AnnounceActionPlaceholder`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)
- [ ] `AnnounceEvent`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)
- [ ] `AnnounceEventBytes`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)
- [ ] `ScrapeRequest`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)
- [ ] `AnnounceResponse<Ipv4AddrBytes>` / `AnnounceResponse<Ipv6AddrBytes>`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)
- [ ] `AnnounceResponseFixedData`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)
- [ ] `ScrapeResponse`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)
- [ ] `TorrentScrapeStatistics`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)
- [ ] `ErrorResponse`
  - [ ] moved
  - [ ] re-exported from `lib.rs`
  - [ ] consumers updated
  - [ ] validated (`cargo check --workspace`, `linter all`)

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

- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] `cargo test --doc --workspace`
- [ ] `linter all`

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

- [ ] Module boundaries are action-oriented and coherent
- [ ] Shared types remain in `common.rs`
- [ ] No wire format behavior changes introduced
- [ ] No unnecessary cross-module coupling
- [ ] Public API compatibility preserved during migration

## Suggested Commit Slicing

1. `refactor(udp-protocol): move connect types to connect module`
2. `refactor(udp-protocol): move announce types to announce module`
3. `refactor(udp-protocol): move scrape types to scrape module`
4. `docs(issue-1732): document final udp-protocol module layout`
