---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: 1978
github-issue: 2466
spec-path: docs/issues/open/2466-1978-announce-interval-upper-bound/ISSUE.md
branch: "2466-1978-announce-interval-upper-bound-spec"
related-pr: null
last-updated-utc: "2026-10-07 08:59"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md
    - docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
    - docs/adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md
    - docs/issues/closed/2245-2243-review-numeric-protocol-wire-conversions/ISSUE.md
    - packages/primitives/src/announce.rs
---

<!-- skill-link: create-issue -->

# Issue #2466 - Bound Announce Intervals to What Every Delivery Protocol Can Encode

**Parent EPIC:** #1978 - Configuration Overhaul (schema v3.0.0)

## Goal

Make `core.announce_policy.interval` and `core.announce_policy.interval_min` typed values that
reject anything above `i32::MAX` during deserialization, so the tracker fails fast at configuration
load instead of silently clamping the interval on the UDP wire.

## Background

BEP 15 encodes the UDP announce interval as a signed 32-bit integer. HTTP encodes `interval` and
`min interval` as bencode integers (`i64` in `http-protocol`). The domain fields
`primitives::announce::AnnouncePolicy::interval` and `interval_min` are plain `u32` values with no
bound, and the v3 configuration uses `AnnouncePolicy` directly as `Core::announce_policy`.

Before #2245, a configured `interval = 2147483648` was sent over UDP as `-2147483648` (reproduced
against a local tracker). #2245 (PR #2452) fixed the wire defect by clamping the value to
`i32::MAX` through `saturating_wire_i32` in `udp-server`. The configuration is still accepted, so
UDP clients receive `2147483647` while HTTP announces report the configured `2147483648`.

