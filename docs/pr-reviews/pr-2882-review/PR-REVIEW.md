---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/pr-reviews/pr-2870-review/PR-REVIEW.md
---

<!-- skill-link: process-pr-review -->

# PR #2882 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2882.

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

The da2ce7 review numbers its findings `F1` to `F3`, which are also their audit-local IDs. The
Copilot finding has no reviewer ID and gets the audit-local ID `F4`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2882-f1 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2882-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2882-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2882-f4 | Copilot | Suggestion (inferred) | link-integrity | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Drop the closed issue from the title

- PR number: 2882
- Source review ID: 5921518992
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2882#discussion_r4586903101
- Concern: The title's `[#2845]` names an issue this PR neither implements nor closes; the `open-pull-request` skill keeps a related or already-closed issue in the body.
- Solution: Retitled the PR to `docs(pr-reviews): mark C13 of the PR 2870 audit fixed`; the description still names #2845.
- Current-tree verification: The PR title has no issue reference.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2882#discussion_r4586949924
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2882#discussion_r4586949924

### F2 - State the validation in the description

- PR number: 2882
- Source review ID: 5921518992
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2882#discussion_r4586903117
- Concern: The description named no validator run, linter or hook.
- Solution: Ran `validate-audit-record.py` for PR 2870 and PR 2882 as `josecelano-bot`, and added a Validation section with those results, the pre-commit hook and the commit-id check.
- Current-tree verification: The PR description has a Validation section.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2882#discussion_r4586950479
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2882#discussion_r4586950479

### F3 - Name a commit-id check run that can fail

- PR number: 2882
- Source review ID: 5921518992
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2882#discussion_r4586903134
- Concern: C13's verification named a commit-id check run on `develop`, where the diff is empty and the check scans no file.
- Solution: C13's verification now names the run at this PR's head, which scans the PR 2870 audit.
- Current-tree verification: `docs/pr-reviews/pr-2870-review/PR-REVIEW.md` C13 names the run at the head of PR #2882.
- Resolution reference: docs(pr-reviews): address the review of PR 2882 (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2882#discussion_r4586950973

### F4 - Keep the commit subject as C13's resolution reference

- PR number: 2882
- Source review ID: 5921545601
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2882#discussion_r4586922421
- Concern: Copilot suggested adding a commit or merge URL next to C13's commit-subject resolution reference.
- Solution: No change. The `process-pr-review` skill requires a `FIXED` resolution reference to be a unique Conventional Commit subject and forbids a SHA, and the validator checks that subject against the follow-up PR's commits. C13's Follow-up PR URL already links PR #2880.
- Current-tree verification: The subject is the only commit with it, in PR #2880, and the validator finds it.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2882#discussion_r4586951391
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2882#discussion_r4586951391

## Processing Log

- 2026-11-16 14:20 UTC - da2ce7 review 5921518992 (approved) submitted with three inline findings, none blocking, and an ACK of the head.
- 2026-11-16 14:22 UTC - Copilot review 5921545601 submitted with one inline finding.
- 2026-11-16 14:23 UTC - Retitled the PR (F1) and added a Validation section to the description (F2).
- 2026-11-16 14:24 UTC - Replied on the four threads.
- 2026-11-16 14:25 UTC - Committed the F3 fix and this record (`docs(pr-reviews): address the review of PR 2882 (#2845)`).

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
