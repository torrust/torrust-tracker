---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2836 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2836.

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

Round 1 is Copilot's review `5862929633` (F1 and F2). Its summary says the full agentic review did not start before a timeout.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2836-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2836-f2 | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The moved-tasks row cites scenarios without naming their specification

- PR number: 2836
- Source review ID: 5862929633
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2836#discussion_r4541234224
- Concern: The row said (M1, M2) and the reviewer read it as the scenarios of the #384 specification, which has M1 to M4.
- Solution: The row names the scenarios as those of the #2804 specification and says where M3 is covered.
- Current-tree verification: Read the manual scenario tables of both specifications; the #2804 one has M1 to M3.
- Resolution reference: docs(issues): fix the scenario reference and a duplicated heading after review of PR #2836
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2836#discussion_r4541284431

### F2 - Duplicated Disposable Verification Scripts heading

- PR number: 2836
- Source review ID: 5862929633
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2836#discussion_r4541234389
- Concern: The heading appeared twice in the #384 specification.
- Solution: Removed the second heading in all seven new specifications, which shared the defect.
- Current-tree verification: Counted the heading in the seven files; each has one.
- Resolution reference: docs(issues): fix the scenario reference and a duplicated heading after review of PR #2836
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2836#discussion_r4541285000

## Processing Log

- 2026-11-11 17:39 UTC - Round 1: fix committed at 17:39:58Z (`git log`), after both findings were checked against the tree.
- 2026-11-11 17:43 UTC - Replied on the two threads at 17:42:57Z and 17:43:00Z (forge `created_at`) and resolved them; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-11 17:43 UTC - Started this audit record, after the replies, because it holds their URLs.

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
