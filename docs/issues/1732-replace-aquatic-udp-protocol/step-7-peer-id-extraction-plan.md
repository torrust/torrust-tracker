# Step 7: PeerId Extraction Plan

## Goal

Remove the duplicated `PeerId` / `PeerClient` implementation currently present in both
`packages/primitives/src/peer_id.rs` and `packages/udp-protocol/src/peer_id.rs` by creating an
in-house `peer-id` crate under `packages/` and moving shared logic there, without creating an
incorrect dependency from `bittorrent-udp-tracker-protocol` to `torrust-tracker-primitives`.

The extraction target is a local workspace package, managed in the same way as other
`packages/*` crates (for example, `packages/udp-protocol`).

## Scope

In scope:

- Analyze the duplicated `PeerId` / `PeerClient` logic and extract the shared implementation
- Introduce a new in-house crate (`packages/peer-id`) for peer-id parsing and client
  identification
- Register the crate as a local Cargo workspace member and consume it via path dependencies
- Move shared `PeerId` / `PeerClient` logic into that crate
- Update `torrust-tracker-primitives` to consume the new crate
- Update `bittorrent-udp-tracker-protocol` to consume the new crate
- Preserve public behavior and current consumer expectations during migration
- Add a final follow-up modularization step to split the extracted peer-id crate internals into
  smaller `PeerId`-focused and `PeerClient`-focused modules

Out of scope:

- Reworking unrelated primitive/domain types
- Renaming public `PeerId` / `PeerClient` APIs without compatibility planning
- Folding the udp protocol crate into the tracker primitives crate
- General package renaming or rebranding in this step
- Large API redesign of peer-id semantics beyond extraction and modularization

## Current Problem

The same `PeerId` / `PeerClient` implementation exists in two places:

- `packages/primitives/src/peer_id.rs`
- `packages/udp-protocol/src/peer_id.rs`

This duplication is real and substantial.

The two copies are nearly identical in parsing, client identification, and formatting behavior.
The main difference is that the udp-protocol copy derives wire-oriented `zerocopy` traits because
it is used directly as a protocol wire type.

## Historical Context

Aquatic already had this logic extracted in a dedicated `peer_id` crate.

During the in-house migration, we merged that logic into `packages/udp-protocol` instead of
keeping a standalone crate, and the workspace now also has the same implementation in
`packages/primitives`.

This plan explicitly corrects that design decision by re-introducing an in-house `peer-id` crate.

## Architectural Constraint

The obvious shortcut would be to keep only `torrust-tracker-primitives::PeerId` and make
`bittorrent-udp-tracker-protocol` depend on `torrust-tracker-primitives`.

That is not the right dependency direction.

Why:

- `bittorrent-udp-tracker-protocol` is intended to remain a low-level, generic protocol crate.
- `torrust-tracker-primitives` is no longer purely generic; it already contains tracker-domain
  concerns and depends on tracker-specific crates.
- Making the generic wire-format crate depend on the tracker-domain crate would invert layering.

Conclusion:

- The duplication should probably be removed.
- It should not be removed by keeping only the copy in `torrust-tracker-primitives`.
- The correct fix is extraction into a separate in-house `peer-id` crate.

## Proposed Target Layout

Introduce a new crate under `packages/` named:

- `peer-id`

The crate should stay generic in responsibility even though it is maintained in-house.

Proposed ownership after extraction:

- New in-house `peer-id` crate owns:
  - `PeerId`
  - `PeerClient`
  - peer-id parsing and client-detection logic
  - formatting and helper methods like `first_8_bytes_hex`
- `torrust-tracker-primitives` re-exports or wraps the extracted types as needed
- `bittorrent-udp-tracker-protocol` re-exports or wraps the extracted types as needed

## Design Notes

Primary implementation shape:

### Default: Shared Canonical Type With Features

The new generic crate defines the canonical `PeerId` / `PeerClient` types and uses feature flags
for optional integrations:

