---
pr-number: 2987
pr-url: https://github.com/torrust/torrust-tracker/pull/2987
last-updated-utc: "2026-11-22 09:30"
---

<!-- skill-link: process-pr-review -->

# PR #2987 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2987.

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

- Round 1: Copilot review 6006219669 (COMMENTED, at the first head) with two open issues, as audit findings F1 and F2. Copilot noted that its full agentic review did not start before the timeout; the two findings came from its lite review.
- Round 2: da2ce7 review 6006228043 (CHANGES_REQUESTED, at the first head) with the reviewer's findings F1 to F3, as audit findings F3 to F5. F1 is Major and blocking; F2 and F3 are about the PR description.
- Round 3, on the maintainer's instruction ("Yes, fix them"): da2ce7 review 6006407237 (APPROVED at the round 2 audit head; dismissed by the round 3 push) with the reviewer's findings F5 and F6, both Nits, as audit findings F6 and F7. The review body says the reviewer's F1 to F3 stay fixed and F4 is not used.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2987-f1 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2987-f2 | Copilot | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2987-f3 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2987-f4 | Human | Minor | formatting | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2987-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2987-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2987-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The new log entry did not say how PR #2983 relates to the 2026-11-20 entry

- PR number: 2987
- Source review ID: 6006219669
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653327081
- Concern: The 2026-11-20 entry already names PR #2983 as completing #1419, and the new 2026-11-21 entry called PR #2983 the close-out of a closure on 2026-11-21, which reads as a timeline inconsistency.
- Solution: The new entry says PR #2983 merged on 2026-11-21 13:16 UTC and closed #1419, and that the 2026-11-20 entry was written inside PR #2983 before it merged. The 2026-11-20 entry is unchanged, because the progress log is append-only; the new entry was first pushed in this pull request and is not merged, so it was edited in place.
- Current-tree verification: `grep -c '^  written inside PR #2983 before it merged' docs/issues/open/2392-test-isolation/EPIC.md` prints 1.
- Resolution reference: docs(issues): [#2392] clarify the #1419 closure log entry and Phase 4 wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653353782

### F2 - The Phase 4 outcome ended with an unclear "it"

- PR number: 2987
- Source review ID: 6006219669
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653327100
- Concern: In "so none is tracked outside it", the referent of "it" was unclear.
- Solution: The outcome's second sentence named the EPIC. Round 2 (F3) then replaced the sentence with the reviewer's wording, which names this EPIC and has no unclear "it" either.
- Current-tree verification: `grep -c '^  work (R8, AC8a and R9) was done under this EPIC, and none is deferred to another issue.$' docs/issues/open/2392-test-isolation/EPIC.md` prints 1.
- Resolution reference: docs(issues): [#2392] clarify the #1419 closure log entry and Phase 4 wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653353971

### F3 - The Phase 4 outcome said all #1419 work was done under this EPIC

- PR number: 2987
- Source review ID: 6006228043
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653333898
- Concern: The archived #1419 spec records R1 to R7 as done by 2026-09-10, before this EPIC's first progress-log entry, and this EPIC's own 2026-11-18 Decisions entry says only R8, AC8a and R9 continued here. The PR description repeated the claim, and the merge tool copies the description into the merge commit.
- Solution: The outcome uses the reviewer's wording: #1419 closed on 2026-11-21; its remaining work (R8, AC8a and R9) was done under this EPIC, and none is deferred to another issue. The PR description says the same.
- Current-tree verification: `grep -c 'skill). #1419 closed on 2026-11-21; its remaining$' docs/issues/open/2392-test-isolation/EPIC.md` prints 1, and `grep -c 'with all its' docs/issues/open/2392-test-isolation/EPIC.md` prints 0.
- Resolution reference: docs(issues): [#2392] say only the remaining #1419 work was done under this EPIC
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653379494

### F4 - The PR description was hard-wrapped in three places

- PR number: 2987
- Source review ID: 6006228043
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653333902
- Concern: The two Summary bullets and the paragraph after them each continued on a second line, against the `write-markdown-docs` and `open-pull-request` skills.
- Solution: Each paragraph and bullet of the description is one line. The description is outside the tree, so this fix has no commit.
- Current-tree verification: `gh-bot pr view 2987 --repo torrust/torrust-tracker --json body --jq .body | grep -c '^  '` prints 0 (the description is not in the tree; this ran at 15:27 UTC on 2026-11-21).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653379647
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653379647

### F5 - The PR description named no changed file

- PR number: 2987
- Source review ID: 6006228043
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653333905
- Concern: The `open-pull-request` skill asks the body to list the files or packages touched.
- Solution: The description names `docs/issues/open/2392-test-isolation/EPIC.md` and this audit record. The description is outside the tree, so this fix has no commit.
- Current-tree verification: `gh-bot pr view 2987 --repo torrust/torrust-tracker --json body --jq .body | grep -c 'Changed file: .docs/issues/open/2392-test-isolation/EPIC.md.'` prints 1 (the description is not in the tree; this ran at 15:27 UTC on 2026-11-21).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653379789
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653379789

### F6 - The EPIC's stamp and log said 13:24 after the review rounds changed the text

- PR number: 2987
- Source review ID: 6006407237
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653489257
- Concern: The round 1 and round 2 commits rewrote the progress-log entry and the Phase 4 outcome at about 15:14 and 15:20 UTC, but `last-updated-utc` and the entry stamp stayed at 2026-11-21 13:24.
- Solution: `last-updated-utc` carries the time of the round 3 change, and an appended progress-log entry records what the review rounds changed and when. The 13:24 entry keeps its stamp, because the log is append-only.
- Current-tree verification: `grep -c 'Review rounds of PR #2987 changed the$' docs/issues/open/2392-test-isolation/EPIC.md` prints 1, and `grep -c '^last-updated-utc: "2026-11-21 13:24"' docs/issues/open/2392-test-isolation/EPIC.md` prints 0.
- Resolution reference: docs(issues): [#2392] stamp the PR 2987 review changes and log the audit start
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4657527405

### F7 - The Processing Log did not say when the record was started

- PR number: 2987
- Source review ID: 6006407237
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4653489260
- Concern: The log opened with the round 1 replies at 15:19 and the record first appeared in a commit at 15:28, so a reader could not tell whether it existed before the first reply. The second entry's 15:27 stamp also differed from the about 15:20 time of the resolutions it records.
- Solution: An appended entry says the record was created right after the round 1 replies and first committed at about 15:28Z, and explains the 15:27 stamp. Earlier entries are unchanged, because the log is append-only.
- Current-tree verification: `grep -c '^- [0-9-]* [0-9:]* UTC - Correction for the record' docs/pr-reviews/pr-2987-review/PR-REVIEW.md` prints 1.
- Resolution reference: docs(issues): [#2392] stamp the PR 2987 review changes and log the audit start
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2987#discussion_r4657527507

## Processing Log

- 2026-11-21 15:19 UTC - Round 1: Copilot review 6006219669 with F1 and F2. Committed and pushed the fix (`docs(issues): [#2392] clarify the #1419 closure log entry and Phase 4 wording`). Replied to the two threads (about 15:19:37Z). Added rows F1 and F2.
- 2026-11-21 15:27 UTC - The validator on the worktree record reported 2 rows, 1 log entry and 0 failures. Resolved the two round 1 threads (about 15:20:09Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply. A GraphQL refresh then found 3 unresolved threads: da2ce7 review 6006228043 had arrived at 15:14:22Z.
- 2026-11-21 15:27 UTC - Round 2: da2ce7 review 6006228043 (CHANGES_REQUESTED) with F3 to F5. Committed and pushed the fix for F3 (`docs(issues): [#2392] say only the remaining #1419 work was done under this EPIC`), which also replaced the round 1 wording of F2. Edited the PR description for F3 to F5 at 15:21:31Z. Replied to the three threads (about 15:26:42Z). Added rows F3 to F5 and updated the F2 verification to the current tree.
- 2026-11-21 15:27 UTC - The validator on the worktree record reported 5 rows and 0 failures. Resolved the three round 2 threads (about 15:27:23Z) with the same selection; a GraphQL refresh found no unresolved thread, and the newest review by a reviewer in the review list was 6006228043. Committed these entries.
- 2026-11-22 09:25 UTC - Correction for the record's start: this record did not exist before the first reply. It was created in the worktree right after the round 1 replies (about 15:20Z on 2026-11-21) and first committed in `docs(pr-reviews): [#2392] audit review rounds 1 and 2 of PR 2987` (about 15:28Z). The second entry is stamped 15:27, when it was written; the round 1 resolutions it records happened at about 15:20:09Z.
- 2026-11-22 09:30 UTC - Round 3, on the maintainer's instruction ("Yes, fix them"): da2ce7 review 6006407237 (an approval at the round 2 audit head) with F6 and F7. Committed and pushed the fix (`docs(issues): [#2392] stamp the PR 2987 review changes and log the audit start`); the push dismissed the approval. Replied to the two threads (about 09:29:46Z). Added rows F6 and F7.
- 2026-11-22 09:30 UTC - The validator on the worktree record reported 7 rows, 5 log entries and 0 failures. Resolved the two round 3 threads (about 09:30:18Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread, and the newest review by a reviewer in the review list was 6006407237. Committed these entries.

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
