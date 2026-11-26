---
pr-number: 3077
pr-url: https://github.com/torrust/torrust-tracker/pull/3077
last-updated-utc: "2026-11-26 17:56"
---

<!-- skill-link: process-pr-review -->

# PR #3077 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3077.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (the findings were fixed before the merge).

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Reviewer: the author's GitHub login, for example `da2ce7` or `coderabbitai[bot]`; `unknown`
  when GitHub shows no author
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

Round 1, on the first head of the pull request (`a05870195`): da2ce7's review 6078022494
(`APPROVED`) with four inline threads (F1 to F4). Its body lists the same four findings, so it adds
no row.

Round 2, on head `dbe1c213a`: CodeRabbit review 6078161355 with one inline thread (F5).
Round 3, on head `73975031a`: da2ce7's review 6078616888 (`APPROVED`) with two inline threads (F6,
F7). Its body lists the same two findings, so it adds no row.

Commit ids in this record name the heads the reviews saw. The branch is merged without a rebase,
so they stay reachable from `develop`; every fix in the tree is cited by its commit subject, and
the description edits (F3, F6) by their reply URL.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3077-f1 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3077-f2 | da2ce7 | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3077-f3 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3077-f4 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3077-f5 | coderabbitai[bot] | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3077-f6 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3077-f7 | da2ce7 | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A ticked checkpoint claimed a reviewer validation that did not happen

- PR number: 3077
- Source review ID: 6078022494
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711457944
- Concern: The spec ticked "Reviewer validated acceptance criteria and updated checkboxes", and its
  progress log called da2ce7's review the reviewer validation, but the author ticked the acceptance
  criteria in PR #3044 and the ACK on its final head names what it verified, not the acceptance
  criteria.
- Solution: The checkpoint is unticked with the reason, as in the spec of #2618, and the progress
  log entry says why it stays open. The sweep the skill asks for found the committer checkpoint
  ticked although no Committer agent was used; it is unticked with its reason in the same commit.
- Current-tree verification: `grep -n 'Reviewer validated\|Committer verified'` on the spec shows
  both checkpoints as `- [ ]` with a reason in parentheses.
