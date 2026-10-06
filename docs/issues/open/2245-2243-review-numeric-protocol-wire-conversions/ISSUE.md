---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 2243
github-issue: 2245
spec-path: docs/issues/open/2245-2243-review-numeric-protocol-wire-conversions/ISSUE.md
branch: "2245-2243-review-numeric-protocol-wire-conversions"
related-pr: null
last-updated-utc: "2026-10-06 11:32"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/clippy-allow-inventory.md
---

<!-- skill-link: create-issue -->

# Issue #2245 - Review Numeric Protocol Wire Conversions

**Parent EPIC:** #2243 - Review Numeric Conversion Boundaries

## Goal

Review the UDP protocol narrowing conversions, record which are guaranteed by BEP field widths or
upstream bounds, and leave each retained cast with a native `reason` or a clearer bounded expression.

## Background

The vendored UDP protocol crate broadly allows `cast_possible_truncation`, and UDP announce
response construction narrows tracker values (`announce_interval`, seeder and leecher counts) to
signed 32-bit wire fields. Some of these are inherent to the protocol: BEP 15 fixes the field
widths, and the values are bounded by configuration or by realistic swarm sizes. The review must
record, per conversion, what bounds the input, whether that bound is enforced or assumed, and
whether the crate-level allowance can become item-level reasons.

## Scope

### In Scope

- #2158 entries A156 and A171 only.
- UDP protocol and UDP server conversion boundaries and their protocol newtypes.
- For each conversion: retain with a native item-level `reason`, replace with a bounded expression,
  or fix a demonstrated defect. The crate-level A156 allowance is expected to become item-level
  attributes so each retained cast carries its own reason.
- Apply the shared [Clippy exception decision framework](../../closed/2158-2003-inventory-existing-clippy-allows/clippy-exception-decision-framework.md)
  when deciding whether a retained conversion exception is permanent or temporary.

### Out of Scope

- The twelve nonnumeric UDP crate-level allowances.
- Metric aggregate and independent domain conversion entries.
- Changing BEP-defined field widths or response semantics without an approved protocol decision.

## Architectural Decisions

- Related ADRs: None.
- ADRs to create: Create one if checked protocol-boundary behavior changes externally observable
  response handling beyond the current protocol contract.

## Design and Ownership Review

Not applicable: this work changes protocol conversion boundaries and does not introduce child
processes, asynchronous I/O, network readiness, resource cleanup, or reusable test fixtures.

## Review Outcomes

Approved by the maintainer on 2026-10-06. Both allowances are removed.

### Conversion Inventory (T1)

| Entry | Source | Wire field | Source/target bounds |
| ----- | ------ | ---------- | -------------------- |
| A156 | `udp-protocol` `Request::parse_bytes`, scrape branch: `bytes.position() as usize` | None: a slice offset after reading the scrape header | Removing the crate-level allowance shows this is the only cast it hid. After three fixed-width reads (`i64`, `i32`, `i32`) the `Cursor` position is 16 and can never exceed the slice length, which is already a `usize`. The cast cannot truncate. |
| A171 | `udp-server` `build_response`: announce interval | `announce_interval` (`i32`, BEP 15) | `announce_policy.interval` is a `u32` with no configuration bound. **Defect, reproduced:** with `interval = 2147483648` a local tracker answers `"announce_interval": -2147483648`. HTTP is unaffected (bencode carries the `u32`). |
| A171 | `udp-server` `build_response`: seeders and leechers | `seeders`, `leechers` (`i32`, BEP 15) | `SwarmMetadata.complete`/`.incomplete` are `u32` counters. A swarm above `i32::MAX` peers is not realistic, but `as i32` would turn it negative on the wire. |

### Decisions (T2)

**A156: `usize::try_from(bytes.position())`, mapped to the parser's existing `invalid data` error
(decision-framework outcome 1, behaviour-preserving).** The crate-level allowance is removed. A
one-line source comment states the invariant that keeps the error branch unreachable. The first
approved form, `try_from(..).expect(..)`, was revised during implementation: pedantic Clippy
(`missing_panics_doc`) would have required the public `Request::parse_bytes`, which parses every
incoming packet, to document a panic that cannot happen. A parse error keeps the network parser
panic-free at the same size. Rejected alternatives:

- Parse the fixed 16-byte scrape header straight from the slice (split it off, or use `zerocopy` as
  the announce branch does), without a `Cursor`. This removes the conversion entirely, but it is a
  larger rewrite of working vendored parser code. It also touches the error paths, which must keep
  the same error kinds for short inputs, and that risk is not justified by an unreachable truncation.
