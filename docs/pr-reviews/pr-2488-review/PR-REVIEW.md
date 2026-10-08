---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2488 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2488>.

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

Copilot review 5456474248 raised one inline finding without a reviewer finding ID; it is recorded
as F1. The review body only summarizes that same finding and the PR changes, so it has no row of
its own.

Review 5456541068 (josecelano) is the empty review that holds the author's F1 reply, so it has no
row.

Human review 5456830805 (da2ce7, round 1) numbered its inline findings F1-F3. Its F2 and F3 keep
their IDs; its F1 collides with the Copilot finding already recorded as F1 and is recorded as F4,
with the reviewer's ID in its detail entry. The review body restates those three findings and
lists items it explicitly calls "noted, with no finding" (no issue link, a PR body that does not
mention the follow-up commits, and one mis-closed code span in the body), so the body has no row of
its own.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2488-f1` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2488-f4` | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2488-f2` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2488-f3` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The ADR keeps dangling references to the deleted `.github/prompts/` directory

- PR number: 2488
- Source review ID: 5456474248
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2488#discussion_r4218839155>
- Concern: Deleting the last prompt files removes the `.github/prompts/` directory, but the AI agent governance ADR still listed it in its `semantic-links.related-artifacts` frontmatter and as "Prompt adapters" in its References section.
- Solution: Removed both references from the ADR. The dated 2026-08-21 review-log row that mentions inspecting "prompts" was left unchanged because it records historical state. Two closed issue documents that name the deleted prompt file are historical records and were also left unchanged.
- Current-tree verification: `git ls-files .github/prompts` prints nothing. A repo-wide search for `.github/prompts` finds no remaining path reference that treats the directory as live. Its matches are the two closed specs `docs/issues/closed/2233-2003-tune-unified-pr-review-process/code-span-path-case-inventory.tsv` and `docs/issues/closed/2295-2278-single-source-audit-roster/ISSUE.md` (historical), the ADR's 2026-10-08 review record and the #2349 spec, which record the removal, and this audit record. The pre-commit script passes all nine steps. This sentence was corrected after review; as first written it listed only the two closed specs.
- Resolution reference: `docs(adrs): drop dangling .github/prompts references`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2488#discussion_r4218893132>

### F4 - Removing the prompt adapters needs the review record the ADR's cadence requires

- PR number: 2488
- Source review ID: 5456830805
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2488#discussion_r4219129893>
- Concern: Removing a tracked prompt triggers the ADR's review cadence, which requires a dated review record. The PR removed the inventory row without one, leaving the inventory labelled "initial" and the only 2026-08-21 record out of step with the current inventory.
- Solution: Added a 2026-10-08 row for `.github/prompts/`, with evidence state Tracked, the removal scenario, where each workflow now lives, and the limitation that no prompt adapter remains tracked. Renamed the inventory intro to "The tracked inventory is:" and the heading to `### Review records`. The 2026-08-21 row's text is unchanged; only its padding was reflowed for the wider table.
- Current-tree verification: the ADR's review-record table has the 2026-08-21 and 2026-10-08 rows, each with the header's seven cells; `git diff -w` shows no change to the 2026-08-21 row; no file links to the old `initial-review-record` anchor; the pre-commit script passes.
- Resolution reference: `docs(adrs): record the prompt adapter removal review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2488#discussion_r4219204213>

### F2 - The open #2349 spec still counts the deleted prompt among the checked files

- PR number: 2488
- Source review ID: 5456830805
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2488#discussion_r4219129905>
- Concern: The planned #2349 spec's Background says the checker reads 15 workflow documents, including the Copilot-suggestions prompt; after this PR it reads 14.
- Solution: Updated the Background to 14 documents, dropped the prompt from the list, and noted that PR #2488 removed it. Added a progress-log entry and bumped `last-updated-utc`; the 2026-09-26 log entry stays as the historical count.
- Current-tree verification: the distinct file paths the checker reads, from its const tables and `verify_*` functions, give 14 under the spec's counting, none under `.github/prompts/`; the pre-commit script passes.
- Resolution reference: `docs(issues): [#2349] count 14 checked files after the prompt removal`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2488#discussion_r4219204492>

### F3 - The F1 verification sentence is false once the audit is committed

- PR number: 2488
- Source review ID: 5456830805
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2488#discussion_r4219129910>
- Concern: F1's current-tree verification said a `.github/prompts` search matched only the two closed specs, but it also matched this audit record.
- Solution: Rewrote the sentence to list every current match (the two closed specs, the ADR's 2026-10-08 review record, the #2349 spec, and this record) and to note the original wording.
- Current-tree verification: a repo-wide `.github/prompts` search matches exactly those five files; `validate-audit-record.py` reports 0 failures.
- Resolution reference: `docs(pr-reviews): correct the PR #2488 F1 search verification`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2488#discussion_r4219204790>

## Processing Log

- 2026-10-08 12:26 UTC - Fetched review 5456474248 and its single inline thread with GraphQL; recorded F1.
- 2026-10-08 12:27 UTC - Committed and pushed `docs(adrs): drop dangling .github/prompts references`, then replied on the F1 thread.
- 2026-10-08 12:28 UTC - Created this audit record.
- 2026-10-08 12:53 UTC - Fetched da2ce7 review 5456830805 (round 1, changes requested) and its three inline threads with GraphQL; recorded F4 (reviewer F1), F2, and F3.
- 2026-10-08 12:58 UTC - Committed and pushed the F4, F2, and F3 fixes, then replied on all three threads.
- 2026-10-08 12:59 UTC - Recorded F4, F2, and F3 in this audit.

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
