---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2814 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2814.

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

Round 1 is da2ce7's review `5848125288` (F1 to F5) and Copilot's review `5848148251` (F11, numbered after the reviewer-provided IDs). Round 2 is da2ce7's review `5849963483` (F6 to F10). Both rounds were processed together because the first was found late.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2814-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2814-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2814-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2814-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2814-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2814-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2814-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2814-f8 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2814-f9 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2814-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2814-f11 | Copilot | Suggestion (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The T2 exposure table attributes client exposure to the wrong origin sites

- PR number: 2814
- Source review ID: 5848125288
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4529566733
- Concern: The table mapped InsertFailed and DeleteFailed to UDP, key errors to REST and the whitelist error to REST, and left out the class-C database conversion and P8, P9, P11.
- Solution: Re-traced every row from its producers to the formatting line and added the missing rows.
- Current-tree verification: Read the T2 table against persisted.rs, adapters/auth_key.rs, error_mapping.rs and torrent_metrics_store.rs.
- Resolution reference: docs(issues): correct the T2 exposure trace and record the experiments for #2804
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531015171

### F2 - Closures are not marked and the P14 note contradicts E3

- PR number: 2814
- Source review ID: 5848125288
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4529566745
- Concern: Closure sites were not marked and E8 was cited for a synchronous site.
- Solution: Added a Shape column and corrected the citations.
- Current-tree verification: Read the Shape column and the last section.
- Resolution reference: docs(issues): correct the T2 exposure trace and record the experiments for #2804
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531015658

### F3 - The experiments are recorded without program, commands or output

- PR number: 2814
- Source review ID: 5848125288
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4529566750
- Concern: Only the results of E1 to E12 were recorded.
- Solution: Added experiments.md with programs, commands, toolchain and output, and fixed the E9 citation.
- Current-tree verification: Re-ran both programs and compared the output.
- Resolution reference: docs(issues): correct the T2 exposure trace and record the experiments for #2804
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531016115

### F4 - The dependency-tree standalone crates are outside the scan

- PR number: 2814
- Source review ID: 5848125288
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4529566756
- Concern: The scan covered workspace directories only.
- Solution: Scanned the eight registry crates and recorded the result.
- Current-tree verification: Ran grep over each unpacked crate.
- Resolution reference: docs(issues): correct the T2 exposure trace and record the experiments for #2804
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531016709

### F5 - Class W includes or in a helper, but the helper sites are class D

- PR number: 2814
- Source review ID: 5848125288
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4529566762
- Concern: The class definitions overlapped.
- Solution: Removed the helper clause from class W.
- Current-tree verification: Read the Classification section.
- Resolution reference: docs(issues): correct the T2 exposure trace and record the experiments for #2804
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531017130

### F6 - The introduction misstates this PR's review state and which PR is carried

- PR number: 2814
- Source review ID: 5849963483
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4530936821
- Concern: The retrospective said PR 2814 had no findings, which was false when it was committed, and made PR 2805 the subject of the carried sentence.
- Solution: Rewrote the introduction.
- Current-tree verification: Read the first paragraph after the edit.
- Resolution reference: docs(pr-reviews): correct the intro and figures of the PR #2805 retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531017916

### F7 - The file cites the wrong template and has no Purpose section

- PR number: 2814
- Source review ID: 5849963483
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4530936829
- Concern: The text cited the audit template for the location rule and lacked the Purpose section.
- Solution: Cited PR-REVIEW-RETROSPECTIVE.md and added a Purpose section.
- Current-tree verification: Read the frontmatter and the Purpose section.
- Resolution reference: docs(pr-reviews): correct the intro and figures of the PR #2805 retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531018417

### F8 - Three summary figures do not recompute from the sources

- PR number: 2814
- Source review ID: 5849963483
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4530936836
- Concern: The audit-only commit count, the audit-record finding count, the review count source and the first review time were wrong or without a source.
- Solution: Recomputed each figure and named its source.
- Current-tree verification: Ran gh api for reviews and commits and git show --stat.
- Resolution reference: docs(pr-reviews): correct the intro and figures of the PR #2805 retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531018894

### F9 - rounds 1, 2, 4 and 5 leaves out round 3 yet the list includes F20

- PR number: 2814
- Source review ID: 5849963483
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4530936843
- Concern: The round list was incomplete.
- Solution: Said all five rounds and named the finding of each.
- Current-tree verification: Read the Root Causes list.
- Resolution reference: docs(pr-reviews): correct the intro and figures of the PR #2805 retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531019411

### F10 - The description's Files list does not include this file

- PR number: 2814
- Source review ID: 5849963483
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4530936850
- Concern: The PR description omitted the retrospective.
- Solution: Edited the PR description to list all six files and say why the retrospective is carried here.
- Current-tree verification: Read the PR body after the edit.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531019787
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531019787

### F11 - Tables use a double leading pipe

- PR number: 2814
- Source review ID: 5848148251
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4529583761
- Concern: The reviewer saw rows starting with two pipes.
- Solution: No change: the tables use a single leading pipe.
- Current-tree verification: grep -n '^||' finds nothing in the inventory; prettier and markdownlint pass.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531017591
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2814#discussion_r4531017591

## Processing Log

- 2026-11-10 21:55 UTC - Started audit for review rounds 1 and 2.
- 2026-11-10 21:55 UTC - Committed the audit record; validate-audit-record.py reported 11 rows and 0 failures.
- 2026-11-10 21:56 UTC - Resolved the eleven threads; a GraphQL refresh showed 0 unresolved threads.

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
