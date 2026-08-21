---
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 144
github-issue: null
spec-path: docs/issues/drafts/144-rename-peer-updated-milliseconds-ago-to-updated-at-ms.md
branch: null
related-pr: null
last-updated-utc: 2026-06-24
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/rest-api-protocol/src/v1/resources/peer.rs
    - packages/rest-api-runtime-adapter/src/conversion.rs
    - packages/axum-rest-api-server/src/v1/context/torrent/resources/peer.rs
    - packages/rest-api-client/src/v1/client.rs
    - docs/issues/open/1930-1669-si-33-rest-api-contract-first-architecture.md
---

# Add `peer.updated_at_ms` field to v1 REST API and deprecate `updated_milliseconds_ago`

## Subissue of EPIC #144 — API v2

## Problem

The v1 peer DTO field `updated_milliseconds_ago` has a misleading name. It
behaves as an **absolute Unix timestamp** (milliseconds since epoch) of the
peer's last announce, but the `_ago` suffix implies a **relative duration**
("how long ago since the last update").

Both `updated` (deprecated) and `updated_milliseconds_ago` hold the **same
value** — they were populated identically in the original implementation and
remain identical in the current runtime adapter.

## Evidence

The field was introduced in commit `bc3d246f` (Nov 2022, "feat(api): in torrent
endpoint rename field to"). The diff shows:

```diff
+    #[deprecated(since = "2.0.0", note = "please use `updated_milliseconds_ago` instead")]
     pub updated: u128,
+    pub updated_milliseconds_ago: u128,
```

Both populated identically:

```rust
updated: peer.updated.as_millis(),                          // Unix timestamp in ms
updated_milliseconds_ago: peer.updated.as_millis(),          // same value
```

Where `peer.updated` is of type `DurationSinceUnixEpoch` — an absolute Unix
timestamp measured in milliseconds.

The original intent was **hypothesis #2**: to add the unit "milliseconds" to
the field name so clients know the value is in ms rather than seconds. The
`_ago` suffix is a misnomer.

See `docs/issues/open/1930-1669-si-33-rest-api-contract-first-architecture.md`
(Follow-up Tasks section) for the full analysis.

## Proposed Solution

### This issue (v1 additive change)

Add a new field `updated_at_ms` to the v1 protocol DTO alongside the existing
two fields. The existing fields stay in place:

| Field                      | Status                         | Value                | Removed in |
| -------------------------- | ------------------------------ | -------------------- | ---------- |
| `updated`                  | stays deprecated               | Unix timestamp in ms | v2         |
| `updated_milliseconds_ago` | stays (but becomes deprecated) | Unix timestamp in ms | v2         |
| `updated_at_ms`            | **new**                        | Unix timestamp in ms | —          |

Rationale for `updated_at_ms`:

- `_at` is a widely adopted API convention indicating a timestamp/point-in-time
  (e.g. `created_at`, `updated_at`).
- `_ms` unambiguously signals the unit is milliseconds.
- Total length: 14 chars — concise and self-documenting.

### v2 (future, tracked in EPIC #144)

Remove the `updated` and `updated_milliseconds_ago` fields entirely. Clients
will have had a full v1 cycle to migrate to `updated_at_ms`.

### Scope

| Area            | File(s)                                                                         | Change                                                          |
| --------------- | ------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| Protocol DTO    | `packages/rest-api-protocol/src/v1/resources/peer.rs`                           | Add field `updated_at_ms`, deprecate `updated_milliseconds_ago` |
| Runtime adapter | `packages/rest-api-runtime-adapter/src/conversion.rs`                           | Populate `updated_at_ms` with same Unix ms value                |
| Axum tests      | `packages/axum-rest-api-server/tests/server/v1/contract/context/torrent.rs`     | Add `updated_at_ms` to inline DTO literals                      |
| Axum tests      | `packages/axum-rest-api-server/tests/server/v1/asserts.rs`                      | Add `updated_at_ms` to inline DTO literals                      |
| E2E tests       | `src/console/ci/qbittorrent_e2e/tracker/client.rs`                              | Add `updated_at_ms` to inline DTO literals                      |
| E2E tests       | `src/console/ci/qbittorrent_e2e/scenario_steps/tracker/verify_tracker_swarm.rs` | Add `updated_at_ms` to inline DTO literals                      |
| REST API client | `packages/rest-api-client/src/v1/client.rs`                                     | Update if client parses field by name                           |
| Issue spec      | `docs/issues/open/1930-1669-si-33-rest-api-contract-first-architecture.md`      | Update follow-up task                                           |
| API docs        | `packages/axum-rest-api-server/src/v1/context/torrent/mod.rs`                   | Update endpoint documentation examples                          |

### Not in scope

- Removing the deprecated `updated` or `updated_milliseconds_ago` fields (v2 scope).
- Changing the domain type `DurationSinceUnixEpoch` or domain `peer::Peer`.
- Any protocol v2 changes.

## Verification

- [ ] `cargo check --workspace` passes.
- [ ] `linter all` passes.
- [ ] Integration tests (`cargo test --test integration`) pass.
- [ ] E2E scenario tests compile.
- [ ] `cargo +nightly doc --no-deps --workspace --all-features` succeeds.