- Keep `as usize` behind an item-level allowance with a permanent reason. Rejected because a
  behaviour-preserving fix exists (framework rule 1).

**A171 interval: reject at configuration load, and clamp when encoding (defect fix).**

- `Core::validate` (v3 schema), already run at startup by `bootstrap::app::setup`, rejects an
  `announce_policy.interval` above `i32::MAX` with a new `SemanticValidationError` variant, so
  the tracker fails fast instead of serving invalid UDP replies. This changes which configurations
  are accepted, but only for values that already produced invalid replies; the 4.0.0 major release
  permits that breaking change. No ADR: response handling stays within the BEP 15 contract. Only
  `interval` is checked: `interval_min` is never sent over UDP, and the v2 schema is kept only for
  backward compatibility.
- `build_response` also clamps the interval to `i32::MAX`, because `Core` can be constructed
  without validation (for example in tests or by code that uses the crates as libraries).
- Rejected: clamping alone (an absurd configuration would be accepted without complaint, and UDP
  and HTTP would report different intervals), and validation alone (the conversion in
  `udp-server` would still need handling for unvalidated `Core` values).

**A171 seeders and leechers: clamp to `i32::MAX`.** One private helper converts all three wire
fields, so the wire value is never negative, and the boundary tests target the helper directly.
Rejected: an item-level allowance with a "swarms never reach 2^31 peers" reason (an unenforced
bound next to code being fixed anyway), and `expect` (a panic in a request handler driven by swarm
size).

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Inventory narrowing conversions | See Conversion Inventory. |
| T2 | DONE | Judge each cast | See Decisions. |
| T3 | TODO | Apply outcomes | One commit per boundary: `udp-protocol` (A156), `configuration` (interval validation), `udp-server` (A171 clamping). |
| T4 | TODO | Reconcile inventory | Edit the A156 and A171 rows of the closed #2158 inventory in place, following the #2246 precedent. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T2 | Review table in this specification | Commit after maintainer review of the outcomes. |
| T3 | One protocol boundary's reasons, alternatives, and tests | Commit after focused validation and required review. |
| T4 | Inventory evidence | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/numeric-conversion-wire-review/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation (#2247)
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-15 14:46 UTC - GitHub Copilot - Drafted from #2158's wire numeric conversion design input; assigned exclusive ownership of A156 and A171 - Awaiting maintainer review
- 2026-09-16 12:20 UTC - josecelano - Reframed as a review with retain-with-reason as a valid outcome - Chat decision
- 2026-10-06 11:24 UTC - josecelano - Approved T1-T2: A156 `try_from` + `expect` with the rationale recorded here; A171 interval rejected at configuration load and clamped when encoding (defect reproduced locally); seeders and leechers clamped through a shared helper; no ADR - Chat decision
- 2026-10-06 11:32 UTC - josecelano - Revised A156 from `expect` to a mapped parse error, because `missing_panics_doc` would have made the public parser document an impossible panic - Chat decision

## Acceptance Criteria

- [ ] Every A156/A171 narrowing conversion has a recorded bound source and outcome.
- [ ] Every retained cast carries a native item-level `reason`; the crate-level A156 attribute is retired.
- [ ] Every changed conversion has a focused test covering valid extrema and out-of-range handling.
- [ ] `linter all` exits with code `0` and relevant package tests pass.

## Verification Plan

### Automatic Checks

- UDP protocol serialization and deserialization boundary tests.
- UDP server response tests.
- Focused Clippy checks, `linter all`, and required pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Announce interval bounds | Run a local UDP tracker with `interval = 2147483647` and announce; then start it with `interval = 2147483648`. | The first response encodes `announce_interval` 2147483647; the second configuration fails validation at startup. | TODO | `manual-verification-evidence.md` section M1 |

The A156 error branch and the clamping in `build_response` cannot be reached through a validated
configuration or a realistic swarm, so they are covered by unit tests only. A156 is covered by the
existing scrape round-trip tests, because its failure path is unreachable by construction.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Pending implementation. |
| AC2 | TODO | Pending tests. |
| AC3 | TODO | #2158 inventory evidence. |
| AC4 | TODO | Pending validation. |

## Risks and Trade-offs

- Vendored protocol code may have sound but unwritten bounds. Record them rather than rewriting
  working wire code.
- Validating only at encoding can leave invalid values circulating internally. Prefer the narrowest
  boundary that can prove the value is valid without changing established wire behavior.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in
  this folder if the work invalidates an assumption, changes design materially, or yields a reusable
  lesson; otherwise add a progress-log entry stating why none is needed.

## References

- Related issues: #2158
- Related PRs: None
- Related ADRs: None
