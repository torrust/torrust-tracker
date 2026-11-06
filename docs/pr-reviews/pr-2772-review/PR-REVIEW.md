---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2772 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2772>.

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

Round 1 is the Copilot review `5804546552` (submitted 2026-11-06 17:49 UTC; Copilot's agentic review did not run in full, and its overview lists two open issues, both inline). The two later empty `COMMENTED` reviews are the bot's own thread replies. Neither comment carries a `[Severity][Id]` prefix, so the finding IDs are audit-local and the severities are inferred. All CI checks were green at the time, so no CI finding exists.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2772-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2772-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Stale "proposed" status sentence in the wiring plan

- PR number: 2772
- Source review ID: 5804546552
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4495749132
- Concern: `wiring-modules-tests.md` still said "proposed, waiting for the maintainer's approval" although its frontmatter is `status: done` and its Results section records the approval.
- Solution: The sentence now reads "Status: done; the maintainer approved the plan on 2026-11-06 (no test is added)". The other S6 plans were checked for the same sentence and have none (their append-only log lines keep the original wording).
- Current-tree verification: `grep` of the three S6 plans for "proposed" and "waiting" finds only append-only progress-log lines; `linter markdown` passes.
- Resolution reference: docs(issues): [#2637] align two S6 plans with their final state
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4496391545

### F2 - Label names of the duplicate-token plan differ from the test

- PR number: 2772
- Source review ID: 5804546552
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4495749212
- Concern: `v1-middlewares-auth-tests.md` named the labels `first` and `second`, but the test uses `first`, `copy` and `other`.
- Solution: The plan table now names `first` and `copy` (the shared value) and `other`, as implemented. The test was not changed.
- Current-tree verification: the labels in the plan match the `AccessTokens::from` fixture of `it_should_authenticate_a_token_configured_under_more_than_one_label` in `auth.rs`.
- Resolution reference: docs(issues): [#2637] align two S6 plans with their final state
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4496392284

## Processing Log

- 2026-11-06 17:49 UTC - Copilot review `5804546552` submitted with two inline findings on the PR opened at 16:59 UTC. No human review existed at this point.
- 2026-11-06 18:57 UTC - Fixed F1 and F2 in one signed commit and pushed the branch (the commit subject, not an id, is cited above), replied to both threads and resolved them; both resolve mutations returned `isResolved: true`, and a refresh found no unresolved thread.

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