- `serde`
- `quickcheck`
- `zerocopy`

Pros:

- Removes duplication at the root
- Preserves one canonical implementation
- Keeps both dependent crates thin
- Closest to the original Aquatic `peer_id` crate intent

Cons:

- Requires care to avoid feature leakage or awkward optional derives

Fallback implementation shape:

### Fallback: Shared Logic Plus Thin Local Wrapper Types

The new generic crate exposes parsing/client-identification logic, but each consumer crate keeps
its own `PeerId` newtype and forwards to the shared logic.

Pros:

- Keeps wire-specific and domain-specific trait derives local
- Minimizes feature coupling between crates

Cons:

- Retains some small wrapper duplication
- Less complete deduplication than Option A

## Current Recommendation

Proceed with the default shape (canonical `PeerId` / `PeerClient` in `packages/peer-id`).

If optional `zerocopy` support makes the shared crate awkward or leaky, switch to the fallback
wrapper strategy while still centralizing all parsing/client-identification logic in
`packages/peer-id`.

## Constraints

- Preserve current public behavior.
- Do not introduce a dependency from `bittorrent-udp-tracker-protocol` to
  `torrust-tracker-primitives`.
- Keep validation narrow and incremental.
- Use signed, logically sliced commits.

## Execution Strategy

Follow the same strategy used in previous refactors:

- create the target crate first
- move one logical piece at a time
- preserve compatibility with re-exports where useful
- validate after each slice
- avoid broad consumer churn until compatibility is in place

## Execution Plan

### Phase 0: Baseline and Safety Net

- [ ] Record baseline:
  - [ ] `cargo check --workspace`
  - [ ] `cargo test --workspace`
  - [ ] `linter all`
- [ ] Capture current exports of both peer-id implementations
- [ ] Capture current consumers of both `PeerId` types across the workspace

Exit criteria:

- [ ] Baseline recorded and green

### Phase 1: Create the Generic Extraction Target

- [ ] Create new in-house crate at `packages/peer-id`
- [ ] Add package metadata, README, and initial module layout
- [ ] Add `packages/peer-id` to workspace members in root `Cargo.toml`
- [ ] Wire local path dependencies from consumer crates to `packages/peer-id`
- [ ] Seed crate contents from the former Aquatic `peer_id` design and current in-house logic
- [ ] Confirm default shape or fallback shape based on trait/feature ergonomics

Exit criteria:

- [ ] New crate exists and builds
- [ ] Workspace resolution works through local path dependencies
- [ ] No existing consumers changed yet

### Phase 2: Move Shared Logic

- [ ] Move shared `PeerClient` enum and client-detection logic into the new crate
- [ ] Move shared `PeerId` behavior into the new crate
- [ ] Preserve helper behavior such as `first_8_bytes_hex`
- [ ] Add tests to ensure extracted behavior matches current behavior

Exit criteria:

- [ ] New crate owns the shared logic
- [ ] Tests confirm behavioral parity

### Phase 3: Integrate With `torrust-tracker-primitives`

- [ ] Update `packages/primitives` to use the extracted crate
- [ ] Preserve current public `PeerId` / `PeerClient` API
- [ ] Decide whether primitives re-exports the extracted types directly or wraps them

Exit criteria:

- [ ] `torrust-tracker-primitives` compiles unchanged for consumers
- [ ] Workspace build remains green

### Phase 4: Integrate With `bittorrent-udp-tracker-protocol`

- [ ] Update `packages/udp-protocol` to use the extracted crate
- [ ] Preserve wire-format requirements (`zerocopy` support or wrapper strategy)
- [ ] Remove duplicated peer-id logic from udp-protocol

Exit criteria:

- [ ] `bittorrent-udp-tracker-protocol` no longer owns the duplicated implementation
- [ ] Protocol behavior remains unchanged

### Phase 5: Cleanup and Final Documentation

