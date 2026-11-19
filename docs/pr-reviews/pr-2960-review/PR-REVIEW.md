---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2960 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2960.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2960-f1` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2960-f2` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2960-f3` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The PR title carries an issue reference that the PR neither implements nor closes

- PR number: 2960
- Source review ID: 5972146805
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2960#discussion_r4626523341
- Concern: The PR title carried `[#1488]`. The `open-pull-request` skill keeps `[#<issue>]` in the title for the issue the PR implements or closes; a related or already-closed issue belongs in the PR body. Issue 1488 was closed by PR #2951.
- Solution: Fixed outside the tree: the title is now `docs(pr-reviews): cite the conversation reply as the resolution of F8 in the review audit of PR #2951`. The description keeps `Related to #1488`, and the commit subjects keep the reference.
- Current-tree verification: `gh pr view 2960 --repo torrust/torrust-tracker --json title --jq .title` prints the title without `[#1488]`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2960#discussion_r4626548301
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2960#discussion_r4626548301

### F2 - The merge time of PR 2951 names no source

- PR number: 2960
- Source review ID: 5972146805
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2960#discussion_r4626523354
- Concern: The audit of PR #2951 says in two places that it merged at 08:07:03 UTC without naming the source, while the merge commit carries 07:57:06 UTC as its author and committer time.
- Solution: Both places now say that 08:07:03 UTC is `merged_at` of the PR on GitHub, and give the time of the merge commit with its source.
- Current-tree verification: `grep -c "merged_at. of the PR on GitHub" docs/pr-reviews/pr-2951-review/PR-REVIEW.md` prints 2. `gh api repos/torrust/torrust-tracker/pulls/2951 --jq .merged_at` prints `2026-11-19T08:07:03Z`, and `git log -1 --format=%cI` of the merge commit of PR #2951 prints `2026-11-19T07:57:06Z`.
- Resolution reference: `docs(pr-reviews): [#1488] name the source of the merge time in the review audit of PR #2951`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2960#discussion_r4626548736

### F3 - The description says one Processing Log entry is added; the diff adds three

- PR number: 2960
- Source review ID: 5972146805
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2960#discussion_r4626523364
- Concern: The PR description said that F9 comes with "a Processing Log entry". The diff adds three entries, and the first one, a correction to the round 2 entries, is not about F9 and was not mentioned.
- Solution: Fixed outside the tree: the description now names the three entries, and also the two changes of this round.
- Current-tree verification: `gh pr view 2960 --repo torrust/torrust-tracker --json body --jq .body` shows the line that starts with "Three Processing Log entries are added".
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2960#discussion_r4626549053
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2960#discussion_r4626549053

## Processing Log

- 2026-11-19 09:11 UTC - Started audit. Review 5972146805 (da2ce7, `APPROVED`, submitted 2026-11-19 09:08:36 UTC on the commit `docs(pr-reviews): [#1488] record round 3 and the post-merge follow-up in the review audit of PR #2951`) has three inline threads, F1 to F3. The reviewer says that none of them blocks. Its body has no other actionable assertion.
- 2026-11-19 09:11 UTC - F2 fixed in `docs(pr-reviews): [#1488] name the source of the merge time in the review audit of PR #2951`, committed at 09:10:16Z (`git log --format=%cI`). F1 and F3 fixed outside the tree: the title and the description of the PR were edited after that commit and before the replies.
- 2026-11-19 09:11 UTC - Replied on the three threads at 09:11:01Z, 09:11:04Z and 09:11:06Z (`created_at` of the replies). The replies were posted before the push, so that the fix commit and this record go out in one push; the threads are resolved after it.

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
