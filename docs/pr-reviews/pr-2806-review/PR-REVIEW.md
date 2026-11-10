---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2806 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2806.

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

Three review rounds so far: Copilot (review 5845033563, F1), da2ce7's first round (review 5845036517, F2-F6, dismissed
when the head moved, its findings still processed) and da2ce7's second round (review 5845491910, F7-F9). da2ce7's IDs
F1-F5 and F6-F8 collide with earlier rows, so they are reassigned here and the original ID is kept in each detail entry;
F4 re-raises F1.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2806-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F2 | review-finding:pr-2806-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2806-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2806-f4 | Human | Nit | documentation | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F5 | review-finding:pr-2806-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2806-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2806-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2806-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2806-f9 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The progress-log entries name the agent but not the model

- PR number: 2806
- Source review ID: 5845033563
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2806#pullrequestreview-5845033563
- Concern: The log entries named the tool and not the model, against the issue template and the create-issue skill.
- Solution: Named the model in every agent entry of 2026-11-10; answered in a PR comment, since the finding is in the review body.
- Current-tree verification: `grep -n 'Claude Sonnet' ISSUE.md` lists the entries.
- Resolution reference: docs(issues): name the model in the #2713 progress-log entries
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2806#issuecomment-6772420177

### F2 - The PR title is not a Conventional Commits line and the description omits its validation

- PR number: 2806
- Source review ID: 5845036517
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527246569
- Concern: The title started with `[#2713]`, and the description lacked the validation run and the `Related to` form.
- Solution: Retitled to the commit subject and rewrote the description; both are outside the tree, so the reply is the durable reference.
- Current-tree verification: `gh pr view 2806 --json title,body` shows the new title and the validation line.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527353177
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527353177

### F3 - The checkpoint says the spec was drafted in `docs/issues/closed/`

- PR number: 2806
- Source review ID: 5845036517
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527246592
- Concern: The path replacement made the drafting checkpoint false.
- Solution: The checkpoint names `docs/issues/drafts/` and the move to `open/`.
- Current-tree verification: Read line 157 of `ISSUE.md`.
- Resolution reference: docs(issues): keep the drafting checkpoint of #2713 accurate
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527353704

### F4 - The new log entry names the agent but not its model

- PR number: 2806
- Source review ID: 5845036517
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527246611
- Concern: Same concern as F1; the earlier entries of the same session also omit the model.
- Solution: The entries of 2026-11-10 name the model; it was already pushed when this review was read.
- Current-tree verification: `grep -n 'Claude Sonnet' ISSUE.md`.
- Resolution reference: docs(issues): name the model in the #2713 progress-log entries
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527354179

### F5 - The reason no retrospective is needed is not in the progress log

- PR number: 2806
- Source review ID: 5845036517
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527246625
- Concern: The archive entry did not give the reason the retrospective line gives.
- Solution: Added the reason to the archive entry.
- Current-tree verification: Read the 15:44 entry of `ISSUE.md`.
- Resolution reference: docs(issues): give the reason no retrospective is needed in the #2713 log
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527354599

### F6 - `last-updated-utc` of the EPIC moves with no EPIC log entry

- PR number: 2806
- Source review ID: 5845036517
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527246644
- Concern: Row 18 turning DONE had no progress-log entry in the EPIC.
- Solution: Added the entry, stamped with its commit time.
- Current-tree verification: Read the last entries of the EPIC progress log.
- Resolution reference: docs(issues): add the #2713 DONE entry to the EPIC #2003 progress log
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527355028

### F7 - The PR processes review findings but carries no review audit record

- PR number: 2806
- Source review ID: 5845491910
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527547269
- Concern: `process-pr-review` makes the author own `docs/pr-reviews/pr-2806-review/PR-REVIEW.md`; none existed.
- Solution: Added this record and ran `validate-audit-record.py`.
- Current-tree verification: `validate-audit-record.py --pr-number 2806` exits 0.
- Resolution reference: docs(pr-reviews): add the review audit record for #2806
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527721928

### F8 - The 16:05 entry says every entry above names the model; three do not

- PR number: 2806
- Source review ID: 5845491910
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527547280
- Concern: The 2026-11-04 entries name no model, so "every entry above" overstated.
- Solution: Narrowed the sentence to the entries of 2026-11-10.
- Current-tree verification: Read the 16:05 entry of `ISSUE.md`.
- Resolution reference: docs(issues): narrow the model claim in the #2713 log to the 2026-11-10 entries
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527722428

### F9 - The spec's stamp moves with no log entry for the two corrections

- PR number: 2806
- Source review ID: 5845491910
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527547283
- Concern: The 16:07 and 16:08 edits had no log line, and the stamp moved to 16:09 in the EPIC commit.
- Solution: One log entry, stamped with its commit time, records the corrections, and `last-updated-utc` moves with it.
- Current-tree verification: Read the 16:40 entry and the frontmatter of `ISSUE.md`.
- Resolution reference: docs(issues): log the #2713 checkpoint and retrospective corrections
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2806#discussion_r4527722967

## Processing Log

- 2026-11-10 16:07 UTC - Copilot's finding F1 answered in a PR comment (comment creation time) and fixed in the commit with subject "docs(issues): name the model in the #2713 progress-log entries".
- 2026-11-10 16:11 UTC - Replies to da2ce7's first-round threads F2-F6 posted (reply creation time); the threads were resolved after them.
- 2026-11-10 16:40 UTC - Second-round fixes for F8 and F9 committed (commit time of "docs(issues): log the #2713 checkpoint and retrospective corrections").

Append entries only. If an entry was rewritten in place, restore the prior entries, then append a
correction that names the rewritten content; do not rewrite the record again.

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
