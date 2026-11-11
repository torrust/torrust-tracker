---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2816 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2816.

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

Review 5856200684 of da2ce7 (F1 to F6, changes requested).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2816-f1 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2816-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2816-f3 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2816-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2816-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2816-f6 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The close-out entries point to an audit that has no F21 to F28 and records no decision

- PR number: 2816
- Source review ID: 5856200684
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535868286
- Concern: The spec's close-out entry and the EPIC's entry sent the reader to the #2812 audit for findings F21 to F28 and attributed a decision to the maintainer, but that audit stops at F20 and records no approval URL.
- Solution: Reworded both entries to name review 5850042659 of PR #2812 as the source and to say only that the findings were left unaddressed at the merge; dropped the pointer and the decision attribution. The decision was given in chat and has no durable GitHub URL, so it is not written into the audit.
- Current-tree verification: Searched the two entries and the PR 2812 audit for `F21` and for the audit pointer.
- Resolution reference: chore(issues): [#2576] fix the close-out findings F1 to F6 of the archive PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535909830

### F2 - The close-out entry is filed under References, not the Progress Log

- PR number: 2816
- Source review ID: 5856200684
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535868300
- Concern: The EPIC entry was appended after the References section, so the Progress Log still ended at the 2026-11-10 12:33 entry.
- Solution: Moved the entry to the end of the Progress Log.
- Current-tree verification: Read the EPIC around the end of the Progress Log and the end of the file.
- Resolution reference: chore(issues): [#2576] fix the close-out findings F1 to F6 of the archive PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535910158

### F3 - The move breaks the evidence file's link to readme-audit.md

- PR number: 2816
- Source review ID: 5856200684
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535868308
- Concern: The relative link reached the EPIC folder from `open/` but not from `closed/`, and the link checker skips `docs/issues/closed/`.
- Solution: Changed the link to `../../open/1669-overhaul-packages/readme-audit.md`.
- Current-tree verification: Resolved the link target path from the new location.
- Resolution reference: chore(issues): [#2576] fix the close-out findings F1 to F6 of the archive PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535910428

### F4 - AC6 is ticked while the verification table and evidence still say it is pending

- PR number: 2816
- Source review ID: 5856200684
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535868317
- Concern: The Acceptance Verification row and the evidence line still said AC6 was pending the merge.
- Solution: Set the row to DONE with the merge evidence and updated the evidence line and its `last-updated-utc`.
- Current-tree verification: Read the AC6 row, the evidence line and the checkbox.
- Resolution reference: chore(issues): [#2576] fix the close-out findings F1 to F6 of the archive PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535910645

### F5 - The completion-review checkpoint is ticked without a reason

- PR number: 2816
- Source review ID: 5856200684
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535868323
- Concern: No progress-log entry explained why no implementation retrospective was needed.
- Solution: Added the reason to the 08:24 entry.
- Current-tree verification: Read the 08:24 entry.
- Resolution reference: chore(issues): [#2576] fix the close-out findings F1 to F6 of the archive PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535910890

### F6 - Three affected documents keep their old last-updated-utc

- PR number: 2816
- Source review ID: 5856200684
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535868329
- Concern: The #2583 spec, the #2594 spec and the moved evidence file kept old stamps although they were edited.
- Solution: Updated all three stamps to the time of the edit.
- Current-tree verification: Compared the three stamps with the commit time from `git log --format=%cI`.
- Resolution reference: chore(issues): [#2576] fix the close-out findings F1 to F6 of the archive PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2816#discussion_r4535911249

## Processing Log

- 2026-11-11 08:57 UTC - Review 5856200684 of da2ce7 received (six findings).
- 2026-11-11 09:02 UTC - F1 to F6 fixed in commit `chore(issues): [#2576] fix the close-out findings F1 to F6 of the archive PR` (created 08:59:51 UTC) and pushed; the six replies were posted between 09:02:00 and 09:02:11 UTC, after the push.
- 2026-11-11 09:05 UTC - Correction: the entry above was first stamped 09:03 UTC, a minute after the commit that carried it (created 09:02:39 UTC). The stamp was edited in place to 09:02 UTC.

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
