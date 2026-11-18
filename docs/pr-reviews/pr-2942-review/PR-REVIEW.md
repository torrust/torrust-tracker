---
pr-number: 2942
pr-url: https://github.com/torrust/torrust-tracker/pull/2942
last-updated-utc: "2026-11-18 12:19"
---

<!-- skill-link: process-pr-review -->

# PR #2942 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2942.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

Round 1 is the approving review `5956009614` of `da2ce7` (F1 to F6, reviewer-provided IDs) on the head that carried the
first audit commit for the late review of pull request 2920, and Copilot's review `5956016204` (COMMENTED, two comments
without IDs, recorded as F7 and F8) on the head before it, with the fix of F20 of pull request 2905.
Round 2 is the approving review `5956340268` of `da2ce7` (F22 to F25, reviewer-provided IDs) on the head that carried
the round 1 audit.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2942-f1` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2942-f2` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2942-f3` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2942-f4` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2942-f5` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2942-f6` | Human | Suggestion | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F7 | `review-finding:pr-2942-f7` | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F8 | `review-finding:pr-2942-f8` | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | `review-finding:pr-2942-f22` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F23 | `review-finding:pr-2942-f23` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F24 | `review-finding:pr-2942-f24` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F25 | `review-finding:pr-2942-f25` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - F16 says it arrived after the merge, but the record dates it before the merge

- PR number: 2942
- Source review ID: 5956009614
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613444212
- Concern: The Solution of F16 in the audit of pull request 2920 said "Received after the merge", and the round 6 introduction read the same way, but both reviews of round 6 were submitted before the merge (09:37:42Z and 09:53:37Z; merged 09:54:45Z).
- Solution: Both now say the feedback was submitted before the merge and unprocessed when it merged.
- Current-tree verification: `grep -c "Received after the merge"` on `docs/pr-reviews/pr-2920-review/PR-REVIEW.md` prints 0; the round 6 introduction says both reviews were submitted before the merge.
- Resolution reference: `docs(pr-reviews): [#2904] say the late review of PR #2920 came before its merge`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613536101

### F2 - The round 6 entry states a push that had not happened when it was written

- PR number: 2942
- Source review ID: 5956009614
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613444228
- Concern: The round 6 entry of the audit of pull request 2920 says "It was pushed right after, with this entry", written in the commit the push carried, which `process-pr-review` (Statements in Your Own Records) forbids. Its closing "until it merges" has no clear subject.
- Solution: The log is append-only, so the entry stays as written. A correction entry after it says the sentence was written before the push, records the push with its `git ls-remote` time, and says "until it merges" means until pull request #2942 merges. A stray blank line that split the log list is removed.
- Current-tree verification: The audit of pull request 2920 has the correction entry, naming 11:36:42Z; `validate-audit-record.py` exited 0 for it (19 rows, 19 log entries).
- Resolution reference: `docs(pr-reviews): [#2904] record the push of round 6 in the review audit of PR #2920`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613536477

### F3 - "that commit's subject" points at the merge commit, not the cited subject

- PR number: 2942
- Source review ID: 5956009614
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613444247
- Concern: The 11:30 entry of the audit of pull request 2905 says "that commit's subject" right after naming the merge commit of pull request 2920, while the row cites the subject of the fixing commit inside it.
- Solution: The entry stays as written (append-only log); a clarification entry after it names the fixing commit.
- Current-tree verification: The audit of pull request 2905 has the clarification entry naming `docs(issues): [#2904] log the named-model exception in the EPIC progress log`; `validate-audit-record.py` exited 0 for it (22 rows, 22 log entries).
- Resolution reference: `docs(pr-reviews): [#2904] name the fixing commit of F20 in the review audit of PR #2905`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613537031

### F4 - The description has no validation section

- PR number: 2942
- Source review ID: 5956009614
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613444263
- Concern: `open-pull-request` (Title and Description Convention) asks the description to include the validation performed; this one named no check.
- Solution: The description now has a Validation section naming the pre-commit gate, the pre-push hook, the audit validator for both records and the `git log --format=%aI` check.
- Current-tree verification: `gh pr view 2942 --json body` shows the `## Validation` heading and its three items.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2942#issuecomment-6977895799
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613537450

### F5 - The description names one process slip; the record names two

