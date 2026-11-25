---
pr-number: 3048
pr-url: https://github.com/torrust/torrust-tracker/pull/3048
last-updated-utc: "2026-11-25 12:50"
---

<!-- skill-link: process-pr-review -->

# PR #3048 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3048.

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

Round 1, on the head `chore(issues): archive closed issue #3031 spec to docs/issues/closed` (first commit): da2ce7 review 6057833343 (approved, two inline threads and one finding in the review body about the title). The `claude[bot]` reviews (6057334420, automated review disabled for forks; 6057711042, no issues found) and the CodeRabbit comment (rate limited) hold no request and have no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3048-f1 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3048-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3048-f3 | Human | Nit | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - Six relative links in the moved spec point at a missing folder

- PR number: 3048
- Source review ID: 6057833343
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3048#discussion_r4695065484
- Concern: Lines 34, 36, 37, 285, 286 and 287 of the moved `ISSUE.md` used `../2387-review-tracker-lifecycle-logs/`, which from `docs/issues/closed/` is a folder that does not exist, because the EPIC stays in `docs/issues/open/`; the link checker excludes `closed/`.
- Solution: The six links start with `../../open/2387-review-tracker-lifecycle-logs/`.
- Current-tree verification: from the moved folder, every distinct `](../...` target of `ISSUE.md` was tested with `[ -e ... ]` and the three targets (`EPIC.md`, `decisions.md`, `baseline-lifecycle-logs.md`) exist.
- Resolution reference: docs(issues): [#3031] point the archived spec at the parent EPIC in open/
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3048#discussion_r4695128697

### F2 - The archive log entry omits two ticked acceptance criteria

- PR number: 3048
- Source review ID: 6057833343
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3048#discussion_r4695065492
- Concern: The entry said it ticked "the checkpoints that the merge and the checks support", but the change also ticked the acceptance criteria "`linter all` exits with code `0`" and "Relevant tests pass".
- Solution: The entry names the two acceptance criteria.
- Current-tree verification: `grep -n "Relevant tests pass" docs/issues/closed/3031-2387-si-1-statistics-disabled-warning-per-event/ISSUE.md` shows the criterion and the archive entry, which now names it.
- Resolution reference: docs(issues): [#3031] point the archived spec at the parent EPIC in open/
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3048#discussion_r4695129175

### F3 - The title's `[#3031]` does not match the archive skill

- PR number: 3048
- Source review ID: 6057833343
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3048#pullrequestreview-6057833343
- Concern: `open-pull-request` keeps the `[#N]` reference for an issue the pull request implements or closes, and the archive skill's title has none, so the merge subject should match the commit subject.
- Solution: The title of the pull request was edited to `chore(issues): archive closed issue #3031 spec to docs/issues/closed`, the commit subject. This is outside the repository tree, so the resolution reference is the conversation response.
- Current-tree verification: `gh-bot pr view 3048 --json title` and the `renamed` event of the PR timeline (2026-11-25 12:45:40 UTC) show the new title.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3048#issuecomment-7162914983
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3048#issuecomment-7162914983

## Processing Log

- 2026-11-25 12:44 UTC - Round 1: da2ce7 review 6057833343 (12:44:40) with two inline threads and one body finding. Rows F1 to F3. The audit record was created after the first fixes and replies; this is the late start.
- 2026-11-25 12:45 UTC - Edited the pull request title (F3; timeline 12:45:40) and committed the fixes of F1 and F2 (`docs(issues): [#3031] point the archived spec at the parent EPIC in open/`, 12:45:45), then pushed.
- 2026-11-25 12:49 UTC - Replied on the two threads after the push (12:49:33 and 12:49:35), resolved them, and posted the conversation response for F3 (12:49:48).
- 2026-11-25 12:50 UTC - Created this audit record after the replies and resolutions; the validation script ran after this entry.

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
