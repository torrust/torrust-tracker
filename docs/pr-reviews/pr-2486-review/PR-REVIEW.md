---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261007-ai-model-provenance-in-commits/README.md
---

<!-- skill-link: process-pr-review -->

# PR #2486 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2486>.

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

Copilot review 5455025162 left one inline finding, headed `[Major][F1]`, on the `project-automation-is-a-dependency` paragraph under Q11 of the discussion README. The reviewer's ID F1 collides with no earlier finding, so it is kept. The review's overview lists the same finding with a Low severity icon and adds no other request, so it adds no row; the row records the thread's bracket, which the validator compares.

Jose Celano's review 5469325284 (COMMENTED, at `30b601421`) left five inline findings, headed `[Major][R1]`, `[Minor][R2]`, `[Minor][R4]`, `[Suggestion][R5]` and `[Nit][R6]`, and one review-body finding, `[Minor][R3]`, which has no thread and is `NON_RESOLVABLE`. The reviewer's IDs collide with no earlier finding, so they are kept. The rest of the review body is his verdict and adds no request. His conversation comment 6079919819 lists R1 to R4 as his pre-merge asks and R5 and R6 as non-blocking, and adds no other request.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2486-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| R1 | `review-finding:pr-2486-r1` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| R2 | `review-finding:pr-2486-r2` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| R3 | `review-finding:pr-2486-r3` | Human | Minor | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| R4 | `review-finding:pr-2486-r4` | Human | Minor | metadata | ORIGINAL | NO_ACTION | OPEN |
| R5 | `review-finding:pr-2486-r5` | Human | Suggestion | documentation | ORIGINAL | NO_ACTION | RESOLVED |
| R6 | `review-finding:pr-2486-r6` | Human | Nit | formatting | ORIGINAL | NO_ACTION | RESOLVED |

## Finding Details

### F1 - The project-automation entry states pinning as a fact about the repository

- PR number: 2486
- Source review ID: 5455025162
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4217662427>
- Concern: The `project-automation-is-a-dependency` paragraph opened with "The tools the project runs in continuous integration are locked and fully defined.", which does not hold for the repository: CI selects floating toolchains such as `dtolnay/rust-toolchain@stable` (`.github/workflows/testing.yaml:57`) and `@nightly` (`.github/workflows/generate_coverage_pr.yaml:41`), and the agent profiles enable model invocation without naming a model (`.github/agents/pr-reviewer.agent.md:7`). Because `docs/discussions/AGENTS.md` makes an inaccurate repository claim a round blocker, the reviewer asked for the pinning to be phrased as a requirement of the proposal or qualified. The thread's bracket is `[Major]`; the review overview labels the same finding with a Low severity icon.
- Solution: The opening sentence is replaced by two sentences that state the concept instead of describing the workflows: "A public process attracts provenance of its own accord: what the project's automation runs is visible in tracked bytes and changes only through a pull request, so a model that acts inside it can be named exactly where a personal tool cannot. The record's subject is not that a model was used but the choices it may make in that process, such as certifying a new tagged release; this rule proposes to pin such a model's identity so that a choice can be traced to it." The rest of the entry is unchanged.
- Current-tree verification: At the branch head, `git grep -F 'locked and fully defined'` finds nothing under `docs/discussions`; README line 337 (line 334 on the commit the thread was posted on), the line the thread anchors, opens with the two sentences quoted above; a word diff from the round commit to the fix changes only that opening sentence. Copilot's citations still read as cited (`@stable` at `testing.yaml:57`, `@nightly` at `generate_coverage_pr.yaml:41`, `disable-model-invocation: false` and no model field in `pr-reviewer.agent.md`), and the paragraph no longer states that CI tooling is locked: its remaining statement that such a model's identity is pinned follows the new sentence in which the rule proposes that pinning.
- Resolution reference: `docs(discussions): [#2003] state the pinning premise of my project-automation entry as a concept`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4217828891>

### R1 - The F1 fix restates the repository claim as a concept