- PR number: 2942
- Source review ID: 5956009614
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613444283
- Concern: The audit bullet of the description named only the replies posted before the approval comment; the round 6 entry and the commit body name a second slip.
- Solution: The bullet now names both slips.
- Current-tree verification: `gh pr view 2942 --json body` shows both slips in the audit bullet.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2942#issuecomment-6977895799
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613537865

### F6 - The evidence template still prescribes the unquoted form

- PR number: 2942
- Source review ID: 5956009614
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613444306
- Concern: `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` line 4 has `last-updated-utc: YYYY-MM-DD HH:MM` without quotes, and seven open evidence files copy it.
- Solution: No change here, as the reviewer suggests: quoting the template and the files that copy it is outside the scope of this follow-up. It is raised with the maintainer as a separate follow-up.
- Current-tree verification: `sed -n 4p docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` prints the unquoted form.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613789740
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613789740

### F7 - The approval line mixes the `UTC` and `Z` notations

- PR number: 2942
- Source review ID: 5956016204
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613449769
- Concern: The Ownership line of the audit of pull request 2920 gives one time as `09:58 UTC` and another as `11:26:15Z`; Copilot suggests one notation throughout.
- Solution: No change. The Ownership lines of the audits of pull requests 2905 and 2920 both use `HH:MM UTC` for a time the maintainer stated and `HH:MM:SSZ` for the comment's `created_at`; this line follows them. The records are not uniform elsewhere: the audit of pull request 2905 writes the forge `mergedAt` of pull request 2920 as `09:54:45 UTC` twice, where the audit of pull request 2920 writes `09:54:45Z`. The first reply on this thread claimed the convention held in every entry; a second reply corrects that.
- Current-tree verification: The Ownership lines of both audits use the two forms as stated; `grep -n "09:54:45 UTC"` on `docs/pr-reviews/pr-2905-review/PR-REVIEW.md` prints lines 363 and 404.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613790102
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613790102

### F8 - The corrected verification line of F15 is hard to parse

- PR number: 2942
- Source review ID: 5956016204
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613449856
- Concern: "for four of the six commit times the log entries before the rebase entry give" is hard to read in an audit.
- Solution: The line now gives the four printed times, then says they are four of the six that the earlier log entries give, then names the other two.
- Current-tree verification: The Current-tree verification line of F15 in the audit of pull request 2920 reads as three sentences with that order.
- Resolution reference: `docs(pr-reviews): [#2904] reword the verification line of F15 in the review audit of PR #2920`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613583883

### F22 - F6 and F7 record `NO_ACTION` with `RESOLVED`, against this record's own rules

- PR number: 2942
- Source review ID: 5956340268
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613728263
- Concern: The rows of F6 and F7 were `NO_ACTION`/`RESOLVED` with replies starting "No change"; step 7 of `process-pr-review` and this record's rules pair a no-change in-PR thread with the reply `Superseded by <FindingId>: <reason>.` and `NO_ACTION`/`SUPERSEDED`.
- Solution: Both rows are `NO_ACTION`/`SUPERSEDED`, and each thread has a reply in that exact form, which is now the row's Reply URL.
- Current-tree verification: The rows of F6 and F7 read `NO_ACTION | SUPERSEDED`; their Reply URLs point at replies that start `Superseded by F6:` and `Superseded by F7:`.
- Resolution reference: `docs(pr-reviews): [#2904] correct the no-change rows and the validation order in the review audit of PR #2942`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613790596

### F23 - F7's reason claims a notation convention that this pull request's own lines break

- PR number: 2942
- Source review ID: 5956340268
- Reviewer finding ID: F23
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613728276
- Concern: The Solution of F7 said forge and command times are `HH:MM:SSZ` in every entry and in the audit of pull request 2905, but two lines of that audit added by this pull request write the forge `mergedAt` as `09:54:45 UTC`.
- Solution: The claim is narrowed to the Ownership lines, and the two lines that break it are named; the log of the audit of pull request 2905 is not edited, since it is append-only. The second reply on the F7 thread corrects the first.
- Current-tree verification: `grep -n "09:54:45 UTC"` on `docs/pr-reviews/pr-2905-review/PR-REVIEW.md` prints lines 363 and 404, the two lines the Solution of F7 names.
- Resolution reference: `docs(pr-reviews): [#2904] correct the no-change rows and the validation order in the review audit of PR #2942`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613791118

### F24 - The stamp of the audit of pull request 2920 did not move with the F15 rewording

