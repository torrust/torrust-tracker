<!-- markdownlint-disable MD003 -->
issue-spec: docs/issues/closed/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/ISSUE.md
last-updated-utc: 2026-09-25
last-updated-utc: 2026-09-17 15:18
---

# Implementation Retrospective

## Outcome

The activity-metrics updater now receives `max_peer_timeout` as policy and
derives the absolute inactivity-cutoff timestamp immediately before every
metadata update. The fixed-build tracker recheck confirms that a peer announced
after startup changes from active to inactive after the timeout.

## Cutoff Ownership Decision

No shared cutoff helper is introduced in this bug fix. The two remaining thin
call sites deliberately calculate their own cutoff from `CurrentClock` and
`max_peer_timeout`:

- `tracker-core` depends on `swarm-coordination-registry`; placing a helper in
  `tracker-core` for the updater to call would create a dependency cycle.
- Exporting the helper from `swarm-coordination-registry` would make a generic
  time calculation part of that package's public API solely so `tracker-core`
  can reuse it. The existing dependency does not justify coupling cleanup to
  the metrics package's policy implementation.
- Moving the helper to `primitives` would expand a stable shared package API
  for two one-line calculations and requires broader timestamp-contract and
  compatibility review than this focused bug fix.

The current duplication is therefore intentional, bounded, and documented. The
parked [follow-up issue draft](follow-up-issue-draft.md) owns any future
timestamp naming or cross-package API audit.

## Regression-Test Evidence

The causal initial state is visible in both stopped-clock tests: the updater
runner starts first, then the peer announces. The only behavioral difference is
the elapsed stopped-clock duration relative to the two-second timeout:

- Before the timeout, the test advances one second and asserts the inactive
  peer gauge is `0`.
- After the timeout, the test advances three seconds and asserts the inactive
  peer gauge is `1`.

The production Act in each test is the updater tick advanced by the shared
15-second interval. The assertions read the observable gauge directly.

The following temporary test was applied only in a detached worktree at
pre-fix commit `524faf38` (the parent of implementation commit `1b6eaacc`),
then removed. It started the old updater with the startup-time cutoff, announced
a peer afterwards, advanced three seconds, and asserted the inactive gauge was
`1`:

```text
cargo test --manifest-path .tmp/issue-2226-pre-fix/Cargo.toml -p torrust-tracker-swarm-coordination-registry it_should_count_a_peer_announced_after_startup_inactive_after_the_timeout_elapses -- --nocapture

running 1 test
thread 'statistics::activity_metrics_updater::tests::it_should_count_a_peer_announced_after_startup_inactive_after_the_timeout_elapses' panicked at packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs:243:9:
assertion failed: (value - 1.0).abs() < f64::EPSILON
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 104 filtered out
```

The current focused suite passes with both boundaries:

```text
cargo test -p torrust-tracker-swarm-coordination-registry --lib statistics::activity_metrics_updater

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 101 filtered out
```

## Reusable Lessons

- A duration policy and the absolute timestamp derived from it must have
  distinct names and lifetimes; computing the latter at construction time can
  silently turn a recurring policy into a startup snapshot.
- A deterministic temporal regression test must make both the event ordering
  and the before/after timeout boundary visible.
- Correcting the stale calculation did not require expanding the domain or
  public API naming scope. The existing forensic findings and parked follow-up
  preserve that work for a separately reviewed change.

<!-- End of implementation retrospective. -->