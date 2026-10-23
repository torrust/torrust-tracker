---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2625 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2625>.

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

## Findings

Round 1: da2ce7's review 5625548479 (APPROVED, 2026-10-23 08:05:27 UTC, on the opening head, the commit whose subject is "chore(issues): archive closed issue #2618 spec to docs/issues/closed") carries four inline threads, F1 to F4, and no independently actionable assertion in its body; da2ce7 also posted an ACK of that head at 08:05:31 UTC, which is stale after the fix commits. Copilot had not reviewed when this audit started; its review 5625619558 (COMMENTED, 2026-10-23 08:12:49 UTC, on the same opening head) arrived while the fixes were being prepared. It opens no inline thread and its body, which says "No open issues", only summarizes the change, so it raises no independently actionable assertion and adds no row. The finding IDs are the reviewer's own; none collides with another. Severities are the bracketed labels of the source comments.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2625-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2625-f2 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2625-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2625-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2625-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2625-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2625-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2625-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2625-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

Round 2: da2ce7's review 5625764397 (APPROVED, 2026-10-23 08:27:33 UTC, on the head that carried the round 1 audit record) carries three inline threads, F5 to F7, and no independently actionable assertion in its body; its ACK (08:27:38 UTC) names that head and calls them non-blocking. The IDs continue the first round's and collide with none.

Round 3: da2ce7's review 5625983587 (APPROVED, 2026-10-23 08:49:09 UTC, on the head that carried the round 2 resolution entry) carries two inline threads, F8 and F9, both Nits, and no independently actionable assertion in its body; its ACK (08:49:13 UTC) names that head and calls them non-blocking. The IDs continue the earlier rounds' and collide with none.

## Finding Details

### F1 - The archive rewrote a historical log line

