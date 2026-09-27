---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2348 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2348>.

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

## Findings

Audit IDs `F1`-`F3` are the three Copilot review 5325942058 threads, which carry no finding IDs.
Copilot rated F1 "High" and F2-F3 "Medium", which are not in the severity vocabulary, so they are
recorded as `Major (inferred)` and `Minor (inferred)`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2348-f1` | Copilot | Major (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2348-f2` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2348-f3` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - EPIC #2003 `last-updated-utc` is date-only

- PR number: 2348
- Source review ID: 5325942058
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2348#discussion_r4111401431>
- Concern: the EPIC frontmatter requires `YYYY-MM-DD HH:MM`, but the changed value is date-only.
- Solution: the value is now `"2026-09-26 13:13"`. It was already date-only (`2026-09-25`) before
  this PR, which only bumped the date; touching the field brings it to the required format.
- Current-tree verification: `git grep -n '^last-updated-utc: "2026-09-26 13:13"' HEAD -- docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`
  matches line 8.
- Resolution reference: `docs(issues): use the UTC minute format for EPIC #2003 last-updated`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2348#discussion_r4111476325>

### F2 - The close-out resolves `FOLLOW_UP` threads before their owning fix merges

- PR number: 2348
- Source review ID: 5325942058
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2348#discussion_r4111401442>
- Concern: the Scope close-out bullet and T7 resolved every thread after this task's PR merges,
  although findings sent to separate issues must stay open until their own fix merges; Copilot
  noted the same problem at the T7 row.
- Solution: the close-out now covers only findings this task fixed or declined. Each `FOLLOW_UP`
  thread stays open with a reply naming its owning issue until that fix merges, and that issue
  then updates the audit and resolves the thread. The Scope bullet, T7, AC4, and M2 say this
  consistently.
- Current-tree verification: `git grep -n "keeps its thread open\|stays open with a reply naming\|Only \`FOLLOW_UP\` threads are unresolved" HEAD -- docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md`
  matches lines 122, 167, and 231; AC4 at line 209 carries the same condition.
- Resolution reference: `docs(issues): keep #2347 follow-up threads open and time approval for new audits`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2348#discussion_r4111476396>

### F3 - AC2 requires the approval in audits that do not exist yet

- PR number: 2348
- Source review ID: 5325942058
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2348#discussion_r4111401449>
- Concern: the #2290 and #2293 audits are created in T4, so their Ownership sections cannot hold
  the approval URL before the first repository change that creates them.
- Solution: AC2 now requires the approval before any fix or thread resolution, recorded in the
  existing audits' Ownership sections when their rows are added and in the #2290 and #2293 audits
  when they are created, matching the no-audit rule added to `process-pr-review` in PR #2344.
- Current-tree verification: `git grep -n "when those audits are created" HEAD -- docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md`
  matches AC2 at line 207.
- Resolution reference: `docs(issues): keep #2347 follow-up threads open and time approval for new audits`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2348#discussion_r4111476485>

## Processing Log

- 2026-09-26 13:17 UTC - Started audit for round 1: Copilot review 5325942058, three inline
  threads and a summary with no further request. Committed the F1 fix and one commit for F2 and
  F3, pushed, re-derived each fix from the pushed tree, and replied on all three threads.
- 2026-09-26 13:26 UTC - After `docs(pr-reviews): add PR #2348 review audit` was pushed,
  `reply-status --login josecelano` reported 3 of 3 threads replied, and the three threads were
  resolved. A refreshed GraphQL fetch reports 3 threads, 0 unresolved.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
