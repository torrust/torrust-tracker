---
pr-number: 3059
pr-url: https://github.com/torrust/torrust-tracker/pull/3059
last-updated-utc: "2026-11-25 17:52"
---

<!-- skill-link: process-pr-review -->

# PR #3059 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3059.

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

<!-- Guidance omitted from an audit record; instantiate the heading and table below. -->

## Findings

Round 1 is review 6061778095 (`claude[bot]`) and review 6061865252 (`da2ce7`, later dismissed by the
push). Round 2 is review 6062023160 (`da2ce7`). F9 and F10 re-raise F5 and F6 in round 2. Round 3 is review 6062267415 (`da2ce7`); F12 re-raises F7.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3059-f1 | claude[bot] | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3059-f2 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3059-f3 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3059-f4 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3059-f5 | da2ce7 | Minor | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F6 | review-finding:pr-3059-f6 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F7 | review-finding:pr-3059-f7 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F8 | review-finding:pr-3059-f8 | da2ce7 | Minor | other | ORIGINAL | FIXED | NON_RESOLVABLE |
| F9 | review-finding:pr-3059-f9 | da2ce7 | Minor | metadata | RE_RAISE_OF:F5 | FIXED | NON_RESOLVABLE |
| F10 | review-finding:pr-3059-f10 | da2ce7 | Nit | metadata | RE_RAISE_OF:F6 | FIXED | NON_RESOLVABLE |
| F11 | review-finding:pr-3059-f11 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3059-f12 | da2ce7 | Minor | documentation | RE_RAISE_OF:F7 | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The in-folder wording does not cover EPIC folders

- PR number: 3059
- Source review ID: 6061778095
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#discussion_r4698231533
- Concern: The new text says the key points at `ISSUE.md`, but EPIC folders carry `EPIC.md`.
- Solution: Both places now say the folder's own spec, `ISSUE.md` or `EPIC.md`.
- Current-tree verification: Read the two paragraphs in the tree; the later rewrite (F2) keeps `ISSUE.md`, or `EPIC.md` for an EPIC.
- Resolution reference: `docs(templates): cover EPIC.md in the in-folder issue-spec clarification`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3059#discussion_r4698326753

### F2 - The skill and the convention use different criteria

- PR number: 3059
- Source review ID: 6061865252
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#discussion_r4698306593
- Concern: The skill draws the line by location, the convention by role; they disagree on the security report.
- Solution: One criterion in both files: a document whose template prescribes an `issue-spec` key that points at the issue's primary spec and is rewritten when it moves keeps the key.
- Current-tree verification: `git grep -n "template prescribes" -- docs/skills .claude/skills/write-markdown-docs` finds one paragraph in each file.
- Resolution reference: `docs(skills): state one criterion for template-prescribed issue-spec keys`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3059#discussion_r4698561309

### F3 - The catalog row and Maintenance Workflow step 2 keep the unqualified rule

- PR number: 3059
- Source review ID: 6061865252
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#discussion_r4698306604
- Concern: Two statements outside the hunk still say to replace every `issue-spec` marker.
- Solution: The catalog row mentions the template-prescribed use; step 2 keeps the key where its template prescribes it and links to the lifecycle section.
- Current-tree verification: Read both lines in the tree.
- Resolution reference: `docs(skills): state one criterion for template-prescribed issue-spec keys`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3059#discussion_r4698561886

### F4 - The wording is narrower than the tree

- PR number: 3059
- Source review ID: 6061865252
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#discussion_r4698306607
- Concern: "sibling `ISSUE.md`" and "(`open/` to `closed/`)" miss EPIC folders, a subfolder document and the `drafts/` to `open/` move.
- Solution: The paragraph names the issue's primary spec (`ISSUE.md`, or `EPIC.md` for an EPIC) and says the key is rewritten when that spec moves. The one subfolder document is not named; the criterion is the primary spec.
- Current-tree verification: Read the paragraph in the tree.
- Resolution reference: `docs(skills): state one criterion for template-prescribed issue-spec keys`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3059#discussion_r4698562387

### F5 - The description says "~80" and omits validation and files

- PR number: 3059
- Source review ID: 6061865252
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#pullrequestreview-6061865252
- Concern: The description gave too low a count of the evidence files and omitted the validation run and the files touched.
- Solution: Edited the PR description: counts 156 and 138, the two files, the validation run.
- Current-tree verification: Counted `issue-spec` keys in the front matter of `docs/issues/**/*.md`: 156 documents, 138 of them `manual-verification-evidence.md`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3059#issuecomment-7169733295
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3059#issuecomment-7169733295

### F6 - The `templates` scope names an area the diff does not touch

