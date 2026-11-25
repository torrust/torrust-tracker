---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
    - "issue #2996"
---

<!-- skill-link: process-pr-review -->

# PR #3057 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/3057>.

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

Review 6061315413 (da2ce7, APPROVED, 2026-11-25 16:38 UTC) posted three inline comments, F1 to F3. Its body lists the same three findings and says none blocks, so it adds no row.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3057-f1 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3057-f2 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3057-f3 | da2ce7 | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3057-f4 | coderabbitai[bot] | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3057-f5 | claude[bot] | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Correct the PR description about the EPIC frontmatter

- PR number: 3057
- Source review ID: 6061315413
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3057#discussion_r4697839941>
- Concern: the description said the EPIC's frontmatter link is updated, but only `last-updated-utc` changed there, and "only the spec's first checkpoint" keeps the old folder path, which two review audits also do.
- Solution: the PR description now says the row and the `last-updated-utc` stamp changed, and that the old path survives only in the spec's first checkpoint and in the historical review audits of PR 3001 and PR 3041.
- Current-tree verification: the description is not a tree file; `gh-bot api repos/torrust/torrust-tracker/pulls/3057 --jq .body` shows the corrected paragraph.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/3057#discussion_r4697928782>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3057#discussion_r4697928782>

### F2 - Correct the finding count in the progress log

- PR number: 3057
- Source review ID: 6061315413
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3057#discussion_r4697839953>
- Concern: the spec's progress log said the reviews fixed 11 findings, but the cited audit records 10 `FIXED` and one `NO_ACTION`.
- Solution: the line now reads "recorded 11 findings (10 fixed, one declined)".
- Current-tree verification: `grep -n "recorded 11 findings" docs/issues/closed/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md` prints the line.
- Resolution reference: docs(issues): [#2996] correct the SI-8 progress log and checkpoint wording after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3057#discussion_r4697929575>

### F3 - Keep the checkpoint template sentence intact

- PR number: 3057
- Source review ID: 6061315413
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3057#discussion_r4697839968>
- Concern: the "not applicable" note sat inside the template sentence of the reviewer-reports checkpoint.
- Solution: the parenthetical moved to the end of the line, after the template wording.
- Current-tree verification: `grep -n "Independent reviewer reports" docs/issues/closed/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md` prints the template sentence followed by the note.
- Resolution reference: docs(issues): [#2996] correct the SI-8 progress log and checkpoint wording after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3057#discussion_r4697930322>

### F4 - Do not claim that no reviewer received the folder

- PR number: 3057
- Source review ID: 6061480325
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3057#discussion_r4697974668>
- Concern: the reviewer-reports checkpoint said no independent reviewer received this folder, while the PR 3041 audit shows da2ce7, Greptile and CodeRabbit commented on its files.
- Solution: the note now says no independent reviewer agent was asked to review the folder, and that the PR 3041 reviews are not independent-reviewer reports and are recorded in the PR 3041 audit, so the claim no longer contradicts it.
- Current-tree verification: `grep -n "Independent reviewer reports" docs/issues/closed/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md` prints the new note.
- Resolution reference: docs(issues): [#2996] state which reviews the SI-8 reviewer-reports checkpoint excludes
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3057#discussion_r4698078783>

### F5 - Add the template's Status Values and Completion Rules sections

- PR number: 3057
- Source review ID: 6061793903
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3057#discussion_r4698245003>
- Concern: the record had no `## Status Values` and no `## Completion Rules`, which `docs/templates/PR-REVIEW-TEMPLATE.md` copies verbatim into each audit record. The same gap, and the `Author class` column of records made before the template named the reviewer by login, is the second Minor point of review 6061765866 (da2ce7), which has no thread and no separate row.
- Solution: the record now carries both sections copied from the template, and its table uses the `Reviewer` column with the source author's login. The F1 resolution reference now names the reply that states the description edit, as the template asks for a fix outside the tree, which is the first Minor point of the same review.
- Current-tree verification: `grep -n "^## " docs/pr-reviews/pr-3057-review/PR-REVIEW.md` lists Ownership, Status Values, Findings, Finding Details, Processing Log and Completion Rules; the validator reports ok.
- Resolution reference: docs(pr-reviews): [#2996] bring the PR 3057 audit in line with the review template
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3057#discussion_r4698329245>

## Processing Log

- 2026-11-25 16:42 UTC - Started audit for review 6061315413 (da2ce7); F1 to F3 fixed, replies and thread resolution follow after the push.
- 2026-11-25 16:46 UTC - Replied on the F1 to F3 threads after pushing the fixes and resolved all three; a GraphQL refresh shows no open thread.
- 2026-11-25 16:55 UTC - Review 6061480325 (CodeRabbit) added F4, fixed in a signed commit; reply and thread resolution follow after the push.
- 2026-11-25 16:58 UTC - Replied on the F4 thread after pushing the fix and resolved it; a GraphQL refresh shows no open thread.
- 2026-11-25 17:20 UTC - Review 6061793903 (Claude app) added F5, which also covers the two Minor points in the body of review 6061765866 (da2ce7); fixed in a signed commit. The findings table now names reviewers by login, and the F1 resolution reference names its reply URL; reply and thread resolution follow after the push.
- 2026-11-25 17:25 UTC - Replied on the F5 thread after pushing the fix and resolved it; a GraphQL refresh shows no open thread.

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
