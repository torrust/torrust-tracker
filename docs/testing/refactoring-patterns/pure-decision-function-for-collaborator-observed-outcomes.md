---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs
    - packages/swarm-coordination-registry/src/statistics/mod.rs
    - docs/testing/refactoring-patterns/README.md
---

# Pure Decision Function for Collaborator-Observed Outcomes

## Problem

A module makes one small decision and hands it to a collaborator, whose answer is the only thing
the module's tests can observe. The tests then pass or fail because of the collaborator's rules,
not the module's. A reader inside the module cannot explain the expected value without opening
another file, and an unrelated change to the collaborator breaks tests that never mentioned it.

In the originating case, the activity metrics job computed an inactivity cutoff timestamp and
passed it to `Registry::get_activity_metadata`. The only way to check the cutoff was to announce a
peer, run the job, and read a gauge. Whether the peer counted as inactive depended on the
registry's `updated <= cutoff` comparison and on the repository setting (not incrementing) the
gauge. The tests looked like unit tests of the job but were collaboration tests of three modules.

## Pattern

1. Name the module's own decision and extract it into a pure function with explicit inputs:

   ```rust
   fn inactivity_cutoff(now: DurationSinceUnixEpoch, max_peer_timeout: u32) -> DurationSinceUnixEpoch {
       now.checked_sub(Duration::from_secs(u64::from(max_peer_timeout)))
           .unwrap_or_default()
   }
   ```

   The caller keeps the side-effecting read visible: `inactivity_cutoff(CurrentClock::now(), ...)`.

2. Unit-test the pure function directly, one edge case per test: the normal case, the zero
   boundary, and the fallback when the subtraction cannot succeed.

3. Move the end-to-end tests that need the collaborators up to the level that owns the whole
   story, typically a `tests` module in the parent `mod.rs`. Label them as collaboration tests in
   the module comment and keep only the minimal pair that guards the user-visible regression.

4. Leave a short note in each module listing the tests that remain missing and the issue tracking
   them, so the classification survives the pull request.

## Selection Criteria

Use this pattern when all of the following hold:

1. The module computes a value from its inputs before delegating.
2. That value is not returned or otherwise exposed by the module's API.
3. Existing tests assert on the collaborator's output to infer the value.
4. The computation has edge cases worth pinning (boundaries, fallbacks, unit conversions).

## Do Not Use When

- The "decision" is a direct pass-through of a parameter; there is nothing to extract.
- The collaborator's rule is the behavior under test; write that test in the collaborator's module.
- Extraction would require a trait or injection point only to observe one value; prefer a pure
  function over a mock seam.
- The end-to-end pair would be removed rather than moved. The collaboration tests still guard the
  regression the pure tests cannot see (that the job reads the clock on every tick).

## Repository Example

[`packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs`](../../../packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs)
extracts `inactivity_cutoff` and pins three cases in `tests::inactivity_cutoff`. The before/after
timeout tests that need a real `Registry` and `Repository` live in
[`packages/swarm-coordination-registry/src/statistics/mod.rs`](../../../packages/swarm-coordination-registry/src/statistics/mod.rs)
under a state-named fixture, `JobWithOnePeerAnnouncedAtStartup`. The split was made while fixing
issue #2226, where the cutoff had been captured once at startup; remaining gaps are noted in the
owning modules and tracked by the package coverage EPIC #1347.

## Determinism

The pure function has no clock or I/O, so its tests need no runtime. The collaboration tests use
Tokio paused time for scheduling and `clock::Stopped` for domain time, advanced through named
fixture methods so each test states which clock it moves.