The #2245 plan first rejected the value in `Core::validate`. That conflicts with the configuration
validation ADR, which classifies a single-value bound as a **value invariant**: it must be a typed
newtype rejected during deserialization, and one-field rules must not be added to `Validator`. See
the [#2245 specification](../../closed/2245-2243-review-numeric-protocol-wire-conversions/ISSUE.md)
and its implementation retrospective.

## Decisions

Recorded with the maintainer on 2026-10-07:

1. **One protocol-agnostic value, bounded by the tightest delivery protocol.** The configuration
   keeps a single interval value for every delivery protocol (HTTP, UDP, and future ones such as
   WebTorrent). Its bound is the tightest limit among the supported protocols: `i32::MAX` seconds
   (about 68 years), from BEP 15. Realistic intervals are minutes to hours, so only nonsensical
   values are rejected. Per-protocol configuration values and per-protocol validation were rejected
   because a value could then be valid for one protocol and invalid for another.
2. **The bounded type lives in `primitives`.** Because the bound is a domain rule (an interval
   must be representable on every supported protocol), the type belongs to the domain and is used
   directly by the configuration, as `AnnouncePolicy` already is. `interval_min` uses the same type,
   because HTTP sends it too.
3. **Parent EPIC.** Configuration EPIC #1978 is reopened and this issue is added as its subissue.
   Configuration schema v3.0.0 is active in the code but not yet published on crates.io.
4. **Breaking change accepted.** Changing the `AnnouncePolicy` field types is acceptable: the
   tracker binary and library are heading for the 4.0.0 major release, and the other workspace
   packages version independently.
5. **Error message.** The error names the field, the configured value, and the limit
   (`2147483647`).
6. **ADR.** The rule in decision 1 is recorded as a repository-wide ADR, because it will apply to
   other numeric configuration values and future delivery protocols.

## Scope

### In Scope

- A bounded interval type in `primitives` whose construction and `Deserialize` reject values above
  `i32::MAX`, following the value-invariant ADR and the constrained-field newtype ADR.
- Changing `AnnouncePolicy::interval` and `AnnouncePolicy::interval_min` to that type and updating
  their consumers (`tracker-core`, `http-protocol`, `axum-http-server`, `udp-server`,
  `configuration`, and test or console consumers).
- Converting the UDP interval through the type without clamping. `saturating_wire_i32` remains for
  the seeder and leecher counts, which no configuration bounds.

### Out of Scope

- An `interval_min <= interval` consistency rule; that would be a separate `Validator` rule.
- The v2 configuration schema, which is kept only for backward compatibility.
- Seeder and leecher count clamping, which #2245 owns.
- Applying the ADR rule to other configuration values; later issues do that when they touch them.

## Architectural Decisions

- Related ADRs:
  - [Separate configuration value invariants from consistency validation](../../../adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md)
  - [Use newtypes for constrained configuration field types](../../../adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md)
- ADRs to create: [Bound protocol-agnostic values by the tightest delivery protocol](../../../adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md),
  added in the specification PR so the rule is reviewed before implementation.

## Design and Ownership Review

Not applicable: a value type change with no child processes, asynchronous I/O, network readiness,
resource cleanup, or reusable test fixtures.

## Bug-Fix Process

Follows [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

- Analysis: the configuration accepts an interval that cannot be represented on the UDP wire.
  After #2245 the UDP reply is clamped, so the remaining defect is silent acceptance and the
  divergence between UDP and HTTP intervals.
- Reproduction: start a tracker with `interval = 2147483648`; it starts, and the UDP announce
  returns `2147483647` (clamped) while the HTTP announce returns `2147483648`. Record it in
  `manual-verification-evidence.md` before changing code.
- Regression test boundary: construction and deserialization of the bounded type (unit tests at
  `i32::MAX` and `i32::MAX + 1`), plus a configuration-load test that rejects the value.

## Regression Test Strategy

Unit tests on the bounded type: `2147483647` is accepted and `2147483648` is rejected, both through
construction and through `Deserialize`. Configuration tests prove that loading a TOML with
`interval = 2147483648` or `interval_min = 2147483648` fails with an error naming the field. These
must fail against the current `u32` fields; record the red and green runs in the evidence file.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Resolve the open questions | Decisions section; ADR added in the specification PR. |
| T2 | TODO | Add the bounded type to `primitives` | Construction and `Deserialize` reject values above `i32::MAX`; unit tests at the boundary. |
| T3 | TODO | Adopt it for `interval` and `interval_min` | Consumers updated; configuration-load tests reject `2147483648` for both fields. |
| T4 | TODO | Convert the UDP interval through the type | No clamp for the interval; `saturating_wire_i32` kept for peer counts. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T2 | Bounded type and its boundary unit tests | Commit after focused validation and test-design review. |
| T3 | Field type change, consumer updates, configuration-load tests | Commit after focused validation and test-design review. |
| T4 | UDP interval conversion | Commit after focused validation; may merge into T3 if the type change forces it. |

Tests follow the `write-unit-test` skill, with the prose-first Arrange-Act-Assert review after each
passing increment and before maintainer review and commit. Commits are signed Conventional Commits
with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1978-announce-interval-upper-bound/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-06 12:29 UTC - GitHub Copilot - Local draft written from the #2245 follow-up decision - Parked until PR #2452 merged
- 2026-10-06 16:41 UTC - GitHub Copilot - Moved to `docs/issues/drafts/` after #2245 was archived (PR #2455); refreshed facts against `develop`; added Commit Points and Acceptance Verification - Awaiting maintainer review
- 2026-10-07 08:29 UTC - josecelano - Answered the open questions: one protocol-agnostic value bounded by the tightest delivery protocol, type in `primitives` (also for `interval_min`), subissue of reopened #1978, breaking change accepted for 4.0.0, ADR in the specification PR - Chat decision
- 2026-10-07 08:59 UTC - GitHub Copilot - Maintainer approved the specification; created #2466, reopened #1978 and linked #2466 as its subissue; moved this spec to `docs/issues/open/` - Specification PR

## Acceptance Criteria

- [ ] AC1: A configuration with `core.announce_policy.interval` or `interval_min` above
  `2147483647` fails to load with an error naming the field, the value, and the limit.
- [ ] AC2: `2147483647` is accepted, and UDP and HTTP announces report the same interval.
- [ ] AC3: The bound is enforced by a type in `primitives`, not by `Validator`, as the ADRs require.
- [ ] AC4: The UDP interval is converted without clamping.
- [ ] AC5: `linter all` exits with code `0` and relevant tests pass.
- [ ] AC6: Manual verification scenarios are executed and documented in issue-local
  `manual-verification-evidence.md`.

## Verification Plan

### Automatic Checks

- Bounded-type unit tests and configuration-load tests.
- Focused Clippy for changed packages, `linter all`, and pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Reproduce, then reject an out-of-range interval | Start a local tracker with `interval = 2147483648`, before and after the fix. | Before: starts; UDP reports `2147483647`, HTTP `2147483648`. After: startup fails with an error naming `interval` and the limit. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Reject an out-of-range minimum interval | Start with `interval_min = 2147483648`. | Startup fails with an error naming `interval_min` and the limit. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Accept the boundary | Start with `interval = 2147483647` and announce over UDP and HTTP. | Both report `2147483647`. | TODO | `manual-verification-evidence.md` section V3 |

### Disposable Verification Scripts

None planned. The scenarios use the tracker binary and `tracker_client`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Configuration-load tests, M1, and M2. |
| AC2 | TODO | Bounded-type unit test and M3. |
| AC3 | TODO | Type definition and absence of a `Validator` rule. |
| AC4 | TODO | UDP response code and M3. |
| AC5 | TODO | `linter all` and test output. |
| AC6 | TODO | `manual-verification-evidence.md`. |

## Risks and Trade-offs

- A future delivery protocol with a tighter limit lowers the bound for every protocol, which is a
  breaking configuration change. The ADR accepts this in exchange for one unambiguous value.
- Rejecting a previously accepted configuration breaks deployments with an absurd interval; the
  error message must make the fix obvious.

## Implementation Completion Review

- Retrospective: `Not yet assessed`. Create `implementation-retrospective.md` if the type design
  changes during implementation or yields a reusable lesson; otherwise record why none is needed
  in the Progress Log.

## References

- Related issues: #2245 (origin), #2243 (numeric conversion EPIC), #1978 (parent EPIC)
- Related PRs: #2452 (wire clamp), #2455 (#2245 archive)
- Related ADRs: see Architectural Decisions
