---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2951 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2951.

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
| F1 | `review-finding:pr-2951-f1` | Copilot | Suggestion (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2951-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2951-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2951-f4` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2951-f5` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2951-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2951-f7` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2951-f8` | Human | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - Name the sub-issue identifier of 2221 in the problem list

- PR number: 2951
- Source review ID: 5963238031
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4619341298
- Concern: In the EPIC problem list, the status line of problem 3 names `#2221` by issue number only, while the other status lines use the roadmap identifier with the number, for example `SI-4 (#2169)`. The reviewer suggests `SI-5 (#2221)` so the problem list can be matched with the roadmap table.
- Solution: Changed it to `SI-5 (#2221)`. The same pattern was in three more places of the list, so they were changed in the same commit: `SI-10 (#2274)` in problem 4, `SI-14 (#2342)` and `SI-15 (#2370)` in problem 5. Problem 7 named `#2414` as `SI-22`, which is the identifier of its parent `#2410`; it is now `SI-22.2 (#2414)`. `#1586` keeps its plain number because the roadmap gives it no SI identifier.
- Current-tree verification: `grep -n -o "SI-[0-9.]* (#2221)\|SI-[0-9.]* (#2274)\|SI-[0-9.]* (#2342)\|SI-[0-9.]* (#2370)\|SI-[0-9.]* (#2414)" docs/issues/closed/1488-overhaul-tracker-shutdown/EPIC.md` prints the five identifiers on lines 115, 119, 124, 124 and 132. `ls docs/issues/closed` shows the folders `2221-1488-si-5-...`, `2274-1488-si-10-...`, `2342-1488-si-14-...`, `2370-1488-si-15-...` and `2414-2410-si-22-2-...`.
- Resolution reference: `docs(issues): [#1488] name the sub-issue identifiers in the EPIC problem list`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4619397335

### F2 - Restore two historical records of issue 2281

- PR number: 2951
- Source review ID: 5971000003
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625638358
- Concern: The path update rewrote two historical records so that they contradict themselves. The decision list of 2026-09-24 in `docs/issues/closed/2281-2264-frontmatter-validator-command/ISSUE.md` named `EPIC.md` but still said the file "is not renamed here", and line 166 of `agent-review-reports.md` counted the `EPIC.md` path among the 42 `ISSUE.md` records. Neither line is a link, and Step 4 of `cleanup-completed-issues` leaves such records unchanged.
- Solution: Restored both files to the merge base. `ISSUE.md` had no other change, so its `last-updated-utc` stamp is restored too.
- Current-tree verification: `git diff --stat 3a12cd095 HEAD -- docs/issues/closed/2281-2264-frontmatter-validator-command` prints nothing.
- Resolution reference: `docs(issues): [#1488] address the review of the EPIC close-out`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625665084

### F3 - Keep the historical citation in the evidence of issue 2413

- PR number: 2951
- Source review ID: 5971000003
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625638363
- Concern: In `docs/issues/closed/2413-2410-si-22-1-document-event-flows-and-draft-adr/manual-verification-evidence.md` the path named the new `EPIC.md` but "lines 279-280" was where the sentence sat in the old `ISSUE.md`; in `EPIC.md` it is at lines 349-350.
- Solution: Kept the historical path, the first of the two options the reviewer gave: the file is restored to the merge base, with its original stamp.
- Current-tree verification: `git diff --stat 3a12cd095 HEAD -- docs/issues/closed/2413-2410-si-22-1-document-event-flows-and-draft-adr` prints nothing.
- Resolution reference: `docs(issues): [#1488] address the review of the EPIC close-out`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625665437

### F4 - Do not cite a branch commit by its id in this record

- PR number: 2951
- Source review ID: 5971000003
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625638368
- Concern: The first entry of the processing log of this record cited the reviewed commit by its abbreviated id. The commit-id check of `open-pull-request` reports it, because a rebase invalidates branch ids.
- Solution: The entry now cites the commit by its subject.
- Current-tree verification: `grep -c "close the shutdown-overhaul EPIC and archive its spec" docs/pr-reviews/pr-2951-review/PR-REVIEW.md` counts the subject, and `grep -n -E "commit .[0-9a-f]{9}" docs/pr-reviews/pr-2951-review/PR-REVIEW.md` prints nothing.
- Resolution reference: `docs(pr-reviews): [#1488] record round 2 in the review audit of PR #2951`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625665738

### F5 - Do not state the close date before the close happens

- PR number: 2951
- Source review ID: 5971000003
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625638376
- Concern: Four lines said the EPIC was closed on 2026-11-18: `docs/architecture/application-jobs.md`, the status of `docs/features/shutdown-process/README.md`, and the completion note and the roadmap note of `EPIC.md`. The issue closes when this PR merges, which is not earlier than 2026-11-19, and it was not closed separately.
- Solution: The four lines now say that the maintainer decided on 2026-11-18 to close the EPIC. The intro of the problem list had the same problem ("at the close of the EPIC (2026-11-18)") and now says "at the close-out of the EPIC (written on 2026-11-18)".
- Current-tree verification: `grep -rn "closed on 2026-11-18\|Completed on 2026-11-18" docs/architecture docs/features docs/issues/closed/1488-overhaul-tracker-shutdown` prints nothing.
- Resolution reference: `docs(issues): [#1488] address the review of the EPIC close-out`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625666013

### F6 - Use EPIC.md as the link text in the spec of issue 2583

- PR number: 2951
- Source review ID: 5971000003
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625638383
- Concern: In `docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` the link text of the #1488 prerequisite still said `ISSUE.md` while the target is `EPIC.md`.
- Solution: The link text is now `EPIC.md`.
- Current-tree verification: `grep -rn "ISSUE.md.\](.*1488-overhaul-tracker-shutdown/EPIC.md" docs --include="*.md"` prints nothing.
- Resolution reference: `docs(issues): [#1488] address the review of the EPIC close-out`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625666285

### F7 - The exit code does not name any components

- PR number: 2951
- Source review ID: 5971000003
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625638387
- Concern: The status of problem 7 in `EPIC.md` said that SI-20 "made the final log line and the exit code name the components". `shutdown_exit_code` returns 0 or 1; the names come from `incomplete_components` in the final error line of `report_shutdown` (`src/app.rs`).
- Solution: Used the wording the reviewer suggested: the final log line names the components that did not stop gracefully, and the exit code is 1 when any did.
- Current-tree verification: `grep -n "exit code 1 when any did" docs/issues/closed/1488-overhaul-tracker-shutdown/EPIC.md` prints one line, in problem 7.
- Resolution reference: `docs(issues): [#1488] address the review of the EPIC close-out`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4625666631

### F8 - The description counts 45 closed specs

- PR number: 2951
- Source review ID: 5971000003
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2951#pullrequestreview-5971000003
- Concern: The body of the review says that the PR description's "45 closed specs" does not match: the reviewer counts 44 changed files in 30 closed folders outside the EPIC folder. The finding has no line and no thread.
- Solution: The count was wrong. After the fix of F2 and F3 the PR changes 41 files in 28 closed spec folders, and the description now says so and names the three historical records that keep the old path.
- Current-tree verification: `git diff --name-only 3a12cd095 HEAD -- docs/issues/closed` without the EPIC folder lists 41 files in 28 folders. The same command at the reviewed head lists 44 files in 30 folders, as the reviewer counted.
- Resolution reference: `docs(issues): [#1488] address the review of the EPIC close-out`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2951#issuecomment-7002876440

## Processing Log

- 2026-11-18 19:11 UTC - Started audit. Review 5963238031 (Copilot, `COMMENTED`, on the commit `docs(issues): [#1488] close the shutdown-overhaul EPIC and archive its spec`) has one inline thread and no actionable assertion in its body. The body says Copilot could not run its full agentic review because it did not start before the timeout; the review effort is "Lite".
- 2026-11-18 19:11 UTC - F1 fixed in `docs(issues): [#1488] name the sub-issue identifiers in the EPIC problem list`, committed at 19:08:41Z (`git log --format=%cI`) and pushed at 19:11:05Z (reflog of the remote-tracking branch), before this record was created. The first commit of this subject listed three of the four issue numbers in its body and was amended before the push; only the amended commit was pushed.
- 2026-11-18 19:11 UTC - Replied on the thread of F1 at 19:11:28Z (`created_at` of the reply). `validate-audit-record.py --pr-number 2951`, run with the token of `josecelano-bot`, exited 0 (1 row, 0 failures).
- 2026-11-18 19:11 UTC - Resolved thread `PRRT_kwDOGp2yqc65JacV` as `josecelano-bot`. A GraphQL refresh of the review threads counts 0 unresolved threads.
- 2026-11-19 07:38 UTC - Review 5963793766 (Copilot, `COMMENTED`, submitted 2026-11-18 19:42 UTC on the commit `docs(pr-reviews): [#1488] add the review audit of PR #2951`) has no inline thread and says it found no open issues. Nothing to record as a finding.
- 2026-11-19 07:38 UTC - Round 2. Review 5971000003 (da2ce7, `APPROVED`, submitted 2026-11-19 07:33 UTC on the same commit) has six inline threads, which carry the reviewer's ids F1 to F6, and one Nit in its body without a thread. They are F2 to F8 of this record. The reviewer says that none of them blocks.
- 2026-11-19 07:38 UTC - F2, F3, F5, F6, F7 and F8 fixed in `docs(issues): [#1488] address the review of the EPIC close-out`, committed at 07:36:06Z (`git log --format=%cI`). F4 is fixed in the commit that adds this entry. The description of the PR was corrected for F8.
- 2026-11-19 07:38 UTC - Replied on the six threads between 07:36:56Z and 07:37:08Z (`created_at` of the replies) and answered the Nit of the body in a comment on the PR conversation. The replies were posted before the push, so that the fix commit and this record go out in one push; the threads are resolved after it.

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
