# Step 7: PeerId Extraction Plan

## Goal

Remove duplicated `PeerId` / `PeerClient` implementations by extracting them into an in-house
shared crate at `packages/peer-id`, while preserving correct dependency direction:

- `bittorrent-udp-tracker-protocol` must not depend on `torrust-tracker-primitives`
- both crates consume `bittorrent-peer-id` via local path dependencies

## Context

Aquatic previously kept this logic in a dedicated `peer_id` crate.
During in-house migration, that logic ended up duplicated in:

- `packages/udp-protocol/src/peer_id.rs`
- `packages/primitives/src/peer_id.rs`

This plan restores the standalone shared-crate approach in-house.

## Scope

In scope:

- Create local workspace package `packages/peer-id`
- Move shared `PeerId` / `PeerClient` logic into that package
- Migrate `packages/udp-protocol` to consume it
- Migrate `packages/primitives` to consume it
- Keep public API compatibility for existing consumers
- Add a final internal module split step in `packages/peer-id` (`PeerId` and `PeerClient` modules)

Out of scope:

- Large API redesign of peer-id semantics
- Inverting crate dependency direction
- Folding protocol and domain crates together

## Implementation Shape

Default:

- canonical `PeerId` / `PeerClient` in `packages/peer-id`
- optional features for integrations (`serde`, `quickcheck`, `zerocopy`)

Fallback (if needed):

- keep thin local wrappers in consumers, but centralize parsing/client-identification logic in
  `packages/peer-id`

## Workspace Membership Note

`packages/peer-id` is consumed through local path dependencies.
Cargo workspace membership is auto-discovered in this repository setup, so explicit addition in
`[workspace].members` is not required.

## Execution Plan

### Phase 0: Baseline and Safety Net

- [ ] Record baseline:
  - [ ] `cargo check --workspace`
  - [ ] `cargo test --workspace`
  - [ ] `cargo test --doc --workspace`
  - [ ] `linter all`
- [ ] Capture current exports of both peer-id implementations
- [ ] Capture current consumers of both `PeerId` types

Exit criteria:

- [ ] Baseline recorded and green

### Phase 1: Create Extraction Target

- [x] Create new in-house crate at `packages/peer-id`
- [x] Add crate metadata and README
- [x] Add root module with exports (`PeerId`, `PeerClient`)
- [x] Wire local path dependencies from consumer crates
- [x] Seed crate contents from Aquatic-derived logic and in-house behavior

Exit criteria:

- [x] New crate exists and builds
- [x] Workspace resolution works through path dependencies
- [ ] No existing consumers changed yet

### Phase 2: Move Shared Logic

- [x] Move shared `PeerClient` detection/parsing logic into `packages/peer-id`
- [x] Move shared `PeerId` behavior into `packages/peer-id`
- [x] Preserve helper behavior (`first_8_bytes_hex`)
- [x] Add tests in `packages/peer-id` for behavior parity

Exit criteria:

- [x] Shared crate owns core logic
- [x] Behavior parity is validated

### Phase 3: Integrate With `bittorrent-udp-tracker-protocol`

- [x] Replace local peer-id module usage with `bittorrent-peer-id`
- [x] Preserve wire requirements (`zerocopy` feature)
- [x] Remove duplicated udp-protocol peer-id implementation

Exit criteria:

- [x] `bittorrent-udp-tracker-protocol` no longer owns duplicated peer-id logic
- [x] Protocol behavior remains unchanged

### Phase 4: Integrate With `torrust-tracker-primitives`

- [x] Replace local peer-id implementation with shared crate compatibility re-exports
- [x] Preserve public API for root exports and module-path imports

Exit criteria:

- [x] `torrust-tracker-primitives` compiles unchanged for consumers
- [x] Workspace build remains green

### Phase 5: Cleanup and Final Documentation

- [x] Remove leftover duplicated peer-id code
- [x] Document final ownership boundaries in issue docs
- [x] Record any remaining follow-up tasks

Exit criteria:

- [x] Duplication removed or reduced to intentional thin compatibility layers
- [x] Final structure documented

### Phase 6: Final Internal Module Split (Post-Extraction)

- [x] Split `packages/peer-id` internals into focused modules
- [x] Move `PeerId` type/helpers into dedicated module
- [x] Move `PeerClient` enum/detection logic into dedicated module
- [x] Preserve crate public API through root re-exports
- [x] Update tests to match new internal module boundaries

Exit criteria:

- [x] Internal module boundaries are clear and maintainable
- [x] Public API remains unchanged
- [x] Validation gate passes after split

## Deliverables

- [x] In-house shared crate created: `packages/peer-id`
- [x] Shared peer-id logic extracted
- [x] `udp-protocol` integrated with shared crate
- [x] `primitives` integrated with shared crate
- [x] Duplicate implementations removed from original locations
- [x] `packages/peer-id` internal module split completed
- [x] Final docs/progress notes updated

## Validation Gate

- [x] `cargo check --workspace`
- [x] `cargo test --workspace`
- [x] `cargo test --doc --workspace`
- [x] `linter all`

## Final Ownership (Implemented)

- `packages/peer-id`: canonical ownership of `PeerId` and `PeerClient`
- `packages/peer-id/src/peer_id.rs`: `PeerId` type and helpers
- `packages/peer-id/src/peer_client.rs`: `PeerClient` enum and client detection/parsing logic
- `packages/udp-protocol`: consumes `bittorrent-peer-id` (no local duplicated peer-id logic)
- `packages/primitives`: compatibility re-export module preserving existing public API paths

## Risks

### Risk 1: Wrong dependency direction

Impact: high

Mitigation:

- Keep `udp-protocol` independent of `torrust-tracker-primitives`
- Depend on `bittorrent-peer-id` from both crates

### Risk 2: Trait support divergence

Impact: high

Mitigation:

- Keep integration features explicit (`zerocopy`, `serde`, `quickcheck`)
- Validate protocol serialization behavior after every slice

### Risk 3: API breakage during internal module split

Impact: medium

Mitigation:

- Keep root `pub use` API stable while reorganizing internals
- Run full validation before closing Step 7

## Suggested Commit Slicing

1. `docs(issue-1732): add peer-id extraction plan`
2. `refactor(peer-id): create in-house crate and migrate udp-protocol`
3. `refactor(primitives): integrate extracted peer-id crate`
4. `refactor(peer-id): split peer-id crate into focused internal modules`
5. `docs(issue-1732): document final peer-id ownership`
