---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261007-ai-model-provenance-in-commits/README.md
---

<!-- skill-link: process-pr-review -->

# PR #2467 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2467>.

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

Human review 5440113493 by `da2ce7` (`CHANGES_REQUESTED`, 09:10 UTC) supplied the finding IDs
`F1` to `F4` with bracketed severities; all four are kept. Its body summarizes the same four
threads. Its other remarks, including that the PR body links the EPIC as "Related to #2003" rather
than the `Refs` form, are listed under "Checked, no finding" or carry no request, so they get no
row; "Related to #N" is the form `AGENTS.md` prescribes for a related issue.

Copilot review 5440144697 (09:13 UTC) supplied the finding IDs `F1` and `F2`, which collide with
this audit, so they take `F5` and `F6` in source order. F5 asks for the same frontmatter change as
F1 and is a re-raise. The review body summarizes the two threads and adds no other assertion.

Cameron's separate PR conversation comment
<https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101> is a draft position
on the discussion's open questions, not a review of the document, so it gets no row. The maintainer
chose not to transcribe it in this PR; the response
<https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6041029813> invites it as its
author's own round after merge.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2467-f1` | Human | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2467-f2` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2467-f3` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2467-f4` | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2467-f5` | Copilot | Minor | metadata | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F6 | `review-finding:pr-2467-f6` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The `"pull request #2461"` related-artifacts entry is not a convention form

- PR number: 2467
- Source review ID: 5440113493
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4205093404>
- Concern: The semantic-link convention defines `related-artifacts` values as repository paths,
  quoted `issue #NNNN` values, or `review-finding:` references. `"pull request #2461"` is none of
  these, and the frontmatter validator does not catch it because value syntax runs only for
  `schema-version: 1` records.
- Solution: Removed the entry. The body still names PR #2461, and `"issue #2458"` reaches its
  specification.
- Current-tree verification: `grep -c "pull request #2461"` on the discussion README returns 0;
  `grep -n "#2461"` finds the body mentions at :28 and :58. `frontmatter-validator` on the file
  exits 0.
- Resolution reference: docs(discussions): drop the unsupported pull-request related-artifacts entry
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4208681393>

### F2 - The Scope row does not place the discussion in the four-aspect frame

- PR number: 2467
- Source review ID: 5440113493
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4205093410>
- Concern: EPIC #2003 sorts proposals by the four aspects of the goals-and-boundaries discussion.
  The sibling discussions name their aspect in the Scope row and list that discussion in
  `related-artifacts`; this one did neither.
- Solution: The maintainer chose aspects 2 and 3. The Scope row now names aspect 2 (the
  commit-metadata contract and its format gate) and aspect 3 (the models an orchestration routes
  work to), links Goals and Boundaries, and the frontmatter lists its README.
- Current-tree verification: the README's :24 holds the new Scope row and :6 the new
  `related-artifacts` path; `linter markdown` and `linter lychee` exit 0.
- Resolution reference: docs(discussions): place the provenance discussion in the four-aspect frame
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4208681809>

### F3 - The siblings' framing sentence is missing

- PR number: 2467
- Source review ID: 5440113493
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4205093420>
- Concern: Each sibling says in its opening section that nothing in it changes a specification
  and that a conclusion takes effect only when the owner records it. Here that framing appeared
  only in the PR body, which does not enter history, so the EU AI Act section read normatively.
- Solution: Added "Nothing here changes a specification or a rule. A conclusion takes effect only
  when the owner of EPIC #2003 records it." at the end of "Why This Discussion". "Or a rule" covers
  the follow-up's planned `AGENTS.md` and skill changes.
- Current-tree verification: `grep -n "Nothing here changes"` on the README finds :45.
- Resolution reference: docs(discussions): state that the provenance discussion changes no rule
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4208682090>

### F4 - `valueonly` belongs in `project-words.txt`

- PR number: 2467
- Source review ID: 5440113493
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4205093426>
- Concern: `AGENTS.md` sends new technical terms to the shared dictionary. `valueonly` is a git
  format option the follow-up work would reuse; the regulation's term used once fits the inline
  precedent.
- Solution: Added `valueonly` to `project-words.txt` and removed it from the inline
  `cspell:ignore`, which keeps the regulation's term.
- Current-tree verification: `grep -n "^valueonly$" project-words.txt` finds :537;
  `format-project-words.sh` reports the file already formatted; `linter cspell` exits 0.
- Resolution reference: docs(discussions): move valueonly to the project dictionary
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4208682405>

### F5 - Remove the unsupported pull-request related-artifacts reference

- PR number: 2467
- Source review ID: 5440144697
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4205119431>
- Concern: `pull request #2461` is not an approved v1 `related-artifacts` reference; remove it
  unless the reference union is extended first.
- Solution: Same current-tree change as F1, which removed the entry; no further change.
- Current-tree verification: as for F1, `grep -c "pull request #2461"` on the README returns 0.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4208682831>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4208682831>

### F6 - The `%(trailers)` example cannot be pasted into a POSIX shell

- PR number: 2467
- Source review ID: 5440144697
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4205119475>
- Concern: In the Options table, the unquoted `%(trailers)` format argument is parsed as shell
  syntax. The later `valueonly` example already quotes it.
- Solution: Quoted the argument as `git log --format='%(trailers)'` in the O3 row, keeping the
  table's column widths.
- Current-tree verification: the README's :122 holds the quoted form; `sh -n -c` accepts the
  command, `git log -1` with that format runs, and `linter markdown` exits 0.
- Resolution reference: docs(discussions): quote the trailers format argument in the options table
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2467#discussion_r4208683199>

## Processing Log

- 2026-10-07 15:02 UTC - Started audit. Fetched all six review threads with GraphQL (all
  unresolved, none outdated) and both review bodies; normalized them into F1 to F6.
- 2026-10-07 15:24 UTC - Pushed the five fix commits after the pre-commit gate passed.
- 2026-10-07 15:24 UTC - Replied on all six threads after re-checking each claim against the pushed
  tree; responded to the position comment.
- 2026-10-07 15:28 UTC - The audit commit `docs(pr-reviews): add the PR #2467 review audit` was
  made although the pre-commit gate failed: a pipe through `tail` hid the gate's exit code. cspell
  rejected one regulation term in the F4 entry, which the next commit rewords.

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
