---
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2243
github-issue: 2246
spec-path: docs/issues/open/2246-2243-review-domain-numeric-conversions/ISSUE.md
branch: "2246-2243-review-domain-numeric-conversions"
related-pr: null
last-updated-utc: 2026-09-16 12:20
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2158-2003-inventory-existing-clippy-allows/clippy-allow-inventory.md
---

<!-- skill-link: create-issue -->

# Issue #2246 - Review Domain Numeric Conversions

**Parent EPIC:** #2243 - Review Numeric Conversion Boundaries

## Goal

Review the three independent domain narrowing conversions identified by #2158, record the
invariant behind each, and leave each retained cast with a native `reason` or a clearer expression.

## Background

Three unrelated conversions cross domain-type boundaries: peer timestamp serialization to `u64`
milliseconds (A099, `primitives`), benchmark-only swarm seeder and leecher counts to `u32` (A123,
`torrent-repository-benchmarking`), and the positive client-supplied `numwant` value to `usize` in
`PeersWanted::from_client_request` (A129, `tracker-core`). A129 already has a comment stating its
invariant (`value > 0`); the others do not. Each needs its own recorded contract rather than a
shared conclusion.

## Scope

### In Scope

- #2158 entries A099, A123, and A129.
- Boundary behavior in primitives, torrent repository benchmarking, and tracker core.
- For each entry: retain with a native `reason`, replace with `TryFrom` or a bounded type where it
  is clearer, or fix a demonstrated defect.

### Out of Scope

- Metric aggregate and protocol wire conversion work.
- Unrelated numeric casts discovered outside the three owned inventory entries.
- Broad shared conversion abstractions without demonstrated common behavior.

## Architectural Decisions

- Related ADRs: None.
- ADRs to create: None known; create one only if these separate domain contracts require a shared
  public conversion convention.

## Design and Ownership Review

Not applicable: this work changes synchronous domain conversion boundaries and does not introduce
child processes, asynchronous I/O, network readiness, resource cleanup, or reusable test fixtures.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | State each local invariant | Document source and target bounds for A099, A123, and A129, including platform assumptions. |
| T2 | TODO | Judge each cast | Per entry: lossless by invariant (retain with reason), clearer alternative available, or defective. |
| T3 | TODO | Apply outcomes | Add native reasons; implement and test alternatives or fixes where chosen. |
| T4 | TODO | Reconcile inventory | Update #2158 evidence by ID with each outcome. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T2 | Review table in this specification | Commit after maintainer review of the outcomes. |
| T3 | One owned package's reasons, alternatives, and tests | Commit after focused validation and required review. |
| T4 | Inventory evidence for that package | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/numeric-conversion-domain-review/ISSUE.md`
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

- 2026-09-15 14:46 UTC - GitHub Copilot - Drafted from the #2158 domain numeric conversion design input - Awaiting maintainer review
- 2026-09-16 12:20 UTC - josecelano - Reframed as a review with retain-with-reason as a valid outcome - Chat decision

## Acceptance Criteria

- [ ] A099, A123, and A129 each have a documented source/target range contract and outcome.
- [ ] Every retained cast carries a native `reason` stating its invariant.
- [ ] Every changed conversion has a focused test for the stated bound or new behavior.
- [ ] `linter all` exits with code `0` and relevant package tests pass.

## Verification Plan

### Automatic Checks

- Focused primitives, torrent repository benchmarking, and tracker core tests.
- Focused Clippy checks, `linter all`, and required pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Announce `numwant` boundary | Announce to a local tracker with a positive `numwant` at the documented boundary and with `numwant=0`. | Positive values cap at the tracker limit; zero returns as many peers as possible. | TODO | `manual-verification-evidence.md` section M1 |
| M2 | Timestamp serialization | Query the REST API torrent endpoint for a torrent with a recently announced peer. | The serialized peer timestamp matches the documented millisecond contract. | TODO | `manual-verification-evidence.md` section M2 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Pending review. |
| AC2 | TODO | Source attributes. |
| AC3 | TODO | Pending tests. |
| AC4 | TODO | Pending validation. |

## Risks and Trade-offs

- A conversion that is safe on one architecture may not be safe on every supported target. State
  the actual type bounds and test the selected deterministic behavior.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in
  this folder if the work invalidates an assumption, changes design materially, or yields a reusable
  lesson; otherwise add a progress-log entry stating why none is needed.

## References

- Related issues: #2158
- Related PRs: None
- Related ADRs: None
