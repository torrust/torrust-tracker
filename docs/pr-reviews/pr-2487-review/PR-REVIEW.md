---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261007-ai-model-provenance-in-commits/README.md
---

<!-- skill-link: process-pr-review -->

# PR #2487 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2487>.

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

Copilot review 5455109937 (10:20 UTC) supplied no finding IDs, so its two inline comments take `F1`
and `F2` in source order. Their severities are inferred from the review's badges, as in the PR #2461
audit: Medium as Minor, and Low as Suggestion for the "Consider linking" comment. The review body
lists the two threads and a warning that the full agentic review did not start before its timeout;
neither adds a request, so they get no row.

Human review 5455373856 by `da2ce7` (`APPROVED`, round 1, 10:45 UTC) supplied the finding ID `F1`
with a non-blocking `[Minor]` severity. `F1` collides with this audit, so it takes `F3`. The review
body summarizes that thread and lists the rest under "Checked, no finding", so it adds no other row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2487-f1` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2487-f2` | Copilot | Suggestion (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2487-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The word-for-word claim omitted the added attribution lines

- PR number: 2487
- Source review ID: 5455109937
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2487#discussion_r4217731437>
- Concern: The PR description and the Context said each topic kept the open question's words, but
  each topic also appends "Added by Jose Celano.", so the claim was inaccurate.
- Solution: Kept the attribution, which the discussion template asks for, and corrected the Context
  sentence to say that each topic gained the template's "Added by" attribution. The PR description
  was edited to say the same and that the word-for-word check ran after removing that line.
- Current-tree verification: the README's :34-36 end "each topic gained the template's \"Added by\"
  attribution."; `linter markdown` and `linter lychee` exit 0; the PR description contains "after
  removing the added".
- Resolution reference: docs(discussions): state that each provenance topic gained an attribution line
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2487#discussion_r4217755116>

### F2 - Link the first mention of PR #2467

- PR number: 2487
- Source review ID: 5455109937
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2487#discussion_r4217731528>
- Concern: The Context's first mention of PR #2467 was not a link, although a later sentence links
  it.
- Solution: Linked the first mention and reflowed the paragraph.
- Current-tree verification: the README's :31 reads "This discussion was opened in
  [PR #2467](https://github.com/torrust/torrust-tracker/pull/2467) with"; `linter markdown` and
  `linter lychee` exit 0.
- Resolution reference: docs(discussions): link the first mention of PR #2467 in the provenance Context
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2487#discussion_r4217755375>

### F3 - The Outcome placeholder changed without being stated

- PR number: 2487
- Source review ID: 5455373856
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2487#discussion_r4217949739>
- Concern: The Outcome placeholder changed from "Pending review." to the template's "Pending: no
  decision recorded." in a round that is not a decision round, and none of the round's statements
  of its changes (the Context paragraph, the first commit body, the PR body) mentioned it. The
  reviewer did not block, because adopting the template brings its placeholder, and suggested a
  line in the PR body, which becomes the merge commit message.
- Solution: Kept the placeholder and added the suggested line to the PR body's Changes list. The
  README is unchanged.
- Current-tree verification: the PR body contains "The Outcome placeholder now reads as the
  template's"; the README's Outcome still reads "Pending: no decision recorded.".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2487#issuecomment-6059620329>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2487#discussion_r4218767626>

## Processing Log

- 2026-10-08 10:21 UTC - Started audit. Fetched the two review threads with GraphQL (unresolved,
  not outdated) and the review body; normalized them into F1 and F2.
- 2026-10-08 10:23 UTC - Pushed both fixes after the pre-commit gate passed, edited the PR
  description for F1, re-checked both claims against the pushed tree, and replied on both threads.
- 2026-10-08 10:34 UTC - Copied the Status Values block verbatim from the template, the same fix
  that `review-finding:pr-2485-f2` required on the PR #2485 audit.
- 2026-10-08 12:15 UTC - Human review 5455373856 (`da2ce7`, round 1) approved at the head that
  carried the verbatim fix and raised F3 (reviewer F1). Fixed F3 by editing the PR body, posted
  the resolution comment, and replied on the thread. Then rebased onto `develop`, which had moved
  six commits, so the merge tool would accept the merge; the push moves the head off the approved
  commit, so the reviewer was asked to re-ACK the new head.

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
