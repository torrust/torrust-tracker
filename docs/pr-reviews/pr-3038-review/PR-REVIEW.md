---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
    - "issue #384"
---

<!-- skill-link: process-pr-review -->

# PR #3038 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/3038>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Review 6044345436 (Greptile, 2026-11-24 17:39 UTC) posted one inline comment, F1. Its body is only that comment, so it adds no row.
Review 6044459239 (Greptile, 2026-11-24 17:47 UTC, on the audit commit) posted one inline comment, F2; its body is empty, so it adds no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3038-f1 | Unknown | P2 | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3038-f2 | Unknown | P2 | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Parent EPIC link breaks

- PR number: 3038
- Source review ID: 6044345436
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3038#discussion_r4684080689>
- Concern: after the spec moved to `closed/`, its relative link `../2912-remove-source-locations-from-errors/EPIC.md` pointed inside `closed/`, but the EPIC is still in `open/`.
- Solution: the link is now `../../open/2912-remove-source-locations-from-errors/EPIC.md`, which resolves to the EPIC in `open/`.
- Current-tree verification: `test -f docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` succeeds, and line 27 of the archived `ISSUE.md` uses the `../../open/` form.
- Resolution reference: docs(issues): [#384] link the parent EPIC in open/ from the archived SI-7 spec
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3038#discussion_r4684154526>

### F2 - Audit lacks the reply URL

- PR number: 3038
- Source review ID: 6044459239
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3038#discussion_r4684178317>
- Concern: the audit commit the reviewer read held `Reply URL: pending` for F1, which fails `validate-audit-record.py` until the reply URL is recorded.
- Solution: the reply URL of F1 was already recorded in the commit `docs(pr-reviews): [#384] record the reply of round 1 in the PR 3038 audit`, pushed before this review was read, and the validator reports `failures: 0`. This row and its audit commit are the only change.
- Current-tree verification: `grep -c "Reply URL: pending" docs/pr-reviews/pr-3038-review/PR-REVIEW.md` prints `1` for F2 only until the reply below is recorded, and the validator run after this commit reports `failures: 0`.
- Resolution reference: docs(pr-reviews): [#384] record the reply of round 1 in the PR 3038 audit
- Follow-up PR URL: N/A
- Reply URL: pending
