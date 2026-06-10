---
doc-type: spec
issue-type: task
status: draft
priority: p2
epic: 1669
spec-path: docs/issues/drafts/1669-remove-udp-protocol-peer-id-re-export.md
last-updated-utc: 2026-06-10
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-06-10.md
---

# Remove `udp-protocol` Re-export of `PeerId`/`PeerClient`

## Subissue of EPIC #1669 — Overhaul: Packages

## Problem

After the extraction of `bittorrent-peer-id` → `torrust-peer-id` (SI-19, #1884),
the `torrust-tracker-udp-tracker-protocol` crate still re-exports `PeerId` and
`PeerClient` at its public API boundary:

```rust
// packages/udp-protocol/src/lib.rs line 28
pub use torrust_peer_id::{PeerClient, PeerId};

// packages/udp-protocol/src/common.rs line 15
pub use crate::{PeerClient, PeerId};
```

This creates an unnecessary coupling: consumers import `PeerId` from the UDP
protocol crate instead of directly from `torrust-peer-id`. This is how
`axum-http-server` ended up with a dependency on `udp-tracker-protocol` solely
for `PeerId` (identified in the 2026-06-10 coupling report as the #1 thin
dependency).

## Scope

Four consumers need updating:

### 1. `axum-http-server` (`tests/` only)

Replace `use torrust_tracker_udp_tracker_protocol::PeerId` with
`use torrust_peer_id::PeerId` in:

- `packages/axum-http-server/tests/server/requests/announce.rs`
- `packages/axum-http-server/tests/server/v1/contract.rs`

If `udp-tracker-protocol` becomes unused in `axum-http-server`, remove the
dependency from `Cargo.toml` entirely. Otherwise demote to dev-dep if only
test code uses it.

### 2. `tracker-client` (`packages/tracker-client`)

Replace `use torrust_tracker_udp_tracker_protocol::PeerId` with
`use torrust_peer_id::PeerId` in:

- `packages/tracker-client/src/peer_id.rs`
- `packages/tracker-client/src/http/client/requests/announce.rs`

Add `torrust-peer-id` to `packages/tracker-client/Cargo.toml` if not present.

### 3. `udp-server`

Replace `use torrust_tracker_udp_tracker_protocol::PeerClient` with
`use torrust_peer_id::PeerClient` in:

- `packages/udp-server/src/statistics/event/handler/error.rs`

Add `torrust-peer-id` to `packages/udp-server/Cargo.toml` if not present.

### 4. `udp-protocol` (internal)

Remove the `pub use torrust_peer_id::{PeerClient, PeerId}` re-export from:

- `packages/udp-protocol/src/lib.rs`
- `packages/udp-protocol/src/common.rs`

Internal code in `udp-protocol` that uses these types (e.g. `announce.rs`,
`request.rs`) should import directly from `torrust-peer-id` or use the already
available dependency declared in its `Cargo.toml`.

## Acceptance Criteria

1. No workspace crate imports `PeerId` or `PeerClient` from `torrust-tracker-udp-tracker-protocol` (or `torrust_tracker_udp_tracker_protocol`).
2. `cargo test --workspace` passes.
3. `cargo machete` passes (no unused deps).
4. `linter all` passes.

## Verification

- [ ] All 4 consumers updated to import from `torrust-peer-id` directly
- [ ] Re-exports removed from `udp-protocol`
- [ ] `cargo test --workspace` — pass
- [ ] `cargo machete` — pass
- [ ] `linter all` — pass
