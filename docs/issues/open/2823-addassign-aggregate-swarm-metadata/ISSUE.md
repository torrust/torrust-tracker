---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p3
epic: null
github-issue: 2823
spec-path: docs/issues/open/2823-addassign-aggregate-swarm-metadata/ISSUE.md
branch: "2823-addassign-aggregate-swarm-metadata"
related-pr: 2827
last-updated-utc: "2026-11-11 00:00"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2823 - Remove (or harden) `AddAssign` for `AggregateActiveSwarmMetadata`

## Goal

Make the overflow edge case of `impl AddAssign for AggregateActiveSwarmMetadata` an explicit,
documented decision: delete the impl if nothing uses it; otherwise make it panic on overflow in
every build profile, with a `#[should_panic]` test and a comment saying it is practically
impossible.

## Background

Finding F4 of the `packages/primitives` testing analysis done under EPIC #1347. The analysis's finding register is not published in the repository, so this spec is self-contained (like #2798 and #2804). The issue is related to the EPIC, not one of its listed subissues. In
`packages/primitives/src/swarm_metadata.rs`, `impl AddAssign for AggregateActiveSwarmMetadata`
does plain `+=` on four `u64` fields (`total_complete`, `total_downloaded`, `total_incomplete`,
`total_torrents`). On overflow it panics in debug builds and silently wraps in release builds (the
workspace profiles do not set `overflow-checks`).

Overflow is practically impossible (a `u64` counter of torrents/peers would need ~1.8e19 events).
The maintainer still wants the edge case explicit, because AI agents keep detecting it and
reporting it again, which wastes tokens and adds review noise. A documented decision stops that.

But unused code should not be kept. The in-tree callers that build an `AggregateActiveSwarmMetadata`
(`swarm-coordination-registry` `Registry::get_aggregate_swarm_metadata`, and the ten
`get_metrics` implementations in eight modules of `torrent-repository-benchmarking`) sum the four fields by hand
(`metrics.total_complete += u64::from(stats.complete)` …) and never use `AddAssign`. So the impl
looks like dead code, and the aggregation logic is duplicated in eleven production copies (plus a hand-written one in
`torrent-repository-benchmarking/tests/repository/mod.rs`).

The live summations use the same unchecked `u64 +=`, so they also panic in debug and wrap in
release. Removing the impl removes one unused copy of that behaviour, not the behaviour itself;
the live summations are out of scope here (see Out of Scope).

### Investigation results

- **Removal experiment**: deleted the impl and the `use std::ops::AddAssign;` import, then ran
  `cargo check --workspace --all-targets --all-features` (stable Rust toolchain). Result: **compiles cleanly** (all workspace crates, tests, benches, examples). `cargo test --doc --workspace`
  (`cargo test --doc --workspace`, stable Rust toolchain) also passes. Nothing uses the impl. The experiment edit was reverted afterwards.
