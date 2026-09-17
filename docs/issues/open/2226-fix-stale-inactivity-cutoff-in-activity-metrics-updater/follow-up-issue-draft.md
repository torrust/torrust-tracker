---
doc-type: issue-draft
issue-type: refactor
status: parked
parent-issue: 2226
github-issue: null
spec-path: docs/issues/open/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/follow-up-issue-draft.md
last-updated-utc: 2026-09-17
---

# Follow-up Issue Draft: Clarify Peer Timestamp Naming and Cutoff Contracts

## Status

This is a parked draft. Do not create a GitHub issue from it yet. It records
future work so a contributor can review the timestamp contract before changing
the affected code.

## Goal

Make timestamp and duration semantics explicit across the internal peer model,
inactivity classification, cleanup code, REST conversion boundaries, tests, and
documentation without changing the behavior already fixed by issue `#2226`.

The central distinction is:

```text
max_peer_timeout              = elapsed duration from configuration
peer.last_updated_at          = absolute timestamp of the latest announce
inactivity_cutoff_timestamp   = absolute timestamp used as the classification boundary
```

## Motivation and Historical Context

The activity-metrics defect in issue `#2226` was possible because the original
job accepted a precomputed absolute cutoff timestamp and reused it forever.
The calculation was eventually moved into each update tick by commit
`1b6eaacc`, but the surrounding names still allow a timestamp to be confused
with an elapsed duration.

The original activity-metrics implementation was introduced by commit
`677deacd` for issue `#1523`. Its bootstrap helper was named
`peer_inactivity_cutoff_timestamp` and documented a timestamp result, but the
long-running job accepted the calculated value instead of the timeout policy.

