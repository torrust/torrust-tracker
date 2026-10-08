---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2473-2003-asynchronous-discussion-rounds/ISSUE.md
    - docs/discussions/AGENTS.md
---

<!-- skill-link: process-pr-review -->

# PR #2481 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2481>.

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
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`,
  `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Copilot review 5453016407 (07:13 UTC) supplied no finding ID, so its one inline comment takes `F1`.
Its severity is inferred from the review's "Medium severity" badge as Minor, as in the PR #2461
audit. The review body also notes that the two rules beyond the specification still need the EPIC
owner's confirmation; the PR body already asks him for it, and the note requests no change, so it
gets no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2481-f1` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The no-agreement merge gate also covered decision rounds

- PR number: 2481
- Source review ID: 5453016407
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2481#discussion_r4216040581>
- Concern: The merge gate's closed list of blocking reasons applied to every round, including
  decision rounds, so reviewers could not withhold agreement on an Outcome. That contradicted
  "Agreement is needed only here" in the Decision Round section.
- Solution: Scoped the gate to opening and contribution rounds, stated that a decision round passes
  the same document checks and also needs its reviewers' agreement, and said in the Decision Round
  section that reviewers may block it for disagreeing with the decision. The heading is unchanged
  because the specification's acceptance evidence cites it.
- Current-tree verification: `docs/discussions/AGENTS.md` :67-68 begin "An opening or
  contribution round merges", :76-77 end "also needs its reviewers' agreement.", and :81-82 begin
  "Agreement is needed only here: reviewers may block a decision round"; `linter markdown` and
  `linter cspell` exit 0.
- Resolution reference: docs(discussions): [#2473] limit the no-agreement merge gate to opening and contribution rounds
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2481#discussion_r4216062379>

## Processing Log

- 2026-10-08 07:14 UTC - Started audit. Fetched the one review thread with GraphQL (unresolved,
  not outdated) and the review body; normalized them into F1.
- 2026-10-08 07:15 UTC - Pushed the F1 fix after the pre-commit gate passed, re-checked the claim
  against the pushed tree, and replied on the thread.

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
