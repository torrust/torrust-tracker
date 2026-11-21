---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2989 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2989.

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

Round 1 is Copilot review `6006218967` (`COMMENTED`, on head `docs(issues): [#2949] record the implementation progress and the manual verification evidence`), with one inline comment, F1 (an audit-local ID). The comment carries no severity label, so `Nit` is inferred from its prose: a formatting defect in one list item. The review body says the full agentic review did not run because it did not start before the timeout, and raises nothing else.

Round 2 is da2ce7's two reviews. Review `6006229600` (`APPROVED` when submitted at 2026-11-21 15:14:41 UTC, on the same head as round 1; GitHub shows it as `DISMISSED` since the round-1 push) has three inline comments that carry the reviewer's IDs F1 to F3 and explicit severity labels. This record's first version left that review out: the round-1 paragraph above and the first Processing Log entry describe round 1 as the Copilot review alone, and stay as written. Review `6006354345` (`COMMENTED`, on head `docs(pr-reviews): [#2949] add the PR #2989 review audit record with round 1`) has two inline comments, the reviewer's F4 and F5. The reviewer's F1 collides with this record's F1, so the five are recorded under the next audit-local IDs, F2 to F6, with the reviewer's IDs as their reviewer finding IDs. The row for the reviewer's F4 (F5 here) is added after this record's round-2 commit is pushed, because its reply cites that commit.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F1 | `review-finding:pr-2989-f1` | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2989-f2` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2989-f3` | Human | Nit | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F4 | `review-finding:pr-2989-f4` | Human | Suggestion | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2989-f5` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2989-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A continuation line of the evidence list is not indented

- PR number: 2989
- Source review ID: 6006218967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653326417
- Concern: In `manual-verification-evidence.md`, the "Artifact under test" item wrapped a commit subject across lines, and the last line was not indented as part of the list item.
- Solution: The commit subject is on one line of its own, indented under the item, so the whole entry is one list item.
- Current-tree verification: `sed -n 17,19p docs/issues/open/2949-2003-keep-reviewer-ack-after-audit-only-commits/manual-verification-evidence.md` prints the item with both continuation lines indented by two spaces.
- Resolution reference: `docs(issues): [#2949] keep the artifact line of the evidence inside its list item`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653358233

### F2 - The #2363-line sentence gives the ACK rule as the reason for the dismissal

- PR number: 2989
- Source review ID: 6006229600
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653335092
- Concern: The sentence added to the EPIC's #2363 line let a reader take the ACK rule as the cause of the stale-approval dismissal, which comes from a repository setting the specification puts out of scope.
- Solution: The sentence uses the reviewer's wording: issue #2949 decided that the audit record stays on the branch, so the dismissal is expected, and the ACKs on the earlier head go stale the same way.
- Current-tree verification: `grep -n 'stale-approval dismissal' docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` prints the line with the new sentence.
- Resolution reference: `docs(issues): [#2949] keep the approval dismissal apart from the ACK rule on the EPIC's #2363 line`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653467176

### F3 - The row 24 status change rides in a commit whose subject names only the #2363 line

- PR number: 2989
- Source review ID: 6006229600
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653335095
- Concern: Commit Points assigns the EPIC row to T6, but the status change to `IN_PROGRESS` is in the T5 commit, whose subject names only the #2363 line.
- Solution: No change. The branch history is kept, and the EPIC progress-log line of that commit names both changes. The close-out commit that sets row 24 to `DONE` will name the row change in its subject.
- Current-tree verification: `git log --format=%s -- docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md | grep 'name the strict-ACK decision'` prints the subject of the T5 commit, and the EPIC progress-log entry of 2026-11-21 13:22 UTC names the row change and the #2363 line.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653467291
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653467291

### F4 - Step 4 links to the top of the skill, not to the subsection

- PR number: 2989
- Source review ID: 6006229600
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653335102
- Concern: The new sentence in `wait-for-pr-review` step 4 said "That section" and linked only to the top of `merge-pull-request`, while `process-pr-review` step 9 links the subsection.
- Solution: Step 4 links `../merge-pull-request/SKILL.md#an-ack-names-one-commit`.
- Current-tree verification: `grep -n 'an-ack-names-one-commit' .claude/skills/wait-for-pr-review/SKILL.md` prints the link in step 4.
- Resolution reference: `docs(wait-for-pr-review): [#2949] link the exit condition straight to the ACK subsection`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653467423

### F5 - The record leaves out review 6006229600 and its three threads

- PR number: 2989
- Source review ID: 6006354345
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653440624
- Concern: Review 6006229600 was submitted on the first head before this record was created, with three inline comments that had no reply, row or resolution, so the record's account of round 1 was incomplete.
- Solution: The round-2 paragraph names the review, rows F2 to F4 record its comments with the reviewer's IDs, and a Processing Log entry says how round 1 missed it. The round-1 paragraph and entry stay as written.
- Current-tree verification: `grep -c 6006229600 docs/pr-reviews/pr-2989-review/PR-REVIEW.md` prints a count above zero, and the six review threads the pull request had at that point, listed with the GraphQL `reviewThreads` query, map one to one to the Source URLs of rows F1 to F6.
- Resolution reference: `docs(pr-reviews): [#2949] record PR #2989 review round 2 with the review that round 1 missed`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653484912

### F6 - The description's file list does not name the audit record

- PR number: 2989
- Source review ID: 6006354345
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653440630
- Concern: "What changes" in the description listed six files while the branch changes seven, and did not mention that `ISSUE.md` gains `related-pr: 2989`.
- Solution: The description lists the audit record as a seventh item and names `related-pr: 2989`. The description is outside the diff, so no commit carries this fix.
- Current-tree verification: `gh pr view 2989 --json body` shows the item for `docs/pr-reviews/pr-2989-review/PR-REVIEW.md`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653467548
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653467548

## Processing Log

- 2026-11-21 15:20 UTC - AI assistant (Claude Code, Claude Fable 5.2, issue #2949 session) - Started audit. Round 1: the poller reported Copilot review 6006218967 (submitted 2026-11-21 15:12:33 UTC). Normalized its inline comment as F1, fixed it in its own commit, pushed it with the commit that records this pull request in the specification, replied on the thread (15:20:30 UTC), then resolved the thread.
- 2026-11-21 15:50 UTC - AI assistant (Claude Code, Claude Fable 5.2, issue #2949 session) - Correction to the entry above, and round 2. da2ce7's review 6006229600 (submitted 15:14:41 UTC, with an approval and an `ACK` on the first head at 15:14:45 UTC) arrived before the round-1 push, but the poller had already returned the Copilot review and was restarted from a later cursor, so the review was not seen: round 1 was processed and this record was created without it, and the round-1 push made that `ACK` stale. da2ce7's review 6006354345 (submitted 15:42:44 UTC) pointed this out. Normalized the five inline comments of both reviews as F2 to F6. Fixed F2 and F4 in their own commits and pushed them; fixed F6 in the description; answered F3 with no change. Replied on those four threads (15:50:02 to 15:50:09 UTC) and resolved them. F5, about this record, is answered and recorded after this commit is pushed.
- 2026-11-21 15:55 UTC - AI assistant (Claude Code, Claude Fable 5.2, issue #2949 session) - Round 2, continued: after the round-2 record was pushed, replied on F5's thread (15:55:29 UTC), resolved it and added its row. All six threads of this pull request now have a row, a reply and a resolution.

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
