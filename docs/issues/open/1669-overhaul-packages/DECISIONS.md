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
