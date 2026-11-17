---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2892 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2892.

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

F1 to F4 are in da2ce7's review 5925521300, in that source order, and keep his IDs. It was submitted as `APPROVED`; GitHub dismissed it at 18:53:19 UTC when a merge of `develop` was pushed, so its state is now `DISMISSED`. His later review 5925708366 (`COMMENTED`) raises no new finding.

F5 is the one finding in Copilot's review 5925530913. Its ID F1 collides with F1 above, so it is numbered F5, the next free audit-local ID, and Copilot's ID is kept as the reviewer finding ID.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2892-f1 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2892-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2892-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2892-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2892-f5 | Copilot | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Give F3 of the PR 2886 audit a durable approval reference

- PR number: 2892
- Source review ID: 5925521300
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2892#discussion_r4589982387
- Concern: F3 of PR 2886 entered its audit after the merge, so `process-pr-review` asks for the maintainer's approval as a durable comment URL; the Ownership line said `N/A` and the decision had no reference.
- Solution: A PR #2886 comment records the maintainer's chat decision to merge as is, created by the contributor account on his behalf as in the PR 2870 audit; the Ownership line, F3's resolution reference and the 17:26 log entry of the PR 2886 audit cite it.
- Current-tree verification: The Ownership line of `docs/pr-reviews/pr-2886-review/PR-REVIEW.md`; `validate-audit-record.py --pr-number 2886` reports ok.
- Resolution reference: docs(pr-reviews): [#2866] cite the maintainer's F3 decision in the PR 2886 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2892#discussion_r4590212537

### F2 - Say which merge push dismissed an approval

- PR number: 2892
- Source review ID: 5925521300
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2892#discussion_r4589982397
- Concern: The retrospective said each merge push dismissed the approval; only the last one did. The body of the earlier audit commit says the same more sharply.
- Solution: The retrospective says the last merge push dismissed the standing approval. The commit body is left as is: rewording a pushed commit needs a force push, which this session cannot run, as for F3 of PR 2886.
- Current-tree verification: The "What Changed During Implementation" section of the retrospective.
- Resolution reference: docs(issues): [#2866] say which merge push dismissed an approval
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2892#discussion_r4590213093

### F3 - Stamp the evidence frontmatter edit

- PR number: 2892
- Source review ID: 5925521300
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2892#discussion_r4589982410
- Concern: The archive changed the evidence file's `issue-spec` but not its `last-updated-utc`, against `cleanup-completed-issues` Step 4.
- Solution: `last-updated-utc` carries the time of the edit.
- Current-tree verification: The frontmatter of `manual-verification-evidence.md` in the archived spec folder.
- Resolution reference: docs(issues): [#2866] stamp the evidence frontmatter edit of the archive
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2892#discussion_r4590213601

### F4 - Note that not every moved skill was under dev/

- PR number: 2892
- Source review ID: 5925521300
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2892#discussion_r4589982417
- Concern: The retrospective said the skills moved from `.github/skills/dev/<area>/<name>/`; three were not under `dev/`.
- Solution: The Outcome says the skills moved from `.github/skills/` (mostly `dev/<area>/<name>/`).
- Current-tree verification: The Outcome section of the retrospective.
- Resolution reference: docs(issues): [#2866] note that not every moved skill was under dev/
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2892#discussion_r4590214028

### F5 - Record a no-change finding as superseded

- PR number: 2892
- Source review ID: 5925530913
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2892#discussion_r4589990048
- Concern: F3 of the PR 2886 audit had no change but was recorded `NO_ACTION`/`RESOLVED`; the Completion Rules and `process-pr-review` prescribe `SUPERSEDED`.
- Solution: The F3 row of the PR 2886 audit is `NO_ACTION`/`SUPERSEDED`; the GitHub thread stays resolved.
- Current-tree verification: The F3 row of `docs/pr-reviews/pr-2886-review/PR-REVIEW.md`.
- Resolution reference: docs(pr-reviews): [#2866] record F3 of the PR 2886 audit as superseded
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2892#discussion_r4590214392

## Processing Log

- 2026-11-16 18:46 UTC - Review 5925521300 (da2ce7, `APPROVED`) submitted on the head commit `docs(pr-reviews): [#2866] record the last reviews and the merge of PR #2886`: two Minor and two Nit findings, none blocking, recorded here as F1 to F4.
- 2026-11-16 18:46 UTC - Review 5925530913 (Copilot, `COMMENTED`) submitted on the same head: one Nit, recorded here as F5.
- 2026-11-16 18:53 UTC - Pushed the merge of `develop` that resolved a conflict in the EPIC #2003 spec (merge commit made 18:46:46 UTC); the push dismissed review 5925521300 (timeline event `review_dismissed`, 18:53:19 UTC). An earlier push attempt of the same commit was rejected by a test that failed in the pre-push run and passed when rerun.
- 2026-11-16 18:59 UTC - Review 5925708366 (da2ce7, `COMMENTED`) submitted on the merge head: no new finding; F1 to F5 open.
- 2026-11-16 19:00 UTC - Posted the PR #2886 comment that records the maintainer's F3 decision (19:00:24 UTC), for F1.
- 2026-11-16 19:07 UTC - Pushed the five fix commits (committed 19:00 to 19:03 UTC; push log time 19:07:15 UTC). Replied to the five threads (19:07:28 to 19:07:39 UTC) and resolved them (19:07:52 UTC); a GraphQL refresh shows no unresolved thread.

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
