---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2880 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2880.

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

The da2ce7 review numbers its findings `F1` to `F5`; Copilot's review uses `R1` and `R2`, and uses `R2`
twice. They get the audit-local IDs `C1` to `C8`, the second da2ce7 review's `F6` gets `C9`, and
all keep their own IDs as reviewer finding IDs.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| C1 | review-finding:pr-2880-c1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| C2 | review-finding:pr-2880-c2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| C3 | review-finding:pr-2880-c3 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| C4 | review-finding:pr-2880-c4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| C5 | review-finding:pr-2880-c5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| C6 | review-finding:pr-2880-c6 | Copilot | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| C7 | review-finding:pr-2880-c7 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| C8 | review-finding:pr-2880-c8 | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| C9 | review-finding:pr-2880-c9 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### C1 - Name the reset log line the M4 filter drops

- PR number: 2880
- Source review ID: 5918391180
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584502187
- Concern: The M4 evidence said the ban reset writes no log line; it writes one at INFO that the script's filter drops.
- Solution: Reworded the M4 Result to name the dropped line (`connection id errors filter cleared`).
- Current-tree verification: `manual-verification-evidence.md` M4 Result names the line and keeps the metric and the answered connect as the evidence.
- Resolution reference: docs(issues): address the review of the #2845 close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584531483

### C2 - Clear the unrecorded design-review checkpoint

- PR number: 2880
- Source review ID: 5918391180
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584502203
- Concern: The design-review checkpoint was ticked, but no prose-first Arrange-Act-Assert review was recorded after T2, T3 or T4.
- Solution: Unticked it with the reason, and named the missing review in the retrospective next to the weak first regression test.
- Current-tree verification: `ISSUE.md` has the checkpoint unticked with its reason; `implementation-retrospective.md` names the missing review.
- Resolution reference: docs(issues): address the review of the #2845 close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584531761

### C3 - Repair the archived spec's EPIC link

- PR number: 2880
- Source review ID: 5918391180
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584502210
- Concern: After the move, `../1488-overhaul-tracker-shutdown/ISSUE.md` resolves under `docs/issues/closed/`, where the EPIC does not exist.
- Solution: Changed the link to `../../open/1488-overhaul-tracker-shutdown/ISSUE.md`.
- Current-tree verification: The link target `docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md` exists.
- Resolution reference: docs(issues): address the review of the #2845 close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584532060

### C4 - Do not call M1 a binary run

- PR number: 2880
- Source review ID: 5918391180
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584502220
- Concern: The EPIC row and the retrospective said M1 to M4 ran on the binary; M1 is a tree search.
- Solution: Reworded both: M1 to M4 are recorded, M2 to M4 ran on the binary.
- Current-tree verification: EPIC row 23 and the retrospective's Outcome say M1 is a search of the tree.
- Resolution reference: docs(issues): address the review of the #2845 close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584532419

### C5 - State the HTTP wording exception in the criterion

- PR number: 2880
- Source review ID: 5918391180
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584502225
- Concern: The unchanged-behaviour criterion was ticked without T1 decision 1's change to the two HTTP tracker error messages.
- Solution: Added the exception to the criterion.
- Current-tree verification: `ISSUE.md` criterion ends with the HTTP exception from T1 decision 1.
- Resolution reference: docs(issues): address the review of the #2845 close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584532768

### C6 - Repair the archived spec's EPIC link (Copilot)

- PR number: 2880
- Source review ID: 5918391597
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584502538
- Concern: Same concern as C3.
- Solution: Same fix as C3.
- Current-tree verification: As C3.
- Resolution reference: docs(issues): address the review of the #2845 close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584533128

### C7 - State the HTTP wording exception (Copilot)

- PR number: 2880
- Source review ID: 5918391597
- Reviewer finding ID: R2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584502595
- Concern: Same concern as C5.
- Solution: Same fix as C5.
- Current-tree verification: As C5.
- Resolution reference: docs(issues): address the review of the #2845 close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584533507

### C8 - Keep C13 of the PR 2870 audit open until this PR merges

- PR number: 2880
- Source review ID: 5918391597
- Reviewer finding ID: R2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584502637
- Concern: C13 was recorded `FIXED` / `RESOLVED` while its follow-up (this PR) is still open, against `process-pr-review` late-finding step 5.
- Solution: Set C13 to `FOLLOW_UP` / `OPEN` with the durable reply URL as its resolution reference, and reopened the F13 thread on PR 2870.
- Current-tree verification: `docs/pr-reviews/pr-2870-review/PR-REVIEW.md` C13 row is `FOLLOW_UP` / `OPEN`; the F13 thread on PR 2870 is unresolved.
- Resolution reference: docs(issues): address the review of the #2845 close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584533904

### C9 - Record the post-merge approval for C13

- PR number: 2880
- Source review ID: 5918589470
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584652000
- Concern: C13 follows the late-finding workflow, but the PR 2870 audit still said `Post-merge workflow approval: N/A`, and its Processing Log missed the reply cited as C13's resolution reference and the reopening of the thread.
- Solution: Posted a durable approval comment on PR 2870 on the maintainer's behalf and cited it in the Ownership section with the approval, review and merge times; added the three missing log entries.
- Current-tree verification: `docs/pr-reviews/pr-2870-review/PR-REVIEW.md` Ownership cites `https://github.com/torrust/torrust-tracker/pull/2870#issuecomment-6914058589`, and its log has the 09:51, 10:24 and 10:37 UTC entries.
- Resolution reference: docs(pr-reviews): record the post-merge approval for C13 in the PR 2870 audit (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2880#discussion_r4584666009

## Processing Log

- 2026-11-16 10:21 UTC - da2ce7 review 5918391180 (approved) submitted with five inline findings, none blocking.
- 2026-11-16 10:21 UTC - Copilot review 5918391597 submitted with three inline findings.
- 2026-11-16 10:23 UTC - Committed the fix for all eight findings (`docs(issues): address the review of the #2845 close-out`).
- 2026-11-16 10:24 UTC - Replied on the eight threads and reopened the F13 thread on PR 2870 (C8).
- 2026-11-16 10:36 UTC - da2ce7 review 5918589470 (approved) submitted with one inline finding (F6, non-blocking).
- 2026-11-16 10:37 UTC - Posted the durable approval comment on PR 2870 and committed the C9 fix.
- 2026-11-16 10:38 UTC - Replied on the F6 thread.

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
