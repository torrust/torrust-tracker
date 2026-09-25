<!-- markdownlint-disable MD003 -->
issue-spec: docs/issues/closed/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/ISSUE.md
last-updated-utc: 2026-09-25
last-updated-utc: 2026-09-17
---

# Forensic Findings: Activity-Metrics Cutoff

## Executive Summary

The inactivity bug was caused by confusing two related but different concepts:
the configured timeout duration and the absolute timestamp derived from that
duration. Both `Peer.updated` and the inactivity cutoff are timestamps. The
original metrics job calculated the cutoff once during application bootstrap
and reused that old timestamp for every later update. The result was that
peers announcing after startup could remain classified as active indefinitely
by the activity-metrics updater.

The names `updated`, `cutoff`, and `inactivity_cutoff` did not make the
timestamp contract explicit. This ambiguity did not change the Rust types, but
it made the incorrect lifetime of the cutoff easier to introduce and harder to
spot in review.

## Timestamp Contract

The relevant values have these meanings:

| Value | Meaning | Example |
| --- | --- | --- |
| `max_peer_timeout` | Configured elapsed duration | `5 seconds` |
| `Peer.updated` | Absolute timestamp of the peer's latest update | `12:30:35` |
| `inactivity_cutoff_timestamp` | Absolute timestamp before which updates are inactive | `12:30:40` |

The classification rule is a timestamp comparison:

```text
peer.last_updated_at <= inactivity_cutoff_timestamp
```

For example, if a peer was last updated at `12:30:35` and the cutoff is
`12:30:40`, the peer is inactive. The cutoff is calculated from a duration,
but the resulting value is not a duration:

```rust
CurrentClock::now_sub(&Duration::from_secs(max_peer_timeout))
```

The owning type confirms this contract: `Peer.updated` is a
`DurationSinceUnixEpoch`, and the coordinator compares it with another
`DurationSinceUnixEpoch` value.

## Historical Reconstruction

### Original Feature Introduction