- PR number: 3059
- Source review ID: 6061865252
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#pullrequestreview-6061865252
- Concern: The commit and PR scope is `templates`, but the diff changes a skill and the convention.
- Solution: Changed the PR title and the new commit to the `skills` scope; the two earlier commits keep their subjects because rewriting pushed history would invalidate reviews and ACKs.
- Current-tree verification: The PR title and the subject of the newest fix commit read `docs(skills): ...`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3059#issuecomment-7169733295
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3059#issuecomment-7169733295

### F7 - No audit record although a thread was answered

- PR number: 3059
- Source review ID: 6062023160
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#pullrequestreview-6062023160
- Concern: The head carried no `docs/pr-reviews/pr-3059-review/PR-REVIEW.md`.
- Solution: Added this record.
- Current-tree verification: `ls docs/pr-reviews/pr-3059-review` shows `PR-REVIEW.md` at the head that carries this commit.
- Resolution reference: `docs(pr-reviews): [#3059] add the review audit record`
- Follow-up PR URL: N/A
- Reply URL: N/A

### F8 - A reply cites a branch commit id

- PR number: 3059
- Source review ID: 6062023160
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#pullrequestreview-6062023160
- Concern: Reply 4698326753 cited a branch commit id instead of the Conventional Commit subject.
- Solution: Edited the reply to cite the commit subject.
- Current-tree verification: Fetched the reply body with `gh api`; it names `docs(templates): cover EPIC.md in the in-folder issue-spec clarification` and no commit id.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3059#issuecomment-7169733295
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3059#issuecomment-7169733295

### F9 - The description still says "~80" and omits validation and files

- PR number: 3059
- Source review ID: 6062023160
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#pullrequestreview-6062023160
- Concern: Restates F5 against the head after the first fix.
- Solution: Same fix as F5.
- Current-tree verification: Same as F5.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3059#issuecomment-7169733295
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3059#issuecomment-7169733295

### F10 - The `templates` scope is repeated in the new commit

- PR number: 3059
- Source review ID: 6062023160
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#pullrequestreview-6062023160
- Concern: Restates F6: the first fix commit repeated the `templates` scope.
- Solution: Same fix as F6.
- Current-tree verification: Same as F6.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3059#issuecomment-7169733295
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3059#issuecomment-7169733295

### F11 - The template criterion leaves out 18 in-folder documents

- PR number: 3059
- Source review ID: 6062267415
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#discussion_r4698649361
- Concern: The criterion of the F2 fix (a template prescribes the key) drops 18 in-folder documents that carry an `issue-spec` key pointing at their issue's spec, so they fit neither the catalog row nor the exception.
- Solution: Both files, the catalog row and Maintenance Workflow step 2 now cover a document inside the issue's own folder, or one whose template prescribes the key, whose key points at the issue's primary spec and is rewritten when it moves.
- Current-tree verification: Read the four places in the tree after the fix; `git grep -n "inside the issue's" -- docs/skills .claude/skills/write-markdown-docs` finds the paragraph in each file.
- Resolution reference: `docs(skills): extend the issue-spec exception to every in-folder document`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3059#discussion_r4698740473

### F12 - No audit record at this head

- PR number: 3059
- Source review ID: 6062267415
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3059#pullrequestreview-6062267415
- Concern: Restates F7 at head `1e3bbe6de`, which predates the audit record.
- Solution: The audit record was added in the commit that followed; this round extends it.
- Current-tree verification: `ls docs/pr-reviews/pr-3059-review` shows `PR-REVIEW.md` at the current head.
- Resolution reference: `docs(pr-reviews): [#3059] add the review audit record`
- Follow-up PR URL: N/A
- Reply URL: N/A

## Processing Log

- 2026-11-25 17:08 UTC - `claude[bot]` review 6061778095 with inline finding F1 (review 6061692305 at 17:02 was the fork notice only, no finding).
- 2026-11-25 17:14 UTC - `da2ce7` review 6061865252: F2 to F6 recorded.
- 2026-11-25 17:15 UTC - Replied to F1, resolved its thread; fix commit `docs(templates): cover EPIC.md in the in-folder issue-spec clarification`.
- 2026-11-25 17:25 UTC - `da2ce7` review 6062023160: F7, F8, F9, F10 recorded. The audit was started after the first reply; this entry logs that late start.
- 2026-11-25 17:32 UTC - Fix commit `docs(skills): state one criterion for template-prescribed issue-spec keys` pushed.
- 2026-11-25 17:35 UTC - Replied to the threads of F2, F3 and F4 and resolved them; edited the reply of F1 (F8); edited the PR title and description (F5, F6); posted the consolidated response https://github.com/torrust/torrust-tracker/pull/3059#issuecomment-7169733295.
- 2026-11-25 17:42 UTC - `da2ce7` review 6062267415: F11 and F12 recorded.
- 2026-11-25 17:48 UTC - Fix commit `docs(skills): extend the issue-spec exception to every in-folder document` pushed.
- 2026-11-25 17:50 UTC - Replied to and resolved the thread of F11.

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