- PR number: 2625
- Source review ID: 5625548479
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357426385>
- Concern: the 2026-10-21 16:38 progress log entry records the promotion from `drafts/` to `open/`; the archive's path replacement made it say `docs/issues/closed/...`, though the skill keeps records that accurately name the path of their time.
- Solution: the entry says `docs/issues/open/...` again; the 07:32 entry records the archive.
- Current-tree verification: `git show` of the spec-PR commit that wrote the entry (it says `open/`), and the skill's lines on historical records.
- Resolution reference: docs(issues): [#2618] restore the 16:38 log path, cite the merge as the ADR approval and give the unticked reason
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357486684>

### F2 - The EPIC status still says planned

- PR number: 2625
- Source review ID: 5625548479
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357426391>
- Concern: the EPIC frontmatter said `status: planned` although row 1 is DONE and the next row is named, which is the state AC6 measures.
- Solution: the EPIC `status` is `in-progress`, with a progress log entry.
- Current-tree verification: read the EPIC frontmatter and the allowed status values (`planned`, `in-progress`, ...) before and after.
- Resolution reference: docs(issues): [#2482] set the EPIC in-progress and cite the merge as the ADR approval
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357486923>

### F3 - The AC2 evidence cites an approval that no artifact records

- PR number: 2625
- Source review ID: 5625548479
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357426394>
- Concern: the AC2 evidence cell cited the maintainer's answer to the archive question, which exists only in the session; the durable evidence is his merge of PR #2624 (`bf03425ef`).
- Solution: the AC2 cell in the spec and in the EPIC cites the merge; the append-only 07:32 entries are corrected by new log entries in both files.
- Current-tree verification: `gh pr view 2624 --json mergeCommit` gives `bf03425ef`; grep of the two files for the old wording.
- Resolution reference: docs(issues): [#2618] restore the 16:38 log path, cite the merge as the ADR approval and give the unticked reason
- Further resolution reference: docs(issues): [#2482] set the EPIC in-progress and cite the merge as the ADR approval
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357487104>

### F4 - The PR body says both unticked items carry a reason, but only one does

- PR number: 2625
- Source review ID: 5625548479
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357426400>
- Concern: the "Committer verified spec progress" checkpoint stayed unticked without a reason.
- Solution: the checkpoint states its reason (the Committer agent was not used); the PR body now says each unticked item has its reason in the spec.
- Current-tree verification: `grep -n 'Committer verified'` of the spec; read the edited PR body.
- Resolution reference: docs(issues): [#2618] restore the 16:38 log path, cite the merge as the ADR approval and give the unticked reason
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357487327>

### F5 - The PR description still says the approval came after the merge

- PR number: 2625
- Source review ID: 5625764397
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357594411>
- Concern: the PR description, which the merge tool copies into the merge commit, said "the maintainer approved the written ADR on 2026-10-23, after the merge", while the files say the approval is the merge; the merge commit `bf03425ef` lists the maintainer's ACK of the head under "ACKs for top commit", so the approval came at the merge. The merged files were right, but the description cannot be corrected after the merge.
- Solution: the description now says the maintainer approved the written ADR by ACKing and merging #2624, and that the ticks land here because nothing may be pushed to a pull request branch after the ACK. This is a change to the pull request description, not to the branch.
- Current-tree verification: `git log -1 --format=%b bf03425ef` lists `ACK ff44a2bac...` twice; `gh pr view 2625 --json body` contains the new sentence after the edit (updated 08:29:01 UTC).
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357609931>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357609931>

### F6 - The record omits the review bot's review 5625619558

- PR number: 2625
- Source review ID: 5625764397
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357594422>
- Concern: Copilot's review 5625619558 (COMMENTED, 08:12:49 UTC, on the opening head) opened no thread and says "No open issues", but the audit neither mentioned it nor logged it.
- Solution: the Findings introduction says the review carries no inline thread and no independently actionable assertion, and the Processing Log has an entry for it.
- Current-tree verification: `gh api .../reviews/5625619558` shows state COMMENTED, 08:12:49 UTC, the opening head, and `.../comments` returns 0 comments; its body summarizes the change and says "No open issues".
- Resolution reference: docs(pr-reviews): add the review bot review and the event times to the PR 2625 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357610207>

### F7 - A log entry carries the poll time instead of the event time

- PR number: 2625
- Source review ID: 5625764397
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357594429>
- Concern: the first entry is stamped 08:07 for the submission of review 5625548479, which was submitted at 08:05:27 UTC; 08:07 was the poll time.
- Solution: the Processing Log is append-only, so the entry is not rewritten; a new entry states that the review was submitted at 08:05:27 UTC and the ACK at 08:05:31 UTC, and that 08:07 is the poll time. The new entries are stamped with the time they were written, so the log stays in order.
- Current-tree verification: `gh api .../reviews` gives 08:05:27Z for review 5625548479 and the issue comment 08:05:31Z.
- Resolution reference: docs(pr-reviews): add the review bot review and the event times to the PR 2625 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357610482>

### F8 - A log entry is stamped later than its own commit

- PR number: 2625
- Source review ID: 5625983587
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357763728>
- Concern: the 08:35 entry was written before the commit that contains it (committed 08:31:19 UTC, pushed 08:33:43 UTC), and it logs the pull request description edit without that edit's own time (08:29:01 UTC). I chose the 08:35 stamp instead of reading the clock.
- Solution: an appended correction entry says the 08:35 entry was written by 08:31 UTC at the latest and that the description was edited at 08:29:01 UTC; the entry itself is not rewritten because the log is append-only.
- Current-tree verification: `git log --format='%cI %s'` gives the commit time 09:31:19+01:00 (08:31:19 UTC); `gh pr view 2625 --json updatedAt` showed 08:29:01Z after the edit.
- Resolution reference: docs(pr-reviews): correct the PR 2625 audit stamps and cite the reply for the description fix
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357783663>

### F9 - F5's resolution reference should be the reply URL

- PR number: 2625
- Source review ID: 5625983587
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357763736>
- Concern: F5's fix is a pull request description edit, which lives outside the tree, but its resolution reference named an audit commit that only records the fix; the skill says to use the durable URL of the conversation response that states it.
- Solution: F5's resolution reference is now the reply URL <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357609931>.
- Current-tree verification: `grep -n 'Resolution reference'` of the F5 detail shows the URL; the validator accepts it (it skips references that start with `http`).
- Resolution reference: docs(pr-reviews): correct the PR 2625 audit stamps and cite the reply for the description fix
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2625#discussion_r4357783971>

## Processing Log

- 2026-10-23 08:07 UTC - da2ce7 submitted review 5625548479 (APPROVED) with four inline threads (F1 to F4) and an ACK of the opening head. GraphQL data at the 08:07 UTC poll: 4 threads, 4 unresolved.
- 2026-10-23 08:13 UTC - Verified each claim against the tree, committed the fixes (commit times 08:10 UTC), pushed them, edited the PR body (F4) and replied on every thread (reply times 08:13:32 to 08:13:37 UTC).
- 2026-10-23 08:14 UTC - Resolved all 4 threads; the reply check exited 0 before resolving, and a GraphQL query at 08:14 UTC showed 4 threads, 0 unresolved. The ACK names the head before the fixes, so a new ACK is needed on the head that carries this audit record.
- 2026-10-23 08:28 UTC - da2ce7 submitted review 5625764397 (APPROVED) with three threads (F5 to F7) and an ACK of the head that carried the round 1 audit record (review 08:27:33 UTC, ACK 08:27:38 UTC). GraphQL data at the 08:28 UTC poll: 7 threads, 3 unresolved.
- 2026-10-23 08:35 UTC - Correction to the first Processing Log entry (F7): review 5625548479 was submitted at 08:05:27 UTC and its ACK posted at 08:05:31 UTC; 08:07 UTC was the poll that saw them. Recorded now: Copilot's review 5625619558 (COMMENTED, submitted 08:12:49 UTC, on the opening head) opened no thread and raised no actionable assertion (F6). Edited the pull request description (F5).
- 2026-10-23 08:37 UTC - Replied on the three round 2 threads (reply times 08:29:34 to 08:29:38 UTC), pushed the audit commit, and resolved F5 to F7; the reply check exited 0 before resolving, and a GraphQL query at 08:33 UTC (before this entry was written) showed 7 threads, 0 unresolved. The ACK names the head before this record's last commit, so a new ACK is needed on the head that carries it.
- 2026-10-23 08:51 UTC - Corrections after da2ce7's round 3 review 5625983587 (F8, F9). F8: the 08:35 entry's stamp was not taken from the clock; its text was written before the commit whose subject is "docs(pr-reviews): add the review bot review and the event times to the PR 2625 audit" was made at 08:31:19 UTC (pushed 08:33:43 UTC), so it was written by 08:31 UTC at the latest, and the pull request description it mentions was edited at 08:29:01 UTC. The entry is not rewritten, because the log is append-only. F9: F5's resolution reference now is the durable URL of the reply that states the fix, because the fix is a pull request description edit, which lives outside the tree.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
