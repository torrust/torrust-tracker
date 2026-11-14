---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2860 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2860.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: NA (not merged)

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2860-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2860-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2860-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2860-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2860-f5 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2860-f6 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2860-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| C1 | review-finding:pr-2860-c1 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| C2 | review-finding:pr-2860-c2 | Copilot | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| C3 | review-finding:pr-2860-c3 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - F13 follow-up PR URL missing

- PR number: 2860
- Source review ID: 5903621787
- Source URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4572936237
- Concern: Audit record would fail its validator after merge.
- Solution: Set F13's Follow-up PR URL to PR #2860.
- Current-tree verification: Read the changed file and ran `validate-audit-record.py` for PR 2858 and 2860.
- Resolution reference: docs(issues): [#2844] address the review of the close-out PR
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4573525182

### F2 - Moved spec's EPIC link broken

- PR number: 2860
- Source review ID: 5903621787
- Source URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4572936241
- Concern: Backlink resolved under closed/.
- Solution: Pointed it at the EPIC in open/.
- Current-tree verification: Read the changed file and ran `validate-audit-record.py` for PR 2858 and 2860.
- Resolution reference: docs(issues): [#2844] address the review of the close-out PR
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4573525087

### F3 - Log entry counts disagree with the audit

- PR number: 2860
- Source review ID: 5903621787
- Source URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4572936243
- Concern: Round and thread counts wrong.
- Solution: Reworded to four rounds, 14 threads, F13 fixed here.
- Current-tree verification: Read the changed file and ran `validate-audit-record.py` for PR 2858 and 2860.
- Resolution reference: docs(issues): [#2844] address the review of the close-out PR
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4573524992

### F4 - Ownership line says not merged

- PR number: 2860
- Source review ID: 5903621787
- Source URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4572936249
- Concern: Line stale after the merge.
- Solution: Updated the Ownership line.
- Current-tree verification: Read the changed file and ran `validate-audit-record.py` for PR 2858 and 2860.
- Resolution reference: docs(issues): [#2844] address the review of the close-out PR
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4573524928

### F5 - EPIC last-updated-utc not moved

- PR number: 2860
- Source review ID: 5903621787
- Source URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4572936251
- Concern: Stamp stayed at 2026-11-12.
- Solution: Set to the commit minute.
- Current-tree verification: Read the changed file and ran `validate-audit-record.py` for PR 2858 and 2860.
- Resolution reference: docs(issues): [#2844] address the review of the close-out PR
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4573525480

### F6 - Loose progress list

- PR number: 2860
- Source review ID: 5903621787
- Source URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4572936256
- Concern: Blank line before the new entry.
- Solution: Removed it.
- Current-tree verification: Read the changed file and ran `validate-audit-record.py` for PR 2858 and 2860.
- Resolution reference: docs(issues): [#2844] address the review of the close-out PR
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4573525571

### F7 - Description over-claims ticked rows

- PR number: 2860
- Source review ID: 5903621787
- Source URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4572936260
- Concern: Description said acceptance rows were ticked.
- Solution: Edited the description.
- Current-tree verification: Read the changed file and ran `validate-audit-record.py` for PR 2858 and 2860.
- Resolution reference: docs(issues): [#2844] address the review of the close-out PR
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4573525674

### C1 - Archived spec backlink (same as F2)

- PR number: 2860
- Source review ID: 5903663856
- Source URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4572965391
- Concern: Same as F2.
- Solution: Fixed with F2.
- Current-tree verification: Read the changed file and ran `validate-audit-record.py` for PR 2858 and 2860.
- Resolution reference: docs(issues): [#2844] address the review of the close-out PR
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4573525287

### C2 - EPIC timestamp (same as F5)

- PR number: 2860
- Source review ID: 5903663856
- Source URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4572965409
- Concern: Same as F5.
- Solution: Fixed with F5.
- Current-tree verification: Read the changed file and ran `validate-audit-record.py` for PR 2858 and 2860.
- Resolution reference: docs(issues): [#2844] address the review of the close-out PR
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4573525388

### C3 - F13 follow-up PR URL (same as F1)

- PR number: 2860
- Source review ID: 5903663856
- Source URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4572965419
- Concern: Same as F1.
- Solution: Fixed with F1.
- Current-tree verification: Read the changed file and ran `validate-audit-record.py` for PR 2858 and 2860.
- Resolution reference: docs(issues): [#2844] address the review of the close-out PR
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2860#discussion_r4573524837

## Processing Log

- 2026-11-14 12:38 UTC - Reviews by da2ce7 (7 findings) and Copilot (3 findings, all duplicates of da2ce7's) arrived at the first head; all ten threads answered, then fixed and recorded in one commit.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
