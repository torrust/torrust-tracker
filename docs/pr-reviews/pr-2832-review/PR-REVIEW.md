---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2832 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2832.

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

Round 1: da2ce7 review 5860110297 (changes requested, submitted 14:14:48Z), findings F1 to F5. Round 2: da2ce7 review 5860471258 (approved, submitted 14:40:32Z), findings F6, F8 and F9 (his `F7`, the missing audit record, was fixed in round 1 and is not a separate row). The automated review 5860133630 (14:16:32Z, on the first head) is a notice that its full review did not start before a timeout; it makes no request and has no row. F5 is a review-body finding without an inline thread, so it is `NON_RESOLVABLE`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2832-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2832-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2832-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2832-f4 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2832-f5 | Human | Nit | documentation | ORIGINAL | NO_ACTION | NON_RESOLVABLE |
| F6 | review-finding:pr-2832-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2832-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2832-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The archive rewrote the 16:17 progress-log entry

- PR number: 2832
- Source review ID: 5860110297
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4538952363
- Concern: The path replacement also changed the historical 16:17 entry, which now said the spec moved to `closed/` when the issue was created.
- Solution: Restored `docs/issues/open/` in that entry and appended a correction entry.
- Current-tree verification: `grep -n "16:17 UTC" docs/issues/closed/2808-workspace-package-release-process/ISSUE.md` shows the `open/` path.
- Resolution reference: docs(issues): [#2808] restore the open/ path in the 16:17 log entry of the archived spec; docs(issues): [#2808] log the corrections to the archive entry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4539045118

### F2 - Reviewer-validation checkpoint ticked without a reviewer edit

- PR number: 2832
- Source review ID: 5860110297
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4538952371
- Concern: No reviewer edited the spec, so the tick overstated the record.
- Solution: Unticked the checkpoint and recorded the reason in the appended log entry.
- Current-tree verification: `grep -n "Reviewer validated" docs/issues/closed/2808-*/ISSUE.md` shows `- [ ]`.
- Resolution reference: docs(issues): [#2808] leave the reviewer-validation checkpoint open in the archived spec; docs(issues): [#2808] log the corrections to the archive entry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4539045622

### F3 - Independent-reports checkpoint left open without a reason

- PR number: 2832
- Source review ID: 5860110297
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4538952380
- Concern: The checkpoint was open while the description said the remaining ones were ticked.
- Solution: Ticked it and named the audit records of PR 2809 and PR 2825 in the appended log entry.
- Current-tree verification: `grep -n "Independent reviewer reports" docs/issues/closed/2808-*/ISSUE.md` shows `- [x]`.
- Resolution reference: docs(issues): [#2808] tick the independent-reports checkpoint of the archived spec; docs(issues): [#2808] log the corrections to the archive entry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4539046264

### F4 - The follow-up issue text has no frontmatter

- PR number: 2832
- Source review ID: 5860110297
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4538952387
- Concern: Step 3 of `cleanup-completed-issues` asks for a minimal block on supplementary docs.
- Solution: Added `spec-path`, `last-updated-utc` and `semantic-links` above the first heading.
- Current-tree verification: `head -8 docs/issues/closed/2808-*/torrust-server-lib-followup-issue.md` shows the block.
- Resolution reference: docs(issues): [#2808] add frontmatter to the follow-up issue text of the archived spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4539046850

### F5 - The title carries [#2808], reserved for the issue a PR closes

- PR number: 2832
- Source review ID: 5860110297
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2832#pullrequestreview-5860110297
- Concern: `open-pull-request` reserves the number for the issue a PR implements or closes; #2808 is already closed.
- Solution: No change: the archive skill's title template names the issue number, so the two skills disagree and the archive skill was followed. Explained in the PR conversation.
- Current-tree verification: `gh-bot pr view 2832 --json title` shows the unchanged title.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2832#issuecomment-6798383022
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2832#issuecomment-6798383022

### F6 - The 14:19 correction sits under References, not in the progress log

- PR number: 2832
- Source review ID: 5860471258
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4539237812
- Concern: The entry was appended to `## References` after the `Process docs` bullet, so a reader of the log sees no correction.
- Solution: Moved the entry to the progress log after the 13:51 entry, reworded it to say it corrects the archive commit's checkpoint ticks, and bumped `last-updated-utc`.
- Current-tree verification: `sed -n 216,222p docs/issues/closed/2808-*/ISSUE.md` shows the entry after the 13:51 entry; `tail -3` of the file ends with References bullets only.
- Resolution reference: docs(issues): [#2808] move the 14:19 correction into the progress log of the archived spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4539289982

### F8 - The description says the remaining checkpoints are ticked while one stays open

- PR number: 2832
- Source review ID: 5860471258
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4539237824
- Concern: With F2 fixed, "Reviewer validated acceptance criteria" stays open, but the PR description said the remaining checkpoints are ticked.
- Solution: Edited the PR description to say one checkpoint stays open and why.
- Current-tree verification: `gh-bot pr view 2832 --json body` contains "which stays open because no reviewer edited the spec".
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4539290471
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4539290471

### F9 - The audit does not mention the automated review 5860133630

- PR number: 2832
- Source review ID: 5860471258
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4539237834
- Concern: Review 5860133630 (14:16:32Z, on the first head) is a timeout notice with no request; no row is right, but nothing showed it was read.
- Solution: Named it in the intro and appended a log entry.
- Current-tree verification: The `## Findings` intro and the Processing Log of this record name review 5860133630.
- Resolution reference: docs(pr-reviews): [#2808] record round 2 of the PR 2832 review audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2832#discussion_r4539290926

## Processing Log

- 2026-11-11 14:14 UTC - Started audit: da2ce7 review 5860110297 (submitted 14:14:48Z) with four inline threads and one review-body finding.
- 2026-11-11 14:19 UTC - Fix commits at 14:17:12Z, 14:17:43Z, 14:18:12Z and 14:19:50Z, and the log commit at 14:19:51Z (`git log --format=%cI`); pushed after the last.
- 2026-11-11 14:23 UTC - Replied on the four threads (reply `created_at` 14:23:12Z to 14:23:20Z) and posted the F5 explanation on the PR conversation (14:23:22Z), then resolved the four threads through GraphQL (four results, each `isResolved: true`).
- 2026-11-11 14:45 UTC - Round 2: da2ce7 review 5860471258 (approved, submitted 14:40:32Z) with three comments. Fix commit `docs(issues): [#2808] move the 14:19 correction into the progress log of the archived spec` at 14:42:43Z (`git log --format=%cI`), pushed after it; the PR description was edited for F8 after that. Replied on the three threads (reply `created_at` 14:45:02Z to 14:45:07Z) and resolved them through GraphQL (three results, each `isResolved: true`). The automated review 5860133630 (14:16:32Z) was read: a timeout notice with no request, so no row (finding F9).

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
