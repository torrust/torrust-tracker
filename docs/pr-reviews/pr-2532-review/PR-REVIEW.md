---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2521-1669-remove-unused-udp-stats-traits/ISSUE.md
    - docs/issues/closed/2517-1669-encapsulate-statistics-repository-state/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2532 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2532>.

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

Review 5521450385 (round 1, `copilot-pull-request-reviewer`, state `COMMENTED`, submitted
2026-10-14 16:48 UTC on the head whose tip commit was
`chore(issues): archive closed issue #2521 spec to docs/issues/closed`,
<https://github.com/torrust/torrust-tracker/pull/2532#pullrequestreview-5521450385>) left no inline comment and reports "0 open findings". Its
"Needs a closer look" sentence is nevertheless an independently actionable assertion, recorded as
`F1` with `Thread state=NON_RESOLVABLE`. The rest of the body summarizes the changes and is not a
further finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2532-f1` | Copilot | Nit (inferred) | link-integrity | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The archived #2517 spec still points at the old open path of the #2521 spec

- PR number: 2532
- Source review ID: 5521450385
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2532#pullrequestreview-5521450385>
- Concern: "One live cross-reference in the archived #2517 specification still points to the removed open path": the Out of Scope note in `docs/issues/closed/2517-1669-encapsulate-statistics-repository-state/ISSUE.md` names `docs/issues/open/2521-1669-remove-unused-udp-stats-traits/ISSUE.md`, which this PR moves.
- Solution: The reference had been retained as a historical record, but it is a navigational "tracked by" pointer, so it now names the closed path; the #2517 spec's `last-updated-utc` and progress log record the change. Outside this audit, the two remaining mentions of the old path are `Current-tree verification` commands in the PR #2529 audit, which record the commands as they were run.
- Current-tree verification: outside this audit, whose Concern quotes the old path and whose verification names the search pattern, `rg -n 'open/2521-1669-remove-unused-udp-stats-traits'` matches only lines 132 and 145 of `docs/pr-reviews/pr-2529-review/PR-REVIEW.md`; line 133 of the #2517 spec names `docs/issues/closed/2521-1669-remove-unused-udp-stats-traits/ISSUE.md`.
- Resolution reference: `docs(issues): [#2521] point the archived #2517 spec at the closed #2521 spec`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2532#issuecomment-6173714194>

## Processing Log

- 2026-10-14 16:48 UTC - Review 5521450385 (`copilot-pull-request-reviewer`, `COMMENTED`) left no
  inline comment; its body flags the #2517 spec reference.
- 2026-10-14 16:55 UTC - Committed the F1 fix at the maintainer's request.
- 2026-10-14 16:58 UTC - Pushed the fix and posted the PR conversation response.
- 2026-10-14 16:58 UTC - Created this audit.

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
