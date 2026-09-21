---
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2243
github-issue: 2244
spec-path: docs/issues/open/2244-2243-review-metric-aggregate-conversions/ISSUE.md
branch: "2244-2243-review-metric-aggregate-conversions"
related-pr: null
last-updated-utc: 2026-09-18 14:40
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/clippy-allow-inventory.md
---

<!-- skill-link: create-issue -->

# Issue #2244 - Review Metric Aggregate Conversions

**Parent EPIC:** #2243 - Review Numeric Conversion Boundaries

## Goal

Review each cast at a metric boundary, decide whether it is correct for the metric's semantics, and
leave every retained cast with a native `reason` or replace it with a clearer expression.

## Background

Two cast directions are involved. Aggregate outputs return `f64` for gauges and averages, and the
tracker casts them to `u64` (`cast_sign_loss`, `cast_possible_truncation`). Gauge inputs cast
`usize` counts to `f64` (`cast_precision_loss`). Many of these are ordinary: a counter is an
integer, but its mean over time is a float, and a count far below `2^53` converts exactly. The
reasons were never recorded, so the review must state, per metric, whether the value is an integer
counter, a gauge, or an average, what its bounds are, and what the cast does at the edges.

## Scope

### In Scope

- #2158 entries A080-A087, A113-A114, A143-A154, A170, A178-A222, and A224-A227.
- Both directions: `f64` aggregate output to integer, and integer count to `f64` gauge input.
- Metric semantics and, where a bound or behavior changes, focused tests in HTTP core, UDP core,
  UDP server, and swarm coordination registry as required by the owned entries.
- For each owned entry: retain with a native `reason`, replace with a clearer conversion, or fix a
  demonstrated defect.
- Apply the shared [Clippy exception decision framework](../../closed/2158-2003-inventory-existing-clippy-allows/clippy-exception-decision-framework.md)
  when deciding whether a retained conversion exception is permanent or temporary.

### Out of Scope

- Protocol-width conversions owned by the wire-validation child issue.
- Domain conversions owned by the domain-contract child issue.
- A change to `torrust-metrics` unless tracker-owned boundaries cannot express the required policy.

## Architectural Decisions

- Related ADRs: None.
- ADRs to create: Create one only if metric aggregate semantics require a repository-wide API policy.

## Design and Ownership Review

Not applicable: this work changes local conversion boundaries and does not introduce child
processes, asynchronous I/O, network readiness, resource cleanup, or reusable test fixtures.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Classify aggregate semantics | Record whether each owned value is an integer counter, gauge, or average, and its realistic bounds. |
| T2 | TODO | Judge each cast | Per entry: correct as-is, clearer alternative available, or defective. Record edge behavior (fractional, negative, non-finite, out of range). |
| T3 | TODO | Apply outcomes | Add native reasons to retained casts; implement and test alternatives or fixes where chosen. |
| T4 | TODO | Reconcile inventory | Update #2158 evidence by ID with each outcome. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T2 | Review table in this specification | Commit after maintainer review of the outcomes. |
| T3 | One package's reasons, alternatives, and tests | Commit after focused validation and required review. |
| T4 | Inventory evidence for that package | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/numeric-conversion-metric-aggregate-review/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-15 14:46 UTC - GitHub Copilot - Drafted from the #2158 metric aggregate design input - Awaiting maintainer review
- 2026-09-16 12:20 UTC - josecelano - Reframed as a review with retain-with-reason as a valid outcome - Chat decision

## Acceptance Criteria

- [ ] Every owned entry has a recorded semantics classification, bounds, and outcome.
- [ ] Every retained cast carries a native `reason` stating its invariant.
- [ ] Every changed conversion has a focused test for the stated bound or new behavior.
- [ ] `linter all` exits with code `0` and relevant package tests pass.

## Verification Plan

### Automatic Checks

- Focused HTTP core, UDP core, UDP server, and swarm-registry tests for changed packages.
- Focused Clippy checks for changed packages.
- `linter all` and required pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Inspect metric output | Run a local tracker with metric collection enabled and inspect the affected statistics endpoint or output. | Integer and average metrics match their documented semantics. | TODO | `manual-verification-evidence.md` section M1 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Pending implementation. |
| AC2 | TODO | Pending tests. |
| AC3 | TODO | #2158 inventory evidence. |
| AC4 | TODO | Pending validation. |

## Risks and Trade-offs

- Treating every cast as a defect leads to needless helpers and behavior changes. Retaining a
  documented cast is a complete outcome.
- Saturating, rejecting, rounding, and retaining floating values express different metric contracts.
  Choose behavior per metric semantics rather than applying one conversion helper.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in
  this folder if the work invalidates an assumption, changes design materially, or yields a reusable
  lesson; otherwise add a progress-log entry stating why none is needed.

## References

- Related issues: #2158
- Related PRs: None
- Related ADRs: None
