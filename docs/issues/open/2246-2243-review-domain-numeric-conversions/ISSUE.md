---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 2243
github-issue: 2246
spec-path: docs/issues/open/2246-2243-review-domain-numeric-conversions/ISSUE.md
branch: "2246-2243-review-domain-numeric-conversions"
related-pr: null
last-updated-utc: "2026-10-05 15:04"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/clippy-allow-inventory.md
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
- Apply the shared [Clippy exception decision framework](../../closed/2158-2003-inventory-existing-clippy-allows/clippy-exception-decision-framework.md)
  when deciding whether a retained conversion exception is permanent or temporary.

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

## Review Outcomes

Approved by the maintainer on 2026-10-05. All three allowances are removed, with different
outcomes:

- A129 takes decision-framework outcome 1: a behaviour-preserving fix.
- A099 and A123 change behaviour for out-of-range values only. The old casts silently truncated or
  wrapped; the new conversions fail explicitly, with a serde error (A099) or a panic (A123). This
  is the EPIC review policy's safer-conversion outcome. For A099, the public function's documented
  error contract gains a case, which decision-framework outcome 2 permits ahead of 4.0.0. Neither
  out-of-range value is reachable in production: peer timestamps come from the clock, and A123 is
  in a benchmarking-only crate.

| Entry | Source | Source/target bounds | Outcome | Validation |
| ----- | ------ | -------------------- | ------- | ---------- |
| A099 | `primitives` `ser_unix_time_value` | `Duration::as_millis()` is `u128`; `u64` milliseconds cover ~584 million years, but `Duration` can exceed that and `as u64` silently truncates. | Replace with `u64::try_from` and fail serialization with `serde::ser::Error::custom` when out of range. | Unit tests: a normal timestamp, `u64::MAX` ms, and the first value beyond it (error). |
| A123 | `torrent-repository-benchmarking` `EntrySingle::get_swarm_metadata` | Seeder/leecher counts are `usize`; `SwarmMetadata` fields are `u32`. A swarm above `u32::MAX` peers is not realistic, but `as u32` would silently wrap. | Replace with `u32::try_from(..).expect(..)` in a `peer_count_as_u32` helper, mirroring the reverse conversion in the production `Coordinator::seeders_and_leechers`. | Unit tests on the helper: `u32::MAX` converts; on 64-bit targets `u32::MAX + 1` panics (on 32-bit targets the conversion cannot fail). |
| A129 | `tracker-core` `PeersWanted::from_client_request` | Guarded `i32` with `value > 0`; target `usize`. | Replace `value as usize` with `Self::only(value.unsigned_abs())`: `u32` carries the non-negative invariant, the function stays `const`, and `u32 -> usize` is not linted. | Existing tests for -1, 0, max-1, max, and max+1. |

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | State each local invariant | See Review Outcomes. |
| T2 | DONE | Judge each cast | See Review Outcomes. |
| T3 | DONE | Apply outcomes | One commit per package: tracker-core (A129), primitives (A099), torrent-repository-benchmarking (A123). |
| T4 | DONE | Reconcile inventory | Edited the A099, A123, and A129 rows of the closed #2158 inventory in place, following the #2261 precedent. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T2 | Review table in this specification | Commit after maintainer review of the outcomes. |
| T3 | One owned package's reasons, alternatives, and tests | Commit after focused validation and required review. |
| T4 | Inventory evidence for that package | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/numeric-conversion-domain-review/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation (#2247)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-15 14:46 UTC - GitHub Copilot - Drafted from the #2158 domain numeric conversion design input - Awaiting maintainer review
- 2026-09-16 12:20 UTC - josecelano - Reframed as a review with retain-with-reason as a valid outcome - Chat decision
- 2026-10-05 12:54 UTC - josecelano - Approved T1-T2 outcomes (all three allowances removed), in-place #2158 inventory update, and replacing M2 because the REST API never calls `ser_unix_time_value` - Chat decision
- 2026-10-05 13:18 UTC - GitHub Copilot - Implemented T3 (three per-package commits) and T4 (inventory reconciliation); M1, pre-push checks, and the completion review remain - In progress
- 2026-10-05 15:04 UTC - GitHub Copilot - M1 passed against a local tracker (cap 2; `numwant` 0, -1, 1, 2, 3, `i32::MAX`); pre-push checks passed; no retrospective needed (see Implementation Completion Review) - Ready for PR

## Acceptance Criteria

- [x] A099, A123, and A129 each have a documented source/target range contract and outcome.
- [x] Every retained cast carries a native `reason` stating its invariant.
- [x] Every changed conversion has a focused test for the stated bound or new behavior.
- [x] `linter all` exits with code `0` and relevant package tests pass.

## Verification Plan

### Automatic Checks

- Focused primitives, torrent repository benchmarking, and tracker core tests.
- Focused Clippy checks, `linter all`, and required pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Announce `numwant` boundary | Announce to a local UDP tracker with a positive `numwant` at the documented boundary and with `numwant=0`. | Positive values cap at the tracker limit; zero returns as many peers as possible. | DONE | `manual-verification-evidence.md` section V1 - M1 |

A099 and A123 have no manual scenario. The REST API peer resource computes its own `u128`
timestamps and never calls `ser_unix_time_value`; only tracker-core tests serialize `peer::Peer`
with serde. A123 lives in a benchmarking-only crate. Both are covered by unit tests.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | Review Outcomes table; #2158 inventory rows A099, A123, A129. |
| AC2 | DONE | No cast retained: all three allowances removed; `grep -rn "#2246"` over Rust sources finds no remaining temporary reason. |
| AC3 | DONE | A099: three new `unix_time_value_serialization` tests (the out-of-range test failed against the old cast). A129: existing `from_client_request` boundary tests. A123: two new `peer_count_as_u32` tests (the 64-bit `u32::MAX + 1` test failed against a wrapping cast). |
| AC4 | DONE | Focused Clippy and tests pass for the three packages; `linter all` passes in every pre-commit run; pre-push checks (nightly fmt/check/doc and the full test suite) pass. |

## Risks and Trade-offs

- A conversion that is safe on one architecture may not be safe on every supported target. State
  the actual type bounds and test the selected deterministic behavior.

## Implementation Completion Review

- Retrospective: Not needed. The design held: every entry took the approved outcome with no
  surprises during implementation. The only invalidated assumption (M2 targeting the REST API,
  which never calls `ser_unix_time_value`) was caught and corrected during spec review and is
  recorded in the Progress Log and Manual Verification Scenarios.
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in
  this folder if the work invalidates an assumption, changes design materially, or yields a reusable
  lesson; otherwise add a progress-log entry stating why none is needed.

## References

- Related issues: #2158
- Related PRs: None
- Related ADRs: None
