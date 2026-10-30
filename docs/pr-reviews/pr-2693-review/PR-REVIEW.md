---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2693 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2693.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (this pull request is the approved follow-up of #2690; its approval is recorded in `docs/pr-reviews/pr-2690-review/PR-REVIEW.md`)

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

All findings are by the human reviewer `da2ce7`. F1 to F6 are from review 5715892693 (CHANGES_REQUESTED, 2026-10-30 17:09:01 UTC, on the head whose subject is `docs(pr-reviews): [#2547] record the post-merge findings F20 to F22 of PR 2690`). The reviewer's own IDs F1 to F6 are kept; this pull request had no audit before, so there is no collision. F2 to F4 are blocking, F1, F5 and F6 are nonblocking. Review 5716380696 (CHANGES_REQUESTED, 2026-10-30 17:50:05 UTC, on the head whose subject is `docs(pr-reviews): [#2547] record the resolution of the PR 2693 review threads`) resolved F1 to F4 and F6 and raised F7 (Minor, blocking), which continues F5; F8 is its "Not yet" reply (comment r4426734938) in the thread of F5; that reply belongs to review 5716383184, which is its Source review ID and the review its Source URL names, while the comment link is kept in the Concern. Review 5717204981 (APPROVED, 2026-10-30 18:51:18 UTC, on the head whose subject is `docs(pr-reviews): [#2547] record the second review round of PR 2693`, with the reviewer's own ACK comment) had no blocking finding and one nonblocking Nit, F9, on that attribution; the reviewer numbered it F9 because this audit already held its own row F8.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2693-f1 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2693-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2693-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2693-f4 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2693-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2693-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2693-f7 | Human | Minor | correctness | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F8 | review-finding:pr-2693-f8 | Human | Minor (inferred) | correctness | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F9 | review-finding:pr-2693-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Refresh the affected spec's update date

- PR number: 2693
- Source review ID: 5715892693
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426354663
- Concern: The archive repairs a live reference in the #2507 spec, whose `last-updated-utc` still read "2026-10-16 15:40"; cleanup step 4 requires the current date when an affected document is modified.
- Solution: The finding was right: `last-updated-utc` of the #2507 spec is refreshed and `status: done` is preserved.
- Current-tree verification: `docs/issues/closed/2507-stop-requiring-pull-request-rebases/ISSUE.md` has the refreshed `last-updated-utc` and `status: done`.
- Resolution reference: `docs(issues): [#2547] refresh the update date of the #2507 spec after the path repair`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426445674

### F2 - Correct the first commit's before/after-merge claim

- PR number: 2693
- Source review ID: 5715892693
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426354668
- Concern: The body of the first commit said the F20/F21 findings were "raised after the merge", but review 5713751044 and its comments were submitted at 14:28:48 UTC and the merge was at 16:24:59 UTC: they preceded it by 1 h 56 min 11 s and were unprocessed at merge.
- Solution: The finding was right. A commit body cannot be fixed by a new commit, so the history was rewritten once (`git rebase -i`, `git push --force-with-lease`) to reword that body; every commit was kept and the final tree is identical to the one before the rewrite. The pull request body says the history was rewritten, and the old commit id is replaced here by the commit's subject.
- Current-tree verification: `git log` of the branch shows the reworded body ("still unprocessed when that pull request merged at 16:24:59 UTC"); `git diff <pre-rewrite head> HEAD` is empty.
- Resolution reference: `docs(issues): [#2547] make the merge-list construction reproducible and the CI total consistent`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426446031

### F3 - Preserve the reviewer-provided finding provenance

- PR number: 2693
- Source review ID: 5715892693
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426354678
- Concern: The audit of PR #2690 said the reviewer did not number F20 to F22, but the source comments are titled `[Minor][F20]`, `[Minor][F21]` and `[Nit][F22]`; these are retained reviewer IDs, and `Reviewer finding ID: N/A` may remain under the template.
- Solution: The finding was right: the introduction of that audit now says the IDs are retained from the comment titles.
- Current-tree verification: The introduction of `docs/pr-reviews/pr-2690-review/PR-REVIEW.md` names the three titles.
- Resolution reference: `docs(pr-reviews): [#2547] retain the reviewer-provided ids of F20 to F22`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426446480

