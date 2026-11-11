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
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. The author does not resolve
  reviewer threads here, so rows stay `OPEN` until the reviewer resolves them.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Round 1: da2ce7 review 5859800072 (`F1` to `F5`) and Copilot review 5859882741 (`R1`; its own label collides with the da2ce7 labels). Round 2: da2ce7 review 5860282035 (`F6` to `F8`).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2830-f1 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F2 | review-finding:pr-2830-f2 | Human | Minor | metadata | ORIGINAL | FIXED | OPEN |
| F3 | review-finding:pr-2830-f3 | Human | Minor | metadata | ORIGINAL | FIXED | OPEN |
| F4 | review-finding:pr-2830-f4 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F5 | review-finding:pr-2830-f5 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| R1 | review-finding:pr-2830-r1 | Copilot | Nit | formatting | RE_RAISE_OF:F4 | FIXED | OPEN |
| F6 | review-finding:pr-2830-f6 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F7 | review-finding:pr-2830-f7 | Human | Minor | metadata | ORIGINAL | FIXED | OPEN |
| F8 | review-finding:pr-2830-f8 | Human | Nit | maintainability | ORIGINAL | FIXED | OPEN |

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
- Solution: Title is the archive template; the description has `Related to #2776`, the closure check and a validation line. The change is on the forge, so the evidence is the reply.
- Current-tree verification: `gh-bot pr view 2830 --repo torrust/torrust-tracker --json title,body`.
- Resolution reference: Forge change (title and description), stated in the reply
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
- Reply URL: PENDING

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
- Reply URL: PENDING

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
- Reply URL: PENDING

## Processing Log

- 2026-11-11 14:13 UTC - Round 1 received (da2ce7 at 13:52, Copilot at 13:58); replies for `F1` to `F5` and `R1` posted at 14:13 UTC after the fix commit `docs(issues): [#2776] address PR 2830 review`. The replies say "Fixed in 615e8d84d"; that id is that commit, mapped here because review comments cannot be edited. This audit was not created before the first replies, as `F6` points out; it is added in round 2.
- 2026-11-11 16:08 UTC - Round 2 review of da2ce7 received at 14:26 UTC (`F6` to `F8`); this audit and the stamp fix are the commits of the round.

## Completion Rules

- Re-derive the reply claim against the current tree before replying.
- Reply on every thread; thread resolution is left to the reviewer.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA.
