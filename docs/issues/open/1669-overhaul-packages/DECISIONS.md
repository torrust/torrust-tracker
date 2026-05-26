---
semantic-links:
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/adrs/
---

# EPIC #1669 — Design Decisions Log

This file records structural options that were **considered and discarded** during the
overhaul of the Cargo workspace package structure (EPIC #1669). Its purpose is to
prevent re-litigating settled decisions and to preserve the reasoning for future
contributors.

At the end of the refactor this log is intended to serve as the primary source material
for a new repo-level ADR documenting why the workspace ended up in its final shape.

**Format**: newest entry first. Each entry has a short title, the date it was decided,
the proposal, the reasoning, and a reference to any supporting artifact.

---

## DEC-04 — Match package folder names to crate names without prefix

**Date**: 2026-05-26
**Status**: Adopted

### Proposal

Use package folder names that match the crate name with the ownership prefix removed.
Examples:

- `torrust-tracker-rest-api-client` -> `rest-api-client`
- `torrust-tracker-udp-server` -> `udp-server`

### Why it was adopted

1. **Lower navigation friction**: the folder name can be inferred directly from crate name.
2. **Consistent workspace layout**: the same naming rule applies across packages.
3. **Cleaner documentation tables**: desired-state tables can show old vs new folder names
   explicitly with less ambiguity.

### Supporting artifact

[EPIC.md](EPIC.md) Desired Package State section.

---

## DEC-03 — Prefix indicates ownership/subdomain, not expected reusability

**Date**: 2026-05-26
**Status**: Adopted

### Proposal

Treat crate prefixes as ownership and release-identity markers. Reusability potential is not
encoded in the prefix. Tracker-domain crates use `torrust-tracker-` while organisation-level
shared crates use `torrust-`.

### Why it was adopted

1. **Clear ownership semantics**: prefixes map to workspace/product area rather than guesses
   about future external reuse.
2. **Stable naming over time**: avoids churn from renaming crates whenever perceived
   reusability changes.
3. **Consistent release identity**: tracker-owned crates remain identifiable as tracker crates
   even if reused outside this repository.

### Supporting artifact

[EPIC.md](EPIC.md) naming policy and Desired Package State tables.

---

## DEC-02 — Use `torrust-` as the default prefix for Torrust organisation crates

**Date**: 2026-05-26
**Status**: Adopted

### Proposal

Use `torrust-` as the default prefix for crates published by Torrust organisation
repositories. In practice, that means preferring names such as `torrust-bencode`,
`torrust-dht`, and `torrust-metainfo` rather than extending the prefix to
`torrust-bittorrent-` for every crate in the BitTorrent sub-project.

### Why it was adopted

1. **Shorter crate names**: the extra `bittorrent` segment adds length without adding
   enough value for the common case.
2. **Consistent organisation-level naming**: `torrust-` already scopes the crate to the
   Torrust organisation, which is the most important part for discoverability.
3. **Avoids redundant repetition**: the BitTorrent context is already obvious from the
   surrounding repository and package documentation.
4. **Leaves room for exceptions**: if a future crate really needs a more specific prefix,
   that can be recorded explicitly as an exception rather than becoming the default.

### Supporting discussion

[torrust/bittorrent#64](https://github.com/torrust/torrust-bittorrent/issues/64)
and its comments.

---

## DEC-01 — Do not merge protocol and core packages into feature-gated crates

**Date**: 2026-05-21
**Status**: Discarded

### Proposal

Merge the two protocol crates and the two protocol-specific core crates into single
crates controlled by Cargo features (`udp` and `http`, both disabled by default):

| Before                             | After                                                         |
| ---------------------------------- | ------------------------------------------------------------- |
| `packages/udp-protocol`            | _(removed)_                                                   |
| `packages/http-protocol`           | _(removed)_                                                   |
| `packages/udp-tracker-core`        | _(removed)_                                                   |
| `packages/http-tracker-core`       | _(removed)_                                                   |
| _(new)_                            | `packages/protocol`                                           |
| `packages/tracker-core` (existing) | `packages/tracker-core` (expanded with `udp`/`http` features) |

Crate renames implied:
`bittorrent-udp-tracker-protocol` + `bittorrent-http-tracker-protocol`
→ `bittorrent-tracker-protocol`

`bittorrent-udp-tracker-core` + `bittorrent-http-tracker-core` absorbed into
`bittorrent-tracker-core` as `udp` and `http` features.

### Why it was discarded

1. **Circular dependency blocker**: `bittorrent-http-tracker-protocol` already depends on
   `bittorrent-tracker-core` for four error types. After the merge the chain would be
   `bittorrent-tracker-core[http] → bittorrent-tracker-protocol[http] → bittorrent-tracker-core`,
   which Cargo refuses to compile. Resolving it requires a non-trivial prerequisite
   refactor (relocating error types) not present in the current plan.

2. **Coupling hidden, not removed**: the logical coupling between the packages does not
   decrease. Inter-crate edges (visible to `cargo tree`, enforceable with `cargo deny`)
   become intra-crate feature coupling (invisible by default, no equivalent tooling).

3. **Worse isolation for protocol-specification changes**: a BEP update currently has a
   clean, single-crate blast radius. After the merge a UDP-only change lives in a file
   that also contains HTTP protocol code; reviewers must filter irrelevant context and
   contributors must maintain `#[cfg(feature)]` discipline permanently.

4. **No benefit for cross-protocol same-layer changes**: the genuinely shared
   announce/scrape/whitelist logic already lives in the base `bittorrent-tracker-core`.
   The protocol-specific code in the core packages is not shared — it just sits at the
   same architectural layer.

5. **Extraction becomes harder**: the EPIC's stated direction is to eventually extract
   `bittorrent-*` crates to standalone repositories. A feature-gated merged crate is
   harder to publish with clean SemVer than two independent crates.

6. **Incremental compilation and test isolation degraded**: any change to the merged crate
   invalidates the compiled artifact for all features; per-feature test suites risk
   unintended cross-feature interactions.

### Supporting artifact

[workspace-coupling-report-proposed-merge.md](workspace-coupling-report-proposed-merge.md)
— full "as-if" coupling graph and three-dimension pros/cons analysis.