The REST API has already addressed a related ambiguity. Issue [#2130](https://github.com/torrust/torrust-tracker/issues/2130)
introduced `updated_at_ms` while retaining the misleading v1 fields for
compatibility. That issue explicitly left the internal domain `peer::Peer`
unchanged. Earlier issue [#1930](https://github.com/torrust/torrust-tracker/issues/1930)
recorded the original REST naming problem.

## Proposed Naming

| Current name | Proposed name | Semantic contract |
| --- | --- | --- |
| `Peer.updated` | `Peer.last_updated_at` or `Peer.updated_at` | Absolute timestamp of the latest announce. |
| `ReadInfo::get_updated` | `ReadInfo::get_last_updated_at` or `ReadInfo::get_updated_at` | Returns the same absolute timestamp. |
| `PeerBuilder::updated_on` | `PeerBuilder::last_updated_at` or `updated_at` | Sets an absolute timestamp in fixtures. |
| `PeerBuilder::last_updated_on` | Align with the chosen builder name | Remove the duplicate terminology after migration. |
| `current_cutoff` | `inactivity_cutoff_timestamp` | Absolute timestamp used to classify or remove inactive peers. |
| `inactivity_cutoff` | `inactivity_cutoff_timestamp` | Avoid ambiguity between a timestamp and a duration. |
| `max_peer_timeout` | Keep unchanged | Correctly represents an elapsed duration/policy. |

The final names must be chosen before implementation. Prefer one consistent
`*_at` convention for absolute timestamps and reserve `Duration`-like names for
elapsed periods.

## Naming Decision: Protocol Fidelity Versus Domain Clarity

Do not rename `Peer.updated` automatically. First establish which layer owns
the name and whether the name is required by a protocol or wire contract.

Use this decision rule:

1. In a protocol-facing package or a Rust type that intentionally mirrors a
  BEP field, preserve the protocol terminology even when it is not the best
  domain name. Add Rust documentation that states the actual type and time
  semantics when the protocol name is ambiguous.
2. In a domain, application, or coordination package that is not required to
  mirror the wire format, prefer a semantic name such as `last_updated_at`.
  Convert to the protocol name at the package boundary.
3. If the current `Peer` type is shared between those roles, do not make a
  unilateral rename. Decide whether to split the protocol representation from
  the domain representation, or document why the shared type intentionally
  keeps the protocol-compatible name.

This repository already follows the boundary pattern for the REST API: issue
`#2130` introduced `updated_at_ms` in the REST DTO while preserving the domain
timestamp source and legacy v1 wire fields. The future implementation must
verify whether `Peer.updated` is genuinely required to mirror a BEP field or is
only an historical internal name. The result of that verification must be
recorded before changing the field.

## Required Investigation Before Implementation

The future contributor must re-analyse the current tree before editing. At a
minimum, inspect:

- All struct literals and direct accesses to `Peer.updated`.
- Both `ReadInfo` implementations for `Peer` and `Arc<Peer>`.
- Peer fixture builders and test names.
- `count_inactive_peers`, `get_activity_metadata`, and
  `remove_inactive_peers`.
- The tracker-core torrent-manager cleanup path and its cutoff calculation.
- REST protocol DTOs and runtime conversion code, especially the compatibility
  fields retained by issue `#2130`.
- Persistence, serialization, generated documentation, and external-facing
  package APIs.
- Any downstream or standalone package that consumes the primitives crate.

The contributor must confirm whether the field is part of a compatibility
contract before changing its serialized name. A Rust field rename can affect
struct literals and API consumers even when the runtime behavior is unchanged.

## Scope

### In Scope

- Choose and document unambiguous names for absolute peer timestamps.
- Rename the internal field, accessors, fixtures, and local cutoff variables if
  the compatibility audit permits it.
- Align comments and generated API documentation with the timestamp contract.
- Add focused tests that make timestamp-versus-duration semantics visible.
- Review whether cutoff calculation should be shared between metrics and cleanup
  code without violating package boundaries.
- Preserve the REST v1 compatibility behavior established by issue `#2130`.

### Out of Scope

- Reopening or changing the behavioral fix in issue `#2226` without new
  evidence.
- Removing the deprecated REST v1 fields from the current API.
- Implementing API v2.
- Changing timeout values, cleanup policy, metrics names, or retention policy.
- Claiming that the activity-metrics bug prevented peer removal without a
  separate cleanup-path reproduction.

## Test and Verification Strategy

Before implementation, write down the one semantic difference each test makes
visible:

1. A peer's latest-update timestamp is before the inactivity cutoff, so it is
   inactive.
2. A peer's latest-update timestamp is after the inactivity cutoff, so it is
   active.
3. The timeout value is a duration used to derive the cutoff timestamp.
4. Metrics classification and peer removal are tested separately.

Use the stopped clock for deterministic unit tests. Preserve coverage for:

- Coordinator counting with absolute timestamps.
- Registry removal with absolute timestamps.
- Activity metrics deriving a fresh cutoff from the timeout policy.
- REST conversion preserving the documented wire fields and values.
- Any serialization or compatibility behavior affected by the chosen rename.

Repeat the original runtime reproduction from issue `#2226` if the updater or
cleanup implementation changes. Record the exact commands and output in an
issue-local evidence document.

## Acceptance Criteria

- [ ] The chosen names distinguish elapsed durations from absolute timestamps.
- [ ] The implementation records whether each affected type is protocol-facing
  or domain-facing, and explains any decision to preserve a spec name.
- [ ] All affected internal call sites, accessors, fixtures, and documentation
      use the chosen terminology consistently.
- [ ] REST v1 compatibility fields remain unchanged unless a separate approved
      versioning decision allows otherwise.
- [ ] Metrics classification and peer removal have independently verified
      timestamp semantics.
- [ ] Deterministic tests cover active and inactive boundary cases.
- [ ] A runtime recheck confirms no regression in metrics or cleanup behavior.
- [ ] `linter all`, relevant package tests, documentation checks, and applicable
      pre-push checks pass.
- [ ] The final issue records the compatibility and package-boundary decisions.

## References

- Parent bug issue: [#2226](https://github.com/torrust/torrust-tracker/issues/2226)
- Parent bug specification: `ISSUE.md`
- Forensic findings: `forensic-findings.md`
- Pre-fix runtime evidence: `evidence.md`
- Related REST naming issue: [#2130](https://github.com/torrust/torrust-tracker/issues/2130)
- Earlier REST contract analysis: [#1930](https://github.com/torrust/torrust-tracker/issues/1930)
- Original activity-metrics implementation: commit `677deacd`
- Stale-cutoff fix: commit `1b6eaacc` and PR [#2252](https://github.com/torrust/torrust-tracker/pull/2252)
- Repository process follow-up: [#2230](https://github.com/torrust/torrust-tracker/issues/2230)
