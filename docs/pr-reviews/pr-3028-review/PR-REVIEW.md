---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
    - "issue #384"
---

<!-- skill-link: process-pr-review -->

# PR #3028 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/3028>.

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

Review 6040228663 (CodeRabbit, 2026-11-24 13:41 UTC) posted two inline comments, F1 and F2. Its body only repeats them, so it adds no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3028-f1 | Unknown | Minor | correctness | ORIGINAL | FIXED | OPEN |
| F2 | review-finding:pr-3028-f2 | Unknown | Minor | security | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - Reconcile AC1 with the recorded baseline

- PR number: 3028
- Source review ID: 6040228663
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3028#discussion_r4680816244>
- Concern: AC1 said each reply matches a recorded baseline reply that held a path, but the baseline replies R1 to R5 held none, so the checked criterion overstated what the evidence shows.
- Solution: AC1 now says every baseline reply that held a path (13 of the recorded replies) holds none now, and that replies that never held a path (R1 to R5) are no-leak notes, not baseline matches. The progress log records the change.
- Current-tree verification: `grep -n "AC1:" docs/issues/open/384-strip-location-from-errors-sent-to-clients/ISSUE.md` prints the narrowed text.
- Resolution reference: docs(issues): [#384] narrow AC1 to the recorded baseline and check the Windows path forms
- Follow-up PR URL: N/A
- Reply URL: pending

### F2 - Check the Windows path substrings too

- PR number: 3028
- Source review ID: 6040228663
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3028#discussion_r4680816254>
- Concern: the path check in `manual-verification-evidence.md` omitted `packages\` and `src\`, two of the seven forbidden substrings of the spec's Regression Test Strategy.
- Solution: the check was re-run with all seven substrings (`.rs:`, `packages/`, `packages\`, `src/`, `src\`, `.cargo/registry`, `rustlib/src`) over `responses-after.txt` and `responses-extra-after.txt`; it finds nothing (exit status 1), and the document records the command and the result.
- Current-tree verification: the same `grep -nF` command in the "Summary of the path check" section exits with status 1.
- Resolution reference: docs(issues): [#384] narrow AC1 to the recorded baseline and check the Windows path forms
- Follow-up PR URL: N/A
- Reply URL: pending
