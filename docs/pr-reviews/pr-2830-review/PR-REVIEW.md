---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2830 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2830.

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

Round 1: da2ce7 review 5859800072 (`F1` to `F5`) and Copilot review 5859882741 (`R1`; the bot's review and comment carry no finding ID, so `R1` is an audit-local reference, kept once assigned). Round 2: da2ce7 review 5860282035 (`F6` to `F8`). Round 3: da2ce7 review 5861952787, whose labels `F6` (a re-raise), `F9` and `F10` are recorded as `F11`, `F9` and `F10`; `F6` was already used, so the re-raise gets the next audit-local ID and the reviewer's label is kept in `Reviewer finding ID`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2830-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2830-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2830-f3 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2830-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2830-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2830-r1 | Copilot | Nit (inferred) | formatting | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F6 | review-finding:pr-2830-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2830-f7 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2830-f8 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2830-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2830-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2830-f11 | Human | Minor | documentation | RE_RAISE_OF:F5 | FIXED | RESOLVED |

## Finding Details

### F1 - The EPIC's Subissues table has no row for #2776.

- PR number: 2830
- Source review ID: 5859800072
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538708384
- Concern: The EPIC's Subissues table has no row for #2776.
- Solution: Row 3 added (DONE, A2); the placeholder is row 4.
- Current-tree verification: `git grep -n "#2776 - Adopt" docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md`.
- Resolution reference: docs(issues): [#2776] address PR 2830 review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538936115

### F2 - Two completed checkpoints stay unticked.

- PR number: 2830
- Source review ID: 5859800072
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538708400
- Concern: Two completed checkpoints stay unticked.
- Solution: Both checkpoints ticked.
- Current-tree verification: `git grep -n "\[x\] Issue closed and spec moved" docs/issues/closed/2776-2411-axum-server-0-9-timeouts/ISSUE.md`.
- Resolution reference: docs(issues): [#2776] address PR 2830 review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538936529

### F3 - The evidence record and the EPIC keep their old `last-updated-utc`.

- PR number: 2830
- Source review ID: 5859800072
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538708403
- Concern: The evidence record and the EPIC keep their old `last-updated-utc`.
- Solution: Both restamped (the EPIC later restamped again, see F7).
- Current-tree verification: `git grep -n last-updated-utc` on both files.
- Resolution reference: docs(issues): [#2776] address PR 2830 review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538936948

### F4 - Four `Current-tree verification` lines of the PR 2813 audit were rewritten, changing a past record.

- PR number: 2830
- Source review ID: 5859800072
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538708413
- Concern: Four `Current-tree verification` lines of the PR 2813 audit were rewritten, changing a past record.
- Solution: The audit is restored to its original content; this PR no longer touches it.
- Current-tree verification: `git diff b8fac7f7f -- docs/pr-reviews/pr-2813-review` prints nothing.
- Resolution reference: docs(issues): [#2776] address PR 2830 review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538937440

### F5 - The title and description depart from the open-pull-request and archive templates.

- PR number: 2830
- Source review ID: 5859800072
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538708422
- Concern: The title and description depart from the open-pull-request and archive templates.
- Solution: Title is the archive template; the description has `Related to #2776`, the closure check and a validation line. The change is on the forge; a PR conversation comment states it and is cited as the resolution.
- Current-tree verification: `gh-bot pr view 2830 --repo torrust/torrust-tracker --json title,body`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2830#issuecomment-6801479863
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538937916

### R1 - Indentation of a line of the 2813 audit looked lost.

- PR number: 2830
- Source review ID: 5859882741
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538774127
- Concern: Indentation of a line of the 2813 audit looked lost.
- Solution: Superseded by the F4 fix: the file is restored byte for byte. The claim itself did not hold, as the first commit changed only the path on those lines.
- Current-tree verification: `git diff b8fac7f7f -- docs/pr-reviews/pr-2813-review` prints nothing.
- Resolution reference: docs(issues): [#2776] address PR 2830 review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4538938352

### F6 - The head has no audit record.

- PR number: 2830
- Source review ID: 5860282035
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4539086459
- Concern: The head has no audit record.
- Solution: This audit is added, with the processing log recording that it came after the first replies. F5 cites the reply URL as its resolution because the change lives on the forge, and R1 is `RE_RAISE_OF:F4` because that fix superseded it.
- Current-tree verification: This file exists.
- Resolution reference: docs(pr-reviews): [#2830] add the PR audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4540274432

### F7 - The `last-updated-utc` of the EPIC and the spec (13:50) is earlier than the edits it dates (committed 14:08).

- PR number: 2830
- Source review ID: 5860282035
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4539086468
- Concern: The `last-updated-utc` of the EPIC and the spec (13:50) is earlier than the edits it dates (committed 14:08).
- Solution: Both restamped with the time of the fix commit.
- Current-tree verification: `git grep -n last-updated-utc` on the EPIC and the spec.
- Resolution reference: docs(issues): [#2776] stamp the EPIC and the spec with the time of the fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4540274915

### F8 - One commit carried four independent fixes.

- PR number: 2830
- Source review ID: 5860282035
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4539086478
- Concern: One commit carried four independent fixes.
- Solution: No rewrite was asked; from this round on each independent fix has its own Conventional commit (the stamps, the audit).
- Current-tree verification: `git log --oneline` of the branch.
- Resolution reference: docs(issues): [#2776] stamp the EPIC and the spec with the time of the fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4540275325

### F9 - `Status Values` and `Completion Rules` are not the template's verbatim text.

- PR number: 2830
- Source review ID: 5861952787
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4540432028
- Concern: `Status Values` and `Completion Rules` are not the template's verbatim text.
- Solution: Both sections are restored from the template. Threads are now resolved by the author after the reply, as `process-pr-review` asks.
- Current-tree verification: `diff` of both sections against `PR-REVIEW-TEMPLATE.md`.
- Resolution reference: docs(pr-reviews): [#2830] restore the template sections and fix the R1 and F5 rows
- Follow-up PR URL: N/A
- Reply URL: PENDING

### F10 - The reason given for `R1` does not hold: the bot's review and comment carry no finding ID.

- PR number: 2830
- Source review ID: 5861952787
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4540432046
- Concern: The reason given for `R1` does not hold: the bot's review and comment carry no finding ID.
- Solution: The reason is corrected in the Findings paragraph and the severity of `R1` is marked `(inferred)`; `R1` keeps its reference.
- Current-tree verification: `git grep -n "audit-local reference" docs/pr-reviews/pr-2830-review/PR-REVIEW.md`.
- Resolution reference: docs(pr-reviews): [#2830] restore the template sections and fix the R1 and F5 rows
- Follow-up PR URL: N/A
- Reply URL: PENDING

### F11 - F5's `Resolution reference` is prose, so the audit validator fails.

- PR number: 2830
- Source review ID: 5861952787
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2830#discussion_r4540432016
- Concern: F5's `Resolution reference` is prose, so the audit validator fails.
- Solution: A PR conversation comment states the title and description change; its URL is the F5 resolution reference.
- Current-tree verification: `git grep -n issuecomment-6801479863 docs/pr-reviews/pr-2830-review/PR-REVIEW.md`.
- Resolution reference: docs(pr-reviews): [#2830] restore the template sections and fix the R1 and F5 rows
- Follow-up PR URL: N/A
- Reply URL: PENDING

## Processing Log

- 2026-11-11 14:13 UTC - Round 1 received (da2ce7 at 13:52, Copilot at 13:58); replies for `F1` to `F5` and `R1` posted at 14:13 UTC after the fix commit `docs(issues): [#2776] address PR 2830 review`. The replies say "Fixed in 615e8d84d"; that id is that commit, mapped here because review comments cannot be edited. This audit was not created before the first replies, as `F6` points out; it is added in round 2.
- 2026-11-11 16:08 UTC - Round 2 review of da2ce7 received at 14:26 UTC (`F6` to `F8`); this audit and the stamp fix are the commits of the round.
- 2026-11-11 16:11 UTC - Round 2 replies for `F6` to `F8` posted after the two fix commits; their reply URLs are recorded in the commit that carries this entry.
- 2026-11-11 16:40 UTC - Round 3 review of da2ce7 received at 16:25 UTC (`F11`, `F9`, `F10`); the fix commit is `docs(pr-reviews): [#2830] restore the template sections and fix the R1 and F5 rows`. From this round the author resolves threads after replying, as the restored template says; all earlier threads are resolved with them.

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
