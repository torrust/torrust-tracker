---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p2
epic: 1978
github-issue: 2466
spec-path: docs/issues/closed/2466-1978-announce-interval-upper-bound/ISSUE.md
branch: "2466-1978-announce-interval-upper-bound"
related-pr: 2680
last-updated-utc: "2026-10-28 13:35"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md
    - docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
    - docs/adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md
    - "issue #2245"
    - packages/primitives/src/announce.rs
    - packages/configuration/docs/migrate-v2-to-v3.md
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
UDP clients receive `2147483647` while HTTP announces report the configured `2147483648`
(reproduced on 2026-10-07; see `manual-verification-evidence.md` section V0).

The #2245 plan first rejected the value in `Core::validate`. That conflicts with the configuration
validation ADR, which classifies a single-value bound as a **value invariant**: it must be a typed
newtype rejected during deserialization, and one-field rules must not be added to `Validator`. See
the [#2245 issue](https://github.com/torrust/torrust-tracker/issues/2245), its specification, and
its implementation retrospective.

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
   directly by the configuration, as `AnnouncePolicy` already is. `interval_min` uses the same type
   because it measures the same real-world quantity as `interval`, a re-announce interval in
   seconds. No protocol imposes the bound on it: BEP 15 has no minimum-interval field, and only
   HTTP sends `min interval`.
3. **Parent EPIC.** Configuration EPIC #1978 is reopened and this issue is added as its subissue.
   Configuration schema v3.0.0 is active in the code but not yet published on crates.io.
4. **Breaking change accepted.** Changing the `AnnouncePolicy` field types is acceptable: the
   tracker binary and library are heading for the 4.0.0 major release, and the other workspace
   packages version independently.
5. **Error message.** The error names the field, the configured value, and the limit
   (`2147483647`).
6. **ADR.** The rule in decision 1 is recorded as a repository-wide ADR, because it will apply to
   other numeric configuration values and future delivery protocols.
7. **Type name** (recorded 2026-10-27, before implementation). The bounded type is `AnnounceIntervalInSecs`, following the
   `IpBansResetIntervalInSecs` precedent.
8. **Representation and accessors.** The type wraps a `u32`. `get()` returns `u32`, and
   `From<AnnounceIntervalInSecs>` is implemented for `i64` (HTTP), and `get_i32()` returns the
   value as `i32` (UDP). Neither clamps, because the bound guarantees both conversions are
   infallible.
9. **Constructors.** `new(u32)` and `TryFrom<u32>` are fallible; `From<u16>` is infallible
   because every `u16` is within the bound, which keeps tests and literals ergonomic.
10. **Error naming the field.** The type's error states the value and the limit; the field name
    comes from the key path of the TOML loader error. The configuration-load tests assert that the
    final load error contains all three. If the loader error does not name the key, the maintainer
    is consulted before adding any per-field wrapper, which the ADRs discourage.
11. **Serialization and v2.** `Serialize` emits a plain integer, so configuration round-trips and
    generated documentation are unchanged. `v2_0_0` inherits the bound, as already scoped.

## Scope

### In Scope

- A bounded interval type in `primitives` whose construction and `Deserialize` reject values above
  `i32::MAX`, following the value-invariant ADR and the constrained-field newtype ADR.
- Changing `AnnouncePolicy::interval` and `AnnouncePolicy::interval_min` to that type and updating
  their consumers (`tracker-core`, `http-protocol`, `axum-http-server`, `udp-server`,
  `configuration`, and test or console consumers).
- Converting the UDP interval through the type without clamping. `saturating_wire_i32` remains for
  the seeder and leecher counts, which no configuration bounds.
- Module-level doc comments on the bounded type and the UDP interval conversion that link back to
  the ADR.
- Updating `packages/configuration/docs/migrate-v2-to-v3.md`, as the parent EPIC requires for a
  configuration public API change: a quick-reference row and the new bound on
  `[core.announce_policy]`.

### Out of Scope

- An `interval_min <= interval` consistency rule; that would be a separate `Validator` rule.
- Other changes to the v2 configuration schema. v2 shares `AnnouncePolicy` with v3, so it inherits
  the bound; this reaches only library users of `v2_0_0`, because the tracker no longer loads v2
  at runtime.
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
- Reproduction: **Reproduced** before review (2026-10-07). With `interval = 2147483648` the
  tracker starts, the UDP announce returns `2147483647` (clamped), and the HTTP announce returns
  `2147483648`. See `manual-verification-evidence.md` section V0.
- Regression test boundary: configuration-load tests, written first and recorded red against the
  current `u32` fields (T2); then construction and deserialization of the bounded type (unit tests
  at `i32::MAX` and `i32::MAX + 1`).

## Regression Test Strategy

Configuration-load tests are the regression tests: loading a TOML with `interval = 2147483648` or
`interval_min = 2147483648` must fail with an error naming the field, the value (`2147483648`),
and the limit (`2147483647`). They are written first and fail against the current `u32` fields;
the red run is recorded in the evidence file before the fix (T2), and the green run after it (T7).
Unit tests on the bounded type cover `2147483647` accepted and `2147483648` rejected, both through
construction and through `Deserialize`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Resolve the open questions | Decisions section; ADR added in the specification PR. |
| T2 | DONE | Write the configuration-load regression tests | Assert field, value, and limit for `interval` and `interval_min`. Red run recorded in the evidence file; the T2 tests passed after T4. |
| T3 | DONE | Add the bounded type to `primitives` | `AnnounceIntervalInSecs` in `packages/primitives/src/announce_interval.rs`; boundary unit tests; module doc links the ADR. |
| T4 | DONE | Fix: adopt it for `interval` and `interval_min` | `AnnouncePolicy` and consumers updated (`axum-http-server`, `http-core`, `tracker-core`, `udp-server`); the T2 tests pass. `http-protocol` keeps its own wire policy type (it does not depend on `primitives`), so the Axum handler converts with `get()`. |
| T5 | DONE | Convert the UDP interval through the type | `announce_interval_on_the_wire` converts without clamping; `saturating_wire_i32` kept for peer counts; doc comment links the ADR. |
| T6 | DONE | Update the migration guide | Quick-reference row, Step 11, and a checklist item in `migrate-v2-to-v3.md`. |
| T7 | DONE | Green run and recheck | T2 tests green; M1 "after", M2, and M3 recorded in the evidence file. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T3 | Bounded type and its boundary unit tests | Commit after focused validation and test-design review. |
| T2 + T4 | Regression tests, field type change, consumer updates | One commit: the red tests cannot pass the pre-push gate alone. The red run stays in the evidence file. |
| T5 | UDP interval conversion | Commit after focused validation; may merge into T2 + T4 if the type change forces it. |
| T6 | Migration guide update | Separate documentation commit. |
| T7 | Evidence and spec progress | Separate documentation commit. |

Tests follow the `write-unit-test` skill, with the prose-first Arrange-Act-Assert review after each
passing increment and before maintainer review and commit. Commits are signed Conventional Commits
with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1978-announce-interval-upper-bound/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation (PR #2468)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-06 12:29 UTC - GitHub Copilot - Local draft written from the #2245 follow-up decision - Parked until PR #2452 merged
- 2026-10-06 16:41 UTC - GitHub Copilot - Moved to `docs/issues/drafts/` after #2245 was archived (PR #2455); refreshed facts against `develop`; added Commit Points and Acceptance Verification - Awaiting maintainer review
- 2026-10-07 08:29 UTC - josecelano - Answered the open questions: one protocol-agnostic value bounded by the tightest delivery protocol, type in `primitives` (also for `interval_min`), subissue of reopened #1978, breaking change accepted for 4.0.0, ADR in the specification PR - Chat decision
- 2026-10-07 08:59 UTC - GitHub Copilot - Maintainer approved the specification; created #2466, reopened #1978 and linked #2466 as its subissue; moved this spec to `docs/issues/open/` - Specification PR
- 2026-10-07 10:24 UTC - GitHub Copilot - Applied PR #2468 round-1 review fixes: reproduced the defect before implementation (V0, Reproduced), added the red-run regression task and the migration-guide task, asserted field, value, and limit in tests and scenarios, restored the verification notes and two template checkpoints, scoped the v2 inheritance, and cited #2245 by issue number - In review
- 2026-10-07 13:12 UTC - josecelano - Kept one type for `interval` and `interval_min` because both measure the same quantity, although no protocol bounds `interval_min` (Copilot's review overview had noted the missing protocol justification) - Chat decision
- 2026-10-27 15:00 UTC - josecelano - Pre-implementation questions answered: type `AnnounceIntervalInSecs` wrapping `u32`, infallible `From` to `i64` plus `get_i32()` for UDP, fallible `new(u32)` plus `From<u16>`, field name taken from the loader key path, serialization and v2 inheritance confirmed (decisions 7-11) - Chat decision
- 2026-10-28 07:35 UTC - GitHub Copilot - Implemented T2-T7 on branch `2466-1978-announce-interval-upper-bound`: regression tests (red then green), `AnnounceIntervalInSecs`, `AnnouncePolicy` and consumers, unclamped UDP interval, migration guide Step 11, and manual scenarios M1-M3 (V1-V3) - Awaiting maintainer review
- 2026-10-28 13:03 UTC - josecelano - Merged PR #2680 (implementation); GitHub closed #2466 two seconds later through the `Closes` keyword - Merged
- 2026-10-28 13:35 UTC - GitHub Copilot - Archived this spec to `docs/issues/closed/`: `status: done`, `spec-path`, `related-pr`, and the acceptance and workflow checkboxes updated from the recorded evidence; the "independent reviewer reports" checkpoint stays open because no `agent-review-reports.md` was recorded - Archive PR

## Acceptance Criteria

- [x] AC1: A configuration with `core.announce_policy.interval` or `interval_min` above
  `2147483647` fails to load with an error naming the field, the value, and the limit.
- [x] AC2: `2147483647` is accepted, and UDP and HTTP announces report the same interval.
- [x] AC3: The bound is enforced by a type in `primitives`, not by `Validator`, as the ADRs require.
- [x] AC4: The UDP interval is converted without clamping.
- [x] AC5: `linter all` exits with code `0` and relevant tests pass.
- [x] AC6: Manual verification scenarios are executed and documented in issue-local
  `manual-verification-evidence.md`.
- [x] AC7: `packages/configuration/docs/migrate-v2-to-v3.md` documents the new bound.

## Verification Plan

### Automatic Checks

- Bounded-type unit tests and configuration-load tests (stable Rust toolchain).
- Focused Clippy for changed packages, `linter all`, and pre-push checks.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Reproduce, then reject an out-of-range interval | Start a local tracker with `interval = 2147483648`, before and after the fix. | Before: starts; UDP reports `2147483647`, HTTP `2147483648`. After: startup fails with an error naming `interval`, the value `2147483648`, and the limit `2147483647`. | DONE | `manual-verification-evidence.md` sections V0 and V1 |
| M2 | Reject an out-of-range minimum interval | Start with `interval_min = 2147483648`. | Startup fails with an error naming `interval_min`, the value `2147483648`, and the limit `2147483647`. | DONE | `manual-verification-evidence.md` section V2 |
| M3 | Accept the boundary | Start with `interval = 2147483647` and announce over UDP and HTTP. | Both report `2147483647`. | DONE | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented
  use of the feature or reproduction of the bug fix, not a simulated result and not merely running
  automated tests.
- Every recorded validation command result must identify the toolchain or runtime that produced
  it when one can affect behavior; M1-M3 record the Rust toolchain that built the tracker and
  `tracker_client`.
- `manual-verification-evidence.md` follows `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
  Record actual prerequisites, actions, commands, program output, relevant tracker logs, and
  outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The scenarios use the tracker binary and `tracker_client`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | Configuration-load tests (`it_should_reject_*announce_interval_above_the_i32_maximum_*`), V1 and V2 in the evidence file. |
| AC2 | DONE | `it_should_accept_the_i32_maximum_for_both_announce_intervals`, the bounded-type unit tests, and V3. |
| AC3 | DONE | `AnnounceIntervalInSecs` in `primitives`; no rule added to `Validator`. |
| AC4 | DONE | `announce_interval_on_the_wire` in `udp-server`, its handler test, and V3. |
| AC5 | DONE | `linter all` (run by the pre-commit hook on every commit) and `pre-push.sh` (nightly format, check and docs, packaging, full stable test suite) exited `0` on 2026-10-28. |
| AC6 | DONE | `manual-verification-evidence.md` sections V0-V3. |
| AC7 | DONE | Step 11, the quick-reference row, and the checklist item in `migrate-v2-to-v3.md`. |

## Risks and Trade-offs

- A future delivery protocol with a tighter limit lowers the bound for every protocol, which is a
  breaking configuration change. The ADR accepts this in exchange for one unambiguous value.
- Rejecting a previously accepted configuration breaks deployments with an absurd interval; the
  error message must make the fix obvious.

## Implementation Completion Review

- Retrospective: not needed. The type design matched the plan except that the type's error states
  the value and the limit, and the field name comes from the loader's key path (decision 10),
  which the configuration-load tests and V1-V2 confirm. `http-protocol` keeps its own wire
  policy type, so only the Axum handler converts with `get()`.

## References

- Related issues: #2245 (origin), #2243 (numeric conversion EPIC), #1978 (parent EPIC)
- Related PRs: #2468 (specification), #2680 (implementation), #2452 (wire clamp), #2455 (#2245 archive)
- Related ADRs: see Architectural Decisions
