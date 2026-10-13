---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261007-ai-model-provenance-in-commits/README.md
---

<!-- skill-link: process-pr-review -->

# PR #2502 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2502>.

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

Copilot review 5503111347 left two inline findings without severity brackets, so their severities
are inferred. Its overview adds no other request, and its suppressed comments are not retrievable,
so they add no row. Cameron Garnham's review 5503154527 approved the head `99107d94f` and left one
inline finding headed `[Suggestion][F1]`; that ID collides with audit `F1`, so it is recorded as
audit `F3`. His ACK comment on the PR conversation contains no request and adds no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2502-f1` | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2502-f2` | Copilot | Nit (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2502-f3` | Human | Suggestion | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The PR description said the headings had no PR number yet

- PR number: 2502
- Source review ID: 5503111347
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2502#discussion_r4258297586>
- Concern: The PR description said that the new entry headings carry no PR number yet and that a
  follow-up commit would add it, while the headings at the reviewed head already include
  `PR #2502`.
- Solution: The description had already been corrected at 11:41:45 UTC, before the review was
  submitted at 11:56:35 UTC. It now reads: "A second commit adds `PR #2502` to the three entry
  headings once the PR number was known, as the template asks." No change to the tree.
- Current-tree verification: `gh pr view 2502 --json body` shows the corrected sentence and no
  "no PR number yet" text; the GraphQL `lastEditedAt` of the description is 2026-10-13T11:41:45Z;
  the three `##### Q<n> - Jose Celano` headings in the README end with `, PR #2502`.
- Resolution reference: pending
- Follow-up PR URL: N/A
- Reply URL: pending

### F2 - The issue #2473 reference in the Q12 entry is not linked

- PR number: 2502
- Source review ID: 5503111347
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2502#discussion_r4258297823>
- Concern: `issue #2473` in the Q12 entry is plain text; Copilot suggested a link so that readers
  can navigate to it directly.
- Solution: Linked as `[issue #2473](https://github.com/torrust/torrust-tracker/issues/2473)` in the
  same edit as F3, which rewords that sentence.
- Current-tree verification: the Q12 entry contains the link; `linter all` passes in the
  pre-commit gate.
- Resolution reference: `docs(discussions): [#2003] name the rounds convention and link issue #2473 in my Q12 entry`
- Follow-up PR URL: N/A
- Reply URL: pending

### F3 - Name the rounds convention as what issue #2473 introduced

- PR number: 2502
- Source review ID: 5503154527
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2502#discussion_r4258330332>
- Concern: `docs/discussions/AGENTS.md` predates issue #2473 (added on 2026-10-03 with a single-PR
  lifecycle); #2473 introduced the rounds convention in it. The reviewer noted that the claim holds
  when read with the sentence's subject, so it is not a merge-gate finding, and suggested "the
  rounds convention in `docs/discussions/AGENTS.md`, introduced by issue #2473".
- Solution: The Q12 entry now says that a change to the round rules belongs to "the rounds
  convention in `docs/discussions/AGENTS.md`, introduced by issue #2473", in the reviewer's
  wording, with the link from F2.
- Current-tree verification: `git grep -n 'rounds convention in' -- docs/discussions` finds the
  sentence in the Q12 entry; the diff of the fix commit changes only that sentence.
- Resolution reference: `docs(discussions): [#2003] name the rounds convention and link issue #2473 in my Q12 entry`
- Follow-up PR URL: N/A
- Reply URL: pending

## Processing Log

- 2026-10-13 11:41 UTC - Corrected the PR description sentence about the PR number in the entry
  headings, after the second commit added `PR #2502`.
- 2026-10-13 11:56 UTC - Copilot review 5503111347 submitted with two inline findings, recorded as
  F1 and F2.
- 2026-10-13 11:59 UTC - Cameron Garnham's review 5503154527 approved head `99107d94f` with one
  inline suggestion, recorded as F3; his ACK comment followed at 11:59:55 UTC.
- 2026-10-13 12:17 UTC - Rebased the branch onto `develop` `c7aeaf46c` without conflicts, fixed F2
  and F3 in one commit, and started this audit.

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
