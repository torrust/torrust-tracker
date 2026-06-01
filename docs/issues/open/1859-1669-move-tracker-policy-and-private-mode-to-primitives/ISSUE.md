---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1859
spec-path: docs/issues/open/1859-1669-move-tracker-policy-and-private-mode-to-primitives/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-06-01 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v2_0_0/core.rs
    - packages/primitives/src/
    - packages/tracker-core/
    - packages/swarm-coordination-registry/
    - packages/torrent-repository-benchmarking/
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #1859 — Move `TrackerPolicy`, `TORRENT_PEERS_LIMIT`, and `PrivateMode` to `torrust-tracker-primitives`

## Goal

Move three domain primitive types that are currently misplaced in
`torrust-tracker-configuration` into `torrust-tracker-primitives`, where they
semantically belong.

This is **FU-1** from the analysis in issue
[#1856](https://github.com/torrust/torrust-tracker/issues/1856) (DEC-07).

This issue is a subissue of EPIC [#1669](../1669-overhaul-packages/EPIC.md).

## Background

Issue #1856 analyzed the coupling of `torrust-tracker-configuration`. The conclusion
(DEC-07) was that the package boundary should remain unchanged — except for three types
that are domain policy objects, not service configuration:

| Type                        | Current location                | Correct location             |
| --------------------------- | ------------------------------- | ---------------------------- |
| `TrackerPolicy`             | `torrust-tracker-configuration` | `torrust-tracker-primitives` |
| `TORRENT_PEERS_LIMIT`       | `torrust-tracker-configuration` | `torrust-tracker-primitives` |
| `v2_0_0::core::PrivateMode` | `torrust-tracker-configuration` | `torrust-tracker-primitives` |

These types have no relationship to the config file schema, TOML deserialization, or
schema versioning. Their presence in the config package forces `swarm-coordination-registry`
and `torrent-repository-benchmarking` to depend on `torrust-tracker-configuration` — despite
using no actual configuration types.

### Current production consumers

| Type                  | Packages that use it in production                                               |
| --------------------- | -------------------------------------------------------------------------------- |
| `TrackerPolicy`       | `tracker-core`, `swarm-coordination-registry`, `torrent-repository-benchmarking` |
| `TORRENT_PEERS_LIMIT` | `tracker-core`                                                                   |
| `PrivateMode`         | `tracker-core` (authentication logic)                                            |

After this move, `swarm-coordination-registry` and `torrent-repository-benchmarking` will
no longer depend on `torrust-tracker-configuration`.

## Proposed Implementation Plan

### Step 1 — Add types to `torrust-tracker-primitives`

Define `TrackerPolicy`, `TORRENT_PEERS_LIMIT`, and `PrivateMode` in appropriate
modules under `packages/primitives/src/`. Choose module names that reflect their
domain semantics (e.g. `policy`, `mode`).

### Step 2 — Re-export from `torrust-tracker-configuration` (backwards compat)

To avoid a big-bang import update, temporarily re-export the moved types from
`torrust-tracker-configuration` with a `#[deprecated]` attribute pointing to the
new location. This keeps the workspace compiling while each import site is migrated.

> **Alternative**: perform all import site updates in a single commit without the
> re-export step. Acceptable if the workspace is small enough that this is not
> disruptive.

### Step 3 — Update all import sites

Update every `use torrust_tracker_configuration::...` that references the moved types
to import from `torrust_tracker_primitives` instead. Key files:

- `packages/tracker-core/src/announce_handler.rs`
- `packages/tracker-core/src/torrent/repository/in_memory.rs`
- `packages/tracker-core/src/authentication/mod.rs`
- `packages/tracker-core/src/authentication/service.rs`
- `packages/swarm-coordination-registry/src/coordinator.rs`
- `packages/swarm-coordination-registry/src/registry.rs`
- `packages/torrent-repository-benchmarking/` (all `entry/*.rs`, `repository/*.rs`)

### Step 4 — Remove re-exports and update Cargo.toml

Once all import sites are updated:

1. Remove the re-export shims from `torrust-tracker-configuration`.
2. Remove the original type definitions from the config package.
3. Update `swarm-coordination-registry/Cargo.toml` — remove `torrust-tracker-configuration`.
4. Update `torrent-repository-benchmarking/Cargo.toml` — remove `torrust-tracker-configuration`.
5. Confirm `torrust-tracker-primitives` is already a dependency (or add it) in every
   package that previously depended on the config package for these types.

### Step 5 — Verify

```bash
cargo test --workspace
cargo clippy -- -D warnings
```

Confirm that `swarm-coordination-registry` and `torrent-repository-benchmarking` no longer
list `torrust-tracker-configuration` in their `[dependencies]`.

## Acceptance Criteria

- [ ] `TrackerPolicy` is defined in `torrust-tracker-primitives`
- [ ] `TORRENT_PEERS_LIMIT` is defined in `torrust-tracker-primitives`
- [ ] `PrivateMode` is defined in `torrust-tracker-primitives`
- [ ] All import sites across the workspace import from `torrust-tracker-primitives`
- [ ] `swarm-coordination-registry` no longer lists `torrust-tracker-configuration` as a
      direct (non-dev) dependency
- [ ] `torrent-repository-benchmarking` no longer lists `torrust-tracker-configuration`
      as a direct (non-dev) dependency
- [ ] All tests pass (`cargo test --workspace --all-features`)
- [ ] No new clippy warnings

## Out of Scope

- Changing any other config types or package boundaries
- Changing how `tracker-core` or `EnvContainer` is initialized (FU-3, #1861)
- Schema version or TOML deserialization changes
- Moving `TslConfig` (FU-2, #1860)

## Layer Impact

This change moves types between the `primitives` layer and the `configuration` package.
No forbidden dependency edges are introduced:

- `tracker-core` → `torrust-tracker-primitives`: **already exists**
- `swarm-coordination-registry` → `torrust-tracker-primitives`: **already exists**
- `torrust-tracker-configuration` → `torrust-tracker-primitives`: **already exists**

The forbidden edges listed in the EPIC are not affected.

## Related

- Parent EPIC: #1669 — [EPIC.md](../1669-overhaul-packages/EPIC.md)
- Decision: DEC-07 in [DECISIONS.md](../1669-overhaul-packages/DECISIONS.md)
- Analysis: #1856 — [ISSUE.md](../1856-1669-analyse-configuration-package-coupling/ISSUE.md)
- Follow-ups: FU-2 (#1860), FU-3 (#1861)