- [ ] Remove leftover duplicated peer-id code
- [ ] Document final ownership boundaries
- [ ] Record follow-up work if any wrapper types remain by design

Exit criteria:

- [ ] Duplication removed or reduced to intentional thin wrappers only
- [ ] Final structure documented

### Phase 6: Final Internal Module Split (Post-Extraction)

- [ ] Split `packages/peer-id` internals into smaller modules with clear ownership
- [ ] Move `PeerId` type and `PeerId` helpers into a dedicated module
- [ ] Move `PeerClient` enum and detection/parsing logic into a dedicated module
- [ ] Keep the crate public API stable via re-exports from crate root
- [ ] Update internal tests/module-local tests to match the new module boundaries

Exit criteria:

- [ ] Internal module boundaries are clearer and easier to maintain
- [ ] Public API remains unchanged for downstream crates
- [ ] Validation gate remains green after the split

## Tracking Checklist

### Deliverables

- [ ] New in-house `packages/peer-id` crate created
- [ ] Workspace member wiring completed (`Cargo.toml` + path deps)
- [ ] Shared peer-id logic extracted
- [ ] `torrust-tracker-primitives` integrated with extracted crate
- [ ] `bittorrent-udp-tracker-protocol` integrated with extracted crate
- [ ] Duplicated implementations removed or reduced to thin wrappers only
- [ ] Extracted peer-id crate internally split into smaller modules
- [ ] Docs updated

### Work Item Tracker

- [ ] `packages/peer-id` crate scaffolded
- [ ] Aquatic-to-in-house mapping documented
- [ ] Shared `PeerId` extraction implemented
- [ ] Shared `PeerClient` extraction implemented
- [ ] `zerocopy` strategy decided
- [ ] primitives integration validated
- [ ] udp-protocol integration validated
- [ ] final duplication removed
- [ ] `PeerId` module split completed
- [ ] `PeerClient` module split completed

## Validation Gate

- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] `cargo test --doc --workspace`
- [ ] `linter all`

## Risk Register

### Risk 1: Wrong dependency direction

Impact: high

Mitigation:

- Do not make `bittorrent-udp-tracker-protocol` depend on `torrust-tracker-primitives`
- Extract into `packages/peer-id` instead

### Risk 4: Repeating migration mistake

Impact: medium

Mitigation:

- Keep peer-id concerns in `packages/peer-id` and do not merge them into feature crates
- Document in Step 7 that Aquatic's standalone `peer_id` separation is intentionally restored

### Risk 2: Trait support divergence

Impact: high

Mitigation:

- Decide explicitly whether `zerocopy` support belongs in the shared crate or in thin wrappers
- Validate protocol serialization/deserialization behavior after integration

### Risk 3: Hidden consumer differences

Impact: medium

Mitigation:

- Search all workspace consumers before changing public surfaces
- Preserve compatibility until the new crate is fully integrated

### Risk 5: API breakage during internal module split

Impact: medium

Mitigation:

- Keep all public types re-exported from the crate root while reorganizing internals
- Run full validation after the module split before closing Step 7

## Review Checklist

- [ ] The protocol crate remains independent from tracker-domain crates
- [ ] Shared logic is owned in one place only
- [ ] Wire-format behavior remains unchanged
- [ ] Public consumer behavior remains unchanged
- [ ] The final dependency direction is coherent
- [ ] The historical Aquatic separation is restored in-house
- [ ] Internal module split is complete without public API changes

## Suggested Commit Slicing

1. `docs(issue-1732): add peer-id extraction plan`
2. `refactor(peer-id): create in-house peer-id crate`
3. `refactor(peer-id): extract shared PeerClient logic`
4. `refactor(peer-id): extract shared PeerId type`
5. `refactor(primitives): integrate extracted peer-id crate`
6. `refactor(udp-protocol): integrate extracted peer-id crate`
7. `refactor(peer-id): split peer-id crate into focused internal modules`
8. `docs(issue-1732): document final peer-id ownership`
