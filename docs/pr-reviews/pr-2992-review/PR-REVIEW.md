---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2992 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2992.

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

da2ce7's review `6011641355` (`APPROVED` when submitted at 2026-11-22 10:12:09 UTC, on head `chore(issues): archive closed issue #2949 spec to docs/issues/closed and set EPIC row 24 to DONE`; `DISMISSED` since the round-1 push) has no inline comment and states "Findings: none", so it has no row.

Round 1 is Copilot review `6011656162` (`COMMENTED`, on the same head), with one inline comment, F1 (an audit-local ID). The comment carries no severity label; `Suggestion` is inferred from its wording ("consider formatting"). The review body says the full agentic review did not run because it did not start before the timeout, and raises nothing else.

Round 2 is da2ce7's review `6011725447` (`COMMENTED`, on head `docs(issues): [#2949] quote the section title in EPIC row 24`), with one inline comment that carries the reviewer's ID F4 and an explicit severity label. F4 collides with no ID in this record, so the row keeps it and the record has no F2 or F3. The review was submitted after the round-1 fix was pushed and before this record's first commit reached the branch. da2ce7's review `6011764970` (`COMMENTED`, on head `docs(pr-reviews): [#2949] add the PR #2992 review audit record with round 1`) has no inline comment: it says F4 was partly addressed at that head and that its row keeps the reviewer's ID.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F1 | `review-finding:pr-2992-f1` | Copilot | Suggestion (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2992-f4` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The section title in EPIC row 24 reads as part of the sentence

- PR number: 2992
- Source review ID: 6011656162
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2992#discussion_r4657672369
- Concern: EPIC row 24 named the section An ACK Names One Commit as plain text, so the title could read as part of the surrounding sentence.
- Solution: The row puts the title in quotes.
- Current-tree verification: `grep -c 'the section "An ACK Names One Commit"' docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` prints 1.
- Resolution reference: `docs(issues): [#2949] quote the section title in EPIC row 24`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2992#discussion_r4657692217

### F4 - The thread was answered and resolved before the audit record existed

- PR number: 2992
- Source review ID: 6011725447
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2992#discussion_r4657706285
- Concern: The automated review's comment received a fix, a reply and a resolution, but at that head the branch had no `docs/pr-reviews/pr-2992-review/`; `process-pr-review` creates the record before the first reply.
- Solution: The record was added in the next commit, with the row for F1. The late start is logged in the Processing Log.
- Current-tree verification: `test -f docs/pr-reviews/pr-2992-review/PR-REVIEW.md` exits 0, and the pull request's two review threads map one to one to the Source URLs of rows F1 and F4.
- Resolution reference: `docs(pr-reviews): [#2949] add the PR #2992 review audit record with round 1`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2992#discussion_r4657709157

## Processing Log

- 2026-11-22 10:23 UTC - AI assistant (Claude Code, Claude Fable 5.2, issue #2949 session) - Started audit. The poller reported da2ce7's review 6011641355 (an approval with no finding) and their `ACK` on the first head (2026-11-22 10:12:12 UTC), then Copilot review 6011656162 (submitted 10:16:48 UTC). Round 1: normalized Copilot's inline comment as F1, fixed it in its own commit and pushed it, which made da2ce7's `ACK` stale; replied on the thread (10:23:18 UTC), then resolved it. Before writing this entry, listed every review and conversation comment of the pull request: none is left out.
- 2026-11-22 10:29 UTC - AI assistant (Claude Code, Claude Fable 5.2, issue #2949 session) - Late start, and round 2. The reply on F1's thread (10:23:18 UTC) and its resolution came before this record existed; the record's first commit was pushed about five minutes later. `process-pr-review` says to create the record before the first reply. da2ce7's review 6011725447 (submitted 2026-11-22 10:28:04 UTC, on the head between the two pushes) pointed this out. Normalized its inline comment as F2; the record's first commit already fixed it. Replied on the thread (10:29:01 UTC) and resolved it. Listed every review and conversation comment of the pull request before writing this entry: none is left out.
- 2026-11-22 10:34 UTC - AI assistant (Claude Code, Claude Fable 5.2, issue #2949 session) - Correction to the entry above: it says the comment was normalized as F2. da2ce7's review 6011764970 (submitted 2026-11-22 10:33:56 UTC) pointed out that `process-pr-review` step 3 keeps a reviewer's ID that collides with nothing, so the row is renamed F4, with the reference `review-finding:pr-2992-f4`. The first reply on the thread also said F2; a second reply (10:34:24 UTC, edited once to name a commit subject instead of a time) corrects it. The entry above stays as written.

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
