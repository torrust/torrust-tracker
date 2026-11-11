---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p3
epic: 1347
github-issue: 2823
spec-path: docs/issues/open/2823-1347-addassign-aggregate-swarm-metadata/ISSUE.md
branch: "2823-addassign-aggregate-swarm-metadata"
related-pr: null
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

Finding F4 of the EPIC #1347 `packages/primitives` analysis. In
`packages/primitives/src/swarm_metadata.rs`, `impl AddAssign for AggregateActiveSwarmMetadata`
does plain `+=` on four `u64` fields (`total_complete`, `total_downloaded`, `total_incomplete`,
`total_torrents`). On overflow it panics in debug builds and silently wraps in release builds (the
workspace profiles do not set `overflow-checks`).

Overflow is practically impossible (a `u64` counter of torrents/peers would need ~1.8e19 events).
The maintainer still wants the edge case explicit, because AI agents keep detecting it and
reporting it again, which wastes tokens and adds review noise. A documented decision stops that.

But unused code should not be kept. The in-tree callers that build an `AggregateActiveSwarmMetadata`
(`swarm-coordination-registry` `Registry::get_aggregate_swarm_metadata`, and the five
`torrent-repository-benchmarking` repositories' `get_metrics`) sum the four fields by hand
(`metrics.total_complete += u64::from(stats.complete)` …) and never use `AddAssign`. So the impl
looks like dead code, and the aggregation logic is duplicated in six places.

### Investigation results

See the experiment and history sections below (filled in from the removal experiment).

- **Removal experiment**: deleted the impl and the `use std::ops::AddAssign;` import, then ran
  `cargo check --workspace --all-targets --all-features`. Result: **compiles cleanly** (all workspace crates, tests, benches, examples). `cargo test --doc --workspace`
  also passes. Nothing uses the impl. The experiment edit was reverted afterwards.
- **History**: the impl came from `TorrentsMetrics` (`src/core`, 2023), moved to
  `packages/primitives` in `9a43815d0` (2024-02-13, "move torrent/repository to packages") and
  renamed twice (`144a338b9` → `AggregateSwarmMetadata`, `3d6fc651d` → `AggregateActiveSwarmMetadata`).
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

- Other `packages/primitives` tests (they stay with the `primitives tests` session, which defers
  the `AddAssign` lines to this issue).
- Deduplicating the hand-written summation in the registry / benchmarking repositories (note only;
  a separate issue if the maintainer wants it).
- Setting `overflow-checks` in the workspace release profile.

## Architectural Decisions

- Related ADRs: none.
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
| T1  | DONE   | Confirm the impl is unused (removal experiment, all targets) | `cargo check/test --workspace --all-targets` + doctests pass |
| T2  | DONE   | Delete impl and import (or harden, if T1 finds a user)       | `linter all` clean                                           |
| T3  | TODO   | Tell the `primitives tests` session the outcome              | Message with issue number and outcome                        |

## Commit Points

| Task | Coherent change set                  | Commit policy                                        |
| ---- | ------------------------------------ | ---------------------------------------------------- |
| T2   | Delete (or harden + test) the impl   | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2823-1347-addassign-aggregate-swarm-metadata/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2823)
- [x] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-11 00:00 UTC - Claude Code (Sonnet 5.5) - Drafted spec; ran removal experiment (check all targets/features + doctests: pass) and history check. Recommendation: delete.

## Acceptance Criteria

- [ ] The impl is either deleted (preferred if unused) or panics on overflow in debug and release
- [ ] If hardened: `#[should_panic]` test and a "practically impossible" comment exist
- [ ] `cargo check/test --workspace --all-targets` and doctests pass
- [ ] `linter all` exits with code `0`
- [ ] The `primitives tests` session is told the outcome

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace --all-targets --all-features` and `cargo test --doc --workspace`

### Manual Verification Scenarios

Not applicable: no user-visible behaviour changes.

## Risks and Trade-offs

- Deleting a public item of `torrust-tracker-primitives` is an API change for external crates that
  might use it. Mitigation: the crate is internal to the workspace, published only for the
  tracker's own packages.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

### Lessons (lessons.md-style note)

- `Registry::get_aggregate_swarm_metadata` (swarm-coordination-registry) and the five
  `get_metrics` implementations in `torrent-repository-benchmarking` each re-implement the
  summation of the four aggregate fields by hand. This duplication is why the `AddAssign` impl
  became unused. If the summation is ever consolidated, a single helper in `primitives` (with
  explicit overflow behaviour) is the natural home.

## References

- Related issues: EPIC #1347; sibling findings #2798 (F1, Peer `Ord`), #2804 (F3, `Location::caller` audit)
- Related ADRs: none