Commit [`677deacd`](https://github.com/torrust/torrust-tracker/commit/677deacdc419526122eff62973f2685ac976a5eb), implementing issue `#1523`,
introduced the activity-metrics updater. The application bootstrap computed a
timestamp with `peer_inactivity_cutoff_timestamp(...)` and passed that value
to the long-running job. The job then passed the same value to every periodic
update.

The helper documentation correctly described its result as a timestamp:

```rust
/// Returns the timestamp of the cutoff for inactive peers.
fn peer_inactivity_cutoff_timestamp(max_peer_timeout: u32) -> Duration {
    CurrentClock::now_sub(&Duration::from_secs(u64::from(max_peer_timeout)))
        .unwrap_or_default()
}
```

However, the API accepted the calculated timestamp rather than the timeout
policy. That made a startup-time value look like a valid permanent job input.
There was no test proving that a peer announced after job creation becomes
inactive on a later update tick.

### Refactors That Preserved the Defect

Later package and lifecycle refactors preserved the same data flow:

- `9f8ab59d` moved `DurationSinceUnixEpoch` into the clock package.
- `660c01d4` renamed the clock crate to `torrust-clock`.
- `674e72cd` migrated the updater to cancellation-token lifecycle handling.
- `1b6eaacc` fixed the defect by passing `max_peer_timeout` into the job and
  deriving the cutoff inside each update.

The refactors were not the original source of the bug. They retained an
existing design that had already been incorrect since the activity-metrics
feature was introduced.

### API Naming Work Was Deliberately Separate

Issue [#2130](https://github.com/torrust/torrust-tracker/issues/2130) corrected
the v1 REST DTO field from the misleading `updated_milliseconds_ago` to
`updated_at_ms`. Its specification explicitly kept the legacy API fields for
v1 compatibility and listed changing the domain `peer::Peer` as out of scope.
That explains why the API representation was corrected while the internal
field remained named `updated`.

The REST naming problem originated earlier in issue [#1930](https://github.com/torrust/torrust-tracker/issues/1930), where the API contract history
identified `updated_milliseconds_ago` as an absolute Unix timestamp despite its
relative-duration suffix. The API correction therefore addressed a real and
separate ambiguity, but it did not establish a complete timestamp naming
contract for the internal domain model.

## Runtime Consequence

The pre-fix reproduction in `evidence.md` announced one peer after tracker
startup with `max_peer_timeout = 20`. The peer was still reported as active
about 267 seconds after its timeout had expired, and the metrics remained at
zero inactive peers through the captured observation.

The stale cutoff caused the following behavior:

1. The startup cutoff remained fixed in the past.
2. A peer announced after startup had an `updated` timestamp newer than that
   fixed cutoff.
3. Advancing wall-clock time did not move the cutoff forward.
4. The peer therefore continued to satisfy the updater's active classification.

This directly corrupted the inactive-peer and inactive-torrent metrics. It is
more precise to say that the metrics updater could keep reporting post-startup
peers as active indefinitely than to say that this updater prevented all peer
removal. `get_activity_metadata` only counts peers. Removal is performed by the
separate `remove_inactive_peers` path, which must be analysed independently
before claiming that registry cleanup was also affected.

## Naming and API Follow-Up

The following names should be considered for a separate follow-up issue or
refactor PR:

| Current name | Proposed name | Reason |
| --- | --- | --- |
| `Peer.updated` | `Peer.last_updated_at` or `Peer.updated_at` | Makes clear that the value is the latest-update timestamp, not elapsed time. |
| `get_updated` | `get_last_updated_at` or `get_updated_at` | Keeps the accessor aligned with the field contract. |
| `current_cutoff` | `inactivity_cutoff_timestamp` | Makes clear that the value is an absolute timestamp. |
| `inactivity_cutoff` used for a calculated value | `inactivity_cutoff_timestamp` | Distinguishes the timestamp from `max_peer_timeout`, which is a duration. |
| `peer_inactivity_cutoff_timestamp` | Remove the startup helper, or retain it only as a clearly scoped calculation helper | The calculation must happen at update time, not during job construction. |

This follow-up should audit all call sites, builders, protocol adapters,
serialization or REST DTOs, tests, and documentation before renaming the public
peer field. It should also review whether cutoff computation is duplicated in
the activity updater and cleanup path. The rename should not be folded into the
minimal behavioral fix in PR [#2252](https://github.com/torrust/torrust-tracker/pull/2252)
unless maintainers explicitly expand that PR's scope.

Repository history and issue specifications reviewed for this investigation did
not identify a separate issue for renaming the internal `Peer.updated` field.
Issue #2130 is related evidence, not an internal-rename issue: it intentionally
excluded the domain type. A follow-up issue should be opened if maintainers
want the internal field, accessor, fixtures, and timestamp terminology aligned
with the corrected API naming.

## Related Issues and Pull Requests

- [#1523](https://github.com/torrust/torrust-tracker/issues/1523) introduced
  the activity-metrics feature; see commit `677deacd`.
- [#1930](https://github.com/torrust/torrust-tracker/issues/1930) recorded the
  earlier REST contract analysis that identified the misleading
  `updated_milliseconds_ago` name.
- [#2130](https://github.com/torrust/torrust-tracker/issues/2130) corrected the
  REST API with `updated_at_ms` while deliberately leaving the domain peer type
  unchanged for scope and compatibility reasons.
- [#1488](https://github.com/torrust/torrust-tracker/issues/1488) is the
  shutdown-overhaul epic whose SI-5 work touched the same updater lifecycle.
- [#2226](https://github.com/torrust/torrust-tracker/issues/2226) tracks the
  stale cutoff bug and its behavioral fix.
- [#2230](https://github.com/torrust/torrust-tracker/issues/2230) tracks the
  repository-owned `fix-bug` workflow and bug-spec guardrails that were created
  after this investigation; it is process follow-up, not the code fix.
- [#2252](https://github.com/torrust/torrust-tracker/pull/2252) contains the
  implementation fix and deterministic regression test.

## Evidence Sources

- [`evidence.md`](evidence.md): pre-fix runtime reproduction and captured
  source wiring.
- [`packages/primitives/src/peer.rs`](../../../../packages/primitives/src/peer.rs):
  `Peer.updated` type and peer timestamp accessors.
- [`packages/swarm-coordination-registry/src/swarm/coordinator.rs`](../../../../packages/swarm-coordination-registry/src/swarm/coordinator.rs):
  timestamp comparison used by inactivity classification.
- [`packages/swarm-coordination-registry/src/swarm/registry.rs`](../../../../packages/swarm-coordination-registry/src/swarm/registry.rs):
  aggregate counting and separate inactive-peer removal operations.
- [`src/bootstrap/jobs/activity_metrics_updater.rs`](../../../../src/bootstrap/jobs/activity_metrics_updater.rs):
  application-level cutoff wiring before and after the fix.
- [`packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs`](../../../../packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs):
  periodic metrics job and regression test.
