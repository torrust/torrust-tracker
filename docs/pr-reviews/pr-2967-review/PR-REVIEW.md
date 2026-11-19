---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2967 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2967.

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
| F1 | `review-finding:pr-2967-f1` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2967-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2967-f3` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2967-f4` | Copilot | Nit (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The new log entry sits under Completion Rules, not in the Processing Log

- PR number: 2967
- Source review ID: 5974196064
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2967#discussion_r4628089546
- Concern: The log entry about the merge of PR #2960 was appended at the end of `docs/pr-reviews/pr-2951-review/PR-REVIEW.md`, so it became the seventh bullet of `## Completion Rules`. The Processing Log ended one entry earlier, with F9 still `FOLLOW_UP` / `OPEN`, and the Completion Rules section no longer equalled the template. The validator did not notice, because it reads from `## Processing Log` to the end of the file.
- Solution: The entry is moved to the end of the Processing Log, after the 08:23 entry about the merge of PR #2951. The cause was an append at the end of the file instead of at the end of the section. The PR description is updated.
- Current-tree verification: `sed -n '/^## Processing Log/,/^## Completion Rules/p' docs/pr-reviews/pr-2951-review/PR-REVIEW.md | grep -c "^- 2026-11-19 11:03 UTC"` prints 1, and `sed -n '/^## Completion Rules/,$p'` of that file equals the same section at the merge base (`diff` prints nothing).
- Resolution reference: `docs(pr-reviews): [#1488] move the close-out log entry into the Processing Log of the review audit of PR #2951`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2967#discussion_r4628444427

### F2 - The verification command of F4 now prints two lines

- PR number: 2967
- Source review ID: 5974196064
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2967#discussion_r4628089562
- Concern: The Solution of F9 and the new log entry of `docs/pr-reviews/pr-2951-review/PR-REVIEW.md` named the merge commit of PR #2960 by its abbreviated id. Row F4 of that record says that a grep for such ids prints nothing, so the record contradicted itself.
- Solution: Both places name the merge commit by its subject, `Merge torrust/torrust-tracker#2960: ...`, and no abbreviated id.
- Current-tree verification: `grep -c -E "commit .[0-9a-f]{9}" docs/pr-reviews/pr-2951-review/PR-REVIEW.md` prints 0.
- Resolution reference: `docs(pr-reviews): [#1488] name the merge of PR #2960 by its subject in the review audit of PR #2951`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2967#discussion_r4628444913

### F3 - The verification of F9 reads the working tree, while its text speaks of develop

- PR number: 2967
- Source review ID: 5974279754
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2967#discussion_r4628152599
- Concern: The Current-tree verification of F9 in `docs/pr-reviews/pr-2951-review/PR-REVIEW.md` gave a `grep` on the working tree, but said that the result is on `develop` at the merge commit of PR #2960.
- Solution: The command now reads the file from `torrust/develop` with `git show`, and the text says that it was run after `git fetch torrust develop`.
- Current-tree verification: `grep -c "git show torrust/develop:docs/pr-reviews/pr-2951-review/PR-REVIEW.md | grep" docs/pr-reviews/pr-2951-review/PR-REVIEW.md` prints 1.
- Resolution reference: `docs(pr-reviews): [#1488] read develop in the verification of F9 in the review audit of PR #2951`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2967#discussion_r4628445336

### F4 - The merge commit is cited by an abbreviated id

- PR number: 2967
- Source review ID: 5974279754
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2967#discussion_r4628152676
- Concern: The reviewer asks for the full 40-character id or a commit URL in place of the abbreviated id of the merge commit of PR #2960 in `docs/pr-reviews/pr-2951-review/PR-REVIEW.md`.
- Solution: No separate change. Superseded by F2: the record no longer cites the id; both places name the merge commit by its subject.
- Current-tree verification: Same as F2: `grep -c -E "commit .[0-9a-f]{9}" docs/pr-reviews/pr-2951-review/PR-REVIEW.md` prints 0.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2967#discussion_r4628445770
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2967#discussion_r4628445770

## Processing Log

- 2026-11-19 12:06 UTC - Started audit. Review 5974196064 (da2ce7, `APPROVED`, submitted 2026-11-19 11:32:19 UTC on the commit `docs(pr-reviews): [#1488] mark F9 fixed and resolved in the review audit of PR #2951`) has two inline threads, F1 and F2; the reviewer says that neither blocks. Review 5974279754 (Copilot, `COMMENTED`, 11:38 UTC, same commit) has two inline threads, F3 and F4; its severities are read from its "Medium" and "Low" labels. The maintainer and da2ce7 had posted their ACK comments for that commit (11:27:17Z and 11:32:23Z), so the agent asked the maintainer before pushing; the maintainer answered "Push now" in the agent chat.
- 2026-11-19 12:06 UTC - F1 fixed in `docs(pr-reviews): [#1488] move the close-out log entry into the Processing Log of the review audit of PR #2951`, F2 in `docs(pr-reviews): [#1488] name the merge of PR #2960 by its subject in the review audit of PR #2951`, F3 in `docs(pr-reviews): [#1488] read develop in the verification of F9 in the review audit of PR #2951`; committed at 12:02:52Z, 12:04:00Z and 12:05:08Z (`git log --format=%cI`). F4 is a duplicate of F2 and needs no change of its own.
- 2026-11-19 12:06 UTC - Replied on the four threads at 12:06:03Z, 12:06:06Z, 12:06:08Z and 12:06:11Z (`created_at` of the replies); the reply to F4 is the `Superseded by F2` form. The replies were posted before the push, so that the fix commits and this record go out in one push; the threads are resolved after it. The PR description was updated after the replies.

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
