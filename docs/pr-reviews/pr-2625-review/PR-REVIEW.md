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

Round 1: da2ce7's review 5625548479 (APPROVED, 2026-10-23 08:05:27 UTC, on the opening head, the commit whose subject is "chore(issues): archive closed issue #2618 spec to docs/issues/closed") carries four inline threads, F1 to F4, and no independently actionable assertion in its body; da2ce7 also posted an ACK of that head at 08:05:31 UTC, which is stale after the fix commits. Copilot had not reviewed when this audit started. The finding IDs are the reviewer's own; none collides with another. Severities are the bracketed labels of the source comments.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2625-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2625-f2 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2625-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2625-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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

## Processing Log

- 2026-10-23 08:07 UTC - da2ce7 submitted review 5625548479 (APPROVED) with four inline threads (F1 to F4) and an ACK of the opening head. GraphQL data at the 08:07 UTC poll: 4 threads, 4 unresolved.
- 2026-10-23 08:13 UTC - Verified each claim against the tree, committed the fixes (commit times 08:10 UTC), pushed them, edited the PR body (F4) and replied on every thread (reply times 08:13:32 to 08:13:37 UTC).
- 2026-10-23 08:14 UTC - Resolved all 4 threads; the reply check exited 0 before resolving, and a GraphQL query at 08:14 UTC showed 4 threads, 0 unresolved. The ACK names the head before the fixes, so a new ACK is needed on the head that carries this audit record.

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
