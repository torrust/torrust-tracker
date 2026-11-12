---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: null
github-issue: 2842
spec-path: docs/issues/open/2842-inbound-tolerance-adr/ISSUE.md
branch: "2842-inbound-tolerance-adr"
related-pr: null
last-updated-utc: "2026-11-11 17:10"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/create-adr/SKILL.md
    - docs/adrs/20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md
---

<!-- skill-link: create-issue -->

# Issue #2842 - ADR: reject malformed inbound protocol values unless real clients need the tolerance

## Goal

Record a repository-level ADR that says when the tracker tolerates a non-conforming client request
and when it rejects it, so UDP and HTTP stop disagreeing by accident.

## Background

Found while writing the `primitives` tests for EPIC #1347. The UDP announce accepts a negative
`left` (signed 64-bit on the wire), while HTTP rejects it, and `Peer::is_seeder` treats a negative
`left` as a seeder. Bug #2838 (PR #2839) fixes that case; it needs a rule to
apply. No ADR covers inbound values: the existing
[tightest-protocol ADR](../../../adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md)
covers values the tracker sends.

Maintainer position (2026-11-11): tolerant implementations accept the widest range of clients,
but the maintainer prefers strictness to encourage good implementations, unless a meaningful
share of real clients relies on the tolerant behavior.

## Scope

### In Scope

- One ADR in `docs/adrs/` with the rule, a register of tolerances (empty), and an inventory of the
  existing inbound leniencies found in `http-protocol`, `udp-protocol` and `udp-server`.
- The row in `docs/adrs/index.md`.

### Out of Scope

- Any production-code change, including the `left` fix, the rejection metrics, and aligning the
  other inventory items. Each becomes its own issue after the ADR is merged.

## Architectural Decisions

- Related ADRs: the tightest-protocol, value-invariant, newtype, protocol/domain decoupling,
  scrape-cap and announce-`ip` ADRs (linked from the ADR).
- ADRs to create: `20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md`.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

| ID  | Status | Task                                         | Notes / Expected Output                           |
| --- | ------ | -------------------------------------------- | ------------------------------------------------- |
| T1  | DONE   | Draft the ADR and inventory                  | ADR file written, inventory checked against code  |
| T2  | TODO   | Maintainer review and decisions              | Open questions in the PR resolved                 |
| T3  | TODO   | Index row, `linter all`, PR                  | Docs-only PR, `Closes #<issue>`                   |
| T4  | TODO   | Propose follow-up issues from the inventory  | One issue per leniency to apply the rule          |

## Commit Points

| Task | Coherent change set                  | Commit policy                                        |
| ---- | ------------------------------------ | ---------------------------------------------------- |
| T1   | ADR file plus index row              | Commit after `linter all`.                           |
| T3   | Issue spec moved to `docs/issues/open/` | Commit after `linter all`.                        |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2842-inbound-tolerance-adr/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-11 17:10 UTC - Claude Code (Claude Sonnet 5.5) - Drafted spec and ADR.
- 2026-11-12 14:00 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; issue #2842 created.

## Acceptance Criteria

- [ ] AC1: The ADR states the rule (strict by default, evidence-based tolerance, recorded in code
      and ADR, measurable, same over every delivery protocol, decided per field).
- [ ] AC2: The ADR cites bug #2838 (PR #2839) as the first application and the existing
      related ADRs.
- [ ] AC3: The inventory matches the current code (each row verified or marked "to verify").
- [ ] AC4: No production code changes.
- [ ] `linter all` exits with code `0`

## Verification Plan

### Automatic Checks

- `linter all` (markdown, cspell, links)

### Manual Verification Scenarios

Not applicable: documentation-only change, no behavior to exercise.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   | ADR PR   |
| AC2   | TODO   | ADR PR   |
| AC3   | TODO   | ADR PR   |
| AC4   | TODO   | PR diff  |

## Risks and Trade-offs

- The inventory can go stale; it is labeled as a snapshot and each row links to its follow-up.
- The rule commits the project to adding a rejection metric; that work is tracked separately.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Related issues: #1347 (EPIC), #2838 (fixed by PR #2839)
- Related ADRs: `docs/adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md`
