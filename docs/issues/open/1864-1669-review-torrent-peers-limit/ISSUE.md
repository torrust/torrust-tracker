---
doc-type: issue
issue-type: task
status: open
priority: p3
github-issue: 1864
spec-path: docs/issues/open/1864-1669-review-torrent-peers-limit/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-06-01 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/primitives/src/policy.rs
    - packages/tracker-core/src/announce_handler.rs
    - packages/tracker-core/src/torrent/repository/in_memory.rs
    - packages/swarm-coordination-registry/src/swarm/registry.rs
    - packages/axum-http-server/src/lib.rs
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
---

<!-- skill-link: create-issue -->

# Issue #1864 — Review and refactor `TORRENT_PEERS_LIMIT`: hardcoded constant vs. config option

## Goal

Decide whether `TORRENT_PEERS_LIMIT` should remain a global compile-time constant,
be localised to each consuming package, or become a runtime configuration field.
Record the decision and implement it.

This is a follow-up to issue [#1859](../closed/1859-1669-move-tracker-policy-and-private-mode-to-primitives/ISSUE.md)
and a sub-task of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md).

## Background

Issue #1859 moved `TORRENT_PEERS_LIMIT` (`74`) and `TrackerPolicy` from
`torrust-tracker-configuration` into `torrust-tracker-primitives`. That was the
right first step to break the configuration coupling, but the constant is still a
global value shared across multiple packages.

### Current usages

`TORRENT_PEERS_LIMIT` is used in three distinct roles:

1. **Parse-time cap in `From<i32/u32> for PeersWanted`** (announce handler):

   ```rust
   // packages/tracker-core/src/announce_handler.rs
   impl From<i32> for PeersWanted {
       fn from(value: i32) -> Self {
           ...
           PeersWanted::Only { amount: amount.min(TORRENT_PEERS_LIMIT) }
       }
   }
   ```

   Because this is a `From` impl, runtime injection is not possible — the limit is
   baked in at the trait boundary.

2. **Default return count in `PeersWanted::limit()`** — returned when the client
   requested `AsManyAsPossible`.

3. **Query cap in repository methods** — `in_memory.rs` and `swarm/registry.rs`
   call `get_peers` / `get_swarm_peers` with `TORRENT_PEERS_LIMIT` as the hard
   ceiling.

## Questions to Resolve

- Should `TORRENT_PEERS_LIMIT` remain a single global constant, or should each
  package define its own local default?
- Should the cap become a runtime configuration option (e.g., a field on
  `TrackerPolicy`) so it can be tuned per deployment without recompilation?
- For the `From<i32/u32> for PeersWanted` trait impls, which cannot accept injected
  state, is a package-local constant the right answer, or should the impls be
  replaced by explicit constructors / free functions that accept the limit?
- If it becomes a config option, where does it sit in the configuration hierarchy
  and how is it threaded through to the repository query methods?

## Possible Approaches

| Approach                                              | Pros                               | Cons                                                        |
| ----------------------------------------------------- | ---------------------------------- | ----------------------------------------------------------- |
| Keep global constant in `primitives` (current state)  | Simple, no API churn               | Magic number, not tunable, couples packages                 |
| Move constant into each consuming package             | Removes cross-package coupling     | Duplication, values can drift                               |
| Add `max_peers_per_announce` field to `TrackerPolicy` | Runtime-tunable, operator-visible  | Requires plumbing through announce handler and repositories |
| Replace `From` impls with explicit constructors       | Removes implicit global dependency | API change for callers                                      |

## Acceptance Criteria

- [ ] A decision (ADR or `DECISIONS.md` entry under EPIC #1669) recording the chosen
      approach and the rationale.
- [ ] If the decision is to change the current design: implementation is complete,
      all tests pass, and the doc reference in `axum-http-server/src/lib.rs` is updated.
- [ ] `cargo test --workspace` passes.
- [ ] `linter all` passes.
