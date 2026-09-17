---
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2243
github-issue: 2245
spec-path: docs/issues/open/2245-2243-review-numeric-protocol-wire-conversions/ISSUE.md
branch: "2245-2243-review-numeric-protocol-wire-conversions"
related-pr: null
last-updated-utc: 2026-09-16 12:20
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2158-2003-inventory-existing-clippy-allows/clippy-allow-inventory.md
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

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Inventory narrowing conversions | List each cast covered by A156 and A171 with its wire field and the source of its bound. |
| T2 | TODO | Judge each cast | Per cast: bound guaranteed (retain with reason), bound assumed (decide enforce or document), or defective. |
| T3 | TODO | Apply outcomes | Add item-level reasons; implement and test bounded alternatives where chosen; retire the crate-level attribute. |
| T4 | TODO | Reconcile inventory | Update #2158 evidence for A156 and A171 with each outcome. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T2 | Review table in this specification | Commit after maintainer review of the outcomes. |
| T3 | One protocol boundary's reasons, alternatives, and tests | Commit after focused validation and required review. |
| T4 | Inventory evidence | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/numeric-conversion-wire-review/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
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

- 2026-09-15 14:46 UTC - GitHub Copilot - Drafted from #2158's wire numeric conversion design input; assigned exclusive ownership of A156 and A171 - Awaiting maintainer review
- 2026-09-16 12:20 UTC - josecelano - Reframed as a review with retain-with-reason as a valid outcome - Chat decision

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
| M1 | Announce response bounds | Send a local UDP announce request with the interval configured at a documented boundary. | The response encodes the documented value. | TODO | `manual-verification-evidence.md` section M1 |

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
