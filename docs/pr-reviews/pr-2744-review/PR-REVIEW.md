---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2744 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2744>.

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

Round 1 is the Copilot review `5798210789` (submitted 2026-11-06 09:28 UTC) with one inline comment. It carries no
`[Severity][Id]` prefix, so the finding ID is audit-local and the severity is inferred. All CI checks were green at the
time, so no CI finding exists.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2744-f1 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Say the deadline length when the test deadline helper times out

- PR number: 2744
- Source review ID: 5798210789
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2744#discussion_r4490945560
- Concern: When `within_deadline` times out, its panic message names neither the deadline length nor the timed-out step; the reviewer suggested including `TEST_DEADLINE` and, optionally, a short context label in the signature.
- Solution: The maintainer decided to fix the duration only. The panic now reads `the awaited work finishes within the test deadline of 5s`. The step label was declined: the panic already carries the failing test's name and the line of the call, and the label would change about 25 call sites.
- Current-tree verification: A pending future placed before the container setup in a helper still fails the tests with the new message, clippy passes, and the 167 unit tests pass.
- Resolution reference: test(axum-rest-api-server): [#2637] name the deadline length in the test deadline panic
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2744#discussion_r4491139688

## Processing Log

- 2026-11-06 09:28 UTC - Copilot review `5798210789` submitted with one inline finding. No human review existed at this point.
- 2026-11-06 09:49 UTC - Fixed F1 in one signed commit and pushed the branch (the commit subject, not an id, is cited above), replied to the thread (09:51 UTC) and resolved it (09:51 UTC); the resolve mutation returned `isResolved: true`.

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