- PR number: 2942
- Source review ID: 5956340268
- Reviewer finding ID: F24
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613728290
- Concern: The rewording of the F15 line changed the audit of pull request 2920 at 11:59:47Z without moving its `last-updated-utc`.
- Solution: The stamp is moved.
- Current-tree verification: `sed -n 4p docs/pr-reviews/pr-2920-review/PR-REVIEW.md` prints `last-updated-utc: "2026-11-18 12:15"`.
- Resolution reference: `docs(pr-reviews): [#2904] move the stamp of the review audit of PR #2920`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613791636

### F25 - F1 to F6 were resolved before the validator ran, and the log did not say so

- PR number: 2942
- Source review ID: 5956340268
- Reviewer finding ID: F25
- Source URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613728300
- Concern: The threads of F1 to F6 were resolved at 12:00:41Z, before the only validator run (8 rows); `process-pr-review` (Validation Script) runs the validator before resolving any thread.
- Solution: A correction entry records the order.
- Current-tree verification: The Processing Log has the correction entry naming 12:00:41Z.
- Resolution reference: `docs(pr-reviews): [#2904] correct the no-change rows and the validation order in the review audit of PR #2942`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2942#discussion_r4613792237

## Processing Log

- 2026-11-18 11:59 UTC - Started audit, late: the six replies of round 1 were posted before this record existed, which the `process-pr-review` workflow asks for the other way round.
- 2026-11-18 11:59 UTC - Round 1: the approving review `5956009614` of `da2ce7` (submitted 11:51:24Z, forge `submitted_at`) with six findings. The fixes of F1, F2 and F3 were committed between 11:52:17Z and 11:55:15Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 11:58:05Z (`date -u`). The description was edited and the conversation response for F4 and F5 posted at 11:58:18Z, and the six replies were posted between 11:58:27Z and 11:58:38Z (forge `created_at`), after the push. F6 is `NO_ACTION`: the reviewer asked for no change here.
- 2026-11-18 12:03 UTC - Resolved the six threads of F1 to F6 by their thread IDs at 12:00:41Z (`date -u`), each with its reply as the last comment and its fix pushed or, for F4, F5 and F6, stated.
- 2026-11-18 12:03 UTC - Copilot's review `5956016204` (submitted 11:51:50Z, forge `submitted_at`), found after the first entries were written, with two comments, recorded as F7 and F8. The reply to F7, which needs no change, was posted at 12:00:42Z. The fix of F8 was committed at 11:59:47Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 12:02:26Z (`date -u`), and the reply was posted at 12:02:32Z (forge `created_at`), after the push. `validate-audit-record.py --pr-number 2942 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (8 rows, 0 failures). Resolved the two threads of F7 and F8 by their thread IDs at 12:03:04Z; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-18 12:16 UTC - Correction to the entries above, after the review `5956340268` of `da2ce7`. The rows of F6 and F7 were `NO_ACTION`/`RESOLVED` with replies that start "No change"; this record's rules pair a no-change in-PR thread with the reply `Superseded by <FindingId>: <reason>.` and `NO_ACTION`/`SUPERSEDED`, so the rows are now `SUPERSEDED` and each thread gets that reply. The Solution of F7 claimed a notation convention that two lines of the audit of pull request 2905 break; it is narrowed. The six threads of F1 to F6 were resolved at 12:00:41Z, before the only validator run, which covered 8 rows and so ran after the rows of F7 and F8 existed: `process-pr-review` (Validation Script) runs the validator before resolving any thread.
- 2026-11-18 12:19 UTC - Round 2: the approving review `5956340268` of `da2ce7` (submitted 12:14:25Z, forge `submitted_at`) with four findings, recorded as F22 to F25 under the reviewer's IDs. Their fixes were committed at 12:15:19Z and 12:16:21Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 12:19:16Z (`date -u`). The two `Superseded by` replies on the threads of F6 and F7 and the four replies of round 2 were posted between 12:19:26Z and 12:19:38Z (forge `created_at`), after the push.
- 2026-11-18 12:21 UTC - `validate-audit-record.py --pr-number 2942 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (12 rows, 0 failures) at 12:20:50Z (`date -u`). Then resolved the four threads of F22 to F25 by their thread IDs, by 12:20:58Z; the threads of F6 and F7 stayed resolved. A GraphQL refresh showed 0 unresolved threads.

<!-- Copied verbatim into each audit record. -->

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
