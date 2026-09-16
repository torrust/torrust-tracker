---
doc-type: epic
status: planned
github-issue: 2243
spec-path: docs/issues/open/2243-review-numeric-conversion-boundaries/EPIC.md
epic-owner: josecelano
last-updated-utc: 2026-09-16 12:20
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2158-2003-inventory-existing-clippy-allows/ISSUE.md
    - docs/issues/open/2158-2003-inventory-existing-clippy-allows/clippy-allow-inventory.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# EPIC #2243 - Review Numeric Conversion Boundaries

## Goal

Review every temporary numeric-conversion Clippy suppression found by #2158 and give each a
documented outcome: retain with a native, source-specific `reason`; replace with a clearer or safer
conversion; or fix a demonstrated defect.

## Why This Is Needed

The #2158 inventory found 77 numeric-conversion suppressions across multiple packages without a
recorded rationale. Undocumented does not mean wrong: many conversions are mathematically normal
(integer counters averaged as floats, bounded counts encoded into fixed wire widths). The problem is
that the reasoning was never written down, so a future reader cannot tell a legitimate exception
from a latent defect. Each conversion needs a review that records what the value means, what its
bounds are, and why the chosen representation is acceptable.

## Review Policy

Apply the same three-question review to every owned conversion:

1. Is the conversion correct for the value's actual semantics and bounds? Record the invariant that
   makes it so, or the case that breaks it.
2. Is there a clearer or safer expression (a bounded type, `From`/`TryFrom`, a rounding or
   saturating operation, or keeping the float) that does not distort the domain meaning?
3. If the cast stays, add a native `reason` stating the invariant; use `#[expect]` when the lint
   fires on every supported toolchain, otherwise `#[allow(..., reason = "...")]`.

Retaining a suppression with a precise reason is a complete, valid outcome. Do not change behavior
merely to satisfy the lint. Add a focused test only where it protects a stated bound or a changed
behavior.

## Scope

### In Scope

- Review and document metric aggregate, protocol wire, and domain conversion boundaries.
- Deliver the three focused child issues below with a recorded outcome for every owned entry.
- Update #2158 inventory evidence as individual suppressions are documented, narrowed, or removed.

### Out of Scope

- Direct-removal and retained-rationale work owned by #2158.
- The twelve nonnumeric crate-level UDP protocol allowances owned by a separate follow-up.
- Numeric conversions outside the #2158 inventory unless a child issue finds a necessary adjacent case.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue | Local Spec | Status | Notes |
| ----- | ----- | ---------- | ------ | ----- |
| 1 | #2244 - Review metric aggregate conversions | `docs/issues/open/2244-2243-review-metric-aggregate-conversions/ISSUE.md` | TODO | Owns 72 metric entries: A080-A087, A113-A114, A143-A154, A170, A178-A222, A224-A227. |
| 2 | #2245 - Review numeric protocol wire conversions | `docs/issues/open/2245-2243-review-numeric-protocol-wire-conversions/ISSUE.md` | TODO | Owns A156 and A171 only. |
| 3 | #2246 - Review domain numeric conversions | `docs/issues/open/2246-2243-review-domain-numeric-conversions/ISSUE.md` | TODO | Owns A099, A123, and A129. |

## Delivery Strategy

Create this EPIC and all child issues from one reviewed specification bundle. Create the EPIC GitHub
issue first, then the child issues, and move every approved specification to `docs/issues/open/`.
Merge the resulting documentation-only PR before implementation. Each child implementation starts
from current `develop`, uses its own branch and PR, and updates #2158 by inventory ID.

The child issues may proceed independently after their shared conversion policy is agreed. When a
child reveals a repository-wide conversion design decision, stop and create an ADR before continuing.

This bundle references the #2158 inventory, which lands through the #2158 documentation PR. Merge
that PR before this specification PR so every inventory ID cited here resolves in `develop`.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review and record a retrospective or why none is needed.

### Phase 1

- Outcome: review policy agreed; child specifications approved and GitHub issues created.
- Exit criteria: this specification PR merged with all three child specifications in `docs/issues/open/`.

### Phase 2

- Outcome: each child reviewed on its own branch and PR; every owned entry retained with a reason,
  improved, or fixed.
- Exit criteria: no owned suppression lacks a native `reason`, and #2158 inventory evidence is
  updated by ID with each outcome.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [x] Subissues created and linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-15 14:46 UTC - GitHub Copilot - Drafted from #2158's three approved-for-review numeric design inputs - Awaiting maintainer review
- 2026-09-16 12:20 UTC - josecelano - Reframed as a review: undocumented exceptions may be legitimate; each conversion is judged on semantics, alternatives, and then documented with a native reason if retained - Chat decision

## Acceptance Criteria

- [ ] All three child issues are created and linked after specification-bundle approval.
- [ ] Each child has a clear non-overlapping inventory ownership boundary.
- [ ] A156 and A171 are owned only by the wire-review child issue.
- [ ] Every owned entry has a recorded outcome: retained with a native reason, improved, or fixed.
- [ ] Every implemented child records automated and manual verification evidence.
- [ ] #2158 is updated with the outcome for every owned entry.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | GitHub issues #2244, #2245, and #2246 were created and linked. |
| AC2 | DONE | Child specification scope sections define non-overlapping ownership. |
| AC3 | DONE | Wire-review specification scope section explicitly owns A156 and A171 only. |
| AC4 | TODO | Child review tables and source attributes. |
| AC5 | TODO | Child issue verification artifacts. |
| AC6 | TODO | #2158 inventory reconciliation. |

## Risks and Trade-offs

- Presuming a suppression is a defect leads to needless behavior changes. The review policy makes
  retain-with-reason a first-class outcome.
- A shared helper can hide distinct metric, wire, and domain semantics. Keep ownership with each
  boundary unless a demonstrated common abstraction is warranted.
- Changing conversion behavior can affect public protocol responses. Require boundary-value tests
  and explicit failure behavior before changing a conversion.

## References

- Related issues: #2158
- Related PRs: None
- Related ADRs: None