### F4 - Retain the F3 continuation relationship for F20 and F21

- PR number: 2693
- Source review ID: 5715892693
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426354684
- Concern: The source comments of F20 and F21 explicitly continue round-1 F3, but both rows were marked `ORIGINAL`, which loses the re-raise history; the workflow requires `RE_RAISE_OF:F3`. F22 stays original.
- Solution: The finding was right: both rows of the #2690 audit now carry `RE_RAISE_OF:F3`, and their concerns say they continue F3.
- Current-tree verification: The tracking rows of F20 and F21 in `docs/pr-reviews/pr-2690-review/PR-REVIEW.md` read `RE_RAISE_OF:F3`; the validator passes.
- Resolution reference: `docs(pr-reviews): [#2547] record F20 and F21 as re-raises of F3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426446797

### F5 - Use reply URLs as the pending follow-up resolution references

- PR number: 2693
- Source review ID: 5715892693
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426354689
- Concern: A `FOLLOW_UP` finding requires a durable reply URL as its resolution reference; the three new details of the #2690 audit cited commit subjects, which is the form for `FIXED`.
- Solution: The finding was right: the Resolution reference of F20, F21 and F22 is the reply URL on its thread, with the commit subject as context and the follow-up pull request in its own field.
- Current-tree verification: The three detail entries in `docs/pr-reviews/pr-2690-review/PR-REVIEW.md` cite r4426151564, r4426151939 and r4426152427.
- Resolution reference: `docs(pr-reviews): [#2547] use the reply urls as the pending resolution references`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426447169

### F6 - Remove the closing-keyword form from the archive PR title

- PR number: 2693
- Source review ID: 5715892693
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426354695
- Concern: The pull request title contained `closed #2547`, which the skill's closing-keyword pattern matches; the title becomes the merge subject and would request closure of the already closed issue.
- Solution: The finding was right: the title is now "docs(issues): archive the spec for closed issue #2547 and correct the post-merge review findings of PR 2690". A title edit is not a commit, so the reference is the reply.
- Current-tree verification: The skill's pattern finds no match in the pull request title, the body or any commit message of the branch.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426447491
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426447491

### F7 - Keep the pending resolution references parseable

- PR number: 2693
- Source review ID: 5716380696
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426732741
- Concern: Continues F5. The semicolon in the parenthetical of each F20 to F22 Resolution reference in the PR #2690 audit split it into two references, and the validator read the text after it as a nonexistent commit subject: `validate-audit-record.py --pr-number 2690` reported 3 failures, while the earlier statement that the validator passes covered only the PR #2693 audit.
- Solution: The finding was right and was reproduced (3 failures). Each Resolution reference is only the durable reply URL, and the commit subject moved into Solution. Both audit records were revalidated at the fixed head.
- Current-tree verification: `validate-audit-record.py --pr-number 2690` returns ok (22 rows, 0 failures) and `--pr-number 2693` returns ok (8 rows, 0 failures) at this head.
- Resolution reference: `docs(pr-reviews): [#2547] keep the pending resolution references parseable`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426784465

### F8 - Re-check of F5 at the previous head

- PR number: 2693
- Source review ID: 5716383184
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2693#pullrequestreview-5716383184
- Concern: A "Not yet" reply (inline comment r4426734938, link https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426734938; it belongs to review 5716383184 and replies in the thread of F5, root r4426354689): the reply URLs are right, but the semicolon-separated text is parsed as nonexistent commit subjects; F7 continues it.
- Solution: The fix of F7 completes F5; see that finding.
- Current-tree verification: Same as F7.
- Resolution reference: `docs(pr-reviews): [#2547] keep the pending resolution references parseable`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4426785048

### F9 - Preserve the actual review owning the F5 reply

- PR number: 2693
- Source review ID: 5717204981
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4427378823
- Concern: The row F8 named review 5716380696 as the source of the reply r4426734938, but that comment has `pull_request_review_id` 5716383184 and `in_reply_to_id` 4426354689; the validator's review-URL branch only compares the two supplied review IDs, so its pass did not validate this provenance. Nonblocking: the reply link was right.
- Solution: The finding was right and was verified against the API. The row F8 and the introduction name review 5716383184, with the comment link and the F5 thread relationship kept in the Concern.
- Current-tree verification: The detail of F8 reads Source review ID 5716383184 and a Source URL for that review; `validate-audit-record.py --pr-number 2693` still returns ok.
- Resolution reference: `docs(pr-reviews): [#2547] attribute the F5 re-check reply to its owning review`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2693#discussion_r4427416460

## Processing Log

- 2026-10-30 17:10 UTC - Review 5715892693 arrived (CHANGES_REQUESTED, 17:09:01 UTC, six inline findings F1 to F6, all confirmed against the tree and the PR metadata). The maintainer had asked on 2026-10-30 to fix the reviewer's findings if a review came within 15 minutes.
- 2026-10-30 17:20 UTC - Fixes and replies (completed events). The fix commits were created at 2026-10-30 17:11:21 UTC (F1), 17:12:16 UTC (F3), 17:13:00 UTC (F4) and 17:13:45 and 17:15:04 UTC (F5). F2 reworded the body of the first commit of the branch, whose original author time is 16:33:01 UTC, by one scripted (non-interactive) `git rebase -i` over all eight commits; the final tree was verified identical to the pre-rewrite head, and the branch was pushed with `--force-with-lease` (the PR timeline shows `head_ref_force_pushed` at 17:18:24 UTC). F6 was done by editing the pull request title (`renamed` event at 17:18:31 UTC). The six replies were posted from 17:18:50 to 17:19:01 UTC. Not yet events when this entry was written: the resolution of the six threads and the push of this record.
- 2026-10-30 17:23 UTC - Resolved the six threads (GraphQL `resolveReviewThread`, 17:23 UTC); a GraphQL refresh at 17:23 UTC reported 0 unresolved threads. The record of the fixes was pushed at 17:22 UTC (head whose subject is `docs(pr-reviews): [#2547] record the review of PR 2693`). `validate-audit-record.py --pr-number 2693` exited 0 (6 rows, 0 failures) before this entry. The three threads of #2690 (F20 to F22) stay open until this pull request merges.
- 2026-10-30 17:57 UTC - Round 2: review 5716380696 (CHANGES_REQUESTED, 17:50:05 UTC) confirmed F1 to F4 and F6 and raised F7. It was reproduced (the validator failed on the PR #2690 audit with 3 failures, which this audit's 17:23 entry did not cover: it validated only this audit). The fix commit was created at 17:52:45 UTC and pushed at about 17:55 UTC; the replies on the F7 thread and on the F5 thread were posted at 17:55:30 and 17:55:32 UTC, the F7 thread was resolved afterwards, and a GraphQL refresh at 17:55:58 UTC reported 0 unresolved threads. The maintainer re-requested da2ce7 after the previous push. Not yet events when this entry was written: the push of this record.
- 2026-10-30 18:57 UTC - Round 3: review 5717204981 (APPROVED, 18:51:18 UTC, with da2ce7's own ACK comment on the head whose subject is `docs(pr-reviews): [#2547] record the second review round of PR 2693`) confirmed F5 and F7 and raised one nonblocking Nit, F9, on the source-review attribution of the row F8. It was verified against the API (the reply r4426734938 belongs to review 5716383184). The fix commit was created at 18:52:58 UTC and pushed before the reply was posted (18:55:31 UTC); the thread was resolved at about 18:55:44 UTC and a GraphQL refresh reported 0 unresolved threads. The maintainer had asked to fix newer findings, so the nit was fixed before merge although the reviewer allowed it after; that makes the ACK on the earlier head stale. Not yet events when this entry was written: the push of this record.

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
