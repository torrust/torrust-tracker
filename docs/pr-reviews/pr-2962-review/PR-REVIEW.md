---
pr-number: 2962
pr-url: https://github.com/torrust/torrust-tracker/pull/2962
last-updated-utc: "2026-11-19 09:16"
---

<!-- skill-link: process-pr-review -->

# PR #2962 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2962.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

- Round 1: da2ce7 review 5972179903 (APPROVED, at the first head) with the reviewer's findings F1 to F4, none blocking, as audit findings F1 to F4. Copilot review 5972120109 (COMMENTED, at the first head) reported no open issue and has no thread.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2962-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2962-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2962-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2962-f4 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The PR description had no validation section and linked the issues with `Refs`

- PR number: 2962
- Source review ID: 5972179903
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2962#discussion_r4626547951
- Concern: The `open-pull-request` skill requires "Validation performed" in the body and gives `Related to #N` for a spec-only pull request. The description stated no check and ended with a `Refs` line. The merge tool copies the description into the merge commit.
- Solution: The description has a Validation section and ends with `Related to #2392` and `Related to #1419` on their own lines. The description is outside the tree, so this fix has no commit.
- Current-tree verification: `gh-bot pr view 2962 --repo torrust/torrust-tracker --json body --jq .body | grep -c '^Related to #'` prints 2 (the description is not in the tree; this ran at 09:16 UTC on 2026-11-19).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2962#discussion_r4626598181
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2962#discussion_r4626598181

### F2 - The progress-log entry referred to the maintainer as "he"

- PR number: 2962
- Source review ID: 5972179903
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2962#discussion_r4626547978
- Concern: Every other entry of the EPIC names the role; the pronoun assumes a gender the record does not need.
- Solution: The entry says "its session has started". The entry was first pushed in this pull request and is not merged, so it was edited in place, as the reviewer suggested.
- Current-tree verification: `grep -c 'its session has started' docs/issues/open/2392-test-isolation/EPIC.md` prints 1.
- Resolution reference: docs(issues): [#2392] name all remaining #1419 work in order 5 and record the dated decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2962#discussion_r4626598624

### F3 - The order 5 row named AC8a as the only remaining work

- PR number: 2962
- Source review ID: 5972179903
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2962#discussion_r4626547936
- Concern: The row's parenthetical described a review only, while R8, AC8a (which also asks to prove cooperative completion) and R9 remain.
- Solution: The row names R8, AC8a and R9, with the reviewer's wording.
- Current-tree verification: `grep -c 'remaining R8, AC8a and R9 (review' docs/issues/open/2392-test-isolation/EPIC.md` prints 1.
- Resolution reference: docs(issues): [#2392] name all remaining #1419 work in order 5 and record the dated decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2962#discussion_r4626599070

### F4 - The decision that #1419 stays in this EPIC was undated and not in the Decisions section

- PR number: 2962
- Source review ID: 5972179903
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2962#discussion_r4626547969
- Concern: The archived EPIC #1488 spec dates the decision 2026-11-18. Here it was an undated clause of the 2026-11-19 entry, so a reader could take it for a decision of that day.
- Solution: The Decisions section has an entry dated 2026-11-18 that cites the archived EPIC #1488 spec, and the progress-log entry carries the date. The Phase 4 outcome text is unchanged; the reply says why.
- Current-tree verification: `grep -c '^- 2026-11-18 (maintainer, recorded in the archived EPIC #1488 spec)' docs/issues/open/2392-test-isolation/EPIC.md` prints 1.
- Resolution reference: docs(issues): [#2392] name all remaining #1419 work in order 5 and record the dated decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2962#discussion_r4626599532

## Processing Log

- 2026-11-19 09:16 UTC - Round 1: da2ce7 review 5972179903 (an approval) with F1 to F4. Edited the PR description for F1. Committed and pushed the fix for F2 to F4 (`docs(issues): [#2392] name all remaining #1419 work in order 5 and record the dated decision`); the push dismissed the approval. Replied to the four threads (09:15:55Z to 09:16:03Z). Added rows F1 to F4.
- 2026-11-19 09:16 UTC - The validator on the worktree record reported 4 rows, 1 log entry and 0 failures. Resolved the four threads (about 09:16:50Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread, and the newest review by a reviewer in the review list was 5972179903. Committed these entries.

<!-- Copied verbatim into each audit record. -->

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