- Resolution reference: docs(issues): [#3037] leave the reviewer and committer checkpoints open with the reason
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711531588

### F2 - The evidence file stamp was not refreshed

- PR number: 3077
- Source review ID: 6078022494
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711457964
- Concern: The archive move changed the evidence file's frontmatter, but its `last-updated-utc`
  still read `2026-11-24 18:41`; Step 4 of `cleanup-completed-issues` says to refresh it.
- Solution: The stamp now reads `2026-11-26 17:04`, taken from `date -u` when the fix was written.
- Current-tree verification: line 4 of
  `docs/issues/closed/3037-2003-reviewer-neutral-pr-review/manual-verification-evidence.md` reads
  `last-updated-utc: "2026-11-26 17:04"`.
- Resolution reference: docs(issues): [#3037] refresh the evidence file stamp after the archive move
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711532128

### F3 - The description had no issue link and no issue-state check

- PR number: 3077
- Source review ID: 6078022494
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711457974
- Concern: The description mentioned #3037 without a `Related to #3037` line and did not say the
  issue's state was checked, as Step 7 of `cleanup-completed-issues` and `open-pull-request` ask.
- Solution: The description now starts with `Related to #3037` and says the issue was already
  closed when PR #3044 merged, checked with `gh issue view 3037`.
- Current-tree verification: `gh-bot pr view 3077 --json body` shows the new first line;
  `gh-bot issue view 3037 --json state,closedAt` prints `CLOSED 2026-11-25T12:25:10Z`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711532676
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711532676

### F4 - The test design review had no frontmatter

- PR number: 3077
- Source review ID: 6078022494
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711457984
- Concern: Step 3 of `cleanup-completed-issues` says a supplementary document without frontmatter
  gets a minimal block linking back to the spec, and `test-design-review.md` moved without one.
- Solution: The file now has `spec-path`, `last-updated-utc` and `semantic-links` back to the
  spec. The older archives the reviewer named are left as they are.
- Current-tree verification: `head -7` of
  `docs/issues/closed/3037-2003-reviewer-neutral-pr-review/test-design-review.md` shows the block,
  and the frontmatter validator of the pre-commit checks passed on it.
- Resolution reference: docs(issues): [#3037] add frontmatter to the test design review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711533091

### F5 - The ADR linked the issue spec by its path

- PR number: 3077
- Source review ID: 6078161355
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711558645
- Concern: The ADR listed the spec path in its `related-artifacts` and again under References, so
  each move of the spec changes two entries. The semantic-link convention asks a long-lived
  artifact to use `"issue #NNNN"` and not to repeat `related-artifacts` under References.
- Solution: The frontmatter lists `"issue #3037"`, and the References entry with the spec path is
  removed.
- Current-tree verification: `rg -n 3037-2003` on
  `docs/adrs/20261124182709_review_pull_requests_without_a_mandatory_reviewer.md` prints nothing,
  and its frontmatter lists `- "issue #3037"`.
- Resolution reference: docs(adrs): [#3037] link the issue by number instead of its spec path
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711685354

### F6 - The description no longer matched the branch

- PR number: 3077
- Source review ID: 6078616888
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711905629
- Concern: After the F1 and F5 fixes the description still said "the last checkpoints ticked" and
  that the ADR's spec path was updated, while two checkpoints are now open with their reasons and
  the ADR links `"issue #3037"`. The merge commit copies the description.
- Solution: The description now says which checkpoint is ticked and which stay open, that the ADR
  links the issue by number, and names the evidence-file stamp, the test-design-review frontmatter
  and this audit record.
- Current-tree verification: `gh-bot pr view 3077 --json body` shows the new bullets.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711956449
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711956449

### F7 - The record cited head commits by id without saying why

- PR number: 3077
- Source review ID: 6078616888
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711905639
- Concern: The record names three heads by id, which the commit-id check of `open-pull-request`
  prints; a review audit should name commits by subject, or say why an id is kept.
- Solution: The Findings introduction now says the ids name the heads the reviews saw, which stay
  reachable from `develop` because the branch is merged without a rebase, and that every fix in
  the tree is cited by its subject.
- Current-tree verification: the sentence follows the round summaries in `## Findings`; every
  `Resolution reference` of a `FIXED` row in the tree is a commit subject or a reply URL.
- Resolution reference: docs(pr-reviews): [#3037] say why the PR 3077 audit names head commits by id
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3077#discussion_r4711956756

## Processing Log

- 2026-11-26 17:03 UTC - Round 1: da2ce7's review 6078022494 (17:03:45, `APPROVED`, on `a05870195`) with four threads.
- 2026-11-26 17:06 UTC - Committed the fixes for F1, F2 and F4 (last commit 17:06:55), pushed them (head `dbe1c213a`) before the replies, and edited the description for F3.
- 2026-11-26 17:11 UTC - Replied on the four threads (r4711531588, r4711532128, r4711532676, r4711533091) and resolved them; created this audit record.
- 2026-11-26 17:13 UTC - Round 2: CodeRabbit review 6078161355 (17:13:38, on `dbe1c213a`) with one thread (F5).
- 2026-11-26 17:18 UTC - Committed the F5 fix (17:18:44), pushed it (head `efcd82c11`), replied (r4711685354) and resolved the thread.
- 2026-11-26 17:49 UTC - Round 3: da2ce7's review 6078616888 (17:49:50, `APPROVED`, on `73975031a`) with two threads (F6, F7).
- 2026-11-26 17:56 UTC - Edited the description for F6, committed the F7 fix (17:51:49) and pushed it (head `d7f7b9bd6`), replied on both threads (r4711956449, r4711956756) and resolved them.

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