- PR number: 2486
- Source review ID: 5469325284
- Reviewer finding ID: R1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4229526754>
- Concern: The opening sentence of `project-automation-is-a-dependency` after the F1 fix said "what the project's automation runs is visible in tracked bytes and changes only through a pull request", a present-tense claim about the repository that the reviewer's evidence contradicts: CI selects floating toolchains, the agent profiles set `disable-model-invocation: false` and name no model, and automated review is enabled in repository settings rather than tracked files. Under the merge gate in `docs/discussions/AGENTS.md`, a factual claim about the repository that does not hold blocks the round.
- Solution: The opening sentence now states the reason under the rule rather than a fact about today's repository: "A public process attracts provenance of its own accord: a project tracks what its automation uses for the same reason it ships a lockfile, to constrain the scope of its dependencies, and under this rule a model that acts inside that automation is one of them, so it can be named exactly where a personal tool cannot." The rest of the entry is unchanged.
- Current-tree verification: Under Q11's `#### Positions`, the paragraph headed `project-automation-is-a-dependency` opens with the sentence quoted above, and `git grep -F 'is visible in tracked bytes and changes only through a pull request'` finds nothing under `docs/discussions`; the rest of the paragraph, from "The record's subject is not that a model was used", is byte-identical to the head the review was posted on.
- Resolution reference: `docs(discussions): [#2003] restate my automation-dependency reason and note the B4 revision`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r{reply-R1}>

### R2 - The Q11 provenance line no longer matches the B4 entry

- PR number: 2486
- Source review ID: 5469325284
- Reviewer finding ID: R2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4229526762>
- Concern: The Q11 "First posted as" line reads as a statement that the Q11 entries are the text first posted under `B1` to `B6`, but `project-automation-is-a-dependency` no longer reads as posted, because its opening sentence was replaced in this round.
- Solution: The line gains one sentence: "`project-automation-is-a-dependency` revises `B4`'s opening sentence in this round (PR #2486 review findings F1 and R1)."
- Current-tree verification: The "First posted as" line that closes Q11's entry ends with the sentence quoted above, after the number labels `B1` to `B6`.
- Resolution reference: `docs(discussions): [#2003] restate my automation-dependency reason and note the B4 revision`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r{reply-R2}>

### R3 - The description's verification claim did not hold for B4

- PR number: 2486
- Source review ID: 5469325284
- Reviewer finding ID: R3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2486#pullrequestreview-5469325284>
- Concern: The pull-request description's Verification section said that the sixteen points rendered whole are identical to the posted comment and that the entries are unchanged since, which no longer held for `B4` after the F1 fix; the merge tool appends the description to the merge commit, so the claim could not be corrected after merge.
- Solution: The description was replaced at 11:21 UTC, outside the repository tree: its Verification section now says that fifteen of the sixteen points rendered whole are identical and that `B4`'s opening sentence was revised for review finding F1.
- Current-tree verification: `contrib/dev-tools/git/github-merge.py` line 647 at `686a42f45` appends the description to the merge message; the description captured after the replacement reads "Fifteen of the sixteen points rendered whole are identical; B4's opening sentence was revised afterwards for review finding F1".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2486#issuecomment-{response-R3}>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2486#issuecomment-{response-R3}>

### R4 - Whether an AI assistant drafted entry text

- PR number: 2486
- Source review ID: 5469325284
- Reviewer finding ID: R4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4229526774>
- Concern: The reviewer asked whether an AI assistant drafted or researched any entry text, because the attribution rule in `docs/discussions/AGENTS.md` asks an entry to say so and a missing attribution is a merge-gate reason; if none did, a reply saying so is enough.
- Solution: Answered on the thread, with no README change: "My contributions may use private tooling, that may or may not include ML." The thread is left open for the reviewer.
- Current-tree verification: Reply 4229731579 sits on thread 4229526774 (its `in_reply_to_id`), posted at 11:44 UTC; no README line changed for R4.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4229731579>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4229731579>

### R5 - The release-certification example and `humans-author-and-certify`

