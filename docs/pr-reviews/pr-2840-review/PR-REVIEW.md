---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2840 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2840.

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

F1 to F3 are in da2ce7's review 5875801804 and are inline threads; each carries his own finding ID. R1 and R2 are assertions in the summary of Copilot's review 5875883208, which has no inline thread, so their source is the review and their thread state is `NON_RESOLVABLE`; one PR conversation response names both with their dispositions.

F5 and F7 to F12 are in da2ce7's review 5876404520, and F14 to F19 in his review 5876857550 (which approved the head of the previous push); their rows keep the reviewer's own finding IDs, which collide with none of the earlier ones. All of them are inline threads.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2840-f1 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2840-f2 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2840-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2840-r1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| R2 | review-finding:pr-2840-r2 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F1 | FIXED | NON_RESOLVABLE |
| F5 | review-finding:pr-2840-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2840-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2840-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2840-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2840-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2840-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2840-f12 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2840-f14 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2840-f15 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2840-f16 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2840-f17 | Human | Nit | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F18 | review-finding:pr-2840-f18 | Human | Nit | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The ticked checkpoint contradicts the unticked `linter all` criterion

- PR number: 2840
- Source review ID: 5875801804
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551481295
- Concern: `linter all` was not recorded and its criterion was unticked while the checkpoint was ticked.
- Solution: `linter all` was run (exit 0) and recorded in the evidence, and the criterion is ticked.
- Current-tree verification: The "Linters" section of the evidence and the criterion in the spec.
- Resolution reference: docs(issues): [#2822] record the linter run and state the AC5 deviation in the close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551561813

### F2 - The completion review says no deviation but AC5 and D2 were narrowed

- PR number: 2840
- Source review ID: 5875801804
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551481302
- Concern: The review said the implementation followed the merged spec with no deviation, but AC5 and D2's test bullet were narrowed in PR #2835.
- Solution: The review states the narrowing as a deviation with the reason for no separate retrospective; the PR description is corrected.
- Current-tree verification: Implementation Completion Review in the spec; `gh pr view 2840 --json body`.
- Resolution reference: docs(issues): [#2822] record the linter run and state the AC5 deviation in the close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551562531

### F3 - The reason for no retrospective is not in the progress log

- PR number: 2840
- Source review ID: 5875801804
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551481312
- Concern: The reason was only in the completion review.
- Solution: A new progress-log entry carries the reason.
- Current-tree verification: The last progress-log entry of the spec.
- Resolution reference: docs(issues): [#2822] record the linter run and state the AC5 deviation in the close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551563276

### R1 - The archived issue lacks the required issue-local retrospective

- PR number: 2840
- Source review ID: 5875883208
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#pullrequestreview-5875883208
- Concern: The summary of Copilot's review says the archived issue has no issue-local retrospective.
- Solution: `implementation-retrospective.md` is added from the template and the completion review points to it.
- Current-tree verification: `implementation-retrospective.md` exists in the closed folder; the Implementation Completion Review names it.
- Resolution reference: docs(issues): [#2822] add the implementation retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#issuecomment-6826373230

### R2 - No recorded `linter all` verification evidence

- PR number: 2840
- Source review ID: 5875883208
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#pullrequestreview-5875883208
- Concern: The summary says the `linter all` verification is not recorded.
- Solution: `linter all` was run (exit 0) and recorded in the evidence; its criterion is ticked.
- Current-tree verification: The "Linters" section of the evidence and the criterion in the spec.
- Resolution reference: docs(issues): [#2822] record the linter run and state the AC5 deviation in the close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#issuecomment-6826373230

### F5 - The run is dated 14:08 UTC in the evidence but the 14:07 entry reports it

- PR number: 2840
- Source review ID: 5876404520
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551940242
- Concern: The 14:07 progress-log entry reported a run that was made at 14:08.
- Solution: An appended log entry in the spec says the run was made at 14:08 UTC and the 14:07 stamp is a minute early.
- Current-tree verification: The last entries of the spec's Progress Log.
- Resolution reference: docs(issues): [#2822] correct the retrospective and say where its request was made
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552018945

### F10 - The retrospective request has no trace on the PR

- PR number: 2840
- Source review ID: 5876404520
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551940216
- Concern: The spec said the maintainer asked during review of the close-out PR, but the only request on the PR is the overview review.
- Solution: The completion review says where the request was made (the session, after Copilot's review, row R1) and an appended entry clarifies the 14:23 entry.
- Current-tree verification: The Implementation Completion Review and the last Progress Log entry of the spec.
- Resolution reference: docs(issues): [#2822] correct the retrospective and say where its request was made
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552016683

### F11 - Items 1 and 2 of the retrospective contradict the evidence

- PR number: 2840
- Source review ID: 5876404520
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551940225
- Concern: Item 1 said the tests failed for the stated reason and item 2 said no `gh` was ever on `PATH`.
- Solution: Item 1 gives eight failed and two passed; item 2 says the default-`gh` scenarios run a fake named `gh` from the sandbox `PATH`.
- Current-tree verification: What Went Well in `implementation-retrospective.md`.
- Resolution reference: docs(issues): [#2822] correct the retrospective and say where its request was made
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552017467

### F12 - The retrospective gives the missing agents as the reason it was missed

- PR number: 2840
- Source review ID: 5876404520
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551940234
- Concern: The rule was in front of the work and the close-out replaced it.
- Solution: The Root Cause names the close-out's first commit as the decision; the agent point stays under Improvements.
- Current-tree verification: Root Cause in `implementation-retrospective.md`.
- Resolution reference: docs(issues): [#2822] correct the retrospective and say where its request was made
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552018237

### F7 - The log never says when the record was started

- PR number: 2840
- Source review ID: 5876404520
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551940179
- Concern: The record started after the first replies and the log did not say so.
- Solution: An appended entry states the start at 14:14 UTC and the earlier replies.
- Current-tree verification: The corrections entry of the Processing Log.
- Resolution reference: docs(pr-reviews): [#2822] correct the audit of PR #2840 and record the second review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552080487

### F8 - R2 is a re-raise of F1, not original

- PR number: 2840
- Source review ID: 5876404520
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551940197
- Concern: R2 asks for the same change as F1, later.
- Solution: R2 is `RE_RAISE_OF:F1` with category `documentation`; the correction is logged.
- Current-tree verification: The R2 row and the corrections entry.
- Resolution reference: docs(pr-reviews): [#2822] correct the audit of PR #2840 and record the second review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552081438

### F9 - The 14:30 entry is stamped later than its commit

- PR number: 2840
- Source review ID: 5876404520
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551940208
- Concern: The stamp and the response time were not taken from the commit and the forge.
- Solution: An appended entry corrects them (commit 14:29:43 UTC, response 14:28:57 UTC).
- Current-tree verification: The corrections entry of the Processing Log.
- Resolution reference: docs(pr-reviews): [#2822] correct the audit of PR #2840 and record the second review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552082276

### F14 - "Addressed the second review" names no review

- PR number: 2840
- Source review ID: 5876857550
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552282451
- Concern: The spec log entry did not name the review it meant.
- Solution: A later entry names review 5876857550 and says which review the earlier entry meant.
- Current-tree verification: The last Progress Log entry of the spec.
- Resolution reference: docs(issues): [#2822] state the process cause of the missed retrospective and name the review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552349759

### F15 - The retrospective puts the cause on a person

- PR number: 2840
- Source review ID: 5876857550
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552282438
- Concern: "The decision not to follow it was mine" is individual blame in a blameless review.
- Solution: The root cause bullet states the process cause.
- Current-tree verification: Root Cause in `implementation-retrospective.md`.
- Resolution reference: docs(issues): [#2822] state the process cause of the missed retrospective and name the review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552348985

### F16 - "With `create-issue` saying the same" overstates it

- PR number: 2840
- Source review ID: 5876857550
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552282464
- Concern: `create-issue` is less strict than the spec's instruction.
- Solution: The bullet says what `create-issue` asks and that the spec's instruction was stricter.
- Current-tree verification: Root Cause in `implementation-retrospective.md`.
- Resolution reference: docs(issues): [#2822] state the process cause of the missed retrospective and name the review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552350716

### F17 - A commit body says two stamps where it clarifies one stamp and a wording

- PR number: 2840
- Source review ID: 5876857550
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552282474
- Concern: The tree needs no change; only a commit body is imprecise.
- Solution: No change and no rewrite of the pushed branch; the reply states what the commit clarifies.
- Current-tree verification: The reply.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552351534
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552351534

### F18 - A commit subject does not say what the commit adds

- PR number: 2840
- Source review ID: 5876857550
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552282478
- Concern: The tree needs no change; only a commit subject is imprecise.
- Solution: No change and no rewrite of the pushed branch; the reply states what the commit adds.
- Current-tree verification: The reply.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552352355
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4552352355

## Processing Log

- 2026-11-12 14:06 UTC - Review 5875801804 (da2ce7, `CHANGES_REQUESTED`) submitted on the head of the close-out PR: F1, F2, F3.
- 2026-11-12 14:13 UTC - Replied to and resolved the three threads after the commit `docs(issues): [#2822] record the linter run and state the AC5 deviation in the close-out` was pushed; the reply URLs are in the entries.
- 2026-11-12 14:30 UTC - Review 5875883208 (Copilot, `COMMENTED`) had been submitted at 14:12 UTC on the first head of the close-out PR: R1 and R2 in its summary, no inline threads. The retrospective commit was pushed at 14:28 UTC (push log time), and the PR conversation response that names the review and both findings with their dispositions and resolution subjects was posted at 14:29 UTC (its URL is in the entries). One entry carries these events because the 14:13 entry above was written before the review was read.
- 2026-11-12 14:51 UTC - Review 5876404520 (da2ce7, `APPROVED`) had been submitted at 14:44 UTC on the head of the previous push, with seven findings, none blocking. The spec commit that fixes F5, F10, F11 and F12 was pushed at 14:50 UTC (push log time) and those four threads were replied to and resolved at 14:51 UTC; their rows are in this record.
- 2026-11-12 14:51 UTC - Corrections, the entries above stay as they are. (a) This record was started at 14:14 UTC (the commit that added it was made at 14:14:08 UTC), after the first three replies, which were posted at 14:13:46 to 14:13:54 UTC. (b) Row R2 asks for the same change as F1, later than F1, so it is `RE_RAISE_OF:F1` with the category `documentation`; its row was changed in place. (c) The 14:30 entry was stamped later than its commit (made at 14:29:43 UTC), and the PR conversation response it dates at 14:29 was posted at 14:28:57 UTC (forge time); the retrospective commit's push at 14:28 UTC is as logged.
- 2026-11-12 14:56 UTC - The audit commit that fixes F7, F8 and F9 was pushed at 14:55 UTC (push log time) and those three threads were replied to and resolved after it; their rows and reply URLs are in this record.
- 2026-11-12 15:18 UTC - Review 5876857550 (da2ce7, `APPROVED`) had been submitted at 15:12 UTC on the head of the previous push, with six findings, none blocking. The spec commit that fixes F14, F15 and F16 was pushed at 15:18 UTC (push log time); those threads and the two no-change threads (F17, F18) were replied to and resolved at 15:18 UTC, and their rows are in this record. F19, the introduction of this record, is fixed by the commit that carries this entry.

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