- **History**: `git log -S'impl AddAssign for'` first hits `9a43815d0` (2024-02-13, "move
  torrent/repository to packages"), so that commit added the impl for `TorrentsMetrics`; in the tree
  just before it no `.rs` file names `AddAssign` and `TorrentsMetrics` in `src/core/mod.rs` has no
  such impl. It was then renamed twice (`144a338b9` → `AggregateSwarmMetadata`, `3d6fc651d` → `AggregateActiveSwarmMetadata`).
  No in-tree caller of `+=` between two `TorrentsMetrics`/aggregate values was found by
  `git log -S`/`-G`; the callers always summed per-torrent stats by hand into a default value.

## Scope

### In Scope

- Decide used/unused by the removal experiment and the history check.
- If unused: delete the impl and its import (supersedes the hardening).
- If used: replace `+=` with `checked_add(..).expect("...")` on each field (same behaviour in
  debug and release), add a `#[should_panic]` test per field (or one test covering all), and a
  comment saying overflow is practically impossible and why it panics instead of wrapping.
- Callers the impl turns out to have (none expected).
- Record in the spec (`lessons.md`-style note) that the registry and the benchmarking repositories
  duplicate the aggregate summation instead of sharing one function.

### Out of Scope

- Other `packages/primitives` tests (they stay with the primitives package-testing work, which defers
  the `AddAssign` lines to this issue).
- Deduplicating the hand-written summation in the registry / benchmarking repositories, and
  changing its release-mode wrap on overflow (note only; a separate issue if the maintainer wants it).
- Setting `overflow-checks` in the workspace release profile.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260629000000_adopt_independent_package_versioning.md` (the major-version consequence of the breaking change).
- ADRs to create: None known.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable. The overflow is a latent, practically unreachable edge case, not observed broken
behaviour; no real-artifact reproduction exists.

## Regression Test Strategy

Not applicable for the delete branch. For the harden branch: a `#[should_panic(expected = "...")]` unit test in `swarm_metadata.rs` adding `1` to `u64::MAX` on each field.

## Implementation Plan

| ID  | Status | Task                                                         | Notes / Expected Output                                      |
| --- | ------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| T1  | DONE   | Confirm the impl is unused (removal experiment, all targets) | `cargo check --workspace --all-targets --all-features` and `cargo test --doc --workspace` pass (stable) |
| T2  | DONE   | Delete impl and import (or harden, if T1 finds a user)       | `linter all` clean                                           |

## Commit Points

| Task | Coherent change set                  | Commit policy                                        |
| ---- | ------------------------------------ | ---------------------------------------------------- |
| T2   | Delete (or harden + test) the impl   | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2823-addassign-aggregate-swarm-metadata/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2823)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-11 00:00 UTC - Claude Code (Sonnet 5.5) - Drafted spec; ran removal experiment (check all targets/features + doctests: pass) and history check. Recommendation: delete.
- 2026-11-11 UTC - Claude Code (Sonnet 5.5) - Maintainer approved deletion; created #2823 and PR #2827; review round 1 of PR 2827 (da2ce7 F1-F6): recorded the breaking-change/major-version consequence, corrected counts and history, set `epic: null` like #2798/#2804, ticked the checkpoints and criteria, added the Acceptance Verification table, noted that the live summation keeps the same wrap, dropped the session-addressed task, reworded the commit body.

## Acceptance Criteria

- [x] AC1: The impl is deleted (it is unused), so no unchecked `+=` remains in `swarm_metadata.rs`
- [x] AC2: `cargo check --workspace --all-targets --all-features` and `cargo test --doc --workspace` pass
- [x] AC3: `linter all` exits with code `0`

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo check --workspace --all-targets --all-features` and `cargo test --doc --workspace` (stable Rust toolchain)

### Manual Verification Scenarios

Not applicable: no user-visible behaviour changes.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                           |
| ----- | ---------------------- | -------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | PR #2827 diff removes the impl and import                                                          |
| AC2   | DONE                   | `cargo check --workspace --all-targets --all-features` and `cargo test --doc --workspace`, stable  |
| AC3   | DONE                   | `linter all` exit 0 at `80ac60d7c`                                                                 |

## Risks and Trade-offs

- `torrust-tracker-primitives` is publishable (`publish = true` through the workspace, version
  3.1.0), so removing a public trait impl is a breaking change: it breaks any downstream
  `a += b` on the type. Its next publish is a major version under
  `docs/adrs/20260629000000_adopt_independent_package_versioning.md`. That major version is already
  required because `Ord` was removed from `Peer` (see the Risks section of
  `docs/issues/open/2798-peer-ord-inconsistent-with-eq/ISSUE.md`). No external user of the impl is
  expected (not verified).

## Implementation Completion Review

- Retrospective: `Not yet assessed`

### Lessons (lessons.md-style note)

- `Registry::get_aggregate_swarm_metadata` (swarm-coordination-registry) and the ten
  `get_metrics` implementations in `torrent-repository-benchmarking` (eight modules; the test helper in
  `tests/repository/mod.rs` has another copy) each re-implement the
  summation of the four aggregate fields by hand. This duplication is why the `AddAssign` impl
  became unused. If the summation is ever consolidated, a single helper in `primitives` (with
  explicit overflow behaviour) is the natural home. The live summations keep the unchecked `u64 +=` (panic in debug, wrap in release);
  that is accepted as practically impossible and is out of scope for this issue.

## References

- Related issues: EPIC #1347 (related, not a listed subissue); sibling findings #2798 (F1, Peer `Ord`), #2804 (F3, `Location::caller` audit)
- Related ADRs: `docs/adrs/20260629000000_adopt_independent_package_versioning.md`