- PR number: 2486
- Source review ID: 5469325284
- Reviewer finding ID: R5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4229526780>
- Concern: The `project-automation-is-a-dependency` entry gives "certifying a new tagged release" as an example of a choice a model may make, while `humans-author-and-certify` reserves certification for humans; the reviewer suggested a different example, such as preparing a release, and marked the point non-blocking, on substance.
- Solution: Left as it is: the point is substance rather than document quality, so it belongs to the decision round.
- Current-tree verification: The `project-automation-is-a-dependency` paragraph still contains "such as certifying a new tagged release".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r{reply-R5}>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r{reply-R5}>

### R6 - Entry paragraphs are single lines

- PR number: 2486
- Source review ID: 5469325284
- Reviewer finding ID: R6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4229526789>
- Concern: The entries are single long lines while the rest of the file wraps prose at 100 columns; `MD013` is off, so lint passes, and the same holds for this audit record. The reviewer requested no change to the title.
- Solution: Left as it is: the entries keep the form their author writes, one line per paragraph.
- Current-tree verification: The entry paragraphs remain single lines, and `.markdownlint.json` sets `MD013` to `false`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r{reply-R6}>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r{reply-R6}>

## Processing Log

- 2026-10-08 09:58 UTC - Opened the pull request with the round and requested reviews; Copilot's review started at 09:59 UTC.
- 2026-10-08 10:12 UTC - Squashed the branch to one commit, `docs(discussions): [#2003] add my round on AI model provenance in commits`, and force-pushed it at 10:12:29 UTC.
- 2026-10-08 10:13 UTC - Copilot review 5455025162, submitted at 10:13:04 UTC on the head from before the squash, left one inline finding (comment 4217662427), recorded as F1 (`Major`). Its overview labels the finding Low and adds no other request, so it adds no row.
- 2026-10-08 10:30 UTC - Fixed F1 in `docs(discussions): [#2003] state the pinning premise of my project-automation entry as a concept` (author time 10:30:04 UTC) and pushed it; the reply below was posted on that head.
- 2026-10-08 10:31 UTC - Replied `FIXED` on the F1 thread at 10:31:46 UTC with the fix commit's subject and the restated sentences, then resolved the thread.
- 2026-10-08 10:32 UTC - Amended the fix commit under the same subject and author time to restate the sentence as a concept (committer time 10:32:18 UTC) and force-pushed it at 10:32:22 UTC; the reply was edited at 10:32:22 UTC and now names the amended commit.
- 2026-10-08 10:33 UTC - Recorded F1 here from the GitHub capture taken at 10:33 UTC: the pull request's only review thread is resolved and outdated, and no unresolved actionable thread remains.
- 2026-10-09 11:18 UTC - Jose Celano's review 5469325284 (COMMENTED, at `30b601421`) left five inline findings and one review-body finding, recorded under the reviewer's IDs as R1 (`Major`), R2, R3 and R4 (`Minor`), R5 (`Suggestion`) and R6 (`Nit`).
- 2026-10-09 11:21 UTC - Replaced the pull-request description so that its Verification section says fifteen of the sixteen points are identical and that `B4`'s opening sentence was revised for F1 (R3, fixed outside the tree).
- 2026-10-09 11:25 UTC - The maintainer's conversation comment 6079919819 stated that he intends to merge, listed R1 to R4 as pre-merge asks, and left R5 and R6 open to take or leave.
- 2026-10-09 11:44 UTC - Answered R4 on its thread (reply 4229731579) with no README change, and left the thread open.
- 2026-10-09 12:28 UTC - Fixed R1 and R2 in `docs(discussions): [#2003] restate my automation-dependency reason and note the B4 revision`.
- 2026-10-09 12:29 UTC - Recorded R1 to R6 here. The replies on R1, R2, R5 and R6 and the conversation response for R3 follow the push, so their URLs are placeholders until they exist; the threads of R1, R2, R5 and R6 are resolved after their replies are posted, and R4's stays open.

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
